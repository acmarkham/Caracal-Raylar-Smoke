/// the allocation bitmap signals whether or not a cluster is in use.
/// the AllocationBitmapDirEntry identifies where to locate the bitmap.
/// for example, first_cluster 2 means that the allocation bitmap is the very first cluster in the cluster heap (cluster id 0 and 1 are not valid)
/// each bit in the allocation bitmap points to a cluster (starting at cluster 2).
/// therefore the following bits (as they are layed out in memory) map to the following clusters
///         [byte 0                ][byte 1                ][byte 2                ][byte 3                ]
/// bit      7  6  5  4  3  2  1  0  7  6  5  4  3  2  1  0  7  6  5  4  3  2  1  0  7  6  5  4  3  2  1  0
/// cluster  9  8  7  6  5  4  3  2 17 16 15 14 13 12 11 10 25 24 23 22 21 20 19 18 33 32 31 30 29 28 27 26
/// NOTE: the above layout is not obvious from the spec but it has been confirmed with how the windows implementation writes the bits.
/// for example an allocation bitmap of this bit string "11111111 11111111 00000111" means that clusters 2 - 20 inclusive are allocated. That is 19 clusters in total.
/// Quote from the microsoft spec section 7.1.5 Note "The first bit in the bitmap is the lowest-order bit of the first byte."
///
/// NOTE: I encountered what appears to be a logic error in the linux kernel exfat implementation where bits are incorrectly counted from MSB to LSB and not the other way around when locating free allocations.
/// this does not affect the allocation bitmap write consistency but could possibly lead to unnecessary fragmentation
///
use core::{marker::PhantomData, ops::Range};

use super::{
    BlockDevice, bisync,
    directory_entry::AllocationBitmapDirEntry,
    error::ExFatError,
    fat::Fat,
    file::{NO_CLUSTER_ID, Touched, TouchedKind, TouchedSector},
    file_system::ExFatResult,
    slot_cache::SlotCache,
};

const FIRST_CLUSTER_ID: u32 = 2;

const fn clusters_per_bitmap_sector<const SIZE: usize>() -> u32 {
    SIZE as u32 * u8::BITS
}

const fn bitmap_sector_index<const SIZE: usize>(cluster_id: u32) -> u32 {
    (cluster_id - FIRST_CLUSTER_ID) / clusters_per_bitmap_sector::<SIZE>()
}

const fn first_cluster_in_bitmap_sector<const SIZE: usize>(sector_index: u32) -> u32 {
    FIRST_CLUSTER_ID + sector_index * clusters_per_bitmap_sector::<SIZE>()
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub(crate) struct AllocationBitmap<const SIZE: usize> {
    /// start of the allocation bitmap table
    pub first_cluster: u32,
    /// size, in sectors, of the allocation bitmap
    pub num_sectors: u32,
}

impl<const SIZE: usize> AllocationBitmap<SIZE> {
    pub(crate) fn new(alloc_bitmap: &AllocationBitmapDirEntry) -> Self {
        let num_sectors = alloc_bitmap.data_length.div_ceil(SIZE as u64) as u32;

        Self {
            first_cluster: alloc_bitmap.first_cluster,
            num_sectors,
        }
    }
}

pub(crate) struct AllocatedRun {
    pub first_cluster: u32,
    pub cluster_count: u32,
}

#[derive(Debug, Clone)]
pub(crate) enum StoredChain {
    Empty,
    Contiguous {
        first: u32,
        cluster_count: u32,
    },
    Fat {
        first: u32,
        last: u32,
        cluster_count: u32,
    },
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Default)]
pub(crate) struct AllocationBitmapSlim {
    pub first_sector: u32,
    pub num_sectors: u32,
}

#[derive(Debug)]
pub(crate) struct Allocator<D, const SIZE: usize, const N: usize>
where
    D: BlockDevice<SIZE>,
{
    pub bitmap: AllocationBitmapSlim,
    cache: SlotCache<D, SIZE, N>,
    next_search_cluster: u32,
    _phantom: PhantomData<D>,
}

impl<D, const SIZE: usize, const N: usize> Allocator<D, SIZE, N>
where
    D: BlockDevice<SIZE>,
{
    pub fn new() -> Self {
        Self {
            bitmap: AllocationBitmapSlim::default(),
            cache: SlotCache::new(),
            next_search_cluster: FIRST_CLUSTER_ID,
            _phantom: PhantomData::default(),
        }
    }

    #[bisync]
    pub async fn flush_sector(&mut self, io: &mut D, sector: u32) -> ExFatResult<(), D, SIZE> {
        self.cache.flush_sector(io, sector).await?;
        Ok(())
    }

    #[bisync]
    pub async fn flush(&mut self, io: &mut D) -> ExFatResult<(), D, SIZE> {
        self.cache.flush(io).await?;
        Ok(())
    }

    #[bisync]
    pub async fn allocate(
        &mut self,
        io: &mut D,
        touched: &mut impl Touched,
        chain: &StoredChain,
        count: u32,
    ) -> ExFatResult<AllocatedRun, D, SIZE> {
        match chain {
            StoredChain::Empty => {
                let run = self.find_free_clusters(io, count).await?;
                self.mark_allocated(io, touched, &run, true).await?;
                Ok(run)
            }
            StoredChain::Contiguous {
                first,
                cluster_count,
            } => {
                let from_cluster = first + cluster_count;
                let run = self
                    .find_free_clusters_from(io, from_cluster, count)
                    .await?;
                self.mark_allocated(io, touched, &run, true).await?;
                Ok(run)
            }
            StoredChain::Fat {
                first: _first,
                last,
                cluster_count: _len,
            } => {
                let from_cluster = last + 1;
                let run = self
                    .find_free_clusters_from(io, from_cluster, count)
                    .await?;
                self.mark_allocated(io, touched, &run, true).await?;
                Ok(run)
            }
        }
    }

    #[bisync]
    pub async fn free(
        &mut self,
        io: &mut D,
        touched: &mut impl Touched,
        fat: &mut Fat<D, SIZE, N>,
        chain: &StoredChain,
    ) -> ExFatResult<(), D, SIZE> {
        match chain {
            StoredChain::Empty => {
                // nothing to do
            }
            StoredChain::Contiguous {
                first,
                cluster_count,
            } => {
                let run = AllocatedRun {
                    first_cluster: *first,
                    cluster_count: *cluster_count,
                };
                self.mark_allocated(io, touched, &run, false).await?
            }
            StoredChain::Fat {
                first,
                last: _last,
                cluster_count: _cluster_count,
            } => {
                let mut cluster_id = *first;

                while let Some(next_cluster_id) =
                    fat.next_cluster_in_fat_chain(cluster_id, io).await?
                {
                    let run = AllocatedRun {
                        first_cluster: cluster_id,
                        cluster_count: 1,
                    };
                    self.mark_allocated(io, touched, &run, false).await?;
                    fat.set(io, touched, cluster_id, NO_CLUSTER_ID).await?;
                    cluster_id = next_cluster_id;
                }

                // the last cluster
                fat.set(io, touched, cluster_id, NO_CLUSTER_ID).await?;
            }
        }
        Ok(())
    }

    fn set_bit_range(block: &mut [u8], range: Range<u32>, set: bool) {
        if set {
            for bit in range {
                // set the bit
                let byte = bit as usize / 8;
                let mask = 1u8 << (bit % 8);
                block[byte] |= mask;
            }
        } else {
            for bit in range {
                // clear the bit
                let byte = bit as usize / 8;
                let mask = 1u8 << (bit % 8);
                block[byte] &= !mask;
            }
        }
    }

    #[bisync]
    pub(crate) async fn mark_allocated(
        &mut self,
        io: &mut D,
        touched: &mut impl Touched,
        run: &AllocatedRun,
        allocated: bool,
    ) -> ExFatResult<(), D, SIZE> {
        let first_sector = self.bitmap.first_sector;
        let num_sectors = self.bitmap.num_sectors;

        let clusters_per_sector = clusters_per_bitmap_sector::<SIZE>();
        let sector_index = bitmap_sector_index::<SIZE>(run.first_cluster);
        let mut remaining = run.cluster_count;
        let mut cluster_id = run.first_cluster;

        for sector_offset in sector_index..num_sectors {
            let sector_id = first_sector + sector_offset;
            let slot = self.cache.read_mut(sector_id, io).await?;

            let first_cluster_of_slot = sector_offset * clusters_per_sector + FIRST_CLUSTER_ID;
            let start = cluster_id - first_cluster_of_slot;
            let end = clusters_per_sector.min(start + remaining);
            Self::set_bit_range(slot.as_mut_slice(), start..end, allocated);
            touched.insert(TouchedSector::new(TouchedKind::Bitmap, sector_id));
            let num_clusters_in_slot = end - start;
            remaining -= num_clusters_in_slot;
            cluster_id += num_clusters_in_slot;

            if remaining == 0 {
                break;
            }
        }

        Ok(())
    }

    /// locates the next free set of contiguous clusters.
    /// if from_cluster is Some it will, like all clusters, only be included if it is NOT allocated.
    #[bisync]
    async fn find_free_clusters_from(
        &mut self,
        io: &mut D,
        from_cluster: u32,
        num_clusters: u32,
    ) -> ExFatResult<AllocatedRun, D, SIZE> {
        let first_sector = self.bitmap.first_sector;
        let num_sectors = self.bitmap.num_sectors;

        let sector_index = bitmap_sector_index::<SIZE>(from_cluster);
        let mut cluster_id = first_cluster_in_bitmap_sector::<SIZE>(sector_index);
        let mut first_cluster = None;
        let mut count = 0;

        for sector_offset in sector_index..num_sectors {
            let slot = self.cache.read(first_sector + sector_offset, io).await?;

            let (bytes, _remainder) = slot.as_slice().as_chunks::<4>();
            for chunk in bytes {
                if u32::from_le_bytes(*chunk) != u32::MAX {
                    for byte in chunk {
                        for bit in 0..u8::BITS {
                            if *byte & 1 << bit == 0 {
                                if cluster_id < from_cluster {
                                    cluster_id += 1;
                                    continue;
                                }

                                if first_cluster.is_none() {
                                    first_cluster = Some(cluster_id);
                                    count = 1;
                                } else {
                                    count += 1;
                                }

                                if count == num_clusters {
                                    return Ok(AllocatedRun {
                                        first_cluster: first_cluster.unwrap(),
                                        cluster_count: num_clusters,
                                    });
                                }
                            } else {
                                if let Some(first_cluster) = first_cluster {
                                    return Ok(AllocatedRun {
                                        first_cluster: first_cluster,
                                        cluster_count: count,
                                    });
                                }
                            }

                            cluster_id += 1;
                        }
                    }
                } else {
                    cluster_id += u32::BITS;
                    first_cluster = None;
                }
            }
        }

        Err(ExFatError::DiskFull)
    }

    /// locates the next free set of contiguous clusters.
    /// this is used for creating a new file
    #[bisync]
    pub(crate) async fn find_free_clusters(
        &mut self,
        io: &mut D,
        num_clusters: u32,
    ) -> ExFatResult<AllocatedRun, D, SIZE> {
        let search_start = self.next_search_cluster;
        let sector_id = self.bitmap.first_sector;
        let num_sectors = self.bitmap.num_sectors;
        // Each bit, rather than each byte, represents one cluster. Dividing by
        // the sector byte size reads the wrong bitmap sector after the first
        // SIZE clusters and can allocate a cluster that is already in use.
        let sector_index = bitmap_sector_index::<SIZE>(search_start);
        let mut cluster_id = first_cluster_in_bitmap_sector::<SIZE>(sector_index);
        let mut first_cluster = None;
        let mut count = 0;

        let mut fallback = None;

        for sector_offset in sector_index..num_sectors {
            let slot = self.cache.read(sector_id + sector_offset, io).await?;
            let (bytes, _remainder) = slot.as_slice().as_chunks::<4>();
            for chunk in bytes {
                if u32::from_le_bytes(*chunk) != u32::MAX {
                    for byte in chunk {
                        for bit in 0..u8::BITS {
                            if *byte & 1 << bit == 0 {
                                // The scan starts at the beginning of the
                                // containing bitmap sector. Do not relabel its
                                // earlier bits as `search_start` clusters.
                                if cluster_id < search_start {
                                    cluster_id += 1;
                                    continue;
                                }

                                if first_cluster.is_none() {
                                    first_cluster = Some(cluster_id);
                                    count = 1
                                } else {
                                    count += 1;
                                }

                                if count == num_clusters {
                                    let first = first_cluster.unwrap();
                                    self.next_search_cluster = first + count;
                                    return Ok(AllocatedRun {
                                        first_cluster: first,
                                        cluster_count: num_clusters,
                                    });
                                }
                            } else {
                                if fallback.is_none() {
                                    if let Some(cluster_id) = first_cluster {
                                        fallback = Some(AllocatedRun {
                                            first_cluster: cluster_id,
                                            cluster_count: count,
                                        })
                                    }
                                }
                                first_cluster = None;
                            }

                            cluster_id += 1;
                        }
                    }
                } else {
                    cluster_id += u32::BITS;
                    first_cluster = None;
                }
            }
        }

        self.next_search_cluster = FIRST_CLUSTER_ID;
        match fallback {
            Some(run) => Ok(run),
            None => Err(ExFatError::Unexpected("disk full")),
        }
    }
}

#[cfg(test)]
mod tests {
    use aligned::Aligned;
    use alloc::{vec, vec::Vec};

    use super::super::only_sync;
    use super::{
        AllocationBitmapSlim, Allocator, FIRST_CLUSTER_ID, bitmap_sector_index,
        clusters_per_bitmap_sector, first_cluster_in_bitmap_sector,
    };

    const BLOCK_SIZE: usize = 512;
    const BITMAP_SECTOR: u32 = 100;

    #[derive(Debug)]
    struct DummyBlockDevice {
        blocks: Vec<[u8; BLOCK_SIZE]>,
    }

    #[only_sync]
    impl super::BlockDevice<BLOCK_SIZE> for DummyBlockDevice {
        type Error = ();
        type Align = aligned::A4;

        fn read(
            &mut self,
            block_address: u32,
            data: &mut [Aligned<Self::Align, [u8; BLOCK_SIZE]>],
        ) -> Result<(), Self::Error> {
            data[0].copy_from_slice(&self.blocks[(block_address - BITMAP_SECTOR) as usize]);
            Ok(())
        }

        fn write(
            &mut self,
            block_address: u32,
            data: &[Aligned<Self::Align, [u8; BLOCK_SIZE]>],
        ) -> Result<(), Self::Error> {
            self.blocks[(block_address - BITMAP_SECTOR) as usize]
                .copy_from_slice(data[0].as_slice());
            Ok(())
        }

        fn size(&mut self) -> Result<u64, Self::Error> {
            Ok(self.blocks.len() as u64 * BLOCK_SIZE as u64)
        }
    }

    #[test]
    fn bitmap_sector_index_counts_bits_not_bytes() {
        const SECTOR_SIZE: usize = 512;
        const CLUSTERS_PER_SECTOR: u32 = clusters_per_bitmap_sector::<SECTOR_SIZE>();

        assert_eq!(CLUSTERS_PER_SECTOR, 4096);
        assert_eq!(bitmap_sector_index::<SECTOR_SIZE>(FIRST_CLUSTER_ID), 0);
        assert_eq!(
            bitmap_sector_index::<SECTOR_SIZE>(FIRST_CLUSTER_ID + CLUSTERS_PER_SECTOR - 1),
            0
        );
        assert_eq!(
            bitmap_sector_index::<SECTOR_SIZE>(FIRST_CLUSTER_ID + CLUSTERS_PER_SECTOR),
            1
        );
        assert_eq!(first_cluster_in_bitmap_sector::<SECTOR_SIZE>(0), 2);
        assert_eq!(first_cluster_in_bitmap_sector::<SECTOR_SIZE>(1), 4098);
    }

    #[only_sync]
    #[test]
    fn new_file_scan_preserves_bit_offset_within_bitmap_sector() {
        let search_start = 600;
        let bitmap_bit = search_start - FIRST_CLUSTER_ID;
        let mut first_sector = [0u8; BLOCK_SIZE];
        first_sector[bitmap_bit as usize / 8] |= 1 << (bitmap_bit % 8);
        let mut device = DummyBlockDevice {
            blocks: vec![first_sector, [0; BLOCK_SIZE]],
        };
        let mut allocator = Allocator::<DummyBlockDevice, BLOCK_SIZE, 2>::new();
        allocator.bitmap = AllocationBitmapSlim {
            first_sector: BITMAP_SECTOR,
            num_sectors: 2,
        };
        allocator.next_search_cluster = search_start;

        let run = allocator.find_free_clusters(&mut device, 1).unwrap();

        assert_eq!(run.first_cluster, search_start + 1);
        assert_eq!(run.cluster_count, 1);
    }
}
