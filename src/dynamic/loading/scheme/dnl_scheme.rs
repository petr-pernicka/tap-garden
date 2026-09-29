use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::{Cvn, DynamicNetwork};

pub trait DnlScheme<RP, FD: FundamentalDiagram> {
    fn run(
        &self,
        network: &DynamicNetwork<RP, FD>,
        time_steps: usize,
        step_size: f64,
        routing_parameters: &RP,
    ) -> Vec<(Cvn, Cvn)>;
}
