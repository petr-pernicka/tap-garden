pub struct Costs {
    data: Vec<f64>,
    nodes: usize,
    time_steps: usize,
    destinations: usize,
}

impl Costs {
    pub fn new(init_value: f64, nodes: usize, time_steps: usize, destinations: usize) -> Self {
        debug_assert!(init_value >= 0.0);

        Self {
            data: vec![init_value; nodes * destinations * time_steps],
            nodes,
            time_steps,
            destinations,
        }
    }

    pub fn set_cost(&mut self, node: usize, tn: usize, destination: usize, cost: f64) {
        debug_assert!(node < self.nodes);
        debug_assert!(
            tn < self.time_steps,
            "Cost set for invalid time {tn}, must be lower than {}",
            self.time_steps
        );
        debug_assert!(destination < self.destinations);
        debug_assert!(cost >= 0.0);

        self.data[destination * self.time_steps * self.nodes + tn * self.nodes + node] = cost;
    }

    pub fn get_cost(&self, node: usize, tn: usize, destination: usize) -> f64 {
        debug_assert!(node < self.nodes);
        debug_assert!(tn < self.time_steps);
        debug_assert!(destination < self.destinations);

        self.data[destination * self.time_steps * self.nodes + tn * self.nodes + node]
    }
}
