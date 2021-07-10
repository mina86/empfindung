// CMC l:c computation implementation.
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

//! Implementation of the CMC l:c colour distance algorithm.
//!
//! The CMC l:c is a quasimetric which is parameterised by two weights: `l` and
//! `c`.  Commonly used pair of weights are 1:1 and 2:1.
//!
//! Note that the distance is not symmetrical, i.e. in general case `diff(a, b,
//! ksub) != diff(b, a, ksub)`.  Prefer [`crate::cie00`] module if you needa
//! proper metric.

/// Returns the CMC l:c colour difference between two L\*a\*b\* colours using
/// specified `l` and `c` parameters.
///
/// ### Example
///
/// ```
/// extern crate empfindung;
/// extern crate lab;
///
/// use empfindung::cmc;
///
/// fn main() {
///     let colour_1 = lab::Lab {
///         l: 38.972,
///         a: 58.991,
///         b: 37.138,
///     };
///
///     let colour_2 = lab::Lab {
///         l: 54.528,
///         a: 42.416,
///         b: 54.497,
///     };
///
///     let delta_e = cmc::diff(colour_1, colour_2, (1.0, 1.0));
///     println!("The colour difference is: {}", delta_e);
///     assert_eq!(22.751015, delta_e);
///
///     let delta_e = cmc::diff(colour_1, colour_2, (2.0, 1.0));
///     println!("The colour difference is: {}", delta_e);
///     assert_eq!(17.743946, delta_e);
/// }
/// ```
pub fn diff(
    reference: impl crate::ToLab,
    colour: impl crate::ToLab,
    lc: (f32, f32),
) -> f32 {
    diff_impl(reference.to_lab(), colour.to_lab(), lc)
}

fn diff_impl(
    reference: (f32, f32, f32),
    colour: (f32, f32, f32),
    lc: (f32, f32),
) -> f32 {
    let delta_l = reference.0 - colour.0;
    let delta_a = reference.1 - colour.1;
    let delta_b = reference.2 - colour.2;
    let c_1 = super::math::hypot(reference.1, reference.2);
    let c_2 = super::math::hypot(colour.1, colour.2);
    let delta_c = c_1 - c_2;
    let delta_h = (delta_a.powi(2) + delta_b.powi(2) - delta_c.powi(2)).sqrt();

    let s_l = if reference.0 < 16.0 {
        const S: f64 = 1639.0f64 / 3206.0f64;
        S as f32
    } else {
        (0.040975 * reference.0) / (1.0 + 0.01765 * reference.0)
    };
    let s_c = ((0.0638 * c_1) / (1.0 + (0.0131 * c_1))) + 0.638;

    let tmp = c_1.powi(4);
    let f = (tmp / (tmp + 1900.0)).sqrt();
    let t = get_t(reference.1, reference.2);
    let s_h = s_c * (f * t + 1.0 - f);

    let l = delta_l / (lc.0 * s_l);
    let c = delta_c / (lc.1 * s_c);
    let h = delta_h / s_h;
    (l * l + c * c + h * h).sqrt()
}

/// Returns the CMC l:c colour difference between two sRGB colours using
/// specified `l` and `c` parameters.
///
/// ### Example
///
/// ```
/// extern crate empfindung;
///
/// use empfindung::cmc;
///
/// fn main() {
///     let colour_1 = [234, 76, 76];
///     let colour_2 = [76, 187, 234];
///
///     let delta_e = cmc::diff_rgb(&colour_1, &colour_2, (1.0, 1.0));
///     println!("The colour difference is: {}", delta_e);
///     assert_eq!(64.49067, delta_e);
///
///     let delta_e = cmc::diff_rgb(&colour_1, &colour_2, (2.0, 1.0));
///     println!("The colour difference is: {}", delta_e);
///     assert_eq!(63.303917, delta_e);
/// }
/// ```
pub fn diff_rgb(reference: &[u8; 3], colour: &[u8; 3], lc: (f32, f32)) -> f32 {
    diff(
        lab::Lab::from_rgb(reference),
        lab::Lab::from_rgb(colour),
        lc,
    )
}

/// ΔE CMC 1:1 parameters.
pub const LC11: (f32, f32) = (1.0, 1.0);
/// ΔE CMC 2:1 parameters.
pub const LC21: (f32, f32) = (2.0, 1.0);


fn get_t(a: f32, b: f32) -> f32 {
    use std::f64::consts::{PI, TAU};

    // (164 - 360) / 360 = -196 / 360 = -49 / 90
    const START: f32 = (-PI * 49.0 / 45.0) as f32;
    // (345 - 360) / 360 = -15 / 360 = -1 / 24
    const END: f32 = (-TAU / 24.0) as f32;

    let h = b.atan2(a);
    let ft = |m: f32, d: f32| (m * (h + d).cos()).abs();
    if START <= h && h <= END {
        // 168 / 360 = 7 / 15
        0.56 + ft(0.2, (TAU * 7.0 / 15.0) as f32)
    } else {
        // 35 / 128 = 7 / 36
        0.36 + ft(0.4, (PI * 7.0 / 36.0) as f32)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_zero() {
        crate::testutil::do_test_zero(|a, b| super::diff(a, b, (1.0, 1.0)));
        crate::testutil::do_test_zero(|a, b| super::diff(a, b, (2.0, 1.0)));
        crate::testutil::do_test_zero(|a, b| super::diff(a, b, (1.0, 2.0)));
    }

    #[rustfmt::skip]
    static TESTS: [(f32, (f32, f32, f32), (f32, f32, f32)); 34] = [
        (67.4802, (100.0,     0.0050,  -0.0100), ( 0.0000,   0.0000,   0.0000)),
        ( 1.7387, (50.0000,   2.6772, -79.7751), (50.0000,   0.0000, -82.7485)),
        ( 2.4966, (50.0000,   3.1571, -77.2803), (50.0000,   0.0000, -82.7485)),
        ( 3.3049, (50.0000,   2.8361, -74.0200), (50.0000,   0.0000, -82.7485)),
        ( 0.8574, (50.0000,  -1.3802, -84.2814), (50.0000,   0.0000, -82.7485)),
        ( 0.8833, (50.0000,  -1.1848, -84.8006), (50.0000,   0.0000, -82.7485)),
        ( 0.9782, (50.0000,  -0.9009, -85.5211), (50.0000,   0.0000, -82.7485)),
        ( 3.5048, (50.0000,   0.0000,   0.0000), (50.0000,  -1.0000,   2.0000)),
        ( 2.8793, (50.0000,  -1.0000,   2.0000), (50.0000,   0.0000,   0.0000)),
        ( 6.5784, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0009)),
        ( 6.5784, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0010)),
        ( 6.5784, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0011)),
        ( 6.5784, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0012)),
        ( 6.6749, (50.0000,  -0.0010,   2.4900), (50.0000,   0.0009,  -2.4900)),
        ( 6.6749, (50.0000,  -0.0010,   2.4900), (50.0000,   0.0011,  -2.4900)),
        ( 4.6685, (50.0000,   2.5000,   0.0000), (50.0000,   0.0000,  -2.5000)),
        (42.1088, (50.0000,   2.5000,   0.0000), (73.0000,  25.0000, -18.0000)),
        (39.4589, (50.0000,   2.5000,   0.0000), (61.0000,  -5.0000,  29.0000)),
        (38.3601, (50.0000,   2.5000,   0.0000), (56.0000, -27.0000,  -3.0000)),
        (33.9366, (50.0000,   2.5000,   0.0000), (58.0000,  24.0000,  15.0000)),
        ( 1.1440, (50.0000,   2.5000,   0.0000), (50.0000,   3.1736,   0.5854)),
        ( 1.0060, (50.0000,   2.5000,   0.0000), (50.0000,   3.2972,   0.0000)),
        ( 1.1130, (50.0000,   2.5000,   0.0000), (50.0000,   1.8634,   0.5757)),
        ( 1.0534, (50.0000,   2.5000,   0.0000), (50.0000,   3.2592,   0.3350)),
        ( 1.4282, (60.2574, -34.0099,  36.2677), (60.4626, -34.1751,  39.4387)),
        ( 1.2548, (63.0109, -31.0961,  -5.8663), (62.8187, -29.7946,  -4.0864)),
        ( 1.7684, (61.2901,   3.7196,  -5.3901), (61.4292,   2.2480,  -4.9620)),
        ( 2.0626, (35.0831, -44.1164,   3.7933), (35.0232, -40.0716,   1.5901)),
        ( 3.0870, (22.7233,  20.0904, -46.6940), (23.0331,  14.9730, -42.5619)),
        ( 1.7489, (36.4612,  47.8580,  18.3852), (36.2715,  50.5065,  21.2231)),
        ( 1.9010, (90.8027,  -2.0831,   1.4410), (91.1528,  -1.6435,   0.0447)),
        ( 1.7026, (90.9257,  -0.5406,  -0.9208), (88.6381,  -0.8985,  -0.7239)),
        ( 1.8024, ( 6.7747,  -0.2908,  -2.4247), ( 5.8714,  -0.0985,  -2.2286)),
        ( 2.4484, ( 2.0776,   0.0795,  -1.1350), ( 0.9033,  -0.0636,  -0.5514)),
    ];

    #[test]
    fn test_difference() {
        let diff = |a, b| super::diff(a, b, (1.0, 1.0));
        crate::testutil::do_test_difference(&TESTS, diff);
    }
}
