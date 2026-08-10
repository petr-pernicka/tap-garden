use crate::data::matrix::Matrix;

pub struct TimeDependentODM {
    demand: Box<[f64]>,
    zones: u32,
    time_steps: u32,
}

impl TimeDependentODM {
    pub fn new(demand: Vec<f64>, zones: u32, time_steps: u32) -> Self {
        debug_assert_eq!(demand.len(), (zones * zones * time_steps) as usize);
        debug_assert!(demand.iter().all(|x| *x >= 0.));

        TimeDependentODM {
            demand: demand.into_boxed_slice(),
            zones,
            time_steps,
        }
    }

    pub fn zones(&self) -> u32 {
        self.zones
    }

    pub fn time_steps(&self) -> u32 {
        self.time_steps
    }

    pub fn demand(&self, origin: u32, destination: u32, time: u32) -> f64 {
        debug_assert!(origin < self.zones);
        debug_assert!(destination < self.zones);
        debug_assert!(time < self.time_steps);

        self.demand[(time * self.zones * self.zones + origin * self.zones + destination) as usize]
    }

    pub fn scale(&mut self, scale: f64) {
        debug_assert!(scale >= 0.);

        self.demand.iter_mut().for_each(|x| *x *= scale);
    }

    pub fn from_static_uniform(odm: &Matrix<f64>, time_steps: u32) -> Self {
        let distribution = vec![1. / time_steps as f64; time_steps as usize];
        Self::from_static(odm, &distribution)
    }

    pub fn from_static_gaussian(odm: &Matrix<f64>, time_steps: u32) -> Self {
        let mut distribution = vec![0.; time_steps as usize];
        let std = time_steps as f64 / 6.;
        let mean = (time_steps - 1) as f64 / 2.;

        let mut sum = 0.;
        for (i, val) in distribution.iter_mut().enumerate() {
            let x = i as f64 - mean;
            *val = (- x * x / (2. * std * std)).exp();
            sum += *val;
        }

        distribution.iter_mut()
            .for_each(|x| *x /= sum);

        Self::from_static(odm, &distribution)
    }

    pub fn from_static(odm: &Matrix<f64>, distribution: &[f64]) -> Self {
        debug_assert_eq!(odm.rows(), odm.cols());
        debug_assert!(odm.data().iter().all(|x| *x >= 0.));
        debug_assert!(distribution.iter().all(|x| *x >= 0.));
        debug_assert!((distribution.iter().sum::<f64>() - 1.).abs() < 1e-10);

        let time = distribution.len();
        let zones = odm.rows();

        let mut demand = vec![0.0; odm.rows() * odm.cols() * time];

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
            zones: zones as u32,
            time_steps: time as u32,
        }
    }
}
