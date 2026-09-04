//! The HOSTLESS-crossing closed rim at the CERTIFIED scalar (feature
//! `interval`) — the interval twin of `fillet_h5_r1_probes`'s carve and
//! closed-form rows.
//!
//! **What this lane is here to show, and what it is not.** The unit's
//! new decision is STRUCTURAL: which of the two moves lands a crossing's
//! host foot, read off where the rim sits in its host face's loop
//! structure. That is key equality and cycle walking, and no interval
//! arithmetic touches it — routing cannot go one way at `f64` and
//! another at `Interval`. What the arithmetic DOES carry is the foot's
//! own position, `scaled(host carrier).eval(t)` at the crossing's own
//! parameter, and everything the trim arcs and the band's torus are
//! then described against. So the claim here is that the strut foot's
//! arithmetic brackets: the same rims carve through the same doors, the
//! result is tier-3 valid (which re-derives every trimline's tangential
//! contact and the slit's seam description at rest), and the volume
//! enclosure BRACKETS the hand closed form narrowly enough to be a
//! claim rather than a shrug.
//!
//! Both material sides run, because the foot's arithmetic is the same
//! on both and a bracket that held only on the convex side would be
//! evidence about the fixture rather than the lane. Every profile
//! coordinate is dyadic, so the fixtures' own enclosures are points and
//! the widths below are this lane's.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Interval, Real, Tol};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{bowl_at, rim_arcs_at, waisted_at, wedge_fill};
use topo::{Body, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn repaired(mut body: Body<Interval>) -> Body<Interval> {
    body.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair, at the certified scalar");
    body
}

/// An enclosure must contain its truth AND be a claim: under a
/// nanometre-cubed wide on bodies of volume of order one.
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
fn the_hostless_rim_carves_at_the_certified_scalar_on_both_material_sides() {
    let r = 0.05;
    let s2 = core::f64::consts::SQRT_2;
    for (name, source, rim, k, da, db, adds) in [
        (
            "the bowl's concave floor",
            repaired(bowl_at::<Interval>(tol())),
            (1.0, 1.0),
            (1.0, 1.0),
            (-1.0, 0.0),
            (1.0 / s2, 1.0 / s2),
            true,
        ),
        (
            "the waisted body's convex base",
            repaired(waisted_at::<Interval>(tol())),
            (1.0, 0.0),
            (1.0, 0.0),
            (-1.0, 0.0),
            (-1.0 / s2, 1.0 / s2),
            false,
        ),
    ] {
        let arcs = rim_arcs_at(&source, rim.0, rim.1);
        assert_eq!(arcs.len(), 2, "{name}: the repaired rim is two arcs");
        let p0 = mass_properties(&source, tol()).expect("interval props");

        let out = fillet_edges(&source, &arcs, iv(r), tol())
            .unwrap_or_else(|e| panic!("{name} carves at Interval, got {e:?}"));
        assert_eq!(out.band_faces.len(), 1, "{name}: one annulus band");
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{name}: tier-3 valid at Interval, got {e:?}"));
        let p1 = mass_properties(&out.body, tol()).expect("interval props");
        assert_eq!(
            p1.volume_pad, 0.0,
            "{name}: every face of the carve is closed-form"
        );

        // The truth: the source volume's own midpoint moved by the hand
        // fill, signed by the material side. The source enclosure is a
        // point on these dyadic fixtures, which is what lets the sum be
        // a truth rather than a second enclosure.
        let fill = wedge_fill(k, da, db, r);
        let delta = if adds { fill } else { -fill };
        assert!(
            p0.volume.hi() - p0.volume.lo() < 1e-12,
            "{name}: the source's own enclosure is a point"
        );
        assert_brackets(p1.volume, p0.volume.lo() + delta, name);
        // And the SIGN is definite, not a straddle: the concave band
        // adds material and the convex one removes it, at this scalar
        // too.
        assert_eq!(
            (p1.volume.lo() - p0.volume.hi()) > 0.0,
            adds,
            "{name}: the material side is definite at Interval"
        );
    }
}
