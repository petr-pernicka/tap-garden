use crate::data::matrix::Matrix;
use crate::static_::ODM;

pub struct TimeDependentODM {
    demand: Box<[f64]>,
    zones: usize,
    time_steps: usize,
}

impl TimeDependentODM {
    pub fn new(demand: Vec<f64>, zones: usize, time_steps: usize) -> Self {
        debug_assert_eq!(demand.len(), zones * zones * time_steps);
        debug_assert!(demand.iter().all(|x| *x >= 0.));

        TimeDependentODM {
            demand: demand.into_boxed_slice(),
            zones,
            time_steps,
        }
    }

    pub fn zones(&self) -> usize {
        self.zones
    }

    pub fn time_steps(&self) -> usize {
        self.time_steps
    }

    pub fn demand(&self, origin: usize, destination: usize, time: usize) -> f64 {
        debug_assert!(origin < self.zones);
        debug_assert!(destination < self.zones);
        debug_assert!(time < self.time_steps);

        self.demand[time * self.zones * self.zones + origin * self.zones + destination]
    }

    pub fn scale(&mut self, scale: f64) {
        debug_assert!(scale >= 0.);

        self.demand.iter_mut().for_each(|x| *x *= scale);
    }

    pub fn from_static_uniform(odm: &ODM, time_steps: usize) -> Self {
        let distribution = vec![1. / time_steps as f64; time_steps];
        Self::from_static(odm, &distribution)
    }

    pub fn from_static_gaussian(odm: &ODM, time_steps: usize) -> Self {
        let mut distribution = vec![0.; time_steps];
        let std = time_steps as f64 / 6.;
        let mean = (time_steps - 1) as f64 / 2.;

        let mut sum = 0.;
        for (i, val) in distribution.iter_mut().enumerate() {
            let x = i as f64 - mean;
            *val = (-x * x / (2. * std * std)).exp();
            sum += *val;
        }

        distribution.iter_mut().for_each(|x| *x /= sum);

        Self::from_static(odm, &distribution)
    }

    pub fn from_static(odm: &ODM, distribution: &[f64]) -> Self {
        debug_assert_eq!(odm.rows(), odm.cols());
        debug_assert!(odm.data().iter().all(|x| *x >= 0.));
        debug_assert!(distribution.iter().all(|x| *x >= 0.));
        debug_assert!((distribution.iter().sum::<f64>() - 1.).abs() < 1e-10);

        let time_steps = distribution.len();
        let zones = odm.rows();

        let mut demand = vec![0.0; odm.rows() * odm.cols() * time_steps];

        for i in 0..zones {
            for j in 0..zones {
                let static_demand = odm[(i, j)];

                for t in 0..distribution.len() {
                    demand[t * zones * zones + i * zones + j] = static_demand * distribution[t];
                }
            }
        }

        TimeDependentODM {
            demand: demand.into_boxed_slice(),
            zones,
            time_steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tdodm = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);

        assert_eq!(tdodm.demand, vec![1., 2., 3., 4., 5., 6., 7., 8.].into());
        assert_eq!(tdodm.zones(), 2);
        assert_eq!(tdodm.time_steps(), 2);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_new_bad1() {
        let _ = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 1);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_new_bad2() {
        let _ = TimeDependentODM::new(vec![-1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);
    }

    #[test]
    fn test_zones() {
        let tdodm = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);

        assert_eq!(tdodm.zones(), 2);
    }

    #[test]
    fn test_time_steps() {
        let tdodm = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);

        assert_eq!(tdodm.time_steps(), 2);
    }

    #[test]
    fn test_demand() {
        let tdodm = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);

        assert_eq!(tdodm.demand(0, 0, 0), 1.);
        assert_eq!(tdodm.demand(0, 0, 1), 5.);
        assert_eq!(tdodm.demand(1, 1, 1), 8.);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_demand_bad1() {
        let tdodm = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);
        tdodm.demand(3, 0, 0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_demand_bad2() {
        let tdodm = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);
        tdodm.demand(0, 3, 0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_demand_bad3() {
        let tdodm = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);
        tdodm.demand(0, 0, 3);
    }

    #[test]
    fn test_scale() {
        let mut tdodm = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);

        tdodm.scale(2.);

        assert_eq!(tdodm.demand(0, 0, 0), 2.);
        assert_eq!(tdodm.demand(0, 0, 1), 10.);
        assert_eq!(tdodm.demand(1, 1, 1), 16.);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_scale_bad() {
        let mut tdodm = TimeDependentODM::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2, 2);

        tdodm.scale(-1.);
    }

    #[test]
    fn test_from_static_uniform() {
        let mut matrix = Matrix::new(2, 2, 0.);
        matrix[(0, 0)] = 1.;
        matrix[(1, 1)] = 4.;

        let tdodm = TimeDependentODM::from_static_uniform(&matrix, 4);

        assert_eq!(tdodm.zones(), 2);
        assert_eq!(tdodm.time_steps(), 4);
        assert_eq!(tdodm.demand(0, 0, 0), 0.25);
        assert_eq!(tdodm.demand(0, 0, 3), 0.25);
        assert_eq!(tdodm.demand(1, 1, 0), 1.);
        assert_eq!(tdodm.demand(1, 1, 3), 1.);
    }

    #[test]
    fn test_from_static_gaussian() {
        let mut matrix = Matrix::new(2, 2, 0.);
        matrix[(0, 0)] = 42.;

        let tdodm = TimeDependentODM::from_static_gaussian(&matrix, 4);

        assert_eq!(tdodm.zones(), 2);
        assert_eq!(tdodm.time_steps(), 4);
        assert!(tdodm.demand(0, 0, 0) < tdodm.demand(0, 0, 1));
        assert_eq!(tdodm.demand(0, 0, 1), tdodm.demand(0, 0, 2));
        assert!(tdodm.demand(0, 0, 2) > tdodm.demand(0, 0, 3));
        assert!({
            let mut sum = 0.;
            for i in 0..4 {
                sum += tdodm.demand(0, 0, i);
            }
            sum == 42.
        })
    }

    #[test]
    fn test_from_static() {
        let mut matrix = Matrix::new(2, 2, 0.);
        matrix[(0, 0)] = 42.;

        let distribution = [0.1, 0.2, 0.3, 0.4];
        let tdodm = TimeDependentODM::from_static(&matrix, &distribution);

        assert_eq!(tdodm.zones(), 2);
        assert_eq!(tdodm.time_steps(), 4);
        assert_eq!(tdodm.demand(0, 0, 0), 42. * 0.1);
        assert_eq!(tdodm.demand(0, 0, 1), 42. * 0.2);
        assert_eq!(tdodm.demand(0, 0, 2), 42. * 0.3);
        assert_eq!(tdodm.demand(0, 0, 3), 42. * 0.4);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_from_static_bad1() {
        let mut matrix = Matrix::new(3, 2, 0.);
        matrix[(0, 0)] = 42.;

        let distribution = [0.1, 0.2, 0.3, 0.4];
        let _ = TimeDependentODM::from_static(&matrix, &distribution);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_from_static_bad2() {
        let mut matrix = Matrix::new(2, 2, 0.);
        matrix[(0, 0)] = -42.;

        let distribution = [0.1, 0.2, 0.3, 0.4];
        let _ = TimeDependentODM::from_static(&matrix, &distribution);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_from_static_bad3() {
        let mut matrix = Matrix::new(2, 2, 0.);
        matrix[(0, 0)] = 42.;

        let distribution = [-0.1, 0.2, 0.3, 0.4];
        let _ = TimeDependentODM::from_static(&matrix, &distribution);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_from_static_bad4() {
        let mut matrix = Matrix::new(2, 2, 0.);
        matrix[(0, 0)] = 42.;

        let distribution = [0.3, 0.2, 0.3, 0.4];
        let _ = TimeDependentODM::from_static(&matrix, &distribution);
    }
}
