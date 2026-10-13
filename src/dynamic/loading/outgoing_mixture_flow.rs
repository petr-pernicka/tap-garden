use crate::data::function::IntPiecewiseLinearFunction;
use crate::dynamic::Cvn;
use crate::dynamic::routing::MixtureFlow;

pub fn outgoing_mixture_flow(
    tn: usize,
    flow: f64,
    cinflow: &Cvn,
    coutflow: &Cvn,
    mixture_inflow: &IntPiecewiseLinearFunction<MixtureFlow>,
) -> MixtureFlow {
    debug_assert!(flow >= 0.);

    if flow < 1e-1 {
        return MixtureFlow::zero();
    }
    if tn == 0 {
        return MixtureFlow::zero();
    }

    // this method occurs in the instant before t-th time step, since it retrieves
    // the mixture of sending flow, which itself is computedin the same instant

    // This is the point at the very front of the link
    let outflow1 = coutflow[tn];
    // This is the point at the back of the flow
    let outflow2 = coutflow[tn] + flow;

    // The time at which outflow2 entered the link: cumulativeInflow[t2] == outflow2
    let t2 = time_of_entering(outflow2, &cinflow, tn);
    // The time at which outflow1 entered the link: cumulativeInflow[t1] == outflow1
    let t1 = time_of_entering(outflow1, &cinflow, t2 as usize);

    // It is guaranteed that t1 < t2 and t1 == t2 only if flow == 0 (and that is handled above)
    debug_assert!(t1 < t2);

    // Create the mixture flow of the `flow` amount of flow at the beggining of the
    // link by summing `inflow` over the time interval [t1, t2].
    let t1_trunc = t1 as usize;
    let t2_trunc = t2 as usize;

    if t1_trunc == t2_trunc {
        // All flow arrived in the same time step
        return mixture_inflow[t1_trunc].copy_with_flow(flow);
    }

    let t1_fract = t1.fract();
    let t2_fract = t2.fract();

    let mut result = &mixture_inflow[t1_trunc] * (1. - t1_fract);
    if t2_fract != 0. {
        result = &result + &(&mixture_inflow[t2_trunc] * t2_fract);
    }
    if t1_trunc + 1 < t2_trunc {
        for t in t1_trunc + 1..t2_trunc {
            result = &result + &mixture_inflow[t];
        }
    }

    debug_assert_eq!(result.total_flow(), flow);
    MixtureFlow::zero()
}

fn time_of_entering(outflow: f64, cinflow: &Cvn, start: usize) -> f64 {
    for t in (0..=start).rev() {
        if (cinflow[t] - outflow).abs() < 1e-10 {
            // found time is integer (or close enough)
            return t as f64;
        } else if cinflow[t] < outflow {
            // found time is not integer -> must interpolate
            let a = cinflow[t];
            let b = cinflow[t + 1];

            // t + p is the found time
            let p = (outflow - a) / (b - a);
            return t as f64 + p;
        }
    }

    0.
}
