pub trait Lirp {
	fn linp(&self, other: &Self, p: f64) -> Self;
}

impl Lirp for f64 {
	fn linp(&self, other: &Self, p: f64) -> Self {
		debug_assert!((0.0..=1.0).contains(&p));
		
		(1.0 - p) * self + p * other
	}
}