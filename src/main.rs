use tap_garden::io;
use anyhow::Result;

fn main() -> Result<()> {
    let (_, _, _, _, _) = io::load_dir("./data/SiouxFalls")?;

    Ok(())
}
