//! Fundamental diagrams of all kinds.
//!
//! # Fundamental diagram
//!
//! Fundamental diagram (FD) is simply a function Q: R -> R. It maps density at point *k(x, t)* on
//! the link to flow at that point *q(x, t)*. It is one of the three equations defining
//! the LWR theory.
//!
//! [Link models][super::LinkModel] use the FD for computation of sending and receiving
//! flows.
//!
//! There are several requirements on the shape of the FD:
//! 1. It is defined on the interval *[0, k_j]*, where *k_j* is the jam density.
//! 2. It is non-negative and concave.
//! 3. It has two zeros, one at *k=0* and one at *k=k_j*.

mod fd;
mod piecewise_linear;
mod trapezoidal;

pub use fd::FundamentalDiagram;
pub use piecewise_linear::PiecewiseLinearFd;
pub use trapezoidal::TrapezoidalFd;
pub use trapezoidal::OmitParam;
