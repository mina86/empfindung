//! # Empfindung
//!
//! Empfindung is a pure-Rust implementation of the [CIEDE2000
//! algorithm](http://en.wikipedia.org/wiki/Color_difference#CIEDE2000) which
//! serves to quantify the difference between two colors.
//!
//! ## Example:
//!
//! ```
//! use empfindung::de2000;
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
//!     let delta_e = de2000::diff(color_1, color_2);
//!     println!("The color difference is: {}", delta_e);
//!     assert_eq!(20.553642, delta_e);
//! }
//! ```

pub mod de2000;

pub use de2000::DE2000;
