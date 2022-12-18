// ΔE₉₄ computation implementation.
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

//! Implementation of the CIE94 colour distance algorithm.
//!
//! The CIE94 (ΔE₉₄) is a quasimetric which can be parameterised with three
//! parameters which indicate what effect difference in lightness, chroma and
//! hue have on the computed distance.  The module provides [`diff`] function
//! which requires those parameters to be specified as a [`Params`] argument to
//! customise the coefficients.
//!
//! Note that the distance is not symmetrical, i.e. in general case `diff(a, b,
//! ksub) != diff(b, a, ksub)`.  Prefer [`crate::cie00`] module if you needa
//! proper metric or [`crate::cie76`] module if additional performance cost is
//! not acceptable.

/// `k` parameters adjusting what effect lightness, hue and chroma difference
/// will have on the calculated distance.
///
/// To construct the object, either create it directly by providing your own
/// choice of parameters, or use [`Params::graphic`] or [`Params::textiles`]
/// methods which use parameters defined for graphic arts and textiles
/// respectively.  The default values, i.e. what [`Params::default`] returns,
/// are ones used for graphic arts since the assumption is that the crate is
/// used mostly for computer graphics.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Params {
    /// The k_L parameter.
    pub l: f32,
    /// The K_1 parameter.  (Called `c` because `1` is not a valid identifier
    /// and because the parameter affects chroma difference).
    pub c: f32,
    /// The K_2 parameter.  (Called `h` because `2` is not a valid identifier
    /// and because the parameter affects hue difference).
    pub h: f32,
}

#[deprecated(note = "Use Params name instead")]
pub type KSubParams = Params;

impl Default for Params {
    /// Returns parameters weighted for graphic arts.
    fn default() -> Self { Self::graphic() }
}

impl Params {
    /// Returns parameters weighted for graphic arts.
    pub fn graphic() -> Self {
        Self {
            l: 1.0,
            c: 0.045,
            h: 0.015,
        }
    }

    /// Returns parameters weighted for textiles.
    pub fn textiles() -> Self {
        Self {
            l: 2.0,
            c: 0.048,
            h: 0.014,
        }
    }
}


/// Returns the CIE94 colour difference between two L\*a\*b\* colours using
/// specified `k` parameters.
///
/// Use [`Params::graphic()`] or [`Params::textiles()`] to construct parameters
/// depending on the application.
///
/// ### Example
///
/// ```
/// use empfindung::cie94;
///
#[cfg_attr(
    feature = "lab",
    doc = "
let reference = lab::Lab { l: 38.972, a: 58.991, b: 37.138 };
let colour = lab::Lab { l: 54.528, a: 42.416, b: 54.497 };
"
)]
#[cfg_attr(
    not(feature = "lab"),
    doc = "
let reference = (38.972, 58.991, 37.138);
let colour = (54.528, 42.416, 54.497);
"
)]
///
/// let delta_e = cie94::diff(reference, colour, cie94::Params::graphic());
/// println!("The colour difference is: {}", delta_e);
/// approx::assert_abs_diff_eq!(19.482761, delta_e, epsilon = 0.001);
#[cfg_attr(
    all(feature = "lab", feature = "rgb"),
    doc = r#"

let reference = rgb::RGB::<u8>::new(234, 76, 76);
let colour = rgb::RGB::<u8>::new(76, 187, 234);

let delta_e = cie94::diff(
    &reference, &colour, cie94::Params::graphic());
println!("The colour difference is: {}", delta_e);
approx::assert_abs_diff_eq!(50.87644, delta_e, epsilon = 0.001);
"#
)]
/// ```
pub fn diff(
    reference: impl crate::ToLab,
    colour: impl crate::ToLab,
    ksub: Params,
) -> f32 {
    diff_impl(reference.to_lab(), colour.to_lab(), ksub)
}

fn diff_impl(
    reference: (f32, f32, f32),
    colour: (f32, f32, f32),
    ksub: Params,
) -> f32 {
    let delta_l = reference.0 - colour.0;
    let delta_a = reference.1 - colour.1;
    let delta_b = reference.2 - colour.2;
    let c_1 = super::math::hypot(reference.1, reference.2);
    let c_2 = super::math::hypot(colour.1, colour.2);
    let delta_c = c_1 - c_2;
    let delta_h = (delta_a.powi(2) + delta_b.powi(2) - delta_c.powi(2)).sqrt();

    let l = delta_l / ksub.l;
    let c = delta_c / (1.0 + ksub.c * c_1);
    let h = delta_h / (1.0 + ksub.h * c_1);

    (l * l + c * c + h * h).sqrt()
}

/// Returns the CIE94 colour difference between two sRGB colours using custom
/// `k` parameters.
///
/// Use [`Params::graphic()`] or [`Params::textiles()`] to construct parameters
/// depending on the application.
///
/// ### Example
///
/// ```
/// use empfindung::cie94;
///
/// let reference = [234, 76, 76];
/// let colour = [76, 187, 234];
///
/// let delta_e = cie94::diff_rgb(
///     &reference, &colour, cie94::Params::graphic());
/// println!("The colour difference is: {}", delta_e);
/// approx::assert_abs_diff_eq!(50.87644, delta_e, epsilon = 0.001);
/// ```
#[cfg(feature = "lab")]
#[deprecated(note = "Use cie94::diff() with rgb::RGB8 argument")]
pub fn diff_rgb(reference: &[u8; 3], colour: &[u8; 3], ksub: Params) -> f32 {
    diff(
        lab::Lab::from_rgb(reference),
        lab::Lab::from_rgb(colour),
        ksub,
    )
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_zero_graphic() {
        let ksub = super::Params::graphic();
        crate::testutil::do_test_zero(|a, b| super::diff(a, b, ksub))
    }

    #[test]
    fn test_zero_textiles() {
        let ksub = super::Params::textiles();
        crate::testutil::do_test_zero(|a, b| super::diff(a, b, ksub))
    }

    #[rustfmt::skip]
    static TESTS: [(f32, (f32, f32, f32), (f32, f32, f32)); 34] = [
        (100.0,   (100.0,     0.0050,  -0.0100), ( 0.0000,   0.0000,   0.0000)),
        ( 1.3950, (50.0000,   2.6772, -79.7751), (50.0000,   0.0000, -82.7485)),
        ( 1.9341, (50.0000,   3.1571, -77.2803), (50.0000,   0.0000, -82.7485)),
        ( 2.4543, (50.0000,   2.8361, -74.0200), (50.0000,   0.0000, -82.7485)),
        ( 0.6845, (50.0000,  -1.3802, -84.2814), (50.0000,   0.0000, -82.7485)),
        ( 0.6696, (50.0000,  -1.1848, -84.8006), (50.0000,   0.0000, -82.7485)),
        ( 0.6919, (50.0000,  -0.9009, -85.5211), (50.0000,   0.0000, -82.7485)),
        ( 2.2361, (50.0000,   0.0000,   0.0000), (50.0000,  -1.0000,   2.0000)),
        ( 2.0316, (50.0000,  -1.0000,   2.0000), (50.0000,   0.0000,   0.0000)),
        ( 4.8007, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0009)),
        ( 4.8007, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0010)),
        ( 4.8007, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0011)),
        ( 4.8007, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0012)),
        ( 4.8007, (50.0000,  -0.0010,   2.4900), (50.0000,   0.0009,  -2.4900)),
        ( 4.8007, (50.0000,  -0.0010,   2.4900), (50.0000,   0.0011,  -2.4900)),
        ( 3.4077, (50.0000,   2.5000,   0.0000), (50.0000,   0.0000,  -2.5000)),
        (34.6892, (50.0000,   2.5000,   0.0000), (73.0000,  25.0000, -18.0000)),
        (29.4414, (50.0000,   2.5000,   0.0000), (61.0000,  -5.0000,  29.0000)),
        (27.9141, (50.0000,   2.5000,   0.0000), (56.0000, -27.0000,  -3.0000)),
        (24.9377, (50.0000,   2.5000,   0.0000), (58.0000,  24.0000,  15.0000)),
        ( 0.8221, (50.0000,   2.5000,   0.0000), (50.0000,   3.1736,   0.5854)),
        ( 0.7166, (50.0000,   2.5000,   0.0000), (50.0000,   3.2972,   0.0000)),
        ( 0.8049, (50.0000,   2.5000,   0.0000), (50.0000,   1.8634,   0.5757)),
        ( 0.7528, (50.0000,   2.5000,   0.0000), (50.0000,   3.2592,   0.3350)),
        ( 1.3910, (60.2574, -34.0099,  36.2677), (60.4626, -34.1751,  39.4387)),
        ( 1.2481, (63.0109, -31.0961,  -5.8663), (62.8187, -29.7946,  -4.0864)),
        ( 1.2980, (61.2901,   3.7196,  -5.3901), (61.4292,   2.2480,  -4.9620)),
        ( 1.8205, (35.0831, -44.1164,   3.7933), (35.0232, -40.0716,   1.5901)),
        ( 2.5561, (22.7233,  20.0904, -46.6940), (23.0331,  14.9730, -42.5619)),
        ( 1.4249, (36.4612,  47.8580,  18.3852), (36.2715,  50.5065,  21.2231)),
        ( 1.4195, (90.8027,  -2.0831,   1.4410), (91.1528,  -1.6435,   0.0447)),
        ( 2.3226, (90.9257,  -0.5406,  -0.9208), (88.6381,  -0.8985,  -0.7239)),
        ( 0.9385, ( 6.7747,  -0.2908,  -2.4247), ( 5.8714,  -0.0985,  -2.2286)),
        ( 1.3065, ( 2.0776,   0.0795,  -1.1350), ( 0.9033,  -0.0636,  -0.5514)),
    ];

    #[test]
    fn test_difference() {
        let diff = |a, b| super::diff(a, b, super::Params::default());
        crate::testutil::do_test_difference(&TESTS, diff);
    }
}
