use crate::dynamic::Cvn;
use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::loading::link::model::LinkModel;

pub struct Connector;

impl<FD: FundamentalDiagram> LinkModel<FD> for Connector {
    fn sending_flow(
        &self,
        tn: usize,
        _step_size: f64,
        _fd: &FD,
        cinflow: &Cvn,
        coutflow: &Cvn,
    ) -> (f64, Option<f64>) {
        ((cinflow[tn] - coutflow[tn]).max(0.0), None)
    }

    fn receiving_flow(
        &self,
        _tn: usize,
        _step_size: f64,
        _fd: &FD,
        _cinflow: &Cvn,
        _coutflow: &Cvn,
    ) -> (f64, Option<f64>) {
        (f64::INFINITY, None)
    }
}
