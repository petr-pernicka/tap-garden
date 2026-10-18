pub struct Costs {
    data: Vec<f64>,
    nodes: usize,
    destinations: usize,
    time_steps: usize,
}

impl Costs {
    pub fn new(init_value: f64, nodes: usize, destinations: usize, time_steps: usize) -> Self {
        debug_assert!(init_value >= 0.0);
        
        Self {
            data: vec![init_value; nodes * destinations * time_steps],
            nodes,
            destinations,
            time_steps,
        }
    }
    
    pub fn set_cost(&mut self, node: usize, tn: usize, destination: usize, cost: f64) {
        debug_assert!(node < self.nodes);
        debug_assert!(tn < self.time_steps);
        debug_assert!(destination < self.destinations);
        debug_assert!(cost > 0.0);

        self.data[(destination * self.time_steps + tn) * self.nodes + node] = cost;
    }
    
    pub fn get_cost(&self, node: usize, tn: usize, destination: usize) -> f64 {
        debug_assert!(node < self.nodes);
        debug_assert!(tn < self.time_steps);
        debug_assert!(destination < self.destinations);
        
        self.data[(destination * self.time_steps + tn) * self.nodes + node]
    }
}
