//! All things dynamic traffic assignment.
pub mod loading;
pub mod routing;
mod tdodm;
mod dynamic_network;
pub mod equilibrium;
pub mod tdsp;
mod outgoing_link_indices;
mod costs;

use crate::data::function::{Increasing, IntPiecewiseLinearFunction};
pub use tdodm::TimeDependentODM;
pub use dynamic_network::DynamicNetwork;
pub use outgoing_link_indices::OutgoingLinkIndices;
pub use costs::Costs;

/// Cumulative Vehicle Number (CVN). It is a real, increasing, piecewise-linear function
/// with break-points defined at integers - the instant normalized time.
pub type Cvn = IntPiecewiseLinearFunction<f64, Increasing>;
