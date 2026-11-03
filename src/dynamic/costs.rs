use crate::network::NodeIndex;
use std::ops::Range;

pub struct Costs {
    data: Vec<f64>,
    used_nodes: Range<NodeIndex>,
    time_steps: usize,
    destinations: usize,
}

impl Costs {
    pub fn new(
        init_value: f64,
        used_nodes: Range<NodeIndex>,
        time_steps: usize,
        destinations: usize,
    ) -> Self {
        debug_assert!(init_value >= 0.0);

        Self {
            data: vec![init_value; used_nodes.len() * destinations * time_steps],
            used_nodes,
            time_steps,
            destinations,
        }
    }

    pub fn set_cost(&mut self, n: usize, tn: usize, d: usize, cost: f64) {
        debug_assert!(self.used_nodes.contains(&n));
        debug_assert!(
            tn < self.time_steps,
            "Cost set for invalid time {tn}, must be lower than {}",
            self.time_steps
        );
        debug_assert!(d < self.destinations);
        debug_assert!(cost >= 0.0);
        let n = n - self.used_nodes.start;

        self.data[d * self.time_steps * self.used_nodes.len() + tn * self.used_nodes.len() + n] =
            cost;
    }

    pub fn get_cost(&self, n: usize, tn: usize, d: usize) -> f64 {
        debug_assert!(self.used_nodes.contains(&n));
        debug_assert!(tn < self.time_steps);
        debug_assert!(d < self.destinations);
        let n = n - self.used_nodes.start;

        self.data[d * self.time_steps * self.used_nodes.len() + tn * self.used_nodes.len() + n]
    }
}
