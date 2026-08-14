use crate::dynamic::loading::link::diagram::{FundamentalDiagram, PiecewiseLinearFd, TrapezoidalFd};
use super::LinkModel;
use crate::util::Lirp;

pub struct Ltm {}

impl LinkModel<TrapezoidalFd> for Ltm {
	
	fn sending_flow(&self, tn: usize, step_size: f64, fd: &TrapezoidalFd, cinflow: &[f64], coutflow: &[f64]) -> (f64, Option<f64>) {
		debug_assert!(cinflow.len() == coutflow.len());
		debug_assert!(step_size > 0.);
		debug_assert!(tn < cinflow.len());
		
		let t0 = (
			(tn + 1) as f64 - fd.length() / (fd.free_flow_speed() * step_size)
		).max(0.0);
		
		let phi = t0.fract();
		let t0_floor = t0.trunc() as usize;
		let coutflow_tn_plus_1 = cinflow[t0_floor].linp(&cinflow[t0_floor + 1], phi);
		
		(
			(coutflow_tn_plus_1 - coutflow[tn])
				.clamp(0.0, fd.capacity() * step_size),
			Some(if t0 > tn as f64 { phi } else { 0.0 }),
		)
	}
	
	fn receiving_flow(&self, tn: usize, step_size: f64, fd: &TrapezoidalFd, cinflow: &[f64], coutflow: &[f64]) -> (f64, Option<f64>) {
		debug_assert!(cinflow.len() == coutflow.len());
		debug_assert!(step_size > 0.);
		debug_assert!(tn < cinflow.len());
		
		let t0 = (
			(tn + 1) as f64 - fd.length() / (fd.backward_wave_speed() * step_size)
		).max(0.0);
		
		let psi = t0.fract();
		let t0_floor = t0.trunc() as usize;
		let cinflow_tn_plus_1 = coutflow[t0_floor].linp(&coutflow[t0_floor + 1], psi)
			+ fd.jam_density() * fd.length();
		
		(
			(cinflow_tn_plus_1 - cinflow[tn])
				.clamp(0.0, fd.capacity() * step_size),
			Some(if t0 > tn as f64 { psi } else { 0.0 }),
		)
	}
}

impl LinkModel<PiecewiseLinearFd> for Ltm {
	
	fn sending_flow(&self, tn: usize, step_size: f64, fd: &PiecewiseLinearFd, cinflow: &[f64], coutflow: &[f64]) -> (f64, Option<f64>) {
		debug_assert!(cinflow.len() == coutflow.len());
		debug_assert!(step_size > 0.);
		debug_assert!(tn < cinflow.len());
		
		let mut min_coutflow_tn_plus_1 = f64::INFINITY;
		let mut min_t0 = -1.0;
		let mut min_phi = -1.0;
		
		for i in 0..fd.points().len()-1 {
			let [point, next_point] = fd.points()[i..=i+1] else { unreachable!() };
			
		    let wave_speed = (next_point.1 - point.1) / (next_point.0 - point.0);
			if wave_speed <= 0. {
				// the wave_speed == 0. case is handled by capping the returned value by
				// fd.capacity() * step_size
				break;
			}
			
			let t0 = (
				(tn + 1) as f64 - fd.length() / (wave_speed * step_size)
			).max(0.0);
			
			let (density, flow) = point;
			
			let phi = t0.fract();
			let t1_floor = t0.trunc() as usize;
			let coutflow_tn_plus_1 = cinflow[t1_floor].linp(&cinflow[t1_floor + 1], phi)
				+ (flow / wave_speed - density) * fd.length();
			
			if coutflow_tn_plus_1 < min_coutflow_tn_plus_1 {
				min_coutflow_tn_plus_1 = coutflow_tn_plus_1;
				min_t0 = t0;
				min_phi = phi;
			}
		}
		
		debug_assert!(min_coutflow_tn_plus_1.is_finite());
		
		(
			(min_coutflow_tn_plus_1 - coutflow[tn])
				.clamp(0.0, fd.capacity() * step_size),
			Some(if min_t0 > tn as f64 { min_phi } else { 0.0 }),
		)
	}
	
	fn receiving_flow(&self, tn: usize, step_size: f64, fd: &PiecewiseLinearFd, cinflow: &[f64], coutflow: &[f64]) -> (f64, Option<f64>) {
		debug_assert!(cinflow.len() == coutflow.len());
		debug_assert!(step_size > 0.);
		debug_assert!(tn < cinflow.len());
		
		let mut min_cinflow_tn_plus_1 = f64::INFINITY;
		let mut min_t0 = -1.0;
		let mut min_psi = -1.0;
		
		for i in fd.points().len()-2..0 {
			let [point, next_point] = fd.points()[i..=i+1] else { unreachable!() };
			
			let wave_speed = (next_point.1 - point.1) / (next_point.0 - point.0);
			if wave_speed >= 0. {
				// the wave_speed == 0. case is handled by capping the returned value by
				// fd.capacity() * step_size
				break;
			}
			
			let t0 = (
				(tn + 1) as f64 + fd.length() / (wave_speed * step_size)
			).max(0.0);
			
			let (density, flow) = point;
			
			let psi = t0.fract();
			let t0_floor = t0.trunc() as usize;
			let cinflow_tn_plus_1 = coutflow[t0_floor].linp(&coutflow[t0_floor + 1], psi)
				+ (flow / wave_speed + density) * fd.length();
			
			if cinflow_tn_plus_1 < min_cinflow_tn_plus_1 {
				min_cinflow_tn_plus_1 = cinflow_tn_plus_1;
				min_t0 = t0;
				min_psi = psi;
			}
		}
		
		debug_assert!(min_cinflow_tn_plus_1.is_finite());
		
		(
			(min_cinflow_tn_plus_1 - cinflow[tn])
				.clamp(0.0, fd.capacity() * step_size),
			Some(if min_t0 > tn as f64 { min_psi } else { 0.0 }),
		)
	}
}
