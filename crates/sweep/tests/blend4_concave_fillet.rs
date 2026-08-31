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
//! The fixture is the concave-chamfer suite's vented cavity — the
//! cube's mirror: a block whose rectangular cavity has twelve concave
//! edges meeting at eight all-concave trihedra, vented by a round
//! chimney so the body stays one shell. Its builders are restated
//! here (the suite-tree fixture-copy class the chamfer suite already
//! declares; a shared home is a test-support change no row here
//! needs).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::arms::corner_ball;
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig, FILLET3_CORNER_RECOURSE};
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, subtract, validate, validate_closed};

/// The fillet radius, meters.
const R: f64 = 0.25;

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

/// An axis-aligned box, authored the way a user would: a rectangle
/// profile on a translated sketch plane, extruded.
fn brick(lo: Point3<f64>, hi: Point3<f64>) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(lo.x, lo.y),
        Point2::new(hi.x, lo.y),
        Point2::new(hi.x, hi.y),
        Point2::new(lo.x, hi.y),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, lo.z) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a rectangle is a valid profile");
    extrude(&profile, Extrusion::Distance(hi.z - lo.z), Tol::witness())
        .expect("a brick extrudes")
        .body
}

/// A circular rod: two half-arc profile segments extruded, so its wall
/// is a cylinder and the ring it cuts in a plane is a circle.
fn rod(center: Point2<f64>, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(center.x - r, center.y), 1.0),
        ProfileVertex::new(Point2::new(center.x + r, center.y), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z0) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), Tol::witness())
        .expect("a rod extrudes")
        .body
}

/// The vented cavity: block `[0,4]³`, cavity `[1,3]³`, round chimney
/// of radius `0.5` on `x = y = 2` from `z = 2.5` clear of the top —
/// one shell, twelve concave cavity edges, eight all-concave corners.
/// The vent's mouth ring sits `0.5` clear of every cavity wall, twice
/// the setback `R` carves at, so the ring rides through.
fn vented_cavity() -> Body<f64> {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 2.0), 0.5, 2.5, 5.0);
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let vented = subtract(&block, &vent, Tol::witness())
        .expect("the vent cut succeeds")
        .body()
        .expect("the vent cut leaves material")
        .body
        .clone();
    subtract(&vented, &cavity, Tol::witness())
        .expect("the cavity cut succeeds")
        .body()
        .expect("the cavity cut leaves material")
        .body
        .clone()
}

/// The cavity's twelve edges, found by their endpoints — both of them
/// corners of the cavity box `[1,3]³` — never by index.
fn cavity_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let corner = |q: Point3<f64>| {
        [q.x, q.y, q.z]
            .iter()
            .all(|c| (c - 1.0).abs() < 1e-12 || (c - 3.0).abs() < 1e-12)
    };
    let mut found: Vec<EdgeKey> = body
        .edges()
        .filter(|(k, _)| {
            let Some(e) = body.get_edge(*k) else {
                return false;
            };
            let Some(h) = body.get_half_edge(e.he_plus) else {
                return false;
            };
            let Some(end) = body.half_edge_end(e.he_plus) else {
                return false;
            };
            let pt = |vk| {
                body.get_vertex(vk)
                    .and_then(|x| body.get_point(x.point))
                    .copied()
            };
            match (pt(h.start), pt(end)) {
                (Some(a), Some(b)) => corner(a) && corner(b),
                _ => false,
            }
        })
        .map(|(k, _)| k)
        .collect();
    found.sort_unstable();
    found
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
    let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, false);
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
    let concave = corner_ball(verts, normals, r, false);
    for (i, n) in normals.iter().enumerate() {
        let depth = (concave.center - verts[i]).dot(*n);
        assert!(
            (depth - r).abs() < 1e-15,
            "wall {i}: the ball rests at +r in the void, got {depth}"
        );
    }
    assert!((concave.independence - 0.8).abs() < 1e-15);
    let convex = corner_ball(verts, normals, r, true);
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
    let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, false);
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
/// EITHER arm — the convex apex direction, antipodal to the concave
/// patch's own apex (whose feet lie along `−n_i`). The unit folds the
/// side into the stored chart, and this row now pins the closed
/// state: each ball's pole aims at its own apex foot.
#[test]
fn the_stored_chart_pole_aims_at_each_sides_own_apex() {
    let r = 0.15;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)];
    let mean = (normals[0] + normals[1] + normals[2]).normalize();
    for (convex, apex) in [(true, mean), (false, -mean)] {
        let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, convex);
        let Surface::Sphere { axis, .. } = ball.surface else {
            panic!("the corner ball's surface is a sphere");
        };
        let aim = axis.dot(apex);
        assert!(
            (aim - 1.0).abs() < 1e-15,
            "the pole aims at this side's apex foot (convex {convex}, dot {aim})"
        );
    }
}

/// **The volume the filleted cavity's void encloses** — the rolling
/// ball's mirror of the chamfer suite's closed form. A concave fillet
/// on every edge and corner of the cavity leaves exactly the void the
/// ball can sweep: the Minkowski sum of the shrunk box (side
/// `a − 2r`) with the ball, whose Steiner decomposition is exact —
/// `L³ + 6L²r + 3πLr² + (4/3)πr³` with `L = a − 2r`. The material
/// volume is the block, less that rounded void, less the vent's
/// cylinder above the cavity ceiling.
fn filleted_cavity_volume(a: f64, r: f64) -> f64 {
    let block = 4.0_f64.powi(3);
    let vent = core::f64::consts::PI * 0.5 * 0.5 * (4.0 - 3.0);
    let l = a - 2.0 * r;
    let rounded_void = l.powi(3)
        + 6.0 * l * l * r
        + 3.0 * core::f64::consts::PI * l * r * r
        + (4.0 / 3.0) * core::f64::consts::PI * r.powi(3);
    block - rounded_void - vent
}

/// **THE FILLETED CAVITY** — all twelve concave edges at one radius:
/// twelve quarter-cylinder cove bands, eight concave sphere-octant
/// corners, tiers 1–3, the census, the Euler relation, the certified
/// volume against the Steiner closed form, and a watertight mesh.
///
/// The volume row is also the CHART pin: the closed-form curved-face
/// inventory computes a sphere patch only as an iso-parameter
/// rectangle in its own chart, so a concave octant charted with the
/// convex aim (pole antipodal to the patch, feet a half-turn off the
/// seam) does not integrate — the carve would land, and this row
/// would still redden on `VolumeUncomputable`.
#[test]
fn the_filleted_cavity() {
    let body = vented_cavity();
    let out = fillet_edges(&body, &cavity_edges(&body), R, band(), Tol::witness())
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

    let want = filleted_cavity_volume(2.0, R);
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
    let out = fillet_edges(&body, &cavity_edges(&body), R, band(), Tol::witness())
        .expect("the cavity fillets");
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
    let cut = fillet_edges(&cube_body, &cube_edges, R, band(), Tol::witness())
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
    let refused = fillet_edges(&bracket, &reflex, 0.1, band(), Tol::witness())
        .expect_err("a mixed corner refuses");
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
    fillet_edges(&cube_body, &cube_edges, 0.1, band(), Tol::witness())
        .expect("the all-convex clause carves");
    let cavity = vented_cavity();
    fillet_edges(&cavity, &cavity_edges(&cavity), 0.1, band(), Tol::witness())
        .expect("the all-concave clause carves");
}
