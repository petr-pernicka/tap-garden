use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;
use tap_garden::io;
use tap_garden::static_::sssp::SSSP;

fn one_to_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("SSSP::one_to_all");

    for dir in ["./data/SiouxFalls", "./data/ChicagoSketch", "./data/Sydney"] {
        let (network, _, _, _, link_data) = io::load_dir(dir).unwrap();
        let link_costs = link_data
            .iter()
            .map(|data| data["vdf_fftt"].parse::<f64>().unwrap())
            .collect::<Vec<_>>();

        let origins = (0..network.zones().len())
            .step_by((network.zones().len() / 50).max(1))
            .collect::<Vec<_>>();

        let mut sssp = SSSP::new(&network);
        group.bench_function(
            format!("Network {dir} (# of origins: {})", origins.len()),
            |bencher| {
                bencher.iter_with_large_drop(|| {
                    origins
                        .iter()
                        .map(|&o| sssp.one_to_all(black_box(o), black_box(&link_costs)))
                        .collect::<Vec<_>>()
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    name = sssp;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(20));
    targets = one_to_all
);

criterion_main!(sssp);
