use crate::dynamic::Cvn;

pub fn link_travel_times(
    cinflow: &Cvn,
    coutflow: &Cvn,
    step_size: f64,
    link_fftt: f64,
) -> Vec<f64> {
    debug_assert_eq!(cinflow.num_points(), coutflow.num_points());
    debug_assert!(step_size > 0.0);
    debug_assert!(link_fftt > 0.0);

    let mut travel_times = vec![f64::INFINITY; cinflow.num_points()];

    for tn in 0..travel_times.len() {
        // The number of a vehicle, that entered the link at time t
        let n = cinflow[tn];

        // Arrival time - the time at which this vehicle left the link
        let mut arrival_time = f64::INFINITY;
        for tn2 in tn..travel_times.len() {
            let m2 = coutflow[tn2];

            if (n - m2).abs() < 1e-8 {
                // The exit time is exactly integer
                arrival_time = tn2 as f64;
                break;
            } else if n < m2 {
                // The exit time must be interpolated
                let tn1 = tn2 - 1;
                let m1 = coutflow[tn1];

                arrival_time = tn1 as f64 + (n - m1) / (m2 - m1);
                break;
            }
        }

        // Travel time cannot be lower than free flow travel time
        travel_times[tn] = ((arrival_time - tn as f64) * step_size).max(link_fftt);
    }

    travel_times
}

pub fn link_travel_time(
    departure_time: f64,
    cinflow: &Cvn,
    coutflow: &Cvn,
    step_size: f64,
    link_fftt: f64,
) -> f64 {
    debug_assert!(departure_time >= 0.0);
    debug_assert_eq!(cinflow.num_points(), coutflow.num_points());
    debug_assert!(step_size > 0.0);
    debug_assert!(link_fftt > 0.0);

    let departure_time_rounded = departure_time.round();

    // Compute the number n of vehicle that entered at time departure_time
    let n = if (departure_time_rounded - departure_time).abs() < 1e-8 {
        cinflow[departure_time_rounded as usize]
    } else {
        let tn1 = departure_time.floor() as usize;
        let tn2 = tn1 + 1;
        let p = departure_time.fract();

        (1.0 - p) * cinflow[tn1] + p * cinflow[tn2]
    };

    // Arrival time - the time at which this vehicle left the link
    let mut arrival_time = f64::INFINITY;
    for tn2 in (departure_time as usize)..cinflow.num_points() {
        let m2 = coutflow[tn2];

        if (n - m2).abs() < 1e-8 {
            arrival_time = tn2 as f64;
            break;
        } else if n < m2 {
            // The exit time must be interpolated
            let tn1 = tn2 - 1;
            let m1 = coutflow[tn1];

            arrival_time = tn1 as f64 + (n - m1) / (m2 - m1);
            break;
        }
    }

    ((arrival_time - departure_time) * step_size).max(link_fftt)
}
