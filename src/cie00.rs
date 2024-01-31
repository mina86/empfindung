// ΔE₀₀ computation implementation.
// Copyright (c) 2017 Elliot Jackson
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

//! Implementation of the CIEDE2000 colour distance algorithm.
//!
//! The CIEDE2000 (ΔE₀₀) is a metric which can be parameterised with three
//! parameters which indicate what effect difference in lightness, chroma and
//! hue have on the computed distance.  The module provides [`diff`] function
//! which uses default parameters as well as [`diff_with_params`] which accepts
//! [`Params`] argument to customise the coefficients.

/// Returns the CIEDE2000 colour difference between two L\*a\*b\* colours.
///
/// ## Example
///
/// ```
/// use empfindung::cie00;
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
/// let delta_e = cie00::diff(colour_1, colour_2);
/// approx::assert_abs_diff_eq!(20.553642, delta_e, epsilon = 0.001);
#[cfg_attr(
    all(feature = "lab", feature = "rgb"),
    doc = r#"

let color_1 = rgb::RGB { r: 234u8, g: 76, b: 76 };
let color_2 = rgb::alt::BGR { r: 76u8, g: 187, b: 234 };
let delta_e = cie00::diff(color_1, color_2);
approx::assert_abs_diff_eq!(58.90164, delta_e, epsilon = 0.001);

let grey = rgb::alt::Gray(128u8);
let delta_e = cie00::diff(color_1, color_2);
approx::assert_abs_diff_eq!(58.90164, delta_e, epsilon = 0.001);
"#
)]
/// ```
pub fn diff(color_1: impl crate::ToLab, color_2: impl crate::ToLab) -> f32 {
    diff_with_params(color_1, color_2, Params::default())
}

/// Returns the CIEDE2000 colour difference between two sRGB colours.
///
/// ## Example
///
/// ```
/// use empfindung::cie00;
///
/// let colour_1 = [234, 76, 76];
/// let colour_2 = [76, 187, 234];
///
/// let delta_e = cie00::diff_rgb(&colour_1, &colour_2);
/// approx::assert_abs_diff_eq!(58.90164, delta_e, epsilon = 0.001);
/// ```
#[cfg(feature = "lab")]
#[deprecated(note = "Use cie00::diff() with rgb::RGB8 argument")]
pub fn diff_rgb(color_1: &[u8; 3], color_2: &[u8; 3]) -> f32 {
    diff(lab::Lab::from_rgb(color_1), lab::Lab::from_rgb(color_2))
}

/// `k` parameters adjusting what effect lightness, hue and chroma difference
/// will have on the calculated distance.
///
/// By default the values equal one.  The larger the value, the smaller impact
/// each component will have.  Note that setting any of those values to zero
/// will make the difference infinite (which is unlikely to be a desired
/// result).
///
/// To construct the object, either create it directly by providing your own
/// choice of parameters, or use [`Params::default`] or [`Params::yang2012`]
/// methods.  The former returns object with all parameters equal one while the
/// latter returns parameters as devised by Yang et al.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Params {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

#[deprecated(note = "Use Params name instead")]
pub type KSubParams = Params;

/// Returns the CIEDE2000 colour difference between two L\*a\*b\* colours using
/// custom `k` parameters.
///
/// ## Example
///
/// ```
/// use empfindung::cie00;
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
/// let delta_e = cie00::diff_with_params(
///     colour_1, colour_2, cie00::Params::yang2012());
/// approx::assert_abs_diff_eq!(23.524858, delta_e, epsilon = 0.001);
/// ```
pub fn diff_with_params(
    color_1: impl crate::ToLab,
    color_2: impl crate::ToLab,
    ksub: Params,
) -> f32 {
    diff_impl(color_1.to_lab(), color_2.to_lab(), ksub)
}

fn diff_impl(
    color_1: (f32, f32, f32),
    color_2: (f32, f32, f32),
    ksub: Params,
) -> f32 {
    let l_bar = (color_1.0 + color_2.0) * 0.5;
    let delta_l = color_2.0 - color_1.0;

    let c1 = super::math::hypot(color_1.1, color_1.2);
    let c2 = super::math::hypot(color_2.1, color_2.2);

    const TWENTY_FIVE_TO_SEVENTH: f32 = 6103515625f32;
    let tmp = ((c1 + c2) * 0.5).powi(7);
    let tmp = 1.5 - (tmp / (tmp + TWENTY_FIVE_TO_SEVENTH)).sqrt() * 0.5;
    let a_prime_1 = color_1.1 * tmp;
    let a_prime_2 = color_2.1 * tmp;

    let c_prime_1 = super::math::hypot(a_prime_1, color_1.2);
    let c_prime_2 = super::math::hypot(a_prime_2, color_2.2);
    let c_prime_bar = (c_prime_1 + c_prime_2) * 0.5;
    let delta_c_prime = c_prime_2 - c_prime_1;

    let tmp = (l_bar - 50.0).powi(2);
    let s_sub_l = 1.0 + (0.015 * tmp) / (20.0 + tmp).sqrt();

    let s_sub_c = 1.0 + 0.045 * c_prime_bar;

    let h_prime_1 = get_h_prime(color_1.2, a_prime_1);
    let h_prime_2 = get_h_prime(color_2.2, a_prime_2);
    let delta_h_prime = get_delta_h_prime(c1, c2, h_prime_1, h_prime_2);

    let delta_upcase_h_prime =
        2.0 * (c_prime_1 * c_prime_2).sqrt() * (0.5 * delta_h_prime).sin();

    let upcase_h_prime_bar = if (h_prime_1 - h_prime_2).abs() > PI_32 {
        (h_prime_1 + h_prime_2) * 0.5 + PI_32
    } else {
        (h_prime_1 + h_prime_2) * 0.5
    };

    let upcase_t = get_upcase_t(upcase_h_prime_bar);

    let s_sub_upcase_h = 1.0 + 0.015 * c_prime_bar * upcase_t;

    let lightness = delta_l / (ksub.l * s_sub_l);
    let chroma = delta_c_prime / (ksub.c * s_sub_c);
    let hue = delta_upcase_h_prime / (ksub.h * s_sub_upcase_h);
    let r_sub_t = get_r_sub_t(c_prime_bar, upcase_h_prime_bar);

    (lightness.powi(2) + chroma.powi(2) + hue.powi(2) + r_sub_t * chroma * hue)
        .sqrt()
}

/// Returns the CIEDE2000 colour difference between two sRGB colours using
/// custom `k` parameters.
///
/// ## Example
///
/// ```
/// use empfindung::cie00;
///
/// let colour_1 = [234, 76, 76];
/// let colour_2 = [76, 187, 234];
///
/// let delta_e = cie00::diff_rgb_with_params(
///     &colour_1, &colour_2, cie00::Params::yang2012());
/// approx::assert_abs_diff_eq!(26.88325, delta_e, epsilon = 0.001);
/// ```
#[cfg(feature = "lab")]
#[deprecated(note = "Use cie00::diff_with_params() with rgb::RGB8 argument")]
pub fn diff_rgb_with_params(
    color_1: &[u8; 3],
    color_2: &[u8; 3],
    ksub: Params,
) -> f32 {
    diff_with_params(
        lab::Lab::from_rgb(color_1),
        lab::Lab::from_rgb(color_2),
        ksub,
    )
}


#[deprecated(note = "Use cie00::diff() or cie00::diff_rgb() instead")]
pub struct DE2000;

#[allow(deprecated)]
#[cfg(feature = "lab")]
impl DE2000 {
    /// Returns the colour difference between two `Lab` colours.
    ///
    /// ## Example
    ///
    /// ```
    /// use empfindung::DE2000;
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
    /// let delta_e = DE2000::new(colour_1, colour_2);
    /// approx::assert_abs_diff_eq!(20.553642, delta_e, epsilon = 0.001);
    /// ```
    #[deprecated(note = "Use cie00::diff() instead")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(color_1: lab::Lab, color_2: lab::Lab) -> f32 {
        diff_with_params(color_1, color_2, Params::default())
    }

    /// Returns the colour difference between two RGB colours.
    ///
    /// ## Example
    ///
    /// ```
    /// use empfindung::DE2000;
    ///
    /// let colour_1 = [234, 76, 76];
    /// let colour_2 = [76, 187, 234];
    ///
    /// let delta_e = DE2000::from_rgb(&colour_1, &colour_2);
    /// approx::assert_abs_diff_eq!(58.90164, delta_e, epsilon = 0.001);
    /// ```
    #[deprecated(note = "Use cie00::diff_rgb() instead")]
    pub fn from_rgb(color_1: &[u8; 3], color_2: &[u8; 3]) -> f32 {
        diff_rgb(color_1, color_2)
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            l: 1.0,
            c: 1.0,
            h: 1.0,
        }
    }
}

impl Params {
    /// Returns parameters as determined in (Yang, 2012).
    ///
    /// See Yang Yang, Jun Ming, Nenghai Yu, ‘Color Image Quality Assessment
    /// Based on CIEDE2000’, Advances in Multimedia, vol. 2012, Article ID
    /// 273723, 6 pages, 2012. <https://doi.org/10.1155/2012/273723>.
    ///
    /// Note that inclusion of this function does not imply endorsement of those
    /// values.  Colorimetry is hard and it’s up to the user to determine
    /// correct values to use.  This function is here just for reference.  If in
    /// doubt use `Params::default()` which is what [`diff`] function uses.
    pub fn yang2012() -> Self {
        Self {
            l: 0.65,
            c: 1.0,
            h: 4.0,
        }
    }
}

fn get_h_prime(x: f32, y: f32) -> f32 {
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    let rad = x.atan2(y);
    if rad < 0.0 {
        rad + TAU_32
    } else {
        rad
    }
}

fn get_delta_h_prime(c1: f32, c2: f32, h_prime_1: f32, h_prime_2: f32) -> f32 {
    if 0.0 == c1 || 0.0 == c2 {
        return 0.0;
    }
    let diff = h_prime_2 - h_prime_1;
    if diff.abs() <= PI_32 {
        diff
    } else if h_prime_2 <= h_prime_1 {
        diff + TAU_32
    } else {
        diff - TAU_32
    }
}

#[rustfmt::skip]
fn get_upcase_t(upcase_h_prime_bar: f32) -> f32 {
    const THIRTY_DEG_IN_RAD: f32 = (TAU_64 / 12.0) as f32;
    const SIX_DEG_IN_RAD: f32 = (TAU_64 / 60.0) as f32;
    const SIXTY_THREE_DEG_IN_RAD: f32 = (TAU_64 * 0.175) as f32;

    1.0 - 0.17 * (      upcase_h_prime_bar - THIRTY_DEG_IN_RAD     ).cos()
        + 0.24 * (2.0 * upcase_h_prime_bar                         ).cos()
        + 0.32 * (3.0 * upcase_h_prime_bar + SIX_DEG_IN_RAD        ).cos()
        - 0.20 * (4.0 * upcase_h_prime_bar - SIXTY_THREE_DEG_IN_RAD).cos()
}

fn get_r_sub_t(c_prime_bar: f32, upcase_h_prime_bar: f32) -> f32 {
    const TWENTY_FIVE_TO_SEVENTH: f32 = 6103515625f32;
    let c7 = c_prime_bar.powi(7);
    let h = upcase_h_prime_bar * (14.4 / TAU_64) as f32 - 11.0;
    -2.0 * (c7 / (c7 + TWENTY_FIVE_TO_SEVENTH)).sqrt() *
        ((-h.powi(2)).exp() * (TAU_64 / 6.0) as f32).sin()
}

const TAU_32: f32 = std::f32::consts::TAU;
const PI_32: f32 = std::f32::consts::PI;
const TAU_64: f64 = std::f64::consts::TAU;


#[cfg(test)]
mod tests {
    #[test]
    fn test_zero() { crate::testutil::do_test_zero(|a, b| super::diff(a, b)); }

    #[test]
    fn test_zero_with_params() {
        let ksub = super::Params::yang2012();
        crate::testutil::do_test_zero(|a, b| {
            super::diff_with_params(a, b, ksub)
        });
    }

    #[test]
    fn test_symmetric() {
        crate::testutil::do_test_symmetric(|a, b| super::diff(a, b));
    }

    #[test]
    fn test_symmetric_with_params() {
        let ksub = super::Params::yang2012();
        crate::testutil::do_test_symmetric(|a, b| {
            super::diff_with_params(a, b, ksub)
        });
    }

    // Tests taken from Table 1: "CIEDE2000 total color difference test data" of
    // "The CIEDE2000 Color-Difference Formula: Implementation Notes,
    // Supplementary Test Data, and Mathematical Observations" by Gaurav Sharma,
    // Wencheng Wu and Edul N. Dalal.
    //
    // http://www.ece.rochester.edu/~gsharma/papers/CIEDE2000CRNAFeb05.pdf
    #[rustfmt::skip]
    static TESTS: [(f32, (f32, f32, f32), (f32, f32, f32)); 34] = [
        (100.0,   (100.0,     0.0050,  -0.0100), ( 0.0000,   0.0000,   0.0000)),
        ( 2.0425, (50.0000,   2.6772, -79.7751), (50.0000,   0.0000, -82.7485)),
        ( 2.8615, (50.0000,   3.1571, -77.2803), (50.0000,   0.0000, -82.7485)),
        ( 3.4412, (50.0000,   2.8361, -74.0200), (50.0000,   0.0000, -82.7485)),
        ( 1.0000, (50.0000,  -1.3802, -84.2814), (50.0000,   0.0000, -82.7485)),
        ( 1.0000, (50.0000,  -1.1848, -84.8006), (50.0000,   0.0000, -82.7485)),
        ( 1.0000, (50.0000,  -0.9009, -85.5211), (50.0000,   0.0000, -82.7485)),
        ( 2.3669, (50.0000,   0.0000,   0.0000), (50.0000,  -1.0000,   2.0000)),
        ( 2.3669, (50.0000,  -1.0000,   2.0000), (50.0000,   0.0000,   0.0000)),
        ( 7.1792, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0009)),
        ( 7.1792, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0010)),
        ( 7.2195, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0011)),
        ( 7.2195, (50.0000,   2.4900,  -0.0010), (50.0000,  -2.4900,   0.0012)),
        ( 4.8045, (50.0000,  -0.0010,   2.4900), (50.0000,   0.0009,  -2.4900)),
        ( 4.7461, (50.0000,  -0.0010,   2.4900), (50.0000,   0.0011,  -2.4900)),
        ( 4.3065, (50.0000,   2.5000,   0.0000), (50.0000,   0.0000,  -2.5000)),
        (27.1492, (50.0000,   2.5000,   0.0000), (73.0000,  25.0000, -18.0000)),
        (22.8977, (50.0000,   2.5000,   0.0000), (61.0000,  -5.0000,  29.0000)),
        (31.9030, (50.0000,   2.5000,   0.0000), (56.0000, -27.0000,  -3.0000)),
        (19.4535, (50.0000,   2.5000,   0.0000), (58.0000,  24.0000,  15.0000)),
        ( 1.0000, (50.0000,   2.5000,   0.0000), (50.0000,   3.1736,   0.5854)),
        ( 1.0000, (50.0000,   2.5000,   0.0000), (50.0000,   3.2972,   0.0000)),
        ( 1.0000, (50.0000,   2.5000,   0.0000), (50.0000,   1.8634,   0.5757)),
        ( 1.0000, (50.0000,   2.5000,   0.0000), (50.0000,   3.2592,   0.3350)),
        ( 1.2644, (60.2574, -34.0099,  36.2677), (60.4626, -34.1751,  39.4387)),
        ( 1.2630, (63.0109, -31.0961,  -5.8663), (62.8187, -29.7946,  -4.0864)),
        ( 1.8731, (61.2901,   3.7196,  -5.3901), (61.4292,   2.2480,  -4.9620)),
        ( 1.8645, (35.0831, -44.1164,   3.7933), (35.0232, -40.0716,   1.5901)),
        ( 2.0373, (22.7233,  20.0904, -46.6940), (23.0331,  14.9730, -42.5619)),
        ( 1.4146, (36.4612,  47.8580,  18.3852), (36.2715,  50.5065,  21.2231)),
        ( 1.4441, (90.8027,  -2.0831,   1.4410), (91.1528,  -1.6435,   0.0447)),
        ( 1.5381, (90.9257,  -0.5406,  -0.9208), (88.6381,  -0.8985,  -0.7239)),
        ( 0.6377, ( 6.7747,  -0.2908,  -2.4247), ( 5.8714,  -0.0985,  -2.2286)),
        ( 0.9082, ( 2.0776,   0.0795,  -1.1350), ( 0.9033,  -0.0636,  -0.5514)),
    ];

    #[test]
    fn test_difference() {
        crate::testutil::do_test_difference(&TESTS, super::diff);
    }
}
