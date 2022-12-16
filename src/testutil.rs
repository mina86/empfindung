type Tripple = (f32, f32, f32);

fn generate_colours(count: usize) -> Vec<Tripple> {
    use rand::Rng;
    use rand::SeedableRng;

    let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);
    (0..count)
        .map(|_| {
            (
                rng.gen_range(0.0..=100.0),
                rng.gen_range(-100.0..=100.0),
                rng.gen_range(-110.0..=100.0),
            )
        })
        .collect()
}

pub(super) fn do_test_zero(diff: impl Fn(Tripple, Tripple) -> f32) {
    for colour in generate_colours(10000) {
        assert_eq!(0.0, diff(colour, colour));
    }
}

pub(super) fn do_test_symmetric(diff: impl Fn(Tripple, Tripple) -> f32) {
    for pair in generate_colours(1000).windows(2) {
        assert_eq!(diff(pair[0], pair[1]), diff(pair[1], pair[0]));
    }
}

pub(super) fn do_test_difference(
    tests: &[(f32, Tripple, Tripple)],
    diff: impl Fn(Tripple, Tripple) -> f32,
) {
    for (want, colour_1, colour_2) in tests {
        let got = diff(*colour_1, *colour_2);
        approx::assert_abs_diff_eq!(*want, got, epsilon = 0.001);
    }
}
