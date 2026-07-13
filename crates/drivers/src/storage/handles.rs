#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileHandle(u8);

impl FileHandle {
    pub(crate) const INDEX_MASK: u8 = 0b0000_0011;
    pub(crate) const GENERATION_SHIFT: u8 = 2;
    pub(crate) const GENERATION_MASK: u8 = 0b0011_1111;

    pub(crate) const fn new(index: usize, generation: u8) -> Self {
        Self(((generation & Self::GENERATION_MASK) << Self::GENERATION_SHIFT) | index as u8)
    }

    pub(crate) const fn index(self) -> usize {
        (self.0 & Self::INDEX_MASK) as usize
    }

    pub(crate) const fn generation(self) -> u8 {
        self.0 >> Self::GENERATION_SHIFT
    }

    pub const fn raw(self) -> u8 {
        self.0
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReadHandle(u8);

impl ReadHandle {
    pub(crate) const fn new(generation: u8) -> Self {
        Self(generation)
    }

    pub(crate) const fn generation(self) -> u8 {
        self.0
    }

    pub const fn raw(self) -> u8 {
        self.0
    }
}
