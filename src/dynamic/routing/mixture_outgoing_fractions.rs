use crate::network::{Network, NodeIndex};
use get_size2::GetSize;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Range;

#[derive(GetSize)]
pub struct MixtureOutgoingFractions {
    destinations: usize,
    time_steps: usize,
    temporary_grid: Vec<f64>,
    mof_intersections: Vec<MofIntersection>,
    used_nodes: Range<NodeIndex>,
}

impl MixtureOutgoingFractions {
    pub fn new(network: &Network, used_nodes: Range<NodeIndex>, time_steps: usize) -> Self {
        debug_assert!(used_nodes.end <= network.nodes().len());

        let max_outgoing_links = network
            .nodes()
            .map(|n| network.outgoing_links(n).len())
            .max()
            .unwrap();

        let temporary_grid = vec![0.; network.zones().len() * time_steps * max_outgoing_links];

        let mut mof_intersections = Vec::with_capacity(used_nodes.len());
        for n in used_nodes.clone() {
            mof_intersections.push(MofIntersection {
                num_outgoing_links: network.outgoing_links(n).len(),
                destinations: network.zones().len(),
                time_steps,
                offsets: vec![0; network.zones().len() + 1],
                indices: vec![],
                values: vec![],
                pool: vec![],
            });
        }

        Self {
            destinations: network.zones().len(),
            time_steps,
            temporary_grid,
            mof_intersections,
            used_nodes,
        }
    }

    pub fn compress_for_node(&mut self, n: usize) {
        debug_assert!(self.used_nodes.contains(&n));
        let n = n - self.used_nodes.start;

        self.mof_intersections[n].compress(&mut self.temporary_grid)
    }

    pub fn set_fraction(&mut self, n: usize, tn: usize, d: usize, j: usize, value: f64) {
        debug_assert!(self.used_nodes.contains(&n));
        let n = n - self.used_nodes.start;

        debug_assert!(tn < self.time_steps);
        debug_assert!(d < self.destinations);
        debug_assert!(j < self.mof_intersections[n].num_outgoing_links);
        debug_assert!((0.0..=1.).contains(&value));

        let num_ougoing_links = self.mof_intersections[n].num_outgoing_links;
        self.temporary_grid
            [tn * self.destinations * num_ougoing_links + d * num_ougoing_links + j] = value;
    }

    pub fn get_fractions(&self, n: usize, tn: usize, d: usize) -> &[f64] {
        debug_assert!(self.used_nodes.contains(&n));
        let n = n - self.used_nodes.start;

        debug_assert!(tn < self.time_steps);
        debug_assert!(d < self.destinations);

        self.mof_intersections[n].get_fractions(tn, d)
    }

    pub fn destinations(&self) -> usize {
        self.destinations
    }
}

/// This struct retains the compressed mixture fractions for a single node. The
/// uncompressed number of values is `time_steps * destinations * num_outgoing_links`.
/// The compression interprets the values as a `time_steps x destinations` **grid** of
/// **vectors**, each of length `num_outgoing_links`. The compression consists of 2 phases.
///
/// # Phase 1
/// We observe that many of the vectors has a 1 at some index and the rest are 0s. These
/// are called singletons. A singleton can be represented by a single integer - index of
/// the 1 in the vector. Furthermore, non-singleton vectors may have duplicates in the
/// grid.
///
/// Phase 1 compresses such that it replaces each vector with signed integer:
/// - if the vector is a singleton with 1 at index `i`, it is replaced by `i`,
/// - if the vector is not a singleton, we check if it is in a map
///   *non-singleton vector -> j*:
///     - if yes, the vector is replaced by `-(j + 1)`,
///     - if not, the vector is added to the map with new `j`.
///
/// The `j`s start from 0 and are incremented with each not-yet-encountered non-singleton
/// vector. The phase ends with creating the pool of unique non-singleton vectors from the
/// map - `j`s serve as indices into the pool. Values `i >= 0` in the grid now represent
/// singletons while values `j < 0` represent non-singletons.
///
/// Both the pool and the integer grid are stored in row-major order to further decrease
/// memory consumption.
///
/// # Phase 2
/// We now observe that there are long sequences of constant values across many time steps
/// in the grid
#[derive(GetSize)]
struct MofIntersection {
    num_outgoing_links: usize,
    destinations: usize,
    time_steps: usize,
    offsets: Vec<usize>,
    indices: Vec<usize>,
    values: Vec<usize>,
    pool: Vec<f64>,
}

impl MofIntersection {
    fn get_fractions(&self, tn: usize, d: usize) -> &[f64] {
        unsafe {
            let mut i = *self.offsets.get_unchecked(d);
            let last = *self.offsets.get_unchecked(d + 1) - 1;

            while i < last && *self.indices.get_unchecked(i + 1) <= tn {
                i += 1;
            }

            let index = *self.values.get_unchecked(i);
            self.pool.get_unchecked(
                index * self.num_outgoing_links..(index + 1) * self.num_outgoing_links,
            )
        }
    }

    fn compress(&mut self, temporary_grid: &mut [f64]) {
        let destinations = self.destinations;
        let time_steps = self.time_steps;

        // 1. Create the index grid and pool of unique vectors.
        let mut index_grid = vec![0; destinations * time_steps];

        let mut pool_len = 0;
        let mut vector_to_pool_index: BTreeMap<OrdVec, usize> = BTreeMap::new();

        for d in 0..destinations {
            for tn in 0..time_steps {
                // The slice of the vector for (tn, d).
                let start =
                    tn * destinations * self.num_outgoing_links + d * self.num_outgoing_links;
                let end =
                    tn * destinations * self.num_outgoing_links + (d + 1) * self.num_outgoing_links;
                let vector = &temporary_grid[start..end];

                // Set the value in grid to the index of the vector in the pool of unique
                // vectors, adding it if necessary.
                let pool_index = vector_to_pool_index.get(&OrdVec(vector));

                if let Some(pool_index) = pool_index {
                    index_grid[d * time_steps + tn] = *pool_index;
                } else {
                    vector_to_pool_index.insert(OrdVec(vector), pool_len);
                    index_grid[d * time_steps + tn] = pool_len;
                    pool_len += 1;
                }
            }
        }

        let mut pool = vec![0.; pool_len * self.num_outgoing_links];
        for (vector, pool_index) in vector_to_pool_index {
            let start = pool_index * self.num_outgoing_links;
            let end = (pool_index + 1) * self.num_outgoing_links;
            pool[start..end].copy_from_slice(vector.0);
        }

        // 2. Compress the grid.
        // 2.1. Initialize the offsets for the CSR
        for d in 0..destinations {
            let mut num_periods = 1;
            for tn in 1..time_steps {
                if index_grid[d * time_steps + tn] != index_grid[d * time_steps + tn - 1] {
                    num_periods += 1;
                }
            }

            self.offsets[d + 1] = self.offsets[d] + num_periods;
        }

        let total_num_periods = self.offsets.last().unwrap();
        self.indices = vec![0; *total_num_periods];
        self.values = vec![0; *total_num_periods];

        for d in 0..destinations {
            // values[i] is the value that lies in the grid on positions from
            // grid[indices[i] * destinations + d] to
            // grid[(indices[i + 1] - 1) * destinations + d].
            // That is, on the positions in the grid where t is in interval
            // [indices[i], indices[i + 1]) and for this destination.
            self.indices[self.offsets[d]] = 0;
            self.values[self.offsets[d]] = index_grid[d * time_steps];
            let mut i = self.offsets[d] + 1;
            for tn in 1..time_steps {
                if index_grid[d * time_steps + tn] != index_grid[d * time_steps + tn - 1] {
                    self.indices[i] = tn;
                    self.values[i] = index_grid[d * time_steps + tn];
                    i += 1;
                }
            }
        }

        self.pool = pool;
        temporary_grid[..destinations * time_steps * self.num_outgoing_links].fill(0.);
    }
}

struct OrdVec<'a>(&'a [f64]);

impl Eq for OrdVec<'_> {}

impl PartialEq<Self> for OrdVec<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for OrdVec<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdVec<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        debug_assert_eq!(self.0.len(), other.0.len());

        for i in 0..self.0.len() {
            if self.0[i] < other.0[i] {
                return Ordering::Less;
            }
            if self.0[i] > other.0[i] {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }
}
