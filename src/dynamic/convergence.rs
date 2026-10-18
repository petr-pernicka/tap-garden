use crate::dynamic::travel_time::link_travel_time;
use crate::dynamic::{Cvn, OutgoingLinkIndices, TimeDependentODM};
use crate::network::Network;

pub fn total_system_travel_time(cumulative_flows: &Vec<(Cvn, Cvn)>, step_size: f64) -> f64 {
    debug_assert!(step_size > 0.0);

    let mut tstt = 0.0;

    for (inflow, outflow) in cumulative_flows {
        // The area between the cumulative curves, i.e. the number of vehicles on the
        // link integrated over time.
        tstt += inflow.integrate() - outflow.integrate();
    }

    tstt
}

pub fn shortest_path_travel_time(
    network: &Network,
    cumulative_flows: &Vec<(Cvn, Cvn)>,
    indices: &OutgoingLinkIndices,
    time_steps: usize,
    step_size: f64,
    tdodm: &TimeDependentODM,
    fftt: &[f64],
) -> f64 {
    let mut sptt = 0.0;

    for origin in 0..tdodm.zones() {
        for destination in 0..tdodm.zones() {
            for tn in 0..tdodm.time_steps() {
                let demand = tdodm.demand(origin, destination, tn);
                if demand > 0.0 {
                    let mut cost = 0.0;
                    let mut time = tn as f64;
                    let mut tail = origin;

                    while tail != destination {
                        let j = if time >= (time_steps - 1) as f64 {
                            indices.get_index(tail, time_steps - 1, destination)
                        } else {
                            indices.get_index(tail, time as usize, destination)
                        };

                        let link = &network.outgoing_links(tail)[j as usize];

                        let link_travel_time = link_travel_time(
                            time,
                            &cumulative_flows[link.index()].0,
                            &cumulative_flows[link.index()].1,
                            step_size,
                            fftt[link.index()],
                        );

                        cost += link_travel_time;
                        time += link_travel_time / step_size;
                        tail = link.head();
                    }

                    sptt += cost.min((time_steps - tn) as f64 * step_size);
                }
            }
        }
    }

    sptt
}

pub fn relative_gap(tstt: f64, sptt: f64) -> f64 {
    (tstt - sptt) / sptt
}

pub fn average_excess_cost(tstt: f64, sptt: f64, total_demand: f64) -> f64 {
    (tstt - sptt) / total_demand
}
