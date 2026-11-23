use crate::network::NodeIndex;
use std::ops::Range;
use get_size2::GetSize;

#[derive(GetSize)]
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

    pub fn set_cost(&mut self, tn: usize, n: usize, d: usize, cost: f64) {
        debug_assert!(
            tn < self.time_steps,
            "Cost set for invalid time {tn}, must be lower than {}",
            self.time_steps
        );
        debug_assert!(self.used_nodes.contains(&n));
        debug_assert!(d < self.destinations);
        debug_assert!(cost >= 0.0);
        let n = n - self.used_nodes.start;

        self.data[tn * self.destinations * self.used_nodes.len() + n * self.destinations + d] =
            cost;
    }

    pub fn get_cost(&self, tn: usize, n: usize, d: usize) -> f64 {
        debug_assert!(tn < self.time_steps);
        debug_assert!(self.used_nodes.contains(&n));
        debug_assert!(d < self.destinations);
        let n = n - self.used_nodes.start;

        self.data[tn * self.destinations * self.used_nodes.len() + n * self.destinations + d]
    }
}
