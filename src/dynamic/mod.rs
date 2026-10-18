//! All things dynamic traffic assignment.
mod costs;
mod dynamic_network;
pub mod equilibrium;
pub mod loading;
mod outgoing_link_indices;
pub mod routing;
mod tdodm;
pub mod tdsp;
pub mod travel_time;
pub mod convergence;

use crate::data::function::{Increasing, IntPiecewiseLinearFunction};
pub use costs::Costs;
pub use dynamic_network::DynamicNetwork;
pub use outgoing_link_indices::OutgoingLinkIndices;
pub use tdodm::TimeDependentODM;

/// Cumulative Vehicle Number (CVN). It is a real, increasing, piecewise-linear function
/// with break-points defined at integers - the instant normalized time.
pub type Cvn = IntPiecewiseLinearFunction<f64, Increasing>;
