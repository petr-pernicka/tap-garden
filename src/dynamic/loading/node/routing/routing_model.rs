use crate::dynamic::routing::MixtureFlow;

pub trait RoutingModel {
    fn mixture_inflows_outflows(&self) -> (Vec<MixtureFlow>, Vec<MixtureFlow>);
}