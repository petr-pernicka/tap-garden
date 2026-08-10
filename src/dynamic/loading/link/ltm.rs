use crate::dynamic::loading::link::fd::TrapezoidalFd;
use crate::dynamic::loading::link::LinkModel;
use crate::util::Lirp;

pub struct Ltm {}

impl LinkModel for Ltm {
	
	fn sending_flow(&self, tn: usize, step_size: f64, fd: &TrapezoidalFd, cinflow: &[f64], coutflow: &[f64]) -> (f64, Option<f64>) {
		debug_assert!(tn < cinflow.len());
		
		let t0 = (
			(tn + 1) as f64 - fd.length() / (fd.free_flow_speed() * step_size)
		).max(0.0);
		
		let phi = t0.fract();
		let t1_floor = t0.trunc() as usize;
		let interpolated_inflow = cinflow[t1_floor].linp(&cinflow[t1_floor + 1], phi);
		
		(
			(interpolated_inflow - coutflow[tn])
				.clamp(0.0, fd.capacity() * step_size),
			Some(if t0 > tn as f64 { phi } else { 0.0 }),
		)
	}
	
	fn receiving_flow(&self, tn: usize, step_size: f64, fd: &TrapezoidalFd, cinflow: &[f64], coutflow: &[f64]) -> (f64, Option<f64>) {
		debug_assert!(tn < cinflow.len());
		
		let t0 = (
			(tn + 1) as f64 - fd.length() / (fd.backward_wave_speed() * step_size)
		).max(0.0);
		
		let psi = t0.fract();
		let t0_floor = t0.trunc() as usize;
		let interpolated_outflow = coutflow[t0_floor].linp(&coutflow[t0_floor + 1], psi);
		
		(
			(interpolated_outflow - cinflow[tn] + fd.jam_density() * fd.length())
				.clamp(0.0, fd.capacity() * step_size),
			Some(if t0 > tn as f64 { psi } else { 0.0 }),
		)
	}
}
