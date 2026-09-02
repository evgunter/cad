//! The placeholder discriminator's WIDTH: it reads every channel of
//! every control point, not the first channel of every control point.
//!
//! The crate docs' totality-and-poison section states the rule the
//! shared `net` helper implements — *a placeholder's every control
//! point is all-poison, and a described net carrying poison must fail
//! loudly as such, never masquerade as the benign placeholder*. The
//! narrow reading admits exactly one masquerade: a net whose every
//! point has a poisoned `x` and a finite `y`/`z` is corrupt described
//! geometry that answers "placeholder" and is then handed the benign
//! arm at every consumer that tells the two states apart.
//!
//! What this suite pins, at `f64` and `Dual<f64>`:
//!
//! - the masquerading net reads DESCRIBED, on both halves (the row the
//!   narrow predicate fails);
//! - its mirror — a finite first channel over poisoned rest — reads
//!   described too, which it always did, so the widening moves nets in
//!   one direction only;
//! - the placeholder both payload doors mint still reads placeholder;
//! - the direction itself, as a property: the widened predicate implies
//!   the narrow one over a battery, so the placeholder set only shrank
//!   and no consumer's placeholder arm gained a member;
//! - what the masquerade actually is downstream — evaluation carries
//!   the poison in the one channel and finite values in the others, and
//!   the projection door refuses rather than returning a foot.
//!
//! The interval lane's twin is `net_placeholder_width_interval.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use bvh::{Aabb, Axis};
use geom::curves::boxes::nurbs_curve_aabb;
use geom::surfaces::boxes::nurbs_surface_aabb;
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Dual, Point3, Real};

/// A clamped quadratic knot vector over five control points.
fn knots5() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0], 2).unwrap()
}

/// A bilinear knot vector over a 2x2 net.
fn knots2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// The masquerade, curve half: every control point carries a poisoned
/// `x` and a finite `y`/`z`. Structurally valid — the constructor
/// validates counts and weights, and a coordinate is data.
fn masquerading_curve<T: Real>() -> NurbsCurve3<T> {
    let control: Vec<Point3<T>> = (0..5)
        .map(|i| {
            Point3::new(
                T::from_f64(f64::NAN),
                T::from_f64(1.0),
                T::from_f64(f64::from(i)),
            )
        })
        .collect();
    NurbsCurve3::new(knots5(), control, vec![1.0; 5]).unwrap()
}

/// The masquerade, surface half: the same shape on a bilinear net.
fn masquerading_surface<T: Real>() -> NurbsSurface<T> {
    let control: Vec<Point3<T>> = (0..4)
        .map(|i| {
            Point3::new(
                T::from_f64(f64::NAN),
                T::from_f64(f64::from(i)),
                T::from_f64(2.0),
            )
        })
        .collect();
    NurbsSurface::new(knots2(), knots2(), control, vec![1.0; 4]).unwrap()
}

/// The mirror: a FINITE first channel over a poisoned rest.
fn mirrored_curve<T: Real>() -> NurbsCurve3<T> {
    let control: Vec<Point3<T>> = (0..5)
        .map(|i| {
            Point3::new(
                T::from_f64(f64::from(i)),
                T::from_f64(f64::NAN),
                T::from_f64(f64::NAN),
            )
        })
        .collect();
    NurbsCurve3::new(knots5(), control, vec![1.0; 5]).unwrap()
}

/// The row the narrow predicate fails: a described net whose every
/// control point has a poisoned `x` and a finite `y`/`z` is NOT the
/// benign placeholder, on either half.
#[test]
fn a_net_poisoned_in_one_channel_is_described_not_the_placeholder() {
    let c = masquerading_curve::<f64>();
    assert!(
        !c.is_placeholder(),
        "a control net with finite y and z is described geometry, corrupt in x — \
         answering `placeholder` hands it the benign arm at every consumer"
    );
    let s = masquerading_surface::<f64>();
    assert!(
        !s.is_placeholder(),
        "the surface half answers the same rule as the curve half"
    );
    // The enum doors carry the same answer: this is what a consumer
    // matching on `Surface::Nurbs(payload)` asks.
    let e = Surface::Nurbs(Arc::new(s));
    assert!(matches!(&e, Surface::Nurbs(n) if !n.is_placeholder()));
    let ec = Curve3::Nurbs(Arc::new(c));
    assert!(matches!(&ec, Curve3::Nurbs(n) if !n.is_placeholder()));
}

/// The mirror of that net answers described as well — it did under the
/// narrow reading and it does under the wide one. Both are corrupt
/// described geometry; neither is the placeholder.
#[test]
fn the_mirror_with_a_finite_first_channel_is_described_too() {
    let c = mirrored_curve::<f64>();
    assert!(!c.is_placeholder());
    assert!(!c.map_scalar(Dual::constant).is_placeholder());
}

/// The state the predicate exists to name still answers it, at both
/// payload doors and both enum doors.
#[test]
fn the_minted_placeholder_still_reads_as_the_placeholder() {
    assert!(NurbsCurve3::<f64>::placeholder().is_placeholder());
    assert!(NurbsSurface::<f64>::placeholder().is_placeholder());
    assert!(matches!(
        Curve3::<f64>::nurbs_placeholder(),
        Curve3::Nurbs(ref n) if n.is_placeholder()
    ));
    assert!(matches!(
        Surface::<f64>::nurbs_placeholder(),
        Surface::Nurbs(ref n) if n.is_placeholder()
    ));
    // And through a lift, which is where the two states must not swap.
    let d = Curve3::<f64>::nurbs_placeholder().map_scalar(Dual::constant);
    assert!(matches!(&d, Curve3::Nurbs(n) if n.is_placeholder()));
}

/// The DIRECTION of the change, as a property rather than a sentence:
/// every net the widened predicate calls a placeholder was one under
/// the narrow reading too (all-poison implies a poisoned first
/// channel), so the placeholder set only shrank. No consumer's
/// placeholder arm gained a member; some lost one.
#[test]
fn the_widened_predicate_implies_the_narrow_one() {
    let described = NurbsCurve3::new(
        knots5(),
        (0..5)
            .map(|i| Point3::new(f64::from(i), 1.0, 0.0))
            .collect(),
        vec![1.0; 5],
    )
    .unwrap();
    let mut one_point_poisoned = described.control().to_vec();
    one_point_poisoned[2] = Point3::new(f64::NAN, f64::NAN, f64::NAN);
    let battery = [
        NurbsCurve3::<f64>::placeholder(),
        masquerading_curve::<f64>(),
        mirrored_curve::<f64>(),
        described,
        NurbsCurve3::new(knots5(), one_point_poisoned, vec![1.0; 5]).unwrap(),
    ];
    for c in &battery {
        if c.is_placeholder() {
            assert!(
                c.control().iter().all(|p| p.x.is_poison()),
                "the widened answer must be a subset of the narrow one"
            );
        }
    }
    // And the subset is PROPER: the masquerade is the witness.
    let m = masquerading_curve::<f64>();
    assert!(m.control().iter().all(|p| p.x.is_poison()));
    assert!(!m.is_placeholder());
}

/// What the masquerade is downstream, at the doors a consumer reaches
/// through: evaluation carries the poison in the poisoned channel and
/// finite values in the others — which is why a consumer's y/z margin
/// still decides a sign and the face never refuses — and the surface
/// projection door refuses rather than returning a foot.
#[test]
fn the_masquerade_poisons_evaluation_and_refuses_projection() {
    let c = masquerading_curve::<f64>();
    let p = c.eval(0.5);
    assert!(p.x.is_nan(), "the poisoned channel evaluates to poison");
    assert!(
        p.y.is_finite() && p.z.is_finite(),
        "the other channels evaluate finitely — this is the masquerade's real shape, \
         and a partially poisoned answer is what a consumer then decides against"
    );
    let s = masquerading_surface::<f64>();
    let q = s.eval(0.5, 0.5);
    assert!(q.x.is_nan() && q.y.is_finite() && q.z.is_finite());
    // The chart-image lane's own door (a foot point on the wall): a
    // poisoned Newton system converges nowhere and says so.
    assert!(
        s.project(Point3::new(0.0, 0.0, 0.0)).is_err(),
        "projection onto a poisoned chart must refuse, never return a best-effort foot"
    );
}

/// The lift neither narrows nor widens the rule (the scalar-lift
/// module's one home for this argument): the masquerade lifts to a
/// described net, the placeholder to the placeholder.
#[test]
fn the_widened_answer_survives_a_lift() {
    let e = Curve3::Nurbs(Arc::new(masquerading_curve::<f64>()));
    let d = e.map_scalar(Dual::constant);
    assert!(matches!(&d, Curve3::Nurbs(n) if !n.is_placeholder()));
    let s = Surface::Nurbs(Arc::new(masquerading_surface::<f64>()));
    let sd = s.map_scalar(Dual::constant);
    assert!(matches!(&sd, Surface::Nurbs(n) if !n.is_placeholder()));
}

/// The box doors are the second member of the same class, and the one
/// that lives in this crate: a certified-conservative AABB folded over
/// a net poisoned in ONE channel is poison on that axis and FINITE on
/// the others, and `Aabb::overlaps` is a per-axis test — so the finite
/// axes witness disjointness and the box PRUNES. A box that prunes on
/// geometry it cannot bound is unsound, and the module's own contract
/// (poison never prunes) is what it violates.
#[test]
fn a_net_poisoned_in_one_channel_yields_the_poison_box_on_every_axis() {
    // A box far away on y and z, disjoint from the masquerade's finite
    // lanes: the witness a partially poisoned box would answer with.
    let elsewhere = Aabb::from_points(
        [
            Point3::new(0.0, 500.0, 500.0),
            Point3::new(1.0, 501.0, 501.0),
        ]
        .into_iter(),
    )
    .unwrap();

    for (what, b) in [
        (
            "surface",
            nurbs_surface_aabb(&masquerading_surface::<f64>()),
        ),
        ("curve", nurbs_curve_aabb(&masquerading_curve::<f64>())),
    ] {
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            assert!(
                b.min(axis).is_nan() && b.max(axis).is_nan(),
                "{what}: a net carrying poison anywhere has no honest box on any axis"
            );
        }
        assert!(
            b.overlaps(&elsewhere),
            "{what}: the poison box must never prune — that is the whole of its contract"
        );
    }

    // The placeholder's documented answer is unchanged: still poison.
    let ph = nurbs_surface_aabb(&NurbsSurface::<f64>::placeholder());
    assert!(ph.min(Axis::X).is_nan() && ph.overlaps(&elsewhere));

    // And a fully described net still gets its real, pruning box.
    let described = NurbsSurface::new(
        knots2(),
        knots2(),
        (0..4)
            .map(|i| Point3::new(f64::from(i), 1.0, 2.0))
            .collect(),
        vec![1.0; 4],
    )
    .unwrap();
    assert!(
        !nurbs_surface_aabb(&described).overlaps(&elsewhere),
        "the screen must not cost a described net its pruning power"
    );
}
