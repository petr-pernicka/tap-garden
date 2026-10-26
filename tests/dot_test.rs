mod common;
use crate::common::{load_network, time_it};
use anyhow::Result;
use tap_garden::dynamic::convergence;
use tap_garden::dynamic::equilibrium::AON;
use tap_garden::dynamic::loading::scheme::{BasicDnl, DnlScheme};
use tap_garden::dynamic::tdsp::DOT;
use tap_garden::dynamic::travel_time::link_travel_times;

#[test]
fn dot_test() -> Result<()> {
    dot_test_parameterized("./data/SiouxFalls", 60, 1.0, 30, 1.0, 1.0)?;
    dot_test_parameterized("./data/ChicagoSketch", 800, 0.1, 30, 1.0, 0.5)?;

    Ok(())
}

fn dot_test_parameterized(
    network_dir: &str,
    time_steps: usize,
    step_size: f64,
    odm_steps: usize,
    odm_scale: f64,
    fftt_scale: f64,
) -> Result<()> {
    let (network, odm, free_flow_travel_time) =
        load_network(network_dir, step_size, odm_steps, odm_scale, fftt_scale)?;

    // Run the algorithm
    let dnl = BasicDnl;
    let dot = DOT::new();

    let routing_params = time_it!(
        "Running AON",
        AON::compute_routing_parameters(&network, &free_flow_travel_time, time_steps)
    );

    let cumulative_flows = time_it!(
        "Running DNL",
        dnl.run(&network, time_steps, step_size, &routing_params)
    );

    let travel_times = time_it!(
        "Computing travel times",
        cumulative_flows
            .iter()
            .enumerate()
            .map(|(i, (cinflow, coutflow))| {
                link_travel_times(cinflow, coutflow, step_size, free_flow_travel_time[i])
            })
            .collect::<Vec<_>>()
    );

    let (_costs, indices) = time_it!(
        "Running DOT",
        dot.run::<true>(&network, time_steps, step_size, &travel_times)
    );

    let total_demand = odm.iter().sum::<f64>();
    let tstt = convergence::total_system_travel_time(&cumulative_flows, step_size);
    let sptt = convergence::shortest_path_travel_time(
        &network,
        &cumulative_flows,
        &indices,
        time_steps,
        step_size,
        &odm,
        &free_flow_travel_time,
    );
    let rg = convergence::relative_gap(tstt, sptt);
    let aec = convergence::average_excess_cost(tstt, sptt, total_demand);

    println!("TSTT: {tstt}");
    println!("SPTT: {sptt}");
    println!("RG: {rg}");
    println!("AEC: {aec}");
    assert!(tstt >= sptt - 1e8);

    Ok(())
}
