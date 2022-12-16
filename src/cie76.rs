// ΔE₇₆ computation implementation.
// Copyright (c) 2021 Michał Nazarewicz <mina86@mina86.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Implementation of the CIE76 colour distance algorithm.
//!
//! The CIE76 (ΔE₇₆) is a simple metric based on the L\*a\*b\* colour space.  In
//! fact, it’s nothing more than Euclidean distance between the two colours in
//! that colour space.

/// Returns the CIE76 colour difference between two L\*a\*b\* colours.
///
/// ### Example
///
/// ```
/// use empfindung::cie76;
///
#[cfg_attr(
    feature = "lab",
    doc = "
let colour_1 = lab::Lab { l: 38.972, a: 58.991, b: 37.138 };
let colour_2 = lab::Lab { l: 54.528, a: 42.416, b: 54.497 };
"
)]
#[cfg_attr(
    not(feature = "lab"),
    doc = "
let colour_1 = (38.972, 58.991, 37.138);
let colour_2 = (54.528, 42.416, 54.497);
"
)]
///
/// let delta_e = cie76::diff(colour_1, colour_2);
/// println!("The colour difference is: {}", delta_e);
/// approx::assert_abs_diff_eq!(28.601656, delta_e, epsilon = 0.001);
#[cfg_attr(
    all(feature = "lab", feature = "rgb"),
    doc = r#"

let colour_1 = rgb::RGB::<u8>::new(234, 76, 76);
let colour_2 = rgb::RGB::<u8>::new(76, 187, 234);

let delta_e = cie76::diff(colour_1, colour_2);
println!("The colour difference is: {}", delta_e);
approx::assert_abs_diff_eq!(104.05857, delta_e, epsilon = 0.001);
"#
)]
/// ```
pub fn diff(colour_1: impl crate::ToLab, colour_2: impl crate::ToLab) -> f32 {
    let colour_1 = colour_1.to_lab();
    let colour_2 = colour_2.to_lab();
    let dl = colour_1.0 - colour_2.0;
    let da = colour_1.1 - colour_2.1;
    let db = colour_1.2 - colour_2.2;
    (dl * dl + da * da + db * db).sqrt()
}

/// Returns the CIE76 colour difference between two sRGB colours.
///
/// ### Example
///
/// ```
/// use empfindung::cie76;
///
/// let colour_1 = [234, 76, 76];
/// let colour_2 = [76, 187, 234];
///
/// let delta_e = cie76::diff_rgb(&colour_1, &colour_2);
/// println!("The colour difference is: {}", delta_e);
/// approx::assert_abs_diff_eq!(104.05857, delta_e, epsilon = 0.001);
/// ```
#[cfg(feature = "lab")]
#[deprecated(note = "Use cie76::diff() with rgb::RGB8 argument")]
pub fn diff_rgb(colour_1: &[u8; 3], colour_2: &[u8; 3]) -> f32 {
    diff(lab::Lab::from_rgb(colour_1), lab::Lab::from_rgb(colour_2))
}


#[cfg(test)]
mod tests {
    #[rustfmt::skip]
    static TESTS: [(f32, (f32, f32, f32), (f32, f32, f32)); 6] = [
        ( 5.0, (0.0, 0.0, 0.0), ( 3.0,  4.0,   0.0)),
        ( 5.0, (0.0, 0.0, 0.0), ( 3.0, -4.0,   0.0)),
        ( 5.0, (0.0, 0.0, 0.0), (-3.0,  4.0,   0.0)),
        ( 5.0, (0.0, 4.0, 0.0), ( 3.0,  0.0,   0.0)),
        ( 5.0, (0.0, 2.0, 0.0), ( 3.0, -2.0,   0.0)),
        (97.0, (0.0, 0.0, 0.0), ( 0.0, 65.0, -72.0)),
    ];

    #[test]
    fn test_difference() {
        crate::testutil::do_test_difference(&TESTS, super::diff);
    }

    #[test]
    fn test_zero() { crate::testutil::do_test_zero(|a, b| super::diff(a, b)) }

    #[test]
    fn test_symmetric() {
        crate::testutil::do_test_symmetric(|a, b| super::diff(a, b))
    }
}
