use raylar_drivers::identity::DeviceSerials;

/// Compact deployment identity used on air.
///
/// It is derived from the platform's existing 32-bit device serial rather
/// than introducing a second hardware identity mechanism. Deployments must
/// verify uniqueness when provisioning nodes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NodeId(pub u32);

impl From<DeviceSerials> for NodeId {
    fn from(value: DeviceSerials) -> Self {
        Self(value.serial_32)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GroupId(pub u16);

/// Random boot-session identity. Obtain this from the platform TRNG.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BootId(pub u32);

/// Volatile frame sequence number. Wrapping is intentional and duplicate
/// windows must therefore be bounded to less than half the 16-bit space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Sequence(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DuplicateKey {
    pub node_id: NodeId,
    pub boot_id: BootId,
    pub sequence: Sequence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SequenceState {
    boot_id: BootId,
    next: Sequence,
}

impl SequenceState {
    pub const fn new(boot_id: BootId) -> Self {
        Self {
            boot_id,
            next: Sequence(0),
        }
    }

    pub const fn boot_id(&self) -> BootId {
        self.boot_id
    }

    pub fn set_boot_id(&mut self, boot_id: BootId) {
        if self.boot_id != boot_id {
            self.boot_id = boot_id;
            self.next = Sequence(0);
        }
    }

    pub fn take(&mut self) -> Sequence {
        let sequence = self.next;
        self.next.0 = self.next.0.wrapping_add(1);
        sequence
    }
}
