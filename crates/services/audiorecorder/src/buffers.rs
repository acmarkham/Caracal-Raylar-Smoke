/// Fixed-capacity workspace access without allocation or type erasure.
pub trait RecorderBuffers<const ENCODE: usize, const PCM: usize> {
    fn encoded(&mut self) -> &mut [u8; ENCODE];
    fn split(&mut self) -> (&mut [u8; ENCODE], &mut [i32; PCM]);
}

/// Preserves the original WAV constructor and permits owned host-test fixtures.
pub struct OwnedBuffers<const ENCODE: usize, const PCM: usize> {
    encoded: [u8; ENCODE],
    pcm: [i32; PCM],
}

impl<const ENCODE: usize, const PCM: usize> OwnedBuffers<ENCODE, PCM> {
    pub(crate) const fn new() -> Self {
        Self {
            encoded: [0; ENCODE],
            pcm: [0; PCM],
        }
    }
}

impl<const ENCODE: usize, const PCM: usize> RecorderBuffers<ENCODE, PCM>
    for OwnedBuffers<ENCODE, PCM>
{
    fn encoded(&mut self) -> &mut [u8; ENCODE] {
        &mut self.encoded
    }
    fn split(&mut self) -> (&mut [u8; ENCODE], &mut [i32; PCM]) {
        (&mut self.encoded, &mut self.pcm)
    }
}

/// Firmware borrows static storage so construction never copies large arrays.
pub struct BorrowedBuffers<'a, const ENCODE: usize, const PCM: usize> {
    pub(crate) encoded: &'a mut [u8; ENCODE],
    pub(crate) pcm: &'a mut [i32; PCM],
}

impl<const ENCODE: usize, const PCM: usize> RecorderBuffers<ENCODE, PCM>
    for BorrowedBuffers<'_, ENCODE, PCM>
{
    fn encoded(&mut self) -> &mut [u8; ENCODE] {
        self.encoded
    }
    fn split(&mut self) -> (&mut [u8; ENCODE], &mut [i32; PCM]) {
        (self.encoded, self.pcm)
    }
}
