//! Reviewer probes for PR 5763 (split carries a space by the variable
//! rule). Not for merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::fixture;
use crate::fixture::resolver::{PartStore, with_resolver};
use crate::fixture::round_trip::{composed, flat, same_up_to_ids};
use editor_core::{
    DocEdit, DocumentId, Maintenance, Node, Operand, PartResolver, ProfileDoc, RecipeNodeId,
    SplitError, SplitOutcome, VarId,
};
use fixture::{insert, len, on_frame, square, step};
use geom_core::Tol;

fn block(doc: ProfileDoc, x: f64) -> (ProfileDoc, [RecipeNodeId; 2]) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(x, 0.0, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    (doc, [profile, extrude])
}

fn place_into(doc: ProfileDoc, body: RecipeNodeId, target: VarId) -> (ProfileDoc, RecipeNodeId) {
    let (doc, placement) = step(
        doc,
        DocEdit::place(
            vec![body.into()],
            Operand::Var(target),
            editor_core::stand().bundle(),
        ),
    );
    (doc, placement.expect("the placement mints"))
}

fn new_space(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, VarId) {
    let (doc, node) = insert(doc, Node::Space);
    let var = doc.output(node, 0).expect("space");
    (doc, node, var)
}

fn split(doc: &ProfileDoc, cut: &[RecipeNodeId], seed: &str) -> Result<SplitOutcome, SplitError> {
    editor_core::split(
        doc,
        &cut.iter().copied().collect::<BTreeSet<_>>(),
        DocumentId::derive(seed),
        Tol::witness(),
        None,
    )
}

fn round_trips(doc: &ProfileDoc, out: &SplitOutcome, seed: &str) {
    let mut store = PartStore::default();
    store.insert(out.part.clone(), Tol::witness());
    let store: Arc<dyn PartResolver> = Arc::new(store);
    let back = editor_core::inline(&out.remainder, out.instance, &store, Tol::witness())
        .unwrap_or_else(|e| panic!("{seed}: inlines back: {e}"));
    let (nodes, steps) = composed(doc, out, &back);
    same_up_to_ids(doc, &flat(&back), &nodes, &steps)
        .unwrap_or_else(|e| panic!("{seed}: round trip:\n{e}"));
}

fn no_strand(out: &SplitOutcome, seed: &str) {
    assert!(
        !out.remainder_maintenance
            .iter()
            .chain(out.part_maintenance.iter())
            .any(|m| matches!(m, Maintenance::StrandedRead { .. })),
        "{seed}: strand: {:?} / {:?}",
        out.remainder_maintenance,
        out.part_maintenance
    );
}

/// Three blocks placed into one space, each copy placed into the world.
fn three(seed: &str) -> (ProfileDoc, RecipeNodeId, VarId, Vec<[RecipeNodeId; 4]>) {
    let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let (mut doc, space, var) = new_space(doc);
    let mut rows = Vec::new();
    for x in [0.0, 5.0, 10.0] {
        let (d, [p, b]) = block(doc, x);
        let (d, inner) = place_into(d, b, var);
        let (d, outer) = fixture::place(d, inner);
        doc = d;
        rows.push([p, b, inner, outer]);
    }
    (doc, space, var, rows)
}

#[test]
fn probe_three_readers_split_two_one_refuses() {
    let (doc, _space, var, rows) = three("rv-three-21");
    let cut: Vec<_> = rows[..2].iter().flatten().copied().collect();
    let err = split(&doc, &cut, "rv-three-21-part").expect_err("shared");
    assert!(
        matches!(&err, SplitError::UncutVarReference { var: v, cut_node, kept_node }
            if v.id() == var && (cut_node.id() == rows[0][2] || cut_node.id() == rows[1][2])
               && kept_node.id() == rows[2][2]),
        "{err:?}"
    );
    eprintln!("2/1: {err}");
}

#[test]
fn probe_three_readers_all_cut_round_trip() {
    let (doc, space, _var, rows) = three("rv-three-30");
    let cut: Vec<_> = rows.iter().flatten().copied().collect();
    let out = split(&doc, &cut, "rv-three-30-part").unwrap_or_else(|e| panic!("{e}"));
    assert!(out.remainder.node(space).is_none());
    no_strand(&out, "3/0");
    round_trips(&doc, &out, "3/0");
}

/// a into S1, that copy into S2, that copy into the world; `late`
/// re-targets each placement into a space minted after every reader.
fn nested(seed: &str, late: bool) -> (ProfileDoc, Vec<RecipeNodeId>, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
    let (doc, [_, k]) = block(doc, 20.0);
    let (doc, _) = fixture::place(doc, k);
    let (doc, [p, a]) = block(doc, 0.0);
    if late {
        let (doc, p1) = place_into(doc, a, VarId::WORLD);
        let (doc, p2) = place_into(doc, p1, VarId::WORLD);
        let (doc, p3) = fixture::place(doc, p2);
        let (doc, s1) = fixture::into_new_space(doc, p1);
        let (doc, s2) = fixture::into_new_space(doc, p2);
        let n1 = doc.operation_of(s1).unwrap();
        let n2 = doc.operation_of(s2).unwrap();
        assert_eq!(doc.node_space(p1), Some(s1));
        assert_eq!(doc.node_space(p2), Some(s2));
        (doc, vec![p, a, p1, p2, p3], vec![n1, n2])
    } else {
        let (doc, n1, s1) = new_space(doc);
        let (doc, p1) = place_into(doc, a, s1);
        let (doc, n2, s2) = new_space(doc);
        let (doc, p2) = place_into(doc, p1, s2);
        let (doc, p3) = fixture::place(doc, p2);
        (doc, vec![p, a, p1, p2, p3], vec![n1, n2])
    }
}

#[test]
fn probe_nested_spaces_round_trip() {
    for late in [false, true] {
        for named in [false, true] {
            let seed = format!("rv-nested-{late}-{named}");
            let (doc, mut cut, spaces) = nested(&seed, late);
            if named {
                cut.extend(&spaces);
            }
            let out = split(&doc, &cut, &format!("{seed}-part"))
                .unwrap_or_else(|e| panic!("{seed}: {e}"));
            for s in &spaces {
                assert!(out.node_map.contains_key(s), "{seed}: space carried");
                assert!(out.remainder.node(*s).is_none(), "{seed}: space left");
            }
            no_strand(&out, &seed);
            round_trips(&doc, &out, &seed);
        }
    }
}

/// Only the outer chain cut: p2 (into S2) and p3; p1 in S1 kept.
#[test]
fn probe_nested_partial_cut_refuses() {
    for late in [false, true] {
        let seed = format!("rv-nested-partial-{late}");
        let (doc, cut, _spaces) = nested(&seed, late);
        let err = split(&doc, &cut[3..], &format!("{seed}-part"));
        eprintln!("{seed}: {:?}", err.as_ref().map(|_| ()).map_err(|e| e.to_string()));
        assert!(err.is_err(), "{seed}: the cut copy reads a kept placement");
    }
}

#[test]
fn probe_unread_space_stays_or_goes_if_named() {
    let doc = ProfileDoc::empty(DocumentId::derive("rv-unread"), Tol::witness());
    let (doc, space, _) = new_space(doc);
    let (doc, [_, k]) = block(doc, 9.0);
    let (doc, _) = fixture::place(doc, k);
    let (doc, [p, a]) = block(doc, 0.0);
    let (doc, w) = fixture::place(doc, a);
    let out = split(&doc, &[p, a, w], "rv-unread-part").unwrap();
    assert!(out.remainder.node(space).is_some(), "unread unnamed stays");
    round_trips(&doc, &out, "unread-unnamed");
    let out = split(&doc, &[p, a, w, space], "rv-unread-named-part").unwrap();
    assert!(out.remainder.node(space).is_none(), "unread named goes");
    no_strand(&out, "unread-named");
    round_trips(&doc, &out, "unread-named");
}

/// A placement into a construction's own space, all of it cut.
#[test]
fn probe_construction_space_all_cut_round_trip() {
    let doc = ProfileDoc::empty(DocumentId::derive("rv-own-all"), Tol::witness());
    let (doc, [_, k]) = block(doc, 20.0);
    let (doc, _) = fixture::place(doc, k);
    let (doc, [pa, a]) = block(doc, 0.0);
    let (doc, wa) = fixture::place(doc, a);
    let (doc, [pb, b]) = block(doc, 5.0);
    let own = fixture::space(&doc, a);
    let (doc, into_a) = place_into(doc, b, own);
    let cut = [pa, a, wa, pb, b, into_a];
    let out = split(&doc, &cut, "rv-own-all-part").unwrap_or_else(|e| panic!("{e}"));
    no_strand(&out, "own-all");
    round_trips(&doc, &out, "own-all");
}

/// Inline of a stored part whose `new_space()` is minted after its
/// reader (the carry's space-first deal, inline direction).
#[test]
fn probe_inline_part_with_late_space() {
    let part = ProfileDoc::empty(DocumentId::derive("rv-late-part"), Tol::witness());
    let (part, [_, a]) = block(part, 0.0);
    let (part, p1) = place_into(part, a, VarId::WORLD);
    let (part, _p2) = fixture::place(part, p1);
    let (part, s) = fixture::into_new_space(part, p1);
    let n = part.operation_of(s).unwrap();
    assert!(
        part.ids().iter().position(|&i| i == n) > part.ids().iter().position(|&i| i == p1),
        "late"
    );
    let mut store = PartStore::default();
    let doc_ref = store.insert(part, Tol::witness());
    let o = with_resolver(store.clone());
    let host = ProfileDoc::empty(DocumentId::derive("rv-late-host"), Tol::witness());
    let (host, inst) = fixture::insert_pinned(host, fixture::mated_instance(doc_ref), &o);
    let store: Arc<dyn PartResolver> = Arc::new(store);
    let back = editor_core::inline(&host, inst, &store, Tol::witness())
        .unwrap_or_else(|e| panic!("inline of a late-space part: {e}"));
    let f = flat(&back);
    assert!(f.ids().iter().any(|&i| matches!(f.node(i), Some(Node::Space))));
}

/// The same cut with no space anywhere: is the order fault the space's?
#[test]
fn probe_unread_control_no_space() {
    let doc = ProfileDoc::empty(DocumentId::derive("rv-unread-ctl"), Tol::witness());
    let (doc, [_, k]) = block(doc, 9.0);
    let (doc, _) = fixture::place(doc, k);
    let (doc, [p, a]) = block(doc, 0.0);
    let (doc, w) = fixture::place(doc, a);
    let out = split(&doc, &[p, a, w], "rv-unread-ctl-part").unwrap();
    round_trips(&doc, &out, "control");
}
