use crate::dynamic::Cvn;
use crate::network::Network;

pub trait DnlScheme {
    fn run(&self, network: &Network, time_steps: usize, step_size: f64) -> Vec<(Cvn, Cvn)>;
}
