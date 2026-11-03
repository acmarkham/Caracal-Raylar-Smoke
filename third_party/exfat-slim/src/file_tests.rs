// Exercise real file I/O, FAT links and allocation on an in-memory device.
use super::*;
use alloc::vec;

const SECTOR: usize = 512;
const CLUSTER: usize = 131_072;
const FAT_SECTOR: usize = 1;
const BITMAP_SECTOR: usize = 2;
const HEAP_SECTOR: usize = 8;
const CLUSTERS: usize = 24;

struct MemoryDevice {
    sectors: Vec<[u8; SECTOR]>,
}

impl BlockDevice<SECTOR> for MemoryDevice {
    type Error = ();
    type Align = aligned::A4;

    fn read(
        &mut self,
        address: u32,
        blocks: &mut [Aligned<Self::Align, [u8; SECTOR]>],
    ) -> Result<(), Self::Error> {
        for (offset, block) in blocks.iter_mut().enumerate() {
            block.copy_from_slice(&self.sectors[address as usize + offset]);
        }
        Ok(())
    }

    fn write(
        &mut self,
        address: u32,
        blocks: &[Aligned<Self::Align, [u8; SECTOR]>],
    ) -> Result<(), Self::Error> {
        for (offset, block) in blocks.iter().enumerate() {
            self.sectors[address as usize + offset].copy_from_slice(block.as_slice());
        }
        Ok(())
    }

    fn size(&mut self) -> Result<u64, Self::Error> {
        Ok((self.sectors.len() * SECTOR) as u64)
    }
}

type Fs = FileSystem<MemoryDevice, SECTOR, 4>;

fn filesystem(free: &[u32]) -> Fs {
    let mut fs = mounted(MemoryDevice {
        sectors: vec![[0; SECTOR]; HEAP_SECTOR + CLUSTERS * CLUSTER / SECTOR],
    });
    // Reserved clusters include neighbouring files; only explicitly freed
    // clusters may be allocated. This also forces fragmented extensions.
    fs.dev.sectors[BITMAP_SECTOR].fill(0xff);
    for &cluster in free {
        let bit = (cluster - 2) as usize;
        fs.dev.sectors[BITMAP_SECTOR][bit / 8] &= !(1 << (bit % 8));
    }
    fs
}

fn mounted(device: MemoryDevice) -> Fs {
    let mut fs = Fs::new(device);
    fs.is_mounted = true;
    fs.fs.cluster_count = CLUSTERS as u32;
    fs.fs.cluster_length = CLUSTER as u32;
    fs.fs.sectors_per_cluster = (CLUSTER / SECTOR) as u32;
    fs.fs.cluster_heap_offset = HEAP_SECTOR as u32;
    fs.fs.first_cluster_of_root_dir = 2;
    fs.upcase_table = Default::default();
    fs.fat.start_of_fat_sector = Some(FAT_SECTOR as u32);
    fs.allocator.bitmap.first_sector = BITMAP_SECTOR as u32;
    fs.allocator.bitmap.num_sectors = 1;
    fs
}

fn file(fs: &mut Fs, clusters: &[u32], len: usize, fat: bool) -> File {
    let first = clusters.first().copied().unwrap_or(NO_CLUSTER_ID);
    let mut flags = GeneralSecondaryFlags::AllocationPossible;
    let chain = if clusters.is_empty() {
        flags |= GeneralSecondaryFlags::NoFatChain;
        StoredChain::Empty
    } else if fat {
        for (i, &cluster) in clusters.iter().enumerate() {
            let next = clusters.get(i + 1).copied().unwrap_or(END_OF_CHAIN);
            let offset = cluster as usize * 4;
            fs.dev.sectors[FAT_SECTOR][offset..offset + 4].copy_from_slice(&next.to_le_bytes());
        }
        StoredChain::Fat {
            first,
            last: *clusters.last().unwrap(),
            cluster_count: clusters.len() as u32,
        }
    } else {
        flags |= GeneralSecondaryFlags::NoFatChain;
        StoredChain::Contiguous {
            first,
            cluster_count: clusters.len() as u32,
        }
    };
    for &cluster in clusters {
        let sector = HEAP_SECTOR + (cluster as usize - 2) * CLUSTER / SECTOR;
        for block in &mut fs.dev.sectors[sector..sector + CLUSTER / SECTOR] {
            block.fill(0x5a);
        }
    }
    File::new(
        &FileDetails {
            first_cluster: first,
            data_length: len as u64,
            valid_data_length: len as u64,
            attributes: FileAttributes::Archive,
            name: heapless::String::new(),
            location: Location::new(0, 0),
            flags,
            secondary_count: 2,
        },
        first,
        CLUSTER as u32,
        &OpenOptions::new().read(true).write(true),
        chain,
    )
}

fn read_back(fs: &mut Fs, file: &mut File) -> Vec<u8> {
    file.seek(fs, 0).unwrap();
    let mut data = Vec::new();
    file.read_to_end(fs, &mut data).unwrap();
    data
}

fn fat_links(fs: &mut Fs, first: u32) -> Vec<u32> {
    let mut chain = vec![first];
    while let Some(next) = fs
        .fat
        .next_cluster_in_fat_chain(*chain.last().unwrap(), &mut fs.dev)
        .unwrap()
    {
        assert!(!chain.contains(&next), "FAT cycle");
        chain.push(next);
    }
    chain
}

#[test]
fn seek_to_fragmented_eof_then_append_preserves_existing_data() {
    let mut fs = filesystem(&[14]);
    let mut f = file(&mut fs, &[5, 9, 13], 3 * CLUSTER, true);
    f.seek(&mut fs, (3 * CLUSTER) as u64).unwrap();
    assert_eq!(f.read(&mut fs, &mut [0; 8]).unwrap(), None);
    f.write(&mut fs, &[0x77; 37]).unwrap();
    assert_eq!(fat_links(&mut fs, 5), [5, 9, 13, 14]);
    let mut expected = vec![0x5a; 3 * CLUSTER];
    expected.extend_from_slice(&[0x77; 37]);
    assert_eq!(read_back(&mut fs, &mut f), expected);
}

#[test]
fn header_rewrite_does_not_allocate_or_change_chain() {
    for fat in [false, true] {
        for len in [2 * CLUSTER - 1, 2 * CLUSTER, 2 * CLUSTER + 1] {
            let clusters: &[u32] = match (fat, len > 2 * CLUSTER) {
                (false, false) => &[5, 6],
                (false, true) => &[5, 6, 7],
                (true, false) => &[5, 9],
                (true, true) => &[5, 9, 13],
            };
            let mut fs = filesystem(&[15, 16]);
            let mut f = file(&mut fs, clusters, len, fat);
            let free_before = fs
                .allocator
                .free_cluster_count(&mut fs.dev, CLUSTERS as u32)
                .unwrap();
            f.seek(&mut fs, 8).unwrap();
            f.write(&mut fs, &[0x33; 34]).unwrap();
            assert_eq!(
                fs.allocator
                    .free_cluster_count(&mut fs.dev, CLUSTERS as u32)
                    .unwrap(),
                free_before,
                "header rewrite allocated storage (fat={fat}, len={len})"
            );
            if fat {
                assert_eq!(fat_links(&mut fs, clusters[0]), clusters);
            } else {
                assert!(matches!(f.chain, StoredChain::Contiguous { .. }));
            }
            // Match StorageDriver::rewrite: restore EOF after patching.
            f.seek(&mut fs, len as u64).unwrap();
            let mut expected = vec![0x5a; len];
            expected[8..42].fill(0x33);
            assert_eq!(read_back(&mut fs, &mut f), expected);
        }
    }
}

#[test]
fn extending_overwrite_links_new_clusters_at_tail() {
    for fat in [false, true] {
        let clusters: &[u32] = if fat { &[5, 9] } else { &[5, 6] };
        let mut fs = filesystem(&[15, 16, 17]);
        let mut f = file(&mut fs, clusters, 2 * CLUSTER, fat);
        f.seek(&mut fs, 8).unwrap();
        let payload = vec![0x88; 2 * CLUSTER + 37];
        f.write(&mut fs, &payload).unwrap();
        let mut expected_chain = clusters.to_vec();
        expected_chain.push(15);
        assert_eq!(fat_links(&mut fs, 5), expected_chain);
        let mut expected = vec![0x5a; 8];
        expected.extend_from_slice(&payload);
        assert_eq!(read_back(&mut fs, &mut f), expected);
    }
}

#[test]
fn initial_write_spanning_fragmented_runs_keeps_every_cluster() {
    let mut fs = filesystem(&[5, 6, 9, 10, 13, 14]);
    let mut f = file(&mut fs, &[], 0, false);
    let payload: Vec<u8> = (0..5 * CLUSTER + 17).map(|i| (i % 251) as u8).collect();
    f.write(&mut fs, &payload).unwrap();
    assert_eq!(fat_links(&mut fs, 5), [5, 6, 9, 10, 13, 14]);
    assert_eq!(read_back(&mut fs, &mut f), payload);
}

#[test]
fn seek_boundary_inside_fragmented_file_reads_next_cluster() {
    let mut fs = filesystem(&[]);
    let mut f = file(&mut fs, &[5, 9, 13], 3 * CLUSTER, true);
    let sector = HEAP_SECTOR + (9 - 2) * CLUSTER / SECTOR;
    fs.dev.sectors[sector][..8].fill(0x44);
    f.seek(&mut fs, CLUSTER as u64).unwrap();
    let mut data = [0; 8];
    assert_eq!(f.read(&mut fs, &mut data).unwrap(), Some(8));
    assert_eq!(data, [0x44; 8]);
}

#[test]
fn paired_files_survive_header_rewrite_close_and_reopen_with_fresh_caches() {
    let mut fs = filesystem(&[5, 6, 7, 8, 9]);
    let options = OpenOptions::new().create_new(true).read(true).write(true);
    let mut reference = fs.open("audio.flac", options.clone()).unwrap();
    reference.write(&mut fs, &vec![0x11; CLUSTER]).unwrap();
    let mut sidecar = fs.open("audio.crcl", options).unwrap();
    sidecar.write(&mut fs, &vec![0x22; CLUSTER]).unwrap();
    reference.write(&mut fs, &vec![0x33; CLUSTER]).unwrap();
    assert!(matches!(reference.chain, StoredChain::Fat { .. }));
    reference.seek(&mut fs, 8).unwrap();
    reference.write(&mut fs, &[0x44; 34]).unwrap();
    reference.seek(&mut fs, (2 * CLUSTER) as u64).unwrap();
    reference.close(&mut fs).unwrap();
    sidecar.close(&mut fs).unwrap();

    // Discard every cache and rediscover the files from directory entries.
    let mut reopened = mounted(fs.dev);
    let mut expected = vec![0x11; CLUSTER];
    expected[8..42].fill(0x44);
    expected.extend_from_slice(&vec![0x33; CLUSTER]);
    assert_eq!(reopened.read("audio.flac").unwrap(), expected);
    assert_eq!(reopened.read("audio.crcl").unwrap(), vec![0x22; CLUSTER]);

    let mut reference = reopened
        .open("audio.flac", OpenOptions::new().write(true).append(true))
        .unwrap();
    reference.write(&mut reopened, &[0x55; 17]).unwrap();
    reference.close(&mut reopened).unwrap();
    let mut reopened = mounted(reopened.dev);
    expected.extend_from_slice(&[0x55; 17]);
    assert_eq!(reopened.read("audio.flac").unwrap(), expected);
    assert_eq!(reopened.read("audio.crcl").unwrap(), vec![0x22; CLUSTER]);
}

#[test]
fn failed_fat_seek_preserves_previous_cursor() {
    let mut fs = filesystem(&[]);
    let mut f = file(&mut fs, &[5, 9], 2 * CLUSTER, true);
    fs.dev.sectors[FAT_SECTOR][20..24].copy_from_slice(&END_OF_CHAIN.to_le_bytes());
    f.seek(&mut fs, 8).unwrap();
    let before = (f.cursor, f.current_cluster, f.remaining_bytes_in_cluster);
    assert!(matches!(
        f.seek(&mut fs, CLUSTER as u64 + 1),
        Err(ExFatError::EndOfFatChain)
    ));
    assert_eq!(
        (f.cursor, f.current_cluster, f.remaining_bytes_in_cluster),
        before
    );
    let mut data = [0; 8];
    assert_eq!(f.read(&mut fs, &mut data).unwrap(), Some(8));
    assert_eq!(data, [0x5a; 8]);
}
