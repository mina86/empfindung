// Colour difference computation implementations.
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

//! # Empfindung
//!
//! Empfindung is a library providing implementations of colour difference
//! algorithms.  Specifically, distances based on L\*a\*b\* colour space often
//! referred to as ΔE*.  (This is also where the package gets its name.  The ‘E’
//! stands for German ‘Empfindung’).
//!
//! The crate provides CIEDE2000 (in [`cie00`] module), CIE94 (in [`cie94`]),
//! CIE76 (in [`cie76`] module) and CMC l:c (in [`cmc`] module) implementations.
//!
//! ## Example
//!
//! ```
//! use empfindung::cie00;
//! use empfindung::cie76;
//!
//! fn main() {
//!     let color_1 = lab::Lab {
//!         l: 38.972,
//!         a: 58.991,
//!         b: 37.138,
//!     };
//!
//!     let color_2 = lab::Lab {
//!         l: 54.528,
//!         a: 42.416,
//!         b: 54.497,
//!     };
//!
//!     let delta_e = cie00::diff(color_1, color_2);
//!     println!("The CIEDE2000 colour difference is: {}", delta_e);
//!     assert_eq!(20.553642, delta_e);
//!
//!     let color_1 = (
//!         38.972,
//!         58.991,
//!         37.138,
//!     );
//!
//!     let color_2 = (
//!         54.528,
//!         42.416,
//!         54.497,
//!     );
//!
//!     let delta_e = cie76::diff(color_1, color_2);
//!     println!("The Euclidean distance is: {}", delta_e);
//!     assert_eq!(28.601656, delta_e);
//! }
//! ```
//!
//! ## Crate Features
//!
//! The crate defines `lab` feature which is enabled by default.  That feature
//! adds dependency on the `lab` crate allowing the functions to take `lab::Lab`
//! arguments.  Furthermore, without it `diff_rgb` functions as well as `DE2000`
//! structure won’t be provided (the latter is deprecated anyway though).
//!
//! Chances are that other part of a project depend on `lab` crate if it works
//! on L\*a\*b\* colours space in which case disabling the `lab` feature won’t
//! bring any benefit but it may still be beneficial in cases where `lab` crate
//! is not used anywhere else (e.g. because a different colour conversion
//! libraries are used) or this crate somehow falls behind in its version
//! specification for the `lab` crate (though at the moment that can only happen
//! if `lab` release version 1.0).

pub mod cie00;
pub mod cie76;
pub mod cie94;
pub mod cmc;

#[doc(hidden)]
pub use cie00 as de2000;
#[allow(deprecated)]
pub use cie00::DE2000;


/// Object which can be converted to L\*a\*\b* colour representation.
pub trait ToLab {
    /// Returns L\*, a\* and b\* coordinates of a colour.
    fn to_lab(&self) -> (f32, f32, f32);
}

impl ToLab for (f32, f32, f32) {
    fn to_lab(&self) -> (f32, f32, f32) { *self }
}

impl ToLab for [f32; 3] {
    fn to_lab(&self) -> (f32, f32, f32) { (self[0], self[1], self[2]) }
}

#[cfg(feature = "lab")]
impl ToLab for lab::Lab {
    fn to_lab(&self) -> (f32, f32, f32) { (self.l, self.a, self.b) }
}


pub(crate) mod math {
    pub fn hypot(x: f32, y: f32) -> f32 { (x * x + y * y).sqrt() }
}

#[cfg(test)]
pub(crate) mod testutil;
