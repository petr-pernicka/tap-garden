use crate::network::{Network, NodeIndex};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct SSSP<'a> {
    network: &'a Network,
    priority_queue: BinaryHeap<Reverse<u128>>,
    distance: Vec<f64>,
    tree: Vec<usize>,
}

impl<'a> SSSP<'a> {
    pub fn new(network: &'a Network) -> Self {
        Self {
            network,
            priority_queue: BinaryHeap::with_capacity(network.links().len() + 1),
            distance: vec![0.0; network.nodes().len()],
            tree: vec![0; network.nodes().len()],
        }
    }

    pub fn distance(&self) -> &[f64] {
        &self.distance
    }

    pub fn tree(&self) -> &[usize] {
        &self.tree
    }

    pub fn one_to_all(&mut self, origin: NodeIndex, link_costs: &[f64]) {
        debug_assert!(self.network.zones().contains(&origin));
        debug_assert_eq!(link_costs.len(), self.network.links().len());
        debug_assert!(link_costs.iter().all(|x| *x >= 0.0));

        self.priority_queue.clear();

        self.distance.fill(f64::INFINITY);
        self.distance[origin] = 0.0;
        self.tree.fill(usize::MAX);

        let key = (0f64.to_bits() as u128) << 64 | origin as u128;
        self.priority_queue.push(Reverse(key));

        while let Some(Reverse(key)) = self.priority_queue.pop() {
            let tail = (key & 0xFFFF_FFFF_FFFF_FFFF) as usize;
            let tail_distance = f64::from_bits((key >> 64) as u64);

            if tail_distance > self.distance[tail] {
                continue;
            }

            for outgoing_link in self.network.outgoing_links(tail) {
                let head = outgoing_link.head();
                let head_distance = self.distance[head];
                let link_cost = link_costs[outgoing_link.index()];

                let new_head_distance = tail_distance + link_cost;
                if new_head_distance < head_distance {
                    self.tree[head] = outgoing_link.index();
                    self.distance[head] = new_head_distance;

                    let key = (new_head_distance.to_bits() as u128) << 64 | head as u128;
                    self.priority_queue.push(Reverse(key));
                }
            }
        }
    }

    pub fn all_to_one(&mut self, destination: NodeIndex, link_costs: &[f64]) {
        debug_assert!(self.network.zones().contains(&destination));
        debug_assert_eq!(link_costs.len(), self.network.links().len());
        debug_assert!(link_costs.iter().all(|x| *x >= 0.0));

        self.priority_queue.clear();

        self.distance.fill(f64::INFINITY);
        self.distance[destination] = 0.0;
        self.tree.fill(usize::MAX);

        let key = (0f64.to_bits() as u128) << 64 | destination as u128;
        self.priority_queue.push(Reverse(key));

        while let Some(Reverse(key)) = self.priority_queue.pop() {
            let head = (key & 0xFFFF_FFFF_FFFF_FFFF) as usize;
            let head_distance = f64::from_bits((key >> 64) as u64);

            if head_distance > self.distance[head] {
                continue;
            }

            for incoming_link in self.network.incoming_links(head) {
                let tail = incoming_link.tail();
                let tail_distance = self.distance[tail];
                let link_cost = link_costs[incoming_link.index()];

                let new_tail_distance = head_distance + link_cost;
                if new_tail_distance < tail_distance {
                    self.tree[tail] = incoming_link.index();
                    self.distance[tail] = new_tail_distance;

                    let key = (new_tail_distance.to_bits() as u128) << 64 | head as u128;
                    self.priority_queue.push(Reverse(key));
                }
            }
        }
    }
}
