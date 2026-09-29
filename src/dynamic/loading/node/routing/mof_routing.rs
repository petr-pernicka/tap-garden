use crate::data::Matrix;
use crate::dynamic::loading::node::model::NodeModel;
use crate::dynamic::loading::node::routing::routing_model::RoutedModel;
use crate::dynamic::routing::{MixtureFlow, MixtureOutgoingFractions};

pub struct MofRoutingModel {
    node_index: usize,
}

impl RoutedModel<MixtureOutgoingFractions> for MofRoutingModel {
    fn mixture_inflows_outflows(
        &self,
        mixture_sending_flows: &[MixtureFlow],
        receiving_flows: &[f64],
        tn: usize,
        mofs: &MixtureOutgoingFractions,
        node_model: &Box<dyn NodeModel>,
    ) -> (Vec<MixtureFlow>, Vec<MixtureFlow>) {
        // 1. Compute total turning fractions.
        let mut total_turning_fractions =
            Matrix::new(mixture_sending_flows.len(), receiving_flows.len(), 0.);

        for ij in 0..mixture_sending_flows.len() {
            for d in 0..mixture_sending_flows[ij].size() {
                let destination = mixture_sending_flows[ij].destination(d);
                let portion = mixture_sending_flows[ij].portion(d);

                for jk in 0..receiving_flows.len() {
                    total_turning_fractions[(ij, jk)] +=
                        portion * mofs.get_fraction(self.node_index, tn, destination as usize, jk);
                }
            }
        }

        // 2. Execute the specific node model, which computes the total inflows and total
        // outflows.
        let sending_flows = mixture_sending_flows
            .iter()
            .map(|x| x.total_flow())
            .collect::<Vec<_>>();

        let (total_inflows, total_outflows) = node_model.total_inflows_outflows(
            &total_turning_fractions,
            &sending_flows,
            receiving_flows,
        );

        // 3. Compute the mixture flows.
        let mixture_inflows = mixture_sending_flows
            .iter()
            .enumerate()
            .map(|(i, x)| x.copy_with_flow(total_inflows[i]))
            .collect::<Vec<_>>();

        let mut mixture_outflows = Vec::with_capacity(receiving_flows.len());
        for jk in 0..receiving_flows.len() {
            if total_outflows[jk] <= 0. {
                mixture_outflows.push(MixtureFlow::zero());
            }

            let mut destinations = Vec::with_capacity(mofs.destinations());
            let mut portions = Vec::with_capacity(mofs.destinations());

            let mut counters = vec![0isize; sending_flows.len()];
            for ij in 0..sending_flows.len() {
                if mixture_inflows[ij].total_flow() == 0. {
                    counters[ij] = -1;
                }
            }

            for d in 0..mofs.destinations() {
                let mut sum = 0.;

                for ij in 0..sending_flows.len() {
                    if counters[ij] == -1 {
                        continue;
                    }

                    if mixture_inflows[ij].destination(counters[ij] as usize) as usize == d {
                        sum += mixture_inflows[ij].portion(counters[ij] as usize)
                            * mixture_inflows[ij].total_flow()
                            * mofs.get_fraction(self.node_index, tn, d, jk);
                        counters[ij] += 1;
                    }

                    if counters[ij] as usize == mixture_inflows[ij].size() {
                        counters[ij] = -1;
                    }
                }

                if sum > 0. {
                    destinations.push(d as u32);
                    portions.push(sum / total_outflows[jk]);
                }
            }

            mixture_outflows.push(MixtureFlow::new(total_outflows[jk], destinations, portions))
        }

        (mixture_inflows, mixture_outflows)
    }
}
