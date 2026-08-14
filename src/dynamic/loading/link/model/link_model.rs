use crate::dynamic::loading::link::diagram::FundamentalDiagram;

pub trait LinkModel<FD: FundamentalDiagram> {
    fn sending_flow(
        &self,
        tn: usize,
        step_size: f64,
        fd: &FD,
        cinflow: &[f64],
        coutflow: &[f64],
    ) -> (f64, Option<f64>);

    fn receiving_flow(
        &self,
        tn: usize,
        step_size: f64,
        fd: &FD,
        cinflow: &[f64],
        coutflow: &[f64],
    ) -> (f64, Option<f64>);
}
