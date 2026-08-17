use std::collections::{HashMap, HashSet};
use std::ops::Range;

pub struct Network {
    nodes: usize,
    zones: usize,
    outgoing_indices: Vec<usize>,
    outgoing_links: Vec<Link>,
    incoming_indices: Vec<usize>,
    incoming_links: Vec<Link>,
    mirror_links: Vec<Option<Link>>,
}

pub type Node = usize;

#[derive(Debug, Clone)]
pub struct Link {
    index: usize,
    tail: Node,
    head: Node,
}

impl Network {
    pub fn new(data: Vec<(u32, u32)>, zones: &[u32]) -> (Self, HashMap<u32, Node>) {
        debug_assert!(
            data.iter().collect::<HashSet<_>>().len() == data.len(),
            "The data contain duplicate links!"
        );

        // Create vector of nodes
        let mut id_to_index = HashMap::new();
        let mut node_index = 0;
        let mut add = |n: &u32| {
            if !id_to_index.contains_key(n) {
                id_to_index.insert(*n, node_index);
                node_index += 1;
            }
        };

        for (from, to) in &data {
            add(from);
            add(to);
        }

        // Create vectors of links and indices
        let mut outgoing_links = Vec::with_capacity(data.len());
        for (from, to) in &data {
            outgoing_links.push(Link {
                index: 0,
                tail: id_to_index[from],
                head: id_to_index[to],
            });
        }
        outgoing_links.sort_unstable_by_key(|link| link.tail);

        // Set the indices of links
        for (i, outgoing_link) in outgoing_links.iter_mut().enumerate() {
            outgoing_link.index = i;
        }

        let mut outgoing_indices = vec![0; node_index + 1];
        for link in &outgoing_links {
            outgoing_indices[link.tail + 1] += 1;
        }
        for i in 1..=node_index {
            outgoing_indices[i] += outgoing_indices[i - 1];
        }

        let mut incoming_links = outgoing_links.clone();
        incoming_links.sort_unstable_by_key(|link| link.head);

        let mut incoming_indices = vec![0; node_index + 1];
        for link in &outgoing_links {
            incoming_indices[link.head + 1] += 1;
        }
        for i in 1..=node_index {
            incoming_indices[i] += incoming_indices[i - 1];
        }

        let mut mirror_links = vec![None; data.len()];
        for link in &outgoing_links {
            let range = incoming_indices[link.tail]..incoming_indices[link.tail + 1];
            if let Some(reverse) = incoming_links[range]
                .iter()
                .find(|incoming| incoming.tail == link.head)
            {
                mirror_links[link.index] = Some(reverse.clone());
            }
        }

        (
            Network {
                nodes: node_index,
                zones: zones.len(),
                outgoing_indices,
                outgoing_links,
                incoming_indices,
                incoming_links,
                mirror_links,
            },
            id_to_index,
        )
    }

    pub fn nodes(&self) -> Range<Node> {
        0..self.nodes
    }
    
    pub fn zones(&self) -> Range<Node> {
        0..self.zones
    }

    pub fn links(&self) -> &[Link] {
        &self.outgoing_links[..]
    }

    pub fn outgoing_links(&self, node: Node) -> &[Link] {
        let range = self.outgoing_indices[node]..self.outgoing_indices[node + 1];
        &self.outgoing_links[range]
    }

    pub fn incoming_links(&self, node: Node) -> &[Link] {
        let range = self.incoming_indices[node]..self.incoming_indices[node + 1];
        &self.incoming_links[range]
    }

    pub fn mirror_link(&self, node: Node) -> &Option<Link> {
        &self.mirror_links[node]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = vec![(2, 1), (2, 3), (2, 4), (4, 3), (4, 2)];

        let (network, id_to_index) = Network::new(data, &[]);

        assert_eq!(network.nodes, 4);
        assert_eq!(network.outgoing_indices.len(), 5);
        assert_eq!(network.outgoing_links.len(), 5);
        assert_eq!(
            network.outgoing_indices[id_to_index[&1] + 1]
                - network.outgoing_indices[id_to_index[&1]],
            0
        );
        assert_eq!(
            network.outgoing_indices[id_to_index[&2] + 1]
                - network.outgoing_indices[id_to_index[&2]],
            3
        );
        assert_eq!(
            network.outgoing_indices[id_to_index[&3] + 1]
                - network.outgoing_indices[id_to_index[&3]],
            0
        );
        assert_eq!(
            network.outgoing_indices[id_to_index[&4] + 1]
                - network.outgoing_indices[id_to_index[&4]],
            2
        );
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_new_bad() {
        let data = vec![(2, 1), (2, 3), (2, 4), (4, 3), (4, 2), (2, 1)];

        let _ = Network::new(data, &[]);
    }
}
