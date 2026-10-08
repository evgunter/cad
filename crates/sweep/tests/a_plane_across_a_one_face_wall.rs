//! **A plane across a one-face closed wall.** A transverse plane cuts
//! such a wall — a one-segment circle's extruded cylinder, a full
//! revolve's tube — in a conic that crosses the wall's one wrap edge
//! once, so the section loop has one site and its one segment runs
//! from that site round the whole conic back to it. Every row runs ∪,
//! ∩ and both differences, each in both operand orders, and holds the
//! result to all three tiers and to its closed-form volume.
//!
//! - the CLEAVE tube: a full-revolved tube under a box;
//! - a slab and a blind pocket across a one-segment cylinder's wall;
//! - a tilted plane across that wall;
//! - a tilted plane passing close by the wrap edge's end vertex;
//! - the tube under a box whose side face cuts its outer wall, so one
//!   one-site loop lies beside matched segments in the same faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::{brick, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, Body, validate, validate_closed, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle of radius 1 about the z axis, its one vertex at
/// `(1, 0)`, extruded from `z = 0` to `z = 2`: one wall whose seam strut
/// is its wrap edge, on the meridian through `(1, 0, z)`. `πr²h = 2π`.
fn seam_cylinder() -> Body<f64> {
    let circle: ProfileLoop<f64> = RawLoop::new([(
        Point2::new(1.0, 0.0),
        Segment::Arc(Arc2 {
            centre: Point2::new(0.0, 0.0),
            radius: 1.0,
            sweep: TAU,
        }),
    )]);
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![circle])
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: 2.0,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, depth, tol()).unwrap().body
}

/// The half-space below the plane through `at` with upward normal
/// `R·ẑ`, as a box 8 wide and 8 deep under it: every row's body lies
/// well inside its other faces.
fn below(at: Point3<f64>, turn: Affine3<f64>) -> Body<f64> {
    let raw = brick((-4.0, 4.0), (-4.0, 4.0), (-8.0, 0.0), tol());
    let to = Affine3::translation(at - Point3::origin());
    topo::transform_rigid(&raw, &(to * turn), tol()).unwrap()
}

fn finished(what: &str, body: Body<f64>) -> AtRestBody<f64> {
    topo::test_support::finished(what, body, tol())
}

/// Every op in both orders, each held to its closed form: `a` and `b`
/// of volumes `va` and `vb`, overlapping in `vab`.
fn every_op(what: &str, a: Body<f64>, b: Body<f64>, (va, vb, vab): (f64, f64, f64)) {
    let (a, b) = (finished("A", a), finished("B", b));
    let rows: [(&str, &AtRestBody<f64>, &AtRestBody<f64>, f64); 6] = [
        ("A ∪ B", &a, &b, va + vb - vab),
        ("B ∪ A", &b, &a, va + vb - vab),
        ("A ∩ B", &a, &b, vab),
        ("B ∩ A", &b, &a, vab),
        ("A ∖ B", &a, &b, va - vab),
        ("B ∖ A", &b, &a, vb - vab),
    ];
    for (op, x, y, want) in rows {
        let row = format!("{what}: {op}");
        let got = match &op[2..5] {
            "∪" => topo::union(x, y, tol()),
            "∩" => topo::intersect(x, y, tol()),
            _ => topo::subtract(x, y, tol()),
        }
        .unwrap_or_else(|e| panic!("{row}: {e:?}"));
        let body = &got
            .body()
            .unwrap_or_else(|| panic!("{row}: the result is empty"))
            .body;
        assert_eq!(validate(body), Ok(()), "{row}: tier 1");
        assert_eq!(validate_closed(body), Ok(()), "{row}: tier 2");
        assert_eq!(validate_geometric(body, tol()), Ok(()), "{row}: tier 3");
        let m = topo::mass_properties(body, tol()).unwrap();
        assert!(
            (m.volume - want).abs() <= m.volume_pad + 1e-9 * want.max(1.0),
            "{row}: volume {} ± {}, closed form {want}",
            m.volume,
            m.volume_pad
        );
    }
}

/// **The CLEAVE tube**: the box `(−1.5, 1.5) × (0, 1) × (−1.5, 1.5)`
/// and the tube revolved a full turn about `y` from the rectangle
/// `r ∈ [0.3, 0.5]`, `y ∈ [−0.5, 1.5]`. The box's faces `y = 0` and
/// `y = 1` cut each tube wall across its one seam: four one-site loops.
/// Tube `0.32π`, box 9, overlap `0.16π`.
#[test]
fn a_box_across_a_full_revolved_tube() {
    let p = |x, y| (Point2::new(x, y), 0.0);
    let tube = revolved_about_y(
        vec![p(0.3, -0.5), p(0.5, -0.5), p(0.5, 1.5), p(0.3, 1.5)],
        Revolution::Full,
        tol(),
    );
    let block = brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol());
    every_op(
        "the tube under a box",
        tube,
        block,
        (0.32 * PI, 9.0, 0.16 * PI),
    );
}

/// **A slab across the one-segment cylinder**: `z ∈ [0.5, 1]` over the
/// whole disc, two planes each cutting the wall in a circle through its
/// seam once. Slab 18, overlap `π/2`.
#[test]
fn a_slab_across_a_one_segment_cylinder() {
    let slab = brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol());
    every_op("the slab", seam_cylinder(), slab, (TAU, 18.0, 0.5 * PI));
}

/// **A blind pocket**: the one-segment cylinder raised to
/// `z ∈ [0.5, 2.5]` sunk into the plate `z ∈ [0, 1]`, whose top face
/// cuts the cylinder's wall across its seam; the pocket is the plate
/// less the cylinder. Plate 36, overlap `π/2`.
#[test]
fn a_blind_pocket_through_a_plates_top_face() {
    let up = Affine3::translation(Vec3::new(0.0, 0.0, 0.5));
    let tool = topo::transform_rigid(&seam_cylinder(), &up, tol()).unwrap();
    let plate = brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol());
    every_op("the pocket", plate, tool, (36.0, TAU, 0.5 * PI));
}

/// **A tilted plane**: `z = 1 + y·tan 20°`, turned about `x`, an ellipse
/// through the seam at `z = 1`. Below it the disc holds
/// `∫(1 + y·tan 20°) dA = π`; the box is 512.
#[test]
fn a_tilted_plane_across_a_one_segment_cylinder() {
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        20f64.to_radians(),
    );
    let cutter = below(Point3::new(0.0, 0.0, 1.0), turn);
    every_op(
        "the tilted plane",
        seam_cylinder(),
        cutter,
        (TAU, 512.0, PI),
    );
}

/// **A plane close by the wrap edge's end**: `z = c + m(1 − x)`, turned
/// about `y`, meets the seam at `z = c`, just above the bottom cap's
/// one vertex `(1, 0, 0)`, and rises to `c + 2m` across the disc. Below
/// it the disc holds `π(c + m)`.
#[test]
fn a_plane_close_by_the_wrap_edges_end() {
    let m: f64 = 0.5;
    for c in [0.02, 1e-3] {
        let turn =
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), m.atan());
        let cutter = below(Point3::new(1.0, 0.0, c), turn);
        every_op(
            &format!("the plane {c} above the end"),
            seam_cylinder(),
            cutter,
            (TAU, 512.0, PI * (c + m)),
        );
    }
}

/// **A one-site loop beside matched ones**: the CLEAVE tube under the box
/// `(−1.5, 0.4) × (0, 1) × (−1.5, 1.5)`. Its face `x = 0.4` misses the
/// inner wall and cuts the outer one along two rulings, so in the faces
/// `y = 0` and `y = 1` the inner wall's whole circle, through its seam,
/// lies beside the outer wall's arc, whose ends are on the box's edges.
/// Box 5.7; overlap the annulus `r ∈ [0.3, 0.5]` less the outer disc's
/// segment beyond `x = 0.4`, `0.25·acos 0.8 − 0.4·0.3`.
#[test]
fn a_one_site_loop_beside_matched_ones() {
    let p = |x, y| (Point2::new(x, y), 0.0);
    let tube = revolved_about_y(
        vec![p(0.3, -0.5), p(0.5, -0.5), p(0.5, 1.5), p(0.3, 1.5)],
        Revolution::Full,
        tol(),
    );
    let block = brick((-1.5, 0.4), (0.0, 1.0), (-1.5, 1.5), tol());
    let segment = 0.25 * 0.8f64.acos() - 0.4 * 0.3;
    every_op(
        "the tube beside the box's edge",
        tube,
        block,
        (0.32 * PI, 5.7, 0.16 * PI - segment),
    );
}
