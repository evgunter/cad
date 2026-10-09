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
use profile::test_support::bulge_loop;
use sweep::test_support::{ball_poled_z, ball_poled_z_at, brick, extruded, finished, sketch_at};
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
    /// Its length past the rim.
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
        let v = disc * self.len;
        let (body, vt, shared) = (
            tube_at(self.a, z0, self.len),
            v,
            cap_volume(SQRT_2, SQRT_2 - z0),
        );
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

/// One probe of [`the_probe_family_ships_no_wrong_body`]: the tube,
/// its partner and the three volumes every op is read against.
struct Probe {
    label: String,
    tube: AtRestBody<f64>,
    partner: AtRestBody<f64>,
    volumes: (f64, f64, f64),
}

/// `tube_at(a, z0, len)` turned by `place` about the origin.
fn placed_tube(a: f64, z0: f64, len: f64, place: Affine3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let body = topo::transform_rigid(&tube_at(a, z0, len), &place, tol).unwrap();
    finished("the tube", body, tol)
}

fn about(axis: Vec3<f64>, angle: f64) -> Affine3<f64> {
    Affine3::rotation_about_axis(Point3::origin(), axis, angle)
}

/// The probe family, enumerated:
/// - outside: radii 1, 0.3, 0.7, 0.99 and 0.05, a short tube, tilts
///   from 0.1 to 2 rad, the tube's seam on and 1e-9 off the ball's seam
///   meridians and turned, alone and tilted;
/// - the rim through the chart's pole and offsets 1e-12, ±1e-6 and
///   1e-3 from it, at radii 1 and 0.3, upright and turned;
/// - from inside: a tube touching the sphere at one rim or both,
///   upright and tilted, and a tube ending on a half ball from inside.
fn probes() -> Vec<Probe> {
    let tol = Tol::witness();
    let vb = ball_volume(SQRT_2);
    let mut out = Vec::new();
    let mut outside = |label: String, a: f64, len: f64, place: Affine3<f64>| {
        let z0 = (2.0 - a * a).sqrt();
        out.push(Probe {
            label,
            tube: placed_tube(a, z0, len, place),
            partner: ball(),
            volumes: (PI * a * a * len, vb, cap_volume(SQRT_2, SQRT_2 - z0)),
        });
    };
    for a in [1.0, 0.3, 0.7, 0.99, 0.05] {
        outside(format!("radius {a}"), a, 2.0, Affine3::identity());
    }
    outside("short".into(), 1.0, 0.5, Affine3::identity());
    for tilt in [0.1, 0.5, 1.2, FRAC_PI_2, 2.0] {
        outside(
            format!("tilt {tilt}"),
            1.0,
            2.0,
            about(Vec3::unit_x(), tilt),
        );
    }
    for turn in [0.3, FRAC_PI_2, PI, -FRAC_PI_2, 1e-9] {
        let z = about(Vec3::unit_z(), turn);
        outside(format!("turn {turn}"), 1.0, 2.0, z);
        outside(
            format!("tilt 0.5 turn {turn}"),
            0.7,
            2.0,
            z * about(Vec3::unit_x(), 0.5),
        );
    }
    for a in [1.0_f64, 0.3] {
        let pole = (a / SQRT_2).asin();
        for d in [0.0, 1e-12, 1e-6, -1e-6, 1e-3] {
            let tilt = about(Vec3::unit_x(), pole + d);
            outside(format!("pole a={a} d={d}"), a, 2.0, tilt);
            let turned = about(Vec3::unit_z(), 0.4) * tilt;
            outside(format!("pole turned a={a} d={d}"), a, 2.0, turned);
        }
    }
    for a in [1.0_f64, 0.7] {
        let (z0, disc) = ((2.0 - a * a).sqrt(), PI * a * a);
        for (label, lo, len, place) in [
            ("both rims", -z0, 2.0 * z0, Affine3::identity()),
            ("one rim", 0.0, z0, Affine3::identity()),
            (
                "one rim tilted",
                0.0,
                z0,
                about(Vec3::unit_z(), 0.3) * about(Vec3::unit_x(), 0.7),
            ),
        ] {
            out.push(Probe {
                label: format!("inside, {label}, a={a}"),
                tube: placed_tube(a, lo, len, place),
                partner: ball(),
                volumes: (disc * len, vb, disc * len),
            });
        }
    }
    let cut = finished(
        "z ≥ 0",
        brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 3.0), tol),
        tol,
    );
    let half = match topo::intersect(&ball(), &cut, tol) {
        Ok(BooleanResult::Body(b)) => b.body,
        r => panic!("the half ball: {r:?}"),
    };
    for (a, turn) in [(1.0_f64, 0.0), (0.7, 0.3)] {
        let (z0, disc) = ((2.0 - a * a).sqrt(), PI * a * a);
        out.push(Probe {
            label: format!("inside a half ball, a={a}"),
            tube: placed_tube(a, -1.0, z0 + 1.0, about(Vec3::unit_z(), turn)),
            partner: half.clone(),
            volumes: (disc * (z0 + 1.0), vb / 2.0, disc * z0),
        });
    }
    out
}

/// **The probe family ships no wrong body**, at whatever ε the run is
/// at: every op of every probe, in both member orders, builds sound at
/// its closed form (`outcome`'s `SOUND`: tiers 2 and 3′, the
/// certificate, a legal operand, the volume) or refuses typed. Which
/// ops refuse moves with ε where a probe sits in the band (the 1e-9
/// seam offset, the pole offsets); a body that builds and is wrong, or
/// cannot be measured, is the failure. The rows above pin the poses
/// whose outcome is the same at every ε.
#[test]
fn the_probe_family_ships_no_wrong_body() {
    let tol = Tol::witness();
    let mut wrong = Vec::new();
    for p in probes() {
        for (op, r, want) in every_op_both_orders(&p.tube, &p.partner, p.volumes, tol) {
            let line = outcome(r, want, tol);
            if !(line.starts_with("OK SOUND") || line.starts_with("ERR ") || line == "EMPTY ok") {
                wrong.push(format!("{}, {op} (A the tube): {line}", p.label));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} wrong:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
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

/// The planes in the ball's joined clone, its union with `tube` in both
/// member orders: the aux planes the join minted into it, since the ball
/// has none of its own.
fn ball_planes(tube: &AtRestBody<f64>) -> [usize; 2] {
    let tol = Tol::witness();
    let planes = |b: &topo::Body<f64>| {
        b.surfaces()
            .filter(|(_, s)| matches!(s, geom::Surface::Plane { .. }))
            .count()
    };
    let join = |a: &AtRestBody<f64>, b: &AtRestBody<f64>| {
        topo::test_support::boolean_through_the_join(topo::BooleanOp::Union, a, b, tol)
            .unwrap()
            .expect("the union joins")
    };
    let ball = ball();
    [planes(&join(tube, &ball).1), planes(&join(&ball, tube).0)]
}

/// **One aux plane per edge, and only where no arm reads the pair.**
///
/// - The witness turned 0.4: its rim is two arc edges, split into
///   pieces at the ball's seam meridians. The edge-plane lane mints one
///   plane into the ball per edge, keyed by the edge's split root, so
///   two, not one per piece.
/// - A pipe whose inner rim lies on the ball: the rim is an edge of its
///   top annulus, a plane, so the plane × sphere arm reads the pair and
///   mints one partner copy per pipe face it cuts the ball by (the
///   annulus and the base). The edge-plane lane, taken ahead of that
///   arm, would mint a plane per rim edge instead.
#[test]
fn the_edge_plane_lane_mints_one_plane_per_edge_and_only_where_no_arm_reads() {
    let (witness, _, _) = Pose::new(1.0, 2.0, 0.0, 0.4).tube();
    assert_eq!(ball_planes(&witness), [2, 2], "the witness turned 0.4");
    let tol = Tol::witness();
    let v = |x: f64, b: f64| (Point2::new(x, 0.0), b);
    let pipe = extruded(
        sketch_at(0.0),
        vec![
            bulge_loop(vec![v(1.6, 1.0), v(-1.6, 1.0)]),
            bulge_loop(vec![v(1.0, -1.0), v(-1.0, -1.0)]),
        ],
        1.0,
        tol,
    );
    let pipe = finished("the pipe", pipe, tol);
    assert_eq!(ball_planes(&pipe), [2, 2], "the pipe on the ball");
}
