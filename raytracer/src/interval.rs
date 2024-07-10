use std::ops::{Add, Index, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
    pub const UNIT: Self = Self { min: 0.0, max: 1.0 };
    // leave a small gap to avoid sticking in the same position
    pub const POSITIVE: Self = Self {
        min: 0.001,
        max: f64::INFINITY,
    };
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn min_max(a: f64, b: f64) -> Self {
        Interval::new(a.min(b), a.max(b))
    }

    pub fn length(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.min(self.max).max(self.min)
    }

    pub fn limit_max(&mut self, max: f64) {
        self.max = self.max.min(max);
    }

    pub fn limit_min(&mut self, min: f64) {
        self.min = self.min.max(min);
    }

    pub fn empty(&self) -> bool {
        self.min > self.max
    }

    pub fn intersect(self, other: Self) -> Self {
        Interval::new(self.min.max(other.min), self.max.min(other.max))
    }

    pub fn union(self, other: Self) -> Self {
        Interval::new(self.min.min(other.min), self.max.max(other.max))
    }

    pub fn include(self, other: f64) -> Self {
        Interval::new(self.min.min(other), self.max.max(other))
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, t: f64) -> Interval {
        Interval::new(self.min + t, self.max + t)
    }
}

impl Sub<f64> for Interval {
    type Output = Interval;

    fn sub(self, t: f64) -> Interval {
        Interval::new(self.min - t, self.max - t)
    }
}

impl Index<usize> for Interval {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.min,
            1 => &self.max,
            _ => panic!("Index out of bounds"),
        }
    }
}
