use crate::data::function::linear_interpolation::Linp;
use crate::data::function::monotonicity::{Monotonicity, Unconstrained};
use std::marker::PhantomData;
use std::ops::{Add, Index};

pub struct IntPiecewiseLinearFunction<Y, M = Unconstrained>
where
    Y: Linp<Output = Y>,
    M: Monotonicity<Y>,
{
    data: Vec<Y>,
    monotonicity: PhantomData<M>,
}

impl<Y, M> IntPiecewiseLinearFunction<Y, M>
where
    Y: Linp<Output = Y>,
    M: Monotonicity<Y>,
{
    pub fn new(max_x: usize) -> Self {
        Self {
            data: Vec::with_capacity(max_x),
            monotonicity: PhantomData,
        }
    }

    pub fn num_points(&self) -> usize {
        self.data.len()
    }

    pub fn push_value(&mut self, y: Y) {
        debug_assert!(self.data.len() < self.data.capacity());
        debug_assert!(
            self.data.is_empty()
                || (!self.data.is_empty() && M::check(self.data.last().unwrap(), &y))
        );

        self.data.push(y);
    }

    pub fn value_interpolate(&self, x: f64) -> (Y, f64) {
        debug_assert!(x >= 0.);
        debug_assert!((x.trunc() as usize + 1) < self.data.len());

        let a = x.trunc();
        let p = x - a;
        let a = a as usize;
        let b = a + 1;

        (self.data[a].linp(&self.data[b], p), p)
    }

    pub fn map<Z, F>(&self, f: F) -> IntPiecewiseLinearFunction<Z, Unconstrained>
    where
        Z: Linp<Output = Z>,
        F: FnMut(&Y) -> Z,
    {
        IntPiecewiseLinearFunction {
            data: self.data.iter().map(f).collect(),
            monotonicity: PhantomData,
        }
    }
}

impl<Y, M> IntPiecewiseLinearFunction<Y, M>
where
    Y: Linp<Output = Y> + Add<Output = Y>,
    M: Monotonicity<Y>,
{
    pub fn integrate(&self) -> Y {
        debug_assert!(self.data.len() > 1);

        let mut result = self.data[0].linp(&self.data[1], 0.5);

        for i in 2..self.data.len() {
            result = result + self.data[i - 1].linp(&self.data[i], 0.5);
        }

        result
    }
}

impl<Y, M> Index<usize> for IntPiecewiseLinearFunction<Y, M>
where
    Y: Linp<Output = Y>,
    M: Monotonicity<Y>,
{
    type Output = Y;

    fn index(&self, x: usize) -> &Self::Output {
        debug_assert!(x < self.data.len());

        &self.data[x]
    }
}
