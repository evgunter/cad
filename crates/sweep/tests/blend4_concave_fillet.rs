//! **BLEND-4 — the convexity-parametric fillet corner** (the concave
//! rolling ball, issue 644).
//!
//! The measurement rows came first, committed before anything moved:
//! the plan's precondition was that `corner_ball`'s unexercised
//! concave arm be verified — measured, not assumed — before anything
//! built on it, and that the convex-hardcoded consumers (the feet's
//! sign, the stored chart) be pinned as found. The arm measured
//! correct; the consumers measured convex-hardcoded; the carve rows
//! below are the widening built on that record.
//!
//! The fixture is `common::cavity`'s vented cavity — the cube's
//! mirror: a block whose rectangular cavity has twelve concave edges
//! meeting at eight all-concave trihedra, vented by a round chimney so
//! the body stays one shell. The concave-chamfer suite and both review
//! probe suites carve THE SAME body, from that one home.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{cavity_edges, vented_cavity};
use crate::common::oracles::rounded_box_volume;
use geom::Surface;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::blend::arms::corner_ball;
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, Convexity, CornerConfig, FILLET3_CORNER_RECOURSE};
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, validate, validate_closed};

/// The fillet radius, meters.
const R: f64 = 0.25;

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// **MEASUREMENT 1 — the concave arm rests the ball in the void.**
///
/// A concave trihedron's outward normals point away from the material,
/// into the wedge of void the three walls enclose. The rolling ball at
/// rest there is at distance `r` from every wall ON THE VOID SIDE:
/// `(c − p_i)·n_i = +r`, where the convex rest has `−r`. The concave
/// arm (`signed = +radius`) is written and, until this unit, called by
/// nobody; this row is the verification the plan demands before any
/// consumer is built over it.
///
/// Orthonormal case, exact: walls through the origin with outward
/// normals `+x`, `+y`, `+z` (the mirror of the box corner whose convex
/// rest is pinned in the M5 blend rows) put the centre at `(r, r, r)` —
/// in the void — with the same independence `|det| = 1` the convex arm
/// reports, since the determinant never reads the side.
#[test]
fn the_concave_arm_rests_the_ball_in_the_void_at_depth_r() {
    let r = 0.15;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)];
    let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, Convexity::Concave);
    assert!(
        (ball.center - p(r, r, r)).norm() < 1e-15,
        "the concave rest is at (r, r, r), got {:?}",
        ball.center
    );
    assert!((ball.independence - 1.0).abs() < 1e-15);
    for n in normals {
        let depth = (ball.center - p(0.0, 0.0, 0.0)).dot(n);
        assert!(
            (depth - r).abs() < 1e-15,
            "distance r on the VOID side of every wall, got {depth}"
        );
    }
}

/// **MEASUREMENT 1b — the concave rest holds at an oblique trihedron.**
///
/// The Cramer solve is one expression for both sides, so the property
/// worth measuring is the defining tangency itself, off the orthonormal
/// special case: for independent but non-orthogonal walls the centre
/// still satisfies `(c − p_i)·n_i = +r` for all three, and the reported
/// independence is the same `|det|` the convex solve reports.
#[test]
fn the_concave_rest_holds_at_an_oblique_trihedron() {
    let r = 0.2;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.6, 0.0, 0.8)];
    let verts = [p(0.0, 0.0, 0.0); 3];
    let concave = corner_ball(verts, normals, r, Convexity::Concave);
    for (i, n) in normals.iter().enumerate() {
        let depth = (concave.center - verts[i]).dot(*n);
        assert!(
            (depth - r).abs() < 1e-15,
            "wall {i}: the ball rests at +r in the void, got {depth}"
        );
    }
    assert!((concave.independence - 0.8).abs() < 1e-15);
    let convex = corner_ball(verts, normals, r, Convexity::Convex);
    assert!(
        (convex.independence - concave.independence).abs() < 1e-15,
        "independence is side-blind"
    );
}

/// **MEASUREMENT 2 — the convex feet formula does not survive the
/// concave centre.** The surgery's corner plan derives each foot as
/// `centre + n·r`, which lands ON the support exactly when the centre
/// is at depth `r` INSIDE it (`(c − p)·n = −r`). Under the concave
/// rest the same expression lands `2r` off the wall — on the far side
/// of the void — while `centre − n·r` is the tangency point. This is
/// the measured statement of issue 644's one-change shape: the ball's
/// side and the feet's sign are the same decision, and deriving one
/// without the other builds a corner less coherent than either side
/// alone.
#[test]
fn the_convex_feet_formula_is_two_r_off_the_wall_under_the_concave_rest() {
    let r = 0.15;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)];
    let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, Convexity::Concave);
    for n in normals {
        let convex_formula = ball.center + n * r;
        let off = (convex_formula - p(0.0, 0.0, 0.0)).dot(n);
        assert!(
            (off - 2.0 * r).abs() < 1e-15,
            "the convex-signed foot floats 2r into the void, got {off}"
        );
        let concave_foot = ball.center - n * r;
        let on = (concave_foot - p(0.0, 0.0, 0.0)).dot(n);
        assert!(
            on.abs() < 1e-15,
            "the mirrored sign is the tangency point, got {on}"
        );
    }
}

/// **MEASUREMENT 3, closed — the stored chart follows the side.** As
/// measured at the unit's opening (this row's first committed form),
/// the surface `corner_ball` carried aimed its pole along `+Σn` under
/// EITHER arm — toward the CONVEX patch's centre, away from the
/// concave patch (whose feet lie along `−n_i`, its centre along
/// `−Σn`). The unit folds the side into the stored chart, and this
/// row now pins the closed state: each ball's stored pole aims at its
/// own side's patch centre. ("Patch centre", not "apex": the point of
/// the patch farthest from its three boundary circles, which is not a
/// foot — the production chart's pole, `octant_chart`'s, is the third
/// FOOT up to sign, a different point with its own pins.)
#[test]
fn the_stored_chart_pole_aims_at_each_sides_own_patch_centre() {
    let r = 0.15;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)];
    let mean = (normals[0] + normals[1] + normals[2]).normalize();
    for (convex, patch_centre) in [(Convexity::Convex, mean), (Convexity::Concave, -mean)] {
        let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, convex);
        let Surface::Sphere { axis, .. } = ball.surface else {
            panic!("the corner ball's surface is a sphere");
        };
        let aim = axis.dot(patch_centre);
        assert!(
            (aim - 1.0).abs() < 1e-15,
            "the stored pole aims at this side's patch centre (convex {convex}, dot {aim})"
        );
    }
}

/// **The volume THIS FIXTURE's filleted void encloses at radius
/// [`R`]** — deliberately argument-free, because every other number
/// in it (block `4³`, cavity side `2`, the vent's radius and reach)
/// is the fixture's own; a half-parametric form with the block
/// hardcoded would read as more general than it is. A concave fillet
/// on every edge and corner of the cavity leaves exactly the void the
/// ball can sweep: the Minkowski sum of the shrunk box (side
/// `2 − 2R`) with the ball, whose Steiner decomposition is exact —
/// `L³ + 6L²R + 3πLR² + (4/3)πR³`. The material volume is the block,
/// less that rounded void, less the vent's cylinder above the cavity
/// ceiling.
///
/// The Steiner form itself is [`rounded_box_volume`], where its
/// derivation lives; what stays here is the fixture's own arithmetic.
fn filleted_cavity_volume() -> f64 {
    let block = 4.0_f64.powi(3);
    let vent = core::f64::consts::PI * 0.5 * 0.5 * (4.0 - 3.0);
    let rounded_void = rounded_box_volume(2.0 - 2.0 * R, R);
    block - rounded_void - vent
}

/// **THE FILLETED CAVITY** — all twelve concave edges at one radius:
/// twelve quarter-cylinder cove bands, eight concave sphere-octant
/// corners, tiers 1–3, the census, the Euler relation, the certified
/// volume against the Steiner closed form, and a watertight mesh.
///
/// This row is NOT a chart pin, and saying so is the point (an
/// earlier draft claimed the volume would redden on a convex-aimed
/// concave chart; execution shows it stays green — with axis-aligned
/// walls the patch is an iso-parameter rectangle wherever the chart
/// aims, so the downstream machinery is chart-placement-tolerant).
/// The chart fold's guards are the plan-level mirror pin
/// (`blend::open::planar::tests::a_corner_plan_takes_its_links_convexity`)
/// and the carved-body seam/quarter-turn pin
/// (`review_blend4_r2_probes::r2_the_octant_charts_seam_and_quarter_turn_are_feet_on_both_sides`).
#[test]
fn the_filleted_cavity() {
    let body = vented_cavity();
    let out = fillet_edges(&body, &cavity_edges(&body), R, Tol::witness())
        .expect("the cavity's twelve concave edges fillet");
    let out_body = out.body;

    assert_eq!(validate(&out_body), Ok(()), "tier 1");
    assert_eq!(validate_closed(&out_body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&out_body, Tol::witness()),
        Ok(()),
        "tier 3"
    );

    assert_eq!(out.blend_faces.len(), 12, "one cove band per concave edge");
    assert_eq!(out.corner_faces.len(), 8, "one octant per concave corner");
    assert_eq!(out.band_faces.len(), 0, "no chain of this request closes");
    for fk in &out.corner_faces {
        let f = out_body.get_face(*fk).expect("a minted corner face");
        assert!(
            matches!(
                out_body.get_surface(f.surface),
                Some(Surface::Sphere { .. })
            ),
            "a fillet corner patch is a sphere octant"
        );
        assert!(
            !f.sense,
            "a concave octant's material lies outside its ball, so the chart \
             normal (outward radial) is folded"
        );
    }

    let (nv, ne, nf) = (
        out_body.vertices().count(),
        out_body.edges().count(),
        out_body.faces().count(),
    );
    assert_eq!(
        (nv, ne, nf),
        (36, 66, 34),
        "census — the same carve topology as the chamfered cavity"
    );
    assert_eq!(
        nv as i64 - ne as i64 + nf as i64 - 2,
        2,
        "Euler–Poincaré, corrected for the vent mouth's two ringed faces"
    );

    let want = filleted_cavity_volume();
    let props = topo::mass_properties(&out_body, Tol::witness()).expect("closed-form props");
    assert!(
        (props.volume - want).abs() <= 1e-12 * want,
        "volume {} vs closed form {want}",
        props.volume
    );

    let mesh = mesh::tessellate(&out_body, 5e-3, Tol::witness()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// A minted blend face's outward normal at each of its own boundary
/// vertices — points that are ON the patch by construction (the
/// feet). The chart normal of both minted kinds is the outward
/// radial, so the outward normal is that radial folded through the
/// stored sense bit — read off stored data, never sampled from a
/// mesh.
fn outward_at_boundary(body: &Body<f64>, face: topo::FaceKey) -> Vec<(Point3<f64>, Vec3<f64>)> {
    let f = body.get_face(face).expect("a minted face");
    let s = f.sense_sign::<f64>();
    let radial = |q: Point3<f64>| match body.get_surface(f.surface).expect("its surface") {
        Surface::Cylinder { origin, axis, .. } => {
            let d = q - *origin;
            (d - *axis * d.dot(*axis)).normalize()
        }
        Surface::Sphere { center, .. } => (q - *center).normalize(),
        other => panic!("a fillet carve mints cylinders and spheres, got {other:?}"),
    };
    let first = match body.get_loop(f.outer).expect("an outer loop").boundary {
        topo::LoopBoundary::Cycle { first } => first,
        ref other => panic!("a minted face's boundary is a cycle, got {other:?}"),
    };
    body.loop_cycle(first)
        .expect("the cycle walks")
        .into_iter()
        .map(|he| {
            let vk = body.get_half_edge(he).expect("a cycle half-edge").start;
            let q = *body
                .get_vertex(vk)
                .and_then(|x| body.get_point(x.point))
                .expect("a boundary vertex's point");
            (q, radial(q) * s)
        })
        .collect()
}

/// **THE ORIENTATION ROW: every minted fillet face faces the VOID it
/// fills — and the convex carve's face away from its material.** The
/// concave cavity's void is at the CENTRE, so a face minted outward
/// from the material has the cavity centre on its outward side; on
/// the filleted CUBE the material is at the centre and the same
/// reading must point away. One row, both material sides: a sense
/// convention that ignores the stored convexity verdict — or a feet
/// or ball fold that puts the surface on the wrong side — satisfies
/// at most one half.
#[test]
fn every_minted_fillet_face_faces_its_own_void() {
    let body = vented_cavity();
    let out =
        fillet_edges(&body, &cavity_edges(&body), R, Tol::witness()).expect("the cavity fillets");
    let centre = p(2.0, 2.0, 2.0);
    for face in out.blend_faces.iter().chain(out.corner_faces.iter()) {
        for (at, n) in outward_at_boundary(&out.body, *face) {
            let reach = (centre - at).dot(n);
            assert!(
                reach > 0.5 * R,
                "a concave fillet face {face:?} must face the void it fills: \
                 the cavity centre is {reach} m along its outward normal at {at:?}"
            );
        }
    }

    let cube_body = cube(2.0, Tol::witness());
    let cube_edges: Vec<EdgeKey> = cube_body.edges().map(|(k, _)| k).collect();
    let cut = fillet_edges(&cube_body, &cube_edges, R, Tol::witness())
        .expect("a cube's twelve edges fillet");
    let cube_centre = p(1.0, 1.0, 1.0);
    for face in cut.blend_faces.iter().chain(cut.corner_faces.iter()) {
        for (at, n) in outward_at_boundary(&cut.body, *face) {
            let reach = (cube_centre - at).dot(n);
            assert!(
                reach < -0.5 * R,
                "a convex fillet face {face:?} must face away from its material: \
                 the cube centre is {reach} m along its outward normal at {at:?}"
            );
        }
    }
}

/// **The corner recourse is followable, on both sides, as written.**
/// A fillet refused at a MIXED corner ships a sentence naming the
/// uniform trivalent corner — all convex or all concave — as what
/// carves. This row takes the refusal (the L-bracket's reflex edge,
/// whose ends are 2-of-3 corners), asserts the sentence, then
/// EXECUTES both of its clauses at the refused size: the all-convex
/// cube and the all-concave cavity both carve.
#[test]
fn the_corner_recourse_is_followable_on_both_sides() {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 1.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 2.0),
        Point2::new(0.0, 2.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the L is a valid profile");
    let bracket = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .expect("the bracket extrudes")
        .body;
    let on_reflex = |q: Point3<f64>| (q.x - 1.0).abs() < 1e-12 && (q.y - 1.0).abs() < 1e-12;
    let reflex: Vec<EdgeKey> = bracket
        .edges()
        .filter(|(k, _)| {
            let Some(e) = bracket.get_edge(*k) else {
                return false;
            };
            let pt = |vk| {
                bracket
                    .get_vertex(vk)
                    .and_then(|x| bracket.get_point(x.point))
                    .copied()
            };
            let (Some(a), Some(b)) = (
                bracket
                    .get_half_edge(e.he_plus)
                    .map(|h| h.start)
                    .and_then(pt),
                bracket.half_edge_end(e.he_plus).and_then(pt),
            ) else {
                return false;
            };
            on_reflex(a) && on_reflex(b)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(reflex.len(), 1, "the bracket's one reflex vertical edge");
    let refused =
        fillet_edges(&bracket, &reflex, 0.1, Tol::witness()).expect_err("a mixed corner refuses");
    let text = refused.error.to_string();
    assert!(
        matches!(
            refused.error,
            BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { .. },
                ..
            }
        ),
        "the reflex edge's ends are mixed corners, got {text}"
    );
    assert!(
        text.contains(FILLET3_CORNER_RECOURSE),
        "the refusal ships the corner recourse, got {text}"
    );
    // Both clauses of the sentence, executed at the refused size.
    let cube_body = cube(2.0, Tol::witness());
    let cube_edges: Vec<EdgeKey> = cube_body.edges().map(|(k, _)| k).collect();
    fillet_edges(&cube_body, &cube_edges, 0.1, Tol::witness())
        .expect("the all-convex clause carves");
    let cavity = vented_cavity();
    fillet_edges(&cavity, &cavity_edges(&cavity), 0.1, Tol::witness())
        .expect("the all-concave clause carves");
}
