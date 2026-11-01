//! **A wrap edge's tip reads its face's full turn in the Boolean.** A
//! face that reaches its chart singularity by one wrap edge, walked out
//! and back (DESIGN `[topology]`'s seam clause), cut by boxes whose faces
//! pass through the tip: the half-spaces bounded by the planes `x = 0`
//! (square to the wrap edge's plane) and `z = 0` (holding the wrap edge),
//! and the four quarters. Two bodies take them:
//!
//! - the unit hemisphere whose pole is a tip, its wrap edge from the pole
//!   to the base rim;
//! - the unit ball whose one wrap edge runs pole to pole, both ends tips,
//!   the whole edge lying in the plane `z = 0`.
//!
//! At a tip the face's one corner is a full turn round the one edge, as a
//! slit's tip is (`topo::sector_shape::SlitTips`, the tips
//! `validate::wrap_tips` reads), and a germ along the edge faces the
//! strut by the run's order (`boolean::insert`'s `strut_facing`). Where
//! the plane holds the whole wrap edge, the ball's face is cut into two
//! hemispheres whose every edge lies on the plane, and the join reads
//! their sides at the seam (`boolean::join`'s `seam_germ_side`). Each op,
//! either way round, holds its closed form, passes tiers 3 and 3′ and
//! places a lattice of points as the analytic region does.
//!
//! (A plane through the pole tilted off the axis cuts the sphere in a
//! circle through the pole, which mints no pcurve with or without the
//! tip: `work/charttail/a-section-circle-through-a-sphere-faces-pole-mints-no-pcurve.md`.)
//!
//! The cone of the same height, its apex reached by one wrap edge, splits
//! through the apex at its closed forms; its Boolean through the apex
//! refuses `NormalAtConeApex`, the Boolean's apex sector algebra not yet
//! built.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region, ball, cuboid};
use crate::common::wrap_tip::{cone, hemisphere, one_wrap_edge};
use core::f64::consts::PI;
use geom_brep::EdgeDescriptionSpec;
use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::construction::Construct;
use topo::{AtRestBody, BooleanResult, SplitPart, intersect, split, subtract};

fn tol() -> Tol {
    Tol::witness()
}

/// The hemisphere's volume.
const HEMI: f64 = 2.0 * PI / 3.0;

/// The ball's volume.
const BALL: f64 = 4.0 * PI / 3.0;

fn the_ball() -> Region {
    ball(Point3::origin(), 1.0)
}

fn the_hemisphere() -> Region {
    let b = ball(Point3::origin(), 1.0);
    Box::new(move |p: Point3<f64>| b(p).min(p.y))
}

/// The lattice the rows place points on, and its density for the full
/// rows and the fast one.
const LATTICE: (Point3<f64>, Point3<f64>) = (
    Point3::new(-1.15, -1.15, -1.15),
    Point3::new(1.15, 1.15, 1.15),
);
const FULL: usize = 9;
const FAST: usize = 5;

/// [`membership::holds`] over [`LATTICE`].
fn holds(
    what: &str,
    b: &AtRestBody<f64>,
    want: f64,
    region: &dyn Fn(Point3<f64>) -> f64,
    n: usize,
) {
    membership::holds(what, b, want, region, LATTICE, n);
}

fn body_of(r: Result<BooleanResult<f64>, topo::BooleanError>, what: &str) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{what}: {other:?}"),
    }
}

/// The hemisphere with its pole a wrap tip.
fn tipped_hemisphere() -> AtRestBody<f64> {
    finished(
        "the tipped hemisphere",
        one_wrap_edge(hemisphere(), true).0,
        tol(),
    )
}

/// The unit ball whose one wrap edge runs pole to pole: the full
/// revolve's two meridians made one by `kef` at the north pole.
fn tipped_ball() -> AtRestBody<f64> {
    let t = (PI / 8.0).tan();
    let mut body = revolved_about_y(
        vec![
            (Point2::new(0.0, -1.0), t),
            (Point2::new(1.0, 0.0), t),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let north = crate::common::wrap_tip::vertex_at(&body, crate::common::wrap_tip::NORTH);
    let out = crate::common::wrap_tip::leaving(&body, north);
    let wraps = body.get_half_edge(out[0]).unwrap().edge;
    let surface = body
        .get_face(body.face_of_half_edge(out[0]).unwrap())
        .unwrap()
        .surface;
    body.kef_describing(
        out[1],
        &[(wraps, EdgeDescriptionSpec::wrap(surface))],
        tol(),
    )
    .expect("the survivor restates as the ball's wrap edge");
    finished("the tipped ball", body, tol())
}

/// A body, its region and its volume.
struct Solid {
    what: &'static str,
    body: AtRestBody<f64>,
    region: fn() -> Region,
    volume: f64,
}

fn tipped() -> [Solid; 2] {
    [
        Solid {
            what: "the tipped hemisphere",
            body: tipped_hemisphere(),
            region: the_hemisphere,
            volume: HEMI,
        },
        Solid {
            what: "the tipped ball",
            body: tipped_ball(),
            region: the_ball,
            volume: BALL,
        },
    ]
}

/// Holds `body ∩ tool`, `tool ∩ body`, `body ∖ tool` and `tool ∖ body`,
/// the tool the box over `x × [−2, 2] × z`, to their closed forms and
/// regions, `share` the part of `solid` the box holds.
fn cuts(solid: &Solid, what: &str, (x, z): ((f64, f64), (f64, f64)), share: f64) {
    let y = (-2.0, 2.0);
    let tool = finished("a box", topo::test_support::brick(x, y, z, tol()), tol());
    let size = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
    let inside = share * solid.volume;
    let (r, b) = (solid.region, move || cuboid(x, y, z));
    for (op, result, want, region) in [
        (
            "body ∩ tool",
            intersect(&solid.body, &tool, tol()),
            inside,
            Box::new(move |p| r()(p).min(b()(p))) as Region,
        ),
        (
            "tool ∩ body",
            intersect(&tool, &solid.body, tol()),
            inside,
            Box::new(move |p| r()(p).min(b()(p))),
        ),
        (
            "body ∖ tool",
            subtract(&solid.body, &tool, tol()),
            solid.volume - inside,
            Box::new(move |p| r()(p).min(-b()(p))),
        ),
        (
            "tool ∖ body",
            subtract(&tool, &solid.body, tol()),
            size - inside,
            Box::new(move |p| b()(p).min(-r()(p))),
        ),
    ] {
        let what = format!("{}, {what}, {op}", solid.what);
        holds(&what, &body_of(result, &what), want, &region, FULL);
    }
}

/// **Each body is at rest at its closed form.**
#[test]
fn the_tipped_bodies_are_at_rest() {
    for s in tipped() {
        holds(s.what, &s.body, s.volume, &(s.region)(), FULL);
    }
}

/// **The fast row**: each body less the half-space square to the wrap
/// edge's plane (the tip's corner a full turn), and less the one holding
/// the wrap edge (a germ along the tip's edge).
#[test]
fn a_tipped_body_less_a_half_space_through_the_tip() {
    for s in tipped() {
        for (what, x, z) in [
            ("x > 0", (0.0, 2.0), (-2.0, 2.0)),
            ("z > 0", (-2.0, 2.0), (0.0, 2.0)),
        ] {
            let what = format!("{} less {what}", s.what);
            let tool = finished(
                "a box",
                topo::test_support::brick(x, (-2.0, 2.0), z, tol()),
                tol(),
            );
            let b = (x, z);
            holds(
                &what,
                &body_of(subtract(&s.body, &tool, tol()), &what),
                s.volume / 2.0,
                &move |p| (s.region)()(p).min(-cuboid(b.0, (-2.0, 2.0), b.1)(p)),
                FAST,
            );
        }
    }
}

/// **A half-space through the tip**, either side of either plane.
#[test]
fn a_half_space_through_the_tip_cuts_each_tipped_body() {
    for s in tipped() {
        for (what, x, z) in [
            ("x > 0", (0.0, 2.0), (-2.0, 2.0)),
            ("x < 0", (-2.0, 0.0), (-2.0, 2.0)),
            ("z > 0", (-2.0, 2.0), (0.0, 2.0)),
            ("z < 0", (-2.0, 2.0), (-2.0, 0.0)),
        ] {
            cuts(&s, what, (x, z), 0.5);
        }
    }
}

/// **A quarter through the tip**, each of the four.
#[test]
fn a_quarter_through_the_tip_cuts_each_tipped_body() {
    for s in tipped() {
        for (x, z) in [
            ((0.0, 2.0), (0.0, 2.0)),
            ((-2.0, 0.0), (-2.0, 0.0)),
            ((0.0, 2.0), (-2.0, 0.0)),
            ((-2.0, 0.0), (0.0, 2.0)),
        ] {
            cuts(
                &s,
                &format!("the quarter x ∈ {x:?}, z ∈ {z:?}"),
                (x, z),
                0.25,
            );
        }
    }
}

/// **The cone's apex reached by one wrap edge splits** through planes
/// holding the axis, each side half the cone; its Boolean through the
/// apex refuses by name.
#[test]
fn the_tipped_cones_apex_splits_and_the_boolean_refuses_it_by_name() {
    let body = finished("the tipped cone", one_wrap_edge(cone(), true).0, tol());
    for n in [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)] {
        let what = format!("the split along {n:?}");
        let r = split(
            &body,
            &topo::test_support::split_plane(Point3::origin(), n, tol()),
            tol(),
        )
        .unwrap_or_else(|e| panic!("{what}: {e:?}"));
        for (side, part) in [("above", &r.above), ("below", &r.below)] {
            let SplitPart::Body(b) = part else {
                panic!("{what}: nothing {side}");
            };
            let x = if n.x > 0.0 { 1.0 } else { 0.0 };
            let up = if side == "above" { 1.0 } else { -1.0 };
            holds(
                &format!("{what}, {side}"),
                b,
                PI / 6.0,
                &move |p: Point3<f64>| {
                    let cone = (1.0 - p.y - (p.x * p.x + p.z * p.z).sqrt()).min(p.y);
                    cone.min(up * if x > 0.0 { p.x } else { p.z })
                },
                FULL,
            );
        }
    }
    let half = finished(
        "a box",
        topo::test_support::brick((0.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol()),
        tol(),
    );
    match subtract(&body, &half, tol()) {
        Err(topo::BooleanError::NormalAtConeApex { .. }) => {}
        other => panic!("the tipped cone less x > 0: {other:?}"),
    }
}
