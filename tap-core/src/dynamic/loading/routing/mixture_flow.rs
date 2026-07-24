// TODO substitute Box with Rc for faster clones
struct MixtureFlow {
    total_flow: f64,
    destinations: Box<[u32]>,
    portions: Box<[f64]>,
}

impl MixtureFlow {
    pub fn new(total_flow: f64, destinations: Vec<u32>, portions: Vec<f64>) -> Self {
        debug_assert!(total_flow >= 0.0);
        debug_assert!(destinations.len() == portions.len());
        debug_assert!(portions.iter().all(|x| *x >= 0.0));
        debug_assert!((portions.iter().sum::<f64>() - 1.0).abs() > 1e-10);

        MixtureFlow {
            total_flow,
            destinations: destinations.into_boxed_slice(),
            portions: portions.into_boxed_slice(),
        }
    }

    pub fn zero() -> Self {
        MixtureFlow {
            total_flow: 0.0,
            destinations: Box::from([]),
            portions: Box::from([]),
        }
    }

    pub fn total_flow(&self) -> f64 {
        self.total_flow
    }

    pub fn size(&self) -> usize {
        self.destinations.len()
    }

    pub fn destination(&self, index: usize) -> u32 {
        debug_assert!(index < self.size());

        unsafe {
            *self.destinations.get_unchecked(index)
        }
    }

    pub fn portion(&self, index: usize) -> f64 {
        debug_assert!(index < self.size());

        unsafe {
            *self.portions.get_unchecked(index)
        }
    }

    pub fn copy_with_flow(&self, new_flow: f64) -> Self {
        debug_assert!(new_flow >= 0.0);

        if new_flow == 0.0 {
            return MixtureFlow::zero();
        }

        MixtureFlow {
            total_flow: new_flow,
            destinations: self.destinations.clone(),
            portions: self.portions.clone(),
        }
    }

    pub fn plus(&self, other: &MixtureFlow) -> MixtureFlow {
        if self.total_flow == 0.0 {
            return MixtureFlow {
                total_flow: other.total_flow,
                destinations: other.destinations.clone(),
                portions: other.portions.clone(),
            }
        }

        if other.total_flow == 0.0 {
            return MixtureFlow {
                total_flow: self.total_flow,
                destinations: self.destinations.clone(),
                portions: self.portions.clone(),
            }
        }

        let result_flow = self.total_flow + other.total_flow;
        let mut result_destinations = Vec::<u32>::with_capacity(self.size() + other.size());
        let mut result_portions = Vec::<f64>::with_capacity(self.size() + other.size());

        let m = self.size();
        let n = other.size();
        let mut i = 0;
        let mut j = 0;
        while i < m && j < n {

            if self.destination(i) < other.destination(j) {
                result_destinations.push(self.destination(i));
                result_portions.push(self.portion(i) * self.total_flow / result_flow);

                i += 1;
            } else if self.destination(i) > other.destination(j) {
                result_destinations.push(other.destination(j));
                result_portions.push(other.portion(j) * other.total_flow / result_flow);

                j += 1;
            } else {
                result_destinations.push(self.destination(i));
                result_portions.push(
                    (self.portion(i) * self.total_flow + other.portion(j) * other.total_flow) / result_flow
                );

                i += 1;
                j += 1;
            }
        }

        while i < m {
            result_destinations.push(self.destination(i));
            result_portions.push(self.portion(i) * self.total_flow / result_flow);

            i += 1;
        }

        while j < n {
            result_destinations.push(other.destination(j));
            result_portions.push(other.portion(j) * other.total_flow / result_flow);

            j += 1;
        }

        MixtureFlow::new(result_flow, result_destinations, result_portions)
    }
}
