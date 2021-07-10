type Tripple = (f32, f32, f32);

fn generate_colours(count: usize) -> Vec<Tripple> {
    use rand::Rng;
    use rand::SeedableRng;

    let mut labs = Vec::with_capacity(count);
    let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);
    for _ in 0..count {
        labs.push((
            rng.gen_range(0.0..=100.0),
            rng.gen_range(-100.0..=100.0),
            rng.gen_range(-110.0..=100.0),
        ));
    }
    labs
}

pub fn do_test_zero(diff: impl Fn(Tripple, Tripple) -> f32) {
    for colour in generate_colours(10000) {
        assert_eq!(0.0, diff(colour, colour));
    }
}

pub fn do_test_symmetric(diff: impl Fn(Tripple, Tripple) -> f32) {
    for pair in generate_colours(1000).windows(2) {
        assert_eq!(diff(pair[0], pair[1]), diff(pair[1], pair[0]));
    }
}

pub fn do_test_difference(
    tests: &[(f32, Tripple, Tripple)],
    diff: impl Fn(Tripple, Tripple) -> f32,
) {
    for (expected, colour_1, colour_2) in tests.iter() {
        let got = diff(*colour_1, *colour_2);
        assert_eq!(*expected, (got * 10000.0).round() / 10000.0);
    }
}
