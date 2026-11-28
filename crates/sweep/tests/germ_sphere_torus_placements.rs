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
    let tol = Tol::witness();
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let (ball0, donut0) = (
        placed(&ball(), Vec3::unit_x(), 0.0, origin),
        placed(&donut(), Vec3::unit_x(), 0.0, origin),
    );
    let ring = placed(&donut(), Vec3::unit_x(), 0.7, Vec3::new(0.1, 0.35, 0.2));
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
        (
            "a spline edge in a torus ring's run",
            &donut0,
            &moved_ball(),
        ),
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
    for (what, end, floor) in &got {
        assert_eq!(end, "builds", "{what}: {got:#?}");
        assert!(floor.is_none(), "{what}: {floor:?}");
    }
}

/// The spline edge's placement: the ball turned 1.1 rad about
/// `(0.3, 1, 0.2)` and moved to `(0.42, −0.15, 0.3)`, against the donut at
/// the origin.
fn moved_ball() -> AtRestBody<f64> {
    placed(
        &ball(),
        Vec3::new(0.3, 1.0, 0.2),
        1.1,
        Vec3::new(0.42, -0.15, 0.3),
    )
}

/// The certified enclosure at the pinned resolution, its bracket
/// carried: at ε 1e-6 a body here stops at the floor short of a met
/// number, and inclusion–exclusion reads the bracket either way.
fn bracket(what: &str, body: &AtRestBody<f64>) -> (f64, f64) {
    let tol = Tol::witness();
    let v = topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("{what}: the volume: {e:?}"))
        .enclosure;
    (v.volume, v.volume_pad)
}

/// Each op builds a body that passes tier 3′ (or leaves only pairs the
/// census cannot decide), and the ops agree on one `V(O ∩ B)` within
/// `[0, min(V(donut), V(ball))]` by inclusion–exclusion against the
/// operands' certified volumes
/// ([`crate::common::op_census::assert_one_meet`]). The pose has no
/// independent volume, so the agreement is the oracle.
fn every_op_meets_once(
    ops: &[(
        &'static str,
        crate::common::op_census::Keep,
        Result<topo::BooleanResult<f64>, BooleanError>,
    )],
    o: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) {
    use crate::common::op_census::assert_one_meet;
    use topo::{BooleanResult, ValidationError};
    let tol = Tol::witness();
    let (vo, pad_o) = bracket("the donut", o);
    let (vb, pad_b) = bracket("the ball", b);
    let mut implied = Vec::new();
    for (op, keep, got) in ops {
        let r = got
            .as_ref()
            .unwrap_or_else(|e| panic!("{op}: builds: {e:?}"));
        let BooleanResult::Body(body) = r else {
            panic!("{op}: an empty result where the operands overlap");
        };
        let (v, pad) = bracket(op, &body.body);
        implied.push((*op, keep.meet(vo, vb, v), pad));
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

/// **The spline edge's placement, under every op**: a section loop lies
/// inside the one-band ball and the one-band donut, and its cut-in leaves
/// strut rings on the donut's torus face that no path places. Each is
/// read by its pierce point in the new face (`chord_join::strut_in_run`).
///
/// - **Donut first, ∪, ∩ and ∖** — the item's stop
///   (`work/lathe/a-section-loop-inside-two-one-band-curved-faces-refuses-at-the-section-certificate.md`):
///   on main each refuses `RingHomingAmbiguous` at the torus face's strut
///   ring 6v1.
/// - **Ball first, ∪, ∩ and ∖** — a regression guard: these build on main
///   too, to the same volumes. At ε 1e-12 they run past 300 s each, on
///   main as here, so they are a row of their own, ignored
///   ([`the_spline_edge_placement_ball_first_ops`]).
///
/// All six are held to tier 3′ and to one `V(O ∩ B)` by
/// inclusion–exclusion, so the donut-first bodies are checked against the
/// ball-first ones.
#[test]
fn the_spline_edge_placement_under_every_op() {
    use crate::common::op_census::Keep;
    let tol = Tol::witness();
    let o = placed(&donut(), Vec3::unit_x(), 0.0, Vec3::new(0.0, 0.0, 0.0));
    let b = moved_ball();
    let mut ops = vec![
        ("O∪B", Keep::Union, topo::union(&o, &b, tol)),
        ("O∩B", Keep::Meet, topo::intersect(&o, &b, tol)),
        ("O∖B", Keep::OLessB, topo::subtract(&o, &b, tol)),
    ];
    if tol.eps() != 1e-12 {
        ops.extend(ball_first(&o, &b));
    }
    every_op_meets_once(&ops, &o, &b);
}

fn ball_first(
    o: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> [(
    &'static str,
    crate::common::op_census::Keep,
    Result<topo::BooleanResult<f64>, BooleanError>,
); 3] {
    use crate::common::op_census::Keep;
    let tol = Tol::witness();
    [
        ("B∪O", Keep::Union, topo::union(b, o, tol)),
        ("B∩O", Keep::Meet, topo::intersect(b, o, tol)),
        ("B∖O", Keep::BLessO, topo::subtract(b, o, tol)),
    ]
}

/// **The ball-first ops at ε 1e-12**, where
/// [`the_spline_edge_placement_under_every_op`] holds only the donut-first
/// three: each runs past 300 s, on main as here
/// (`work/lathe/the-ball-first-ops-of-the-ball-donut-spline-placement-run-for-minutes-at-1e-12.md`).
/// At the other ε it adds nothing to that row, which reads all six.
#[test]
#[ignore = "past 300 s per op at ε 1e-12, on main too: work/lathe/the-ball-first-ops-of-the-ball-donut-spline-placement-run-for-minutes-at-1e-12.md"]
fn the_spline_edge_placement_ball_first_ops() {
    let o = placed(&donut(), Vec3::unit_x(), 0.0, Vec3::new(0.0, 0.0, 0.0));
    let b = moved_ball();
    every_op_meets_once(&ball_first(&o, &b), &o, &b);
}
