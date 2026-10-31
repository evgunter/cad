//! Node labels in the viewer (DESIGN.md Band 1, "Node labels"): a
//! rename is one `SetLabel` and one undo, a labelled creation is the
//! insert and its label as one undo, a labelled row leads with its
//! label, and a create form proposes `Kind N` counted among that kind's
//! nodes.

#![allow(clippy::expect_used, clippy::panic)]

use crate::common;
use pncad::document::{Dimension, Formula};
use viewer::session::BooleanSpec;

use pncad::document::{Doc, DocEdit, Label, Node, ProfileProgram, RecipeNodeId, SlotId};
use pncad::geom_core::Tol;
use test_utils::refusal::tag;
use viewer::session::{Creation, DocSession, SessionOp};
use viewer::tree::{Headline, headline, proposed_label};

fn label(text: &str) -> Label {
    Label::new(text).expect("a valid label")
}

/// A square and its extrude: the document and the extrude.
fn extruded(seed: &str, tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived(seed, tol);
    let (doc, profile) = common::square_profile(&doc, 0.02, tol);
    common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: common::len(0.01),
        },
        tol,
    )
}

fn relabelled(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    text: &str,
    tol: Tol,
) -> Doc<ProfileProgram> {
    let (doc, _) = common::edited(
        doc,
        DocEdit::SetLabel {
            node,
            label: Some(label(text)),
        },
        tol,
    );
    doc
}

/// A rename is one edit and one undo; writing the label the node
/// already has is no action and costs no history state; clearing is
/// the same door with `None`.
#[test]
fn a_rename_is_one_undo_and_the_same_label_again_is_no_action() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-rename", tol);
    let mut session = DocSession::inline(doc, tol);
    let rename = |text: Option<&str>| SessionOp::SetLabel {
        node: extrude,
        label: text.map(label),
    };

    let renamed = session.perform(rename(Some("base plate")));
    assert!(renamed.refusal.is_none(), "{:?}", renamed.refusal);
    assert_eq!(renamed.committed.len(), 1, "one edit");
    assert_eq!(session.doc().label(extrude), Some(&label("base plate")));

    let again = session.perform(rename(Some("base plate")));
    assert!(
        again.committed.is_empty() && again.refusal.is_none(),
        "the same label again commits nothing and refuses nothing: {again:?}"
    );

    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    assert_eq!(
        session.doc().label(extrude),
        None,
        "one undo takes the rename back"
    );

    session.perform(SessionOp::Redo);
    let cleared = session.perform(rename(None));
    assert_eq!(cleared.committed.len(), 1, "clearing is one edit");
    assert_eq!(session.doc().label(extrude), None);
}

/// A labelled creation commits the insert and then the label, and one
/// undo takes both back.
#[test]
fn a_labelled_creation_is_the_insert_and_its_label_as_one_undo() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("viewer-node-labels-create", tol);
    let mut session = DocSession::inline(doc, tol);
    let creation = Creation::of(SessionOp::AddProfile {
        loops: vec![common::rectangle_loop([0.0, 0.0], 0.01, 0.01)],
    })
    .expect("adding a profile creates a node");
    let outcome = session.perform(SessionOp::CreateLabelled {
        creation,
        label: label("floor"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [profile] = outcome.minted[..] else {
        panic!("one node minted, got {:?}", outcome.minted);
    };
    assert!(
        matches!(
            &outcome.committed[..],
            [DocEdit::InsertNode { .. }, DocEdit::SetLabel { node, label: Some(l) }]
                if *node == profile && *l == label("floor")
        ),
        "the insert, then its label: {:?}",
        outcome.committed
    );
    assert_eq!(session.doc().label(profile), Some(&label("floor")));

    session.perform(SessionOp::Undo);
    assert!(
        session.doc().node(profile).is_none(),
        "one undo takes the node back"
    );
    assert!(session.doc().labels().is_empty(), "and its label with it");
}

/// Only an operation that creates a node can be labelled.
#[test]
fn only_a_creation_can_be_labelled() {
    let refused = Creation::of(SessionOp::Undo);
    assert!(
        matches!(refused.as_ref().map_err(|op| &**op), Err(SessionOp::Undo)),
        "undo creates nothing, and comes back"
    );
}

/// A labelled row leads with its label and mutes its kind and tag; an
/// unlabelled one reads as the spoken node and its pose.
#[test]
fn a_labelled_rows_headline_is_its_label_with_kind_and_tag_muted() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-headline", tol);
    let t = tag(extrude.0.digest());
    assert_eq!(
        headline(&doc.spoken(extrude), None),
        Headline {
            lead: format!("Extrude {t}"),
            muted: None,
        }
    );
    let doc = relabelled(&doc, extrude, "base plate", tol);
    assert_eq!(
        headline(&doc.spoken(extrude), Some("unused pose")),
        Headline {
            lead: "base plate".to_owned(),
            muted: Some(format!("Extrude {t}")),
        }
    );
}

/// The proposal counts the live nodes of its kind, labelled or not,
/// steps past any `Kind N` a node of that kind already carries, and
/// ignores other kinds' labels.
#[test]
fn a_create_form_proposes_kind_n_counted_among_that_kinds_nodes() {
    let tol = Tol::witness();
    let (doc, first) = extruded("viewer-node-labels-proposal", tol);
    let proposal = |doc: &Doc<ProfileProgram>| {
        proposed_label(doc, "Extrude").map(|label| label.as_str().to_owned())
    };
    assert_eq!(proposal(&doc).as_deref(), Some("Extrude 2"), "one extrude");
    let another = |doc: &Doc<ProfileProgram>| {
        let (doc, profile) = common::square_profile(doc, 0.01, tol);
        let (doc, extrude) = common::inserted(
            &doc,
            Node::Extrude {
                profile: profile.into(),
                distance: common::len(0.02),
            },
            tol,
        );
        (doc, profile, extrude)
    };
    let (doc, _, second) = another(&doc);
    let (doc, profile, _) = another(&doc);
    assert_eq!(
        proposal(&doc).as_deref(),
        Some("Extrude 4"),
        "three unlabelled extrudes"
    );
    let doc = relabelled(&doc, first, "base", tol);
    assert_eq!(
        proposal(&doc).as_deref(),
        Some("Extrude 4"),
        "a label does not change the count"
    );
    let doc = relabelled(&doc, second, "Extrude 4", tol);
    assert_eq!(
        proposal(&doc).as_deref(),
        Some("Extrude 5"),
        "`Extrude 4` is taken by an extrude"
    );
    let doc = relabelled(&doc, profile, "Extrude 5", tol);
    assert_eq!(
        proposal(&doc).as_deref(),
        Some("Extrude 5"),
        "a profile's label is not an extrude's"
    );
    assert_eq!(
        proposed_label(&doc, "Ex\ttrude"),
        None,
        "a noun no label can hold proposes nothing"
    );
}

/// **`op`, labelled `text`, as one recorded action**: the creation is
/// performed alone first, and undone, so that its own outcome is the
/// yardstick. Labelled, it commits through whatever door it takes and
/// its outcome is that same action's with one more edit: the same edits
/// in the same order and then a `SetLabel` on the node it made
/// (`viewer::world::made`: the last it minted that is not a world
/// placement), the same minted ids and the same netted maintenance, and
/// ONE evaluation requested for the whole of it. The label is on that
/// node, and ONE undo returns the document to what it was before the
/// action. Answers that node.
fn labelled_as_one_undo(session: &mut DocSession, op: SessionOp, text: &str) -> RecipeNodeId {
    let before = session.committed_doc().clone();
    let what = format!("{op:?}");
    let alone = session.perform(op.clone());
    assert!(alone.refusal.is_none(), "{what} alone: {:?}", alone.refusal);
    session.perform(SessionOp::Undo);
    session.pump();
    let creation = Creation::of(op).unwrap_or_else(|op| panic!("{op:?} creates a node"));
    let requested = session.generation();
    let outcome = session.perform(SessionOp::CreateLabelled {
        creation,
        label: label(text),
    });
    assert!(outcome.refusal.is_none(), "{what}: {:?}", outcome.refusal);
    assert_eq!(
        session.generation(),
        requested.next(),
        "{what}: one action requests one evaluation"
    );
    let node = viewer::world::made(session.committed_doc(), &outcome.minted)
        .unwrap_or_else(|| panic!("{what} made a node"));
    let labelled = DocEdit::SetLabel {
        node,
        label: Some(label(text)),
    };
    let expected: Vec<_> = alone.committed.iter().cloned().chain([labelled]).collect();
    assert_eq!(
        outcome.committed, expected,
        "{what}: the creation's own edits in order, then its label"
    );
    assert_eq!(outcome.minted, alone.minted, "{what}: the same ids minted");
    assert_eq!(
        outcome.maintenance, alone.maintenance,
        "{what}: the label nets no maintenance of its own"
    );
    assert_eq!(
        session.committed_doc().label(node),
        Some(&label(text)),
        "{what}"
    );
    session.perform(SessionOp::Undo);
    assert!(
        session.committed_doc().bit_eq(&before),
        "{what}: one undo takes the creation and its label back"
    );
    session.perform(SessionOp::Redo);
    assert_eq!(
        session.committed_doc().label(node),
        Some(&label(text)),
        "{what}: redo brings both back"
    );
    session.pump();
    node
}

/// **A refused creation applies no label**, whether the door refuses
/// before its run (an extrude of a node that is no profile) or the run
/// refuses inside it (a profile of no loops, which the edit door
/// refuses): nothing is committed, the label included, no history state
/// is recorded and no evaluation is requested.
#[test]
fn a_refused_creation_applies_no_label() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-refused", tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let before = session.committed_doc().clone();
    let refused = [
        SessionOp::AddExtrude {
            profile: extrude,
            distance: common::len(0.01),
        },
        SessionOp::AddProfile { loops: Vec::new() },
    ];
    for op in refused {
        let what = format!("{op:?}");
        let creation = Creation::of(op).unwrap_or_else(|op| panic!("{op:?} creates a node"));
        let requested = session.generation();
        let outcome = session.perform(SessionOp::CreateLabelled {
            creation,
            label: label("never"),
        });
        assert!(outcome.refusal.is_some(), "{what}: refused");
        assert!(
            outcome.committed.is_empty() && outcome.minted.is_empty(),
            "{what}: nothing committed: {:?}",
            outcome.committed
        );
        assert!(
            session.committed_doc().bit_eq(&before) && session.committed_doc().labels().is_empty(),
            "{what}: the document is as it was, with no label"
        );
        assert!(!session.history().can_undo(), "{what}: no history state");
        assert_eq!(
            session.generation(),
            requested,
            "{what}: nothing to evaluate"
        );
    }
}

/// **Every creating op, labelled, is one undo** — whichever commit door
/// it takes, since every door stages its run through one recorder.
/// The sample set is held to `SessionOp::creates_a_node` through the
/// gesture table's every-op roster: a creation added there and not
/// sampled here is red.
#[test]
fn every_creation_labelled_is_one_undo_whatever_door_commits_it() {
    use common::{ang, asm, len, len3, scl3};
    use viewer::session::{FaceSelection, PartSelectSpec, PatternRuleSpec};

    let tol = Tol::witness();
    let bench = asm::bench("node-labels-every-creation", tol);
    let mut session = asm::open_bench(&bench, tol);
    let mut sampled = std::collections::BTreeSet::new();
    let mut run = |session: &mut DocSession, op: SessionOp, text: &str| {
        sampled.insert(format!("{:?}", std::mem::discriminant(&op)));
        labelled_as_one_undo(session, op, text)
    };

    let profile = run(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![common::rectangle_loop([0.0, 0.0], 0.02, 0.02)],
        },
        "profile",
    );
    let block = run(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.02),
        },
        "block",
    );
    run(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            // An axis written in the profile's own frame.
            axis_through: common::len2([0.03, 0.0]),
            axis_heading: pncad::document::Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(core::f64::consts::PI),
        },
        "turned",
    );
    let moved = run(
        &mut session,
        SessionOp::AddTransform {
            input: block.into(),
            translation: len3([0.005, 0.007, 0.003]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
        "moved",
    );
    run(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Intersect(vec![block.into(), moved.into()]),
        },
        "overlap",
    );
    run(
        &mut session,
        SessionOp::AddSplit {
            target: block.into(),
            origin: len3([0.0, 0.0, 0.01]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
        "halves",
    );
    session.pump();
    run(
        &mut session,
        SessionOp::SketchOnFace {
            face: FaceSelection {
                name: pncad::prelude::StableName {
                    kind: pncad::prelude::EntityKind::Face,
                    node: block,
                    path: vec![pncad::prelude::RoleSeg::Cap(pncad::prelude::CapEnd::End)],
                },
                node: block,
                body: 0,
            },
            loops: vec![common::rectangle_loop([0.0, 0.0], 0.005, 0.005)],
            distance: len(0.005),
            spin: Formula::zero(Dimension::Angle),
        },
        "boss",
    );
    // A footprint drawn in a cylinder's base, running onto its wall's
    // trace and along it.
    let disc = run(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![pncad::document::LoopProgram::circle(0.1, 0.0, 0.01).expect("a disc")],
        },
        "disc",
    );
    let cylinder = run(
        &mut session,
        SessionOp::AddExtrude {
            profile: disc,
            distance: len(0.02),
        },
        "cylinder",
    );
    session.pump();
    let wall = {
        let ev = session.evaluation().expect("landed");
        pncad::select::all_faces(ev, cylinder)
            .into_iter()
            .find(|name| {
                pncad::select::face_carrier_kind(ev, cylinder, name)
                    == Ok(pncad::prelude::SurfaceKind::Cylinder)
            })
            .expect("a wall")
    };
    let pick = |name| FaceSelection {
        name,
        node: cylinder,
        body: 0,
    };
    run(
        &mut session,
        SessionOp::AddDrawnProfile {
            plane: pick(pncad::prelude::StableName {
                kind: pncad::prelude::EntityKind::Face,
                node: cylinder,
                path: vec![pncad::prelude::RoleSeg::Cap(pncad::prelude::CapEnd::Start)],
            }),
            traced: vec![pick(wall)],
            chains: vec![
                pncad::trace::Chain::at(len(0.0), len(-0.005))
                    .toward(
                        Formula::scalar(0.0).expect("finite"),
                        Formula::scalar(1.0).expect("finite"),
                    )
                    .onto_trace(
                        0,
                        pncad::document::TraceCrossing::first(pncad::document::TracePassage::Leave),
                        pncad::document::TraceSense::Against,
                    )
                    .line(len(0.02))
                    .line_to(pncad::document::ProgramTarget::Point([
                        len(0.02),
                        len(-0.005),
                    ]))
                    .line_to(pncad::document::ProgramTarget::Start),
            ],
        },
        "traced",
    );
    let row = || PatternRuleSpec::Linear {
        direction: scl3([1.0, 0.0, 0.0]),
        spacing: len(0.05),
    };
    let pattern = run(
        &mut session,
        SessionOp::AddPattern {
            input: block.into(),
            count: 2,
            rule: row(),
        },
        "row",
    );
    run(
        &mut session,
        SessionOp::AddFusedPattern {
            input: block.into(),
            count: 2,
            rule: row(),
        },
        "fused row",
    );
    run(
        &mut session,
        SessionOp::AddPart {
            of: pattern,
            select: PartSelectSpec::Instance(1),
        },
        "second",
    );
    run(
        &mut session,
        SessionOp::Duplicate {
            input: block.into(),
            beside: None,
        },
        "copy",
    );
    run(
        &mut session,
        SessionOp::AddMirror {
            body: block.into(),
            plane: FaceSelection {
                name: pncad::prelude::StableName {
                    kind: pncad::prelude::EntityKind::Face,
                    node: block,
                    path: vec![pncad::prelude::RoleSeg::Cap(pncad::prelude::CapEnd::End)],
                },
                node: block,
                body: 0,
            },
        },
        "image",
    );
    let edges = pncad::select::all_edges(session.evaluation().expect("landed"), block);
    run(
        &mut session,
        SessionOp::AddFillet {
            target: block,
            radius: len(0.001),
            selection: edges.clone(),
        },
        "rounded",
    );
    run(
        &mut session,
        SessionOp::AddChamfer {
            target: block,
            distance: len(0.001),
            selection: edges,
        },
        "bevelled",
    );
    run(
        &mut session,
        SessionOp::AddInstance { id: bench.post.id },
        "third post",
    );
    // A mate is a constraint in a placement's bundle and makes no node,
    // so it is no creation and takes no label.
    assert!(
        Creation::of(asm::seat_op(
            &bench,
            bench.post_b,
            asm::middle_seat_values(),
        ))
        .is_err(),
        "a mate creates no node"
    );

    let dir = common::tempdir("node-labels-every-creation-roster");
    let roster: std::collections::BTreeSet<String> =
        crate::gesture_table::every_op(block, &dir.join("unused.pncad"))
            .iter()
            .filter(|op| op.creates_a_node())
            .map(|op| format!("{:?}", std::mem::discriminant(op)))
            .collect();
    std::fs::remove_dir_all(&dir).expect("the roster directory is removable");
    assert_eq!(
        sampled, roster,
        "every creating op is sampled, and only those"
    );
    std::fs::remove_dir_all(&bench.dir).expect("the bench directory is removable");
}

/// **A failed row speaks its node as the document holds it now**: the
/// kind, the label and the tag, read off the document when the row is
/// drawn and never out of the evaluation. The same evaluation drawn
/// over the document before and after a rename says each document's
/// label, so a label captured into the failure when it was raised
/// fails here. No rerun can hide that: none happens between the two
/// draws. Through the session, a rename lands a row with the new label
/// and changes nothing else it says.
#[test]
fn a_failed_row_speaks_its_node_with_the_label_it_has_now() {
    let tol = Tol::witness();
    let (doc, extrude, _) = common::broken_document(tol);
    let doc = relabelled(&doc, extrude, "pocket", tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let failed = |rows: &[viewer::tree::TreeRow]| match common::status_of(rows, extrude) {
        viewer::tree::RowStatus::Failed { message, .. } => message,
        other => panic!("the extrude fails: {other:?}"),
    };
    let before = failed(&session.tree_rows());
    assert!(
        before.starts_with(&format!(
            "Extrude \"pocket\" ({}) failed: ",
            tag(extrude.0.digest())
        )),
        "{before}"
    );

    let evaluation = session.evaluation().expect("a run landed");
    let slot = relabelled(session.committed_doc(), extrude, "slot", tol);
    let files = viewer::parts::PartFiles::default();
    let over =
        |doc: &Doc<ProfileProgram>| failed(&viewer::tree::rows(doc, Some(evaluation), &files));
    let expected = before.replacen("\"pocket\"", "\"slot\"", 1);
    assert_eq!(
        over(session.committed_doc()),
        before,
        "the run's own document"
    );
    assert_eq!(
        over(&slot),
        expected,
        "one evaluation over the renamed document says the new label"
    );

    let renamed = session.perform(SessionOp::SetLabel {
        node: extrude,
        label: Some(label("slot")),
    });
    assert!(renamed.refusal.is_none(), "{:?}", renamed.refusal);
    session.pump();
    assert_eq!(
        failed(&session.tree_rows()),
        expected,
        "the rename moves the label and nothing else the row says"
    );
}

/// The refusal and the line `perform_batch` would show for `ops`,
/// performed in order on `session` as one frame's batch: the batch's
/// refusal spoken again from the document it leaves
/// (`frame::batch_refusal`), then ranked (`frame::frame_status`).
fn batch_line(session: &mut DocSession, ops: &[SessionOp]) -> Option<viewer::frame::Message> {
    let mut refusal = None;
    for op in ops {
        if let Some(next) = session.perform(op.clone()).refusal {
            refusal = viewer::session::Refusal::preferred(refusal, next);
        }
    }
    let refusal = viewer::frame::batch_refusal(refusal, ops, session.committed_doc());
    let mut line = None;
    viewer::frame::apply(
        &mut line,
        viewer::frame::frame_status(&[], ops, refusal.as_ref()),
    );
    line
}

/// **A refusal on the status line speaks its node as the document the
/// batch leaves holds it, and the next rename retires it.** The line is
/// a sentence made once, at the end of the batch: a rename later in the
/// same batch is the label it says, and a rename in a later frame is an
/// act the document accepts, which clears the line rather than leaving
/// the old label on screen. Red if the refusal says the node by its tag
/// alone, says the label from before a rename in its own batch, or if a
/// rename leaves the line standing.
#[test]
fn a_kept_refusal_speaks_its_node_and_a_rename_retires_it() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-refusal", tol);
    let doc = relabelled(&doc, extrude, "plate", tol);
    let mut session = DocSession::inline(doc, tol);
    let refused = SessionOp::AddExtrude {
        profile: extrude,
        distance: common::len(0.01),
    };
    let rename = |text: &str| SessionOp::SetLabel {
        node: extrude,
        label: Some(label(text)),
    };
    let said = |line: &Option<viewer::frame::Message>| {
        line.as_ref()
            .map(|m| m.text().to_owned())
            .unwrap_or_default()
    };
    let is_not_a_profile = |text: &str| {
        format!(
            "Extrude \"{text}\" ({}) is not a profile",
            tag(extrude.0.digest())
        )
    };

    let line = batch_line(&mut session, core::slice::from_ref(&refused));
    assert!(
        said(&line).starts_with(&is_not_a_profile("plate")),
        "{}",
        said(&line)
    );

    let line = batch_line(&mut session, &[refused.clone(), rename("slab")]);
    assert!(
        said(&line).starts_with(&is_not_a_profile("slab")),
        "a rename after the refusal in its own batch is the label it says: {}",
        said(&line)
    );

    let mut line = line;
    viewer::frame::apply(
        &mut line,
        viewer::frame::frame_status(&[], &[rename("post")], None),
    );
    assert!(session.perform(rename("post")).refusal.is_none());
    assert_eq!(
        line, None,
        "the rename retires the line that said the old label"
    );
}

/// **An edit the kernel door refuses speaks its node on the line as the
/// document the batch leaves holds it** (`EditError::respoken`): the
/// door spoke the node at the refusal, and a rename later in the same
/// batch is the label the line says. The refusal is an expression
/// typed into a slot the block (an extrude) does not have, which the
/// edit door refuses. Red if the line says the label from before the
/// rename.
#[test]
fn an_edit_door_refusal_says_a_rename_later_in_its_batch() {
    let tol = Tol::witness();
    let (doc, block, _) =
        viewer::test_support::boss_on_block("viewer-node-labels-edit-refusal", tol);
    let doc = relabelled(&doc, block, "plate", tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let line = batch_line(
        &mut session,
        &[
            SessionOp::SetSlotExpression {
                node: block,
                slot: SlotId::RevolveAngle,
                text: "1 rad".to_owned(),
            },
            SessionOp::SetLabel {
                node: block,
                label: Some(label("slab")),
            },
        ],
    );
    let said = line.map(|m| m.text().to_owned()).unwrap_or_default();
    assert!(
        said.contains(&format!("Extrude \"slab\" ({})", tag(block.0.digest()))),
        "{said}"
    );
    assert!(
        !said.contains("plate"),
        "and not the label from before the rename: {said}"
    );
}

/// **A node deleted later in the refusal's batch keeps the label the
/// refusal said** (`SpokenNode::respoken`): the batch's document does
/// not hold it, and within one document's history its id still names
/// that node. Red if the line drops it to `node <tag>`.
#[test]
fn a_node_deleted_later_in_the_batch_keeps_its_label_on_the_line() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-deleted", tol);
    let doc = relabelled(&doc, extrude, "plate", tol);
    let mut session = DocSession::inline(doc, tol);
    let line = batch_line(
        &mut session,
        &[
            SessionOp::AddExtrude {
                profile: extrude,
                distance: common::len(0.01),
            },
            SessionOp::DeleteNode { node: extrude },
        ],
    );
    assert!(
        session.committed_doc().node(extrude).is_none(),
        "the batch deletes the node"
    );
    let said = line.map(|m| m.text().to_owned()).unwrap_or_default();
    assert!(
        said.starts_with(&format!(
            "Extrude \"plate\" ({}) is not a profile",
            tag(extrude.0.digest())
        )),
        "{said}"
    );
}

/// **A batch that replaces the document leaves its refusal as raised**:
/// the new document's ids say nothing about the refusal's, so it is not
/// spoken from them. Red if the refusal is re-spoken from the new
/// document (its node would read `node <tag>`).
#[test]
fn a_refusal_before_a_new_document_in_its_batch_keeps_the_label_it_was_raised_with() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-replaced", tol);
    let doc = relabelled(&doc, extrude, "plate", tol);
    let mut session = DocSession::inline(doc, tol);
    let line = batch_line(
        &mut session,
        &[
            SessionOp::AddExtrude {
                profile: extrude,
                distance: common::len(0.01),
            },
            SessionOp::NewDocument {
                name: "elsewhere".to_owned(),
            },
        ],
    );
    let said = line.map(|m| m.text().to_owned()).unwrap_or_default();
    assert!(
        said.starts_with(&format!(
            "Extrude \"plate\" ({}) is not a profile",
            tag(extrude.0.digest())
        )),
        "{said}"
    );
}

/// **Within one document's history an id names one node**, the claim
/// `SpokenNode::respoken` rests on: two versions that part from one value
/// mint different ids from there on, so a later version holds an id as
/// the node an earlier one did, or not at all. Here: an insert, then
/// from the same value a different insert (an undo, then another edit).
/// Red if the second insert reuses the first's id.
#[test]
fn an_undo_then_a_different_insert_mints_a_different_id() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-mint", tol);
    let profile = match doc.node(extrude) {
        Some(Node::Extrude { profile, .. }) => *profile,
        other => panic!("the fixture's extrude: {other:?}"),
    };
    let (taller, tall) = common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: common::len(0.03),
        },
        tol,
    );
    let (shorter, short) = common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: common::len(0.02),
        },
        tol,
    );
    assert_ne!(tall, short, "two inserts from one value mint two ids");
    assert!(
        shorter.node(tall).is_none(),
        "the other branch's id is absent here"
    );
    assert!(taller.node(short).is_none(), "and this branch's there");
}

/// **A held refusal speaks the node it refused**, label and all. Red if
/// the path editor's refusal says the node by its tag alone.
#[test]
fn the_path_editors_refusal_speaks_its_node() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-held", tol);
    let doc = relabelled(&doc, extrude, "plate", tol);
    let refused = viewer::sketch::held_loops(&doc, extrude).expect_err("an extrude is no profile");
    assert_eq!(
        refused.to_string(),
        format!(
            "Extrude \"plate\" ({}) is not a profile",
            tag(extrude.0.digest())
        )
    );
}

/// **The gather's refusal speaks its nodes**, on the toolbar badge and
/// in the scene's refusal alike. Red if either says a node by its tag
/// alone.
#[test]
fn the_gathers_refusal_speaks_its_nodes() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-product", tol);
    let doc = relabelled(&doc, extrude, "plate", tol);
    let spoken = format!("Extrude \"plate\" ({})", tag(extrude.0.digest()));
    let lineage = pncad::document::ProductError::ContactLineage {
        node: extrude,
        what: "face",
    };
    let badge = viewer::frame::product_badge(Some(&lineage), &doc).expect("a lineage fault badges");
    assert!(badge.label().contains(&spoken), "{}", badge.label());

    let (broken, failed, _) = common::broken_document(tol);
    let broken = relabelled(&broken, failed, "pocket", tol);
    let refused =
        viewer::scene::product_body(&broken, tol).expect_err("a failed placement gathers nothing");
    assert!(
        refused
            .to_string()
            .contains(&format!("Extrude \"pocket\" ({})", tag(failed.0.digest()))),
        "{refused}"
    );
}

/// **The Checks window speaks from the landed document.** The report
/// is the landed run's, so while a rename has not landed the window's
/// body button says the label the run was over, never the committed
/// one's.
#[test]
fn the_checks_window_speaks_from_the_landed_document() {
    let tol = Tol::witness();
    let mut session = DocSession::inline(Doc::empty_derived("checks-window-speaks", tol), tol);
    let big = common::xy_box_in(&mut session, [0.04, 0.02, 0.01]);
    let small = common::xy_box_in(&mut session, [0.02, 0.01, 0.006]);
    let copies = [big, small].map(|body| common::copy_of(session.committed_doc(), body));
    common::product_is(&mut session, &copies);
    let named = session.perform(SessionOp::SetLabel {
        node: big,
        label: Some(label("big block")),
    });
    assert!(named.refusal.is_none(), "{:?}", named.refusal);
    session.pump();
    let renamed = session.perform(SessionOp::SetLabel {
        node: big,
        label: Some(label("renamed")),
    });
    assert!(renamed.refusal.is_none(), "{:?}", renamed.refusal);

    let report = session.checks().expect("the registry ran");
    let (landed, _) = session.landed_pair().expect("a run landed");
    assert_eq!(
        (
            landed.label(big).map(Label::as_str),
            session.doc().label(big).map(Label::as_str)
        ),
        (Some("big block"), Some("renamed")),
        "the rename has not landed: the two documents say two labels"
    );
    let rows = viewer::frame::check_rows(report, landed);
    let row = rows
        .iter()
        .find(|row| row.select == viewer::Selection::Node(big))
        .expect("the two overlapping boxes are an interference finding about the big one");
    let b = tag(big.0.digest());
    assert_eq!(row.button, format!("Extrude \"big block\" ({b})"));
    // The finding is about the two copies, and names them by their
    // placements (A10).
    let copy = |body| tag(common::copy_of(landed, body).0.digest());
    assert!(
        row.sentence.contains(&format!(
            "Place {} and Place {} overlap",
            copy(big),
            copy(small)
        )),
        "the finding speaks both copies from the landed document: {}",
        row.sentence
    );
    assert!(
        !row.sentence.contains("renamed") && !row.button.contains("renamed"),
        "and never the committed label: {row:?}"
    );
}

/// **An unused variable's row selects the variable**: a named parameter
/// nothing reads has no node, so its button selects it in the property
/// panel, said by its name as the sentence says it; an unplaced
/// construction's row selects its node.
#[test]
fn an_unused_variables_row_selects_the_variable() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("checks-window-unused", tol);
    let (doc, _) = common::edited(
        &doc,
        DocEdit::DeclareVar {
            name: pncad::document::VarName::from_static("spare"),
            def: pncad::document::VarDecl::Free(pncad::document::FreeVar::continuous(
                Dimension::Length,
                0.01,
            )),
        },
        tol,
    );
    let spare = doc.var_named("spare").expect("declared");
    let ev = pncad::document::evaluate::<f64>(
        &doc,
        None,
        &pncad::document::CancelToken::new(),
        &pncad::document::EvalOptions::default(),
        tol,
    );
    let report =
        pncad::document::run_checks(&doc, &ev, &pncad::document::ChecksConfig::default(), tol)
            .expect("the checks run");
    let rows = viewer::frame::check_rows(&report, &doc);
    let selects: Vec<_> = rows.iter().map(|row| row.select.clone()).collect();
    assert_eq!(
        selects,
        [
            viewer::Selection::Node(extrude),
            viewer::Selection::Variable(spare)
        ],
        "{rows:?}"
    );
    assert_eq!(rows[1].button, "spare");
    assert!(
        rows[1]
            .sentence
            .contains("check unused: spare: used by nothing"),
        "{}",
        rows[1].sentence
    );
}

/// **A labelled creation's label is saved and reopened with its node**:
/// the file carries the one `SetLabel`, and the reopened document's row
/// leads with the label. (How many undos the reopened action costs is
/// `an-action-of-several-edits-becomes-several-undos-after-reopen`'s
/// subject, and is not pinned here.)
#[test]
fn a_labelled_creation_keeps_its_label_through_save_and_reopen() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("viewer-node-labels-reopen", tol);
    let (doc, profile) = common::square_profile(&doc, 0.02, tol);
    let mut session = DocSession::inline(doc, tol);
    let creation = Creation::of(SessionOp::AddExtrude {
        profile,
        distance: common::len(0.01),
    })
    .unwrap_or_else(|op| panic!("{op:?} creates a node"));
    let text = "base plate — 板";
    let outcome = session.perform(SessionOp::CreateLabelled {
        creation,
        label: label(text),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let node = viewer::world::made(session.committed_doc(), &outcome.minted).expect("made a node");

    let dir = common::tempdir("node-labels-reopen");
    let path = dir.join("plate.pncad");
    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);

    let mut reopened = DocSession::inline(Doc::empty_derived("viewer-node-labels-other", tol), tol);
    let opened = reopened.perform(SessionOp::Open(path));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    assert_eq!(reopened.committed_doc().label(node), Some(&label(text)));
    assert_eq!(
        headline(&reopened.committed_doc().spoken(node), None).lead,
        text,
        "the reopened row leads with the label"
    );
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// **A degenerate creation still mints the node its label lands on**: a
/// pattern of one and a body less itself (the typed empty body) are
/// accepted, so `commit_creation`'s "an accepted run mints" holds on
/// them and each is labelled in its one action.
#[test]
fn a_degenerate_creation_still_mints_the_node_it_labels() {
    use viewer::session::PatternRuleSpec;
    let tol = Tol::witness();
    let (doc, block) = extruded("viewer-node-labels-degenerate", tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let degenerate = [
        SessionOp::AddPattern {
            input: block.into(),
            count: 1,
            rule: PatternRuleSpec::Linear {
                direction: common::scl3([1.0, 0.0, 0.0]),
                spacing: common::len(0.05),
            },
        },
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: block.into(),
                tool: block.into(),
            },
        },
    ];
    for op in degenerate {
        let what = format!("{op:?}");
        let creation = Creation::of(op).unwrap_or_else(|op| panic!("{op:?} creates a node"));
        let outcome = session.perform(SessionOp::CreateLabelled {
            creation,
            label: label("degenerate"),
        });
        assert!(outcome.refusal.is_none(), "{what}: {:?}", outcome.refusal);
        let node = viewer::world::made(session.committed_doc(), &outcome.minted)
            .unwrap_or_else(|| panic!("{what} made a node"));
        assert_eq!(
            session.committed_doc().label(node),
            Some(&label("degenerate")),
            "{what}"
        );
        session.perform(SessionOp::Undo);
    }
}
