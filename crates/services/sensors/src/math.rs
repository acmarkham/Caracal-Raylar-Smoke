pub(crate) const PI: f32 = core::f32::consts::PI;
const CENTIDEGREES_PER_RADIAN: f32 = 18_000.0 / PI;

pub(crate) fn atan2_cdeg(y: f32, x: f32) -> i32 {
    let value = libm::atan2f(y, x) * CENTIDEGREES_PER_RADIAN;
    if value >= 0.0 {
        (value + 0.5) as i32
    } else {
        (value - 0.5) as i32
    }
}

pub(crate) fn heading_cdeg(y: f32, x: f32, correction_cdeg: i32) -> u16 {
    (atan2_cdeg(y, x) + correction_cdeg).rem_euclid(36_000) as u16
}

pub(crate) fn isqrt(value: u64) -> u32 {
    if value < 2 {
        return value as u32;
    }

    let mut x = value;
    let mut next = (x + value / x) / 2;
    while next < x {
        x = next;
        next = (x + value / x) / 2;
    }
    x as u32
}

pub(crate) fn vector_magnitude(x: i32, y: i32, z: i32) -> u32 {
    let x = i64::from(x);
    let y = i64::from(y);
    let z = i64::from(z);
    isqrt((x * x + y * y + z * z) as u64)
}

pub(crate) fn abs_i32(value: i32) -> u32 {
    value.unsigned_abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_square_root_and_heading_are_stable() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(24), 4);
        assert_eq!(isqrt(25), 5);
        assert_eq!(vector_magnitude(300, 400, 0), 500);
        assert_eq!(heading_cdeg(0.0, 1.0, 0), 0);
        assert_eq!(heading_cdeg(1.0, 0.0, 0), 9_000);
        assert_eq!(heading_cdeg(-1.0, 0.0, 0), 27_000);
    }
}
