use crate::network::{Network, NodeIndex};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct SSSP<'a> {
    network: &'a Network,
}

impl<'a> SSSP<'a> {
    pub fn new(network: &'a Network) -> Self {
        Self { network }
    }

    pub fn one_to_all(&self, origin: NodeIndex, link_costs: &[f64]) -> (Vec<f64>, Vec<usize>) {
        debug_assert_eq!(link_costs.len(), self.network.links().len());

        let mut distance = vec![f64::INFINITY; self.network.nodes().len()];
        distance[origin] = 0.0;

        let mut predecessor_tree = vec![0; self.network.nodes().len()];
        let mut priority_queue = BinaryHeap::with_capacity(self.network.links().len() + 1);

        let key = (0f64.to_bits() as u128) << 64 | origin as u128;
        priority_queue.push(Reverse(key));

        while let Some(Reverse(key)) = priority_queue.pop() {
            let tail = (key & 0xFFFF_FFFF_FFFF_FFFF) as usize;
            let tail_distance = f64::from_bits((key >> 64) as u64);

            if tail_distance > distance[tail] {
                continue;
            }

            for outgoing_link in self.network.outgoing_links(tail) {
                let head = outgoing_link.head();
                let head_distance = distance[head];
                let link_cost = link_costs[outgoing_link.index()];

                let new_head_distance = tail_distance + link_cost;
                if new_head_distance < head_distance {
                    predecessor_tree[head] = outgoing_link.index();
                    distance[head] = new_head_distance;

                    let key = (new_head_distance.to_bits() as u128) << 64 | head as u128;
                    priority_queue.push(Reverse(key));
                }
            }
        }

        (distance, predecessor_tree)
    }
}
