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

/// Experimental choices deliberately excluded from production builds.
#[cfg(feature = "benchmark")]
pub mod benchmark {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RiceMethod {
        Auto,
        Four,
        Five,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Search {
        BitCounts,
        Exhaustive,
        MeanNeighbors,
        MeanNeighborsFused,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Options {
        /// None searches all orders; Some forces one order, with bounded fallback.
        pub order: Option<u8>,
        pub rice: RiceMethod,
        pub search: Search,
    }
}

#[cfg(feature = "benchmark")]
impl Analysis<'_> {
    pub(crate) fn select_variant(&mut self, options: benchmark::Options) -> Result<(), Error> {
        use benchmark::{RiceMethod, Search};
        if options.order.is_some_and(|order| order > 4) {
            return Err(Error::InvalidConfig);
        }
        if self.best == Subframe::Constant {
            return Ok(());
        }
        for order in 0..=4u8 {
            if options.order.is_some_and(|forced| forced != order)
                || order as usize >= self.samples.len()
            {
                continue;
            }
            self.rice.clear();
            let mut quotients = [0u64; 31];
            let mut valid = true;
            for index in order as usize..self.samples.len() {
                let r = residual(self.samples, index, order);
                if !(-2147483647..=2147483647).contains(&r) {
                    valid = false;
                    break;
                }
                let u = fold(r);
                match options.search {
                    Search::BitCounts => self.rice.add(u as u32),
                    Search::Exhaustive => {
                        let parameters = if options.rice == RiceMethod::Four {
                            15
                        } else {
                            31
                        };
                        for (k, quotient) in quotients[..parameters].iter_mut().enumerate() {
                            *quotient += u >> k;
                        }
                    }
                    Search::MeanNeighbors | Search::MeanNeighborsFused => self.rice.sum += u,
                }
            }
            if !valid {
                continue;
            }
            let count = (self.samples.len() - order as usize) as u64;
            let mean = self.rice.sum / count;
            let estimate = if mean == 0 { 0 } else { mean.ilog2() as usize };
            if options.search == Search::MeanNeighborsFused {
                // Collect the union of both methods' neighboring candidates,
                // then evaluate all of them in one residual pass. The first
                // pass already proved every folded residual fits in u32.
                let mut candidates = 0u32;
                for width in [4u8, 5] {
                    if (options.rice == RiceMethod::Four && width != 4)
                        || (options.rice == RiceMethod::Five && width != 5)
                    {
                        continue;
                    }
                    let maximum = if width == 4 { 14 } else { 30 };
                    let center = estimate.min(maximum);
                    for k in center.saturating_sub(1)..=(center + 1).min(maximum) {
                        candidates |= 1 << k;
                    }
                }
                for index in order as usize..self.samples.len() {
                    let u = fold(residual(self.samples, index, order)) as u32;
                    let mut remaining = candidates;
                    while remaining != 0 {
                        let k = remaining.trailing_zeros() as usize;
                        quotients[k] += u64::from(u >> k);
                        remaining &= remaining - 1;
                    }
                }
            }
            for width in [4u8, 5] {
                if (options.rice == RiceMethod::Four && width != 4)
                    || (options.rice == RiceMethod::Five && width != 5)
                {
                    continue;
                }
                let maximum = if width == 4 { 14 } else { 30 };
                let center = estimate.min(maximum);
                let mut q = self.rice.sum;
                for k in 0..=maximum {
                    let quotient = match options.search {
                        Search::BitCounts => {
                            let current = q;
                            q = (q - u64::from(self.rice.ones[k])) >> 1;
                            current
                        }
                        Search::Exhaustive => quotients[k],
                        Search::MeanNeighbors | Search::MeanNeighborsFused => {
                            if k < center.saturating_sub(1) || k > (center + 1).min(maximum) {
                                continue;
                            }
                            if options.search == Search::MeanNeighborsFused {
                                quotients[k]
                            } else {
                                self.samples[order as usize..]
                                    .iter()
                                    .enumerate()
                                    .map(|(i, _)| {
                                        fold(residual(self.samples, i + order as usize, order)) >> k
                                    })
                                    .sum()
                            }
                        }
                    };
                    let cost = 8
                        + u64::from(order) * u64::from(self.depth)
                        + 2
                        + 4
                        + u64::from(width)
                        + quotient
                        + count * (1 + k as u64);
                    let bytes = cost.div_ceil(8);
                    if bytes < self.best_bytes {
                        self.best_bytes = bytes;
                        self.best = Subframe::Fixed {
                            order,
                            rice_bits: width,
                            parameter: k as u8,
                        };
                    }
                }
            }
        }
        self.order = 5;
        Ok(())
    }
}
