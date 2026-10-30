use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::routing::MixtureOutgoingFractions;
use crate::dynamic::{Costs, DynamicNetwork, OutgoingLinkIndices};
use crate::network::{Link, Network};
use crate::static_::sssp::SSSP;
use std::marker::PhantomData;

pub struct DOT<FD> {
    fd: PhantomData<FD>,
}

impl<FD: FundamentalDiagram> DOT<FD> {
    pub fn new() -> Self {
        Self { fd: PhantomData }
    }

    pub fn run<const SSSP: bool>(
        &self,
        network: &DynamicNetwork<MixtureOutgoingFractions, FD>,
        time_steps: usize,
        step_size: f64,
        travel_times: &[Vec<f64>],
    ) -> (Costs, OutgoingLinkIndices) {
        debug_assert!(step_size > 0.0);
        debug_assert_eq!(travel_times.len(), network.links().len());
        debug_assert!(travel_times.iter().all(|tt| tt.len() == time_steps + 1));

        // 1. Initialize all costs to infinity, initialize shortest path indices
        let mut costs = Costs::new(
            f64::INFINITY,
            network.nodes().len(),
            time_steps,
            network.zones().len(),
        );

        let mut indices =
            OutgoingLinkIndices::new(network.nodes().len(), time_steps, network.zones().len());

        // 2. Initialize costs from destinations to themselves to 0
        // for tn in 0..time_steps {
        //     for destination in network.zones() {
        //         costs.set_cost(destination, tn, destination, 0.0);
        //     }
        // }

        // 3. (Optional) Initialize costs at the last time step
        if SSSP {
            Self::run_sssp(network, travel_times, time_steps, &mut indices, &mut costs);
        }

        // 4. Set the rest of the values in decreasing order of time
        for destination in network.zones() {
            for tn in (0..=time_steps - 2).rev() {
                for node in network.nodes() {
                    for (i, jk) in network.outgoing_links(node).iter().enumerate() {
                        let new_cost = Self::compute_cost(
                            jk,
                            tn,
                            destination,
                            &costs,
                            time_steps,
                            step_size,
                            travel_times,
                            network
                        );
                        
                        if new_cost < costs.get_cost(node, tn, destination) {
                            costs.set_cost(node, tn, destination, new_cost);
                            indices.set_index(node, tn, destination, i as u8);
                        }
                    }
                }
            }
        }

        (costs, indices)
    }

    /// Computes the travel time from link tail to destination, when departing on the
    /// link at tn.
    fn compute_cost(
        link: &Link,
        tn: usize,
        destination: usize,
        costs: &Costs,
        time_steps: usize,
        step_size: f64,
        travel_times: &[Vec<f64>],
        network: &Network,
    ) -> f64 {
        let travel_time = travel_times[link.index()][tn + 1];
        if travel_time == f64::INFINITY {
            return f64::INFINITY;
        }
        
        // Used in place of step 2. (cost at the destination is 0 for all tn)
        if link.head() == destination {
            return travel_time;
        }
        
        if network.outgoing_links(link.head()).len() == 1 {
            
        }

        let normalized_travel_time = travel_time / step_size;
        // debug_assert!(
        //     normalized_travel_time >= 1.0,
        //     "DOT algorithm does not support violated CFL condition: {}",
        //     normalized_travel_time
        // );

        let normalized_travel_time_rounded = normalized_travel_time.round() as usize;

        // The (normalized rounded) time at which a vehicle departing on the link at tn
        // would arrive at the end of the link
        let time_of_arrival = (tn + normalized_travel_time_rounded).min(time_steps - 1);
        travel_time + costs.get_cost(link.head(), time_of_arrival, destination)
    }

    fn run_sssp(
        network: &DynamicNetwork<MixtureOutgoingFractions, FD>,
        travel_times: &[Vec<f64>],
        time_steps: usize,
        indices: &mut OutgoingLinkIndices,
        costs: &mut Costs,
    ) {
        let mut sssp = SSSP::new(network);

        let last_step_travel_times = travel_times
            .iter()
            .map(|tt| *tt.last().unwrap())
            .collect::<Vec<_>>();

        for destination in network.zones() {
            sssp.all_to_one(destination, &last_step_travel_times);

            for node in network.nodes() {
                if sssp.tree()[node] == usize::MAX {
                    continue;
                }

                let outgoing_index = network
                    .outgoing_links(node)
                    .iter()
                    .position(|link| link.index() == sssp.tree()[node])
                    .unwrap();

                indices.set_index(node, time_steps - 1, destination, outgoing_index as u8);
                costs.set_cost(node, time_steps - 1, destination, sssp.distance()[node]);
            }
        }
    }
}
