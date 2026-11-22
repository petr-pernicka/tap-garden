use crate::dynamic::equilibrium::DynamicUserEquilibrium;
use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::routing::MixtureOutgoingFractions;
use crate::dynamic::{Costs, Cvn, DynamicNetwork, OutgoingLinkIndices};
use std::marker::PhantomData;

pub struct MSA<FD> {
    fd: PhantomData<FD>,
    iteration: usize,
}

impl<FD> MSA<FD> {
    pub fn new() -> Self {
        Self {
            fd: PhantomData,
            iteration: 0,
        }
    }
}

impl<FD: FundamentalDiagram> DynamicUserEquilibrium<MixtureOutgoingFractions, FD> for MSA<FD> {
    fn compute_routing_parameters(
        &mut self,
        network: &DynamicNetwork<MixtureOutgoingFractions, FD>,
        time_steps: usize,
        mofs: MixtureOutgoingFractions,
        indices: &OutgoingLinkIndices,
        _costs: &Costs,
        _step_size: f64,
        _travel_times: &[Vec<f64>],
        _cumulative_flows: &[(Cvn, Cvn)],
        _free_flow_travel_time: &[f64],
    ) -> MixtureOutgoingFractions {
        let lambda = 1.0 / (self.iteration + 2) as f64;
        let mut mofs_new = MixtureOutgoingFractions::new(network, network.nodes(), time_steps);

        for n in network.routed_nodes() {
            for tn in 0..time_steps {
                for d in network.zones() {
                    let current_fractions = mofs.get_fractions(n, tn, d);
                    debug_assert!(current_fractions.len() == network.outgoing_links(n).len());

                    for (jk, fraction) in current_fractions.iter().enumerate() {
                        let is_link_on_shortest_path =
                            f64::from(indices.get_index(tn, n, d) == jk as u8);

                        let new_fraction =
                            (1.0 - lambda) * fraction + is_link_on_shortest_path * lambda;

                        mofs_new.set_fraction(n, tn, d, jk, new_fraction);
                    }
                }
            }

            mofs_new.compress_for_node(n);
        }

        self.iteration += 1;
        mofs_new
    }
}
