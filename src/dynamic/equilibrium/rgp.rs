use crate::dynamic::equilibrium::DynamicUserEquilibrium;
use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::routing::MixtureOutgoingFractions;
use crate::dynamic::tdsp::DOT;
use crate::dynamic::{Costs, Cvn, DynamicNetwork, OutgoingLinkIndices};
use std::marker::PhantomData;

const ETA_1: f64 = 2.0;
const ETA_2: f64 = 2.0 / 3.0;
const ETA_3: f64 = 1.0;
const RO: f64 = 1.0;

enum ScaleFactor {
    MinCost,
    Cost,
}

pub struct RGP<FD> {
    fd: PhantomData<FD>,
    eta_bad: usize,
    scale_factor: ScaleFactor,
}

impl<FD> RGP<FD> {
    pub fn new() -> Self {
        Self {
            fd: PhantomData,
            eta_bad: 0,
            scale_factor: ScaleFactor::MinCost,
        }
    }

    fn scale_factor(&self, min_cost: f64, cost: f64, alpha: f64) -> f64 {
        let numerator = match &self.scale_factor {
            ScaleFactor::MinCost => min_cost,
            ScaleFactor::Cost => cost,
        };

        numerator / (RO * alpha)
    }
    
    pub fn increment_eta_bad(&mut self, criterion_prev: f64, criterion_now: f64) {
        if criterion_now > ETA_3 * criterion_prev {
            self.eta_bad += 1;
        }
    }
}

impl<FD: FundamentalDiagram> DynamicUserEquilibrium<MixtureOutgoingFractions, FD> for RGP<FD> {
    fn compute_routing_parameters(
        &mut self,
        network: &DynamicNetwork<MixtureOutgoingFractions, FD>,
        time_steps: usize,
        mofs: MixtureOutgoingFractions,
        indices: &OutgoingLinkIndices,
        costs: &Costs,
        step_size: f64,
        travel_times: &[Vec<f64>],
        cumulative_flows: &[(Cvn, Cvn)],
        free_flow_travel_time: &[f64],
    ) -> MixtureOutgoingFractions {
        let alpha = (ETA_1 / (ETA_1 + self.eta_bad as f64)).powf(ETA_2);
        let mut mofs_new = MixtureOutgoingFractions::new(network, network.nodes(), time_steps);

        for n in network.routed_nodes() {
            for tn in 0..time_steps {
                for d in network.zones() {
                    let min_cost = costs.get_cost(n, tn, d);
                    if min_cost == 0.0 || min_cost == f64::INFINITY {
                        continue;
                    }

                    let shortest_path_link = indices.get_index(n, tn, d);

                    let mut sum = 0.0;
                    for (jk, outgoing_link) in network.outgoing_links(n).iter().enumerate() {
                        if jk == shortest_path_link as usize {
                            continue;
                        }

                        let cost = DOT::<FD>::compute_cost(
                            outgoing_link,
                            tn,
                            d,
                            costs,
                            time_steps,
                            step_size,
                            travel_times,
                            network,
                            cumulative_flows,
                            free_flow_travel_time,
                        );
	                    
	                    debug_assert!(cost >= min_cost);

                        let g = self.scale_factor(min_cost, cost, alpha);
                        let delta = (cost - min_cost) / (2.0 * g);

                        let new_value = (mofs.get_fraction(n, tn, d, jk) - delta).max(0.0);
                        mofs_new.set_fraction(n, tn, d, jk, new_value);

                        sum += new_value;
                    }

                    mofs_new.set_fraction(n, tn, d, shortest_path_link as usize, 1.0 - sum);
                }
            }

            mofs_new.compress_for_node(n);
        }

        mofs_new
    }
}
