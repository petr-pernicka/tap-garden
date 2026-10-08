use tap_garden::io;
use anyhow::Result;

fn main() -> Result<()> {
    let (node_data, link_data, demand_data) = io::gmns::load_gmns_dir("./data/SiouxFalls")?;
    let (_network, _odm, _id2index) = io::parse::from_gmns_data(&node_data, &link_data, &demand_data);

    Ok(())
}
