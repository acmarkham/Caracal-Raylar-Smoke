#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SignedAxis {
    X,
    NegX,
    Y,
    NegY,
    Z,
    NegZ,
}

impl SignedAxis {
    fn select(self, value: Vector3) -> i32 {
        match self {
            Self::X => value.x,
            Self::NegX => value.x.saturating_neg(),
            Self::Y => value.y,
            Self::NegY => value.y.saturating_neg(),
            Self::Z => value.z,
            Self::NegZ => value.z.saturating_neg(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AxisMap {
    pub x: SignedAxis,
    pub y: SignedAxis,
    pub z: SignedAxis,
}

impl AxisMap {
    pub const IDENTITY: Self = Self {
        x: SignedAxis::X,
        y: SignedAxis::Y,
        z: SignedAxis::Z,
    };

    pub fn apply(self, value: Vector3) -> Vector3 {
        Vector3 {
            x: self.x.select(value),
            y: self.y.select(value),
            z: self.z.select(value),
        }
    }
}

impl Default for AxisMap {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MagneticCalibration {
    pub hard_iron_offset_nt: Vector3,
    /// Row-major soft-iron correction matrix in signed Q15 format.
    pub soft_iron_q15: [[i32; 3]; 3],
}

impl MagneticCalibration {
    pub const IDENTITY: Self = Self {
        hard_iron_offset_nt: Vector3 { x: 0, y: 0, z: 0 },
        soft_iron_q15: [[32_768, 0, 0], [0, 32_768, 0], [0, 0, 32_768]],
    };

    pub fn apply(self, value: Vector3) -> Vector3 {
        let input = [
            i64::from(value.x) - i64::from(self.hard_iron_offset_nt.x),
            i64::from(value.y) - i64::from(self.hard_iron_offset_nt.y),
            i64::from(value.z) - i64::from(self.hard_iron_offset_nt.z),
        ];
        let row = |index: usize| -> i32 {
            let sum = i64::from(self.soft_iron_q15[index][0]) * input[0]
                + i64::from(self.soft_iron_q15[index][1]) * input[1]
                + i64::from(self.soft_iron_q15[index][2]) * input[2];
            (sum / 32_768).clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
        };
        Vector3 {
            x: row(0),
            y: row(1),
            z: row(2),
        }
    }
}

impl Default for MagneticCalibration {
    fn default() -> Self {
        Self::IDENTITY
    }
}
