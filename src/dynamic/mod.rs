//! All things dynamic traffic assignment.
pub mod loading;
pub mod routing;
mod tdodm;
mod dynamic_network;
pub mod equilibrium;

use crate::data::function::{Increasing, IntPiecewiseLinearFunction};
pub use tdodm::TimeDependentODM;
pub use dynamic_network::DynamicNetwork;

/// Cumulative Vehicle Number (CVN). It is a real, increasing, piecewise-linear function
/// with break-points defined at integers - the instant normalized time.
pub type Cvn = IntPiecewiseLinearFunction<f64, Increasing>;
