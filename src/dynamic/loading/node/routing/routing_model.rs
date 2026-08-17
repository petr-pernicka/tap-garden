use crate::dynamic::loading::node::model::NodeModel;
use crate::dynamic::routing::MixtureFlow;

pub trait RoutingModel<RP> {
    fn mixture_inflows_outflows(
        &self,
        routing_paramenters: &RP,
        mixture_sending_flows: &[MixtureFlow],
        receiving_flows: &[f64],
        tn: usize,
        node_model: Box<dyn NodeModel>,
    ) -> (Vec<MixtureFlow>, Vec<MixtureFlow>);
}
