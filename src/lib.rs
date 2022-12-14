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
#![cfg_attr(
    feature = "lab",
    doc = "
let colour_1 = lab::Lab { l: 38.972, a: 58.991, b: 37.138 };
let colour_2 = lab::Lab { l: 54.528, a: 42.416, b: 54.497 };
"
)]
#![cfg_attr(
    not(feature = "lab"),
    doc = "
let colour_1 = (38.972, 58.991, 37.138);
let colour_2 = (54.528, 42.416, 54.497);
"
)]
//!
//! let delta_e = cie00::diff(colour_1, colour_2);
//! println!("The CIEDE2000 colour difference is: {}", delta_e);
//! assert_eq!(20.553642, delta_e);
//!
//! let colour_1 = (38.972, 58.991, 37.138);
//! let colour_2 = (54.528, 42.416, 54.497);
//!
//! let delta_e = cie76::diff(colour_1, colour_2);
//! println!("The Euclidean distance is: {}", delta_e);
//! assert_eq!(28.601656, delta_e);
#![cfg_attr(
    all(feature = "lab", feature = "rgb"),
    doc = r#"

let colour_1 = rgb::RGB::<u8>::new(234, 76, 76);
let colour_2 = rgb::RGB::<u8>::new(76, 187, 234);
let delta_e = cie00::diff(colour_1, colour_2);
println!("The CIEDE200 colour difference is: {}", delta_e);
assert_eq!(58.90164, delta_e);
"#
)]
//! ```
//!
//! ## Crate Features
//!
//! The crate defines `lab` and `rgb` features which are enabled by default.
//!
//! With both of them enabled, create provides [`ToLab`] implementation for
//! `rgb::RGB<u8>` type which means that `diff` functions can be used with
//! `rgb::RGB<u8>` arguments.
//!
//! Furthermore, if `lab` enabled the `diff` functions can accept `lab::Lab`
//! argument and `diff_rgb` functions as well as `DE2000` is provided.  Note
//! that the latter two are a deprecated features.

#[cfg(all(feature = "lab", feature = "rgb"))]
use std::convert::TryInto;

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

impl ToLab for &(f32, f32, f32) {
    fn to_lab(&self) -> (f32, f32, f32) { **self }
}

impl ToLab for [f32; 3] {
    fn to_lab(&self) -> (f32, f32, f32) { (self[0], self[1], self[2]) }
}

impl ToLab for &[f32; 3] {
    fn to_lab(&self) -> (f32, f32, f32) { (self[0], self[1], self[2]) }
}

#[cfg(feature = "lab")]
impl ToLab for lab::Lab {
    fn to_lab(&self) -> (f32, f32, f32) { (self.l, self.a, self.b) }
}

#[cfg(feature = "lab")]
impl ToLab for &lab::Lab {
    fn to_lab(&self) -> (f32, f32, f32) { (self.l, self.a, self.b) }
}

#[cfg(all(feature = "lab", feature = "rgb"))]
impl ToLab for rgb::RGB<u8> {
    fn to_lab(&self) -> (f32, f32, f32) {
        lab::Lab::from_rgb(self.as_ref().try_into().unwrap()).to_lab()
    }
}

#[cfg(all(feature = "lab", feature = "rgb"))]
impl ToLab for &rgb::RGB<u8> {
    fn to_lab(&self) -> (f32, f32, f32) {
        lab::Lab::from_rgb(self.as_ref().try_into().unwrap()).to_lab()
    }
}

#[cfg(all(feature = "lab", feature = "rgb"))]
impl ToLab for rgb::RGBA<u8> {
    /// Assumes an sRGB colour and converts it into L\*a\*\b\*.
    fn to_lab(&self) -> (f32, f32, f32) {
        lab::Lab::from_rgb(self.as_ref()[0..3].try_into().unwrap()).to_lab()
    }
}


pub(crate) mod math {
    pub fn hypot(x: f32, y: f32) -> f32 { (x * x + y * y).sqrt() }
    pub fn hypot_squared(x: f32, y: f32) -> f32 { x * x + y * y }
}

#[cfg(test)]
pub(crate) mod testutil;
