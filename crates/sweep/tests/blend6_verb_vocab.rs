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
use sweep::fillet::{BlendRefusal, FilletError};
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
        matches!(err.error, FilletError::UnsupportedRunOut { .. }),
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
        matches!(err.error, FilletError::ChainNotG1 { .. }),
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
        matches!(err.error, FilletError::UnsupportedRunOut { .. }),
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
        matches!(err.error, FilletError::ChamferArmUnsupported { .. }),
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
        FilletError::NonpositiveSize { .. }
    ));
    assert_speaks_as_the_chamfer(&nonpositive, "nonpositive-size");

    // Invalid input: a repeated edge.
    let repeated = chamfer_edges(&body, &[edges[0], edges[0]], D, band(), t)
        .expect_err("a repeated edge would double a link");
    assert!(matches!(repeated.error, FilletError::RepeatedEdge { .. }));
    assert_speaks_as_the_chamfer(&repeated, "repeated-edge");

    // The shared clearance screen, on the chamfer's own setbacks: all
    // twelve edges at a setback too deep for the cube's faces.
    let clearance = chamfer_edges(&body, &edges, 0.55, band(), t)
        .expect_err("two 0.55 m setbacks do not fit a 1 m face");
    assert!(matches!(
        clearance.error,
        FilletError::FaceClearanceUncertified { .. }
    ));
    assert_speaks_as_the_chamfer(&clearance, "clearance");

    // The shared corner-configuration vocabulary: a four-edge request
    // leaves the top corners' verticals unrequested — a run-out — and
    // one edge's two corners the same; the L-bracket's concave edge
    // is the corner CONFIGURATION case.
    let corner = chamfer_edges(&l_bracket(), &[concave_edge(&l_bracket())], D, band(), t)
        .expect_err("a mixed-convexity corner is out of the octant scope");
    assert!(matches!(
        corner.error,
        FilletError::FilletCornerUnsupported { .. }
    ));
    assert_speaks_as_the_chamfer(&corner, "corner-config");
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
