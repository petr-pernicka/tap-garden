use crate::dynamic::{Cvn, DynamicNetwork};
use crate::dynamic::loading::link::diagram::FundamentalDiagram;

pub trait DnlScheme<RP, FD: FundamentalDiagram> {
    fn run(&self, network: &DynamicNetwork<RP, FD>, time_steps: usize, step_size: f64) -> Vec<(Cvn, Cvn)>;
}
