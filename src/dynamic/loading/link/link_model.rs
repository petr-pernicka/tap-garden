use crate::dynamic::loading::link::fd::TrapezoidalFd;

pub trait LinkModel {
    fn sending_flow(&self, tn: usize, step_size: f64, fd: &TrapezoidalFd, cinflow: &[f64], coutflow: &[f64]) -> (f64, Option<f64>);
    
    fn receiving_flow(&self, tn: usize, step_size: f64, fd: &TrapezoidalFd, cinflow: &[f64], coutflow: &[f64]) -> (f64, Option<f64>);
}