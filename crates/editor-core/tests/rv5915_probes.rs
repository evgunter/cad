//! Review probes for PR 5915 (R1). Not for merge.
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

fn block_at(doc: ProfileDoc, x: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(x, 0.0, 0.5)]);
    let (doc, block) = insert(doc, Node::Extrude { profile: profile.into(), distance: len(1.0) });
    (doc, profile, block)
}

fn place_above(doc: ProfileDoc, split: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    let above = doc.output(split, 0).expect("a split's above");
    let (doc, placed) = step(doc, DocEdit::place(BodyRead::plain(Operand::Var(above)), Operand::Var(VarId::WORLD), editor_core::stand().bundle()));
    (doc, placed.expect("the placement mints"))
}

fn split_block(seed: &str) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let (doc, profile, block) = block_at(doc, 0.0);
    let tool = fixture::plane_on(&doc, block, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(doc, Node::Split { target: block.into(), tool });
    let (doc, placed) = place_above(doc, split);
    (doc, split, vec![profile, block, split, placed])
}

fn tool_of(doc: &ProfileDoc, split: RecipeNodeId) -> VarId {
    let Some(Node::Split { tool, .. }) = doc.node(split) else { panic!("split") };
    tool.read
}

fn rename(doc: ProfileDoc, var: VarId, name: &'static str) -> ProfileDoc {
    step(doc, DocEdit::RenameVar { var: var.into(), name: Some(VarName::from_static(name)) }).0
}

fn try_split(doc: &ProfileDoc, cut: &[RecipeNodeId], seed: &str) -> Result<SplitOutcome, SplitError> {
    editor_core::split(doc, &cut.iter().copied().collect::<BTreeSet<_>>(), DocumentId::derive(&format!("{seed}-part")), Tol::witness(), None)
}

fn round_trips(doc: &ProfileDoc, out: &SplitOutcome, seed: &str) {
    let mut store = PartStore::default();
    store.insert(out.part.clone(), Tol::witness());
    let store: Arc<dyn PartResolver> = Arc::new(store);
    let back = editor_core::inline(&out.remainder, out.instance, &store, Tol::witness())
        .unwrap_or_else(|e| panic!("{seed}: inlines back: {e}"));
    let (nodes, steps) = composed(doc, out, &back);
    same_up_to_ids(doc, &flat(&back), &nodes, &steps).unwrap_or_else(|e| panic!("{seed}: not d up to ids:\n{e}"));
}

/// The inner pose reads of the tool's Slide.
fn inner_poses(doc: &ProfileDoc, pose: VarId) -> Vec<VarId> {
    let def = doc.var(pose).and_then(|v| v.def().pose()).cloned().expect("pose");
    let mut out: Vec<VarId> = def.reads().into_iter().map(|(_, &r, _, _)| r).filter(|&r| doc.var(r).is_some_and(|v| v.def().pose().is_some())).collect();
    out.dedup();
    out
}

fn by_of(doc: &ProfileDoc, pose: VarId) -> VarId {
    let Some(editor_core::pose::PoseDef::Slide { by, .. }) = doc.var(pose).and_then(|v| v.def().pose()).cloned() else { panic!("slide") };
    by
}

#[test]
fn rv_shape_of_the_tool() {
    let (doc, s, _) = split_block("rv-shape");
    let t = tool_of(&doc, s);
    let def = doc.var(t).and_then(|v| v.def().pose()).cloned().unwrap();
    eprintln!("tool {t:?} = {def:?}");
    for i in inner_poses(&doc, t) {
        eprintln!("inner {i:?} = {:?}", doc.var(i).map(|v| v.def().clone()));
    }
}

/// A named pose read by a named pose.
#[test]
fn rv_nested_both_named_round_trips() {
    let seed = "rv-nested-both";
    let (doc, s, cut) = split_block(seed);
    let t = tool_of(&doc, s);
    let inner = inner_poses(&doc, t);
    assert!(!inner.is_empty(), "premise: the slide reads a pose");
    let doc = rename(doc, t, "cut_plane");
    let doc = rename(doc, inner[0], "top_plane");
    let out = try_split(&doc, &cut, seed).unwrap_or_else(|e| panic!("{e}"));
    let held = out.part.var_named("top_plane").expect("inner named pose crosses under its name");
    let outer = out.part.var_named("cut_plane").expect("outer");
    assert!(inner_poses(&out.part, outer).contains(&held), "outer reads inner in part");
    round_trips(&doc, &out, seed);
}

/// A named pose read by an anonymous pose.
#[test]
fn rv_nested_inner_named_round_trips() {
    let seed = "rv-nested-inner";
    let (doc, s, cut) = split_block(seed);
    let t = tool_of(&doc, s);
    let inner = inner_poses(&doc, t);
    let doc = rename(doc, inner[0], "top_plane");
    let out = try_split(&doc, &cut, seed).unwrap_or_else(|e| panic!("{e}"));
    assert!(out.part.var_named("top_plane").is_some());
    round_trips(&doc, &out, seed);
}

/// A named number read by the cut pose and by a kept extrude refuses typed.
#[test]
fn rv_named_number_on_both_sides_refuses_uncut() {
    let seed = "rv-num-both";
    let (doc, s, cut) = split_block(seed);
    let by = by_of(&doc, tool_of(&doc, s));
    let doc = rename(doc, by, "drop");
    let (doc, p2) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(5.0, 0.0, 0.5)]);
    let (doc, _b2) = insert(doc, Node::Extrude { profile: p2.into(), distance: editor_core::Formula::named(VarName::from_static("drop"), editor_core::Dimension::Length) });
    let err = try_split(&doc, &cut, seed).err();
    eprintln!("number both sides: {err:?}");
    assert!(matches!(err, Some(SplitError::UncutVarReference { .. })), "{err:?}");
}

/// A named pose read by a cut split and by a kept split of the cut
/// split's below (same space): the only same-space kept reader.
#[test]
fn rv_named_pose_on_both_sides_refuses_typed() {
    let seed = "rv-pose-both";
    let (doc, s, cut) = split_block(seed);
    let t = tool_of(&doc, s);
    let doc = rename(doc, t, "cut_plane");
    let Some(Node::Split { tool, .. }) = doc.node(s).cloned() else { panic!() };
    let below = doc.output(s, 1).expect("below");
    let (doc, _kept) = insert(doc, Node::Split { target: BodyRead::plain(Operand::Var(below)), tool: editor_core::pose::RoleRead { read: Operand::Var(tool.read), role: tool.role } });
    let err = try_split(&doc, &cut, seed).err();
    eprintln!("pose both sides: {err:?}");
    assert!(err.is_some(), "refuses");
}

/// A part holding a named pose, split again: inline(split(part)) is part.
#[test]
fn rv_a_part_holding_a_named_pose_splits_again() {
    let seed = "rv-part-again";
    let (doc, s, cut) = split_block(seed);
    let t = tool_of(&doc, s);
    let doc = rename(doc, t, "cut_plane");
    let by = by_of(&doc, t);
    let doc = rename(doc, by, "drop");
    let out = try_split(&doc, &cut, seed).unwrap_or_else(|e| panic!("{e}"));
    let part = out.part.clone();
    let ids: Vec<RecipeNodeId> = part.ids().iter().copied().filter(|&id| !matches!(part.node(id), Some(Node::World | Node::Space))).collect();
    eprintln!("part ids: {:?}", ids.iter().map(|&i| part.node(i).map(std::mem::discriminant)).collect::<Vec<_>>());
    let again = try_split(&part, &ids, "rv-part-again-2").unwrap_or_else(|e| panic!("re-split of the part: {e}"));
    assert!(again.part.var_named("cut_plane").is_some());
    round_trips(&part, &again, "rv-part-again-2");
    round_trips(&doc, &out, seed);
}

/// Inline into a host already holding the pose's name refuses typed.
#[test]
fn rv_inline_pose_name_conflict() {
    let seed = "rv-conflict";
    let (doc, s, cut) = split_block(seed);
    let t = tool_of(&doc, s);
    let doc = rename(doc, t, "cut_plane");
    let out = try_split(&doc, &cut, seed).unwrap();
    // Name something in the remainder "cut_plane": a free number.
    let (remainder, _) = step(out.remainder.clone(), DocEdit::DeclareVar { name: VarName::from_static("cut_plane"), def: editor_core::VarDecl::defined(len(1.0)) });
    let mut store = PartStore::default();
    store.insert(out.part.clone(), Tol::witness());
    let store: Arc<dyn PartResolver> = Arc::new(store);
    let r = editor_core::inline(&remainder, out.instance, &store, Tol::witness());
    assert!(matches!(r, Err(editor_core::InlineError::VarNameConflict { .. })), "{r:?}");
}

/// p2_face's face-crossing scene: k kept, its mate `to` reading the top's upper cap.
fn face_across(label: &str) -> (crate::p2_gauges::Parts, ProfileDoc, [RecipeNodeId; 3], editor_core::MateRef) {
    use fixture::head;
    let p = crate::p2_gauges::parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, k) = insert(doc, fixture::mated_instance(p.top));
    let (doc, base) = crate::p2_split::pinned_at(doc, p.base, [4.0, 0.0, 0.0], &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _seat) = crate::p2_split::seated(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let (doc, m) = fixture::constrain_at(doc, k, fixture::rest(head(p.top_cap(k)), head(p.top_upper_cap(top))));
    (p, doc, [base, top, k], m)
}

fn split_face_across(named: bool) -> Result<SplitOutcome, SplitError> {
    let label = if named { "rv-face-named" } else { "rv-face-anon" };
    let (p, doc, [base, top, _k], m) = face_across(label);
    let to = fixture::constraint(&doc, m).to;
    eprintln!("to {to:?}: name {:?} def {:?}", doc.var_name(to), doc.var(to).map(|v| v.def().clone()));
    let doc = if named { rename(doc, to, "seat_plane") } else { doc };
    let o = p.opts();
    editor_core::split(&doc, &crate::p2_gauges::cut(&doc, &[base, top]), DocumentId::derive(&format!("{label}-part")), Tol::witness(), o.resolver.as_ref())
}

#[test]
fn rv_claim5_anonymous_kept_mate_pose_crosses() {
    split_face_across(false).unwrap_or_else(|e| panic!("anonymous crosses: {e}"));
}

#[test]
fn rv_claim5_named_kept_mate_pose_refuses_severed() {
    let r = split_face_across(true);
    eprintln!("named kept mate: {:?}", r.as_ref().err());
    assert!(matches!(r, Err(SplitError::SeveredEdge { .. })), "{:?}", r.err());
}

/// Claim 2: a kept mate whose `to` slides by a named number `gap`
/// that a cut extrude also reads.
#[test]
fn rv_claim2_kept_mate_pose_number_shared_with_cut() {
    use fixture::head;
    let label = "rv-claim2";
    let p = crate::p2_gauges::parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, _) = step(doc, DocEdit::DeclareVar { name: VarName::from_static("gap"), def: editor_core::VarDecl::defined(len(0.0)) });
    let (doc, k) = insert(doc, fixture::mated_instance(p.top));
    let (doc, base) = crate::p2_split::pinned_at(doc, p.base, [4.0, 0.0, 0.0], &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _seat) = crate::p2_split::seated(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let mut m = fixture::rest(head(p.top_cap(k)), head(p.top_upper_cap(top)));
    let gap = || editor_core::Formula::named(VarName::from_static("gap"), editor_core::Dimension::Length);
    m.to = Operand::Pose(Box::new(editor_core::pose::PoseDef::Slide {
        pose: m.to.clone().into(),
        along: editor_core::pose::RoleRead { read: m.to.clone(), role: Some(editor_core::pose::PoseRole::Normal) },
        by: gap(),
    }));
    let (doc, _m) = fixture::constrain_at(doc, k, m);
    let (doc, pr) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(9.0, 0.0, 0.5)]);
    let (doc, e) = insert(doc, Node::Extrude { profile: pr.into(), distance: editor_core::Formula::named(VarName::from_static("gap"), editor_core::Dimension::Length) });
    let mut c = crate::p2_gauges::cut(&doc, &[base, top]);
    c.extend([pr, e]);
    let r = editor_core::split(&doc, &c, DocumentId::derive(&format!("{label}-part")), Tol::witness(), o.resolver.as_ref());
    match &r {
        Ok(out) => {
            let held = out.remainder.var_named("gap");
            eprintln!("CLAIM2 admitted; remainder holds gap: {held:?}; part holds gap: {:?}", out.part.var_named("gap"));
            let ev = fixture::run(&out.remainder, &{ let mut s = p.store.clone(); s.insert(out.part.clone(), Tol::witness()); fixture::resolver::with_resolver(s) });
            let errs: Vec<_> = out.remainder.ids().iter().filter_map(|&id| ev.node_error(id).map(|e| format!("{e:?}"))).collect();
            eprintln!("CLAIM2 remainder node errors: {errs:?}");
        }
        Err(e) => eprintln!("CLAIM2 refused: {e:?}"),
    }
}
