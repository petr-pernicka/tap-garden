use crate::network::{Network, NodeIndex};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Range;
use std::rc::Rc;

pub struct MixtureOutgoingFractions {
    shared_data: Rc<RefCell<SharedData>>,
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

        let shared_data = SharedData {
            destinations: network.zones().len(),
            time_steps,
            temporary_grid: vec![0.; network.zones().len() * time_steps * max_outgoing_links],
        };
        let shared_data = Rc::new(RefCell::new(shared_data));

        let mut mof_intersections = Vec::with_capacity(used_nodes.len());
        for n in used_nodes.clone() {
            mof_intersections.push(MofIntersection {
                shared_data: shared_data.clone(),
                num_outgoing_links: network.outgoing_links(n).len(),
                offsets: vec![0; network.zones().len() + 1],
                indices: vec![],
                values: vec![],
                pool: vec![],
            });
        }

        Self {
            shared_data,
            mof_intersections,
            used_nodes,
        }
    }

    pub fn compress_for_node(&mut self, node: usize) {
        self.mof_intersections[node].compress()
    }

    pub fn set_fraction(&mut self, n: usize, tn: usize, d: usize, j: usize, value: f64) {
        debug_assert!(self.used_nodes.contains(&n));
        debug_assert!(tn < self.shared_data.borrow().time_steps);
        debug_assert!(d < self.shared_data.borrow().destinations);
        debug_assert!(j < self.mof_intersections[n].num_outgoing_links);
        debug_assert!((0.0..=1.).contains(&value));
        let n = n - self.used_nodes.start;

        let mut shared_data = self.shared_data.borrow_mut();
        let destinations = shared_data.destinations;
        let num_ougoing_links = self.mof_intersections[n].num_outgoing_links;
        shared_data.temporary_grid
            [tn * destinations * num_ougoing_links + d * num_ougoing_links + j] = value;
    }

    pub fn get_fraction(&self, n: usize, tn: usize, d: usize, j: usize) -> f64 {
        debug_assert!(n < self.mof_intersections.len());
        debug_assert!(tn < self.shared_data.borrow().time_steps);
        debug_assert!(d < self.shared_data.borrow().destinations);
        debug_assert!(j < self.mof_intersections[n].num_outgoing_links);
        let n = n - self.used_nodes.start;

        self.mof_intersections[n].get_fraction(tn, d, j)
    }

    pub fn destinations(&self) -> usize {
        self.shared_data.borrow().destinations
    }
}

struct SharedData {
    destinations: usize,
    time_steps: usize,
    temporary_grid: Vec<f64>,
}

struct MofIntersection {
    shared_data: Rc<RefCell<SharedData>>,
    num_outgoing_links: usize,
    offsets: Vec<usize>,
    indices: Vec<usize>,
    values: Vec<isize>,
    pool: Vec<f64>,
}

impl MofIntersection {
    fn get_fraction(&self, tn: usize, d: usize, j: usize) -> f64 {
        let mut i = self.offsets[d];
        let last = self.offsets[d + 1] - 1;

        while i < last && self.indices[i + 1] <= tn {
            i += 1;
        }

        let index = self.values[i];
        if index >= 0 {
            // vector[n, tn, d] is a singleton
            if index as usize == j { 1. } else { 0. }
        } else {
            // vector is in the pool
            let pool_index = -(index + 1) as usize;
            self.pool[pool_index * self.num_outgoing_links + j]
        }
    }

    fn compress(&mut self) {
        let &mut SharedData {
            ref destinations,
            ref time_steps,
            ref mut temporary_grid,
        } = &mut *self.shared_data.borrow_mut();

        // 1. Create the index grid and pool of unique vectors.
        let mut index_grid = vec![0isize; destinations * time_steps];

        let mut pool_len = 0;
        let mut vector_to_pool_index: BTreeMap<OrdVec, usize> = BTreeMap::new();

        for d in 0..*destinations {
            for tn in 0..*time_steps {
                // The slice of the vector for (tn, d).
                let start =
                    tn * destinations * self.num_outgoing_links + d * self.num_outgoing_links;
                let end =
                    tn * destinations * self.num_outgoing_links + (d + 1) * self.num_outgoing_links;
                let vector = &temporary_grid[start..end];

                let singleton_index = Self::is_singleton(vector);

                if singleton_index >= 0 {
                    // Vector is singleton with sole 1 at singleton_index. The vector can
                    // be represented only by singleton_index.

                    index_grid[tn * destinations + d] = singleton_index;
                } else {
                    // If not, set the value in grid to the index of the vector in the
                    // pool of unique vectors, adding it if necessary.
                    // Index i into the pool is stored in the grid as -(i + 1),
                    // because non-negative integers represent singletons.
                    let pool_index = vector_to_pool_index.get(&OrdVec(vector));

                    if let Some(pool_index) = pool_index {
                        index_grid[tn * destinations + d] = -(*pool_index as isize + 1);
                    } else {
                        vector_to_pool_index.insert(OrdVec(vector), pool_len);
                        index_grid[tn * destinations + d] = -(pool_len as isize + 1);
                        pool_len += 1;
                    }
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
        for d in 0..*destinations {
            let mut num_periods = 1;
            for tn in 1..*time_steps {
                if index_grid[tn * destinations + d] != index_grid[(tn - 1) * destinations + d] {
                    num_periods += 1;
                }
            }

            self.offsets[d + 1] = self.offsets[d] + num_periods;
        }

        let total_num_periods = self.offsets.last().unwrap();
        self.indices = vec![0; *total_num_periods];
        self.values = vec![0; *total_num_periods];

        for d in 0..*destinations {
            // values[i] is the value that lies in the grid on positions from
            // grid[indices[i] * destinations + d] to
            // grid[(indices[i + 1] - 1) * destinations + d].
            // That is, on the positions in the grid where t is in interval
            // [indices[i], indices[i + 1]) and for this destination.
            self.indices[self.offsets[d]] = 0;
            self.values[self.offsets[d]] = index_grid[d];
            let mut i = self.offsets[d] + 1;
            for tn in 1..*time_steps {
                if index_grid[tn * destinations + d] != index_grid[(tn - 1) * destinations + d] {
                    self.indices[i] = tn;
                    self.values[i] = index_grid[tn * destinations + d];
                    i += 1;
                }
            }
        }

        self.pool = pool;
        temporary_grid.fill(0.);
    }

    fn is_singleton(vector: &[f64]) -> isize {
        for (i, val) in vector.iter().enumerate() {
            if *val == 1. {
                return i as isize;
            }
            if *val > 0. {
                return -1;
            }
        }

        -1
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
