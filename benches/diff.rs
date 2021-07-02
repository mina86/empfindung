use criterion::{criterion_group, criterion_main};
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::rand_core::SeedableRng;

fn generate_colours(count: usize) -> Vec<lab::Lab> {
    let mut rgb = Vec::<[u8; 3]>::with_capacity(count);
    let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);
    rng.fill_bytes(unsafe {
        std::slice::from_raw_parts_mut(rgb.as_mut_ptr() as *mut u8, count * 3)
    });
    unsafe {
        rgb.set_len(rgb.capacity());
    }
    lab::rgbs_to_labs(&rgb)
}

fn diff_benchmark(c: &mut criterion::Criterion) {
    let colours = generate_colours(1_000);
    c.bench_function("de2000", |b| {
        // Use iter_custom so that we can swap the loops for the loop over
        // colours to be outer one.  We do this to minimise how memory access
        // time influences the benchmark.  Doing calculation for data in the
        // same region of memory will make sure it is fetched from cache.
        b.iter_custom(|reps| {
            let start = std::time::Instant::now();
            for pair in colours.windows(2) {
                for _ in 0..reps {
                    let diff = empfindung::de2000::diff(pair[0], pair[1]);
                    criterion::black_box(diff);
                }
            }
            start.elapsed()
        });
    });
}

criterion_group!(benches, diff_benchmark,);
criterion_main!(benches);
