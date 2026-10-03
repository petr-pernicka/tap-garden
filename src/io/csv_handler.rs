use crate::network::{Network, Node};
use crate::static_::ODM;
use anyhow::Result;
use csv::{Reader, StringRecord};
use std::collections::HashMap;
use std::fs::File;

pub fn load_dir(
    dir_path: &str,
) -> Result<(
    Network,
    ODM,
    HashMap<u32, Node>,
    HashMap<u32, (f64, f64)>,
    HashMap<(u32, u32), StringRecord>,
)> {
    load(
        &(dir_path.to_owned() + "/link.csv"),
        &(dir_path.to_owned() + "/node.csv"),
        &(dir_path.to_owned() + "/demand.csv"),
    )
}

pub fn load(
    network_file_path: &str,
    node_file_path: &str,
    odm_file_path: &str,
) -> Result<(
    Network,
    ODM,
    HashMap<u32, Node>,
    HashMap<u32, (f64, f64)>,
    HashMap<(u32, u32), StringRecord>,
)> {
    let network_file = File::open(network_file_path)?;
    let node_file = File::open(node_file_path)?;
    let odm_file = File::open(odm_file_path)?;

    let (zones, coords) = load_nodes(&node_file)?;
    let link_data = load_network(&network_file)?;

    let links = link_data.keys().copied().collect::<Vec<_>>();
    let (network, id2index) = Network::new(links, &zones);

    let odm = load_odm(&odm_file, zones.len(), &id2index)?;

    Ok((network, odm, id2index, coords, link_data))
}

fn load_nodes(node_file: &File) -> Result<(Vec<u32>, HashMap<u32, (f64, f64)>)> {
    let mut reader = Reader::from_reader(node_file);

    let mut zones = Vec::new();
    let mut coords = HashMap::new();
    for record in reader.records() {
        let record = record?;

        let node = record[1].parse::<u32>()?;
        let zone = record[2].parse::<u32>();
        let x_coord = record[3].parse::<f64>()?;
        let y_coord = record[4].parse::<f64>()?;

        if let Ok(zone) = zone {
            zones.push(zone);
        }

        coords.insert(node, (x_coord, y_coord));
    }

    Ok((zones, coords))
}

fn load_network(network_file: &File) -> Result<HashMap<(u32, u32), StringRecord>> {
    let mut reader = Reader::from_reader(network_file);

    let mut links = HashMap::new();
    for record in reader.records() {
        let record = record?;

        let tail = record[2].parse::<u32>()?;
        let head = record[3].parse::<u32>()?;

        links.insert((tail, head), record);
    }

    Ok(links)
}

fn load_odm(odm_file: &File, num_zones: usize, id2index: &HashMap<u32, Node>) -> Result<ODM> {
    let mut reader = Reader::from_reader(odm_file);

    let mut odm = ODM::new(num_zones, num_zones, 0.);
    for record in reader.records() {
        let record = record?;

        let origin = record[0].parse::<u32>()?;
        let origin = id2index[&origin];
        let destination = record[1].parse::<u32>()?;
        let destination = id2index[&destination];
        let demand = record[2].parse::<f64>()?;

        odm[(origin, destination)] = demand;
    }

    Ok(odm)
}
