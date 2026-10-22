use anyhow::Result;
use std::rc::Rc;
use std::time::Instant;
use tap_garden::dynamic::equilibrium::{DynamicUserEquilibrium, AON, MSA};
use tap_garden::dynamic::loading::link::diagram::{OmitParam, TrapezoidalFd, FundamentalDiagram};
use tap_garden::dynamic::loading::link::model::{Connector, LinkModel, Ltm};
use tap_garden::dynamic::loading::node::model::{NodeModel, TampereUnsignalized};
use tap_garden::dynamic::loading::node::routing::{MofRoutingModel, RoutedModel, Zone};
use tap_garden::dynamic::loading::scheme::{BasicDnl, DnlScheme};
use tap_garden::dynamic::routing::MixtureOutgoingFractions;
use tap_garden::dynamic::tdsp::DOT;
use tap_garden::dynamic::travel_time::link_travel_times;
use tap_garden::dynamic::{DynamicNetwork, TimeDependentODM, convergence};
use tap_garden::io;

#[test]
fn dta_test() -> Result<()> {
    dta_test_parameterized("./data/SiouxFalls", 60, 1.0, 30)?;

    Ok(())
}

fn dta_test_parameterized(
    network_dir: &str,
    time_steps: usize,
    step_size: f64,
    odm_steps: usize,
) -> Result<()> {
    let (node_data, link_data, demand_data) = io::gmns::load_gmns_dir(network_dir)?;
    let (network, odm, id2index) = io::parse::from_gmns_data(&node_data, &link_data, &demand_data);
    let (network, Some(id2index)) = network.expand(Some(id2index)) else {
        unreachable!()
    };

    let mut free_flow_times = vec![0.0001; network.links().len()];
    let mut fds = vec![None; network.links().len()];
    for ld in link_data {
        let i = network
            .links()
            .iter()
            .position(|link| {
                link.head() == id2index[&ld.to_node_id] && link.tail() == id2index[&ld.from_node_id]
            })
            .unwrap();

        free_flow_times[i] = ld.vdf_fftt.max(0.1);

        let length = ld.vdf_length_mi.max(0.1);
        let free_flow_time = (ld.vdf_fftt / 2.0).max(0.1);
        let free_flow_speed = length / free_flow_time;
        fds[i] = Some(TrapezoidalFd::triangular(
            length,
            OmitParam::JamDensity {
                capacity: 1.25 * ld.capacity,
                free_flow_speed,
                backward_wave_speed: free_flow_speed / 3.0,
            },
        ))
    }

    let mut link_models = vec![];
    for link in network.links() {
        let model: Box<dyn LinkModel<TrapezoidalFd>> = if fds[link.index()].is_none() {
            Box::new(Connector)
        } else {
            Box::new(Ltm)
        };

        link_models.push(model);
    }

    let fds = fds
        .into_iter()
        .map(|fd| {
            if let Some(fd) = fd {
                fd
            } else {
                TrapezoidalFd::new(0.0, 10_000.0, 0.0, 0.0, 0.0)
            }
        })
        .collect::<Vec<_>>();

    let odm = Rc::new(TimeDependentODM::from_static_gaussian(&odm, odm_steps));

    let zones = network
        .zones()
        .map(|i| Zone::new(i, odm.clone()))
        .collect::<Vec<_>>();

    let mut routed_models: Vec<Box<dyn RoutedModel<MixtureOutgoingFractions>>> = vec![];
    for n in network.zones().len()..network.nodes().len() {
        let routed_model = MofRoutingModel::new(n);
        routed_models.push(Box::new(routed_model));
    }

    let mut node_models: Vec<Box<dyn NodeModel>> = vec![];
    for n in network.zones().len()..network.nodes().len() {
        let priorities = network
            .incoming_links(n)
            .iter()
            .map(|link| fds[link.index()].capacity())
            .collect::<Vec<_>>();

        let node_model = TampereUnsignalized::new(priorities);
        node_models.push(Box::new(node_model));
    }

    let network = DynamicNetwork::new(
        network,
        zones,
        vec![],
        routed_models,
        node_models,
        link_models,
        fds,
    );

    let dnl = BasicDnl;
    let dot = DOT::new();
    let mut due = MSA::new();

    print!("Running AON...");
    let tick = Instant::now();
    let mut routing_params = AON::compute_routing_parameters(&network, &free_flow_times, time_steps);
    let tock = Instant::now();
    println!(" ({:.2}s)", (tock - tick).as_secs_f64());

    println!("Running DNL...");
    let tick = Instant::now();
    let cumulative_flows = dnl.run(&network, time_steps, step_size, &routing_params);
    let tock = Instant::now();
    println!("({:.2}s)", (tock - tick).as_secs_f64());

    println!("Computing travel times...");
    let tick = Instant::now();
    let travel_times = cumulative_flows
        .iter()
        .enumerate()
        .map(|(i, (cinflow, coutflow))| {
            link_travel_times(cinflow, coutflow, step_size, free_flow_times[i])
        })
        .collect::<Vec<_>>();
    let tock = Instant::now();
    println!("({:.2}s)", (tock - tick).as_secs_f64());

    print!("Running DOT...");
    let tick = Instant::now();
    let (_costs, indices) = dot.run::<true>(&network, time_steps, step_size, &travel_times);
    let tock = Instant::now();
    println!(" ({:.2}s)", (tock - tick).as_secs_f64());

    let total_demand = odm.iter().sum::<f64>();
    let tstt = convergence::total_system_travel_time(&cumulative_flows, step_size);
    let sptt = convergence::shortest_path_travel_time(
        &network,
        &cumulative_flows,
        &indices,
        time_steps,
        step_size,
        &odm,
        &free_flow_times,
    );
    let rg = convergence::relative_gap(tstt, sptt);
    let aec = convergence::average_excess_cost(tstt, sptt, total_demand);

    println!("TSTT: {tstt}");
    println!("SPTT: {sptt}");
    println!("RG: {rg}");
    println!("AEC: {aec}");

    for i in 0..10 {
        routing_params = due.compute_routing_parameters(&network, time_steps, routing_params, &indices, &_costs);
    }

    Ok(())
}
