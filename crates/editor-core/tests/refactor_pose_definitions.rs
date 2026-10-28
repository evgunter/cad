//! **A pose definition crosses with its readers** (`refactor.rs`'s
//! module docs, "Variables go where their readers go"): a split whose
//! tool is a pose read off the cut's own geometry carries that pose
//! into the part, its face read through the node map and its numbers
//! through the variables the part holds, and inline carries it back.
//! A named pose crosses once, at its first carried reader, under its
//! name; a named number a pose is defined over moves with its reader.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::fixture;
use crate::fixture::resolver::PartStore;
use crate::fixture::round_trip::{composed, flat, same_up_to_ids};
use editor_core::{
    BodyRead, DocEdit, DocumentId, Node, Operand, PartResolver, ProfileDoc, RecipeNodeId,
    SplitOutcome, VarId, VarName,
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

/// The split of `cut`, which must be admitted.
fn split(doc: &ProfileDoc, cut: &[RecipeNodeId], seed: &str) -> SplitOutcome {
    editor_core::split(
        doc,
        &cut.iter().copied().collect::<BTreeSet<_>>(),
        DocumentId::derive(&format!("{seed}-part")),
        Tol::witness(),
        None,
    )
    .unwrap_or_else(|e| panic!("{seed}: the split is admitted: {e}"))
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
    let pose = tool_of(&doc, split_node);
    let Some(editor_core::pose::PoseDef::Slide { by, .. }) =
        doc.var(pose).and_then(|v| v.def().pose()).cloned()
    else {
        panic!("the tool slides off the top face")
    };
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
