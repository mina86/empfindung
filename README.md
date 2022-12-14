# Empfindung - Quantify colour differences in Rust

[![crates.io](https://img.shields.io/crates/v/empfindung)](https://crates.io/crates/empfindung)
[![Docs](https://docs.rs/empfindung/badge.svg)](https://docs.rs/empfindung)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/mina86/empfindung/blob/master/LICENSE)

Empfindung is a library providing implementations of colour difference
algorithms.  Specifically, distances based on L\*a\*b\* colour space
often referred to as ΔE*.  (This is also where the package gets its
name.  The ‘E’ stands for German ‘Empfindung’).

The crate provides CIEDE2000, CIE94, CIE76 and CMC l:c implementations.

## Installation

If you're using Cargo, just add DeltaE to your `Cargo.toml`:

```toml
[dependencies]
empfindung = "0.2"
```

## Example

```rust
use empfindung::cie00;

fn main() {
    let colour_1 = lab::Lab { l: 38.972, a: 58.991, b: 37.138 };
    let colour_2 = lab::Lab { l: 54.528, a: 42.416, b: 54.497 };

    let empfindung = cie00::diff(colour_1, colour_2);
    println!("The colour difference is: {}", empfindung);

    let colour_1 = ( 38.972, 58.991, 37.138 );
    let colour_2 = ( 54.528, 42.416, 54.497 );

    let delta_e = cie76::diff(colour_1, colour_2);
    println!("The Euclidean distance is: {}", delta_e);
    assert_eq!(28.601656, delta_e);

    let colour_1 = rgb::RGB::<u8>::new(234, 76, 76);
    let colour_2 = rgb::RGB::<u8>::new(76, 187, 234);
    let delta_e = cie00::diff(colour_1, colour_2);
    println!("The CIEDE200 colour difference is: {}", delta_e);
    assert_eq!(58.90164, delta_e);
}
```

## Crate Features

The crate defines `lab` and `rgb` features which are enabled by
default.  The former adds dependency on the `lab` crate and allows
functions to take `lab::Lab` arguments.  The latter adds dependency on
`rgb` crate and further allows functions to take `rgb::RGB<u8>`
arguments.

## About

This crate was originally written by [Elliot
Jackson](https://elliotekj.com) and later forked by [Michał
Nazarewicz](https://mina86.com) after long inactivity.  Aside from the
package name change, it is a drop-in replacement for the `delta_e`
create.

A quick migrating from to `empfindung` can be performed via `use`
declaration as follows:

```rust
use empfindung as delta_e;
```

or changing the paths to use the new crate name.  In particular, if
`use delta_e::DE2000;` declaration is used, it’s enough to replace it
by the following without having to touch the rest of the code:

```rust
use empfindung::DE2000;  // was use delta_e::DE2000;
```

Having said that, the `DE2000` structure is now deprecated and it’s
better to use `empfindung::cie00::diff` directly.

## License

Empfindung is released under the MIT license, See [`LICENSE`
file](https://github.com/mina86/empfindung/blob/master/LICENSE).
