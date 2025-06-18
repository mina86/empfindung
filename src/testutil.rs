type Tripple = (f32, f32, f32);

// Floating point calculations in Miri seem broken.  For example, for normal
// floating point numbers x and y, x+y should equal y+x, but that does not
// always happen in Miri.
//
// The result is that we end up with noisier results and NaNs when running
// calculations on Miri.  NaNs happen when taking square root of negative values
// which normally are guaranteed to be non-negative.
//
// To address that, reduce the comparison epsilon and accept NaNs in some of the
// test functions.
pub const EPSILON: f32 = if cfg!(miri) { 0.01 } else { 0.001 };

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

pub fn do_test_zero(diff: impl Fn(Tripple, Tripple) -> f32) {
    const COUNT: usize = if cfg!(miri) { 10 } else { 10000 };
    for colour in generate_colours(COUNT) {
        let got = diff(colour, colour);
        if !cfg!(miri) {
            assert_eq!(0.0, got);
        } else if !got.is_nan() {
            approx::assert_abs_diff_eq!(0.0, got, epsilon = EPSILON);
        }
    }
}

pub fn do_test_symmetric(diff: impl Fn(Tripple, Tripple) -> f32) {
    const COUNT: usize = if cfg!(miri) { 10 } else { 1000 };
    for pair in generate_colours(COUNT).windows(2) {
        let lhs = diff(pair[0], pair[1]);
        let rhs = diff(pair[1], pair[0]);
        if cfg!(miri) {
            approx::assert_abs_diff_eq!(lhs, rhs, epsilon = EPSILON);
        } else {
            assert_eq!(lhs, rhs);
        }
    }
}

pub fn do_test_difference(
    tests: &[(f32, Tripple, Tripple)],
    diff: impl Fn(Tripple, Tripple) -> f32,
) {
    for (want, colour_1, colour_2) in tests {
        let got = diff(*colour_1, *colour_2);
        if !approx::abs_diff_eq!(*want, got, epsilon = EPSILON) &&
            (!cfg!(miri) || !got.is_nan())
        {
            panic!(
                "{} ≠ {}; colours: {:?}, {:?}",
                want, got, colour_1, colour_2
            );
        }
    }
}
