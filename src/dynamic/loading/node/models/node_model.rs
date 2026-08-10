use crate::data::Matrix;

pub trait NodeModel {
	
	fn total_inflows_outflows(&self, turning_fractions: &Matrix<f64>, sending_flows: &[f64], receiving_flows: &[f64]) -> (Vec<f64>, Vec<f64>);
}