//! **A closed conic lying in the partner's face.** A one-segment circle
//! is one edge with one vertex, both its ends. Where such an edge of one
//! operand lies in a face of the other, the section loop is that edge,
//! and its one site is the edge's vertex: one pair record carries both
//! of the loop's germs, one each way round it. The join takes it as one
//! segment (`boolean::join`'s `one_site_segments`): the edge's own side
//! copies it whole, and the partner's face is pierced at the site and
//! joined there as a ring of its own. Where the partner holds the circle
//! along a closed edge of its own through the same vertex (two coaxial
//! one-segment circles phased alike), both sides copy their edges.
//!
//! The rows, each a one-segment circle built through the raw loop door:
//!
//! - a boss standing on a plate's top face, its vertex at four azimuths,
//!   and one hanging under its bottom face off the axis;
//! - a second boss on a plate whose top face already holds a boss's
//!   ring, and a boss over a hole whose ring lies inside its rim;
//! - two equal coaxial cylinders stacked end to end;
//! - a tube whose rim lies inside one face of a ball;
//! - a puck and the quarter-disc ring revolved round it.
//!
//! Every row runs ∪, ∩ and both differences in both operand orders, and
//! each run is read through `differential::outcome` (tiers 2 and 3′, the
//! certificate, the legal operand, the closed-form volume) and meshed.
//! The ∩ of two solids meeting only in a face is empty. The coaxial
//! stack and the puck run their unions alone: their ∩ and differences
//! refuse `FallbackExtentUnsupported` at the coincident walls, as the
//! two-arc forms do
//! (`work/reachhold/rounded-stack-subtract-and-intersect-refuse-fallback-extent.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2, TAU};

use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::{ball_poled_z, brick, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

use crate::common::differential::{every_op_both_orders, outcome};
use crate::common::oracles::{ball_volume, cap_volume};

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle of radius `r` about `(cx, cy)`, its one vertex
/// at azimuth `az`.
fn circle(cx: f64, cy: f64, r: f64, az: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(cx + r * az.cos(), cy + r * az.sin()),
        Segment::Arc(Arc2 {
            centre: Point2::new(cx, cy),
            radius: r,
            sweep: TAU,
        }),
    )])
}

/// `loops` extruded from `z = z0` up by `h`.
fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0));
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&up, tol()), tol()).unwrap()
}

fn fin(what: &str, body: Body<f64>) -> AtRestBody<f64> {
    topo::test_support::finished(what, body, tol())
}

/// The plate `[−1, 1]² × [−0.5, 0]`: 2.
fn plate() -> AtRestBody<f64> {
    fin(
        "the plate",
        brick((-1.0, 1.0), (-1.0, 1.0), (-0.5, 0.0), tol()),
    )
}

/// One run's verdict: `SOUND` at its closed form, or an empty result
/// where the closed form is empty; a body is meshed, unless it holds a
/// window in a curved face, which the mesher refuses typed
/// (`TessellateError::RingOnCurvedFace`).
fn holds(row: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    if let Ok(BooleanResult::Body(b)) = &r {
        match mesh::tessellate(&b.body, 5e-3, tol()) {
            Ok(m) => mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{row}: mesh {e:?}")),
            Err(mesh::TessellateError::RingOnCurvedFace { .. }) => {}
            Err(e) => panic!("{row}: {e:?}"),
        }
    }
    let line = outcome(r, want, tol());
    let ok = if want == 0.0 {
        line.starts_with("EMPTY ok")
    } else {
        line.starts_with("OK SOUND")
    };
    assert!(ok, "{row}: {line}");
}

/// Every op in both orders between `a` and `b` (volumes `va`, `vb`,
/// overlapping in `vab`), each held to its closed form ([`holds`]).
fn every_op(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, v: (f64, f64, f64)) {
    for (op, r, want) in every_op_both_orders(a, b, v, tol()) {
        holds(&format!("{what}: {op}"), r, want);
    }
}

/// Both unions of `a` and `b`, which meet in no volume, held to `want`.
fn both_unions(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: f64) {
    for (op, r) in [
        ("A ∪ B", topo::union(a, b, tol())),
        ("B ∪ A", topo::union(b, a, tol())),
    ] {
        holds(&format!("{what}: {op}"), r, want);
    }
}

/// **A boss on a plate**: the cylinder `r = 0.3`, `z ∈ [0, 1]`, standing
/// on the top face, its rim's vertex at four azimuths; and the same
/// cylinder hung under the bottom face about `(0.2, 0.1)`. Boss `0.09π`,
/// plate 2, overlap 0.
#[test]
fn a_boss_on_a_plate() {
    let plate = plate();
    for az in [0.0, 1.0, PI, -2.0] {
        let boss = fin(
            "the boss",
            extruded(vec![circle(0.0, 0.0, 0.3, az)], 0.0, 1.0),
        );
        every_op(
            &format!("a boss, vertex at {az}"),
            &boss,
            &plate,
            (0.09 * PI, 2.0, 0.0),
        );
    }
    let under = fin(
        "the boss",
        extruded(vec![circle(0.2, 0.1, 0.3, 0.5)], -1.5, 1.0),
    );
    every_op("a boss under", &under, &plate, (0.09 * PI, 2.0, 0.0));
}

/// **The pierced face's other rings**: a second boss on a plate whose
/// top face holds the first boss's ring, outside the new rim; and a boss
/// over a hole `r = 0.1`, whose ring lies inside the rim and goes to the
/// disc the rim cuts out.
#[test]
fn a_boss_beside_a_ring_and_over_a_hole() {
    let first = fin(
        "the first boss",
        extruded(vec![circle(-0.5, 0.0, 0.3, 0.0)], 0.0, 1.0),
    );
    let second = fin(
        "the second boss",
        extruded(vec![circle(0.5, 0.0, 0.3, 1.0)], 0.0, 1.0),
    );
    let one = topo::union(&plate(), &first, tol())
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    every_op(
        "the second boss",
        &second,
        &one,
        (0.09 * PI, 2.0 + 0.09 * PI, 0.0),
    );
    let square = RawLoop::polygon([
        Point2::new(-1.0, -1.0),
        Point2::new(1.0, -1.0),
        Point2::new(1.0, 1.0),
        Point2::new(-1.0, 1.0),
    ]);
    let holed = fin(
        "the holed plate",
        extruded(vec![square, circle(0.0, 0.0, 0.1, 0.0)], -0.5, 0.5),
    );
    let boss = fin(
        "the boss",
        extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0),
    );
    every_op(
        "a boss over a hole",
        &boss,
        &holed,
        (0.09 * PI, 2.0 - 0.005 * PI, 0.0),
    );
}

/// **Equal coaxial cylinders stacked end to end**: `r = 1` over
/// `z ∈ [0, 1]` and `[1, 2]`, both vertices at azimuth 0, so the rims
/// are one closed edge of each through one site; and with the upper
/// vertex turned 2 rad, where each vertex lies inside the other's rim
/// and the loop has two sites. `2π`.
#[test]
fn coaxial_equal_cylinders_unite() {
    let lo = fin(
        "the lower",
        extruded(vec![circle(0.0, 0.0, 1.0, 0.0)], 0.0, 1.0),
    );
    for az in [0.0, 2.0] {
        let hi = fin(
            "the upper",
            extruded(vec![circle(0.0, 0.0, 1.0, az)], 1.0, 1.0),
        );
        both_unions(&format!("the stack, upper vertex at {az}"), &lo, &hi, TAU);
    }
}

/// **A tube's rim inside one face of a ball**: the tube `r = 1` rising
/// two from where its rim lies on the ball `√2` about the origin, tilted
/// 1.2 rad about `x` so its rim crosses neither of the ball's seam
/// meridians, and turned besides. The ball keeps of the tube the cap
/// above the rim's plane. Read without a mesh: a sphere face bounded by
/// a circle off its chart's latitudes has no tessellation lane
/// (`work/tess/sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane.md`).
#[test]
fn a_rim_inside_a_ball_face() {
    let z0 = 1.0f64;
    let ball = fin("the ball", ball_poled_z(SQRT_2, Vec3::zero(), tol()));
    let o = Point3::origin();
    for turn in [0.0, 0.4] {
        let place = Affine3::rotation_about_axis(o, Vec3::unit_z(), turn)
            * Affine3::rotation_about_axis(o, Vec3::unit_x(), 1.2);
        let tube = topo::transform_rigid(
            &extruded(vec![circle(0.0, 0.0, 1.0, 0.0)], z0, 2.0),
            &geom_core::test_support::rigid(&place, tol()),
            tol(),
        )
        .unwrap();
        let tube = fin("the tube", tube);
        let v = (
            2.0 * PI,
            ball_volume(SQRT_2),
            cap_volume(SQRT_2, SQRT_2 - z0),
        );
        for (op, r, want) in every_op_both_orders(&tube, &ball, v, tol()) {
            let line = outcome(r, want, tol());
            assert!(line.starts_with("OK SOUND"), "turned {turn}, {op}: {line}");
        }
    }
}

/// **A puck and its rounding ring unite**: the puck `r = 1`, `z ∈ [0,
/// 0.25]`, and the quarter disc of radius 0.25 revolved round it, whose
/// torus meets the puck's top along its top rim, whose inner wall is the
/// puck's and whose floor continues the puck's. Both of the puck's rims
/// are edges of the ring through the puck's vertices: the puck's
/// volume and the ring's, by Pappus.
#[test]
fn a_puck_and_its_rounding_ring_unite() {
    let (a, r) = (1.0_f64, 0.25_f64);
    let q = (FRAC_PI_2 / 4.0).tan();
    let ring = revolved_about_y(
        vec![
            (Point2::new(a, 0.0), 0.0),
            (Point2::new(a + r, 0.0), q),
            (Point2::new(a, r), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), FRAC_PI_2);
    let mut ring =
        topo::transform_rigid(&ring, &geom_core::test_support::rigid(&turn, tol()), tol()).unwrap();
    ring.merge_coplanar_faces(tol()).unwrap();
    let ring = fin("the ring", ring);
    let puck = fin("the puck", extruded(vec![circle(0.0, 0.0, a, 0.0)], 0.0, r));
    let quarter = PI * r * r / 4.0 * TAU * (a + 4.0 * r / (3.0 * PI));
    both_unions("the rounded puck", &puck, &ring, PI * a * a * r + quarter);
}
