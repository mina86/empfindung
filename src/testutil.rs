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

pub fn do_test_zero(diff: impl Fn(lab::Lab, lab::Lab) -> f32) {
    for colour in generate_colours(1000) {
        assert_eq!(0.0, diff(colour, colour));
    }
}

pub fn do_test_symmetric(diff: impl Fn(lab::Lab, lab::Lab) -> f32) {
    for pair in generate_colours(1000).windows(2) {
        assert_eq!(diff(pair[0], pair[1]), diff(pair[1], pair[0]));
    }
}

fn from_tripple(lab: (f32, f32, f32)) -> lab::Lab {
    lab::Lab {
        l: lab.0,
        a: lab.1,
        b: lab.2,
    }
}

pub fn do_test_difference(
    tests: &[(f32, (f32, f32, f32), (f32, f32, f32))],
    diff: impl Fn(lab::Lab, lab::Lab) -> f32,
) {
    for (expected, colour_1, colour_2) in tests.iter() {
        let got = diff(from_tripple(*colour_1), from_tripple(*colour_2));
        assert_eq!(*expected, (got * 10000.0).round() / 10000.0);
    }
}
