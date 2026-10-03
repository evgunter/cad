//! Node labels in the viewer (DESIGN.md Band 1, "Node labels"): a
//! rename is one `SetLabel` and one undo, a labelled creation is the
//! insert and its label as one undo, a labelled row leads with its
//! label, and a create form proposes `Kind N` counted among that kind's
//! nodes.

#![allow(clippy::expect_used, clippy::panic)]

use crate::common;
use pncad::document::ExtrudeSide;

use pncad::document::{BooleanOp, Doc, DocEdit, Label, Node, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use test_utils::refusal::tag;
use viewer::session::{Creation, DocSession, ProfilePlane, SessionOp};
use viewer::tree::{Headline, headline, proposed_label};

fn label(text: &str) -> Label {
    Label::new(text).expect("a valid label")
}

/// A frame, a square on it and its extrude: the document and the
/// extrude.
fn extruded(seed: &str, tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived(seed, tol);
    let (doc, profile) = common::framed_square(&doc, 0.02, tol);
    common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.01),
            side: ExtrudeSide::Along,
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
    let creation = Creation::of(SessionOp::AddDatum {
        datum: ProfilePlane::world_xy().expect("the world xy frame lowers"),
    })
    .expect("adding a datum creates a node");
    let outcome = session.perform(SessionOp::CreateLabelled {
        creation,
        label: label("floor"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [frame] = outcome.minted[..] else {
        panic!("one node minted, got {:?}", outcome.minted);
    };
    assert!(
        matches!(
            &outcome.committed[..],
            [DocEdit::InsertNode { .. }, DocEdit::SetLabel { node, label: Some(l) }]
                if *node == frame && *l == label("floor")
        ),
        "the insert, then its label: {:?}",
        outcome.committed
    );
    assert_eq!(session.doc().label(frame), Some(&label("floor")));

    session.perform(SessionOp::Undo);
    assert!(
        session.doc().node(frame).is_none(),
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
    let t = tag(extrude.0);
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
        let (doc, profile) = common::framed_square(doc, 0.01, tol);
        let (doc, extrude) = common::inserted(
            &doc,
            Node::Extrude {
                profile,
                distance: common::len(0.02),
                side: ExtrudeSide::Along,
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

/// **`op`, labelled `text`, as one undo**: the creation commits through
/// whatever door it takes, the action ends with a `SetLabel` on the
/// last node it minted, the label is on that node, and ONE undo returns
/// the document to what it was before the action. Answers that node.
fn labelled_as_one_undo(session: &mut DocSession, op: SessionOp, text: &str) -> RecipeNodeId {
    let before = session.committed_doc().clone();
    let what = format!("{op:?}");
    let creation = Creation::of(op).unwrap_or_else(|op| panic!("{op:?} creates a node"));
    let outcome = session.perform(SessionOp::CreateLabelled {
        creation,
        label: label(text),
    });
    assert!(outcome.refusal.is_none(), "{what}: {:?}", outcome.refusal);
    let node = *outcome
        .minted
        .last()
        .unwrap_or_else(|| panic!("{what} minted"));
    assert!(
        matches!(
            outcome.committed.last(),
            Some(DocEdit::SetLabel { node: at, label: Some(l) }) if *at == node && *l == label(text)
        ),
        "{what}: the action ends with the label: {:?}",
        outcome.committed
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

/// **Every creating op, labelled, is one undo** — whichever commit door
/// it takes (`commit`, `commit_run`, or the boolean's own staged run).
/// The sample set is held to `SessionOp::creates_a_node` through the
/// gesture table's every-op roster: a creation added there and not
/// sampled here is red.
#[test]
fn every_creation_labelled_is_one_undo_whatever_door_commits_it() {
    use common::{ang, asm, len, len3, scl3};
    use pncad::select::ContactClass;
    use viewer::session::{DatumSpec, PartSelectSpec, PatternRuleSpec};

    let tol = Tol::witness();
    let bench = asm::bench("node-labels-every-creation", tol);
    let mut session = asm::open_bench(&bench, tol);
    let mut sampled = std::collections::BTreeSet::new();
    let mut run = |session: &mut DocSession, op: SessionOp, text: &str| {
        sampled.insert(format!("{:?}", std::mem::discriminant(&op)));
        labelled_as_one_undo(session, op, text)
    };

    let frame = run(
        &mut session,
        SessionOp::AddDatum {
            datum: ProfilePlane::world_xy().expect("the world xy frame lowers"),
        },
        "frame",
    );
    let profile = run(
        &mut session,
        SessionOp::AddProfile {
            plane: ProfilePlane::Existing(frame),
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
    let axis = session_axis(&mut session, frame);
    run(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis,
            angle: ang(core::f64::consts::PI),
        },
        "turned",
    );
    let moved = run(
        &mut session,
        SessionOp::AddTransform {
            input: block,
            translation: len3([0.005, 0.007, 0.003]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: ang(0.0),
        },
        "moved",
    );
    run(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Intersect,
            a: block,
            b: moved,
            declare: Vec::new(),
        },
        "overlap",
    );
    let plane = common::session_insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: len3([0.0, 0.0, 0.01]),
                normal: scl3([0.0, 0.0, 1.0]),
            },
        },
    );
    run(
        &mut session,
        SessionOp::AddSplit {
            target: block,
            tool: plane,
        },
        "halves",
    );
    let row = || PatternRuleSpec::Linear {
        direction: scl3([1.0, 0.0, 0.0]),
        spacing: len(0.05),
    };
    let pattern = run(
        &mut session,
        SessionOp::AddPattern {
            input: block,
            count: 2,
            rule: row(),
        },
        "row",
    );
    run(
        &mut session,
        SessionOp::AddPlacedUnion {
            input: block,
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
    run(&mut session, SessionOp::Duplicate { input: block }, "copy");
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
    run(
        &mut session,
        asm::seat_op(
            &bench,
            bench.post_b,
            ContactClass::Tangent,
            asm::middle_seat_alignment(),
        ),
        "seat",
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

/// An axis written in `frame`, for the revolve sample.
fn session_axis(session: &mut DocSession, frame: RecipeNodeId) -> RecipeNodeId {
    common::session_insert(
        session,
        SessionOp::AddDatum {
            datum: viewer::session::DatumSpec::AxisInPlane {
                plane: frame,
                origin: common::len2([0.03, 0.0]),
                direction: common::scl2([0.0, 1.0]),
            },
        },
    )
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
        before.starts_with(&format!("Extrude \"pocket\" ({}) failed: ", tag(extrude.0))),
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
    let is_not_a_profile =
        |text: &str| format!("Extrude \"{text}\" ({}) is not a profile", tag(extrude.0));

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
/// batch is the label the line says. Red if the line says the label
/// from before the rename.
#[test]
fn an_edit_door_refusal_says_a_rename_later_in_its_batch() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-edit-refusal", tol);
    let doc = relabelled(&doc, extrude, "plate", tol);
    let mut session = DocSession::inline(doc, tol);
    let line = batch_line(
        &mut session,
        &[
            SessionOp::AddBoolean {
                op: BooleanOp::Union,
                a: extrude,
                b: extrude,
                declare: Vec::new(),
            },
            SessionOp::SetLabel {
                node: extrude,
                label: Some(label("slab")),
            },
        ],
    );
    let said = line.map(|m| m.text().to_owned()).unwrap_or_default();
    assert!(
        said.contains(&format!(
            "Extrude \"slab\" ({}) is taken as an input twice",
            tag(extrude.0)
        )),
        "{said}"
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
            tag(extrude.0)
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
            tag(extrude.0)
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
            profile,
            distance: common::len(0.03),
            side: ExtrudeSide::Along,
        },
        tol,
    );
    let (shorter, short) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.02),
            side: ExtrudeSide::Along,
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
        format!("Extrude \"plate\" ({}) is not a profile", tag(extrude.0))
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
    let spoken = format!("Extrude \"plate\" ({})", tag(extrude.0));
    let collision = pncad::document::ProductError::Naming {
        node: extrude,
        name: Box::new(pncad::prelude::StableName {
            kind: pncad::prelude::EntityKind::Face,
            node: extrude,
            path: Vec::new(),
        }),
    };
    let badge = viewer::frame::product_badge(Some(&collision), &doc).expect("a collision badges");
    assert!(badge.label().contains(&spoken), "{}", badge.label());

    let (broken, failed, _) = common::broken_document(tol);
    let broken = relabelled(&broken, failed, "pocket", tol);
    let refused =
        viewer::scene::product_body(&broken, tol).expect_err("a failed root gathers nothing");
    assert!(
        refused
            .to_string()
            .contains(&format!("Extrude \"pocket\" ({})", tag(failed.0))),
        "{refused}"
    );
}

/// **The Checks window speaks its roots from the landed document.** The
/// report is the landed run's, so while a rename has not landed the
/// window's root button and the finding's sentence both say the label
/// the run was over, never the committed one's.
#[test]
fn the_checks_window_speaks_its_roots_from_the_landed_document() {
    let tol = Tol::witness();
    let mut session = DocSession::inline(Doc::empty_derived("checks-window-speaks", tol), tol);
    let big = common::xy_box_in(&mut session, [0.04, 0.02, 0.01]);
    let small = common::xy_box_in(&mut session, [0.02, 0.01, 0.006]);
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
        .find(|row| row.root == big)
        .expect("the two overlapping boxes are a separation finding about the big one");
    let (b, s) = (tag(big.0), tag(small.0));
    assert_eq!(row.button, format!("Extrude \"big block\" ({b})"));
    assert!(
        row.sentence.contains(&format!(
            "Extrude \"big block\" ({b}) output 0: not certifiably disjoint from Extrude {s} \
             output 0"
        )),
        "the finding speaks both roots from the landed document: {}",
        row.sentence
    );
    assert!(
        !row.sentence.contains("renamed") && !row.button.contains("renamed"),
        "and never the committed label: {row:?}"
    );
}
