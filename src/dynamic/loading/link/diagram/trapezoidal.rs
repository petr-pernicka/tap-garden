use crate::dynamic::loading::link::diagram::fd::FundamentalDiagram;

pub struct TrapezoidalFd {
    length: f64,
    capacity: f64,
    jam_density: f64,
    free_flow_speed: f64,
    backward_wave_speed: f64,
}

impl FundamentalDiagram for TrapezoidalFd {
    fn length(&self) -> f64 {
        self.length
    }

    fn capacity(&self) -> f64 {
        self.capacity
    }

    fn jam_density(&self) -> f64 {
        self.jam_density
    }

    fn free_flow_speed(&self) -> f64 {
        self.free_flow_speed
    }

    fn backward_wave_speed(&self) -> f64 {
        self.backward_wave_speed
    }
}

impl TrapezoidalFd {
    pub fn new(
        length: f64,
        capacity: f64,
        jam_density: f64,
        free_flow_speed: f64,
        backward_wave_speed: f64,
    ) -> Self {
        debug_assert!(length >= 0.0);
        debug_assert!(capacity >= 0.0);
        debug_assert!(jam_density >= 0.0);
        debug_assert!(free_flow_speed >= 0.0);
        debug_assert!(backward_wave_speed >= 0.0);

        Self {
            length,
            capacity,
            jam_density,
            free_flow_speed,
            backward_wave_speed,
        }
    }

    pub fn new_triangular(length: f64, params: OmitParam) -> Self {
        match params {
            OmitParam::JamDensity {
                capacity,
                free_flow_speed,
                backward_wave_speed,
            } => {
                let jam_density = capacity * (backward_wave_speed + free_flow_speed)
                    / (backward_wave_speed * free_flow_speed);

                Self::new(
                    length,
                    capacity,
                    jam_density,
                    free_flow_speed,
                    backward_wave_speed,
                )
            }
            OmitParam::BackwardWaveSpeed {
                capacity,
                jam_density,
                free_flow_speed,
            } => {
                let backward_wave_speed =
                    -(capacity * free_flow_speed) / (capacity - free_flow_speed * jam_density);

                Self::new(
                    length,
                    capacity,
                    jam_density,
                    free_flow_speed,
                    backward_wave_speed,
                )
            }
        }
    }
}

pub enum OmitParam {
    JamDensity {
        capacity: f64,
        free_flow_speed: f64,
        backward_wave_speed: f64,
    },
    BackwardWaveSpeed {
        capacity: f64,
        jam_density: f64,
        free_flow_speed: f64,
    },
}
