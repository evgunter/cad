//! **A ball against a donut, through the carried lane onto a torus
//! face.** The ball (radius 1/2, poled along `y`) and the donut
//! (`R = 0.6`, `r = 0.2`, revolved about `y`) at placements of the germ
//! row (`work/sect/torus-germ-pairs-have-no-section-frame.md`):
//!
//! - the donut turned 0.7 rad about `x` and moved to `(0.1, 0.35, 0.2)`
//!   (ball ∪ donut): a carried chord closes a ring on a torus face
//!   (`RingClosure::Chord` on the torus's ring path);
//! - the ball turned 1.1 rad about `(0.3, 1, 0.2)` and moved to
//!   `(0.42, −0.15, 0.3)` (donut ∪ ball): a spline edge in the run of a
//!   torus ring (`run_loop_arcs` on the torus);
//! - the donut turned 0.9 rad about `(1, 1, 0)` and moved to
//!   `(0.05, 0.1, 0.02)`, ∪ in both member orders: the ball cuts both
//!   tube circles of its meridian, two curves about the donut's axis
//!   that no plane or meridian tells apart; each germ's component is its
//!   site's foot on the carried section.
//!
//! Each union builds, its chorded sphere and torus faces measured
//! through the rotational lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body, BooleanError};

fn ball() -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -0.5), 1.0),
        (Point2::new(0.0, 0.5), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
        .into_body()
}

fn donut() -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(0.6, -0.2), 1.0),
        (Point2::new(0.6, 0.2), 1.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
    .into_body()
}

/// `b` turned by `angle` about `axis` through the origin, then moved by `t`.
fn placed(b: &Body<f64>, axis: Vec3<f64>, angle: f64, t: Vec3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let turned = topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
            tol,
        ),
    )
    .unwrap();
    let moved =
        topo::transform_rigid(&turned, &geom_core::RigidMotion::translation(t).unwrap()).unwrap();
    finished("the placed operand", moved, tol)
}

/// Where a run ends: `"builds"` for one seamed solid, else its refusal.
fn outcome(got: &Result<topo::BooleanResult<f64>, BooleanError>) -> String {
    match got {
        Ok(r)
            if matches!(
                r.body().map(|b| b.kind),
                Some(topo::BooleanResultKind::Seamed)
            ) =>
        {
            "builds".to_owned()
        }
        Ok(r) => format!("{:?}", r.body().map(|b| b.kind)),
        Err(e) => format!("{e:?}"),
    }
}

/// **A ball against a donut at four placements builds**: each union's
/// chorded sphere faces and its torus faces measure through the
/// rotational lane, the torus ring's face and the faces of a ring's run
/// alike. No body here is held to a volume of its own
/// (`work/spiricarm/the-general-pose-torus-germ-unions-that-build-have-no-volume-truth.md`).
#[test]
fn a_ball_against_a_donut_carries_onto_the_torus_and_builds() {
    // Pinned red at every ε: the spline edge's placement (donut first)
    // leaves a strut ring on the donut's torus face that no path places;
    // it is deferred, and its own polygon then completes inside it, where
    // no join places it either
    // (`work/lathe/a-strut-rings-side-is-read-from-its-own-germ.md`). The
    // other three placements are read first, so they still run.
    test_utils::pinned_red::row(
        Tol::witness().eps(),
        &test_utils::pinned_red::EVERY_ROW,
        "a ball against a donut at four placements",
        &[
            "a spline edge in a torus ring's run",
            "RingHomingAmbiguous { ring: LoopKey(7v1) }",
        ],
        placements_build,
    );
}

fn placements_build() {
    let tol = Tol::witness();
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let (ball0, donut0) = (
        placed(&ball(), Vec3::unit_x(), 0.0, origin),
        placed(&donut(), Vec3::unit_x(), 0.0, origin),
    );
    let ring = placed(&donut(), Vec3::unit_x(), 0.7, Vec3::new(0.1, 0.35, 0.2));
    let moved = placed(
        &ball(),
        Vec3::new(0.3, 1.0, 0.2),
        1.1,
        Vec3::new(0.42, -0.15, 0.3),
    );
    let about = placed(
        &donut(),
        Vec3::new(1.0, 1.0, 0.0),
        0.9,
        Vec3::new(0.05, 0.1, 0.02),
    );
    let got: Vec<(&str, String, Option<String>)> = [
        (
            "a carried chord closing a ring on a torus face",
            &ball0,
            &ring,
        ),
        ("a spline edge in a torus ring's run", &donut0, &moved),
        ("two curves about the axis, ball ∪ donut", &ball0, &about),
        ("two curves about the axis, donut ∪ ball", &about, &ball0),
    ]
    .into_iter()
    .map(|(what, x, y)| {
        let r = topo::union(x, y, tol);
        (
            what,
            outcome(&r),
            crate::germ_torus_components::union_floor(&r, x, y),
        )
    })
    .collect();
    // The spline edge's placement last, so a stop there leaves every
    // other placement read.
    let spline = "a spline edge in a torus ring's run";
    let ordered = got
        .iter()
        .filter(|g| g.0 != spline)
        .chain(got.iter().filter(|g| g.0 == spline));
    for (what, end, floor) in ordered {
        assert_eq!(end, "builds", "{what}: {got:#?}");
        assert!(floor.is_none(), "{what}: {floor:?}");
    }
}

/// **The spline edge's placement, under every op**: the ball turned and
/// moved against the donut at the origin, where a section loop lies
/// inside the one-band ball and the one-band donut and its cut-in leaves
/// strut rings on the donut's torus face that no path places.
///
/// - With the ball first, ∪, ∩ and ∖ build. Each body passes tier 3′ (or
///   leaves only pairs the census cannot decide), and the three agree on
///   one `V(O ∩ B)` within `[0, min(V(donut), V(ball))]` by
///   inclusion–exclusion against the operands' certified volumes
///   ([`crate::common::op_census::assert_one_meet`]). The pose has no
///   independent volume, so the agreement is the oracle. At ε 1e-12 these
///   three run past 300 s each, on main as on this row's head, and are
///   skipped loudly
///   (`work/lathe/the-ball-first-ops-of-the-ball-donut-spline-placement-run-for-minutes-at-1e-12.md`).
/// - With the donut first, each op refuses `RingHomingAmbiguous` at the
///   strut ring whose own polygon completes inside it, deferred and never
///   placed (`work/lathe/a-strut-rings-side-is-read-from-its-own-germ.md`).
///   Pinned: red here the day it builds, so this row moves with the fix.
#[test]
fn the_spline_edge_placement_under_every_op() {
    use crate::common::op_census::{Keep, assert_one_meet, kernel_invariant};
    use topo::{BooleanResult, ValidationError};
    let tol = Tol::witness();
    // The certified enclosure at the pinned resolution, its bracket
    // carried: at ε 1e-6 a body here stops at the floor short of a met
    // number, and inclusion–exclusion reads the bracket either way.
    let bracket = |what: &str, body: &AtRestBody<f64>| {
        let v = topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap_or_else(|e| panic!("{what}: the volume: {e:?}"))
            .enclosure;
        (v.volume, v.volume_pad)
    };
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let donut0 = placed(&donut(), Vec3::unit_x(), 0.0, origin);
    let moved = placed(
        &ball(),
        Vec3::new(0.3, 1.0, 0.2),
        1.1,
        Vec3::new(0.42, -0.15, 0.3),
    );
    let (o, b) = (&donut0, &moved);
    for (op, got) in [
        ("O∪B", topo::union(o, b, tol)),
        ("O∩B", topo::intersect(o, b, tol)),
        ("O∖B", topo::subtract(o, b, tol)),
    ] {
        let e = got
            .err()
            .unwrap_or_else(|| panic!("{op}: refuses at the strut ring"));
        assert!(!kernel_invariant(&e), "{op}: a kernel invariant: {e:?}");
        assert_eq!(
            format!("{e:?}"),
            "Join(BooleanJoin(RingHomingAmbiguous { ring: LoopKey(7v1) }))",
            "{op}: the strut ring's stop"
        );
    }
    if tol.eps() == 1e-12 {
        println!(
            "SKIPPED at ε 1e-12: the ball-first ops run past 300 s each, on main as here \
             (work/lathe/the-ball-first-ops-of-the-ball-donut-spline-placement-run-for-minutes-at-1e-12.md)"
        );
        return;
    }
    let (vo, pad_o) = bracket("the donut", o);
    let (vb, pad_b) = bracket("the ball", b);
    let mut implied = Vec::new();
    for (op, keep, got) in [
        ("B∪O", Keep::Union, topo::union(b, o, tol)),
        ("B∩O", Keep::Meet, topo::intersect(b, o, tol)),
        ("B∖O", Keep::BLessO, topo::subtract(b, o, tol)),
    ] {
        let r = got.unwrap_or_else(|e| panic!("{op}: builds: {e:?}"));
        let BooleanResult::Body(body) = r else {
            panic!("{op}: an empty result where the operands overlap");
        };
        let (v, pad) = bracket(op, &body.body);
        implied.push((op, keep.meet(vo, vb, v), pad));
        // Tier 3′ passes, or fails only on pairs the census cannot decide.
        let t3p = topo::validate_pseudomanifold(&body.body, &body.contacts, tol);
        let held = t3p.as_ref().map_or_else(
            |es| {
                es.iter()
                    .all(|e| matches!(e, ValidationError::CensusUndecidable { .. }))
            },
            |()| true,
        );
        assert!(held, "{op}: tier 3′ {t3p:?}");
    }
    assert_one_meet(
        "the moved ball and the donut",
        &implied,
        vo,
        pad_o + pad_b,
        vb,
    );
}
