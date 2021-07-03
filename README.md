# Empfindung - Quantify color differences in Rust

[![crates.io](https://img.shields.io/crates/v/empfindung)](https://crates.io/crates/empfindung)
[![Docs](https://docs.rs/empfindung/badge.svg)](https://docs.rs/empfindung)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/mina86/empfindung/blob/master/LICENSE)

Empfindung is a pure-Rust implementation of the [CIEDE2000
algorithm](http://en.wikipedia.org/wiki/Color_difference#CIEDE2000) which serves
to quantify the difference between two colors. It is entirely based on the work
of [Zachary Schuessler](http://zaclee.net/), who has written a [Javascript
implementation](https://github.com/zschuessler/DeltaE/blob/master/src/dE00.js)
of the algorithm.

## Installation

If you're using Cargo, just add DeltaE to your `Cargo.toml`:

```toml
[dependencies]
empfindung = "0.2"
```

## Example

```rust
extern crate empfindung;
extern crate lab;

use empfindung::cie00;

fn main() {
    let color_1 = lab::Lab {
        l: 38.972,
        a: 58.991,
        b: 37.138,
    };

    let color_2 = lab::Lab {
        l: 54.528,
        a: 42.416,
        b: 54.497,
    };

    let empfindung = cie00::diff(color_1, color_2);
    println!("The color difference is: {}", empfindung);
}
```

## About

This crate was originally written by [Elliot
Jackson](https://elliotekj.com) and later forked by [Michał
Nazarewicz](https://mina86.com) after long inactivity.  Aside from the
package name change, it is a drop-in replacement for the `delta_e`
create.

Migrating from to `empfindung` can be performed by using `use`
declaration with an alias:

```rust
use empfindung as delta_e;
```

or changing the paths using `delta_e` crate name to use `empfindung`
instead.  In particular, if `use delta_e::DE2000;` declaration was
used, it’s enough to change it to the following without having to
touch the rest of the code:

```rust
use empfindung::DE2000;  // was use delta_e::DE2000;
```

Having said that, the `DE2000` structure is now deprecated and it’s
better to use `empfindung::cie00::diff` directly.

## License

Empfindung is released under the MIT license, See [`LICENSE`
file](https://github.com/mina86/empfindung/blob/master/LICENSE).
