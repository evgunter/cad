//! **A plane across a one-face closed wall.** A transverse plane cuts
//! such a wall — a one-segment circle's extruded cylinder, a full
//! revolve's tube — in a conic that crosses the wall's one wrap edge
//! once, so the section loop has one site and its one segment runs
//! from that site round the whole conic back to it. Every row runs ∪,
//! ∩ and both differences, each in both operand orders
//! (`differential::every_op_both_orders`), so the planar face is the
//! first operand in half the runs and the second in the rest. Each run
//! is read through `differential::outcome` (tiers 2 and 3′, the
//! certificate, the legal operand, the closed-form volume) and its
//! result meshed and `check_mesh`ed.
//!
//! - the CLEAVE tube: a full-revolved tube under a box;
//! - a slab and a blind pocket across a one-segment cylinder's wall;
//! - a tilted plane across that wall;
//! - a tilted plane passing close by the wrap edge's end vertex;
//! - the tube under a box whose side face cuts its outer wall, so one
//!   one-site loop lies beside matched segments in the same faces.
//!
//! Two classes no row here owns are pinned as known, by row: a box
//! less a tube, which leaves a rod in the bore, fails tier 3′
//! (`work/topo/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`),
//! and a result under a tilted plane is no legal operand
//! (`work/contact/at-infinity-probe-measures-in-closed-form-only.md`);
//! a two-arc wall fails both the same way.
//!
//! The one-site loops the join does not build keep
//! `Join(SingleSiteSectionLoop)`: a circle edge lying in the partner's
//! face, and a plane tangent to the wall's cap at the wrap edge's end.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::{brick, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{Body, BooleanError, SplitJoinError};

use crate::common::differential::{every_op_both_orders, outcome};

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle of radius `r` about `(cx, cy)`, its one vertex
/// at azimuth `az`, turning `sweep`.
fn circle(cx: f64, cy: f64, r: f64, az: f64, sweep: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(cx + r * az.cos(), cy + r * az.sin()),
        Segment::Arc(Arc2 {
            centre: Point2::new(cx, cy),
            radius: r,
            sweep,
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
    topo::transform_rigid(&body, &up, tol()).unwrap()
}

/// The one-segment circle of radius 1 about the z axis, its vertex at
/// `(1, 0)`, extruded over `z ∈ [z0, z0 + 2]`: one wall whose seam strut
/// is its wrap edge, on the meridian through `(1, 0, z)`. `2π`.
fn seam_cylinder(z0: f64) -> Body<f64> {
    extruded(vec![circle(0.0, 0.0, 1.0, 0.0, TAU)], z0, 2.0)
}

/// The tube `r ∈ [ri, 0.5]`, `y ∈ [−0.5, 1.5]`, revolved a full turn
/// about `y`: two walls, each with its one seam.
fn tube(ri: f64) -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved_about_y(
        vec![p(ri, -0.5), p(0.5, -0.5), p(0.5, 1.5), p(ri, 1.5)],
        Revolution::Full,
        tol(),
    )
}

/// The half-space below the plane through `at` with upward normal
/// `R·ẑ`, as a box 8 wide and 8 deep under it: every row's body lies
/// well inside its other faces.
fn below(at: Point3<f64>, turn: Affine3<f64>) -> Body<f64> {
    let raw = brick((-4.0, 4.0), (-4.0, 4.0), (-8.0, 0.0), tol());
    let to = Affine3::translation(at - Point3::origin());
    topo::transform_rigid(&raw, &(to * turn), tol()).unwrap()
}

/// What a row's run is held to besides `SOUND`: the one flag a known
/// class sets false, the rest of the line sound.
#[derive(Clone, Copy)]
enum Known {
    /// Tier 3′ refuses the result.
    T3,
    /// The result does not unite with a far brick.
    Operand,
}

/// Every op in both orders between `a` and `b` (volumes `va`, `vb`,
/// overlapping in `vab`), each read through `outcome`: `SOUND`, or for
/// a run `known` names, sound but for that class's flag; and meshed,
/// unless the result holds a window in a curved face, which the mesher
/// refuses typed (`TessellateError::RingOnCurvedFace`,
/// `work/tess/a-notched-or-ringed-cylinder-wall-does-not-tessellate.md`).
fn every_op(what: &str, a: Body<f64>, b: Body<f64>, v: (f64, f64, f64), known: &[(&str, Known)]) {
    let (a, b) = (fin("A", a), fin("B", b));
    for (op, r, want) in every_op_both_orders(&a, &b, v, tol()) {
        let row = format!("{what}: {op}");
        if let Ok(res) = &r {
            let body = &res.body().unwrap_or_else(|| panic!("{row}: empty")).body;
            match mesh::tessellate(body, 5e-3, tol()) {
                Ok(m) => {
                    mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{row}: mesh {e:?}"))
                }
                Err(mesh::TessellateError::RingOnCurvedFace { .. }) => {}
                Err(e) => panic!("{row}: {e:?}"),
            }
        }
        let line = outcome(r, want, tol());
        match known.iter().find(|(o, _)| *o == op).map(|&(_, k)| k) {
            None => assert!(line.starts_with("OK SOUND"), "{row}: {line}"),
            Some(k) => {
                let flags = ["t2=", "t3p=", "cert=", "operand="].map(|f| {
                    let named =
                        matches!((f, k), ("t3p=", Known::T3) | ("operand=", Known::Operand));
                    format!("{f}{}", !named)
                });
                assert!(line.starts_with("OK BAD"), "{row}: {line}");
                for f in &flags {
                    assert!(line.contains(f.as_str()), "{row}: want {f}: {line}");
                }
                let num = |key: &str| -> f64 {
                    let at = line.find(key).unwrap() + key.len();
                    line[at..]
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap()
                };
                assert!((num(" v=") - want).abs() < 1e-7, "{row}: volume: {line}");
            }
        }
    }
}

fn fin(what: &str, body: Body<f64>) -> topo::AtRestBody<f64> {
    topo::test_support::finished(what, body, tol())
}

/// **The CLEAVE tube**: the box `(−1.5, 1.5) × (0, 1) × (−1.5, 1.5)`
/// and the tube `r ∈ [0.3, 0.5]`. The box's faces `y = 0` and `y = 1`
/// cut each tube wall across its one seam: four one-site loops. Tube
/// `0.32π`, box 9, overlap `0.16π`.
#[test]
fn a_box_across_a_full_revolved_tube() {
    let block = brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol());
    every_op(
        "the tube under a box",
        tube(0.3),
        block,
        (0.32 * PI, 9.0, 0.16 * PI),
        &[("B ∖ A", Known::T3)],
    );
}

/// **A slab across the one-segment cylinder**: `z ∈ [0.5, 1]` over the
/// whole disc, two planes each cutting the wall in a circle through its
/// seam once. Slab 18, overlap `π/2`.
#[test]
fn a_slab_across_a_one_segment_cylinder() {
    let slab = brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol());
    every_op(
        "the slab",
        seam_cylinder(0.0),
        slab,
        (TAU, 18.0, 0.5 * PI),
        &[],
    );
}

/// **A blind pocket**: the one-segment cylinder raised to
/// `z ∈ [0.5, 2.5]` sunk into the plate `z ∈ [0, 1]`, whose top face
/// cuts the cylinder's wall across its seam; the pocket is the plate
/// less the cylinder. Plate 36, overlap `π/2`.
#[test]
fn a_blind_pocket_through_a_plates_top_face() {
    let plate = brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol());
    every_op(
        "the pocket",
        plate,
        seam_cylinder(0.5),
        (36.0, TAU, 0.5 * PI),
        &[],
    );
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
        seam_cylinder(0.0),
        cutter,
        (TAU, 512.0, PI),
        &ALL_OPS.map(|op| (op, Known::Operand)),
    );
}

/// The six runs' names, as `every_op_both_orders` gives them.
const ALL_OPS: [&str; 6] = ["A ∪ B", "B ∪ A", "A ∩ B", "B ∩ A", "A ∖ B", "B ∖ A"];

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
        every_op(
            &format!("the plane {c} above the end"),
            seam_cylinder(0.0),
            below(Point3::new(1.0, 0.0, c), turn),
            (TAU, 512.0, PI * (c + m)),
            &ALL_OPS.map(|op| (op, Known::Operand)),
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
    let block = brick((-1.5, 0.4), (0.0, 1.0), (-1.5, 1.5), tol());
    let segment = 0.25 * 0.8f64.acos() - 0.4 * 0.3;
    every_op(
        "the tube beside the box's edge",
        tube(0.3),
        block,
        (0.32 * PI, 5.7, 0.16 * PI - segment),
        &[("B ∖ A", Known::T3)],
    );
}

/// Every op in both orders refuses `Join(SingleSiteSectionLoop { count })`.
fn every_op_refuses(what: &str, a: Body<f64>, b: Body<f64>, count: usize) {
    let (a, b) = (fin("A", a), fin("B", b));
    for (op, r, _) in every_op_both_orders(&a, &b, (0.0, 0.0, 0.0), tol()) {
        assert!(
            matches!(
                r,
                Err(BooleanError::Join(SplitJoinError::SingleSiteSectionLoop { count: c })) if c == count
            ),
            "{what}: {op}: {:?}",
            r.map(|_| "a body")
        );
    }
}

/// **A circle edge lying in the partner's face keeps the refusal.** A
/// bored tube whose outer wall turns from the cylinder `ρ = 0.5`
/// (`y ≤ 0`) to a sphere at the circle `y = 0`, an edge whose one vertex
/// lies in the box face `y = 0`: a coincidence the arm does not take,
/// since the circle edge is at its site. The inner wall `ρ = 0.3`
/// crosses the same face across its seam, which builds, so one loop
/// refuses. Either side of the face.
#[test]
fn a_circle_edge_in_the_partners_face_refuses() {
    // The sphere about (0, −0.2) through (0.5, 0) meets the bore ρ = 0.3
    // at y = top.
    let top = (0.29f64 - 0.09).sqrt() - 0.2;
    let (a0, a1) = (0.2f64.atan2(0.5), (top + 0.2).atan2(0.3));
    let tube = revolved_about_y(
        vec![
            (Point2::new(0.3, -0.5), 0.0),
            (Point2::new(0.5, -0.5), 0.0),
            (Point2::new(0.5, 0.0), ((a1 - a0) / 4.0).tan()),
            (Point2::new(0.3, top), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    for y in [(0.0, 2.0), (-2.0, 0.0)] {
        let block = brick((-1.5, 1.5), y, (-1.5, 1.5), tol());
        every_op_refuses(
            &format!("the in-face circle, box y = {y:?}"),
            tube.clone(),
            block,
            1,
        );
    }
}

/// **A plane tangent to the cap at the wrap edge's end keeps the
/// refusal**: `z = m(1 − x)` touches the bottom rim at its one vertex
/// `(1, 0, 0)`, so its section's one site is that vertex, where the
/// rim, not the wrap edge alone, meets the plane.
#[test]
fn a_plane_tangent_at_the_wrap_edges_end_refuses() {
    let m: f64 = 0.5;
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), m.atan());
    every_op_refuses(
        "the plane tangent at the end vertex",
        seam_cylinder(0.0),
        below(Point3::new(1.0, 0.0, 0.0), turn),
        1,
    );
}
