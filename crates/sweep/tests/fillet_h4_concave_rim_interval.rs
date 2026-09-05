//! The concave waist at the CERTIFIED scalar (feature `interval`) — the
//! interval twin of `fillet_h4_concave_rim`'s waist row.
//!
//! The material-adding band's one new decision is a SIGN — the side the
//! arms rest the ball on, `Convexity::signed`, a fold of the chain's
//! STORED convexity verdict. That fold is a boolean on stored bits and
//! no interval arithmetic touches it; what this lane shows is that the
//! arithmetic UNDER the folded sign brackets the closed form: the
//! concave rim carves at `Interval` through the same doors it carves
//! through at `f64`, the result is tier-3 valid, the volume enclosure
//! BRACKETS `V₀ + ΔV` with `ΔV` the Pappus fill, and the enclosure is
//! narrow enough to be a claim, so `V₁ > V₀` is definite and not a
//! shrug. Every profile coordinate is dyadic, so the fixture's
//! enclosures are points and the widths below are the lane's own.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Bounds, Interval, Real, Tol};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{rim_arcs_at, waist_fill, waisted_at};
use topo::{mass_properties, validate_geometric};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// An enclosure must contain its truth AND be a claim: under a
/// nanometre-cubed wide on bodies of volume ~2.
fn assert_brackets(got: Interval, truth: f64, what: &str) {
    assert!(
        got.lo() <= truth && truth <= got.hi(),
        "{what}: the enclosure must contain the truth: {got:?} vs {truth}"
    );
    assert!(
        got.hi() - got.lo() < 1e-9,
        "{what}: the enclosure must be a claim, not a shrug: width {}",
        got.hi() - got.lo()
    );
}

#[test]
fn the_waist_carves_at_the_certified_scalar_and_brackets_the_pappus_fill() {
    let tol = Tol::witness();
    let r = 0.05;
    let source = waisted_at::<Interval>(tol);
    let arcs = rim_arcs_at(&source, 0.5, 0.5);
    assert_eq!(arcs.len(), 2, "the waist rim is seam-split into two arcs");
    let p0 = mass_properties(&source, tol).expect("interval props");
    let v0 = 7.0 * PI / 12.0;
    assert_brackets(p0.volume, v0, "the source");

    let out = fillet_edges(&source, &arcs, iv(r), tol)
        .unwrap_or_else(|e| panic!("the concave waist carves at Interval, got {e:?}"));
    assert_eq!(out.band_faces.len(), 1, "one annulus band");
    validate_geometric(&out.body, tol).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    let p1 = mass_properties(&out.body, tol).expect("interval props");
    assert_eq!(
        p1.volume_pad, 0.0,
        "closed-form inventory at the certified scalar too"
    );
    assert_brackets(p1.volume, v0 + waist_fill(0.5, r), "the carved body");
    // The sign is DEFINITE at the certified scalar: the whole enclosure
    // of the carved volume lies above the whole enclosure of the source's.
    assert!(
        p1.volume.lo() > p0.volume.hi(),
        "a concave band adds material, definitely: {:?} above {:?}",
        p1.volume,
        p0.volume
    );
}
