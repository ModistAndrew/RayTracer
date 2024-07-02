use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
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
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn min_max(a: f64, b: f64) -> Self {
        Interval::new(a.min(b), a.max(b))
    }

    pub fn size(&self) -> f64 {
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

    pub fn intersect(&mut self, other: Self) {
        self.min = self.min.max(other.min);
        self.max = self.max.min(other.max);
    }

    pub fn union(&mut self, other: Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }

    pub fn empty(&self) -> bool {
        self.min > self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}
