//! **The control-net bracket seam follows the certified door.**
//!
//! `ring_coords` lifts a carrier's control net into the C9 ring, one
//! `RingInterval` per coefficient. The lift reads each coefficient's
//! bracket, and at the `Interval` scalar a bracket can be *sound but
//! inadmissible*: `sqrt([−1, 4])` clamps to `[0, 2]` and records the
//! domain violation only in its decoration (`Trv`). Nothing downstream
//! of the ring can see that — `RingInterval` has no decoration channel
//! — so a coefficient that may not certify has to be refused **here**,
//! at the crossing, or it certifies a composite bound for an expression
//! nobody asked for.
//!
//! The invariant these rows pin: a coefficient that fails
//! [`CertifiedEnclosure`] crosses as poison, a coefficient that passes
//! crosses with its stored endpoints, and the two verdicts are decided
//! per coefficient rather than per carrier.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve2, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, CertifiedEnclosure, Interval, Real};
use geom_core::{Point2, Point3};

/// A domain violation with finite endpoints: `sqrt([−1, 4])` clamps to
/// `[0, 2]` at decoration `Trv`. The finiteness is the whole point — a
/// NaN or empty bracket would surface through storage on its own and
/// the crossing would need no help.
fn trv() -> Interval {
    Interval::from_bounds(-1.0, 4.0).sqrt()
}

/// The discriminating partner: same shape, nothing wrong with it.
fn healthy() -> Interval {
    Interval::from_bounds(1.0, 4.0).sqrt()
}

fn kv() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// The fixture is the case the seam is about, not an already-visible
/// failure: the bracket door hands over finite endpoints and only the
/// certified door refuses.
#[test]
fn the_fixture_is_a_finite_bracket_that_cannot_certify() {
    let x = trv();
    assert_eq!((Bounds::lo(x), Bounds::hi(x)), (0.0, 2.0));
    assert!(x.certified_bracket().is_none());
    assert_eq!(healthy().certified_bracket(), Some((1.0, 2.0)));
}

/// Construction is **not** the door. `NurbsCurve3::new` validates counts
/// and weights; it does not read decorations, so a violated coefficient
/// is stored without complaint and the refusal has to happen at the
/// crossing. This row states where the responsibility lives.
#[test]
fn construction_admits_a_violated_coefficient() {
    let c = NurbsCurve3::new(
        kv(),
        vec![
            Point3::new(trv(), Interval::from_f64(0.0), Interval::from_f64(0.0)),
            Point3::new(
                Interval::from_f64(1.0),
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
            ),
        ],
        vec![1.0, 1.0],
    )
    .unwrap();
    assert!(c.control()[0].x.certified_bracket().is_none());
}

#[test]
fn curve3_ring_coords_refuses_a_violated_coefficient_per_channel() {
    let c = NurbsCurve3::new(
        kv(),
        vec![
            Point3::new(trv(), healthy(), Interval::from_f64(0.0)),
            Point3::new(
                Interval::from_f64(1.0),
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
            ),
        ],
        vec![1.0, 1.0],
    )
    .unwrap();
    let coords = c.ring_coords();
    assert!(
        coords[0][0].is_poison(),
        "the violated coefficient crossed as {:?} — the lift read the \
         BRACKET door, so a clamped `sqrt` certifies a composite bound \
         for an expression that was never evaluated",
        coords[0][0]
    );
    // Per coefficient, not per carrier: the healthy neighbours cross
    // with their stored endpoints.
    assert_eq!((coords[0][1].lo(), coords[0][1].hi()), (1.0, 1.0));
    assert_eq!((coords[1][0].lo(), coords[1][0].hi()), (1.0, 2.0));
    assert_eq!((coords[2][0].lo(), coords[2][0].hi()), (0.0, 0.0));
}

#[test]
fn curve2_ring_coords_refuses_a_violated_coefficient_per_channel() {
    let c = NurbsCurve2::new(
        kv(),
        vec![
            Point2::new(healthy(), trv()),
            Point2::new(Interval::from_f64(1.0), Interval::from_f64(0.0)),
        ],
        vec![1.0, 1.0],
    )
    .unwrap();
    let coords = c.ring_coords();
    assert!(
        coords[1][0].is_poison(),
        "the violated coefficient crossed as {:?}",
        coords[1][0]
    );
    assert_eq!((coords[0][0].lo(), coords[0][0].hi()), (1.0, 2.0));
    assert_eq!((coords[1][1].lo(), coords[1][1].hi()), (0.0, 0.0));
}

/// Non-vacuity from the other side: an all-healthy net crosses whole,
/// so the rows above are pinning a refusal and not a crossing that
/// poisons everything it touches.
#[test]
fn a_certified_net_crosses_unchanged() {
    let c = NurbsCurve3::new(
        kv(),
        vec![
            Point3::new(healthy(), healthy(), healthy()),
            Point3::new(
                Interval::from_f64(2.0),
                Interval::from_f64(3.0),
                Interval::from_f64(4.0),
            ),
        ],
        vec![1.0, 1.0],
    )
    .unwrap();
    for ch in c.ring_coords() {
        for r in ch {
            assert!(!r.is_poison(), "a certified net must cross whole");
        }
    }
}
