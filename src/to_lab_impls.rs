use super::ToLab;

impl ToLab for (f32, f32, f32) {
    #[inline]
    fn to_lab(&self) -> (f32, f32, f32) { *self }
}

impl ToLab for [f32; 3] {
    #[inline]
    fn to_lab(&self) -> (f32, f32, f32) { (self[0], self[1], self[2]) }
}

#[cfg(feature = "lab")]
impl ToLab for lab::Lab {
    #[inline]
    fn to_lab(&self) -> (f32, f32, f32) { (self.l, self.a, self.b) }
}

#[cfg(all(feature = "lab", feature = "rgb"))]
impl ToLab for rgb::RGB<u8> {
    /// Assumes an sRGB colour and converts it into L\*a\*\b\*.
    #[inline]
    fn to_lab(&self) -> (f32, f32, f32) {
        lab::Lab::from_rgb(&[self.r, self.g, self.b]).to_lab()
    }
}

#[cfg(all(feature = "lab", feature = "rgb"))]
impl ToLab for rgb::alt::BGR<u8> {
    /// Assumes an sRGB colour and converts it into L\*a\*\b\*.
    #[inline]
    fn to_lab(&self) -> (f32, f32, f32) {
        lab::Lab::from_rgb(&[self.r, self.g, self.b]).to_lab()
    }
}

#[cfg(feature = "rgb")]
impl ToLab for rgb::alt::Gray<u8> {
    /// Assumes a grey colour in sRGB colour and converts it into L\*a\*\b\*.
    ///
    /// This is faster than converting from a `(r, g, b)` colour so if it’s
    /// known that the colour is grey (that is, `r == g == b`), using `Gray`
    /// type is preferable for performance.
    #[inline]
    fn to_lab(&self) -> (f32, f32, f32) { lab_from_grey(**self) }
}


/// Calculates L\*a\*b\* for a grey colour with given sRGB component.
///
/// Returned a\* and b\* components are always zero.  This is the same as
/// calculating L\*a\*b\* for `(grey, grey, grey)` sRGB colour but faster and
/// more precise.
#[cfg(feature = "rgb")]
fn lab_from_grey(grey: u8) -> (f32, f32, f32) {
    let l = if grey <= 10 {
        /* Linear part of gamma and c < ε part of lab mapping. */
        /*     y = grey / (12.92 * 255)
         *     y’ = (κ * y + 16) / 116
         *     l  = 116 * y’ - 16
         *     l  = 116 * (κ * y + 16) / 116 - 16
         *        = κ * y
         *        = κ * grey / (12.92 * 255)
         *        = grey * (κ / (12.92 * 255)) */
        /* κ / (12.92 * 255) = (29/3)^3 / (12.92 * 255)
         *                   = 24389 / 27 / 12.92 / 255
         *                   = 24389 / 88954.2
         *                   = 243890 / 889542 */
        const KAPPA_OVER_D: f32 = 243890.0 / 889542.0;
        grey as f32 * KAPPA_OVER_D
    } else {
        const A: f32 = 0.055 * 255.0;
        const D: f32 = 1.055 * 255.0;
        let ys = (grey as f32 + A) / D;
        if grey <= 23 {
            /* Exponential part of gamma and c < ε part of lab mapping. */
            /*     y = ((grey / 255 + 0.055) / 1.055)^2.4
             *     y’ = (κ * y + 16) / 116
             *     l  = 116 * y’ - 16
             *     l  = 116 * (κ * y + 16) / 116 - 16
             *        = κ * y */
            /*     κ  = (29/3)^3 = 24389 / 27 */
            const KAPPA: f32 = 24389.0 / 27.0;
            KAPPA * ys.powf(2.4)
        } else {
            /* Exponential part of gamma and c > ε part of lab mapping. */
            /*     y = ((grey / 255 + 0.055) / 1.055)^2.4
             *     y’ = y^(1/3)
             *        = ((grey / 255 + 0.055) / 1.055)^(2.4 / 3)
             *     l  = 116 * y’ - 16 */
            116.0 * ys.powf(24.0 / 30.0) - 16.0
        }
    };

    (l, 0.0, 0.0)
}


/// Tests that `lab_from_grey` gives results close to what `lab` crate gives.
#[cfg(feature = "rgb")]
#[test]
fn test_lab_from_grey() {
    let errors = (0..=255)
        .filter_map(|grey| {
            let want = lab::Lab::from_rgb(&[grey, grey, grey]).l;
            let got = lab_from_grey(grey).0;
            if approx::abs_diff_eq!(want, got, epsilon = 0.00001) {
                None
            } else {
                Some((grey, want, got))
            }
        })
        .collect::<Vec<_>>();
    assert!(errors.is_empty(), "{:?}", errors);
}
