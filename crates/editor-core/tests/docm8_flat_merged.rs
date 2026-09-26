//! **A merged face's name is a flat constituent set, and a
//! member-space declaration resolves through the fold's merges**
//! (DOCM-8; DM4's "merges and order" sentence): the flat mint at the
//! pair emitter, the look-through at the union's routing step, and
//! what neither of them does.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, member_face, run};
use crate::fixture;
use crate::fixture::{Recorder, flush_segs, fname, insert, len, table, wall};

use editor_core::{
    BooleanOp, CapEnd, EntityKind, Entry, NameTable, NamingError, Node, NodeErrorKind, ProfileDoc,
    RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SitedRef, StableName, resolve,
};
use geom_core::Tol;

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
fn chain_pairs(doc: &editor_core::ProfileDoc, chain: &[RecipeNodeId]) -> Vec<(SitedRef, SitedRef)> {
    chain
        .windows(2)
        .flat_map(|w| flush_pairs(doc, (w[0], w[0]), (w[1], w[1])))
        .collect()
}

/// Flush family `fam` of the extrude `ext` — [`flush_segs`]'s `fam`th,
/// spelled by `ext`'s own pieces.
fn family(doc: &ProfileDoc, ext: RecipeNodeId, fam: usize) -> RoleSeg {
    flush_segs(doc, ext)[fam].clone()
}

/// The merged row a whole chain's family fuses to: one `Merged` over
/// every chain member's face of that family, sorted.
fn chain_merged(
    doc: &ProfileDoc,
    union: RecipeNodeId,
    chain: &[RecipeNodeId],
    fam: usize,
) -> StableName {
    let mut set: Vec<StableName> = chain
        .iter()
        .map(|&m| member_face(union, m, fname(m, family(doc, m, fam))))
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
fn assert_chain_rows(
    doc: &ProfileDoc,
    t: &NameTable,
    union: RecipeNodeId,
    chain: &[RecipeNodeId],
    label: &str,
) {
    let merged = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .count();
    assert_eq!(merged, 4, "{label}: four merged rows, one per family");
    for fam in 0..4 {
        let row = chain_merged(doc, union, chain, fam);
        assert!(
            matches!(t.lookup(&row), Some(Entry::Unique(_))),
            "{label}: the chain's family {fam} row {row:?} is not published"
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
        let (docx, union, _) = declared_union(doc.clone(), &order, chain_pairs(&doc, &chain));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(body_of(&ev, union));
        assert!(
            (v - 2.2).abs() < 1e-9,
            "{label}: one fused body, got volume {v}"
        );
        assert_chain_rows(&docx, table(&ev, union), union, &chain, &label);
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
        let (docx, union, _) = declared_union(doc.clone(), &order, chain_pairs(&doc, &chain));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(body_of(&ev, union));
        assert!(
            (v - 2.9).abs() < 1e-9,
            "{label}: one fused body, got volume {v}"
        );
        assert_chain_rows(&docx, table(&ev, union), union, &chain, &label);
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
        let (docx, union, _) = declared_union(doc.clone(), &order, chain_pairs(&doc, &chain));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(body_of(&ev, union));
        assert!(
            (v - 3.2).abs() < 1e-9,
            "{label}: the chain plus the far block, got volume {v}"
        );
        assert_chain_rows(&docx, table(&ev, union), union, &chain, &label);
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
        path: vec![RoleSeg::FromA(inner.into())],
    }
}

fn from_b(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::FromB(inner.into())],
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
        (0..4)
            .map(|fam| {
                (
                    SitedRef::new(a, fname(a, family(&rec.doc, a, fam))),
                    SitedRef::new(b, fname(b, family(&rec.doc, b, fam))),
                )
            })
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
    let at = rec.doc.clone();
    let inner_row = |fam: usize| {
        merged(
            inner,
            vec![
                from_a(inner, fname(a, family(&at, a, fam))),
                from_b(inner, fname(b, family(&at, b, fam))),
            ],
        )
    };
    let decl_ic = rec.insert(Node::declare_rest(
        (0..4)
            .map(|fam| {
                (
                    SitedRef::new(inner, inner_row(fam)),
                    SitedRef::new(c, fname(c, family(&rec.doc, c, fam))),
                )
            })
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
    for fam in 0..4 {
        let row = merged(
            outer,
            vec![
                from_a(outer, from_a(inner, fname(a, family(&rec.doc, a, fam)))),
                from_a(outer, from_b(inner, fname(b, family(&rec.doc, b, fam)))),
                from_b(outer, fname(c, family(&rec.doc, c, fam))),
            ],
        );
        assert!(
            matches!(t.lookup(&row), Some(Entry::Unique(_))),
            "the outer merge of family {fam} is not the flat row {row:?}"
        );
        // And the inner's merged row is retired into it: not a row of
        // the outer, wrapped or not.
        assert!(
            t.lookup(&from_a(outer, inner_row(fam))).is_none(),
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

// `an_absorbed_fold_row_keeps_the_vanished_rung_and_does_not_look_through`
// retired with the class it measured: a declared entity is sited at a
// node that precedes the union, so a row the FOLD minted has no site
// and cannot be declared at all. `docm7_union_declare`'s
// `a_declared_pair_side_that_is_a_bare_name_does_not_load` is what
// stands in its place.

/// **A member face another member contains whole satisfies its declared
/// pair, and a declaration the geometry contradicts refuses in every
/// order.** `a`'s x = 1 wall lies inside `big` and is gone after
/// `big`'s step, in no table and in no merged row's set, with no
/// fragment descending from it.
///
/// Against `touch`, whose x = 1 wall rests on it inside `big`, the
/// pair is a real contact, certified pairwise, and the fold has nothing
/// left to back it with: satisfied, and the union fuses.
///
/// Against `far`, whose x = 6 wall is nowhere near, the declaration
/// claims a contact the geometry contradicts. The pair is judged before
/// the fold although the two boxes are disjoint, because it carries a
/// declaration, so it refuses as the pair boolean does in every order,
/// including the ones where `big` has consumed `a`'s wall first.
#[test]
fn a_member_face_contained_whole_satisfies_its_pair_and_a_contradicted_one_refuses() {
    let doc = ProfileDoc::empty_derived("docm8_vanished", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, big) = block(doc, (0.5, 3.0), (-1.0, 2.0), -1.0, 3.0);
    let (doc, touch) = block(doc, (1.0, 1.2), (0.2, 0.8), 0.2, 0.4);
    let (doc, far) = block(doc, (6.0, 7.0), (0.0, 1.0), 0.0, 1.0);
    let against = |m: RecipeNodeId| {
        vec![(
            SitedRef::new(a, fname(a, wall(&doc, a, 1))),
            SitedRef::new(m, fname(m, wall(&doc, m, 3))),
        )]
    };
    let (docx, union, _) = declared_union(doc.clone(), &[a, big, touch], against(touch));
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    for order in [
        [a, big, far],
        [a, far, big],
        [big, a, far],
        [big, far, a],
        [far, a, big],
        [far, big, a],
    ] {
        let (docx, union, _) = declared_union(doc.clone(), &order, against(far));
        let ev = run(&docx);
        assert!(
            matches!(
                failure(&ev, union),
                Some(NodeErrorKind::Boolean(
                    topo::BooleanError::ContactContradicted { .. }
                ))
            ),
            "{order:?}: expected the contradiction, got {:?}",
            failure(&ev, union)
        );
    }
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
        (0..4)
            .map(|fam| {
                (
                    SitedRef::new(a, fname(a, family(&rec.doc, a, fam))),
                    SitedRef::new(b, fname(b, family(&rec.doc, b, fam))),
                )
            })
            .collect(),
    ));
    let inner = rec.insert(Node::Boolean {
        op: BooleanOp::Union,
        a,
        b,
        declare: Some(decl_ab),
    });
    let c = recorded_block(&mut rec, (1.2, 2.2));
    let at = rec.doc.clone();
    let inner_row = |fam: usize| {
        merged(
            inner,
            vec![
                from_a(inner, fname(a, family(&at, a, fam))),
                from_b(inner, fname(b, family(&at, b, fam))),
            ],
        )
    };
    let decl_ic = rec.insert(Node::declare_rest(
        (0..4)
            .map(|fam| {
                (
                    SitedRef::new(inner, inner_row(fam)),
                    SitedRef::new(c, fname(c, family(&rec.doc, c, fam))),
                )
            })
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
    for fam in 0..4 {
        let outer_row = merged(
            outer,
            vec![
                from_a(outer, from_a(inner, fname(a, family(&rec.doc, a, fam)))),
                from_a(outer, from_b(inner, fname(b, family(&rec.doc, b, fam)))),
                from_b(outer, fname(c, family(&rec.doc, c, fam))),
            ],
        );
        // The consumed operand face: the inner merge, wrapped once.
        let consumed = from_a(outer, inner_row(fam));
        match resolve(ctx, &consumed) {
            Resolution::Failed(f) => assert!(
                f.offers.contains(&outer_row),
                "family {fam}: the consumed inner merged face does not offer the outer row: {:?}",
                f.offers
            ),
            other => panic!("family {fam}: the consumed face resolved: {other:?}"),
        }
        // And a flat constituent — a name that was never a row of any
        // table — offers the same row.
        let constituent = from_a(outer, from_a(inner, fname(a, family(&rec.doc, a, fam))));
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
    let node = Node::declare_rest(
        (0..4)
            .map(|fam| {
                (
                    SitedRef::new(a, fname(a, family(&doc, a, fam))),
                    SitedRef::new(b, fname(b, family(&doc, b, fam))),
                )
            })
            .collect(),
    );
    let (doc, decl_ab) = insert(doc, node);
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
    let carried = |fam: usize| {
        from_b(
            mid,
            merged(
                inner,
                vec![
                    from_a(inner, fname(a, family(&doc, a, fam))),
                    from_b(inner, fname(b, family(&doc, b, fam))),
                ],
            ),
        )
    };
    let node1 = Node::declare_rest(
        (0..4)
            .map(|fam| {
                (
                    SitedRef::new(mid, carried(fam)),
                    SitedRef::new(c, fname(c, family(&doc, c, fam))),
                )
            })
            .collect(),
    );
    let (doc, decl_mc) = insert(doc, node1);
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
    for fam in 0..4 {
        let want = merged(
            outer,
            vec![
                from_a(
                    outer,
                    from_b(mid, from_a(inner, fname(a, family(&doc, a, fam)))),
                ),
                from_a(
                    outer,
                    from_b(mid, from_b(inner, fname(b, family(&doc, b, fam)))),
                ),
                from_b(outer, fname(c, family(&doc, c, fam))),
            ],
        );
        assert!(
            matches!(t.lookup(&want), Some(Entry::Unique(_))),
            "the FromB pass-through did not mint flat for family {fam}: {want:?}"
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
    let pairs = {
        let mut v = flush_pairs(&doc, (a, a), (c, c));
        for k in [0, 2] {
            v.push((
                SitedRef::new(a, fname(a, wall(&doc, a, k))),
                SitedRef::new(s, fname(s, wall(&doc, s, k))),
            ));
        }
        v
    };
    #[derive(Debug, PartialEq)]
    enum Outcome {
        Fused,
        VanishedEndCapOfA,
        SeamVertexNoRule,
    }
    let a_end = |u: RecipeNodeId| member_face(u, a, fname(a, RoleSeg::Cap(CapEnd::End)));
    for (order, want) in [
        (vec![a, c, s], Outcome::Fused),
        (vec![c, a, s], Outcome::Fused),
        (vec![a, s, c], Outcome::VanishedEndCapOfA),
        (vec![s, a, c], Outcome::VanishedEndCapOfA),
        (vec![c, s, a], Outcome::SeamVertexNoRule),
        (vec![s, c, a], Outcome::SeamVertexNoRule),
    ] {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs.clone());
        let ev = run(&docx);
        let got = match failure(&ev, union) {
            None => Outcome::Fused,
            Some(NodeErrorKind::DeclareResolve { error }) if matches!(&**error, ResolveError::Vanished { name, .. } if *name == a_end(union)) => {
                Outcome::VanishedEndCapOfA
            }
            // NOT an `Emission`: this document is well formed, so the
            // refusal is the emitter saying it has no rule for the
            // construction — `tests/wire_legal_union_refusals.rs`
            // carries the argument.
            Some(NodeErrorKind::Naming(NamingError::SeamVertexParentage { .. })) => {
                Outcome::SeamVertexNoRule
            }
            other => panic!("{order:?}: unexpected outcome {other:?}"),
        };
        assert_eq!(got, want, "{order:?}");
        if got == Outcome::Fused {
            let v = volume(body_of(&ev, union));
            assert!((v - 1.6).abs() < 1e-9, "{order:?}: volume {v}");
        }
    }
}

/// **A union's undeclared contact against a face the fold FRAGMENTS is
/// refused between the two members, and is declarable.**
///
/// `s` pokes through `a`'s end cap, so folding it in FRAGMENTS that
/// cap: the accumulation's rows for it are `[FromMember(a), Fragment]`
/// pairs, which the member-keying rule does not collapse to a member's
/// entity and no merge retired. `d` rests flush on `a`'s cap,
/// undeclared. Contact is judged pairwise before the fold (DM4), so the
/// refusal names `a`'s cap and `d`'s bottom, both member faces, rather
/// than the fragment the fold would have met: no refusal names a row
/// the fold minted, and `UndeclarableContact` is not reached.
///
/// Declared, the pair is fed to `d`'s step, where `a`'s cap survives
/// only in pieces; which of them carry the contact is
/// `member-space-look-through-stops-at-splits-containment-and-fragmented-merges`'s
/// question, and the declaration refuses there as a vanished name.
#[test]
fn a_contact_against_a_fold_minted_fragment_is_refused_between_members() {
    let doc = ProfileDoc::empty_derived("docm8_fragment_refusal", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block(doc, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let (doc, d) = block(doc, (0.5, 0.9), (0.2, 0.8), 1.0, 0.4);
    // `a` and `s` share both y-walls; declare them so the fold reaches
    // the step that matters.
    let mut pairs = Vec::new();
    for k in [0, 2] {
        pairs.push((
            SitedRef::new(a, fname(a, wall(&doc, a, k))),
            SitedRef::new(s, fname(s, wall(&doc, s, k))),
        ));
    }
    let (docx, union, _) = declared_union(doc.clone(), &[a, s, d], pairs.clone());
    let ev = run(&docx);
    let what = failure(&ev, union);
    let Some(NodeErrorKind::UndeclaredContact {
        finding, merged, ..
    }) = what
    else {
        panic!("expected the pairwise refusal, got {what:?}")
    };
    assert_eq!(
        finding.pair,
        (
            SitedRef::new(a, fname(a, RoleSeg::Cap(CapEnd::End))),
            SitedRef::new(d, fname(d, RoleSeg::Cap(CapEnd::Start))),
        )
    );
    assert!(merged.0.is_empty() && merged.1.is_empty(), "{merged:?}");
    pairs.push(finding.pair.clone());
    let (docx, union, _) = declared_union(doc, &[a, s, d], pairs);
    let ev = run(&docx);
    assert!(
        matches!(
            failure(&ev, union),
            Some(NodeErrorKind::DeclareResolve { .. })
        ),
        "expected the vanished name at d's step, got {:?}",
        failure(&ev, union)
    );
}
