mod network_loading;

macro_rules! time_it {
    ($label:expr, $expr:expr) => {{
        println!("{}...", $label);
        let tick = ::std::time::Instant::now();
        let result = $expr;
        let tock = ::std::time::Instant::now();
        println!("({:.2}s)", (tock - tick).as_secs_f64());
        result
    }};
}

pub(crate) use time_it;
pub(crate) use network_loading::load_network;
