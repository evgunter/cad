//! **A thin lemon's whole-sheet volume and area keep their digits**
//! (`props::whole_surface`, the spindle sheet's closed form). The lemon
//! `R → −r` is a sliver about its axis: its volume is of order `v*⁵`
//! in the sheet's half angle `v*` (`cos v* = −R/r`), so a form whose
//! terms are of order `v*` cancels nearly every digit, and one that
//! reads `v*` off `acos(−R/r)` rounds its input besides. The truths are
//! `2πr³·f(v*)` and `4πr²·g(v*)`, `g(v) = sin v − v cos v` and
//! `f(v) = g(v) − sin³v/3`, each summed as its series in 60-digit
//! decimal at `R`'s own `f64` value (`r = 1`), with `v*` read as
//! `2·asin(√((1 + R)/2))` and every printed digit kept.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// The truths are written with every digit the 60-digit sum printed, past
// what an `f64` holds; the literal rounds to the nearest `f64` as read.
#![allow(clippy::excessive_precision)]

use geom::Surface;
use geom_core::{Point3, Vec3};

/// The sheet `R`, `r = 1` about `z` through the origin.
fn lemon(major: f64) -> Surface<f64> {
    Surface::Spindle {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        major_radius: major,
        minor_radius: 1.0,
    }
}

/// `(R, volume, area)`, the truths at 60 digits.
const TRUTHS: [(f64, f64, f64); 4] = [
    (
        -0.9,
        1.482_457_104_905_159_26e-2,
        3.765_608_928_668_846_00e-1,
    ),
    (
        -0.999,
        1.498_466_564_275_898_83e-7,
        3.746_755_215_061_206_63e-4,
    ),
    (
        -0.9999,
        4.739_024_357_876_662_75e-10,
        1.184_774_707_447_826_40e-5,
    ),
    (
        -0.99999,
        1.498_625_536_931_917_45e-12,
        3.746_569_729_816_416_98e-7,
    ),
];

/// Each lemon's volume within `1e-9` of the truth, relative, and its
/// area within `1e-13`. The bound is the form's own: its three terms are
/// each of order `v*³`, so it keeps `ε/v*²` of the volume (`2e-11` at
/// `R = −0.99999`, where `v* ≈ 0.0045`); the Pappus terms as written,
/// of order `v*`, kept `ε/v*⁴` and missed by `4e-3` there.
#[test]
fn a_thin_lemons_volume_and_area_match_their_series() {
    for (major, volume, area) in TRUTHS {
        let whole = geom_brep::props::whole_surface(&lemon(major), true).unwrap();
        let v = whole.flux / 3.0;
        let rel_v = ((v - volume) / volume).abs();
        let rel_a = ((whole.area - area) / area).abs();
        assert!(
            rel_v <= 1e-9,
            "R = {major}: volume {v:e} vs {volume:e}, {rel_v:e} off"
        );
        assert!(
            rel_a <= 1e-13,
            "R = {major}: area {:e} vs {area:e}, {rel_a:e} off",
            whole.area
        );
    }
}
