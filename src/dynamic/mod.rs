//! All things dynamic traffic assignment.
pub mod loading;
pub mod routing;
mod tdodm;

use crate::data::function::{Increasing, IntPiecewiseLinearFunction};
pub use tdodm::TimeDependentODM;

/// Cumulative Vehicle Number (CVN). It is a real, increasing, piecewise-linear function
/// with break-points defined at integers - the instant normalized time.
pub type Cvn = IntPiecewiseLinearFunction<f64, Increasing>;
