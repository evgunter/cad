//! **A tube ending on a ball.** A tube whose end rim lies on a ball
//! meets it in that rim: the crossing layer reads the rim as an ON
//! event in the ball's faces, split where it crosses their seam
//! meridians, and the join chords each piece as a segment along the
//! tube's rim edge, inside a ball face. The tube's chord is a copy of
//! its edge; the ball's is its face cut by the rim's plane
//! (`boolean::join`'s `GermLane::EdgePlane`).
//!
//! The ball is `ball_poled_z` of radius `√2` about the origin, its seam
//! meridians in the `xz` plane. A `z`-axis tube of radius `a` standing
//! on it rises from `z0 = √(2 − a²)`, where its rim lies on the sphere,
//! and is then tilted about `x` and turned about `z` about the origin,
//! so its rim stays on the sphere. The ball keeps of the tube only the
//! cap above the rim's plane, which lies inside the tube, so every op's
//! volume is read from the cap's closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2};

use geom_core::{Affine3, Bounds, Interval, Point2, Point3, Real, Tol, Vec3};
use sweep::test_support::{ball_poled_z, ball_poled_z_at, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult, MassPropsError, ValidationError};

use crate::common::differential::{assert_sound_and_meshed, every_op_both_orders, outcome};
use crate::common::oracles::{ball_volume, cap_volume};

/// The `z`-axis tube of radius `a` over `z ∈ [z0, z0 + len]`.
fn tube_at<T: geom_core::Decide + topo::AtRestPolicy>(a: T, z0: T, len: T) -> topo::Body<T> {
    let tol = Tol::witness();
    let o = T::zero();
    let circle = profile::circle(Point2::new(o, o), a, tol).unwrap();
    let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(o, o, z0)));
    let p = profile::Profile::new(plane, vec![circle.into()])
        .validate(tol)
        .unwrap();
    let side = ExtrudeSide::Along;
    extrude(&p, Extrusion::Distance { depth: len, side }, tol)
        .unwrap()
        .body
}

/// One tube pose against the ball.
struct Pose {
    /// The tube's radius.
    a: f64,
    /// Its length past the rim, `len < 0` standing it inside the ball:
    /// from `−z0` up to the rim.
    len: f64,
    /// Its tilt about `x`, then its turn about `z`.
    tilt: f64,
    turn: f64,
}

impl Pose {
    const fn new(a: f64, len: f64, tilt: f64, turn: f64) -> Self {
        Self { a, len, tilt, turn }
    }

    fn z0(&self) -> f64 {
        (2.0 - self.a * self.a).sqrt()
    }

    /// The tube, placed, and its volume and the volume it shares with
    /// the ball.
    fn tube(&self) -> (AtRestBody<f64>, f64, f64) {
        let tol = Tol::witness();
        let (z0, disc) = (self.z0(), PI * self.a * self.a);
        let (body, vt, shared) = if self.len < 0.0 {
            let v = disc * 2.0 * z0;
            (tube_at(self.a, -z0, 2.0 * z0), v, v)
        } else {
            let v = disc * self.len;
            (
                tube_at(self.a, z0, self.len),
                v,
                cap_volume(SQRT_2, SQRT_2 - z0),
            )
        };
        let o = Point3::origin();
        let place = Affine3::rotation_about_axis(o, Vec3::unit_z(), self.turn)
            * Affine3::rotation_about_axis(o, Vec3::unit_x(), self.tilt);
        let placed = topo::transform_rigid(&body, &place, tol).unwrap();
        (finished("the tube", placed, tol), vt, shared)
    }
}

fn ball() -> AtRestBody<f64> {
    let tol = Tol::witness();
    finished("the ball", ball_poled_z(SQRT_2, Vec3::zero(), tol), tol)
}

/// Every op in both orders of `pose`'s tube against the ball, labelled.
#[allow(clippy::type_complexity)] // (label, result, closed form) per run
fn runs(name: &str, pose: &Pose) -> Vec<(String, Result<BooleanResult<f64>, BooleanError>, f64)> {
    let (tube, vt, shared) = pose.tube();
    every_op_both_orders(
        &tube,
        &ball(),
        (vt, ball_volume(SQRT_2), shared),
        Tol::witness(),
    )
    .into_iter()
    .map(|(op, r, want)| (format!("{name}, {op} (A the tube)"), r, want))
    .collect()
}

/// **Every op builds**, sound, meshed and at its closed form, in both
/// member orders: the witness (radius 1, its wall meeting the sphere at
/// 45°) and a short one, the rim's latitude varied by the radius, and
/// the tube's own seam vertex on either of the ball's seam meridians
/// (turns 0 and π) and off them.
#[test]
fn every_op_builds_at_its_closed_form() {
    let poses = [
        ("the witness", Pose::new(1.0, 2.0, 0.0, 0.0)),
        ("a short tube", Pose::new(1.0, 0.5, 0.0, 0.0)),
        ("radius 0.3", Pose::new(0.3, 2.0, 0.0, 0.0)),
        ("radius 0.7", Pose::new(0.7, 2.0, 0.0, 0.0)),
        ("radius 0.99", Pose::new(0.99, 2.0, 0.0, 0.0)),
        ("the tube's seam on −x", Pose::new(1.0, 2.0, 0.0, PI)),
        ("the tube's seam on +y", Pose::new(1.0, 2.0, 0.0, FRAC_PI_2)),
    ];
    for (name, pose) in &poses {
        for (label, r, want) in runs(name, pose) {
            assert_sound_and_meshed(&label, r, want, Tol::witness());
        }
    }
}

/// **A tilted tube builds**: its rim is no latitude of the ball's chart
/// and crosses the seam meridians askew. Every op is sound at its closed
/// form, read without a mesh: a sphere face bounded by a circle off its
/// chart's latitudes has no tessellation lane
/// (`work/tess/sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane.md`).
#[test]
fn a_tilted_tube_builds_at_its_closed_form() {
    let tol = Tol::witness();
    for (label, r, want) in runs("tilted", &Pose::new(0.7, 2.0, 0.5, 0.3)) {
        let line = outcome(r, want, tol);
        assert!(line.starts_with("OK SOUND"), "{label}: {line}");
    }
}

/// **A rim inside one ball face holes it.** Tilted far enough, the rim
/// crosses neither seam meridian, so the ball face keeps it as a ring:
/// the union and `ball ∖ tube` refuse at the result gate
/// (`work/flux/sphere-face-with-a-hole-has-no-closed-form.md`), and the
/// four ops that keep no holed ball face build sound, read without a
/// mesh as the tilted tube's are.
#[test]
fn a_rim_inside_one_ball_face_holes_it() {
    let tol = Tol::witness();
    for (label, r, want) in runs("tilted 1.2", &Pose::new(1.0, 2.0, 1.2, 0.0)) {
        if label.contains('∪') || label.starts_with("tilted 1.2, B ∖ A") {
            let Err(BooleanError::ResultInvalid { errors }) = &r else {
                panic!(
                    "{label}: wanted the result gate, got {}",
                    outcome(r, want, tol)
                );
            };
            assert!(
                matches!(
                    errors.as_slice(),
                    [ValidationError::VolumeUncomputable {
                        source: MassPropsError::RingOnCurvedFace { .. },
                        ..
                    }]
                ),
                "{label}: {errors:?}"
            );
        } else {
            let line = outcome(r, want, tol);
            assert!(line.starts_with("OK SOUND"), "{label}: {line}");
        }
    }
}

/// **A tube ending on the ball from inside** touches the sphere along
/// its rim and crosses nothing, so the no-crossings path takes it, and
/// its extent scan cannot place the sphere's circle in a disc whose
/// boundary it is: every op refuses there
/// (`work/contact/a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan.md`).
#[test]
fn a_tube_ending_on_the_ball_from_inside_refuses_at_the_extent_scan() {
    for (label, r, _) in runs("inside", &Pose::new(1.0, -1.0, 0.0, 0.0)) {
        assert!(
            matches!(r, Err(BooleanError::FallbackExtentUnsupported { .. })),
            "{label}: {r:?}"
        );
    }
}

/// **The witness at the certified scalar**: every op builds at tier 3
/// and its volume enclosure brackets the closed form.
#[test]
fn the_witness_brackets_its_closed_forms_at_the_certified_scalar() {
    let tol = Tol::witness();
    let iv = Interval::from_f64;
    let ball = finished(
        "the ball",
        ball_poled_z_at(iv(2.0).sqrt(), Vec3::new(iv(0.0), iv(0.0), iv(0.0)), tol),
        tol,
    );
    let tube = finished("the tube", tube_at(iv(1.0), iv(1.0), iv(2.0)), tol);
    let (vt, vb, shared) = (
        2.0 * PI,
        ball_volume(SQRT_2),
        cap_volume(SQRT_2, SQRT_2 - 1.0),
    );
    let ops = [
        ("A ∪ B", topo::union(&tube, &ball, tol), vt + vb - shared),
        ("B ∪ A", topo::union(&ball, &tube, tol), vt + vb - shared),
        ("A ∩ B", topo::intersect(&tube, &ball, tol), shared),
        ("B ∩ A", topo::intersect(&ball, &tube, tol), shared),
        ("A ∖ B", topo::subtract(&tube, &ball, tol), vt - shared),
        ("B ∖ A", topo::subtract(&ball, &tube, tol), vb - shared),
    ];
    for (op, r, want) in ops {
        let bb = match r.as_ref().map(BooleanResult::body) {
            Ok(Some(bb)) => bb,
            _ => panic!("{op}: wanted a body, got {r:?}"),
        };
        topo::validate_geometric(&bb.body, tol).unwrap_or_else(|e| panic!("{op}: tier 3: {e:?}"));
        let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
        assert!(
            v.lo() <= want && want <= v.hi() && v.hi() - v.lo() < 1e-9,
            "{op}: [{}, {}] against {want}",
            v.lo(),
            v.hi()
        );
    }
}
