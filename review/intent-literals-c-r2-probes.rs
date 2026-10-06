#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocumentId, EditError, Formula, FreeVar, Maintenance,
    Node, ProfileDoc, ProfileProgram, SlotId, VarDecl, VarName, apply, load, save,
};
use crate::fixture::len;
use geom_core::Tol;

fn step(doc: &ProfileDoc, e: DocEdit<ProfileProgram>) -> Result<editor_core::Applied<ProfileProgram>, EditError> {
    apply(doc, &e, Tol::witness(), &editor_core::RefusingReach)
}
fn empty(s: &str) -> ProfileDoc { ProfileDoc::empty(DocumentId::derive(s), Tol::witness()) }
fn point(position: [Formula; 3], fresh: Vec<VarDecl>) -> DocEdit<ProfileProgram> {
    DocEdit::InsertNode { node: Box::new(Node::Datum(Datum::Point { position })), fresh }
}
fn with_w(s: &str) -> ProfileDoc {
    step(&empty(s), DocEdit::DeclareVar { name: VarName::from_static("w"),
        def: VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.5)) }).unwrap().doc
}
fn w() -> Formula { Formula::named(VarName::from_static("w"), Dimension::Length) }

/// D6: a definition written in mm and in m, bit-equal values: one node id?
#[test]
fn r2_d6_definition_display_unit_moves_the_node_id() {
    let base = with_w("r2-d6");
    let mm = Formula::add(w(), Formula::length_in(125.0, quantity::MM).unwrap()).unwrap();
    let m = Formula::add(w(), Formula::length_in(0.125, quantity::M).unwrap()).unwrap();
    let a = step(&base, point([mm, len(0.0), len(0.0)], vec![])).unwrap();
    let b = step(&base, point([m, len(0.0), len(0.0)], vec![])).unwrap();
    let (ia, ib) = (a.record.minted.unwrap(), b.record.minted.unwrap());
    let va = a.doc.slot(ia, SlotId::Origin(editor_core::Axis3::X)).unwrap();
    let vb = b.doc.slot(ib, SlotId::Origin(editor_core::Axis3::X)).unwrap();
    eprintln!("defined: node {ia:?} vs {ib:?}; var {va:?} vs {vb:?}");
    // lone values for comparison
    let a2 = step(&base, point([Formula::length_in(125.0, quantity::MM).unwrap(), len(0.0), len(0.0)], vec![])).unwrap();
    let b2 = step(&base, point([Formula::length_in(0.125, quantity::M).unwrap(), len(0.0), len(0.0)], vec![])).unwrap();
    eprintln!("free: node {:?} vs {:?}", a2.record.minted, b2.record.minted);
    assert_eq!(a2.record.minted, b2.record.minted, "lone values: one id");
    assert_eq!(ia, ib, "D6: a definition's display unit must not move the node id");
}

/// Lifecycle: a fresh entry read by two slots of one node survives a
/// rewrite of one of them.
#[test]
fn r2_shared_fresh_entry_survives_rewriting_one_reader() {
    let doc = empty("r2-life");
    let f = || Formula::fresh(0, Dimension::Length);
    let fresh = vec![VarDecl::Free(FreeVar::continuous_with(Dimension::Length, 0.25,
        Distribution::Normal { sigma: 0.002 }))];
    let a = step(&doc, point([f(), f(), len(0.0)], fresh)).unwrap();
    let id = a.record.minted.unwrap();
    let shared = a.doc.slot(id, SlotId::Origin(editor_core::Axis3::X)).unwrap();
    let b = step(&a.doc, DocEdit::SetParam { node: id, slot: SlotId::Origin(editor_core::Axis3::X),
        expr: len(1.0), fresh: vec![] }).unwrap();
    assert!(!b.maintenance.iter().any(|m| matches!(m, Maintenance::AnonymousVarRemoved { var } if var.id() == shared)),
        "{:?}", b.maintenance);
    assert!(b.doc.var(shared).is_some());
    // rewrite the second reader too: now it retires, once
    let c = step(&b.doc, DocEdit::SetParam { node: id, slot: SlotId::Origin(editor_core::Axis3::Y),
        expr: len(1.0), fresh: vec![] }).unwrap();
    let removed: Vec<_> = c.maintenance.iter().filter(|m| matches!(m, Maintenance::AnonymousVarRemoved { .. })).collect();
    eprintln!("{removed:?}");
    assert!(c.doc.var(shared).is_none());
    assert!(c.doc.has_minted_var(shared));
}

/// Cross-node: a second node reading the first's anonymous variable by
/// id keeps it alive when the first is rewritten; undo is the old doc.
#[test]
fn r2_anonymous_read_across_nodes_is_not_retired() {
    let doc = empty("r2-cross");
    let a = step(&doc, point([len(0.25), len(0.0), len(0.0)], vec![])).unwrap();
    let pa = a.record.minted.unwrap();
    let anon = a.doc.slot(pa, SlotId::Origin(editor_core::Axis3::X)).unwrap();
    let b = step(&a.doc, point([Formula::var(anon, Dimension::Length), len(1.0), len(0.0)], vec![])).unwrap();
    let c = step(&b.doc, DocEdit::SetParam { node: pa, slot: SlotId::Origin(editor_core::Axis3::X),
        expr: len(2.0), fresh: vec![] }).unwrap();
    assert!(c.doc.var(anon).is_some(), "still read by the second point: {:?}", c.maintenance);
    let text = save(&c.doc, &[], Tol::witness()).unwrap();
    load(&text, Tol::witness()).expect("loads");
}

/// Fresh self-read and a definition reading an id the table will mint.
#[test]
fn r2_fresh_self_read_refuses() {
    let doc = empty("r2-self");
    let e = point([Formula::fresh(0, Dimension::Length), len(0.0), len(0.0)],
        vec![VarDecl::Defined(Formula::mul(Formula::fresh(0, Dimension::Length), crate::fixture::scl(2.0)).unwrap())]);
    let r = step(&doc, e);
    eprintln!("{:?}", r.as_ref().err());
    assert!(r.is_err());
}

/// A fresh entry read only by another fresh entry nothing reads.
#[test]
fn r2_fresh_entry_read_only_by_unread_entry_refuses() {
    let doc = empty("r2-unread2");
    let e = point([len(0.0), len(0.0), len(0.0)],
        vec![VarDecl::Free(FreeVar::continuous(Dimension::Length, 1.0)),
             VarDecl::Defined(Formula::mul(Formula::fresh(0, Dimension::Length), crate::fixture::scl(2.0)).unwrap())]);
    let r = step(&doc, e);
    eprintln!("{:?}", r.as_ref().err());
    assert!(matches!(r, Err(EditError::FreshUnread { .. })));
}

/// Same value typed in two edits on one document: distinct ids.
#[test]
fn r2_same_value_two_edits_two_ids() {
    let doc = empty("r2-twice");
    let a = step(&doc, point([len(0.25), len(0.25), len(0.25)], vec![])).unwrap();
    let pa = a.record.minted.unwrap();
    let b = step(&a.doc, point([len(0.25), len(0.25), len(0.25)], vec![])).unwrap();
    let pb = b.record.minted.unwrap();
    let mut ids = std::collections::BTreeSet::new();
    for p in [pa, pb] { for ax in [editor_core::Axis3::X, editor_core::Axis3::Y, editor_core::Axis3::Z] {
        assert!(ids.insert(b.doc.slot(p, SlotId::Origin(ax)).unwrap()));
    }}
    assert_ne!(pa, pb);
}

/// The id-masked corpus digest at Interval (the f64 masked twin is
/// `m10_p_fence`'s): run on base and head, compare by hand.
#[test]
fn r2_masked_interval_digest() {
    use crate::m10_p_fence::{Seen, Outcome, walk};
    use geom_core::{Bounds, Interval};
    let mut docs: Vec<(String, std::collections::BTreeMap<u64, (String, Vec<u64>)>)> = Vec::new();
    let mut fx: Vec<u64> = Vec::new();
    walk::<Interval>(|seen| match seen {
        Seen::Fixture(i) => fx.push(i as u64),
        Seen::FixtureLoop { vertices, .. } => fx.push(vertices as u64),
        Seen::FixtureVertex { x, y, sweep, .. } => for c in [x, y, sweep] { fx.push(c.lo().to_bits()); fx.push(c.hi().to_bits()); },
        Seen::FixtureRefused(_) => fx.push(u64::MAX),
        Seen::Document(n) => docs.push((n.to_owned(), Default::default())),
        Seen::Node { id, outcome, .. } => { let o = match outcome { Outcome::Poisoned{..} => "p".to_owned(), Outcome::Failed => "f".to_owned(), Outcome::Ok{kind} => kind.to_owned() };
            docs.last_mut().unwrap().1.insert(id, (o, Vec::new())); }
        Seen::Point { id, p, .. } => { let v = &mut docs.last_mut().unwrap().1.get_mut(&id).unwrap().1;
            for c in p.to_array() { v.push(c.lo().to_bits()); v.push(c.hi().to_bits()); } }
    });
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    fx.hash(&mut h);
    let mut per_doc = Vec::new();
    for (name, nodes) in docs {
        let mut held: Vec<(String, Vec<u64>)> = nodes.into_values().map(|(o, mut p)| { let mut ch: Vec<[u64;6]> = p.chunks(6).map(|c| c.try_into().unwrap()).collect(); ch.sort_unstable(); p = ch.concat(); (o, p) }).collect();
        held.sort();
        let mut hd = std::collections::hash_map::DefaultHasher::new();
        held.hash(&mut hd);
        per_doc.push((name.clone(), hd.finish()));
        name.hash(&mut h); held.hash(&mut h);
    }
    for (n, d) in &per_doc { println!("R2MASK {n} {d:016x}"); }
    println!("R2MASK fixture+all {:016x}", h.finish());
}

/// A stale handle: a slot written to read a retired anonymous id.
#[test]
fn r2_slot_reading_a_retired_anonymous_id() {
    let doc = empty("r2-stale");
    let a = step(&doc, point([len(0.25), len(0.0), len(0.0)], vec![])).unwrap();
    let pa = a.record.minted.unwrap();
    let anon = a.doc.slot(pa, SlotId::Origin(editor_core::Axis3::X)).unwrap();
    let b = step(&a.doc, DocEdit::SetParam { node: pa, slot: SlotId::Origin(editor_core::Axis3::X),
        expr: len(2.0), fresh: vec![] }).unwrap();
    assert!(b.doc.var(anon).is_none());
    let r = step(&b.doc, point([Formula::var(anon, Dimension::Length), len(1.0), len(0.0)], vec![]));
    eprintln!("R2STALE {:?}", r.as_ref().err());
    assert!(r.is_err(), "a new reader of a retired anonymous variable is admitted");
}
