use crate::data::BitSet32;
use crate::dynamic::loading::node::routing::NonRoutedModel;
use crate::dynamic::routing::MixtureFlow;

pub struct Merge {
    priorities: Vec<f64>,
}

impl Merge {
    pub fn new(priorities: Vec<f64>) -> Self {
        Self { priorities }
    }
}

impl NonRoutedModel for Merge {
    fn mixture_inflows_outflows(
        &self,
        mixture_sending_flows: &[MixtureFlow],
        receiving_flows: &[f64],
        _tn: usize,
    ) -> (Vec<MixtureFlow>, Vec<MixtureFlow>) {
        debug_assert_eq!(receiving_flows.len(), 1);
        debug_assert!(!mixture_sending_flows.is_empty());

        let receiving_flow = receiving_flows[0];
        let total_sending_flow: f64 = mixture_sending_flows.iter().map(|x| x.total_flow()).sum();

        if total_sending_flow <= receiving_flow {
            // All incoming vehicles can pass.

            let mut mixture_outflow = MixtureFlow::zero();
            for mixture_sending_flow in mixture_sending_flows {
                mixture_outflow = &mixture_outflow + mixture_sending_flow;
            }

            (mixture_sending_flows.to_vec(), vec![mixture_outflow])
        } else {
            // Outgoing link cannot accomodate for all incoming flow.

            let mut remaining_receiving_flow = receiving_flow;
            let mut remaining_sending_flows = mixture_sending_flows
                .iter()
                .map(|x| x.total_flow())
                .collect::<Vec<_>>();

            let mut total_incoming_flows = vec![0.; mixture_sending_flows.len()];

            let mut active_incoming_links = BitSet32::new(mixture_sending_flows.len());
            for ij in 0..mixture_sending_flows.len() {
                if remaining_sending_flows[ij] == 0. {
                    active_incoming_links.set(ij);
                }
            }

            while !active_incoming_links.is_empty() {
                let mut alpha = vec![0.; mixture_sending_flows.len()];
                let mut total_alpha = 0.;
                for ij in active_incoming_links.iter_ones() {
                    alpha[ij] = self.priorities[ij];
                    total_alpha += self.priorities[ij];
                }

                let mut theta = remaining_receiving_flow / total_alpha;
                for ij in active_incoming_links.iter_ones() {
                    theta = theta.min(remaining_sending_flows[ij] / alpha[ij]);
                }

                for ij in active_incoming_links.iter_ones() {
                    total_incoming_flows[ij] += theta * alpha[ij];
                    remaining_sending_flows[ij] -= theta * alpha[ij];
                    remaining_receiving_flow -= theta * alpha[ij];
                }

                if remaining_receiving_flow == 0. {
                    break;
                }

                for (ij, remaining_sending_flow) in remaining_sending_flows.iter().enumerate() {
                    if *remaining_sending_flow == 0. {
                        active_incoming_links.clear(ij);
                    }
                }
            }

            let mut mixture_outflow = MixtureFlow::zero();

            let mut mixture_inflows = Vec::with_capacity(mixture_sending_flows.len());
            for ij in 0..mixture_sending_flows.len() {
                let mixture_inflow =
                    mixture_sending_flows[ij].copy_with_flow(total_incoming_flows[ij]);

                mixture_outflow = &mixture_outflow + &mixture_inflow;
                mixture_inflows.push(mixture_inflow);
            }

            (mixture_inflows, vec![mixture_outflow])
        }
    }
}
