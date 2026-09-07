//! **A merged face's name is a flat constituent set, and a
//! member-space declaration resolves through the fold's merges**
//! (DOCM-8; DM4's "merges and order" sentence): the flat mint at the
//! pair emitter, the look-through at the union's routing step, and
//! what neither of them does.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::docm7_union_declare::{
    block, body_of, declared_union, failure, flush_pairs, member_face, run, table,
};
use crate::fixture;
use crate::fixture::{Recorder, fname, insert, len, wall};

use editor_core::{
    BooleanOp, CapEnd, EntityKind, Entry, NameTable, NamingError, Node, NodeErrorKind, ProfileDoc,
    RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, StableName, resolve,
};
use geom_core::Tol;

/// The four flush families two x-offset blocks share: both y-walls
/// and both caps.
fn flush_segs() -> [RoleSeg; 4] {
    [
        wall(0),
        wall(2),
        RoleSeg::Cap(CapEnd::Start),
        RoleSeg::Cap(CapEnd::End),
    ]
}

/// Every ordering of `items`.
fn permutations(items: &[RecipeNodeId]) -> Vec<Vec<RecipeNodeId>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for (i, &head) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut tail in permutations(&rest) {
            tail.insert(0, head);
            out.push(tail);
        }
    }
    out
}

/// The declared contacts of a CHAIN: every consecutive pair of
/// `chain` meets flush, all four families, in member space.
fn chain_pairs(union: RecipeNodeId, chain: &[RecipeNodeId]) -> Vec<(StableName, StableName)> {
    chain
        .windows(2)
        .flat_map(|w| flush_pairs(union, (w[0], w[0]), (w[1], w[1])))
        .collect()
}

/// The merged row a whole chain's family fuses to: one `Merged` over
/// every chain member's face of that family, sorted.
fn chain_merged(union: RecipeNodeId, chain: &[RecipeNodeId], seg: RoleSeg) -> StableName {
    let mut set: Vec<StableName> = chain
        .iter()
        .map(|&m| member_face(union, m, fname(m, seg.clone())))
        .collect();
    set.sort();
    StableName {
        kind: EntityKind::Face,
        node: union,
        path: vec![RoleSeg::Merged(set)],
    }
}

/// The fused body's merged rows are exactly the four chain families,
/// each a FLAT set over every chain member, whatever order was folded.
fn assert_chain_rows(t: &NameTable, union: RecipeNodeId, chain: &[RecipeNodeId], label: &str) {
    let merged = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .count();
    assert_eq!(merged, 4, "{label}: four merged rows, one per family");
    for seg in flush_segs() {
        let row = chain_merged(union, chain, seg.clone());
        assert!(
            matches!(t.lookup(&row), Some(Entry::Unique(_))),
            "{label}: the chain's {seg:?} row {row:?} is not published"
        );
    }
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass")
        .volume
}

// ---------------------------------------------------------------------
// A1 — the chain fuses in every member order.
// ---------------------------------------------------------------------

/// **Three chained blocks fuse in every member order**, and the fused
/// cap's row is the same flat set in each.
///
/// `a` meets `c` and `c` meets `d`; both contacts are declared in
/// member space and nothing else is. The orders that fold `c` in
/// after one of its neighbours find `c`'s face already inside a
/// `Merged` row, and the pair resolves to that row through the
/// look-through; the orders that fold it in last find both faces as
/// rows. Either way the merge lists the three member faces.
#[test]
fn member_space_declarations_survive_every_order() {
    let doc = ProfileDoc::empty_derived("docm8_chain3", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let chain = [a, c, d];
    for order in permutations(&chain) {
        let label = format!("order {order:?}");
        let (docx, union, _) = declared_union(doc.clone(), &order, |u| chain_pairs(u, &chain));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(&body_of(&ev, union));
        assert!(
            (v - 2.2).abs() < 1e-9,
            "{label}: one fused body, got volume {v}"
        );
        assert_chain_rows(table(&ev, union), union, &chain, &label);
    }
}

/// **A four-member chain (two middles) fuses in every order**: the
/// look-through composes, since the row it rewrites to is itself
/// flat and is what the next merge lists.
#[test]
fn a_four_member_chain_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("docm8_chain4", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, e) = block(doc, (1.9, 2.9), (0.0, 1.0), 0.0, 1.0);
    let chain = [a, c, d, e];
    for order in permutations(&chain) {
        let label = format!("order {order:?}");
        let (docx, union, _) = declared_union(doc.clone(), &order, |u| chain_pairs(u, &chain));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(&body_of(&ev, union));
        assert!(
            (v - 2.9).abs() < 1e-9,
            "{label}: one fused body, got volume {v}"
        );
        assert_chain_rows(table(&ev, union), union, &chain, &label);
    }
}

/// **A chain with a disjoint member fuses in every order too**: the
/// member that touches nothing takes no part in any merge, wherever
/// it sits in the list, and the chain's rows are the chain's alone.
#[test]
fn a_chain_with_a_disjoint_member_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("docm8_chain_far", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let chain = [a, c, d];
    for order in permutations(&[a, c, d, far]) {
        let label = format!("order {order:?}");
        let (docx, union, _) = declared_union(doc.clone(), &order, |u| chain_pairs(u, &chain));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(&body_of(&ev, union));
        assert!(
            (v - 3.2).abs() < 1e-9,
            "{label}: the chain plus the far block, got volume {v}"
        );
        assert_chain_rows(table(&ev, union), union, &chain, &label);
    }
}

// ---------------------------------------------------------------------
// A2 — no nested `Merged` is ever published.
// ---------------------------------------------------------------------

/// **No corpus document publishes a merged face with a merged
/// constituent.** The same walker the naming suites run on every
/// evaluation (`fixture::assert_no_nested_merged`), over the whole
/// registry — `corner_table`'s three chained pair unions included,
/// which is where a merge of a merged face is minted.
#[test]
fn no_corpus_document_publishes_a_nested_merged_row() {
    for d in corpus::documents() {
        let ev = corpus::eval::<f64>(&d.doc);
        fixture::assert_no_nested_merged(&ev);
    }
}

// ---------------------------------------------------------------------
// A3 — the pair boolean's own mint is flat.
// ---------------------------------------------------------------------

/// A block on the xy plane, recorded.
fn recorded_block(rec: &mut Recorder, (x0, x1): (f64, f64)) -> RecipeNodeId {
    let p = rec.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, 0.0), (x1, 0.0), (x1, 1.0), (x0, 1.0)]],
    );
    rec.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    })
}

fn from_a(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::FromA(Box::new(inner))],
    }
}

fn from_b(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::FromB(Box::new(inner))],
    }
}

fn merged(node: RecipeNodeId, mut set: Vec<StableName>) -> StableName {
    set.sort();
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Merged(set)],
    }
}

/// **A boolean over a boolean that declared a merge mints the outer
/// merged face FLAT**: the inner merge's constituents, each wrapped
/// `FromA`, plus the partner wrapped `FromB` — no `Merged` inside —
/// and the edit-log replay of the document is bit-identical.
#[test]
fn a_boolean_over_a_boolean_mints_a_flat_merged_row_and_replays() {
    let tol = Tol::witness();
    let mut rec = Recorder::new();
    let a = recorded_block(&mut rec, (0.0, 1.0));
    let b = recorded_block(&mut rec, (0.5, 1.5));
    let decl_ab = rec.insert(Node::declare_rest(
        flush_segs()
            .into_iter()
            .map(|seg| (fname(a, seg.clone()), fname(b, seg)))
            .collect(),
    ));
    let inner = rec.insert(Node::Boolean {
        op: BooleanOp::Union,
        a,
        b,
        declare: Some(decl_ab),
    });
    let c = recorded_block(&mut rec, (1.2, 2.2));
    // The inner's merged rows, declared against `c`'s faces by name.
    let inner_row = |seg: RoleSeg| {
        merged(
            inner,
            vec![
                from_a(inner, fname(a, seg.clone())),
                from_b(inner, fname(b, seg)),
            ],
        )
    };
    let decl_ic = rec.insert(Node::declare_rest(
        flush_segs()
            .into_iter()
            .map(|seg| (inner_row(seg.clone()), fname(c, seg)))
            .collect(),
    ));
    let outer = rec.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: inner,
        b: c,
        declare: Some(decl_ic),
    });
    let ev = run(&rec.doc);
    assert!(failure(&ev, outer).is_none(), "{:?}", failure(&ev, outer));
    let t = table(&ev, outer);
    for seg in flush_segs() {
        let row = merged(
            outer,
            vec![
                from_a(outer, from_a(inner, fname(a, seg.clone()))),
                from_a(outer, from_b(inner, fname(b, seg.clone()))),
                from_b(outer, fname(c, seg.clone())),
            ],
        );
        assert!(
            matches!(t.lookup(&row), Some(Entry::Unique(_))),
            "the outer merge of {seg:?} is not the flat row {row:?}"
        );
        // And the inner's merged row is retired into it: not a row of
        // the outer, wrapped or not.
        assert!(
            t.lookup(&from_a(outer, inner_row(seg))).is_none(),
            "the inner merged row survived as an outer row"
        );
    }
    let merged_rows = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .count();
    assert_eq!(merged_rows, 4);
    // The edit-log replay is the same document with the same table.
    let empty = ProfileDoc::empty_derived("mod", tol);
    let text = editor_core::persist::save(&empty, &rec.edits, tol).expect("saves");
    let loaded = editor_core::persist::load(&text, tol).expect("loads");
    assert!(
        loaded.doc.bit_eq(&rec.doc),
        "the log replay is not the direct document"
    );
    let replayed = run(&loaded.doc);
    assert!(failure(&replayed, outer).is_none());
    assert_eq!(table(&replayed, outer), t);
    assert_eq!(
        replayed.value(outer).unwrap().content_key,
        ev.value(outer).unwrap().content_key
    );
}

// ---------------------------------------------------------------------
// A4 — the look-through is the union's alone, and what it refuses.
// ---------------------------------------------------------------------

/// **A fold row absorbed by a later merge keeps the vanished rung.**
/// A `Merged` row the fold minted at step 1 and absorbed into a wider
/// merge at step 2 is no row at step 3 — an accumulation-entity name
/// does not look through — and a pair naming it there refuses
/// `Vanished`, not `UnionDeclareStep`: the row vanished INTO a merge,
/// and the merged row that lists its faces is N3's offer for it
/// (`merge_offers`), so the routing diagnosis does not re-say it.
#[test]
fn an_absorbed_fold_row_keeps_the_vanished_rung_and_does_not_look_through() {
    let doc = ProfileDoc::empty_derived("docm8_no_lookthrough", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, e) = block(doc, (1.9, 2.9), (0.0, 1.0), 0.0, 1.0);
    let chain = [a, c, d, e];
    let (doc, union, _) = declared_union(doc, &chain, |u| {
        let mut v = chain_pairs(u, &chain);
        // Step 1's cap row, paired with `e`: routed to `e`'s step,
        // where the row has already been merged into `{a, c, d}`.
        v.push((
            chain_merged(u, &[a, c], RoleSeg::Cap(CapEnd::Start)),
            member_face(u, e, fname(e, RoleSeg::Cap(CapEnd::Start))),
        ));
        v
    });
    let ev = run(&doc);
    let absorbed = chain_merged(union, &[a, c], RoleSeg::Cap(CapEnd::Start));
    assert!(
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::DeclareResolve { error })
                if matches!(&**error, ResolveError::Vanished { name, .. } if *name == absorbed)
        ),
        "expected the vanished rung on the absorbed row, got {:?}",
        failure(&ev, union)
    );
}

/// **A member face the fold consumed WITHOUT a merge keeps the
/// vanished refusal.** `a`'s x = 1 wall lies inside `big` and is gone
/// after step 1 — in no table and in no merged row's set — so a pair
/// naming it at step 2 is refused as the vanished name it is, exactly
/// as before the look-through existed.
#[test]
fn a_member_face_in_no_table_and_no_merged_row_keeps_the_vanished_refusal() {
    let doc = ProfileDoc::empty_derived("docm8_vanished", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, big) = block(doc, (0.5, 3.0), (-1.0, 2.0), -1.0, 3.0);
    let (doc, far) = block(doc, (6.0, 7.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, _) = declared_union(doc, &[a, big, far], |u| {
        vec![(
            member_face(u, a, fname(a, wall(1))),
            member_face(u, far, fname(far, wall(3))),
        )]
    });
    let ev = run(&doc);
    assert!(
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::DeclareResolve { .. })
        ),
        "expected the vanished refusal, got {:?}",
        failure(&ev, union)
    );
}

// ---------------------------------------------------------------------
// N3's offer over a flat set, and the mint's second consumer shapes.
// ---------------------------------------------------------------------

/// **The inner merged face a boolean over a boolean consumed still
/// offers the outer row.** With the flat mint it is a constituent of
/// nothing — the outer row lists its faces re-wrapped — and N3's
/// "referencing one fails with the merged name offered" is kept by
/// reading the set as COVERING it (`names/merged.rs`): every face it
/// stood for is in the outer set. The offers are the base's.
#[test]
fn a_consumed_inner_merged_face_offers_the_outer_flat_row() {
    let mut rec = Recorder::new();
    let a = recorded_block(&mut rec, (0.0, 1.0));
    let b = recorded_block(&mut rec, (0.5, 1.5));
    let decl_ab = rec.insert(Node::declare_rest(
        flush_segs()
            .into_iter()
            .map(|seg| (fname(a, seg.clone()), fname(b, seg)))
            .collect(),
    ));
    let inner = rec.insert(Node::Boolean {
        op: BooleanOp::Union,
        a,
        b,
        declare: Some(decl_ab),
    });
    let c = recorded_block(&mut rec, (1.2, 2.2));
    let inner_row = |seg: RoleSeg| {
        merged(
            inner,
            vec![
                from_a(inner, fname(a, seg.clone())),
                from_b(inner, fname(b, seg)),
            ],
        )
    };
    let decl_ic = rec.insert(Node::declare_rest(
        flush_segs()
            .into_iter()
            .map(|seg| (inner_row(seg.clone()), fname(c, seg)))
            .collect(),
    ));
    let outer = rec.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: inner,
        b: c,
        declare: Some(decl_ic),
    });
    let ev = run(&rec.doc);
    assert!(failure(&ev, outer).is_none(), "{:?}", failure(&ev, outer));
    let ctx = RunCtx {
        doc: &rec.doc,
        eval: &ev,
    };
    for seg in flush_segs() {
        let outer_row = merged(
            outer,
            vec![
                from_a(outer, from_a(inner, fname(a, seg.clone()))),
                from_a(outer, from_b(inner, fname(b, seg.clone()))),
                from_b(outer, fname(c, seg.clone())),
            ],
        );
        // The consumed operand face: the inner merge, wrapped once.
        let consumed = from_a(outer, inner_row(seg.clone()));
        match resolve(ctx, &consumed) {
            Resolution::Failed(f) => assert!(
                f.offers.contains(&outer_row),
                "{seg:?}: the consumed inner merged face does not offer the outer row: {:?}",
                f.offers
            ),
            other => panic!("{seg:?}: the consumed face resolved: {other:?}"),
        }
        // And a flat constituent — a name that was never a row of any
        // table — offers the same row.
        let constituent = from_a(outer, from_a(inner, fname(a, seg)));
        match resolve(ctx, &constituent) {
            Resolution::Failed(f) => assert!(f.offers.contains(&outer_row), "{:?}", f.offers),
            other => panic!("{other:?}"),
        }
    }
}

/// **A merged face carried through a step as operand B, then merged
/// again, is minted flat** — the `FromB` half of reading a name
/// through its wrappers. A union's accumulation is always operand A
/// and the corpus chains `Boolean { a: previous, b: new }`, so nothing
/// else exercises it.
#[test]
fn a_merged_face_passed_through_as_operand_b_is_still_flat() {
    let doc = ProfileDoc::empty_derived("docm8_fromb", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, decl_ab) = insert(
        doc,
        Node::declare_rest(
            flush_segs()
                .into_iter()
                .map(|seg| (fname(a, seg.clone()), fname(b, seg)))
                .collect(),
        ),
    );
    let (doc, inner) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl_ab),
        },
    );
    // A far block, unioned with the merge as operand B, so the merged
    // rows ride through as `mid:FromB(inner:Merged(..))`.
    let (doc, far) = block(doc, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, mid) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: far,
            b: inner,
            declare: None,
        },
    );
    let (doc, c) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let carried = |seg: RoleSeg| {
        from_b(
            mid,
            merged(
                inner,
                vec![
                    from_a(inner, fname(a, seg.clone())),
                    from_b(inner, fname(b, seg)),
                ],
            ),
        )
    };
    let (doc, decl_mc) = insert(
        doc,
        Node::declare_rest(
            flush_segs()
                .into_iter()
                .map(|seg| (carried(seg.clone()), fname(c, seg)))
                .collect(),
        ),
    );
    let (doc, outer) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: mid,
            b: c,
            declare: Some(decl_mc),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, outer).is_none(), "{:?}", failure(&ev, outer));
    let t = table(&ev, outer);
    for seg in flush_segs() {
        let want = merged(
            outer,
            vec![
                from_a(outer, from_b(mid, from_a(inner, fname(a, seg.clone())))),
                from_a(outer, from_b(mid, from_b(inner, fname(b, seg.clone())))),
                from_b(outer, fname(c, seg.clone())),
            ],
        );
        assert!(
            matches!(t.lookup(&want), Some(Entry::Unique(_))),
            "the FromB pass-through did not mint flat for {seg:?}: {want:?}"
        );
    }
}

// ---------------------------------------------------------------------
// The bound: merges only. Measured, not argued.
// ---------------------------------------------------------------------

/// **A declared member face SPLIT by a later member is still
/// order-shaped** — the bound the look-through does not cross,
/// asserted as measured. `a` and `c` meet flush along x; `s` sits on
/// `a`'s top cap with its footprint strictly inside it, so folding `s`
/// in fragments that cap. The orders that fold `s` last fuse; the
/// orders that fold it before `c` refuse `Vanished` on `a`'s end cap
/// (neither a row nor in any merged row's flat set); the orders that
/// fold it before `a` refuse the emitter's seam-vertex `Emission`
/// (`work/docm/two-emitter-refusals-a-legal-declared-union-reaches.md`).
/// Filed as
/// `work/docm/member-space-look-through-stops-at-splits-containment-and-fragmented-merges.md`.
#[test]
fn a_member_face_split_by_a_later_member_is_still_order_shaped() {
    let doc = ProfileDoc::empty_derived("docm8_split_order", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block(doc, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let pairs = move |u: RecipeNodeId| {
        let mut v = flush_pairs(u, (a, a), (c, c));
        for seg in [wall(0), wall(2)] {
            v.push((
                member_face(u, a, fname(a, seg.clone())),
                member_face(u, s, fname(s, seg)),
            ));
        }
        v
    };
    #[derive(Debug, PartialEq)]
    enum Outcome {
        Fused,
        VanishedEndCapOfA,
        SeamVertexEmission,
    }
    let a_end = |u: RecipeNodeId| member_face(u, a, fname(a, RoleSeg::Cap(CapEnd::End)));
    for (order, want) in [
        (vec![a, c, s], Outcome::Fused),
        (vec![c, a, s], Outcome::Fused),
        (vec![a, s, c], Outcome::VanishedEndCapOfA),
        (vec![s, a, c], Outcome::VanishedEndCapOfA),
        (vec![c, s, a], Outcome::SeamVertexEmission),
        (vec![s, c, a], Outcome::SeamVertexEmission),
    ] {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs);
        let ev = run(&docx);
        let got = match failure(&ev, union) {
            None => Outcome::Fused,
            Some(NodeErrorKind::DeclareResolve { error }) if matches!(&**error, ResolveError::Vanished { name, .. } if *name == a_end(union)) => {
                Outcome::VanishedEndCapOfA
            }
            Some(NodeErrorKind::Naming(NamingError::Emission { what }))
                if what.starts_with("seam vertex parentage") =>
            {
                Outcome::SeamVertexEmission
            }
            other => panic!("{order:?}: unexpected outcome {other:?}"),
        };
        assert_eq!(got, want, "{order:?}");
        if got == Outcome::Fused {
            let v = volume(&body_of(&ev, union));
            assert!((v - 1.6).abs() < 1e-9, "{order:?}: volume {v}");
        }
    }
}
