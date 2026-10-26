mod common;
use crate::common::{load_network, time_it};
use anyhow::Result;
use tap_garden::dynamic::equilibrium::AON;
use tap_garden::dynamic::loading::scheme::{BasicDnl, DnlScheme};

#[test]
fn dnl_test() -> Result<()> {
    dnl_test_parameterized("./data/SiouxFalls", 60, 1.0, 30, 1.0, 1.0)?;
    dnl_test_parameterized("./data/ChicagoSketch", 800, 0.1, 30, 1.0, 0.5)?;

    Ok(())
}

fn dnl_test_parameterized(
    network_dir: &str,
    time_steps: usize,
    step_size: f64,
    odm_steps: usize,
    odm_scale: f64,
    fftt_scale: f64,
) -> Result<()> {
    let (network, _, free_flow_travel_time) =
        load_network(network_dir, step_size, odm_steps, odm_scale, fftt_scale)?;

    let dnl = BasicDnl;

    let routing_params = time_it!(
        "Running AON",
        AON::compute_routing_parameters(&network, &free_flow_travel_time, time_steps)
    );

    let cumulative_flows = time_it!(
        "Running DNL",
        dnl.run(&network, time_steps, step_size, &routing_params)
    );

    assert_eq!(cumulative_flows.len(), network.links().len());
    assert!(
        cumulative_flows
            .iter()
            .all(|(cinflow, coutflow)| cinflow.num_points() == time_steps + 1
                && coutflow.num_points() == time_steps + 1)
    );

    Ok(())
}
