//! **The control-net bracket seam follows the certified door** — the
//! surface half of the curve suite of the same name.
//!
//! `NurbsSurface::ring_coords` lifts the control net into the C9 ring,
//! one `RingInterval` per coefficient, by reading each coefficient's
//! bracket. At the `Interval` scalar a bracket can be sound and still
//! inadmissible: `sqrt([−1, 4])` clamps to `[0, 2]` and records the
//! domain violation only in its decoration. `RingInterval` has no
//! decoration channel, so a coefficient that cannot certify must be
//! refused at the crossing or the composite residual bound built from
//! it describes an expression nobody evaluated.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, CertifiedEnclosure, Interval, Point3, Real};

fn trv() -> Interval {
    Interval::from_bounds(-1.0, 4.0).sqrt()
}

fn healthy() -> Interval {
    Interval::from_bounds(1.0, 4.0).sqrt()
}

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn kv() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// A bilinear patch whose `(0, 0)` corner carries `c` in `x`.
fn patch(c: Interval) -> NurbsSurface<Interval> {
    NurbsSurface::new(
        kv(),
        kv(),
        vec![
            Point3::new(c, iv(0.0), iv(0.0)),
            Point3::new(iv(0.0), iv(1.0), iv(0.0)),
            Point3::new(iv(1.0), iv(0.0), iv(0.0)),
            Point3::new(iv(1.0), iv(1.0), iv(0.0)),
        ],
        vec![1.0; 4],
    )
    .unwrap()
}

#[test]
fn the_fixture_is_a_finite_bracket_that_cannot_certify() {
    let x = trv();
    assert_eq!((Bounds::lo(x), Bounds::hi(x)), (0.0, 2.0));
    assert!(x.certified_bracket().is_none());
    assert_eq!(healthy().certified_bracket(), Some((1.0, 2.0)));
}

#[test]
fn surface_ring_coords_refuses_a_violated_coefficient_per_channel() {
    let coords = patch(trv()).ring_coords();
    assert!(
        coords[0][0].is_poison(),
        "the violated coefficient crossed as {:?} — the lift read the \
         BRACKET door, so a clamped `sqrt` reaches the tensor composite \
         bound as a healthy control coordinate",
        coords[0][0]
    );
    // Per coefficient, not per net: the healthy neighbours cross with
    // their stored endpoints.
    assert_eq!((coords[0][2].lo(), coords[0][2].hi()), (1.0, 1.0));
    assert_eq!((coords[1][1].lo(), coords[1][1].hi()), (1.0, 1.0));
}

/// Non-vacuity from the other side.
#[test]
fn a_certified_net_crosses_unchanged() {
    for ch in patch(healthy()).ring_coords() {
        for r in ch {
            assert!(!r.is_poison(), "a certified net must cross whole");
        }
    }
    let coords = patch(healthy()).ring_coords();
    assert_eq!((coords[0][0].lo(), coords[0][0].hi()), (1.0, 2.0));
}
