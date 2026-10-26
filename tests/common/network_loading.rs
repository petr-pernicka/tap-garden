use anyhow::Result;
use std::rc::Rc;
use tap_garden::dynamic::loading::link::diagram::{FundamentalDiagram, OmitParam, TrapezoidalFd};
use tap_garden::dynamic::loading::link::model::{Connector, LinkModel, Ltm};
use tap_garden::dynamic::loading::node::model::{NodeModel, TampereUnsignalized};
use tap_garden::dynamic::loading::node::routing::{MofRoutingModel, RoutedModel, Zone};
use tap_garden::dynamic::routing::MixtureOutgoingFractions;
use tap_garden::dynamic::{DynamicNetwork, TimeDependentODM};
use tap_garden::io;

pub fn load_network(
    network_dir: &str,
    step_size: f64,
    odm_steps: usize,
    odm_scale: f64,
    fftt_scale: f64,
) -> Result<(
    DynamicNetwork<MixtureOutgoingFractions, TrapezoidalFd>,
    Rc<TimeDependentODM>,
    Vec<f64>,
)> {
    let (node_data, link_data, demand_data) = io::gmns::load_gmns_dir(network_dir)?;
    let (network, mut odm, id2index) =
        io::parse::from_gmns_data(&node_data, &link_data, &demand_data);
    let (network, Some(id2index)) = network.expand(Some(id2index)) else {
        unreachable!()
    };

    odm.data_mut().iter_mut().for_each(|x| *x *= odm_scale);

    // Prepare network parameters for the STA assignment
    let mut free_flow_travel_time = vec![0.0001; network.links().len()];
    let mut practical_capacity = vec![0.0; network.links().len()];
    let mut length = vec![0.0001; network.links().len()];
    let mut link_set = vec![false; network.links().len()]; // marks links from link_data

    for ld in link_data {
        let index = network
            .index_of_link(id2index[&ld.from_node_id], id2index[&ld.to_node_id])
            .unwrap();

        free_flow_travel_time[index] = ld.vdf_fftt;
        practical_capacity[index] = ld.capacity;
        length[index] = ld.vdf_length_mi;
        link_set[index] = true;
    }

    // Prepare the data for DTA
    let fds = (0..network.links().len())
        .map(|i| {
            if link_set[i] {
                let length = length[i].max(0.1);
                let capacity = 1.25 * practical_capacity[i];
                let fftt = (fftt_scale * free_flow_travel_time[i]).max(step_size);
                let free_flow_speed = length / fftt;
                let backward_wave_speed = free_flow_speed / 3.0;
                TrapezoidalFd::triangular(
                    length,
                    OmitParam::JamDensity {
                        capacity,
                        free_flow_speed,
                        backward_wave_speed,
                    },
                )
            } else {
                // FDs of connectors
                TrapezoidalFd::new(step_size, 10_000.0, f64::INFINITY, 1.0, 1.0)
            }
        })
        .collect::<Vec<_>>();

    let free_flow_travel_time = fds // Substitute with values used in the DTA
        .iter()
        .map(|fd| fd.length() / fd.free_flow_speed())
        .collect::<Vec<_>>();

    let link_models = link_set
        .iter()
        .map(|is_physical| {
            if *is_physical {
                Box::new(Ltm) as Box<dyn LinkModel<TrapezoidalFd>>
            } else {
                Box::new(Connector)
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

    Ok((network, odm, free_flow_travel_time))
}
