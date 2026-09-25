use std::rc::Rc;
use crate::dynamic::TimeDependentODM;
use crate::dynamic::loading::node::routing::NonRoutedModel;
use crate::dynamic::routing::MixtureFlow;

pub struct Zone {
    index: usize,
    odm: Rc<TimeDependentODM>,
}

impl Zone {
    pub fn new(index: usize, time_dependent_odm: Rc<TimeDependentODM>) -> Zone {
        Zone {
            index,
            odm: time_dependent_odm,
        }
    }

    fn create_mixture_flow_from_odm(&self, tn: usize) -> MixtureFlow {
        let mut origin_flow = 0.;
        for d in 0..self.odm.zones() {
            origin_flow += self.odm.demand(self.index, d, tn);
        }

        if origin_flow == 0. {
            return MixtureFlow::zero();
        }

        let mut destinations = Vec::with_capacity(self.odm.zones());
        let mut portions = Vec::with_capacity(self.odm.zones());
        for d in 0..self.odm.zones() {
            let flow = self.odm.demand(self.index, d, tn);

            if flow > 0. {
                destinations.push(d as u32);
                portions.push(flow / origin_flow);
            }
        }

        MixtureFlow::new(origin_flow, destinations, portions)
    }
}

impl NonRoutedModel for Zone {
    fn mixture_inflows_outflows(
        &self,
        mixture_sending_flows: &[MixtureFlow],
        receiving_flows: &[f64],
        tn: usize,
    ) -> (Vec<MixtureFlow>, Vec<MixtureFlow>) {
        debug_assert_eq!(mixture_sending_flows.len(), 1);
        debug_assert_eq!(receiving_flows.len(), 1);
        debug_assert!(receiving_flows[0] >= 0.);

        // All of the sending flow is received
        (vec![mixture_sending_flows[0].clone()], vec![self.create_mixture_flow_from_odm(tn)])
    }
}
