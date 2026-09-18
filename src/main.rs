use tap_garden::data::function::{Increasing, IntPiecewiseLinearFunction, Unconstrained};
use tap_garden::dynamic::routing::MixtureFlow;

fn main() {
    let mut a: IntPiecewiseLinearFunction<MixtureFlow, Increasing> = IntPiecewiseLinearFunction::new(5);
    a.push_value(MixtureFlow::zero());
    let b = MixtureFlow::new(1., vec![1], vec![1.]);
    a.push_value(b);
}
