//! **A merged face's name is a flat constituent set, and a
//! member-space declaration resolves through the fold's merges**
//! (DOCM-8; DM4's "merges and order" sentence): the flat mint at the
//! pair emitter, the look-through at the union's routing step, and
//! the typed refusal every other consumption of a member face meets
//! there.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, member_face, run};
use crate::fixture;
use crate::fixture::{Recorder, flush_segs, fname, insert, len, table, wall};

use editor_core::{
    BooleanOp, CapEnd, Diagnosis, EntityKind, Entry, Evaluation, FoldConsumption, NameTable,
    NamingError, Node, NodeErrorKind, ProfileDoc, RecipeNodeId, Resolution, ResolveError, RoleSeg,
    RunCtx, SitedRef, StableName, resolve,
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
fn chain_pairs(chain: &[RecipeNodeId]) -> Vec<(SitedRef, SitedRef)> {
    chain
        .windows(2)
        .flat_map(|w| flush_pairs((w[0], w[0]), (w[1], w[1])))
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
        let (docx, union, _) = declared_union(doc.clone(), &order, chain_pairs(&chain));
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
        let (docx, union, _) = declared_union(doc.clone(), &order, chain_pairs(&chain));
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
        let (docx, union, _) = declared_union(doc.clone(), &order, chain_pairs(&chain));
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
        flush_segs()
            .into_iter()
            .map(|seg| {
                (
                    SitedRef::new(a, fname(a, seg.clone())),
                    SitedRef::new(b, fname(b, seg)),
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
            .map(|seg| {
                (
                    SitedRef::new(inner, inner_row(seg.clone())),
                    SitedRef::new(c, fname(c, seg)),
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
            SitedRef::new(a, fname(a, wall(1))),
            SitedRef::new(m, fname(m, wall(3))),
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
        flush_segs()
            .into_iter()
            .map(|seg| {
                (
                    SitedRef::new(a, fname(a, seg.clone())),
                    SitedRef::new(b, fname(b, seg)),
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
            .map(|seg| {
                (
                    SitedRef::new(inner, inner_row(seg.clone())),
                    SitedRef::new(c, fname(c, seg)),
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
                .map(|seg| {
                    (
                        SitedRef::new(a, fname(a, seg.clone())),
                        SitedRef::new(b, fname(b, seg)),
                    )
                })
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
                .map(|seg| {
                    (
                        SitedRef::new(mid, carried(seg.clone())),
                        SitedRef::new(c, fname(c, seg)),
                    )
                })
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
// Past the merges: every other consumption refuses, naming itself.
// Measured, per order.
// ---------------------------------------------------------------------

/// What one member order of a declared union came to, in the terms the
/// tables below pin.
#[derive(Debug, Clone, PartialEq)]
enum Outcome {
    /// One body.
    Fused,
    /// The declaration channel refused a member face the fold consumed
    /// before its pair's step: which face, and by what.
    Consumed(StableName, FoldConsumption),
    /// The emitter has no naming rule for the construction the order
    /// reached (`tests/wire_legal_union_refusals.rs`).
    SeamVertexNoRule,
}

fn outcome(ev: &Evaluation<f64>, union: RecipeNodeId) -> Outcome {
    match failure(ev, union) {
        None => Outcome::Fused,
        Some(NodeErrorKind::DeclareResolve { error }) => match &**error {
            ResolveError::Vanished {
                name,
                diagnosis: Diagnosis::ConsumedByFold { union: at, by },
                last_good: None,
            } if *at == union => Outcome::Consumed(name.clone(), *by),
            other => panic!("a declare refusal that names no composition: {other:?}"),
        },
        Some(NodeErrorKind::Naming(NamingError::SeamVertexParentage { .. })) => {
            Outcome::SeamVertexNoRule
        }
        Some(other) => panic!("unexpected outcome {other:?}"),
    }
}

/// R1's split fixture: `a` and `c` meet flush along x; `s` sits on
/// `a`'s top cap across its whole depth, declared against `a`'s two
/// y-walls, so folding `s` in fragments that cap.
fn split_fixture(doc: ProfileDoc) -> (ProfileDoc, [RecipeNodeId; 3], Vec<(SitedRef, SitedRef)>) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block(doc, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let mut pairs = flush_pairs((a, a), (c, c));
    for seg in [wall(0), wall(2)] {
        pairs.push((
            SitedRef::new(a, fname(a, seg.clone())),
            SitedRef::new(s, fname(s, seg)),
        ));
    }
    (doc, [a, c, s], pairs)
}

/// **A declared member face SPLIT by a later member refuses as a
/// split, with nothing offered.** The orders that fold `s` last fuse;
/// the orders that fold it in before `c` meet `a`'s end cap as
/// fragments only, and refuse naming the split — which fragment the
/// pair meant is a geometric question the routing step does not ask
/// (DM4). The orders that fold `s` before `a` never reach the
/// declaration channel: the emitter has no rule for the seam vertex
/// they build.
#[test]
fn a_member_face_split_by_a_later_member_refuses_as_a_split() {
    let doc = ProfileDoc::empty_derived("docm8_split_order", Tol::witness());
    let (doc, [a, c, s], pairs) = split_fixture(doc);
    let a_end = |u| member_face(u, a, fname(a, RoleSeg::Cap(CapEnd::End)));
    enum Want {
        Fused,
        Split,
        SeamVertexNoRule,
    }
    for (order, want) in [
        (vec![a, c, s], Want::Fused),
        (vec![c, a, s], Want::Fused),
        (vec![a, s, c], Want::Split),
        (vec![s, a, c], Want::Split),
        (vec![c, s, a], Want::SeamVertexNoRule),
        (vec![s, c, a], Want::SeamVertexNoRule),
    ] {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs.clone());
        let ev = run(&docx);
        let want = match want {
            Want::Fused => Outcome::Fused,
            Want::Split => Outcome::Consumed(a_end(union), FoldConsumption::Split),
            Want::SeamVertexNoRule => Outcome::SeamVertexNoRule,
        };
        let got = outcome(&ev, union);
        assert_eq!(got, want, "{order:?}");
        if got == Outcome::Fused {
            let v = volume(body_of(&ev, union));
            assert!((v - 1.6).abs() < 1e-9, "{order:?}: volume {v}");
        }
    }
}

/// **What a member order does to `capped`'s end cap**, when two cap
/// partners meet it flush along x (declared on all four families) and
/// `cutter` rises through that cap across its whole depth — the shape
/// both four-member rows below share.
///
/// `capped` joining an accumulation that already holds `cutter` and a
/// partner builds the seam vertex no rule names. Otherwise the pair
/// naming the cap against the LATER partner is what decides: fed
/// before `cutter` joins, everything fuses; fed after it, the cap is
/// fragments of a merged row when a partner joined before `cutter`,
/// and fragments of its own when none did.
fn cap_outcome(
    order: &[RecipeNodeId],
    capped: RecipeNodeId,
    partners: [RecipeNodeId; 2],
    cutter: RecipeNodeId,
    cap: StableName,
) -> Outcome {
    let pos = |m| order.iter().position(|x| *x == m).unwrap();
    let first_partner = pos(partners[0]).min(pos(partners[1]));
    let last_partner = pos(partners[0]).max(pos(partners[1]));
    if pos(cutter) < pos(capped) && first_partner < pos(capped) {
        Outcome::SeamVertexNoRule
    } else if pos(cutter) > last_partner {
        Outcome::Fused
    } else if first_partner < pos(cutter) {
        Outcome::Consumed(cap, FoldConsumption::FragmentedMerge)
    } else {
        Outcome::Consumed(cap, FoldConsumption::Split)
    }
}

/// Every order of `members` against [`cap_outcome`], and the tally of
/// the four outcomes: fused, the missing seam-vertex rule, split, and
/// fragmented merge.
fn every_cap_order(
    doc: &ProfileDoc,
    members: [RecipeNodeId; 4],
    pairs: &[(SitedRef, SitedRef)],
    capped: RecipeNodeId,
    partners: [RecipeNodeId; 2],
    cutter: RecipeNodeId,
) -> [usize; 4] {
    let mut tally = [0; 4];
    for order in permutations(&members) {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs.to_vec());
        let ev = run(&docx);
        let cap = member_face(union, capped, fname(capped, RoleSeg::Cap(CapEnd::End)));
        let got = outcome(&ev, union);
        assert_eq!(
            got,
            cap_outcome(&order, capped, partners, cutter, cap),
            "{order:?}"
        );
        tally[match got {
            Outcome::Fused => 0,
            Outcome::SeamVertexNoRule => 1,
            Outcome::Consumed(_, FoldConsumption::Split) => 2,
            Outcome::Consumed(_, FoldConsumption::FragmentedMerge) => 3,
        }] += 1;
    }
    tally
}

/// **A member face inside a declared merge that a later member split
/// refuses as a fragmented merge**, and one split BEFORE any merge
/// reached it refuses as a split — the same face, consumed by a
/// different composition in each order, and the refusal says which.
///
/// The split fixture plus `d`, flush with `a` along x on the far side
/// from `c` and clear of `s`. Folding one of `a`'s cap partners in
/// before `s` merges the cap and `s` then fragments the merged row
/// (`[Merged(set), Fragment(q)]`); folding `s` in first fragments
/// `a`'s own cap. Either way the pair naming `a`'s cap against the
/// partner that joins after `s` has no one entity to resolve to.
#[test]
fn a_member_face_inside_a_merge_a_later_member_split_refuses_as_a_fragmented_merge() {
    let doc = ProfileDoc::empty_derived("docm8_fragmented_merge", Tol::witness());
    let (doc, [a, c, s], mut pairs) = split_fixture(doc);
    let (doc, d) = block(doc, (-0.5, 0.1), (0.0, 1.0), 0.0, 1.0);
    pairs.extend(flush_pairs((a, a), (d, d)));
    assert_eq!(
        every_cap_order(&doc, [a, c, s, d], &pairs, a, [c, d], s),
        [6, 10, 4, 4]
    );
}

/// **R1's three-neighbour star, per order**: `c` sits between `w` and
/// `e` along x, each flush with it on all four families, and `t` rises
/// through `c`'s top cap across its depth, declared against `c`'s
/// y-walls. The same law as the row above, on a different fixture:
/// every refusal the declaration channel raises is on `c`'s end cap and
/// names the composition that consumed it.
#[test]
fn the_three_neighbour_star_refuses_as_a_split_or_a_fragmented_merge() {
    let doc = ProfileDoc::empty_derived("docm8_star", Tol::witness());
    let (doc, c) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, w) = block(doc, (-0.5, 0.3), (0.0, 1.0), 0.0, 1.0);
    let (doc, e) = block(doc, (0.7, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, t) = block(doc, (0.4, 0.6), (0.0, 1.0), 0.5, 1.0);
    let mut pairs = flush_pairs((c, c), (w, w));
    pairs.extend(flush_pairs((c, c), (e, e)));
    for seg in [wall(0), wall(2)] {
        pairs.push((
            SitedRef::new(c, fname(c, seg.clone())),
            SitedRef::new(t, fname(t, seg)),
        ));
    }
    assert_eq!(
        every_cap_order(&doc, [c, w, e, t], &pairs, c, [w, e], t),
        [6, 10, 4, 4]
    );
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
    for seg in [wall(0), wall(2)] {
        pairs.push((
            SitedRef::new(a, fname(a, seg.clone())),
            SitedRef::new(s, fname(s, seg)),
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
