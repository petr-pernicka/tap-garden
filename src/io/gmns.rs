use anyhow::Result;
use csv::Reader;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize)]
pub struct NodeData {
    pub node_id: usize,
    pub zone_id: Option<usize>,
    pub x_coord: f64,
    pub y_coord: f64,
}

#[derive(Deserialize)]
pub struct DemandData {
    pub o_zone_id: usize,
    pub d_zone_id: usize,
    pub volume: f64,
}

#[derive(Deserialize)]
pub struct LinkData {
    pub link_id: usize,
    pub from_node_id: usize,
    pub to_node_id: usize,
    pub capacity: f64,
    pub vdf_fftt: f64,
    pub vdf_length_mi: f64,
}

pub fn load_gmns_dir(dir_path: &str) -> Result<(Vec<NodeData>, Vec<LinkData>, Vec<DemandData>)> {
    let dir_path = Path::new(dir_path);

    Ok((
        load_gmns_file(&File::open(dir_path.join("node.csv"))?)?,
        load_gmns_file(&File::open(dir_path.join("link.csv"))?)?,
        load_gmns_file(&File::open(dir_path.join("demand.csv"))?)?,
    ))
}

pub fn load_gmns_file<T>(file: &File) -> Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let mut reader = Reader::from_reader(file);

    let mut result = Vec::new();
    for record in reader.deserialize() {
        let record: T = record?;
        result.push(record);
    }

    Ok(result)
}
