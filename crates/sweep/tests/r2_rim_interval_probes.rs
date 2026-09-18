//! **Reviewer probes for the rim door at the certified scalar** (PR
//! 1821 review lane r2, feature `interval`) — the row that can go RED
//! when [`topo::query::rim_of`]'s bit test stops reading the UPPER
//! bracket end.
//!
//! `same_bits` is `lo` bits AND `hi` bits, and the door's interval row
//! states that as what the lane is for: "at an enclosing scalar 'the
//! same stored value' means BOTH bracket ends agree — two enclosures
//! that merely overlap are different circles to it."
//!
//! Nothing exercised it. Every circle carrier of `waisted_at::<Interval>`
//! is a POINT enclosure (measured: widest `hi - lo` over every stored
//! centre, radius and axis component is exactly `0`), so `lo` alone
//! decides every comparison the shipped rows make, and deleting the
//! `hi` conjunct leaves the whole interval lane green.
//!
//! Here two arcs of one rim are stated on circles that agree on `lo`
//! and differ by one ulp on `hi`. They are DIFFERENT stored values, the
//! seed's match is itself alone, and one arc does not close a circle —
//! so the door must refuse. With `hi` dropped they match and the door
//! answers a rim.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Bounds, Interval, Point3, Real, Tol, Vec3};
use topo::query::rim_of;
use topo::{Body, EdgeKey, FaceSurface, MefSite, MevSite, RimError};

const RIM_Z: f64 = 0.5;

fn rim_r() -> f64 {
    (1.0 - RIM_Z * RIM_Z).sqrt()
}

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
    Point3::new(iv(x), iv(y), iv(z))
}

fn v3(x: f64, y: f64, z: f64) -> Vec3<Interval> {
    Vec3::new(iv(x), iv(y), iv(z))
}

fn at(theta: f64) -> Point3<Interval> {
    let r = rim_r();
    p3(r * theta.cos(), r * theta.sin(), RIM_Z)
}

/// The rim circle, with the radius carrying the given enclosure.
fn rim_circle(radius: Interval) -> Curve3<Interval> {
    Curve3::Circle {
        center: p3(0.0, 0.0, RIM_Z),
        axis: v3(0.0, 0.0, 1.0),
        radius,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// A radius enclosure with the SAME lower end as [`rim_r`] and an upper
/// end one ulp above it: a different stored value that overlaps.
fn widened_r() -> Interval {
    let r = rim_r();
    Interval::from_bounds(r, f64::from_bits(r.to_bits() + 1))
}

/// **The spherical cap of the door's own suite, at `Interval`**, with
/// the second half-arc's carrier radius passed in — so a caller can
/// state both arcs on one value, or on two that share a lower end.
fn capped(second_radius: Interval) -> (Body<Interval>, EdgeKey, EdgeKey) {
    let tol = Tol::witness();
    let mut body = Body::<Interval>::new();
    let seed = body.mvfs(at(0.0)).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: p3(0.0, 0.0, 0.0),
            radius: iv(1.0),
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();

    let first = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(PI),
            EdgeCurveSpec::arc_of_circle(rim_circle(iv(rim_r())), iv(0.0), iv(PI)).unwrap(),
            tol,
        )
        .unwrap()
        .edge;

    let e = body.get_edge(first).unwrap();
    let (he1, he2) = (e.he_minus, e.he_plus);
    let second = body
        .mef(
            MefSite::Chords { he1, he2 },
            EdgeCurveSpec::arc_of_circle(rim_circle(second_radius), iv(PI), iv(TAU)).unwrap(),
            FaceSurface::New(Surface::Plane {
                origin: p3(0.0, 0.0, RIM_Z),
                normal: v3(0.0, 0.0, 1.0),
                u_ref: v3(1.0, 0.0, 0.0),
            }),
            tol,
        )
        .unwrap()
        .edge;

    (body, first, second)
}

/// **The control**: both arcs on one stored circle value, so the door
/// names the rim whole. Without this the row below could pass for the
/// wrong reason — a fixture the door refuses for some other cause.
#[test]
fn r2_one_stored_radius_gives_one_rim_at_the_certified_scalar() {
    let (body, a, b) = capped(iv(rim_r()));
    assert_eq!(rim_of(&body, a).unwrap(), vec![a, b]);
    assert_eq!(rim_of(&body, b).unwrap(), vec![b, a]);
}

/// **Two enclosures that share a lower end and differ on the upper are
/// NOT the same circle.** The claim the interval lane is for, made
/// falsifiable: the arcs' radii agree on every bit of `lo` and differ
/// by one ulp on `hi`, so the seed matches itself alone, the walk finds
/// no arc at its far vertex, and the door refuses `NotOneRim` naming
/// the one arc that matched.
///
/// A `same_bits` that compared only `lo` returns `Ok([a, b])` here.
#[test]
fn r2_a_one_ulp_wider_enclosure_is_a_different_circle_and_the_rim_refuses() {
    let widened = widened_r();
    assert_eq!(
        widened.lo(),
        rim_r(),
        "the two radii agree on the lower end"
    );
    assert!(widened.hi() > rim_r(), "and differ on the upper");

    let (body, a, b) = capped(widened);
    match rim_of(&body, a) {
        Err(RimError::NotOneRim { arcs, .. }) => {
            assert_eq!(arcs, vec![a], "only the seed matched its own circle");
        }
        other => panic!("an overlapping enclosure is a different circle, got {other:?}"),
    }
    match rim_of(&body, b) {
        Err(RimError::NotOneRim { arcs, .. }) => {
            assert_eq!(arcs, vec![b], "and symmetrically from the other arc");
        }
        other => panic!("an overlapping enclosure is a different circle, got {other:?}"),
    }
}
