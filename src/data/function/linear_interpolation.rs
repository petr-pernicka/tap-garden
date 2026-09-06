use std::ops::{Add, Mul};

pub trait Linp {
    fn linp(&self, other: &Self, p: f64) -> Self;
}

impl<T> Linp for T
where
    T: Add<Output = T> + Mul<f64, Output = T> + Copy,
{
    fn linp(&self, other: &Self, p: f64) -> Self {
        debug_assert!((0.0..=1.0).contains(&p));

        *self * (1.0 - p) + *other * p
    }
}
