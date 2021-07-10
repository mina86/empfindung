// Colour difference computation implementations library example.
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

fn parse(arg: std::ffi::OsString) -> Result<u32, String> {
    let arg = arg.into_string().map_err(|arg| {
        format!("{}: not valid Unicode", arg.to_string_lossy())
    })?;
    if arg.len() != 7 ||
        arg.as_bytes()[0] != '#' as u8 ||
        arg.as_bytes()[1] == '+' as u8
    {
        Err(format!("{}: expected argument in #RRGGBB format", arg))
    } else {
        u32::from_str_radix(&arg[1..], 16)
            .map_err(|err| format!("{}: {}", arg, err))
    }
}

fn parse_args() -> Result<(u32, u32), (std::ffi::OsString, String)> {
    let mut argv0 = "example".into();
    let mut colours = [0, 0];
    let mut n = 0;
    for arg in std::env::args_os() {
        if n == 0 {
            argv0 = arg;
        } else if n < 3 {
            colours[n - 1] = match parse(arg) {
                Ok(colour) => colour,
                Err(msg) => return Err((argv0, msg)),
            };
        } else {
            break;
        }
        n += 1;
    }
    if n != 3 {
        Err((argv0, String::from("Expected two arguments")))
    } else {
        Ok((colours[0], colours[1]))
    }
}

// Return (f32, f32, f32) rather than lab::Lab so that the example works even if
// the library is built without lab feature.  Normally, this would just return
// lab::Lab and the library would be built with lab feature enabled (which is
// the default).
fn from_rgb(rgb: u32) -> (f32, f32, f32) {
    let lab =
        lab::Lab::from_rgb(&[(rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8]);
    (lab.l, lab.a, lab.b)
}

pub fn main() {
    use empfindung::*;

    let (a, b) = match parse_args() {
        Ok(colours) => colours,
        Err((argv0, msg)) => {
            eprintln!(
                "{}\nusage: {} #RRGGBB #RRGGBB",
                msg,
                argv0.to_string_lossy()
            );
            std::process::exit(1);
        }
    };

    let a = from_rgb(a);
    let b = from_rgb(b);
    println!("ΔE_76  = {:>11.7}  (Euclidean distance)", cie76::diff(a, b));
    println!(
        "ΔE_94g = {:>11.7}  (parameters for graphic arts)",
        cie94::diff(a, b, cie94::KSubParams::graphic())
    );
    println!(
        "ΔE_94t = {:>11.7}  (parameters for textiles)",
        cie94::diff(a, b, cie94::KSubParams::textiles())
    );
    println!("ΔE_00  = {:>11.7}  (default parameters", cie00::diff(a, b));
    println!(
        "ΔE_00y = {:>11.7}  (parameters by Yang et al)",
        cie00::diff_with_params(a, b, cie00::KSubParams::yang2012())
    );
    println!("ΔE_1:1 = {:>11.7}  (CMC 1:1)", cmc::diff(a, b, cmc::LC11));
    println!("ΔE_1:1 = {:>11.7}  (CMC 2:1)", cmc::diff(a, b, cmc::LC21));
}
