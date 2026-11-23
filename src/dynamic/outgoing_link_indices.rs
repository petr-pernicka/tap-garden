use crate::network::NodeIndex;
use std::ops::Range;
use get_size2::GetSize;

#[derive(GetSize)]
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

    pub fn set_index(&mut self, tn: usize, n: usize, d: usize, index: u8) {
        debug_assert!(tn < self.time_steps);
        debug_assert!(self.used_nodes.contains(&n));
        debug_assert!(d < self.destinations);
        let n = n - self.used_nodes.start;

        self.data[tn * self.destinations * self.used_nodes.len() + n * self.destinations + d] =
            index;
    }

    pub fn get_index(&self, tn: usize, n: usize, d: usize) -> u8 {
        debug_assert!(tn < self.time_steps);
        debug_assert!(self.used_nodes.contains(&n));
        debug_assert!(d < self.destinations);
        let n = n - self.used_nodes.start;

        self.data[tn * self.destinations * self.used_nodes.len() + n * self.destinations + d]
    }
}
