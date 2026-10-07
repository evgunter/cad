#![allow(clippy::expect_used)]
use crate::common;
use editor_core::ExtrudeSide;
use pncad::document::{Dimension, Doc, FreeVar, Node, ProfileProgram, RecipeNodeId, SlotId, VarId, VarName};
use pncad::geom_core::Tol;
use viewer::props::SlotValue;
use viewer::session::{DocSession, SessionOp};

fn setup() -> (DocSession, RecipeNodeId, RecipeNodeId, VarId) {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("probe", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let ex = |d: f64| Node::Extrude { profile, distance: common::len(d), side: ExtrudeSide::Along };
    let (doc, a) = common::inserted(&doc, ex(0.008), tol);
    let (doc, b) = common::inserted(&doc, ex(0.010), tol);
    let mut s = DocSession::inline(doc, tol);
    let o = s.perform(SessionOp::DeclareVar { name: VarName::new("w").unwrap(), value: FreeVar::continuous(Dimension::Length, 0.012) });
    assert!(o.refusal.is_none());
    let w = common::var_of(s.committed_doc(), "w");
    (s, a, b, w)
}
fn typed(s: &mut DocSession, n: RecipeNodeId, v: f64) {
    let o = s.perform(SessionOp::SetSlot { node: n, slot: SlotId::Distance, value: SlotValue::Continuous(v) });
    assert!(o.refusal.is_none(), "{:?}", o.refusal);
}
fn offered(s: &DocSession, n: RecipeNodeId) -> Vec<VarId> {
    s.offered(n, SlotId::Distance).into_iter().map(|o| o.var).collect()
}
fn reads(s: &DocSession, n: RecipeNodeId) -> VarId { s.committed_doc().slot(n, SlotId::Distance).unwrap() }

#[test]
fn probe_drag_after_typing_offers_untyped_value() {
    let (mut s, a, b, w) = setup();
    typed(&mut s, a, 0.012);
    assert_eq!(offered(&s, a), vec![w]);
    for op in [
        SessionOp::BeginGesture { node: a, slot: SlotId::Distance },
        SessionOp::PreviewGesture { node: a, slot: SlotId::Distance, value: 0.010 },
        SessionOp::CommitGesture { node: a, slot: SlotId::Distance },
    ] { let o = s.perform(op); assert!(o.refusal.is_none(), "{:?}", o.refusal); }
    let bv = reads(&s, b);
    eprintln!("PROBE drag: offered after drag to 0.010 = {:?} (b reads {:?})", offered(&s, a), bv);
    assert!(offered(&s, a).is_empty(), "a drag-moved value (nobody typed 0.010) is offered {:?}", offered(&s, a));
}

#[test]
fn probe_value_door_on_own_var_after_typing() {
    let (mut s, a, _b, _w) = setup();
    typed(&mut s, a, 0.012);
    let own = reads(&s, a);
    let o = s.perform(SessionOp::SetVariable { var: own, value: SlotValue::Continuous(0.010) });
    assert!(o.refusal.is_none(), "{:?}", o.refusal);
    assert!(offered(&s, a).is_empty(), "moved own var in place, offered {:?}", offered(&s, a));
}

#[test]
fn probe_redo_revives_offer() {
    let (mut s, a, _b, w) = setup();
    typed(&mut s, a, 0.012);
    s.perform(SessionOp::Undo);
    assert!(offered(&s, a).is_empty());
    let o = s.perform(SessionOp::Redo);
    assert!(o.refusal.is_none(), "{:?}", o.refusal);
    eprintln!("PROBE redo: offered after redo = {:?}", offered(&s, a));
    assert!(offered(&s, a).is_empty(), "redo revived the offer: {:?} (w={w:?})", offered(&s, a));
}

#[test]
fn probe_negative_zero_not_bit_equal() {
    let (mut s, a, _b, _w) = setup();
    let o = s.perform(SessionOp::DeclareVar { name: VarName::new("z").unwrap(), value: FreeVar::continuous(Dimension::Length, -0.0) });
    assert!(o.refusal.is_none(), "{:?}", o.refusal);
    let z = common::var_of(s.committed_doc(), "z");
    let t = s.perform(SessionOp::SetSlot { node: a, slot: SlotId::Distance, value: SlotValue::Continuous(0.0) });
    eprintln!("PROBE -0: typed refusal {:?}", t.refusal);
    eprintln!("PROBE -0: offered {:?} z={z:?}", offered(&s, a));
    assert!(!offered(&s, a).contains(&z), "-0 offered against +0 (== not bit-equal)");
}

#[test]
fn probe_retype_is_noop_and_one_undo_restores() {
    let (mut s, a, _b, _w) = setup();
    let before = reads(&s, a);
    let o = s.perform(SessionOp::SetSlot { node: a, slot: SlotId::Distance, value: SlotValue::Continuous(0.008) });
    eprintln!("PROBE retype committed {}", o.committed.len());
    assert!(o.committed.is_empty());
    typed(&mut s, a, 0.012);
    assert!(s.committed_doc().var(before).is_none(), "VR7 retired");
    s.perform(SessionOp::Undo);
    assert_eq!(reads(&s, a), before);
    assert!(s.committed_doc().var(before).is_some());
}

#[test]
fn probe_accept_stale_var_after_retype() {
    // offer for slot a -> type a different value -> old offered button still accepted?
    let (mut s, a, b, w) = setup();
    typed(&mut s, a, 0.012);
    assert_eq!(offered(&s, a), vec![w]);
    typed(&mut s, a, 0.010);
    let bv = reads(&s, b);
    eprintln!("PROBE retype new offer {:?} b={bv:?}", offered(&s, a));
    // A stale click of w: the op is an unconditional slot write
    let o = s.perform(SessionOp::SetSlotVariable { node: a, slot: SlotId::Distance, var: w });
    eprintln!("PROBE stale accept refusal {:?} reads w now? {}", o.refusal, reads(&s, a) == w);
}

#[test]
fn probe_m7_formula_read_with_an_equal_variable() {
    let (mut s, a, b, w) = setup();
    typed(&mut s, b, 0.012);
    let o = s.perform(SessionOp::SetSlotExpression { node: a, slot: SlotId::Distance, text: "w".to_owned() });
    assert!(o.refusal.is_none());
    assert_eq!(reads(&s, a), w);
    assert!(offered(&s, a).is_empty(), "a read of w is offered {:?}", offered(&s, a));
}
