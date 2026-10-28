//! **A pose definition crosses with its readers** (`refactor.rs`'s
//! module docs, "Variables go where their readers go"): a split whose
//! tool is a pose read off the cut's own geometry carries that pose
//! into the part, its face read through the node map and its numbers
//! through the variables the part holds, and inline carries it back.
//! A named pose crosses once, at its first carried reader, under its
//! name, and so does each named pose under it; a named number a pose is
//! defined over moves with its reader. A named number read on both
//! sides refuses `UncutVarReference`, through a pose or not; a named
//! pose read on both sides refuses earlier, its kept reader severed
//! from the cut its faces lie on.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::fixture;
use crate::fixture::resolver::PartStore;
use crate::fixture::round_trip::{composed, flat, same_up_to_ids};
use editor_core::{
    BodyRead, DocEdit, DocumentId, Node, Operand, PartResolver, ProfileDoc, RecipeNodeId,
    SplitError, SplitOutcome, VarId, VarName,
};
use fixture::{insert, len, on_frame, square, step};
use geom_core::Tol;

/// A unit block, split at mid-height by a plane slid off its own top
/// face (a `Slide` of a `Plane { face }`, no operation's output), the
/// split's `above` placed into the world. Answers the document, the
/// split, and the cut: every node.
fn split_block(seed: &str) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let tool = fixture::plane_on(&doc, block, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: block.into(),
            tool,
        },
    );
    let (doc, placed) = place_above(doc, split);
    (doc, split, vec![profile, block, split, placed])
}

/// `split`'s `above` placed into the world at the identity.
fn place_above(doc: ProfileDoc, split: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    let above = doc.output(split, 0).expect("a split's above");
    let (doc, placed) = step(
        doc,
        DocEdit::place(
            BodyRead::plain(Operand::Var(above)),
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    (doc, placed.expect("the placement mints"))
}

/// The pose `split` cuts with.
fn tool_of(doc: &ProfileDoc, split: RecipeNodeId) -> VarId {
    let Some(Node::Split { tool, .. }) = doc.node(split) else {
        panic!("{split:?} is a split")
    };
    assert!(
        doc.var(tool.read).is_some_and(|v| v.def().pose().is_some())
            && doc.operation_of(tool.read).is_none(),
        "the premise: the tool is a pose definition, no operation's output"
    );
    tool.read
}

fn rename(doc: ProfileDoc, var: VarId, name: &'static str) -> ProfileDoc {
    step(
        doc,
        DocEdit::RenameVar {
            var: var.into(),
            name: Some(VarName::from_static(name)),
        },
    )
    .0
}

/// The split of `cut`.
fn try_split(
    doc: &ProfileDoc,
    cut: &[RecipeNodeId],
    seed: &str,
) -> Result<SplitOutcome, SplitError> {
    editor_core::split(
        doc,
        &cut.iter().copied().collect::<BTreeSet<_>>(),
        DocumentId::derive(&format!("{seed}-part")),
        Tol::witness(),
        None,
    )
}

/// The split of `cut`, which must be admitted.
fn split(doc: &ProfileDoc, cut: &[RecipeNodeId], seed: &str) -> SplitOutcome {
    try_split(doc, cut, seed).unwrap_or_else(|e| panic!("{seed}: the split is admitted: {e}"))
}

/// The poses `pose`'s definition reads.
fn inner_poses(doc: &ProfileDoc, pose: VarId) -> Vec<VarId> {
    let def = doc
        .var(pose)
        .and_then(|v| v.def().pose())
        .cloned()
        .expect("a pose");
    let mut out: Vec<VarId> = def
        .reads()
        .into_iter()
        .map(|(_, &r, _, _)| r)
        .filter(|&r| doc.var(r).is_some_and(|v| v.def().pose().is_some()))
        .collect();
    out.dedup();
    out
}

/// The number the slide `pose` slides by.
fn slide_by(doc: &ProfileDoc, pose: VarId) -> VarId {
    let Some(editor_core::pose::PoseDef::Slide { by, .. }) =
        doc.var(pose).and_then(|v| v.def().pose()).cloned()
    else {
        panic!("the tool slides off the top face")
    };
    by
}

/// The split, inlined back, is `doc` up to minted ids and order (A4),
/// each named pose and number compared by name.
fn round_trips(doc: &ProfileDoc, out: &SplitOutcome, seed: &str) {
    let mut store = PartStore::default();
    store.insert(out.part.clone(), Tol::witness());
    let store: Arc<dyn PartResolver> = Arc::new(store);
    let back = editor_core::inline(&out.remainder, out.instance, &store, Tol::witness())
        .unwrap_or_else(|e| panic!("{seed}: the part inlines back: {e}"));
    let (nodes, steps) = composed(doc, out, &back);
    same_up_to_ids(doc, &flat(&back), &nodes, &steps)
        .unwrap_or_else(|e| panic!("{seed}: inline(split(d)) is d up to ids:\n{e}"));
}

/// A pose written in place at the tool crosses as its definition.
#[test]
fn an_anonymous_pose_tool_crosses_with_its_split() {
    let seed = "pose-anonymous";
    let (doc, _, cut) = split_block(seed);
    let out = split(&doc, &cut, seed);
    round_trips(&doc, &out, seed);
}

/// A named pose two cut splits read crosses once, under its name, and
/// both carried splits read that one pose.
#[test]
fn a_named_pose_two_splits_read_crosses_once_under_its_name() {
    let seed = "pose-named";
    let (doc, first, mut cut) = split_block(seed);
    let pose = tool_of(&doc, first);
    let doc = rename(doc, pose, "cut_plane");
    let Some(Node::Split { tool, .. }) = doc.node(first).cloned() else {
        panic!("the split")
    };
    let (doc, second) = insert(
        doc,
        Node::Split {
            target: cut[1].into(),
            tool: editor_core::pose::RoleRead {
                read: Operand::Var(tool.read),
                role: tool.role,
            },
        },
    );
    assert_eq!(
        tool_of(&doc, second),
        pose,
        "the premise: one pose, read twice"
    );
    let (doc, placed) = place_above(doc, second);
    cut.extend([second, placed]);
    let out = split(&doc, &cut, seed);
    let held = out
        .part
        .var_named("cut_plane")
        .expect("the part holds the pose under its name");
    let readers: Vec<VarId> = out
        .part
        .ids()
        .iter()
        .copied()
        .filter(|&id| matches!(out.part.node(id), Some(Node::Split { .. })))
        .map(|id| tool_of(&out.part, id))
        .collect();
    assert_eq!(
        readers,
        vec![held, held],
        "both carried splits read the one pose"
    );
    assert!(
        out.remainder.var_named("cut_plane").is_none(),
        "the pose left the remainder with its readers"
    );
    round_trips(&doc, &out, seed);
}

/// A named number a pose slides by moves with the pose's reader: the
/// part declares it under its name and the carried pose reads it there.
#[test]
fn a_named_number_a_pose_is_defined_over_moves_with_it() {
    let seed = "pose-named-number";
    let (doc, split_node, cut) = split_block(seed);
    let by = slide_by(&doc, tool_of(&doc, split_node));
    let doc = rename(doc, by, "drop");
    let out = split(&doc, &cut, seed);
    let drop = out
        .part
        .var_named("drop")
        .expect("the number moved into the part");
    let carried = out
        .part
        .ids()
        .iter()
        .copied()
        .find(|&id| matches!(out.part.node(id), Some(Node::Split { .. })))
        .map(|id| tool_of(&out.part, id))
        .expect("the carried split");
    assert!(
        matches!(
            out.part.var(carried).and_then(|v| v.def().pose()),
            Some(editor_core::pose::PoseDef::Slide { by, .. }) if *by == drop
        ),
        "the carried pose slides by the carried number"
    );
    assert!(
        out.remainder.var_named("drop").is_none(),
        "the number left the remainder with its reader"
    );
    round_trips(&doc, &out, seed);
}

/// A part holding a named pose nothing reads refuses inline, naming
/// the pose: a pose crosses only with a reader, and inline does not
/// drop it.
#[test]
fn a_part_holding_an_unread_named_pose_refuses_inline() {
    let seed = "pose-unread";
    let (part, split_node, cut) = split_block(seed);
    let pose = tool_of(&part, split_node);
    let part = rename(part, pose, "cut_plane");
    let (part, _) = fixture::place(part, cut[1]);
    let part = [cut[3], split_node]
        .into_iter()
        .fold(part, |part, id| step(part, DocEdit::DeleteNode { id }).0);
    assert!(
        part.var(pose).is_some() && part.var_named("cut_plane") == Some(pose),
        "the premise: the named pose outlives its reader"
    );
    let mut store = PartStore::default();
    let doc_ref = store.insert(part, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("pose-unread-host"), Tol::witness());
    let (host, instance) = insert(host, fixture::mated_instance(doc_ref));
    let store: Arc<dyn PartResolver> = Arc::new(store);
    match editor_core::inline(&host, instance, &store, Tol::witness()) {
        Err(editor_core::InlineError::UnreadPose { var }) => {
            assert_eq!(var.id(), pose, "the refusal names the unread pose");
        }
        other => panic!("expected UnreadPose, got {other:?}"),
    }
}

/// A named pose under a named pose crosses under its name, and the
/// carried outer pose reads the carried inner one.
#[test]
fn a_named_pose_a_named_pose_reads_crosses_under_both_names() {
    let seed = "pose-nested-both";
    let (doc, split_node, cut) = split_block(seed);
    let outer = tool_of(&doc, split_node);
    let inner = inner_poses(&doc, outer);
    assert!(!inner.is_empty(), "the premise: the slide reads a pose");
    let doc = rename(rename(doc, outer, "cut_plane"), inner[0], "top_plane");
    let out = split(&doc, &cut, seed);
    let held = out
        .part
        .var_named("top_plane")
        .expect("the inner pose crosses under its name");
    let outer = out
        .part
        .var_named("cut_plane")
        .expect("the outer pose crosses");
    assert!(
        inner_poses(&out.part, outer).contains(&held),
        "the carried outer pose reads the carried inner one"
    );
    round_trips(&doc, &out, seed);
}

/// A named pose under a pose written in place crosses under its name.
#[test]
fn a_named_pose_under_an_anonymous_one_crosses_under_its_name() {
    let seed = "pose-nested-inner";
    let (doc, split_node, cut) = split_block(seed);
    let inner = inner_poses(&doc, tool_of(&doc, split_node));
    let doc = rename(doc, inner[0], "top_plane");
    let out = split(&doc, &cut, seed);
    assert!(
        out.part.var_named("top_plane").is_some(),
        "the inner pose crosses under its name"
    );
    round_trips(&doc, &out, seed);
}

/// A part holding a named pose over a named number, split whole again,
/// carries both and inlines back to itself.
#[test]
fn a_part_holding_a_named_pose_over_a_named_number_splits_again() {
    let seed = "pose-part-again";
    let (doc, split_node, cut) = split_block(seed);
    let pose = tool_of(&doc, split_node);
    let by = slide_by(&doc, pose);
    let doc = rename(rename(doc, pose, "cut_plane"), by, "drop");
    let out = split(&doc, &cut, seed);
    let part = out.part.clone();
    let ids: Vec<RecipeNodeId> = part
        .ids()
        .iter()
        .copied()
        .filter(|&id| !matches!(part.node(id), Some(Node::World | Node::Space)))
        .collect();
    let again = split(&part, &ids, "pose-part-again-2");
    assert!(
        again.part.var_named("cut_plane").is_some() && again.part.var_named("drop").is_some(),
        "the second part holds the pose and its number under their names"
    );
    round_trips(&part, &again, "pose-part-again-2");
    round_trips(&doc, &out, seed);
}

/// Inline into a host already holding the pose's name refuses, naming
/// it: a named pose crosses as a variable of its own.
#[test]
fn inlining_a_named_pose_into_a_host_holding_its_name_refuses() {
    let seed = "pose-conflict";
    let (doc, split_node, cut) = split_block(seed);
    let pose = tool_of(&doc, split_node);
    let doc = rename(doc, pose, "cut_plane");
    let out = split(&doc, &cut, seed);
    let (remainder, _) = step(
        out.remainder.clone(),
        DocEdit::DeclareVar {
            name: VarName::from_static("cut_plane"),
            def: editor_core::VarDecl::defined(len(1.0)),
        },
    );
    let mut store = PartStore::default();
    store.insert(out.part.clone(), Tol::witness());
    let store: Arc<dyn PartResolver> = Arc::new(store);
    match editor_core::inline(&remainder, out.instance, &store, Tol::witness()) {
        Err(editor_core::InlineError::VarNameConflict { name }) => {
            assert_eq!(name.as_str(), "cut_plane");
        }
        other => panic!("expected VarNameConflict, got {other:?}"),
    }
}

/// A named number the cut tool's pose slides by and a kept extrude
/// reads is one variable on both sides: `UncutVarReference`.
#[test]
fn a_named_number_a_cut_pose_and_a_kept_node_read_refuses() {
    let seed = "pose-number-both";
    let (doc, split_node, cut) = split_block(seed);
    let by = slide_by(&doc, tool_of(&doc, split_node));
    let doc = rename(doc, by, "drop");
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(5.0, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: editor_core::Formula::named(
                VarName::from_static("drop"),
                editor_core::Dimension::Length,
            ),
        },
    );
    match try_split(&doc, &cut, seed) {
        Err(SplitError::UncutVarReference { var, .. }) => {
            assert_eq!(var.name().map(VarName::as_str), Some("drop"));
        }
        other => panic!("expected UncutVarReference, got {other:?}"),
    }
}

/// A named pose a cut split and a kept split read refuses before the
/// variable rule: the kept reader's target is the cut split's `below`,
/// in the cut's space, so it is severed from the cut (`SeveredEdge`).
/// A kept reader of a pose over the cut's faces always reads the cut.
#[test]
fn a_named_pose_a_cut_and_a_kept_split_read_refuses_severed() {
    let seed = "pose-both";
    let (doc, split_node, cut) = split_block(seed);
    let pose = tool_of(&doc, split_node);
    let doc = rename(doc, pose, "cut_plane");
    let Some(Node::Split { tool, .. }) = doc.node(split_node).cloned() else {
        panic!("the split")
    };
    let below = doc.output(split_node, 1).expect("a split's below");
    let (doc, _) = insert(
        doc,
        Node::Split {
            target: BodyRead::plain(Operand::Var(below)),
            tool: editor_core::pose::RoleRead {
                read: Operand::Var(tool.read),
                role: tool.role,
            },
        },
    );
    let refused = try_split(&doc, &cut, seed);
    assert!(
        matches!(
            refused,
            Err(SplitError::SeveredEdge {
                consumer_is_cut: false,
                ..
            })
        ),
        "{refused:?}"
    );
}

/// **`Doc::slot` answers every slot**: an operand's read as well as an
/// expression slot's variable, and `None` for a slot the node lacks.
#[test]
fn a_slot_answers_an_operands_read_and_an_expressions_variable() {
    use editor_core::{OperandSlot, SlotId};
    let (doc, split_node, cut) = split_block("pose-slot");
    assert_eq!(
        doc.slot(split_node, SlotId::Operand(OperandSlot::Tool)),
        Some(tool_of(&doc, split_node)),
        "a split's tool slot reads its pose"
    );
    assert_eq!(
        doc.slot(split_node, SlotId::Operand(OperandSlot::Target)),
        doc.output(cut[1], 0),
        "a split's target slot reads the block's output"
    );
    assert!(
        doc.slot(cut[1], SlotId::Distance)
            .is_some_and(|var| doc.var(var).is_some()),
        "an extrude's distance reads its variable"
    );
    assert_eq!(
        doc.slot(split_node, SlotId::Operand(OperandSlot::Cut)),
        None,
        "a split has no cut operand"
    );
}
