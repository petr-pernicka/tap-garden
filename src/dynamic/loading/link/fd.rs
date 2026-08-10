pub struct TrapezoidalFd {
	length: f64,
	capacity: f64,
	jam_density: f64,
	free_flow_speed: f64,
	backward_wave_speed: f64,
}

impl TrapezoidalFd {
	
	pub fn new(length: f64, capacity: f64, jam_density: f64, free_flow_speed: f64, backward_wave_speed: f64) -> Self {
		debug_assert!(length >= 0.0);
		debug_assert!(capacity >= 0.0);
		debug_assert!(jam_density >= 0.0);
		debug_assert!(free_flow_speed >= 0.0);
		debug_assert!(backward_wave_speed >= 0.0);
		
		Self {
			length, capacity, jam_density, free_flow_speed, backward_wave_speed
		}
	}
	
	pub fn length(&self) -> f64 {
		self.length
	}
	
	pub fn capacity(&self) -> f64 {
		self.capacity
	}
	
	pub fn jam_density(&self) -> f64 {
		self.jam_density
	}
	
	pub fn free_flow_speed(&self) -> f64 {
		self.free_flow_speed
	}
	
	pub fn backward_wave_speed(&self) -> f64 {
		self.backward_wave_speed
	}
}