use crate::{Error, Subframe};

pub(crate) fn residual(samples: &[i32], index: usize, order: u8) -> i64 {
    let x = |ago| i64::from(samples[index - ago]);
    let prediction = match order {
        0 => 0,
        1 => x(1),
        2 => 2 * x(1) - x(2),
        3 => 3 * x(1) - 3 * x(2) + x(3),
        4 => 4 * x(1) - 6 * x(2) + 4 * x(3) - x(4),
        _ => unreachable!(),
    };
    i64::from(samples[index]) - prediction
}

pub(crate) fn fold(value: i64) -> u64 {
    if value >= 0 {
        (value * 2) as u64
    } else {
        (-value * 2 - 1) as u64
    }
}

/// Exact quotient sums for every Rice parameter, without 31 wide shifts and
/// cost updates per sample. Every residual is folded into at most 32 bits.
struct RiceStats {
    sum: u64,
    ones: [u16; 32],
}

impl RiceStats {
    fn new() -> Self {
        Self {
            sum: 0,
            ones: [0; 32],
        }
    }

    fn add(&mut self, mut folded: u32) {
        self.sum += u64::from(folded);
        // Count only set bits; correlated microphone residuals are small.
        while folded != 0 {
            self.ones[folded.trailing_zeros() as usize] += 1;
            folded &= folded - 1;
        }
    }

    fn clear(&mut self) {
        self.sum = 0;
        self.ones.fill(0);
    }
}

/// A bounded search that can be advanced between executor yields.
pub struct Analysis<'a> {
    samples: &'a [i32],
    depth: u8,
    order: u8,
    index: usize,
    rice: RiceStats,
    best_bytes: u64,
    best: Subframe,
}

impl<'a> Analysis<'a> {
    pub(crate) fn samples(&self) -> &'a [i32] {
        self.samples
    }
    pub(crate) fn depth(&self) -> u8 {
        self.depth
    }
    pub(crate) fn new(samples: &'a [i32], depth: u8) -> Result<Self, Error> {
        let limit = 1i64 << (depth - 1);
        if samples
            .iter()
            .any(|&x| i64::from(x) < -limit || i64::from(x) >= limit)
        {
            return Err(Error::SampleOutOfRange);
        }
        let constant = samples.iter().all(|x| *x == samples[0]);
        Ok(Self {
            samples,
            depth,
            order: 0,
            index: 0,
            rice: RiceStats::new(),
            best_bytes: (8 + if constant {
                u64::from(depth)
            } else {
                samples.len() as u64 * u64::from(depth)
            })
            .div_ceil(8),
            best: if constant {
                Subframe::Constant
            } else {
                Subframe::Verbatim
            },
        })
    }

    /// Analyze at most `budget` residuals. Returns true when finished.
    pub fn advance(&mut self, mut budget: usize) -> bool {
        if self.best == Subframe::Constant {
            return true;
        }
        while self.order <= 4 && (self.order as usize) < self.samples.len() && budget != 0 {
            let r = residual(self.samples, self.index, self.order);
            budget -= 1;
            if !(-2147483647..=2147483647).contains(&r) {
                self.next_order();
                continue;
            }
            self.rice.add(fold(r) as u32);
            self.index += 1;
            if self.index == self.samples.len() {
                // Evaluate 4-bit method first, then 5-bit: ties retain the simpler form.
                for width in [4u8, 5] {
                    let maximum = if width == 4 { 14 } else { 30 };
                    let mut quotients = self.rice.sum;
                    for k in 0..=maximum {
                        let cost = 8
                            + u64::from(self.order) * u64::from(self.depth)
                            + 2
                            + 4
                            + u64::from(width)
                            + quotients
                            + (self.samples.len() - self.order as usize) as u64 * (1 + k as u64);
                        let bytes = cost.div_ceil(8);
                        if bytes < self.best_bytes {
                            self.best_bytes = bytes;
                            self.best = Subframe::Fixed {
                                order: self.order,
                                rice_bits: width,
                                parameter: k as u8,
                            };
                        }
                        // Q(k+1) = (Q(k) - number of set bits at position k) / 2.
                        // This is exact for the sum of individually shifted values.
                        quotients = (quotients - u64::from(self.rice.ones[k])) >> 1;
                    }
                }
                self.next_order();
            }
        }
        self.order > 4 || self.order as usize >= self.samples.len()
    }

    fn next_order(&mut self) {
        self.order += 1;
        self.index = self.order as usize;
        self.rice.clear();
    }

    pub fn result(&self) -> Result<Subframe, Error> {
        if self.best == Subframe::Constant
            || self.order > 4
            || self.order as usize >= self.samples.len()
        {
            Ok(self.best)
        } else {
            Err(Error::AnalysisIncomplete)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RiceStats;

    #[test]
    fn quotient_recurrence_matches_direct_sums_including_full_blocks() {
        for pattern in [0u32, 1, 2, 3, 0x8000_0000, 0xffff_fffe, 0xffff_ffff] {
            let mut stats = RiceStats::new();
            for _ in 0..4096 {
                stats.add(pattern);
            }
            let mut q = stats.sum;
            for k in 0..32 {
                assert_eq!(q, u64::from(pattern >> k) * 4096);
                q = (q - u64::from(stats.ones[k])) >> 1;
            }
        }
        let values = [0, 1, 127, 128, 129, 65535, 65536, 0x7fff_ffff, 0xffff_fffe];
        let mut stats = RiceStats::new();
        for value in values {
            stats.add(value);
        }
        let mut q = stats.sum;
        for k in 0..32 {
            assert_eq!(q, values.iter().map(|v| u64::from(v >> k)).sum::<u64>());
            q = (q - u64::from(stats.ones[k])) >> 1;
        }
    }
}
