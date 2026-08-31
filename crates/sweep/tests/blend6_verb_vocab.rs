//! **The verb a shared blend refusal speaks** — measured through the
//! two public doors, `fillet_edges` and `chamfer_edges`.
//!
//! The two verbs share one refusal vocabulary by design (the
//! near-parallel-enum failure class is what the reuse exists to
//! avoid), so the door that raised a refusal is the only party that
//! knows which verb the caller asked for. This suite pins what each
//! door's caller actually READS.
//!
//! The measured baseline this suite was born with (every shared arm's
//! `Display` opening with a hard-coded `"fillet"`, so a
//! `chamfer_edges` caller was told a fillet refused) is fixed by the
//! `BlendRefusal` wrapper: the door attaches the verb ONCE, the inner
//! error is verb-neutral, and the rows below hold both doors to it —
//! including that no composition renders a verb prefix twice.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::chamfer::chamfer_edges;
use sweep::fillet::build::fillet_edges;
use sweep::fillet::{BlendError, BlendRefusal};
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey};

/// The cube side, meters.
const L: f64 = 1.0;
/// The blend size (radius or setback), meters.
const D: f64 = 0.1;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn all_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges().map(|(k, _)| k).collect()
}

/// The four edges of the cube's top face — a CLOSED chain whose
/// square junctions are not tangent-continuous.
fn top_loop(body: &Body<f64>) -> Vec<EdgeKey> {
    let at_top = |e: EdgeKey| -> bool {
        let Some(edge) = body.get_edge(e) else {
            return false;
        };
        let Some(start) = body.get_half_edge(edge.he_plus).map(|h| h.start) else {
            return false;
        };
        let Some(end) = body.half_edge_end(edge.he_plus) else {
            return false;
        };
        [start, end].into_iter().all(|v| {
            body.get_vertex(v)
                .and_then(|x| body.get_point(x.point))
                .is_some_and(|p| p.z > L - 1e-9)
        })
    };
    let picked: Vec<EdgeKey> = all_edges(body).into_iter().filter(|e| at_top(*e)).collect();
    assert_eq!(picked.len(), 4, "a cube has four top-rim edges");
    picked
}

/// A circular prism: two half-arc profile segments extruded, so every
/// rim edge has a plane and a CYLINDER for supports.
fn cylinder(r: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-r, 0.0), 1.0),
        ProfileVertex::new(Point2::new(r, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a circular prism")
        .body
}

/// **The chamfer-purity claim, in one place**: a chamfer refusal's
/// text opens with the chamfer's verb and never says "fillet" —
/// modulo the `fillet3_*` predicate NAMES, which are K-corpus roster
/// carriers shared by both verbs deliberately (they meter the same
/// margins) and are therefore stripped before the check.
fn assert_speaks_as_the_chamfer(refusal: &BlendRefusal, label: &str) {
    let text = format!("{refusal}");
    assert!(
        text.starts_with("chamfer: "),
        "the {label} refusal must open with the chamfer's verb: {text}"
    );
    let without_rosters = text.replace("fillet3_", "");
    assert!(
        !without_rosters.contains("fillet"),
        "the {label} refusal must not speak as the fillet: {text}"
    );
    assert_eq!(
        text.matches("chamfer: ").count(),
        1,
        "the verb prefix must render exactly once: {text}"
    );
}

/// The fillet-side single-prefix claim: the verb opens the text and
/// is not rendered twice by the door-plus-inner composition.
fn assert_speaks_once_as_the_fillet(refusal: &BlendRefusal, label: &str) {
    let text = format!("{refusal}");
    assert!(
        text.starts_with("fillet: "),
        "the {label} refusal must open with the fillet's verb: {text}"
    );
    assert_eq!(
        text.matches("fillet: ").count(),
        1,
        "the verb prefix must render exactly once: {text}"
    );
    assert!(
        !text.contains("chamfer"),
        "a fillet refusal must not speak as the chamfer: {text}"
    );
}

/// **THE HEADLINE ROW, flipped.** One edge of a cube through
/// `chamfer_edges` refuses over the SHARED run-out arm; the caller
/// who asked for a chamfer now reads `"chamfer: …"` with a recourse
/// that speaks of the blend rather than telling them to fillet. The
/// suite's first commit measured the same row reading
/// `"fillet assembly: …"`.
#[test]
fn a_chamfer_caller_reads_the_chamfer_verb_over_a_shared_run_out() {
    let body = cube(L, Tol::witness());
    let edges = all_edges(&body);
    let err = chamfer_edges(&body, &edges[..1], D, band(), Tol::witness())
        .expect_err("a partially-requested corner is a run-out");
    assert!(
        matches!(err.error, BlendError::UnsupportedRunOut { .. }),
        "the shared run-out arm is what refused: {err:?}"
    );
    assert_speaks_as_the_chamfer(&err, "run-out");
    let text = format!("{err}");
    assert!(
        text.contains("blend a chain that terminates"),
        "the recourse speaks of the blend, not the other verb: {text}"
    );
}

/// The shared BATTERY refusal under the chamfer's verb: the top rim's
/// square corners are not tangent-continuous.
#[test]
fn a_chamfer_caller_reads_the_chamfer_verb_over_a_shared_chain_break() {
    let body = cube(L, Tol::witness());
    let err = chamfer_edges(&body, &top_loop(&body), D, band(), Tol::witness())
        .expect_err("square junctions are not tangent-continuous");
    assert!(
        matches!(err.error, BlendError::ChainNotG1 { .. }),
        "the shared G1 predicate is what refused: {err:?}"
    );
    assert_speaks_as_the_chamfer(&err, "chain-break");
}

/// The fillet caller's verb stays right, and renders ONCE: the same
/// shared run-out arm through `fillet_edges` opens `"fillet: "` with
/// no second verb prefix anywhere in the composition.
#[test]
fn a_fillet_caller_reads_the_fillet_verb_once_over_the_same_shared_arm() {
    let body = cube(L, Tol::witness());
    let edges = all_edges(&body);
    let err = fillet_edges(&body, &edges[..1], D, band(), Tol::witness())
        .expect_err("a partially-requested corner is a run-out");
    assert!(
        matches!(err.error, BlendError::UnsupportedRunOut { .. }),
        "the shared run-out arm is what refused: {err:?}"
    );
    assert_speaks_once_as_the_fillet(&err, "run-out");
}

/// The chamfer's own arm still speaks as the chamfer — through the
/// wrapper now, with the verb rendered exactly once.
#[test]
fn the_chamfers_own_arm_speaks_as_the_chamfer_once() {
    let cyl = cylinder(0.5, 1.0);
    let edges = all_edges(&cyl);
    let err = chamfer_edges(&cyl, &edges, D, band(), Tol::witness())
        .expect_err("a curved support has no ruled strip");
    assert!(
        matches!(err.error, BlendError::ChamferArmUnsupported { .. }),
        "the chamfer's own arm table is what refused: {err:?}"
    );
    assert_speaks_as_the_chamfer(&err, "arm-table");
}

/// **The purity battery**: every distinct shared refusal the chamfer
/// door reaches from the shipped fixtures, each held to the
/// chamfer-purity claim. One test rather than one per row because
/// every row rebuilds the same fixtures
/// (`memories/test-suite-cost.md`); each row carries its own label.
#[test]
fn every_reachable_chamfer_refusal_speaks_as_the_chamfer() {
    let body = cube(L, Tol::witness());
    let edges = all_edges(&body);
    let t = Tol::witness();

    // Invalid input, checked at the door: a nonpositive setback.
    let nonpositive = chamfer_edges(&body, &edges[..1], 0.0, band(), t)
        .expect_err("a zero setback has no band to build");
    assert!(matches!(
        nonpositive.error,
        BlendError::NonpositiveSize { .. }
    ));
    assert_speaks_as_the_chamfer(&nonpositive, "nonpositive-size");

    // Invalid input: a repeated edge.
    let repeated = chamfer_edges(&body, &[edges[0], edges[0]], D, band(), t)
        .expect_err("a repeated edge would double a link");
    assert!(matches!(repeated.error, BlendError::RepeatedEdge { .. }));
    assert_speaks_as_the_chamfer(&repeated, "repeated-edge");

    // The shared clearance screen, on the chamfer's own setbacks: all
    // twelve edges at a setback too deep for the cube's faces.
    let clearance = chamfer_edges(&body, &edges, 0.55, band(), t)
        .expect_err("two 0.55 m setbacks do not fit a 1 m face");
    assert!(matches!(
        clearance.error,
        BlendError::FaceClearanceUncertified { .. }
    ));
    assert_speaks_as_the_chamfer(&clearance, "clearance");

    // The shared corner-configuration vocabulary: a four-edge request
    // leaves the top corners' verticals unrequested — a run-out — and
    // one edge's two corners the same; the L-bracket's concave edge
    // is the corner CONFIGURATION case.
    let corner = chamfer_edges(&l_bracket(), &[concave_edge(&l_bracket())], D, band(), t)
        .expect_err("a mixed-convexity corner is out of the octant scope");
    assert!(matches!(corner.error, BlendError::UnsupportedCorner { .. }));
    assert_speaks_as_the_chamfer(&corner, "corner-config");
}

/// **Followability, per verb (the issue-1278 rule)**: a recourse is a
/// claim about a SECOND request, so the pin executes that second
/// request as the same verb and asserts the promised outcome.
///
/// - clearance: "reduce the blend size" — the chamfer that refused at
///   0.55 m builds at 0.1 m;
/// - corner/run-out: "blend a chain that terminates in a
///   three-convex-edge vertex" — on a cube that is the request whose
///   every corner is fully requested, and it builds;
/// - tangential: "blend an edge whose supports meet at a definite
///   angle" — the cube's edges are such edges, and they build.
#[test]
fn a_chamfer_recourse_followed_as_a_chamfer_reaches_its_promised_outcome() {
    let body = cube(L, Tol::witness());
    let edges = all_edges(&body);
    let t = Tol::witness();

    let refused = chamfer_edges(&body, &edges, 0.55, band(), t)
        .expect_err("two 0.55 m setbacks do not fit a 1 m face");
    assert!(matches!(
        refused.error,
        BlendError::FaceClearanceUncertified { .. }
    ));
    assert!(
        chamfer_edges(&body, &edges, D, band(), t).is_ok(),
        "the reduced blend size the recourse names must build"
    );

    let run_out = chamfer_edges(&body, &edges[..1], D, band(), t)
        .expect_err("a partially-requested corner is a run-out");
    assert!(matches!(
        run_out.error,
        BlendError::UnsupportedRunOut { .. }
    ));
    assert!(
        chamfer_edges(&body, &edges, D, band(), t).is_ok(),
        "the fully-requested-corner request the recourse names must build"
    );
}

/// **The shared TANGENTIAL arm is chamfer-reachable, and speaks as the
/// chamfer**: the dihedral sign is metered during link RESOLUTION,
/// before the arm table, so a co-surface seam meridian (one sphere on
/// both sides, sine exactly zero) refuses `TangentialEdge` through
/// `chamfer_edges` too — not the arm-table refusal.
#[test]
fn a_chamfer_on_a_co_surface_seam_refuses_tangential_as_the_chamfer() {
    let ball = sweep::test_support::revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, -1.0), 1.0),
            ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
        ],
        sweep::Revolution::Full,
        Tol::witness(),
    );
    let seam = ball
        .edges()
        .map(|(k, _)| k)
        .find(|k| both_sides_spheres(&ball, *k))
        .expect("a full ball carries a co-surface seam meridian");
    let err = chamfer_edges(&ball, &[seam], 0.05, band(), Tol::witness())
        .expect_err("a co-surface seam has no definite wedge side");
    match err.error {
        BlendError::TangentialEdge { margin, .. } => {
            assert_eq!(margin, 0.0, "a co-surface seam's sine is structurally zero");
        }
        ref other => panic!("expected the shared tangential arm, got {other:?}"),
    }
    assert_speaks_as_the_chamfer(&err, "tangential");
}

/// **The shared BODY frontier under the chamfer's verb**: two disjoint
/// cubes in one body are valid input the in-place surgery has not been
/// built for, whichever verb asks.
#[test]
fn a_chamfer_on_a_two_solid_body_refuses_the_body_frontier_as_the_chamfer() {
    let mut body = cube(L, Tol::witness());
    let other = cube(L, Tol::witness());
    topo::instance::graft_disjoint_all(&mut body, &other, Tol::witness())
        .expect("a disjoint graft");
    let edges = all_edges(&body);
    let err = chamfer_edges(&body, &edges[..1], D, band(), Tol::witness())
        .expect_err("the in-place surgery is built for one solid");
    assert!(
        matches!(err.error, BlendError::UnsupportedBody { solids, .. } if solids == 2),
        "the body frontier carries the solid count: {err:?}"
    );
    assert_speaks_as_the_chamfer(&err, "two-solid-body");
}

/// **An ESCALATED margin under the chamfer's verb**: a setback whose
/// clearance margin lands inside the band escalates through the
/// funnel, and the message — which names the `fillet3_*` predicate,
/// a roster name both verbs meter under — still opens as the chamfer.
#[test]
fn a_chamfer_escalation_speaks_as_the_chamfer() {
    let body = cube(L, Tol::witness());
    let edges = all_edges(&body);
    let eps = Tol::witness().get().eps;
    // gap 1.0, two setbacks: margin = 1.0 − 2d = 5·eps, inside the band.
    let d = 0.5 - 2.5 * eps;
    let err = chamfer_edges(&body, &edges, d, band(), Tol::witness())
        .expect_err("an in-band clearance margin escalates");
    match err.error {
        BlendError::Escalated { ref source, .. } => {
            assert_eq!(source.predicate, Some("fillet3_face_clearance"));
        }
        ref other => panic!("expected the clearance escalation, got {other:?}"),
    }
    assert_speaks_as_the_chamfer(&err, "escalated");
}

/// **The seam-vertex tag is battery-shadowed for the chamfer on the
/// rim family** (the measured half of the seam recourse's per-verb
/// disposition): a seam-split rim's arcs are plane–sphere, and a
/// chamfer request on one refuses at the ARM TABLE during resolution
/// — before predicate 6 could ever classify the seam vertex. So the
/// seam recourse's carve promise, which names the fillet's closed-rim
/// band, is read by fillet callers alone on these fixtures.
#[test]
fn a_chamfer_on_a_seam_split_rim_arc_refuses_at_the_arm_table_not_the_seam_vertex() {
    let body = sweep::test_support::lantern(Tol::witness());
    let arc = body
        .edges()
        .map(|(k, _)| k)
        .find(|k| plane_sphere_sides(&body, *k))
        .expect("the lantern carries plane–sphere mouth arcs");
    let err = chamfer_edges(&body, &[arc], 0.02, band(), Tol::witness())
        .expect_err("a plane–sphere arc has no ruled strip");
    assert!(
        matches!(err.error, BlendError::ChamferArmUnsupported { .. }),
        "the arm table shadows the seam-vertex classification: {err:?}"
    );
    assert_speaks_as_the_chamfer(&err, "seam-rim-arc");
}

/// Both of `edge`'s supports are spheres (a co-surface seam meridian
/// candidate), and the edge is open.
fn both_sides_spheres(body: &Body<f64>, edge: EdgeKey) -> bool {
    match edge_surfaces(body, edge) {
        Some((a, b)) => {
            matches!(a, geom::Surface::Sphere { .. }) && matches!(b, geom::Surface::Sphere { .. })
        }
        None => false,
    }
}

/// One plane and one sphere support: the rim-arc family.
fn plane_sphere_sides(body: &Body<f64>, edge: EdgeKey) -> bool {
    match edge_surfaces(body, edge) {
        Some((a, b)) => {
            (matches!(a, geom::Surface::Plane { .. }) && matches!(b, geom::Surface::Sphere { .. }))
                || (matches!(a, geom::Surface::Sphere { .. })
                    && matches!(b, geom::Surface::Plane { .. }))
        }
        None => false,
    }
}

/// The two stored support surfaces of an OPEN edge, by structural
/// reads only.
fn edge_surfaces(
    body: &Body<f64>,
    edge: EdgeKey,
) -> Option<(geom::Surface<f64>, geom::Surface<f64>)> {
    let e = body.get_edge(edge)?;
    let start = body.get_half_edge(e.he_plus)?.start;
    if Some(start) == body.half_edge_end(e.he_plus) {
        return None; // closed rim, not the open-arc family
    }
    let face_of = |he: topo::HalfEdgeKey| -> Option<geom::Surface<f64>> {
        let l = body.get_half_edge(he)?.parent_loop;
        let f = body.get_loop(l)?.face;
        body.get_surface(body.get_face(f)?.surface).cloned()
    };
    Some((face_of(e.he_plus)?, face_of(e.he_minus)?))
}

/// An L-bracket: the six-vertex L profile extruded by 1 m. Its one
/// reflex profile corner becomes the body's one concave edge.
fn l_bracket() -> Body<f64> {
    let lp = ProfileLoop::new(
        [
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 0.5),
            (0.5, 0.5),
            (0.5, 1.0),
            (0.0, 1.0),
        ]
        .into_iter()
        .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
        .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("an L is a valid profile");
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .expect("an L-bracket extrudes")
        .body
}

/// The bracket's one concave edge: both supports planes, the dihedral
/// reflex — found by asking the battery's own resolution which edge
/// classified concave is overkill here; the L profile has exactly one
/// reflex corner at (0.5, 0.5), so the concave edge is the vertical
/// one through it.
fn concave_edge(body: &Body<f64>) -> EdgeKey {
    let near = |p: geom_core::Point3<f64>| (p.x - 0.5).abs() < 1e-9 && (p.y - 0.5).abs() < 1e-9;
    let vertical = |e: EdgeKey| -> bool {
        let Some(edge) = body.get_edge(e) else {
            return false;
        };
        let (Some(start), Some(end)) = (
            body.get_half_edge(edge.he_plus).map(|h| h.start),
            body.half_edge_end(edge.he_plus),
        ) else {
            return false;
        };
        [start, end].into_iter().all(|v| {
            body.get_vertex(v)
                .and_then(|x| body.get_point(x.point))
                .is_some_and(|p| near(*p))
        })
    };
    body.edges()
        .map(|(k, _)| k)
        .find(|e| vertical(*e))
        .expect("the L-bracket has a vertical edge through its reflex corner")
}
