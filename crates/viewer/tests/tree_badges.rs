//! **Failed and Poisoned badges carry the TYPED payload's message.**
//!
//! GQ2's per-node result DAG is the whole subject: a failing node is
//! `Failed(NodeError)` and its descendants are `Poisoned { through }`,
//! independent subgraphs complete, and what a badge says is the typed
//! error's own rendering rather than a sentence this crate wrote.
//!
//! The rows that matter most are the downstream ones. A poisoned row
//! must say where the failure is and must NOT recite what it was —
//! the defect that reading gives is four instance rows carrying the
//! same paragraph of refusal prose — and the row it points at must be
//! one this same tree badges FAILED, or "upstream failure at node 5"
//! sends the user somewhere there is nothing to read.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::document::{BooleanOp, CancelToken, EvalOptions, NodeResult, evaluate};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::frame::Tone;
use viewer::session::{DocSession, SessionOp};
use viewer::tree::{self, RowStatus};

#[test]
fn a_failing_document_renders_failed_and_poisoned_from_the_typed_payloads() {
    let tol = Tol::witness();
    let (doc, extrude, moved) = common::broken_document(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let rows = session.tree_rows();
    assert!(tree::has_faults(&rows));

    let failed = rows
        .iter()
        .find(|row| row.id == extrude)
        .expect("the extrude has a row");
    let RowStatus::Failed { message } = &failed.status else {
        panic!("expected Failed, got {:?}", failed.status);
    };
    assert_eq!(failed.status.badge(), "FAILED");
    assert!(!message.is_empty());

    // The message is the payload's, not this crate's: it must be
    // byte-identical to the shipped error's own rendering.
    let evaluation = session.evaluation().expect("a result landed");
    let Some(NodeResult::Failed(error)) = evaluation.result(extrude) else {
        panic!("the evaluation should report the extrude as failed");
    };
    assert_eq!(message, &error.to_string());

    let poisoned = rows
        .iter()
        .find(|row| row.id == moved)
        .expect("the transform has a row");
    match &poisoned.status {
        RowStatus::Poisoned { through, message } => {
            assert_eq!(*through, extrude, "poison names the failure it came from");
            assert_eq!(
                message.as_deref(),
                Some(tree::downstream_wording(extrude).as_str()),
                "a poisoned row POINTS at the cause's row; it does not recite it"
            );
        }
        other => panic!("expected Poisoned, got {other:?}"),
    }
    assert_eq!(poisoned.status.badge(), "POISONED");
}

/// **Only the row that refused is actionable**, over rows a real
/// evaluation produced rather than hand-built statuses: the colour
/// rule the Features pane draws is `RowStatus::tone()`'s answer, so a
/// wrong tone is a wrong colour and this is where it goes red.
#[test]
fn only_the_row_whose_own_operation_refused_is_actionable() {
    let tol = Tol::witness();
    let (doc, extrude, moved) = common::broken_document(tol);
    // A second, unrelated body, so an `Ok` row is in the population.
    let (doc, other_profile) = common::framed_square(&doc, 0.02, tol);
    let mut session = DocSession::inline(doc, tol);

    // Unpumped: nothing has been evaluated, so nothing is actionable.
    assert!(
        session
            .tree_rows()
            .iter()
            .all(|row| row.status.tone() == Tone::Advisory),
        "a document nobody has evaluated yet gives a reader nothing to act on"
    );

    session.pump();
    let rows = session.tree_rows();
    let tone_of = |id| common::status_of(&rows, id).tone();
    assert_eq!(tone_of(extrude), Tone::Actionable, "the node that refused");
    assert_eq!(
        tone_of(moved),
        Tone::Advisory,
        "a poisoned row points at the cause; the cause is where a reader acts"
    );
    assert_eq!(tone_of(other_profile), Tone::Advisory, "a healthy row");
    assert_eq!(
        rows.iter()
            .filter(|row| row.status.tone() == Tone::Actionable)
            .count(),
        1,
        "one broken feature makes one loud row, whatever it poisons"
    );
}

#[test]
fn an_independent_subgraph_completes_beside_a_failure() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = common::broken_document(tol);
    // A second, unrelated body in the same document: GQ2's ratified
    // "a failure poisons only its descendants".
    let (doc, other_profile) = common::framed_square(&doc, 0.02, tol);
    let (doc, other_extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile: other_profile,
            distance: common::len(0.005),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let rows = session.tree_rows();

    let status_of = |id| common::status_of(&rows, id);
    assert!(matches!(status_of(extrude), RowStatus::Failed { .. }));
    assert_eq!(status_of(other_extrude), RowStatus::Ok);
    assert_eq!(status_of(other_profile), RowStatus::Ok);
}

#[test]
fn rows_before_the_first_result_read_as_unevaluated_rather_than_ok() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let session = DocSession::inline(doc, tol);
    // Deliberately NOT pumped: nothing has been evaluated yet.
    let rows = session.tree_rows();
    assert!(rows.iter().all(|row| row.status == RowStatus::Unevaluated));
    assert!(
        !tree::has_faults(&rows),
        "unevaluated is not a fault; it is an absence of measurement"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.id == extrude)
            .map(|row| row.status.badge()),
        Some("—")
    );
}

#[test]
fn a_canceled_runs_missing_tail_reads_as_unevaluated() {
    // A canceled evaluation carries the completed PREFIX only, so
    // nodes past it have no entry at all. The tree must show that as
    // an absence rather than as success.
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let cancel = CancelToken::new();
    cancel.cancel();
    let evaluation = evaluate::<f64>(&doc, None, &cancel, &EvalOptions::default(), tol);
    let rows = tree::rows(&doc, Some(&evaluation));
    assert!(!rows.is_empty());
    assert!(rows.iter().all(|row| row.status == RowStatus::Unevaluated));
    assert!(!tree::has_faults(&rows));
}

#[test]
fn the_tree_marks_the_documents_product_roots() {
    let tol = Tol::witness();
    let (doc, profile, extrude) = common::parametric_plate(tol);
    let rows = tree::rows(&doc, None);
    let root_ids: Vec<_> = rows
        .iter()
        .filter(|row| row.root)
        .map(|row| row.id)
        .collect();
    assert_eq!(
        root_ids,
        vec![extrude],
        "the extrude is the product; the profile it consumes is not"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.id == profile)
            .map(|row| row.kind),
        Some("Profile")
    );
}

/// **A refused mate solve badges the MATE as the cause and everything
/// else it reached as downstream.**
///
/// The kernel records one cluster refusal against every instance in
/// the cluster and every mate holding it together, each as that node's
/// own `Failed` — mates and instances are DAG leaves, so the placement
/// solve poisons across a graph the result DAG has no edges for. The
/// tree must not read that verbatim: the fault names the mate it is
/// about, so that mate's row is the cause and every other row it
/// reached — including instances the mate does not touch, and the
/// other mate in the cluster — reads as downstream of it.
#[test]
fn a_refused_mate_solve_names_the_mate_and_reads_every_other_row_downstream() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-refusal", tol);
    let mut session = common::asm::open_bench(&bench, tol);

    // Two seat mates joining all three instances into ONE cluster:
    // post_a under the shelf's middle, post_b under its quarter point.
    // The second is a planar rest alone, which leaves its pair free
    // to slide and spin, so the solve refuses UNDER naming that mate
    // — a verdict about the pair, which the edit door admits (a mate
    // the table refuses on its own datum is refused at the insert).
    let add_mate = |session: &mut DocSession, post, alignment| {
        common::insert(
            session,
            SessionOp::AddMate {
                a: common::head(common::asm::in_part(post, &bench.post_top)),
                b: common::head(common::asm::in_part(bench.shelf_i, &bench.shelf_bottom)),
                class: ContactClass::Rest,
                alignment,
            },
        )
    };
    let sound = add_mate(
        &mut session,
        bench.post_a,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 2.0, None),
    );
    let offender = add_mate(
        &mut session,
        bench.post_b,
        common::asm::rest_alignment(common::asm::SHELF_LENGTH / 4.0),
    );
    // ONE evaluation over both mates. Pumping between them would give
    // the same rows: a mate's key carries the solve's answer, so the
    // sound mate re-runs when the cluster breaks around it. What this
    // row pins is the attribution, not that.
    session.pump();

    let rows = session.tree_rows();
    assert!(tree::has_faults(&rows), "the cluster refused: {rows:?}");
    let status_of = |id| common::status_of(&rows, id);

    // The offending mate is the cause, and the only row that is.
    let RowStatus::Failed { message } = status_of(offender) else {
        panic!(
            "the offending mate carries the cause: {:?}",
            status_of(offender)
        );
    };
    assert!(
        message.contains(pncad::document::UNDER_RECOURSE),
        "the kernel's own words on the mate's row: {message}"
    );
    let causes: Vec<_> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Failed { .. }))
        .map(|row| row.id)
        .collect();
    assert_eq!(
        causes,
        vec![offender],
        "exactly one actionable row, and it is the mate the fault names"
    );

    // Every other row the refusal reached reads as downstream of it —
    // the two posts, the shelf, and the sound mate. `post_a` is the
    // row the issue is about: the offending mate does not touch it.
    // What each of them says is the POINTER, not a fourth copy of the
    // refusal prose the offending mate's row already carries.
    for (id, what) in [
        (
            bench.post_a,
            "an instance the offending mate does not touch",
        ),
        (bench.post_b, "an instance the offending mate does touch"),
        (bench.shelf_i, "the shelf the cluster hangs from"),
        (sound, "the sound mate in the refused cluster"),
    ] {
        match status_of(id) {
            RowStatus::Poisoned { through, message } => {
                assert_eq!(through, offender, "{what} points at the offending mate");
                assert_eq!(
                    message,
                    Some(tree::downstream_wording(offender)),
                    "{what} points at the cause's row"
                );
                assert!(
                    !message
                        .unwrap_or_default()
                        .contains(pncad::document::UNDER_RECOURSE),
                    "{what} must not recite the refusal a user reads once, on the mate"
                );
            }
            other => panic!("{what} must read as downstream, got {other:?}"),
        }
        assert_eq!(status_of(id).badge(), "POISONED", "{what}");
    }

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}

/// **A contradiction between two mates blames both, both carry the
/// cause, and the rows it reaches point at one of them.**
///
/// Two mates on ONE pair at incompatible alignments: the fold names
/// `held` and `added` together, because neither is the wrong one on
/// the fault's own telling. The first of them EVALUATED before the
/// second broke the pair, and its row still reports the refusal: a
/// mate's content key carries the solve's answer, so the memo cannot
/// serve last evaluation's `Ok` into the run whose fault names it.
#[test]
fn a_contradiction_points_downstream_rows_at_a_row_that_is_actually_failing() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-contradiction", tol);
    let mut session = common::asm::open_bench(&bench, tol);

    let add_mate = |session: &mut DocSession, alignment| {
        common::insert(
            session,
            SessionOp::AddMate {
                a: common::head(common::asm::in_part(bench.post_a, &bench.post_top)),
                b: common::head(common::asm::in_part(bench.shelf_i, &bench.shelf_bottom)),
                class: ContactClass::Rest,
                alignment,
            },
        )
    };
    // The first mate lands and EVALUATES — the memo now holds an `Ok`
    // for it — and only then does the second one contradict it.
    let held = add_mate(
        &mut session,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 2.0, None),
    );
    session.pump();
    let added = add_mate(
        &mut session,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 2.0 + 0.01, None),
    );
    session.pump();

    let rows = session.tree_rows();
    let status_of = |id| common::status_of(&rows, id);
    // THE PREMISE THIS ROW RESTS ON, asserted rather than assumed: the
    // mate that evaluated FIRST reports the refusal too. What made
    // this row distinct from the one above was once a memo hazard —
    // `held` reading `Ok` in the run that blames it — and what makes
    // it distinct now is the kernel's own consistency: a fault naming
    // TWO mates leaves both of them actionable.
    let failing: Vec<_> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Failed { .. }))
        .map(|row| row.id)
        .collect();
    assert!(
        failing.contains(&held),
        "the mate that evaluated before the pair broke is a cause too: {rows:?}"
    );
    assert!(
        failing.contains(&added),
        "the mate that broke the pair is a cause: {rows:?}"
    );
    assert!(
        !failing.contains(&bench.post_a) && !failing.contains(&bench.shelf_i),
        "the mated instances are not causes: {rows:?}"
    );
    // Both mates are blamed, and WHICH one the instances point at is
    // decided, not a coin flip: `blamed_mates` yields the fault's own
    // `held` then `added`, and the tree takes the first — with no
    // corroboration step left to skip it, because the kernel now
    // reports both as failing. So the pointer is `held`, every run.
    for instance in [bench.post_a, bench.shelf_i] {
        match status_of(instance) {
            RowStatus::Poisoned { through, message } => {
                assert_eq!(
                    through, held,
                    "the instance points at the first mate the fault names"
                );
                assert!(
                    matches!(status_of(through), RowStatus::Failed { .. }),
                    "and that mate's own row reports the failure: {:?}",
                    status_of(through)
                );
                assert_eq!(
                    message,
                    Some(tree::downstream_wording(through)),
                    "the message points at that mate's row"
                );
            }
            other => panic!("a mated instance must read as downstream, got {other:?}"),
        }
    }
    // post_b is in no cluster with them: an independent subgraph
    // completes, exactly as GQ2 says.
    assert_eq!(status_of(bench.post_b), RowStatus::Ok);

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}

/// **A row poisoned through a row that is ITSELF downstream points at
/// the terminal cause, not at a POISONED row.**
///
/// The reachable document: a boolean over two instances of a cluster
/// that then refuses. The kernel poisons the boolean through its first
/// blocking input — an instance — and reports that instance as its own
/// `Failed`; the tree redraws the instance as downstream of the mate.
/// Read verbatim the boolean would point at a row drawn POISONED and
/// weak, so the user's walk would be two hops through a row with
/// nothing to act on.
#[test]
fn a_boolean_over_a_refused_clusters_instances_points_at_the_mate() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-two-hop", tol);
    let mut session = common::asm::open_bench(&bench, tol);

    // The boolean lands first, over two instances that are both `Ok`:
    // the operand seat admits an instance (`combine::denotes_body`).
    let boolean = common::insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: bench.post_a,
            b: bench.shelf_i,
        },
    );
    session.pump();
    assert_eq!(
        common::status_of(&session.tree_rows(), boolean),
        RowStatus::Ok,
        "the boolean builds before the cluster refuses"
    );

    let add_mate = |session: &mut DocSession, post, alignment| {
        common::insert(
            session,
            SessionOp::AddMate {
                a: common::head(common::asm::in_part(post, &bench.post_top)),
                b: common::head(common::asm::in_part(bench.shelf_i, &bench.shelf_bottom)),
                class: ContactClass::Rest,
                alignment,
            },
        )
    };
    add_mate(
        &mut session,
        bench.post_a,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 2.0, None),
    );
    let offender = add_mate(
        &mut session,
        bench.post_b,
        common::asm::rest_alignment(common::asm::SHELF_LENGTH / 4.0),
    );
    session.pump();

    // The kernel's own reading: poisoned through the INSTANCE, which
    // is what makes the hop a real one rather than a hypothetical.
    let (_, ev) = session.landed_pair().expect("landed");
    assert_eq!(
        ev.result(boolean).and_then(NodeResult::poisoned_through),
        Some(bench.post_a),
        "the evaluation poisons the boolean through its first blocking input"
    );

    let rows = session.tree_rows();
    match common::status_of(&rows, boolean) {
        RowStatus::Poisoned { through, message } => {
            assert_eq!(
                through, offender,
                "the boolean points past the instance at the mate that refused"
            );
            assert_eq!(message, Some(tree::downstream_wording(offender)));
        }
        other => panic!("the boolean reads as downstream, got {other:?}"),
    }

    // The invariant that makes the pointer worth following, over the
    // whole tree: every POISONED row names a row THIS TREE badges
    // FAILED, so one hop lands on words to read.
    for row in &rows {
        if let RowStatus::Poisoned { through, .. } = &row.status {
            assert!(
                matches!(common::status_of(&rows, *through), RowStatus::Failed { .. }),
                "{:?} points at {through:?}, which draws {}",
                row.id,
                common::status_of(&rows, *through).badge()
            );
        }
    }

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}

// ---- The refusal that names no row ----

/// The env var naming the child process that commits a bandless
/// tolerance. `geom_core::Tolerance` commits once per process and
/// `tests/all.rs` aggregates every suite into ONE binary, so the
/// pathological tolerance runs in a re-exec'd child or it poisons
/// every other suite in the binary
/// (`crates/editor-core/tests/wire_band_cause.rs`'s pattern).
const BAND_PROBE: &str = "TREE_BADGES_BAND_PROBE";

/// An ε large enough that K·ε overflows at the default K, and still
/// finite and strictly positive — so `Tolerance::validate` admits it
/// and `Band::linear` refuses it.
///
/// The same value as `wire_band_cause.rs`'s `OVERFLOW_EPS`, which it
/// was derived from; that both spellings exist is
/// `work/tint/re-exec-child-harness-is-copied-per-suite-and-greens-when-it-does-not-run`.
const BANDLESS_EPS: f64 = f64::MAX / 2.0;

/// **What the child prints once it has run every assertion below.**
///
/// The parent asserts on THIS, not on the child's exit status:
/// libtest exits 0 when a filter matches nothing (*"running 0 tests …
/// test result: ok"*), so a rename of this suite, of the child fn, or
/// of `all.rs`'s nesting would turn the row into a silent pass while
/// the child holds every assertion it has. The sentinel is stronger
/// than a matched-test count as well as cheaper: it is printed after
/// the last assertion, so it also goes missing if the environment
/// guard sends the child down its no-op return.
const BAND_PROBE_DONE: &str = "BAND-PROBE-COMPLETE";

/// **A run-tolerance refusal reaches every instance in the DOCUMENT,
/// blames none of them, and points the eye nowhere** — and no mate
/// can be INSERTED under it, since the edit door asks the solve's own
/// admission, which begins with the band (a loaded snapshot can still
/// hold one).
///
/// `MateFault::Band` is the one fault arm that reaches rows without
/// naming a subject, so it is the one arm `blamed_mates` answers empty
/// for on rows a user can actually meet. What this row pins is what
/// that costs and what it must not buy back:
///
/// - the refusal is the RUN's, not a cluster's — every instance is
///   reached, each a singleton cluster of its own, which is why a
///   badge wording scoped to "this cluster" would name the wrong set;
/// - every reached row keeps the payload's own words, byte-identical,
///   so nothing here composes a sentence onto a failing row;
/// - and NO row is drawn downstream of another, because the fault
///   names no culprit and one may not be invented to have somewhere
///   to send the eye.
///
/// CHILD MODE. No-op unless [`BAND_PROBE`] is set, so the parent suite
/// run passes over it.
#[test]
fn child_band_refusal_rows() {
    use pncad::document::{
        Alignment, AxisSense, DocEdit, DocRef, MateFault, MateFrame, MatePrimitive, Node,
        NodeErrorKind, ProfileDoc, RecipeNodeId, apply, content_pin,
    };
    use pncad::geom_core::Band;
    use pncad::geom_core::tolerance::{DEFAULT_K, Tolerance};
    use pncad::prelude::StableName;
    use pncad::select::EntityKind;

    if std::env::var(BAND_PROBE).is_err() {
        return;
    }
    // DOOR 1 — the premise. `init` validates before it commits, so a
    // pair this call accepts is a pair the run's own validator
    // accepts.
    Tolerance::init(Tolerance {
        eps: BANDLESS_EPS,
        k: DEFAULT_K,
    })
    .expect("the pathological pair is a VALID tolerance");
    let tol = Tol::witness();
    // DOOR 2 — the failure the whole row stands on. If `Band::linear`
    // ever admits this tolerance, `MateFault::Band` stops being
    // reachable and the carve-out below has no subject.
    Band::linear(tol).expect_err("no band exists at this tolerance");

    // The document. Nothing in it carries geometry — a profile cannot
    // be AUTHORED where no band exists, and neither can a mate: the
    // edit door asks the solve's own admission, which begins with the
    // band, so the mate refuses typed at the door (DOOR 2a below) —
    // which is the shape a user meets when a saved document commits
    // its own ε on open: three instances, authored elsewhere, read
    // back at a tolerance that admits no band.
    let part = ProfileDoc::empty_derived("band-part", tol);
    let doc_ref = DocRef {
        id: part.id(),
        pin: content_pin(&part, tol).expect("the pin computes"),
    };
    let mut asm = ProfileDoc::empty_derived("band-asm", tol);
    let a = common::insert_into(&mut asm, Node::instantiate_part(doc_ref), tol);
    let b = common::insert_into(&mut asm, Node::instantiate_part(doc_ref), tol);
    // A third instance, which no mate could touch: every instance is
    // its own singleton cluster here, and the row that decides whether
    // this refusal is a cluster's or the run's is that ALL of them
    // refuse.
    let lone = common::insert_into(&mut asm, Node::instantiate_part(doc_ref), tol);
    let face_of = |instance| {
        common::head(StableName {
            kind: EntityKind::Face,
            node: instance,
            path: Vec::new(),
        })
    };
    let frame = MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    };
    // DOOR 2a — a mate cannot be INSERTED where no band exists: the
    // edit door refuses it with the solve's own `Band`. A snapshot
    // loaded under this tolerance can still hold one, and the solve
    // records `Band` against it the way it does against every
    // instance below.
    let refused = apply(
        &asm,
        &DocEdit::InsertNode {
            node: Node::Mate {
                a: face_of(a),
                b: face_of(b),
                class: ContactClass::Rest,
                alignment: Alignment {
                    a: frame,
                    b: frame,
                    primitive: MatePrimitive::FrameCoincidence,
                    sense: AxisSense::Opposed,
                    clocking: None,
                },
            },
        },
        tol,
        &pncad::document::RefusingReach,
    )
    .expect_err("no band, no mate");
    assert!(
        matches!(
            &refused,
            pncad::document::EditError::MateRefused { fault, .. }
                if matches!(fault.as_ref(), MateFault::Band { .. })
        ),
        "{refused:?}"
    );

    let evaluation: pncad::document::Evaluation<f64> = pncad::document::evaluate(
        &asm,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let rows = tree::rows(&asm, Some(&evaluation));
    assert!(tree::has_faults(&rows), "the run refused: {rows:?}");

    // DOOR 3 — the fault is the MATE arm, not the evaluator's own
    // per-node band door. The whole carve-out is about which arm a
    // user meets: if the evaluator ever refuses mates and instances
    // before the solve does, this goes red and there is no live
    // `MateFault::Band` left to badge.
    let reached: Vec<RecipeNodeId> = [a, b, lone]
        .into_iter()
        .filter(|&id| {
            matches!(
                evaluation.result(id),
                Some(NodeResult::Failed(e))
                    if matches!(&e.kind, NodeErrorKind::Mate(f)
                        if matches!(**f, MateFault::Band { .. }))
            )
        })
        .collect();
    assert_eq!(
        reached,
        vec![a, b, lone],
        "the band refusal reaches every instance, each a cluster of its own — it is the \
         RUN's refusal, not one cluster's"
    );

    // Every reached row draws its own FAILED, carrying the payload's
    // own words byte-identical. Composing a cohort clause onto a
    // failing row's message reddens here.
    for &id in &reached {
        let status = common::status_of(&rows, id);
        assert_eq!(status.badge(), "FAILED", "{id:?}: {status:?}");
        let Some(NodeResult::Failed(error)) = evaluation.result(id) else {
            panic!("{id:?} must be Failed in the evaluation");
        };
        assert_eq!(
            status.message(),
            Some(error.to_string().as_str()),
            "{id:?} must carry the payload's own rendering, not a sentence this crate wrote"
        );
    }

    // And nothing is drawn downstream of anything: the fault names no
    // mate, so there is no row to send the eye to and none is
    // invented. A lane that closes the badging defect by PICKING a
    // culprit reddens here, which is the decision this row holds.
    let pointed: Vec<RecipeNodeId> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Poisoned { .. }))
        .map(|row| row.id)
        .collect();
    assert_eq!(
        pointed,
        Vec::<RecipeNodeId>::new(),
        "a band refusal blames no mate, so no row may point at one"
    );

    // Last act: the parent reads this to know the assertions above ran
    // at all.
    println!("{BAND_PROBE_DONE}");
}

/// The parent of [`child_band_refusal_rows`].
#[test]
fn a_band_refusal_reaches_the_whole_document_and_blames_no_row() {
    let exe = std::env::current_exe().expect("test exe path");
    // Name the probe by MODULE PATH: `tests/all.rs` aggregates the
    // suites, so libtest sees it as
    // `<this_module>::child_band_refusal_rows`.
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::child_band_refusal_rows"),
        None => "child_band_refusal_rows".to_string(),
    };
    let out = std::process::Command::new(exe)
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env(BAND_PROBE, "1")
        .env_remove("CAD_TOLERANCE_EPS")
        .env_remove("CAD_AMBIGUITY_K")
        .output()
        .expect("probe spawns");
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(out.status.success(), "the band row failed:\n{text}");
    // THE ANTI-VACUITY FLOOR, and it is the whole row's: every
    // assertion this test makes lives in the child, and a green exit
    // status is what a child that ran NOTHING also reports. Reading
    // the child's own stdout for a sentinel is the tree's idiom for
    // this (`crates/geom-core/tests/ambiguity_k_env.rs`);
    // `test_utils::vacuity` is the wrong instrument here because its
    // floors are counted and asserted in-process, which is exactly the
    // process whose execution is in doubt.
    assert!(
        text.contains(BAND_PROBE_DONE),
        "the child exited 0 without reaching its assertions — a filter that \
         matches nothing greens. Child output:\n{text}"
    );
}

/// **A document whose only non-`Ok` row is POISONED is not
/// building**, and it has nothing to act on — `tree::has_faults` and
/// [`RowStatus::tone`] read two different axes, and the one that is
/// easy to collapse is this one.
///
/// Every other `has_faults` assertion in this tree stands on a
/// document that also carries a `Failed` row, so dropping `Poisoned`
/// from the fault policy — or rewriting that policy as "any
/// actionable row", which is the natural-looking collapse — reddens
/// nowhere but here. The rows are built rather than evaluated because
/// that is the state no real document reaches: a poisoning always has
/// its cause in the same tree, which is exactly why the arm is
/// otherwise never tested alone.
#[test]
fn a_downstream_failure_alone_is_a_fault_the_reader_cannot_act_on() {
    use pncad::document::RecipeNodeId;

    let row = |id: u64, status: RowStatus| viewer::tree::TreeRow {
        id: RecipeNodeId(id),
        kind: "Transform",
        pose: None,
        depth: 0,
        root: false,
        status,
        note: None,
    };
    let rows = [
        row(1, RowStatus::Ok),
        row(
            2,
            RowStatus::Poisoned {
                through: RecipeNodeId(1),
                message: Some("upstream failure at feature 1".to_owned()),
            },
        ),
    ];
    assert!(
        tree::has_faults(&rows),
        "a row showing someone else's failure still says the document is not building"
    );
    assert!(
        rows.iter().all(|row| row.status.tone() == Tone::Advisory),
        "and nothing in it is a row the reader can act on: {rows:?}"
    );
}
