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
//! one this same tree badges FAILED, or "upstream failure at Mate 000000000005"
//! sends the user somewhere there is nothing to read.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::Bodies;

use pncad::document::{CancelToken, EvalOptions, NodeResult, evaluate};
use pncad::geom_core::Tol;
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

    let failed = common::row_of(&rows, extrude);
    let RowStatus::Failed { message, .. } = &failed.status else {
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
    assert_eq!(message, &error.spoken(session.committed_doc(), evaluation));

    let poisoned = common::row_of(&rows, moved);
    match &poisoned.status {
        RowStatus::Poisoned { through, message } => {
            assert_eq!(*through, extrude, "poison names the failure it came from");
            assert_eq!(
                message.as_deref(),
                Some(
                    format!(
                        "upstream failure at Extrude {} — that row carries the cause",
                        test_utils::refusal::tag(extrude.0.digest())
                    )
                    .as_str()
                ),
                "a poisoned row POINTS at the cause's row, spoken by its kind and tag; it \
                 does not recite it"
            );
        }
        other => panic!("expected Poisoned, got {other:?}"),
    }
    assert_eq!(poisoned.status.badge(), "POISONED");
}

/// **Only the row that refused is actionable**, over rows a real
/// evaluation produced rather than hand-built statuses: the colour
/// rule the Features pane draws is `TreeRow::tone()`'s answer — the
/// status's, on a document with no assertion — so a wrong tone is a
/// wrong colour and this is where it goes red.
#[test]
fn only_the_row_whose_own_operation_refused_is_actionable() {
    let tol = Tol::witness();
    let (doc, extrude, moved) = common::broken_document(tol);
    // A second, unrelated body, so an `Ok` row is in the population.
    let (doc, other_profile) = common::square_profile(&doc, 0.02, tol);
    let mut session = DocSession::inline(doc, tol);

    // Unpumped: nothing has been evaluated, so nothing is actionable.
    assert!(
        session
            .tree_rows()
            .iter()
            .all(|row| row.tone() == Tone::Advisory),
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
            .filter(|row| row.tone() == Tone::Actionable)
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
    let (doc, other_profile) = common::square_profile(&doc, 0.02, tol);
    let (doc, other_extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile: other_profile.into(),
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
    assert_eq!(common::row_of(&rows, extrude).status.badge(), "—");
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
    let rows = tree::rows(
        &doc,
        Some(&evaluation),
        &viewer::parts::PartFiles::default(),
    );
    assert!(!rows.is_empty());
    assert!(rows.iter().all(|row| row.status == RowStatus::Unevaluated));
    assert!(!tree::has_faults(&rows));
}

#[test]
fn the_tree_marks_the_bodies_the_world_places() {
    let tol = Tol::witness();
    let (doc, profile, extrude) = common::parametric_plate(tol);
    let rows = tree::rows(&doc, None, &viewer::parts::PartFiles::default());
    let placed: Vec<_> = rows
        .iter()
        .filter(|row| row.placed)
        .map(|row| row.id)
        .collect();
    assert_eq!(
        placed,
        vec![extrude],
        "the extrude is placed; the profile it reads is not, and the placement's own row is \
         an ordinary one"
    );
    let placement = doc.placements()[0];
    assert!(
        rows.iter().any(|row| row.id == placement && !row.placed),
        "the placement is a row of its own, unbadged"
    );
    assert_eq!(
        common::row_of(&rows, profile).spoken.kind(),
        Some("Profile")
    );
}

/// **A refused bundle badges its PLACEMENT as the cause and every
/// placement chained to it as downstream.**
///
/// The kernel records one bundle refusal against the placement whose
/// bundle refused and against every placement chained to it, each as
/// that node's own `Failed` — a constraint's reads are where a copy
/// stands, not a DAG input, so the solve poisons across a graph the
/// result DAG has no edges for. The tree must not read that verbatim:
/// the fault names the constraints it is about, so their placement's
/// row is the cause and every other row it reached reads as downstream
/// of it.
#[test]
fn a_refused_bundle_names_its_placement_and_reads_every_chained_row_downstream() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-refusal", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    common::asm::refuse_shelf(&mut session, &bench);

    let rows = session.tree_rows();
    assert!(tree::has_faults(&rows), "the shelf refused: {rows:?}");
    let status_of = |id| common::status_of(&rows, id);

    // The shelf, whose bundle refused, is the cause, and the only row
    // that is.
    let RowStatus::Failed { message, .. } = status_of(bench.shelf_i) else {
        panic!(
            "the refused placement carries the cause: {:?}",
            status_of(bench.shelf_i)
        );
    };
    assert!(
        message.contains("did not evaluate, so the bundle is not folded"),
        "the kernel's own words on the placement's row: {message}"
    );
    let causes: Vec<_> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Failed { .. }))
        .map(|row| row.id)
        .collect();
    assert_eq!(
        causes,
        vec![bench.shelf_i],
        "exactly one actionable row, and it is the placement the fault names"
    );

    // post_b, placed against the shelf, reads as downstream of it: the
    // POINTER, not a second copy of the refusal prose the shelf's row
    // already carries.
    match status_of(bench.post_b) {
        RowStatus::Poisoned { through, message } => {
            assert_eq!(through, bench.shelf_i, "post_b points at the shelf");
            assert_eq!(
                message,
                Some(downstream_at(bench.shelf_i)),
                "post_b points at the cause's row"
            );
            assert!(
                !message
                    .unwrap_or_default()
                    .contains("did not evaluate, so the bundle is not folded"),
                "post_b must not recite the refusal a user reads once, on the shelf"
            );
        }
        other => panic!("post_b must read as downstream, got {other:?}"),
    }
    assert_eq!(status_of(bench.post_b).badge(), "POISONED");
    // post_a is only read by the shelf's constraints: it keeps its pose.
    assert_eq!(status_of(bench.post_a), RowStatus::Ok);

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}

/// **A refused bundle blames its placement though it evaluated before.**
///
/// A rest places the shelf and EVALUATES — the memo now holds an
/// `Ok` for the shelf — and then the rest is taken off (the shelf given
/// a space of its own at the identity) and a mate the
/// evaluation refuses put in its place. The shelf's row reports the
/// refusal: a placement's content key carries its bundle, so the memo
/// cannot serve last evaluation's `Ok` into the run whose fault names
/// it.
#[test]
fn a_refused_bundle_badges_the_placement_that_evaluated_before_it() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-contradiction", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    common::asm::unpin(&mut session, bench.shelf_i);

    let held = common::commit_mate(
        &mut session,
        common::asm::shelf_rest_op(&bench, bench.post_a, 0.0),
    );
    assert_eq!(
        common::status_of(&session.tree_rows(), bench.shelf_i),
        RowStatus::Ok,
        "the premise: one rest places the shelf, which is no refusal"
    );
    // The rest off, by a space of the shelf's own at the identity.
    let removed = session.perform(SessionOp::IntoOwnSpace {
        placement: bench.shelf_i,
    });
    assert!(removed.refusal.is_none(), "{:?}", removed.refusal);
    let added = common::commit_mate(
        &mut session,
        common::asm::no_centre_op(&bench, bench.post_a),
    );
    assert_eq!(held.placement, added.placement, "one bundle held each");

    let rows = session.tree_rows();
    let status_of = |id| common::status_of(&rows, id);
    let failing: Vec<_> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Failed { .. }))
        .map(|row| row.id)
        .collect();
    assert_eq!(
        failing,
        vec![bench.shelf_i],
        "the placement whose bundle refused is the one cause: {rows:?}"
    );
    // The posts are in no chain from the shelf: independent subgraphs
    // complete, exactly as GQ2 says.
    assert_eq!(status_of(bench.post_a), RowStatus::Ok);
    assert_eq!(status_of(bench.post_b), RowStatus::Ok);

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}

/// **Two different faults in succession on one placement never serve
/// a stale one, and the repair re-runs on the faulted run as prior.**
///
/// The shelf's bundle refuses one mate where it evaluates, then, that
/// mate taken off (the shelf given a space of its own at the identity),
/// another against the other post. The memo serves only `Ok` priors, so
/// the second fault is the second mate's and never the first's again;
/// taking it off returns the shelf to `Ok`, re-run.
#[test]
fn two_faults_in_succession_on_one_placement_never_serve_a_stale_one() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-succession", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    common::asm::unpin(&mut session, bench.shelf_i);
    let fault = |session: &DocSession| match session
        .evaluation_arc()
        .expect("a result landed")
        .result(bench.shelf_i)
    {
        Some(NodeResult::Failed(err)) => format!("{:?}", err.kind),
        other => panic!("the shelf refuses, got {other:?}"),
    };

    let first = common::commit_mate(
        &mut session,
        common::asm::no_centre_op(&bench, bench.post_a),
    );
    let f1 = fault(&session);
    let removed = session.perform(SessionOp::IntoOwnSpace {
        placement: bench.shelf_i,
    });
    assert!(removed.refusal.is_none(), "{:?}", removed.refusal);
    let second = common::commit_mate(
        &mut session,
        common::asm::no_centre_op(&bench, bench.post_b),
    );
    assert_ne!(first, second);
    let f2 = fault(&session);
    assert_ne!(
        f1, f2,
        "a second, different fault, not the first served again"
    );
    assert!(matches!(
        common::status_of(&session.tree_rows(), bench.shelf_i),
        RowStatus::Failed { .. }
    ));

    let removed = session.perform(SessionOp::IntoOwnSpace {
        placement: bench.shelf_i,
    });
    assert!(removed.refusal.is_none(), "{:?}", removed.refusal);
    session.pump();
    let ev = session.evaluation_arc().expect("a result landed");
    assert!(
        matches!(ev.result(bench.shelf_i), Some(NodeResult::Ok(_))),
        "repaired: {:?}",
        ev.result(bench.shelf_i)
    );
    assert!(
        ev.recomputed >= 1,
        "the prior was Failed, so the placement re-ran"
    );
    assert_eq!(
        common::status_of(&session.tree_rows(), bench.shelf_i),
        RowStatus::Ok
    );

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}

/// **A row poisoned through a chained copy points at the terminal cause,
/// not at a POISONED row.**
///
/// The reachable document: a union of two instances' copies, one of
/// which is then chained to a refused bundle. Its mate reads the
/// shelf's copy, so post_b is poisoned through the shelf, and the
/// union, which reads both, through the shelf as well: the user's walk
/// is one hop, onto the row with the words.
#[test]
fn a_union_over_a_chained_copy_points_at_the_refused_placement() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-two-hop", tol);
    let boolean = common::asm::store_union(&bench, &[bench.post_b, bench.shelf_i], tol);
    let mut session = common::asm::open_bench(&bench, tol);
    assert_eq!(
        common::status_of(&session.tree_rows(), boolean),
        RowStatus::Ok,
        "the union builds before the shelf refuses"
    );

    common::asm::refuse_shelf(&mut session, &bench);

    // The kernel's own reading: post_b poisoned through the shelf its
    // mate reads, and the union through the shelf too.
    let (_, ev) = session.landed_pair().expect("landed");
    assert_eq!(
        ev.result(bench.post_b)
            .and_then(NodeResult::poisoned_through),
        Some(bench.shelf_i),
        "the evaluation poisons the chained copy through the refused shelf"
    );
    assert_eq!(
        ev.result(boolean).and_then(NodeResult::poisoned_through),
        Some(bench.shelf_i),
        "and the union through its nearest failed input"
    );

    let rows = session.tree_rows();
    match common::status_of(&rows, boolean) {
        RowStatus::Poisoned { through, message } => {
            assert_eq!(
                through, bench.shelf_i,
                "the union points at the placement that refused"
            );
            assert_eq!(message, Some(downstream_at(bench.shelf_i)));
        }
        other => panic!("the union reads as downstream, got {other:?}"),
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

/// **Every surface that says why a node has no value names the row the
/// tree names** — over one bundle refusal, the chained node and the
/// union that reads it.
///
/// The kernel's standing for `post_a`, whose mate reads the shelf's
/// copy, and for the union is poisoned through the refused shelf, and
/// the tree draws both downstream of it. The properties panel's verdict
/// on a picked face, the
/// mate tool's dropped pick, the duplicate door and the blend loader
/// each carry a standing, and each must
/// carry the TREE's. The product's gather refusal is drawn by none of
/// them: the tree badges the refused copy at its row, and the at-rest
/// badge takes no verdict on a product that did not gather.
#[test]
fn every_surface_names_the_row_the_tree_names_for_a_chained_node() {
    use pncad::document::{NodeStanding, ProductError};
    use pncad::select::{Resolution, ResolveIndeterminate};
    use viewer::blend::{BlendEvent, BlendTool};
    use viewer::combine::DuplicateFault;
    use viewer::matetool::{MateTool, MateToolEvent};
    use viewer::session::{FaceSelection, Selection, Standing};
    use viewer::tree::DrawnBody;

    let tol = Tol::witness();
    let bench = common::asm::bench("panels-agree", tol);
    let boolean = common::asm::store_union(&bench, &[bench.post_a, bench.shelf_i], tol);
    let mut session = common::asm::open_bench(&bench, tol);

    // A face of post_a, selected and held while it still resolves.
    let face = FaceSelection {
        name: bench.post_top.clone(),
        node: bench.post_a,
        body: 0,
    };
    let outcome = session.perform(SessionOp::Select(Selection::Face(face.clone())));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        session.standing().live(),
        "the premise: the picked face resolves before the shelf refuses: {:?}",
        session.standing()
    );
    let mut mate_tool = MateTool::new();
    mate_tool.pick(session.doc(), face.clone());
    let index = common::asm::index_of(&session);

    common::asm::refuse_shelf_chaining(&mut session, &bench, bench.post_a, bench.post_b);
    let offender = bench.shelf_i;

    let rows = session.tree_rows();
    let (doc, ev) = session.landed_pair().expect("landed");
    // The kernel's own standing for each node.
    assert_eq!(
        ev.usable(bench.post_a).err(),
        Some(NodeStanding::Poisoned {
            node: bench.post_a,
            through: offender
        }),
        "the kernel poisons the chained instance through the refused shelf"
    );
    assert_eq!(
        ev.usable(boolean).err(),
        Some(NodeStanding::Poisoned {
            node: boolean,
            through: offender
        }),
        "and the boolean through it too"
    );
    // The tree's answer, which every other surface must carry.
    let drawn = |id| match common::status_of(&rows, id) {
        RowStatus::Poisoned { through, .. } => NodeStanding::Poisoned { node: id, through },
        other => panic!("the tree draws {id:?} downstream, got {other:?}"),
    };
    let post_a = drawn(bench.post_a);
    assert_eq!(
        post_a,
        NodeStanding::Poisoned {
            node: bench.post_a,
            through: offender
        },
        "the tree points post_a at the refused shelf"
    );
    let boolean_drawn = drawn(boolean);
    assert_eq!(
        tree::cause_row(bench.post_a, ev),
        Some(offender),
        "cause_row"
    );

    let indeterminate = |standing| Resolution::Indeterminate(ResolveIndeterminate { standing });
    match session.standing() {
        Standing::Face { resolution, .. } => assert_eq!(
            resolution.as_deref(),
            Some(&indeterminate(post_a)),
            "the properties panel's verdict on the picked face"
        ),
        other => panic!("the face stays selected, got {other:?}"),
    }
    match mate_tool.reconcile(doc, ev).as_slice() {
        [MateToolEvent::PickLost { resolution, .. }, MateToolEvent::Reloaded { .. }] => assert_eq!(
            **resolution,
            indeterminate(post_a),
            "the mate tool's dropped pick"
        ),
        other => panic!(
            "the mate tool drops its one pick, then reloads the bundle the chaining changed, \
             got {other:?}"
        ),
    }
    // Each refusal carrying the standing whole says its nodes as the
    // document holds them: post_a by its kind, never `node <tag>`.
    match viewer::combine::duplicate_step(doc, ev, &bench.post_a.into(), tol) {
        Err(DuplicateFault::NoValue { standing, .. }) => {
            assert_eq!(standing, post_a, "the duplicate door, post_a");
        }
        other => panic!("the duplicate door refuses post_a, got {other:?}"),
    }
    match viewer::combine::duplicate_step(doc, ev, &boolean.into(), tol) {
        Err(DuplicateFault::NoValue { standing, .. }) => {
            assert_eq!(standing, boolean_drawn, "the duplicate door, the boolean");
        }
        other => panic!("the duplicate door refuses the boolean, got {other:?}"),
    }
    let target = DrawnBody::new(bench.post_a, 0);
    match BlendTool::new().load_all_edges(target, doc, ev, &index) {
        Some(BlendEvent::TargetHasNoValue { standing, .. }) => {
            assert_eq!(standing, post_a, "the blend loader");
        }
        other => panic!("the blend loader refuses post_a, got {other:?}"),
    }
    // The pick index holds post_a undrawn, with its kernel standing, and
    // badges nothing for it: the tree's row, downstream of the refused
    // shelf, is the notice.
    let index = common::index_at(&session, common::asm::delta())
        .expect("a placement with no value does not refuse the index");
    assert!(
        index
            .undrawn()
            .iter()
            .any(|copy| copy.node() == bench.post_a),
        "post_a is held undrawn: {:?}",
        index.undrawn()
    );
    assert_eq!(
        viewer::frame::index_badge(None, Some(&index), session.landed_pair()),
        None,
        "no badge beside the tree's row"
    );

    // The product gather: its value is the kernel's, the tree draws the
    // refused copy downstream of the refused shelf, and the at-rest badge says
    // nothing about it.
    let refused = match session.product_fault() {
        Some(ProductError::Read(
            NodeStanding::Failed { node } | NodeStanding::Poisoned { node, .. },
        )) => *node,
        other => panic!("the gather refuses on a placement, got {other:?}"),
    };
    assert!(
        matches!(common::status_of(&rows, refused), RowStatus::Poisoned { through, .. } if through == offender),
        "the refused copy is drawn downstream of the refused shelf"
    );
    assert_eq!(
        session.at_rest(),
        None,
        "the at-rest badge takes no verdict on a gather refusal; the tree's row carries it"
    );

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}

/// **Every kernel door under `crates/viewer/src` that hands back a
/// node's standing reads it through the tree's answer**, or is admitted
/// below by name with the reason it need not.
///
/// A source census over the code view. A door call counts as read when
/// it sits inside the argument list of a re-read (`*_as_drawn(…)`),
/// or when a re-read appears in the rest
/// of its own statement: from the call to the first `;` at the call's
/// own brace depth, or to the end of the enclosing block. What it cannot
/// see: a kernel door missing from `STANDING_DOORS`, and a door reached
/// through a helper in another crate.
#[test]
fn every_standing_door_in_the_viewer_reads_the_trees_answer() {
    use test_utils::source::{balanced_end, boundary_before, code_only, rust_sources};

    /// The kernel doors whose refusal carries a `NodeStanding`.
    const STANDING_DOORS: &[&str] = &[
        ".usable(",
        "resolve(RunCtx",
        "face_frame(",
        "face_carrier_kind(",
        "NodePick::build_all",
        "patch_names(",
        "boundary_names(",
        "product_recorded(",
        "product(",
        "pick_face(",
        "run_checks_at_rest_on(",
    ];
    /// The re-reads a door call may sit inside, or share a statement
    /// with.
    const REREADS: &[&str] = &[
        "standing_as_drawn(",
        "resolution_as_drawn(",
        "interrogation_as_drawn(",
    ];
    /// `(file, door, reason)`: each admits exactly one unread call.
    const ADMITTED: &[(&str, &str, &str)] = &[
        (
            "blend.rs",
            ".usable(",
            "asks only `is_err()` and draws nothing; the node's row carries it",
        ),
        (
            "tree.rs",
            ".usable(",
            "`readout_of` is asked only of a row `rows` has read `Ok`: it picks which payload \
             to show, never whether the row stands",
        ),
        (
            "tree.rs",
            ".usable(",
            "`split_reading` reads a staged split, which has no row: the split panel's verdict \
             before commit, said in the panel and nowhere the tree draws",
        ),
        (
            "features.rs",
            ".usable(",
            "a unit test's premise: reads the measure's payload to name the reason it expects, \
             and draws nothing",
        ),
        (
            "pickindex.rs",
            "NodePick::build_all",
            "held as an `UndrawnBody`; `frame::index_badge` says only a copy `tree::cause_row` \
             names no row for, whose standing the tree draws unchanged",
        ),
        (
            "pickindex.rs",
            "NodePick::build_all",
            "the memoised build; the same route as the one above",
        ),
        (
            "pickindex.rs",
            "patch_names(",
            "a `PickIndexError::Names` refuses the whole index, and `frame::index_badge` \
             says it Actionable in its own words: louder than the tree, never quieter",
        ),
        (
            "pickindex.rs",
            "boundary_names(",
            "a `PickIndexError::Names` refuses the whole index, and `frame::index_badge` \
             says it Actionable in its own words: louder than the tree, never quieter",
        ),
        (
            "pickindex.rs",
            "pick_face(",
            "`HitTestError::Standing` cannot arise: the parts are built from the evaluation \
             the ray is asked of",
        ),
        (
            "pickindex.rs",
            "pick_face(",
            "the moved instances' pass; the same argument as the one above",
        ),
        (
            "scene.rs",
            "product_recorded(",
            "`scene_of`'s headless path; no chrome surface draws its refusal",
        ),
        (
            "scene.rs",
            "product(",
            "runs only over a pair whose gather already succeeded, so no placement \
             refusal reaches it",
        ),
        (
            "session.rs",
            "product_recorded(",
            "the landing keeps the typed refusal (`product_fault`), and `frame::badge_site` \
             sends a placement's refusal to the feature tree, so no surface draws its \
             standing",
        ),
        (
            "session.rs",
            "run_checks_at_rest_on(",
            "the registry's refusal is dropped (`.ok()`), never drawn",
        ),
        (
            "session.rs",
            "run_checks_at_rest_on(",
            "the no-body subject's registry run; dropped the same way",
        ),
        (
            "session.rs",
            ".usable(",
            "`body_reach` sizes a trace's window off the body's value; a refusal is dropped \
             (`.ok()`) and the window falls back to a unit reach, never drawn",
        ),
    ];

    /// The end of the statement a call at `at` belongs to: the first
    /// `;` at its own brace depth, or the close of its enclosing block.
    fn statement_end(code: &str, at: usize) -> usize {
        let mut depth = 0i32;
        for (off, c) in code[at..].char_indices() {
            match c {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => {
                    depth -= 1;
                    if depth < 0 {
                        return at + off;
                    }
                }
                ';' if depth == 0 => return at + off,
                _ => {}
            }
        }
        code.len()
    }

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut unread: Vec<(String, &str, String)> = Vec::new();
    let mut calls = 0;
    for path in rust_sources(&root) {
        let text = std::fs::read_to_string(&path).expect("a source file reads");
        let code = code_only(&text);
        let file = path
            .file_name()
            .expect("a file name")
            .to_string_lossy()
            .into_owned();
        // Every re-read's argument span, as byte ranges of `code`.
        let spans: Vec<std::ops::Range<usize>> = REREADS
            .iter()
            .flat_map(|reread| {
                code.match_indices(reread)
                    .map(|(at, _)| at + reread.len() - 1)
            })
            .map(|open| open..balanced_end(&code, open).expect("a re-read's call closes"))
            .collect();
        for door in STANDING_DOORS {
            for (at, _) in code.match_indices(door) {
                // A door is a whole word; `product(` called as a method is
                // `Doc::product`, the slot's accessor, not the gather.
                if door.starts_with(|c: char| c.is_alphabetic())
                    && (!boundary_before(&code, at)
                        || (*door == "product(" && code[..at].ends_with('.')))
                {
                    continue;
                }
                calls += 1;
                let inside = spans.iter().any(|span| span.contains(&at));
                let rest = &code[at..statement_end(&code, at)];
                let shares = REREADS.iter().any(|reread| rest.contains(reread));
                if !inside && !shares {
                    let line = test_utils::source::line(&code, at);
                    unread.push((file.clone(), door, format!("{}:{line}", path.display())));
                }
            }
        }
    }
    assert!(
        calls >= 16,
        "the census found the doors it names ({calls} calls)"
    );

    let mut admitted: Vec<(&str, &str)> = ADMITTED.iter().map(|(f, d, _)| (*f, *d)).collect();
    admitted.sort_unstable();
    let mut found: Vec<(&str, &str)> = unread.iter().map(|(f, d, _)| (f.as_str(), *d)).collect();
    found.sort_unstable();
    assert_eq!(
        found, admitted,
        "every standing door reads through the tree's answer or is admitted by name; \
         unread calls: {unread:#?}"
    );
}

/// **A document whose only non-`Ok` row is POISONED is not
/// building** — the axis `tree::has_faults` reads, and the one that
/// is easy to collapse into [`RowStatus::tone`]'s.
///
/// Every other `has_faults` assertion in this tree stands on a
/// document that also carries a `Failed` row, so dropping `Poisoned`
/// from the fault policy — or rewriting that policy as "any
/// actionable row", which is the natural-looking collapse — reddens
/// nowhere but here.
///
/// **Which fixes the shape of the fixture.** A `Poisoned` row's
/// `through` names a row THIS TREE badges `Failed`
/// ([`RowStatus::Poisoned`]'s own doc), so no tree can hold a poisoned
/// row, a healthy cause and nothing else — and a tree that did hold a
/// `Failed` row would satisfy the policy through that row instead,
/// which is the case already covered. The one shape that carries this
/// claim is therefore the one the tree draws when the chain does NOT
/// end at a failure: `Poisoned { message: None }`, which
/// `tree::poisoned_through` mints for exactly that, reporting the
/// broken invariant as absence rather than inventing a cause. `None`
/// is the honest message here for the same reason: the wording
/// `tree::downstream_wording` writes points at a row that carries the
/// cause, and no row here does.
#[test]
fn a_downstream_failure_alone_is_a_fault_the_reader_cannot_act_on() {
    use pncad::document::RecipeNodeId;
    use viewer::test_support::spoken;

    let row = |id: u64, status: RowStatus| tree::TreeRow {
        id: RecipeNodeId::new(0, id),
        spoken: spoken(RecipeNodeId::new(0, id), Some("Place")),
        which: None,
        placed: false,
        status,
        repair_at: None,
        readout: None,
        version_offer: None,
        constraints: Vec::new(),
    };
    let rows = [
        row(1, RowStatus::Ok),
        row(
            2,
            RowStatus::Poisoned {
                through: RecipeNodeId::new(0, 1),
                message: None,
            },
        ),
    ];
    assert!(
        tree::has_faults(&rows),
        "a row showing someone else's failure still says the document is not building: {rows:?}"
    );
}

/// **An empty value says so on its own row, and the node that refuses
/// it links nowhere.** Each failing arm below names an input that
/// evaluated to a legal value the failing node's own choice does not
/// fit — nothing on the input refused — so its row carries no repair
/// link, and the input's row says what its value is where that is
/// the cause.
///
/// One document, built up: an intersect of two blocks that do not
/// meet, first alone and then under a shell; a
/// split whose tool plane clears the block, with a shell reading its
/// empty side; and a member read indexed past a family's count.
#[test]
fn an_empty_value_reads_empty_and_the_node_refusing_it_links_nowhere() {
    use pncad::document::{
        BooleanValue, Doc, Node, NodeErrorKind, ProfileProgram, RecipeNodeId, SplitSide,
        TargetAxis, ValuePayload,
    };
    use pncad::select::SplitHalf;
    use viewer::tree::{Emptiness, Readout};

    let tol = Tol::witness();
    let run = |doc: &Doc<ProfileProgram>| {
        let ev = evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
        let rows = tree::rows(doc, Some(&ev), &viewer::parts::PartFiles::default());
        (ev, rows)
    };

    let doc: Doc<ProfileProgram> = Doc::empty_derived("tree-empty-readout", tol);
    let block = |doc: &Doc<ProfileProgram>, x: f64| {
        let (doc, profile) = common::inserted(doc, common::rectangle([x, 0.0], 0.02, 0.02), tol);
        common::inserted(
            &doc,
            Node::Extrude {
                profile: profile.into(),
                distance: common::len(0.02),
            },
            tol,
        )
    };
    let (doc, near) = block(&doc, 0.0);
    let (doc, far) = block(&doc, 0.1);
    // The far block in the near one's space, where it was drawn: the
    // operand every combine below reads for it.
    let (doc, far_placed) = crate::fixture::slid(doc, near, far, [0.0; 3]);
    let (doc, apart) = common::inserted(
        &doc,
        Node::Intersect {
            members: Bodies::Spelled(vec![near.into(), far_placed.into()]),
        },
        tol,
    );

    // The empty intersect: its row is the only place a reader learns
    // it makes nothing.
    let (ev, rows) = run(&doc);
    assert!(
        matches!(
            ev.usable(apart).ok().map(|value| &value.payload),
            Some(ValuePayload::Boolean(BooleanValue::Empty))
        ),
        "the fixture's blocks do not meet: {:?}",
        ev.result(apart)
    );
    let row = common::row_of(&rows, apart);
    assert!(matches!(row.status, RowStatus::Ok), "{row:?}");
    assert_eq!(
        row.readout,
        Some(Readout::Empty(Emptiness::Whole)),
        "an empty boolean says so on its own row"
    );
    assert_eq!(
        Emptiness::Whole.to_string(),
        "empty",
        "the phrase a reader sees"
    );
    assert_eq!(
        common::row_of(&rows, near).readout,
        None,
        "a body's `Ok` says everything its value does"
    );

    // A consumer of the empty value: `EmptyOperand`.
    let shell_of = |doc: &Doc<ProfileProgram>, input: pncad::document::Operand| {
        common::inserted(doc, Node::shell(input, common::len(0.001), Vec::new()), tol)
    };
    let (doc, moved) = shell_of(&doc, apart.into());

    // A tool plane above the block, read off its own geometry: the
    // split's above side is empty, and a shell reading it refuses
    // `EmptyHalf`.
    let tool =
        editor_core::test_support::plane_on(&doc, near, [0.0, 0.0, 0.05], [0.0, 0.0, 1.0], tol);
    let (doc, split) = common::inserted(
        &doc,
        Node::Split {
            target: near.into(),
            tool,
        },
        tol,
    );
    let (doc, above) = shell_of(
        &doc,
        pncad::document::Operand::output(split, SplitHalf::Above.port()),
    );

    // Beside them, values with material: a subtract that leaves a
    // body, and a split whose tool plane cuts the block in two.
    let (doc, kept) = common::inserted(
        &doc,
        Node::Subtract {
            from: near.into(),
            tool: far_placed.into(),
        },
        tol,
    );
    let through =
        editor_core::test_support::plane_on(&doc, near, [0.0, 0.0, 0.01], [0.0, 0.0, 1.0], tol);
    let (doc, halved) = common::inserted(
        &doc,
        Node::Split {
            target: near.into(),
            tool: through,
        },
        tol,
    );

    // A three-member family, and a read of member 3.
    let (doc, pattern) = common::row_of_copies(&doc, far, 3, (TargetAxis::X, 0.05), tol);
    let (doc, past) = common::member_read(&doc, pattern, 3, tol);

    let (ev, rows) = run(&doc);
    assert!(
        matches!(
            ev.usable(split).ok().map(|value| &value.payload),
            Some(ValuePayload::Split {
                above: SplitSide::Empty,
                below: SplitSide::Body(_),
            })
        ),
        "the fixture's tool plane clears the block: {:?}",
        ev.result(split)
    );
    assert_eq!(
        common::row_of(&rows, split).readout,
        Some(Readout::Empty(Emptiness::Half(SplitHalf::Above))),
        "a split with an empty side says which"
    );
    assert_eq!(
        Emptiness::Half(SplitHalf::Above).to_string(),
        "above half empty"
    );
    assert_eq!(
        Emptiness::Half(SplitHalf::Below).to_string(),
        "below half empty"
    );
    // The readout's half word is the kernel's, as `EmptyHalf` says it.
    for half in SplitHalf::ALL {
        let phrase = Emptiness::Half(half).to_string();
        let word = phrase
            .strip_suffix(" half empty")
            .expect("the readout ends in its suffix");
        let kernel = NodeErrorKind::EmptyHalf {
            input: RecipeNodeId::new(0, 0),
            half,
        }
        .to_string();
        assert!(
            kernel.contains(&format!("the split's {word} half")),
            "{half:?}: the readout says {phrase:?}, the kernel {kernel:?}"
        );
    }
    assert_eq!(
        common::row_of(&rows, apart).readout,
        Some(Readout::Empty(Emptiness::Whole)),
        "the intersect still says so under a shell"
    );
    assert_eq!(
        common::row_of(&rows, pattern).readout,
        None,
        "a family's count is authored, and the refusal's words state it"
    );
    assert!(
        matches!(
            ev.usable(kept).ok().map(|value| &value.payload),
            Some(ValuePayload::Boolean(BooleanValue::Body { .. }))
        ),
        "the subtract leaves the block: {:?}",
        ev.result(kept)
    );
    assert_eq!(
        common::row_of(&rows, kept).readout,
        None,
        "a boolean with a body says everything its value does"
    );
    assert!(
        matches!(
            ev.usable(halved).ok().map(|value| &value.payload),
            Some(ValuePayload::Split {
                above: SplitSide::Body(_),
                below: SplitSide::Body(_),
            })
        ),
        "the tool plane cuts the block: {:?}",
        ev.result(halved)
    );
    assert_eq!(
        common::row_of(&rows, halved).readout,
        None,
        "a split with two bodies says everything its value does"
    );

    // The arm each row raised, among the three under test.
    let raised = |id| match ev.node_error(id).map(|error| &error.kind) {
        Some(NodeErrorKind::EmptyOperand { .. }) => Some("EmptyOperand"),
        Some(NodeErrorKind::EmptyHalf { .. }) => Some("EmptyHalf"),
        Some(NodeErrorKind::MemberOutOfRange { .. }) => Some("MemberOutOfRange"),
        _ => None,
    };
    let arms = [
        (moved, "EmptyOperand"),
        (above, "EmptyHalf"),
        (past, "MemberOutOfRange"),
    ];
    for (id, arm) in arms {
        assert_eq!(
            raised(id),
            Some(arm),
            "the fixture raises {arm}: {:?}",
            ev.result(id)
        );
        let failed = common::row_of(&rows, id);
        assert!(
            matches!(failed.status, RowStatus::Failed { .. }),
            "{arm}: {failed:?}"
        );
        assert_eq!(
            failed.repair_at, None,
            "{arm}: the input it names refused nothing, so the row links nowhere"
        );
    }
}

/// **A placement's row carries its constraints as children**, in bundle
/// order, each by its address — a mate and the values on the freedoms
/// it leaves; the world, which every document holds and nothing
/// deletes, has no row; and removing the mate through its child row's
/// op writes the bundle whole without it, each freedom it fixed written
/// `0`, so the copy stays pinned and the row carries values alone.
#[test]
fn a_placement_row_carries_its_constraints_as_children() {
    use viewer::tree::ConstraintRow;

    let tol = Tol::witness();
    let bench = common::asm::bench("auth16-children", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    common::asm::unpin(&mut session, bench.post_b);
    let seat = common::commit_mate(
        &mut session,
        common::asm::seat_op(&bench, bench.post_b, common::asm::middle_seat_values()),
    );

    let rows = session.tree_rows();
    assert!(
        rows.iter()
            .all(|row| row.id != pncad::document::RecipeNodeId::WORLD),
        "the world has no row"
    );
    let ids: Vec<_> = crate::fixture::bundle(session.committed_doc(), bench.post_b)
        .iter()
        .map(|c| pncad::document::MateRef::new(bench.post_b, c.id))
        .collect();
    assert_eq!(ids.first(), Some(&seat), "the seat leads post_b's bundle");
    assert_eq!(
        common::row_of(&rows, bench.post_b).constraints,
        ids.into_iter()
            .zip([
                "constraint 1: mate",
                "constraint 2: slide along x value",
                "constraint 3: slide along y value",
                "constraint 4: spin value",
            ])
            .map(|(mate, said)| ConstraintRow {
                mate,
                said: said.to_owned(),
                is_mate: said.ends_with(": mate"),
            })
            .collect::<Vec<_>>(),
        "post_b's row carries the seat that places it, and its values"
    );
    let shelf: Vec<_> = crate::fixture::bundle(session.committed_doc(), bench.shelf_i)
        .iter()
        .enumerate()
        .map(|(at, c)| ConstraintRow {
            mate: pncad::document::MateRef::new(bench.shelf_i, c.id),
            is_mate: false,
            said: format!(
                "constraint {}: {} value",
                at + 1,
                c.value().expect("the shelf's pin is values").coord.name()
            ),
        })
        .collect();
    assert_eq!(shelf.len(), 6, "the shelf's pin sets every freedom");
    assert_eq!(
        common::row_of(&rows, bench.shelf_i).constraints,
        shelf,
        "the shelf's row carries the values it stands on"
    );

    let outcome = session.perform(SessionOp::RemoveConstraint { mate: seat });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let bundle = crate::fixture::bundle(session.committed_doc(), bench.post_b);
    assert!(
        bundle.iter().all(|c| c.value().is_some()),
        "the seat is gone, the copy pinned by values: {bundle:?}"
    );
    let rows = session.tree_rows();
    assert_eq!(
        common::row_of(&rows, bench.post_b).constraints.len(),
        bundle.len(),
        "the row carries the bundle the door wrote"
    );
    assert_eq!(common::status_of(&rows, bench.post_b), RowStatus::Ok);
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// What a row downstream of a refused placement says, spelled from the
/// instance's id alone: the instance as the document speaks it.
fn downstream_at(instance: pncad::document::RecipeNodeId) -> String {
    format!(
        "upstream failure at InstantiatePart {} — that row carries the cause",
        test_utils::refusal::tag(instance.0.digest())
    )
}
