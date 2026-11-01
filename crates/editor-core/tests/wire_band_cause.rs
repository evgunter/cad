//! **The band refusals name the cause they caught, not the fact that
//! they caught one** — pinned through the real doors.
//!
//! A band has two failures, at opposite ends of one axis: K·ε
//! overflowing to infinity at an ε within a factor K of `f64::MAX`,
//! and K·ε rounding back down onto ε at a subnormal one. A run reaches
//! the first through `Band::linear`; the second needs an ε below the
//! floor `EPS_MIN`, which the run's validator refuses (geom-core's
//! `an_eps_below_the_floor_is_refused_at_every_door`). The repairs are opposite, so a refusal that says
//! only "the band could not be built" sends the reader the wrong way
//! half the time, and `SelectRefusal::Band` / `NamingError::Band` carry
//! the constructor's own diagnostic instead.
//!
//! THE DOORS, not a re-derivation of them. Both facts this suite
//! asserts — that the pathological pair is a VALID tolerance, and that
//! `Band::linear` refuses it the way claimed — go through
//! [`Tolerance::init`] (which runs the same private `validate` the env
//! path runs) and [`geom_core::Band::linear`] itself. Re-deriving
//! either in the test would leave the premise unguarded: widening
//! `Band::from_thresholds`, or tightening `Tolerance::validate`, must
//! turn this suite red, and only the real call can do that.
//!
//! WHY A CHILD PROCESS. `geom_core::Tolerance` commits once per
//! process (spec D4), `tests/all.rs` aggregates every editor-core
//! suite into ONE binary, and the pathological tolerances here would
//! poison every other suite in it. So each row runs in its own
//! re-exec'd child, the `m4_pr6_eps_diff` pattern: a `#[test]` that
//! no-ops unless its env var is set, spawned by the parent with
//! `current_exe()` filtered to its own module path.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, DocEdit, EvalOptions, NamePat, NamingError, Node, NodeErrorKind, NodeResult,
    ProfileDoc, RecipeNodeId, SelectRefusal, Selector, evaluate, select_where,
};
use fixture::{desc, insert};
use geom_core::tolerance::{DEFAULT_K, Tolerance};
use geom_core::{Band, BandError, BandField, Tol};

/// The env var naming which row a child runs.
const PROBE_ROW: &str = "WIRE_BAND_CAUSE_ROW";

/// **The collapse arm's rendered threshold**: the smallest positive
/// `f64`, at which a K near 1 collapses the band.
const COLLAPSE_EPS: f64 = 5e-324;

/// **The overflow row's ε**: large enough that K·ε overflows at the
/// DEFAULT K, still finite and above `EPS_MIN`.
const OVERFLOW_EPS: f64 = f64::MAX / 2.0;

/// CHILD MODE. No-op unless [`PROBE_ROW`] is set, so the parent suite
/// run passes over it.
#[test]
fn child_band_row() {
    let Ok(row) = std::env::var(PROBE_ROW) else {
        return;
    };
    assert_eq!(row, "overflow", "unknown row");

    // DOOR 1 — the premise. `init` validates before it commits, so a
    // pair this call accepts is a pair the run's own validator
    // accepts. Tightening `validate` against huge ε reddens here.
    Tolerance::init(Tolerance {
        eps: OVERFLOW_EPS,
        k: DEFAULT_K,
    })
    .expect("the pathological pair is a VALID tolerance");

    // DOOR 2 — the failure. `Band::linear`, not the arithmetic it
    // performs: widening `from_thresholds` so a collapsed band never
    // surfaces reddens here.
    let caught = Band::linear(Tol::witness())
        .expect_err("Band::linear must refuse a tolerance that admits no band");
    assert!(
        matches!(caught, BandError::InvalidValue { field: BandField::Escalate, value }
            if value.is_infinite()),
        "expected the overflow arm at eps = {OVERFLOW_EPS:e}, got {caught:?}"
    );

    // The two causes must be distinguishable BEFORE a refusal can
    // distinguish them. The collapse arm is reached through
    // `Band::linear_at` and `Band::angular_at` (geom-core's
    // `band_tolerance.rs`); here it is only rendered.
    let other = BandError::Empty {
        zero: COLLAPSE_EPS,
        escalate: COLLAPSE_EPS,
    };
    assert_ne!(caught.to_string(), other.to_string());

    // The refusals forward it VERBATIM, not a paraphrase of it.
    let inner = caught.to_string();
    let select = SelectRefusal::Band(caught);
    let naming = NamingError::Band(caught);
    assert!(
        select.to_string().contains(&inner),
        "the select refusal renders as {select} and does not carry {inner:?}"
    );
    assert!(
        naming.to_string().contains(&inner),
        "the naming refusal renders as {naming} and does not carry {inner:?}"
    );

    // DOOR 3 — what a USER actually gets. At any tolerance where
    // `Band::linear` fails, the evaluator's own band door refuses
    // every node before an operation runs, and it refuses carrying
    // the same `BandError` verbatim.
    let (doc, node) = bare_placement();
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let Some(NodeResult::Failed(failed)) = ev.nodes.get(&node) else {
        panic!("the node must FAIL here: {:?}", ev.nodes.get(&node));
    };
    assert!(
        matches!(&failed.kind, NodeErrorKind::Band(e) if e.to_string() == inner),
        "the evaluator relabelled its band failure: {:?}",
        failed.kind
    );

    // DOOR 4 — and the authoring door refuses even earlier, the same
    // way: a profile's path validation builds a band before the
    // document will accept the node at all.
    let refused = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(Node::Profile(desc(vec![vec![
                    (0.0, 0.0),
                    (1.0, 0.0),
                    (1.0, 1.0),
                    (0.0, 1.0),
                ]]))),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect_err("a profile cannot be authored where no band exists");
    assert!(
        format!("{refused:?}").contains(&inner),
        "the profile door relabelled its band failure: {refused:?}"
    );

    // WHICH IS WHY the names doors' own band arms are NOT reachable
    // through the public evaluate -> query path: nothing upstream of
    // them survives, so `select_where` short-circuits on the node
    // lookup at `select.rs` and never reaches its `Band::linear`.
    // Pinned, not asserted in prose — if some future door lets a
    // value through at a bandless tolerance, this goes red and the
    // reachability claim above it has to be rewritten.
    assert_eq!(
        select_where(
            &ev,
            node,
            &Selector::of(NamePat::any()),
            &[],
            &doc.var_env::<f64>(),
            Tol::witness(),
        )
        .expect("select_where short-circuits before its band"),
        Vec::<editor_core::StableName>::new(),
        "select_where reached its band door — the arm is live after all"
    );
}

/// A bare placement, copying a `⊔` of nothing — nodes whose authoring
/// builds no band, so a document holding them exists where no band
/// does, and the seat from which `select_where` can be asked anything
/// at all. The placement's fold builds the band.
fn bare_placement() -> (ProfileDoc, RecipeNodeId) {
    let (doc, nothing) = insert(
        ProfileDoc::empty_derived("wire_band_cause", Tol::witness()),
        Node::Assemble {
            members: editor_core::Bodies::Spelled(Vec::new()),
        },
    );
    insert(
        doc,
        crate::fixture::stood(editor_core::BodyRead::plain(editor_core::Operand::Node(
            nothing,
        ))),
    )
}

fn spawn_row(row: &str) {
    let exe = std::env::current_exe().expect("test exe path");
    // Name the probe by MODULE PATH: `tests/all.rs` aggregates the
    // suites, so libtest sees it as `<this_module>::child_band_row`.
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::child_band_row"),
        None => "child_band_row".to_string(),
    };
    let status = std::process::Command::new(exe)
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env(PROBE_ROW, row)
        .env_remove("CAD_TOLERANCE_EPS")
        .env_remove("CAD_AMBIGUITY_K")
        .status()
        .expect("probe spawns");
    assert!(
        status.success(),
        "the {row} row failed (child output above)"
    );
}

#[test]
fn band_refusals_name_which_band_failure_they_caught() {
    spawn_row("overflow");
}
