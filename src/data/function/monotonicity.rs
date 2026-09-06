/// This trait represents different kinds of monotonicity of functions. Having a
/// ```
/// M: Monotonicity
/// ```
/// as a generic parameter enables checks made by the function, which assert the specific
/// monotonicity. The function can also implement specific methods depending on its
/// specified monotonicity, such as the inverse function, which is only defined for
/// strictly increasing or decreasing functions.
///
/// The trait has a single associated function `check` which validates the monotonicity
/// for two consecutive y-values.
///
/// Be aware that whenever is function monotone (that is, it is anything but
/// [Unconstrained]), the type of y-values `Y` must implement [PartialOrd].
pub trait Monotonicity<Y> {
    fn check(prev: &Y, next: &Y) -> bool;
}

pub struct StrictlyIncreasing;

impl<Y: PartialOrd> Monotonicity<Y> for StrictlyIncreasing {
    fn check(prev: &Y, next: &Y) -> bool {
        prev < next
    }
}

pub struct Increasing;

impl<Y: PartialOrd> Monotonicity<Y> for Increasing {
    fn check(prev: &Y, next: &Y) -> bool {
        prev <= next
    }
}

pub struct Unconstrained;

impl<Y> Monotonicity<Y> for Unconstrained {
    fn check(_prev: &Y, _next: &Y) -> bool {
        true
    }
}

pub struct Decreasing;

impl<Y: PartialOrd> Monotonicity<Y> for Decreasing {
    fn check(prev: &Y, next: &Y) -> bool {
        prev >= next
    }
}

pub struct StrictlyDecreasing;

impl<Y: PartialOrd> Monotonicity<Y> for StrictlyDecreasing {
    fn check(prev: &Y, next: &Y) -> bool {
        prev > next
    }
}
