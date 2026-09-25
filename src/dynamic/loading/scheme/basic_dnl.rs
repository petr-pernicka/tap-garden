use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::loading::scheme::DnlScheme;
use crate::dynamic::{Cvn, DynamicNetwork};

pub struct BasicDnl {}

impl<RP, FD: FundamentalDiagram> DnlScheme<RP, FD> for BasicDnl {
    fn run(
        &self,
        network: &DynamicNetwork<RP, FD>,
        time_steps: usize,
        step_size: f64,
    ) -> Vec<(Cvn, Cvn)> {
        let vec_of_cvn = Vec::with_capacity(network.links().len());

        for t in 0..time_steps {}

        vec_of_cvn
    }
}
