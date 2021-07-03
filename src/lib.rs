//! # Empfindung
//!
//! Empfindung is a pure-Rust implementation of the [CIEDE2000
//! algorithm](http://en.wikipedia.org/wiki/Color_difference#CIEDE2000) which
//! serves to quantify the difference between two colors.
//!
//! ## Example:
//!
//! ```
//! use empfindung::cie00;
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
//!     println!("The color difference is: {}", delta_e);
//!     assert_eq!(20.553642, delta_e);
//! }
//! ```

pub mod cie00;

#[doc(hidden)]
pub use cie00 as de2000;
#[allow(deprecated)]
pub use cie00::DE2000;
