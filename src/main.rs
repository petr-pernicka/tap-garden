use tap_garden::io;

fn main() {
    let (network, odm, _, _, _) = io::load_dir("./data/SiouxFalls").unwrap();
}
