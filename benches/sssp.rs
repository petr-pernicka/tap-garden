use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;
use tap_garden::io;
use tap_garden::static_::sssp::SSSP;

fn one_to_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("SSSP::one_to_all");

    for dir in ["./data/SiouxFalls", "./data/ChicagoSketch", "./data/Sydney"] {
        let (node_data, link_data, demand_data) =
            io::gmns::load_gmns_dir(dir).unwrap();
        let (network, _odm, id2index) =
            io::parse::from_gmns_data(&node_data, &link_data, &demand_data);

        let mut link_costs = vec![0.0; link_data.len()];
        for ld in link_data {
            let i = network
                .links()
                .iter()
                .position(|link| {
                    link.tail() == id2index[&ld.from_node_id]
                        && link.head() == id2index[&ld.to_node_id]
                })
                .unwrap();

            link_costs[i] = ld.vdf_fftt;
        }

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
