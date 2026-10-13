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

/// A bounded search that can be advanced between executor yields.
pub struct Analysis<'a> {
    samples: &'a [i32],
    depth: u8,
    order: u8,
    index: usize,
    costs: [u64; 31],
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
            costs: [0; 31],
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
            let u = fold(r);
            for (k, cost) in self.costs.iter_mut().enumerate() {
                *cost = (*cost + (u >> k) + 1 + k as u64).min(self.best_bytes * 8);
            }
            self.index += 1;
            if self.index == self.samples.len() {
                // Evaluate 4-bit method first, then 5-bit: ties retain the simpler form.
                for width in [4u8, 5] {
                    let maximum = if width == 4 { 14 } else { 30 };
                    for k in 0..=maximum {
                        let cost = 8
                            + u64::from(self.order) * u64::from(self.depth)
                            + 2
                            + 4
                            + u64::from(width)
                            + self.costs[k];
                        let bytes = cost.div_ceil(8);
                        if bytes < self.best_bytes {
                            self.best_bytes = bytes;
                            self.best = Subframe::Fixed {
                                order: self.order,
                                rice_bits: width,
                                parameter: k as u8,
                            };
                        }
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
        self.costs.fill(0);
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
