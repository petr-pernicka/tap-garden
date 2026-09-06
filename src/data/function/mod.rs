mod linear_interpolation;
mod monotonicity;
mod piecewise_linear;

pub use linear_interpolation::Linp;
pub use piecewise_linear::IntPiecewiseLinearFunction;
pub use monotonicity::{
    Decreasing, Increasing, StrictlyDecreasing, StrictlyIncreasing, Unconstrained,
};
