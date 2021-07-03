use criterion::{criterion_group, criterion_main};

fn generate_colours(count: usize) -> Vec<lab::Lab> {
    use rand::Rng;
    use rand::SeedableRng;

    let mut labs = Vec::with_capacity(count);
    let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);
    for _ in 0..count {
        labs.push(lab::Lab {
            l: rng.gen_range(0.0..=100.0),
            a: rng.gen_range(-100.0..=100.0),
            b: rng.gen_range(-110.0..=100.0),
        });
    }
    labs
}

fn diff_benchmark(c: &mut criterion::Criterion) {
    let colours = generate_colours(1_000);
    c.bench_function("cie00", |b| {
        // Use iter_custom so that we can swap the loops for the loop over
        // colours to be outer one.  We do this to minimise how memory access
        // time influences the benchmark.  Doing calculation for data in the
        // same region of memory will make sure it is fetched from cache.
        b.iter_custom(|reps| {
            let start = std::time::Instant::now();
            for pair in colours.windows(2) {
                for _ in 0..reps {
                    let diff = empfindung::cie00::diff(pair[0], pair[1]);
                    criterion::black_box(diff);
                }
            }
            start.elapsed()
        });
    });
}

criterion_group!(benches, diff_benchmark,);
criterion_main!(benches);
