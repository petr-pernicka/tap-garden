use crate::dynamic::equilibrium::DynamicUserEquilibrium;
use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::routing::MixtureOutgoingFractions;
use crate::dynamic::{Costs, DynamicNetwork, OutgoingLinkIndices};
use std::marker::PhantomData;

pub struct MSA<RP, FD> {
    rp: PhantomData<RP>,
    fd: PhantomData<FD>,
    iteration: usize,
}

impl<RP, FD> MSA<RP, FD> {
    pub fn new() -> Self {
        Self {
            rp: PhantomData,
            fd: PhantomData,
            iteration: 0,
        }
    }
}

impl<RP, FD: FundamentalDiagram> DynamicUserEquilibrium<RP, FD> for MSA<RP, FD> {
    fn compute_routing_parameters(
        &mut self,
        network: &DynamicNetwork<RP, FD>,
        time_steps: usize,
        mofs: MixtureOutgoingFractions,
        indices: &OutgoingLinkIndices,
        _costs: &Costs,
    ) -> MixtureOutgoingFractions {
        let lambda = 1.0 / (self.iteration + 2) as f64;
        let mut mofs_new = MixtureOutgoingFractions::new(network, time_steps);

        for n in network.nodes() {
            for tn in 0..time_steps {
                for d in network.zones() {
                    for jk in 0..network.outgoing_links(n).len() {
                        let current_fraction = mofs.get_fraction(n, tn, d, jk);

                        let is_link_on_shortest_path =
                            f64::from(indices.get_index(n, tn, d) == jk as u8);

                        let new_fraction =
                            (1.0 - lambda) * current_fraction + is_link_on_shortest_path * lambda;

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
