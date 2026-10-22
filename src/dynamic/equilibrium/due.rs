use crate::dynamic::{Costs, DynamicNetwork, OutgoingLinkIndices};
use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::routing::MixtureOutgoingFractions;

pub trait DynamicUserEquilibrium<RP, FD: FundamentalDiagram> {
    fn compute_routing_parameters(
        &mut self,
        network: &DynamicNetwork<RP, FD>,
        time_steps: usize,
        mofs: MixtureOutgoingFractions,
        indices: &OutgoingLinkIndices,
        costs: &Costs,
    ) -> MixtureOutgoingFractions;
}