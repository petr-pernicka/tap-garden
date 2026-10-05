use tap_garden::io;
use anyhow::Result;

fn main() -> Result<()> {
    let (network, odm, _, _, _) = io::load_dir("./data/SiouxFalls")?;
    let network = network.expand(None);

    Ok(())
}
