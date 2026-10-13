use crate::data::{BitSet32, Matrix};
use crate::dynamic::loading::node::model::NodeModel;

pub struct TampereUnsignalized {
    priorities: Vec<f64>,
}

impl TampereUnsignalized {
    pub fn new(priorities: Vec<f64>) -> Self {
        debug_assert!(priorities.iter().all(|x| *x >= 0.));

        Self { priorities }
    }
}

impl NodeModel for TampereUnsignalized {
    #[allow(non_snake_case)]
    fn total_inflows_outflows(
        &self,
        turning_fractions: &Matrix<f64>,
        sending_flows: &[f64],
        receiving_flows: &[f64],
    ) -> (Vec<f64>, Vec<f64>) {
        debug_assert!(
            turning_fractions
                .data()
                .iter()
                .all(|x| (0.0..=1. + 1e-10).contains(x))
        );
        debug_assert_eq!(sending_flows.len(), turning_fractions.rows());
        debug_assert_eq!(sending_flows.len(), self.priorities.len());
        debug_assert!(sending_flows.iter().all(|x| *x >= 0.));
        debug_assert_eq!(receiving_flows.len(), turning_fractions.cols());
        debug_assert!(receiving_flows.iter().all(|x| *x >= 0.));

        let mut inflows = vec![0.; sending_flows.len()];
        let mut outflows = vec![0.; receiving_flows.len()];

        // 1. Retrieve link constraints and initialize supplies and sets.
        // Initialize oriented sending flow for each pair of incoming and outgoing link.
        let oriented_sending_flows = turning_fractions.mul_rows(sending_flows);

        // Initialize reduced receiving flow of each outgoing link.
        let mut R = receiving_flows.to_vec();

        // For each outgoing link jk, initialize set U_jk of all incoming links that
        // compete for R_jk.
        let mut U = vec![BitSet32::new(sending_flows.len()); receiving_flows.len()];

        // Set of outgoing links jk, towards which nonzero sending flow is directed.
        let mut J = BitSet32::new(receiving_flows.len());

        for jk in 0..receiving_flows.len() {
            for ij in 0..sending_flows.len() {
                if oriented_sending_flows[(ij, jk)] > 0. {
                    U[jk].set(ij);
                    J.set(jk);
                }
            }
        }

        // 2. Determine oriented capacities.
        let oriented_priorities = turning_fractions.mul_rows(&self.priorities);

        // Level of reduction
        let mut lor = vec![0.; receiving_flows.len()];

        while !J.is_empty() {
            // 3. Determine most restrictive constraint.
            let mut min_lor = f64::INFINITY;
            let mut min_lor_jk = None;
            for jk in J.iter_ones() {
                let mut denominator = 0.0;

                for ij in 0..sending_flows.len() {
                    if U[jk].get(ij) {
                        denominator += oriented_priorities[(ij, jk)];
                    }
                }

                lor[jk] = R[jk] / denominator;

                if lor[jk] <= min_lor {
                    min_lor = lor[jk];
                    min_lor_jk = Some(jk);
                }
            }

            debug_assert!(min_lor_jk.is_some());
            let min_lor_jk = min_lor_jk.unwrap();

            // 4. Determine flows of corresponding set U[min_lor_jk] and recalculate R_jk.
            // (a) At least one ij in U[min_lor_jk] is sending flow (demand) constrained.
            let mut any_demand_constrained = false;
            for ij in 0..sending_flows.len() {
                if U[min_lor_jk].get(ij) && sending_flows[ij] <= min_lor * self.priorities[ij] {
                    any_demand_constrained = true;

                    for jk in 0..receiving_flows.len() {
                        inflows[ij] += oriented_sending_flows[(ij, jk)];
                        outflows[jk] += oriented_sending_flows[(ij, jk)];
                    }

                    for jk in 0..receiving_flows.len() {
                        if J.get(jk) {
                            R[jk] -= oriented_sending_flows[(ij, jk)];
                            U[jk].clear(ij);

                            if U[jk].is_empty() {
                                lor[jk] = 1.;
                                J.clear(jk);
                            }
                        }
                    }
                }
            }

            if any_demand_constrained {
                continue;
            }

            // (b) All links of U[min_lor_jk] are constrained by receiving flow (supply)
            // of link min_lor_jk.
            for ij in 0..sending_flows.len() {
                if U[min_lor_jk].get(ij) {
                    for jk in 0..receiving_flows.len() {
                        inflows[ij] += min_lor * oriented_sending_flows[(ij, jk)];
                        outflows[jk] += min_lor * oriented_sending_flows[(ij, jk)];
                    }

                    for jk in 0..receiving_flows.len() {
                        if J.get(jk) {
                            R[jk] -= min_lor * oriented_sending_flows[(ij, jk)];

                            if jk != min_lor_jk {
                                let a = U[min_lor_jk];
                                U[jk].clear_all(&a);

                                if U[jk].is_empty() {
                                    lor[jk] = 1.;
                                    J.clear(jk);
                                }
                            } else {
                                lor[jk] = min_lor;
                                J.clear(jk);
                            }
                        }
                    }
                }
            }
        }

        assert!((inflows.iter().sum::<f64>() - outflows.iter().sum::<f64>()).abs() < 1e-10);
        (inflows, outflows)
    }
}
