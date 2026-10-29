use crate::dynamic::DynamicNetwork;
use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::routing::MixtureOutgoingFractions;
use crate::static_::sssp::SSSP;
use std::marker::PhantomData;

pub struct AON<RP, FD> {
    rp: PhantomData<RP>,
    fd: PhantomData<FD>,
}

impl<RP, FD: FundamentalDiagram> AON<RP, FD> {
    pub fn compute_routing_parameters(
        network: &DynamicNetwork<RP, FD>,
        link_costs: &[f64],
        time_steps: usize,
    ) -> MixtureOutgoingFractions {
        debug_assert_eq!(link_costs.len(), network.links().len());

        let mut sssp = SSSP::new(network);
        let mut successor_trees = Vec::with_capacity(network.zones().len());
        for destination in network.zones() {
            sssp.all_to_one(destination, link_costs);
            successor_trees.push(sssp.tree().to_vec());
        }

        let mut mofs = MixtureOutgoingFractions::new(network, network.nodes(), time_steps);
        for node in network.nodes() {
            for destination in network.zones() {

                let j = if node == destination {
                    0 // Turning params are unused on zones
                } else {
                    // May panic for not strongly-connected graphs (successor tree will
                    // contain usize::MAX)
                    network
                        .outgoing_links(node)
                        .iter()
                        .position(|link| link.index() == successor_trees[destination][node])
                        .unwrap()
                };

                for tn in 0..time_steps {
                    mofs.set_fraction(node, tn, destination, j, 1.0);
                }
            }

            mofs.compress_for_node(node);
        }

        mofs
    }
}
