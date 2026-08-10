pub trait LinkModel {
    fn sending_flow() -> f64;
    
    fn receiving_flow() -> f64;
}