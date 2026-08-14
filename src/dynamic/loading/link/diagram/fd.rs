/// A trait for various shapes of fundamental diagrams.
pub trait FundamentalDiagram {
    fn length(&self) -> f64;

    fn capacity(&self) -> f64;

    fn jam_density(&self) -> f64;

    fn free_flow_speed(&self) -> f64;

    fn backward_wave_speed(&self) -> f64;
}
