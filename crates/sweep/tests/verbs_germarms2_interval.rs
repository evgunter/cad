//! The cyl×cyl germ arm's BODY-LEVEL poses at the CERTIFIED scalar
//! (feature `interval`).
//!
//! **What this file pins, and what it does NOT.** Every row here drives
//! whole BODIES through the public union door, so a row reaches a
//! predicate only if the layers ahead of it admit the pose. The
//! meeting-axes pose does reach the join and stops at the pinch door.
//! The skew pose does not: skew walls carry no declared cover, so it
//! stops at a CROSSING-layer door with the germ pair never minted, and
//! this file's skew row asserts exactly that and nothing about the
//! frame dispatch.
//!
//! So the new predicate's two-arm pin is NOT here. It is
//! `topo::boolean::join`'s `frame_dispatch_interval_tests`, which calls
//! `pair_section_frame` at `Interval` directly and reaches both arms —
//! meeting axes to `Zero`/pinch, skew axes to a definite sign/`NoArm`.
//! That is the certified-scalar statement about
//! `bool_germ_frame_axes_coplanar`; what this file adds is that the
//! BODY-level poses behave at `Interval` as they do at `f64`.
//!
//! The poses are the `f64` suite's, built through the same public
//! doors, so the two lanes differ in the SCALAR and in nothing else.
//! At the intersecting poses the axis-to-axis displacement is near zero
//! by construction — both operands are built centred on the origin, and
//! the seam-clearing spins are rotations ABOUT those axes, so what
//! reaches the margin is rotation crumbs rather than an exactly zero
//! vector. What was measured is the VERDICT: the coplanarity split
//! answers `Zero` at every ε row this suite is sampled at, down to the
//! 1e-12 band. The skew row's displacement is the dyadic `0.375` along
//! `â₁ × â₂`.
//!
//! The re-posed twins are held to the `f64` suite's own EQUALITY at
//! every ε row but the narrowest, and to a narrowly-shaped escape only
//! at `1e-12`, where a general rotation makes the fixture's enclosures
//! wider than the zero band and the certified lane refuses the FIXTURE
//! rather than disagreeing about the geometry.
//! [`same_door_or_escalated`] carries the measurement and both
//! narrowings.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Interval, Point2, Point3, Real, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn cyl(r: f64, h: f64) -> Body<Interval> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(iv(0.0), iv(0.0)), iv(r), tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(iv(0.0), iv(0.0), iv(-h))));
    let vp = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&vp, Extrusion::Distance(iv(2.0 * h)), tol)
        .unwrap()
        .body
}

fn spin(b: &Body<Interval>, axis: Vec3<Interval>, angle: f64) -> Body<Interval> {
    topo::transform_rigid(
        b,
        &Affine3::rotation_about_axis(Point3::new(iv(0.0), iv(0.0), iv(0.0)), axis, iv(angle)),
        Tol::witness(),
    )
    .unwrap()
}

fn x_axis() -> Vec3<Interval> {
    Vec3::new(iv(1.0), iv(0.0), iv(0.0))
}

fn y_axis() -> Vec3<Interval> {
    Vec3::new(iv(0.0), iv(1.0), iv(0.0))
}

fn z_axis() -> Vec3<Interval> {
    Vec3::new(iv(0.0), iv(0.0), iv(1.0))
}

fn repose(b: &Body<Interval>) -> Body<Interval> {
    let r = Affine3::rotation_about_axis(
        Point3::new(iv(0.0), iv(0.0), iv(0.0)),
        Vec3::new(iv(1.0), iv(2.0), iv(3.0)).normalize(),
        iv(0.7),
    );
    topo::transform_rigid(
        b,
        &Affine3::from_parts(
            r.linear,
            r.translation + Vec3::new(iv(0.3), iv(-0.45), iv(0.6)),
        ),
        Tol::witness(),
    )
    .unwrap()
}

fn union_err(a: &Body<Interval>, b: &Body<Interval>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("this family has no join arm")
}

/// **The re-posed twin's obligation at the CERTIFIED scalar**, which is
/// the `f64` suite's EQUALITY at every ε row but the narrowest, and a
/// narrowly-shaped escape only there.
///
/// The re-pose is a general rotation, so the transformed operand's
/// coordinates are irrational and its enclosures are intervals rather
/// than points. At `eps = 1e-9` (the compiled default) and `eps = 1e-6`
/// those enclosures are far inside the zero band and the twin answers
/// the direct pose's door key for key — measured, so it is asserted as
/// an equality rather than assumed away. At `eps = 1e-12` the
/// enclosures are wider than the band itself, and the certified lane
/// refuses the FIXTURE before the geometry is ever in question —
/// measured, on the pinch-door pose:
///
/// ```text
/// CrossingInsertion { operand: A, edge: EdgeKey(7v1), source: Certification {
///   error: Escalated { check: MappedSource, sample: 7, cause: Indeterminate {
///     margin: Enclosure { lo: 0.0, hi: 1.0970253705214178e-12 },
///     band: Band { zero: 1e-12, escalate: 1e-11 },
///     predicate: Some("carrier_matches_mapped_source") } } } }
/// ```
///
/// **Both narrowings matter, and neither is cosmetic.**
///
/// - The escape is gated on the BAND, read from the run's own
///   tolerance, so at the two wider rows an escalation is a FAILURE.
///   A blanket "or escalates" would have accepted an escalation where
///   the measurement says the door is answered exactly.
/// - The escape matches the escalation's SHAPE, not a substring of its
///   rendering. It admits the certification escalation above — a
///   crossing-insertion refusal whose predicate is
///   `carrier_matches_mapped_source` — and it explicitly refuses an
///   escalation of either germ-frame predicate, because those are the
///   very predicates this unit's rows exist to pin: a substring test
///   would have greened a `bool_germ_frame_axes_coplanar` escalation,
///   i.e. the arm going indeterminate, as if it were noise.
fn same_door_or_escalated(direct: &BooleanError, reposed: &BooleanError, what: &str) {
    let (d, r) = (format!("{direct:?}"), format!("{reposed:?}"));
    if d == r {
        return;
    }
    let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
    assert!(
        band.zero() <= 1e-12,
        "{what}: at eps with zero band {:e} the re-posed twin must answer the direct \
         pose's door key for key; direct {d}, re-posed {r}",
        band.zero()
    );
    let BooleanError::CrossingInsertion { source, .. } = reposed else {
        panic!("{what}: the only admitted escape is a crossing-insertion escalation; got {r}");
    };
    let topo::EulerOpError::Certification { error } = source else {
        panic!("{what}: the only admitted escape is a certification refusal; got {r}");
    };
    let geom_brep::CertifyError::Escalated { cause, .. } = error else {
        panic!("{what}: the only admitted escape is an ESCALATION; got {r}");
    };
    assert!(
        !matches!(
            cause.predicate,
            Some("bool_germ_frame_axes_coplanar" | "bool_germ_frame_axes_parallel")
        ),
        "{what}: a germ-frame predicate going indeterminate is the defect this unit \
         pins, never an accepted escape; direct {d}, re-posed {r}"
    );
}

/// **The meeting-axes arm.** The pose whose seams sit off the pinch
/// reaches the join at the certified scalar too, and stops at the door
/// that names the pinch — the same door the `f64` lane reaches. An
/// escalation on the DIRECT pose would say the enclosures decided the
/// lane rather than the geometry, so that half is an equality; the
/// re-posed twin's obligation is [`same_door_or_escalated`]'s, and the
/// reason is written there.
#[test]
fn the_pinch_door_is_reached_at_the_certified_scalar() {
    let a = spin(&cyl(1.0, 1.2), z_axis(), PI / 4.0);
    let b = spin(
        &spin(&cyl(1.0, 1.2), x_axis(), PI / 2.0),
        y_axis(),
        PI / 4.0,
    );
    let err = union_err(&a, &b);
    assert!(
        matches!(err, BooleanError::GermFrameCylinderPinch { .. }),
        "{err:?}"
    );
    same_door_or_escalated(
        &err,
        &union_err(&repose(&a), &repose(&b)),
        "the pinch-door pose",
    );
}

/// **The skew pose at the BODY level**, which is a weaker statement
/// than its name once suggested and is written as the weaker one.
///
/// Slide the pair along the common perpendicular `â₁ × â₂` by the
/// dyadic `0.375` and the pose stops at a CROSSING-layer door: skew
/// walls carry no declared cover, so the germ pair is never minted and
/// `bool_germ_frame_axes_coplanar` is never reached from here. What
/// this row asserts is therefore the LAYER — the pose stays off every
/// join door, the pinch door included — and not the dispatch's skew
/// arm, which is pinned at the certified scalar in
/// `topo::boolean::join`'s `frame_dispatch_interval_tests` instead.
#[test]
fn a_skew_pair_stays_off_the_pinch_door_at_the_certified_scalar() {
    let a = cyl(1.0, 2.0);
    let skew = topo::transform_rigid(
        &spin(&cyl(1.0, 2.0), x_axis(), PI / 2.0),
        &Affine3::translation(Vec3::new(iv(0.375), iv(0.0), iv(0.0))),
        Tol::witness(),
    )
    .unwrap();
    let err = union_err(&a, &skew);
    assert!(
        matches!(
            err,
            BooleanError::CurvedPierceUnsupported { .. }
                | BooleanError::CurvedSectorSideUnsupported { .. }
        ),
        "a skew pair must stop at a crossing-layer door, never a join one: {err:?}"
    );
    // The re-posed twin, on the same obligation as every other row
    // here: a rigid motion moves no contact, so a pose whose direct and
    // re-posed copies disagree is a defect by construction.
    same_door_or_escalated(
        &err,
        &union_err(&repose(&a), &repose(&skew)),
        "the skew pose",
    );
}

/// The Steinmetz pose itself keeps the crossing layer's tangency door
/// at the certified scalar: its seams are ON the pinch points, and a
/// tangency is not a crossing at any order this lane reads.
#[test]
fn the_steinmetz_pose_keeps_the_tangency_door_at_the_certified_scalar() {
    let a = cyl(1.0, 2.0);
    let b = spin(&cyl(1.0, 2.0), x_axis(), PI / 2.0);
    let err = union_err(&a, &b);
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "{err:?}"
    );
    same_door_or_escalated(
        &err,
        &union_err(&repose(&a), &repose(&b)),
        "the Steinmetz pose",
    );
}
