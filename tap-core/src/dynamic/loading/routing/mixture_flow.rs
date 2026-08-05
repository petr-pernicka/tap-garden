use std::ops::Add;

// TODO substitute Box with Rc for faster clones
// TODO potentially single box of (u32, f64)?
#[derive(Debug)]
pub struct MixtureFlow {
    total_flow: f64,
    destinations: Box<[u32]>,
    portions: Box<[f64]>,
}

impl Add<&MixtureFlow> for &MixtureFlow {
    type Output = MixtureFlow;

    fn add(self, rhs: &MixtureFlow) -> Self::Output {
        self.plus(rhs)
    }
}

impl MixtureFlow {
    pub fn new(total_flow: f64, destinations: Vec<u32>, portions: Vec<f64>) -> Self {
        debug_assert!(total_flow >= 0.0);
        debug_assert!(destinations.len() == portions.len());
        debug_assert!(portions.iter().all(|x| *x >= 0.0));
        debug_assert!((portions.iter().sum::<f64>() - 1.0).abs() < 1e-10);
        debug_assert!({
            let mut is_ascending = true;
            for i in 1..destinations.len() {
                if destinations[i] <= destinations[i - 1] {
                    is_ascending = false;
                    break;
                }
            }
            is_ascending
        });

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

        // UNSAFE: If the program doesn't panic with debug assets enabled, UB will not occur.
        unsafe { *self.destinations.get_unchecked(index) }
    }

    pub fn portion(&self, index: usize) -> f64 {
        debug_assert!(index < self.size());

        // UNSAFE: If the program doesn't panic with debug assets enabled, UB will not occur.
        unsafe { *self.portions.get_unchecked(index) }
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
            };
        }

        if other.total_flow == 0.0 {
            return MixtureFlow {
                total_flow: self.total_flow,
                destinations: self.destinations.clone(),
                portions: self.portions.clone(),
            };
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
                    (self.portion(i) * self.total_flow + other.portion(j) * other.total_flow)
                        / result_flow,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);

        assert_eq!(mf.total_flow, 2.5);
        assert_eq!(*mf.destinations, [0, 1]);
        assert_eq!(*mf.portions, [0.4, 0.6]);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_new_bad1() {
        let _mf = MixtureFlow::new(-2.5, vec![0, 1], vec![0.4, 0.6]);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_new_bad2() {
        let _mf = MixtureFlow::new(2.5, vec![0], vec![0.4, 0.6]);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_new_bad3() {
        let _mf = MixtureFlow::new(2.5, vec![0, 1], vec![-0.3, 1.3]);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_new_bad4() {
        let _mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.3, 0.3]);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_new_bad5() {
        let _mf = MixtureFlow::new(2.5, vec![1, 0], vec![0.3, 0.3]);
    }

    #[test]
    fn test_zero() {
        let mf = MixtureFlow::zero();

        assert_eq!(mf.total_flow, 0.0);
        assert_eq!(*mf.destinations, []);
        assert_eq!(*mf.portions, []);
    }

    #[test]
    fn test_size() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);

        assert_eq!(mf.size(), 2);
    }

    #[test]
    fn test_total_flow() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);

        assert_eq!(mf.total_flow, 2.5);
    }

    #[test]
    fn test_destination() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);

        assert_eq!(mf.destination(0), 0);
        assert_eq!(mf.destination(1), 1);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_destination_bad() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);

        mf.destination(2);
    }

    #[test]
    fn test_portion() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);

        assert_eq!(mf.portion(0), 0.4);
        assert_eq!(mf.portion(1), 0.6);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_portion_bad() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);

        mf.portion(2);
    }

    #[test]
    fn test_copy_with_flow1() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);
        let mf2 = mf.copy_with_flow(4.1);

        assert_eq!(mf2.total_flow, 4.1);
        assert_eq!(*mf2.destinations, [0, 1]);
        assert_eq!(*mf2.portions, [0.4, 0.6]);
    }

    #[test]
    fn test_copy_with_flow2() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);
        let mf2 = mf.copy_with_flow(0.0);

        assert_eq!(mf2.total_flow, 0.0);
        assert_eq!(mf2.destinations.len(), 0);
        assert_eq!(mf2.portions.len(), 0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_copy_with_flow_bad() {
        let mf = MixtureFlow::new(2.5, vec![0, 1], vec![0.4, 0.6]);
        let _mf = mf.copy_with_flow(-4.1);
    }
}
