use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use std::hint::black_box;
use tap_garden::dynamic::routing::MixtureFlow;

fn make_flow(n: usize, offset: u32) -> MixtureFlow {
    let destinations: Vec<u32> = (0..n as u32).map(|i| i * 2 + offset).collect();
    let portions: Vec<f64> = vec![1.0 / n as f64; n];
    MixtureFlow::new(1.0, destinations, portions)
}

fn make_random_flow(rng: &mut StdRng, n: usize, universe: u32) -> MixtureFlow {
    let mut destinations: Vec<u32> = (0..universe).sample(rng, n);
    destinations.sort_unstable();

    let mut portions: Vec<f64> = (0..n).map(|_| rng.random_range(0.01..1.0)).collect();
    let sum: f64 = portions.iter().sum();
    for p in &mut portions {
        *p /= sum; // normalize so portions sum to 1.0
    }

    let total_flow = rng.random_range(1.0..100.0);
    MixtureFlow::new(total_flow, destinations, portions)
}

fn mixture_flow_plus(c: &mut Criterion) {
    let mut group = c.benchmark_group("MixtureFlow::plus");

    for size in [10, 100, 1_000, 10_000] {
        let a = make_flow(size, 0);
        let b = make_flow(size, 1); // offset so destinations interleave, not fully overlap

        group.bench_function(format!("merge_interleaved_n{size}"), |bencher| {
            bencher.iter(|| black_box(&a).plus(black_box(&b)))
        });
    }

    let mut rng = StdRng::seed_from_u64(42);
    for size in [10, 100, 1_000, 10_000] {
        let universe = (size * 3) as u32; // controls sparsity/overlap between a and b
        let a = make_random_flow(&mut rng, size, universe);
        let b = make_random_flow(&mut rng, size, universe);

        group.bench_function(format!("merge_random_n{size}"), |bencher| {
            bencher.iter(|| black_box(&a).plus(black_box(&b)))
        });
    }

    let nonzero = make_flow(1_000, 0);
    let zero = MixtureFlow::new(0.0, vec![], vec![]);

    group.bench_function("trivial_self_zero", |bencher| {
        bencher.iter(|| black_box(&zero).plus(black_box(&nonzero)))
    });

    group.bench_function("trivial_other_zero", |bencher| {
        bencher.iter(|| black_box(&nonzero).plus(black_box(&zero)))
    });

    group.finish();
}

criterion_group!(benches, mixture_flow_plus);
criterion_main!(benches);
