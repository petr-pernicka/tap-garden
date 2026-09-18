use std::ops::{Add, Mul};

pub trait Linp {
    type Output;

    fn linp(&self, other: &Self, p: f64) -> Self::Output;
}

impl<T> Linp for T
where
        for<'a> &'a T: Add<Output = T> + Mul<f64, Output = T>
{
    type Output = T;

    fn linp(&self, other: &Self, p: f64) -> Self::Output {
        debug_assert!((0.0..=1.0).contains(&p));

        let a = self * (1.0 - p);
        let b = other * p;
        &a + &b
    }
}
