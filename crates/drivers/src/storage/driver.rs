use exfat_slim::asynchronous::file::File;
use exfat_slim::asynchronous::file_system::FileSystem;
use exfat_slim::asynchronous::BlockDevice;
use heapless::String;

pub const BLOCK_BYTES: usize = 512;
pub const CACHE_BLOCKS: usize = 8;
pub const MAX_WRITE_HANDLES: usize = 4;
pub(crate) const DEFAULT_PATH_LEN: usize = 128;

pub struct StorageDriver<
    D,
    const SIZE: usize = BLOCK_BYTES,
    const CACHE: usize = CACHE_BLOCKS,
    const PATH_LEN: usize = DEFAULT_PATH_LEN,
> where
    D: BlockDevice<SIZE>,
{
    pub(crate) fs: FileSystem<D, SIZE, CACHE>,
    pub(crate) write_slots: [Option<WriteSlot<SIZE, PATH_LEN>>; MAX_WRITE_HANDLES],
    pub(crate) read_slot: Option<ReadSlot>,
    pub(crate) write_generations: [u8; MAX_WRITE_HANDLES],
    pub(crate) read_generation: u8,
}

pub(crate) struct WriteSlot<const SIZE: usize, const PATH_LEN: usize> {
    pub file: File,
    pub path: String<PATH_LEN>,
    pub generation: u8,
    pub logical_len: u64,
    pub committed_len: u64,
    pub last_flushed_len: u64,
    pub pending_block: [u8; SIZE],
    pub pending_len: usize,
    pub dirty: bool,
}

pub(crate) struct ReadSlot {
    pub file: File,
    pub generation: u8,
}

impl<D, const SIZE: usize, const CACHE: usize, const PATH_LEN: usize>
    StorageDriver<D, SIZE, CACHE, PATH_LEN>
where
    D: BlockDevice<SIZE>,
{
    pub fn new(block_device: D) -> Self {
        Self {
            fs: FileSystem::new(block_device),
            write_slots: core::array::from_fn(|_| None),
            read_slot: None,
            write_generations: [0; MAX_WRITE_HANDLES],
            read_generation: 0,
        }
    }

    pub fn into_inner(self) -> D {
        self.fs.unmount()
    }
}
