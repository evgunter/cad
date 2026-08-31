//! **The verb a shared blend refusal speaks** — measured through the
//! two public doors, `fillet_edges` and `chamfer_edges`.
//!
//! The two verbs share one refusal vocabulary by design (the
//! near-parallel-enum failure class is what the reuse exists to
//! avoid), so the door that raised a refusal is the only party that
//! knows which verb the caller asked for. This suite pins what each
//! door's caller actually READS.
//!
//! **Measured state, pinned before the fix**: every shared arm's
//! `Display` opens with a hard-coded `"fillet"`, so a `chamfer_edges`
//! caller is told a fillet refused — the right fact under the wrong
//! verb. The rows below assert that wrong verb VERBATIM; the unit
//! that owns this suite flips them to the wrapper-supplied verb.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::chamfer::chamfer_edges;
use sweep::fillet::FilletError;
use sweep::fillet::build::fillet_edges;
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

/// **THE HEADLINE ROW: the wrong verb, measured.** One edge of a cube
/// through `chamfer_edges` refuses over the SHARED run-out arm — the
/// request does not cover the two corners — and the caller who asked
/// for a chamfer reads `"fillet assembly: …"` with a recourse telling
/// them to *fillet* a fuller chain. The fact is right; the verb named
/// is not the one they called.
#[test]
fn a_chamfer_caller_reads_the_fillet_verb_over_a_shared_run_out() {
    let body = cube(L, Tol::witness());
    let edges = all_edges(&body);
    let err = chamfer_edges(&body, &edges[..1], D, band(), Tol::witness())
        .expect_err("a partially-requested corner is a run-out");
    assert!(
        matches!(err, FilletError::UnsupportedRunOut { .. }),
        "the shared run-out arm is what refused: {err:?}"
    );
    let text = format!("{err}");
    assert!(
        text.starts_with("fillet assembly: "),
        "MEASURED (pre-fix): the chamfer's refusal opens with the fillet's verb: {text}"
    );
    assert!(
        text.contains("fillet a chain that terminates"),
        "MEASURED (pre-fix): the recourse tells a chamfer caller to fillet: {text}"
    );
}

/// The same wrong verb over a shared BATTERY refusal: the top rim's
/// square corners are not tangent-continuous, and the chamfer caller
/// reads `"fillet chain: …"`.
#[test]
fn a_chamfer_caller_reads_the_fillet_verb_over_a_shared_chain_break() {
    let body = cube(L, Tol::witness());
    let err = chamfer_edges(&body, &top_loop(&body), D, band(), Tol::witness())
        .expect_err("square junctions are not tangent-continuous");
    assert!(
        matches!(err, FilletError::ChainNotG1 { .. }),
        "the shared G1 predicate is what refused: {err:?}"
    );
    let text = format!("{err}");
    assert!(
        text.starts_with("fillet chain: "),
        "MEASURED (pre-fix): the chamfer's chain refusal speaks as the fillet: {text}"
    );
}

/// The fillet caller's verb is RIGHT today and must stay right: the
/// same shared run-out arm through `fillet_edges` speaks as the
/// fillet.
#[test]
fn a_fillet_caller_reads_the_fillet_verb_over_the_same_shared_arm() {
    let body = cube(L, Tol::witness());
    let edges = all_edges(&body);
    let err = fillet_edges(&body, &edges[..1], D, band(), Tol::witness())
        .expect_err("a partially-requested corner is a run-out");
    assert!(
        matches!(err, FilletError::UnsupportedRunOut { .. }),
        "the shared run-out arm is what refused: {err:?}"
    );
    let text = format!("{err}");
    assert!(
        text.starts_with("fillet assembly: "),
        "the fillet's refusal speaks as the fillet: {text}"
    );
}

/// The contrast row: the ONE arm the chamfer added for itself already
/// speaks as the chamfer — which is what makes the shared arms' wrong
/// verb a defect of the sharing, not of the vocabulary.
#[test]
fn the_chamfers_own_arm_already_speaks_as_the_chamfer() {
    let cyl = cylinder(0.5, 1.0);
    let edges = all_edges(&cyl);
    let err = chamfer_edges(&cyl, &edges, D, band(), Tol::witness())
        .expect_err("a curved support has no ruled strip");
    assert!(
        matches!(err, FilletError::ChamferArmUnsupported { .. }),
        "the chamfer's own arm table is what refused: {err:?}"
    );
    let text = format!("{err}");
    assert!(
        text.starts_with("chamfer: "),
        "the chamfer's own arm speaks as the chamfer: {text}"
    );
}
