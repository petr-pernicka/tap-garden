use crate::network::NodeIndex;
use std::ops::Range;

pub struct OutgoingLinkIndices {
    data: Vec<u8>,
    used_nodes: Range<NodeIndex>,
    destinations: usize,
    time_steps: usize,
}

impl OutgoingLinkIndices {
    pub fn new(used_nodes: Range<NodeIndex>, time_steps: usize, destinations: usize) -> Self {
        Self {
            data: vec![0; used_nodes.len() * destinations * time_steps],
            used_nodes,
            time_steps,
            destinations,
        }
    }

    pub fn set_index(&mut self, n: usize, tn: usize, d: usize, index: u8) {
        debug_assert!(self.used_nodes.contains(&n));
        debug_assert!(tn < self.time_steps);
        debug_assert!(d < self.destinations);
        let n = n - self.used_nodes.start;

        self.data[d * self.time_steps * self.used_nodes.len() + tn * self.used_nodes.len() + n] =
            index;
    }

    pub fn get_index(&self, n: usize, tn: usize, d: usize) -> u8 {
        debug_assert!(self.used_nodes.contains(&n));
        debug_assert!(tn < self.time_steps);
        debug_assert!(d < self.destinations);
        let n = n - self.used_nodes.start;

        self.data[d * self.time_steps * self.used_nodes.len() + tn * self.used_nodes.len() + n]
    }
}
