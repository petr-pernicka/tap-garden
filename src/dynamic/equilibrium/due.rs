use crate::dynamic::{Costs, Cvn, DynamicNetwork, OutgoingLinkIndices};
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
        step_size: f64,
        travel_times: &[Vec<f64>],
        cumulative_flows: &[(Cvn, Cvn)],
        free_flow_travel_time: &[f64],
    ) -> MixtureOutgoingFractions;
}