# Changelog

## 0.2.4 (2021-11-28)

- Add support for `rgb` crate and deprecate `diff_rgb` functions in
  favour of running `diff` with `rgb::RGB8` arguments.
- Provide `ToLab` implementation for references so that `diff`
  functions can be called by value or by reference.

## 0.2.3 (2021-07-10)

- Rename `de2000` module to `cie00`.  The new name better indicates
  the origin of the metric, i.e. that it was defined by the CIE
  in 2000.
- Add `cie76`, `cie94` and `cmc` modules and thus implementations of
  CIE76, CIE94 and CMC l:c algorithms.
- Add `ToLab` trait so that the diff functions can be used with
  arguments other than `lab::Lab`.  Apart from `lab::Lab`,
  implementations for `(f32, f32, f32)` and `[f32; 3]` are provided.
- Add `lab` feature (enabled by default) which allows compiling the
  crate without depending on `lab` crate.  The feature is required to
  support functions taking `lab::Lab` arguments and by the sRGB
  handling functions.

## 0.2.2 (2021-07-01)

- Add `de2000::diff` and `de2000::diff_rgb` functions.  They deprecate
  `new` and `from_rgb` methods defined in `DE2000` structure.
- Add `de2000::diff_with_params` and `de2000::diff_rgb_with_params`
  functions as well as `de2000::KSubParams` structure.  They allow
  adjusting how much lightness, chroma and hue affect the difference
  calculation.
- Update `lab` dependency to accept anything above 0.4 (including the
  latest 0.10).

## delta_e 0.2.1 (2019-06-23)

- Update `lab` from 0.4 to 0.7

## delta_E 0.2.0 (2017-06-30)

- Add `from_rgb`
- Add an extensive test suite

## delta_e 0.1.0 (2017-05-26)

- Initial release
