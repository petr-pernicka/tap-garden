mod common;
use crate::common::{load_network, time_it};
use anyhow::Result;
use tap_garden::dynamic::convergence;
use tap_garden::dynamic::equilibrium::{AON, DynamicUserEquilibrium, MSA};
use tap_garden::dynamic::loading::scheme::{BasicDnl, DnlScheme};
use tap_garden::dynamic::tdsp::DOT;
use tap_garden::dynamic::travel_time::link_travel_times;

#[test]
fn dta_test() -> Result<()> {
    dta_test_parameterized("./data/SiouxFalls", 120, 1.0, 30, 1.0, 10.0, 100)?;
    dta_test_parameterized("./data/ChicagoSketch", 800, 0.1, 30, 0.5, 1.0, 10)?;

    Ok(())
}

fn dta_test_parameterized(
    network_dir: &str,
    time_steps: usize,
    step_size: f64,
    odm_steps: usize,
    fftt_scale: f64,
    odm_scale: f64,
    msa_steps: usize,
) -> Result<()> {
    let (network, odm, free_flow_travel_time) =
        load_network(network_dir, step_size, odm_steps, odm_scale, fftt_scale)?;

    let dnl = BasicDnl;
    let dot = DOT::new();
    let mut due = MSA::new();

    time_it!("Running DTA", {
        let mut routing_params = time_it!(
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

        let (mut _costs, mut indices) = time_it!(
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
        println!("RG: {rg:.2e}");
        println!("AEC: {aec:.2e}");

        for i in 1..=msa_steps {
            println!("======= Iteration {i} =======");

            routing_params = time_it!(
                "Running MSA",
                due.compute_routing_parameters(
                    &network,
                    time_steps,
                    routing_params,
                    &indices,
                    &_costs
                )
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

            (_costs, indices) = time_it!(
                "Running DOT",
                dot.run::<true>(&network, time_steps, step_size, &travel_times)
            );

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
            println!("RG: {rg:.2e}");
            println!("AEC: {aec:.2e}");
        }
    });

    Ok(())
}
