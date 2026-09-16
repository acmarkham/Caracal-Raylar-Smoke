use crate::identity::DeviceUid;

const UID_BASE_ADDR: usize = 0x0BFA_0700;

pub fn read_device_uid() -> DeviceUid {
    DeviceUid {
        word0: read_uid_word(0),
        word1: read_uid_word(4),
        word2: read_uid_word(8),
    }
}

fn read_uid_word(offset: usize) -> u32 {
    unsafe { core::ptr::read_volatile((UID_BASE_ADDR + offset) as *const u32) }
}
