use crate::network::{Network, NodeID, NodeIndex};
use crate::static_::ODM;
use anyhow::Result;
use csv::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub fn load_dir(
    dir_path: &str,
) -> Result<(
    Network,
    ODM,
    HashMap<NodeID, NodeIndex>,
    HashMap<NodeID, (f64, f64)>,
    Vec<HashMap<String, String>>,
)> {
    let dir_path = Path::new(dir_path);
    load(
        dir_path.join("link.csv").to_str().unwrap(),
        dir_path.join("node.csv").to_str().unwrap(),
        dir_path.join("demand.csv").to_str().unwrap(),
    )
}

pub fn load(
    network_file_path: &str,
    node_file_path: &str,
    odm_file_path: &str,
) -> Result<(
    Network,
    ODM,
    HashMap<NodeID, NodeIndex>,
    HashMap<NodeID, (f64, f64)>,
    Vec<HashMap<String, String>>,
)> {
    let network_file = File::open(network_file_path)?;
    let node_file = File::open(node_file_path)?;
    let odm_file = File::open(odm_file_path)?;

    let (zones, coords) = load_nodes(&node_file)?;
    let link_data = load_links(&network_file)?;

    let links = link_data.keys().copied().collect::<Vec<_>>();
    let (network, id2index) = Network::new(&links, &zones);

    let odm = load_odm(&odm_file, zones.len(), &id2index)?;

    let mut link_data_2 = vec![HashMap::default(); links.len()];
    link_data.into_iter().for_each(|((tail_id, head_id), data)| {
        let link_index = network
            .outgoing_links(id2index[&tail_id])
            .iter()
            .find(|link| link.head() == id2index[&head_id])
            .unwrap()
            .index();

        link_data_2[link_index] = data;
    });

    Ok((network, odm, id2index, coords, link_data_2))
}

fn load_nodes(node_file: &File) -> Result<(Vec<NodeID>, HashMap<NodeID, (f64, f64)>)> {
    let mut reader = Reader::from_reader(node_file);

    let mut zones = Vec::new();
    let mut coords = HashMap::new();
    for record in reader.deserialize() {
        let record: HashMap<String, String> = record?;

        let node = record["node_id"].parse::<NodeID>()?;
        let zone = record["zone_id"].parse::<NodeID>();
        let x_coord = record["x_coord"].parse::<f64>()?;
        let y_coord = record["y_coord"].parse::<f64>()?;

        if let Ok(zone) = zone {
            zones.push(zone);
        }

        coords.insert(node, (x_coord, y_coord));
    }

    Ok((zones, coords))
}

fn load_links(network_file: &File) -> Result<HashMap<(NodeID, NodeID), HashMap<String, String>>> {
    let mut reader = Reader::from_reader(network_file);

    let mut links = HashMap::new();
    for record in reader.deserialize() {
        let record: HashMap<String, String> = record?;

        let tail = record["from_node_id"].parse::<NodeID>()?;
        let head = record["to_node_id"].parse::<NodeID>()?;

        links.insert((tail, head), record);
    }

    Ok(links)
}

fn load_odm(
    odm_file: &File,
    num_zones: usize,
    id2index: &HashMap<NodeID, NodeIndex>,
) -> Result<ODM> {
    let mut reader = Reader::from_reader(odm_file);

    let mut odm = ODM::new(num_zones, num_zones, 0.);
    for record in reader.records() {
        let record = record?;

        let origin = record[0].parse::<NodeID>()?;
        let origin = id2index[&origin];
        let destination = record[1].parse::<NodeID>()?;
        let destination = id2index[&destination];
        let demand = record[2].parse::<f64>()?;

        odm[(origin, destination)] = demand;
    }

    Ok(odm)
}
