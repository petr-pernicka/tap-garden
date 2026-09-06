use crate::dynamic::loading::node::routing::NonRoutedModel;
use crate::dynamic::routing::MixtureFlow;

pub struct Linear {}

impl NonRoutedModel for Linear {
    fn mixture_inflows_outflows(
        &self,
        mixture_sending_flows: &[MixtureFlow],
        receiving_flows: &[f64],
        _tn: usize,
    ) -> (Vec<MixtureFlow>, Vec<MixtureFlow>) {
        debug_assert_eq!(mixture_sending_flows.len(), 1);
        debug_assert_eq!(receiving_flows.len(), 1);
        debug_assert!(receiving_flows[0] >= 0.);

        let flow = receiving_flows[0].min(mixture_sending_flows[0].total_flow());
        let mixture_flow = mixture_sending_flows[0].copy_with_flow(flow);

        (
            vec![mixture_flow.clone()],
            vec![mixture_flow],
        )
    }
}
