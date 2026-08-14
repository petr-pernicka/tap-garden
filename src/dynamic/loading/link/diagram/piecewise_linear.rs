use crate::dynamic::loading::link::diagram::FundamentalDiagram;

pub struct PiecewiseLinearFd {
    length: f64,
    capacity: f64,
    points: Vec<(f64, f64)>,
}

impl FundamentalDiagram for PiecewiseLinearFd {
    fn length(&self) -> f64 {
        self.length
    }

    fn capacity(&self) -> f64 {
        self.capacity
    }

    fn jam_density(&self) -> f64 {
        // UNREACHABLE: self.points has length of at least 3.
        let [.., last] = &self.points[..] else {
            unreachable!()
        };

        last.0
    }

    fn free_flow_speed(&self) -> f64 {
        // UNREACHABLE: self.points has length of at least 3.
        let [first, second, ..] = &self.points[..] else {
            unreachable!()
        };

        // Rise-over-run of the 1st segment.
        (second.1 - first.1) / (second.0 - first.0)
    }

    fn backward_wave_speed(&self) -> f64 {
        // UNREACHABLE: self.points has length of at least 3.
        let [.., pre_last, last] = &self.points[..] else {
            unreachable!()
        };

        // Rise-over-run of the last segment (last.1 is 0).
        pre_last.1 / (last.0 - pre_last.0)
    }
}

impl PiecewiseLinearFd {
    pub fn new(length: f64, capacity: f64, points: Vec<(f64, f64)>) -> PiecewiseLinearFd {
        debug_assert!(length >= 0.0);
        debug_assert!(points.len() >= 3);
        debug_assert!(capacity >= 0.0);
        debug_assert_eq!(
            capacity,
            points.iter().map(|x| x.1).reduce(f64::max).unwrap()
        );
        debug_assert_eq!(points[0], (0., 0.));
        debug_assert_eq!(points.last().unwrap().1, 0.);
        debug_assert!({
            let mut is_slope_decreasing = true;
            let mut prev_slope = (points[1].1 - points[0].1) / (points[1].0 - points[0].0);
            for i in 2..points.len() {
                let slope = (points[i].1 - points[i - 1].1) / (points[i].0 - points[i - 1].0);
                if slope >= prev_slope {
                    is_slope_decreasing = false;
                    break;
                }
                prev_slope = slope;
            }

            is_slope_decreasing
        });

        Self {
            length,
            capacity,
            points,
        }
    }

    pub fn points(&self) -> &[(f64, f64)] {
        &self.points
    }
}
