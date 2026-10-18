pub struct OutgoingLinkIndices {
    data: Vec<u8>,
    nodes: usize,
    destinations: usize,
    time_steps: usize,
}

impl OutgoingLinkIndices {
    pub fn new(nodes: usize, time_steps: usize, destinations: usize) -> Self {
        Self {
            data: vec![0; nodes * destinations * time_steps],
            nodes,
            time_steps,
            destinations,
        }
    }

    pub fn set_index(&mut self, node: usize, tn: usize, destination: usize, index: u8) {
        debug_assert!(node < self.nodes);
        debug_assert!(tn < self.time_steps);
        debug_assert!(destination < self.destinations);

        self.data[destination * self.time_steps * self.nodes + tn * self.nodes + node] = index;
    }

    pub fn get_index(&self, node: usize, tn: usize, destination: usize) -> u8 {
        debug_assert!(node < self.nodes);
        debug_assert!(tn < self.time_steps);
        debug_assert!(destination < self.destinations);

        self.data[destination * self.time_steps * self.nodes + tn * self.nodes + node]
    }
}
