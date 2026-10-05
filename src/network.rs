use std::collections::{HashMap, HashSet};
use std::ops::Range;

/// Both `NodeIndex` and [NodeID] are unique representations/identifiers of nodes in a
/// network. While the only assumption for `NodeID` are that it is an unsinged integer,
/// `NodeIndex` has more assumptions tied with it:
/// - For a [Network] with number of nodes *N*, values of `NodeIndex` span the range
///   given by [Network::nodes()] (i.e. from 0 to *N*).
/// - For a [Network] with number of zones *Z*, the zones are the first *Z* nodes (i.e.
///   values of `NodeIndex` from 0 to *Z*, given by [Network::zones()]).
///
/// `NodeID` is the 'outer' representation used e.g. by methods which load the data.
/// `NodeIndex` is the 'inner' representation which allows direct addressing for more
/// performance.
pub type NodeIndex = usize;

/// See documentation for [NodeIndex].
pub type NodeID = usize;

pub struct Network {
    nodes: usize,
    zones: usize,
    outgoing_indices: Vec<usize>,
    outgoing_links: Vec<Link>,
    incoming_indices: Vec<usize>,
    incoming_links: Vec<Link>,
    mirror_links: Vec<Option<Link>>,
}

impl Network {
    /// Creates new network from link data and zones. The first argument is a slice of
    /// pairs of [NodeID], each pair represents a single link in the network. The second
    /// argument is a slice of [NodeID] and it contains those nodes which are zones.
    ///
    /// The node representation by IDs is converted to representation using indices, see
    /// the documentation of [NodeIndex] for more details.
    pub fn new(
        link_data: &[(NodeID, NodeID)],
        zones: &[NodeID],
    ) -> (Network, HashMap<NodeID, NodeIndex>) {
        debug_assert!(
            link_data.iter().collect::<HashSet<_>>().len() == link_data.len(),
            "The data contain duplicate links!"
        );

        // Create vector of nodes
        let mut id_to_index = HashMap::new();
        let mut node_index = 0;
        let mut add = |n: &NodeID| {
            if !id_to_index.contains_key(n) {
                id_to_index.insert(*n, node_index);
                node_index += 1;
            }
        };

        for zone in zones {
            // first add zones as they will be the first nodes
            add(zone);
        }

        for (from, to) in link_data {
            // then add the rest of the nodes
            add(from);
            add(to);
        }

        let links = link_data
            .iter()
            .map(|(from, to)| (id_to_index[from], id_to_index[to]))
            .collect();

        (Network::build(node_index, zones.len(), links), id_to_index)
    }

    pub fn expand(
        self,
        id2index: Option<HashMap<NodeID, NodeIndex>>,
    ) -> (Network, Option<HashMap<NodeID, NodeIndex>>) {
        let z = self.zones;

        // Former zones (and every other node) move up by z
        // indices 0..z are now free for the new zones.
        let id2index = id2index.map(|mut map| {
            for index in map.values_mut() {
                *index += z;
            }
            map
        });

        let mut links = Vec::with_capacity(self.outgoing_links.len() + 2 * z);

        // New zone <-> former zone connections
        for zone in 0..z {
            links.push((zone, zone + z));
            links.push((zone + z, zone));
        }

        // Existing links, shifted to the new node indices
        links.extend(self.outgoing_links.iter().map(|l| (l.tail + z, l.head + z)));

        (Network::build(self.nodes + z, z, links), id2index)
    }

    pub fn reorder_nodes(
        self,
        new_indices: HashMap<NodeIndex, NodeIndex>,
        id2index: Option<HashMap<NodeID, NodeIndex>>,
    ) -> (Network, Option<HashMap<NodeID, NodeIndex>>) {

        let id2index = id2index.map(|mut map| {
            for index in map.values_mut() {
                *index = new_indices[index];
            }
            map
        });

        let links = self
            .outgoing_links
            .iter()
            .map(|link| (new_indices[&link.tail], new_indices[&link.head]))
            .collect();

        (Network::build(self.nodes, self.zones, links), id2index)
    }

    fn build(nodes: usize, zones: usize, links: Vec<(NodeIndex, NodeIndex)>) -> Network {
        let mut outgoing_links: Vec<Link> = links
            .into_iter()
            .map(|(tail, head)| Link {
                index: 0,
                tail,
                head,
            })
            .collect();
        outgoing_links.sort_unstable_by_key(|link| link.tail);

        // Set the indices of links
        for (i, link) in outgoing_links.iter_mut().enumerate() {
            link.index = i;
        }

        let outgoing_indices = Self::offsets(nodes, outgoing_links.iter().map(|l| l.tail));

        let mut incoming_links = outgoing_links.clone();
        incoming_links.sort_unstable_by_key(|link| link.head);

        let incoming_indices = Self::offsets(nodes, outgoing_links.iter().map(|l| l.head));

        let mut mirror_links = vec![None; outgoing_links.len()];
        for link in &outgoing_links {
            let range = incoming_indices[link.tail]..incoming_indices[link.tail + 1];
            if let Some(reverse) = incoming_links[range]
                .iter()
                .find(|incoming| incoming.tail == link.head)
            {
                mirror_links[link.index] = Some(reverse.clone());
            }
        }

        Network {
            nodes,
            zones,
            outgoing_indices,
            outgoing_links,
            incoming_indices,
            incoming_links,
            mirror_links,
        }
    }

    fn offsets(nodes: usize, keys: impl Iterator<Item = NodeIndex>) -> Vec<usize> {
        let mut offsets = vec![0; nodes + 1];
        for key in keys {
            offsets[key + 1] += 1;
        }
        for i in 1..=nodes {
            offsets[i] += offsets[i - 1];
        }
        offsets
    }

    pub fn nodes(&self) -> Range<NodeIndex> {
        0..self.nodes
    }

    pub fn zones(&self) -> Range<NodeIndex> {
        0..self.zones
    }

    pub fn links(&self) -> &[Link] {
        &self.outgoing_links[..]
    }

    pub fn outgoing_links(&self, node: NodeIndex) -> &[Link] {
        let range = self.outgoing_indices[node]..self.outgoing_indices[node + 1];
        &self.outgoing_links[range]
    }

    pub fn incoming_links(&self, node: NodeIndex) -> &[Link] {
        let range = self.incoming_indices[node]..self.incoming_indices[node + 1];
        &self.incoming_links[range]
    }

    pub fn mirror_link(&self, link_index: usize) -> &Option<Link> {
        &self.mirror_links[link_index]
    }
}

#[derive(Debug, Clone)]
pub struct Link {
    index: usize,
    tail: NodeIndex,
    head: NodeIndex,
}

impl Link {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn tail(&self) -> NodeIndex {
        self.tail
    }

    pub fn head(&self) -> NodeIndex {
        self.head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = vec![(2, 1), (2, 3), (2, 4), (4, 3), (4, 2)];

        let (network, id_to_index) = Network::new(&data, &[]);

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

        let _ = Network::new(&data, &[]);
    }
}
