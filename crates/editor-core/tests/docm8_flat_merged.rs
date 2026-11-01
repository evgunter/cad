//! **A merged face's name is a flat constituent set** (DOCM-8): the
//! flat mint at the pair emitter, over a chain folded in every member
//! order and over a boolean of a boolean.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::corpus;
use crate::corpus::body_of;
use crate::emit_shared_rim_several::{construction, own_name};
use crate::fixture;
use crate::fixture::{Recorder, flush_segs, insert, len, member_face, table};

use editor_core::{
    EntityKind, Entry, NameTable, Node, ProfileDoc, RecipeNodeId, Resolution, RoleSeg, RunCtx,
    StableName,
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

/// Flush family `fam` of the extrude `ext` — [`flush_segs`]'s `fam`th,
/// spelled by `ext`'s own pieces.
fn family(doc: &ProfileDoc, ext: RecipeNodeId, fam: usize) -> RoleSeg {
    flush_segs(doc, ext)[fam].clone()
}

/// Flush family `fam`'s face of operand `m`, as `m` names it: its
/// construction's face, under `m`'s placement where it is one.
fn own_face(doc: &ProfileDoc, m: RecipeNodeId, fam: usize) -> StableName {
    own_name(
        doc,
        m,
        EntityKind::Face,
        vec![family(doc, construction(doc, m), fam)],
    )
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
        .map(|&m| member_face(union, fixture::out(doc, m), own_face(doc, m, fam)))
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
    let merged = |kind| {
        t.iter()
            .filter(|(n, _)| n.kind == kind && matches!(n.path.first(), Some(RoleSeg::Merged(_))))
            .count()
    };
    assert_eq!(
        merged(EntityKind::Face),
        4,
        "{label}: four merged faces, one per family"
    );
    assert_eq!(
        merged(EntityKind::Edge),
        4,
        "{label}: four joined edges, one between each two adjacent families"
    );
    for fam in 0..4 {
        let row = chain_merged(doc, union, chain, fam);
        assert!(
            matches!(t.lookup(&row), Some(Entry::Unique(_))),
            "{label}: the chain's family {fam} row {row:?} is not published"
        );
    }
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "mass",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), Tol::witness()),
        Tol::witness(),
    )
    .volume
}

// ---------------------------------------------------------------------
// A1 — the chain fuses in every member order.
// ---------------------------------------------------------------------

/// **Three chained blocks fuse in every member order**, and the fused
/// cap's row is the same flat set in each.
///
/// `a` meets `c` and `c` meets `d`. The orders that fold `c` in after
/// one of its neighbours find `c`'s face already inside a `Merged`
/// row; the orders that fold it in last find both faces as rows.
/// Either way the merge lists the three member faces.
#[test]
fn a_three_member_chain_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("docm8_chain3", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block_on(doc, a, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let chain = [a, c, d];
    for order in permutations(&chain) {
        let label = format!("order {order:?}");
        let (docx, union) = crate::fixture::union_over(doc.clone(), &order);
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(&(body_of(&ev, union)));
        assert!(
            (v - 2.2).abs() < 1e-9,
            "{label}: one fused body, got volume {v}"
        );
        assert_chain_rows(&docx, table(&ev, union), union, &chain, &label);
    }
}

/// **A four-member chain (two middles) fuses in every order**: a merge
/// of a merged row is itself flat and is what the next merge lists.
#[test]
fn a_four_member_chain_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("docm8_chain4", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block_on(doc, a, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, e) = block_on(doc, a, (1.9, 2.9), (0.0, 1.0), 0.0, 1.0);
    let chain = [a, c, d, e];
    for order in permutations(&chain) {
        let label = format!("order {order:?}");
        let (docx, union) = crate::fixture::union_over(doc.clone(), &order);
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(&(body_of(&ev, union)));
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
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block_on(doc, a, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, far) = block_on(doc, a, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let chain = [a, c, d];
    for order in permutations(&[a, c, d, far]) {
        let label = format!("order {order:?}");
        let (docx, union) = crate::fixture::union_over(doc.clone(), &order);
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{label}: the chain refused: {:?}",
            failure(&ev, union)
        );
        let v = volume(&(body_of(&ev, union)));
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

/// A block on its own xy, recorded — placed against `root` where one
/// is given: the operand.
fn recorded_block(
    rec: &mut Recorder,
    root: Option<RecipeNodeId>,
    (x0, x1): (f64, f64),
) -> RecipeNodeId {
    let p = rec.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, 0.0), (x1, 0.0), (x1, 1.0), (x0, 1.0)]],
    );
    let block = rec.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    match root {
        Some(root) => rec.against(root, block, &editor_core::Frame::IDENTITY),
        None => block,
    }
}

/// The read operand `i` of the boolean `node` holds in `doc` — member
/// `i` of a union, or a subtract's `from` (0) / `tool` (1).
fn operand_read(doc: &ProfileDoc, node: RecipeNodeId, i: usize) -> editor_core::VarId {
    match doc.node(node) {
        Some(Node::Union { members, .. } | Node::Intersect { members, .. }) => {
            members
                .reads()
                .nth(i)
                .unwrap_or_else(|| panic!("{node} has no member {i}"))
                .read
        }
        Some(Node::Subtract { from, tool, .. }) => [from.read, tool.read][i],
        other => panic!("{node} is not a boolean: {other:?}"),
    }
}

/// `inner`, carried into `node` through its operand `i`'s read.
fn from_operand(doc: &ProfileDoc, node: RecipeNodeId, i: usize, inner: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::From {
            read: operand_read(doc, node, i),
            of: inner.into(),
        }],
    }
}

fn from_a(doc: &ProfileDoc, node: RecipeNodeId, inner: StableName) -> StableName {
    from_operand(doc, node, 0, inner)
}

fn from_b(doc: &ProfileDoc, node: RecipeNodeId, inner: StableName) -> StableName {
    from_operand(doc, node, 1, inner)
}

fn merged(node: RecipeNodeId, mut set: Vec<StableName>) -> StableName {
    set.sort();
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Merged(set)],
    }
}

/// **A boolean over a boolean that merged mints the outer merged face
/// FLAT**: the inner merge's constituents, each wrapped
/// `FromA`, plus the partner wrapped `FromB` — no `Merged` inside —
/// and the edit-log replay of the document is bit-identical.
#[test]
fn a_boolean_over_a_boolean_mints_a_flat_merged_row_and_replays() {
    let tol = Tol::witness();
    let mut rec = Recorder::new();
    let a = recorded_block(&mut rec, None, (0.0, 1.0));
    let b = recorded_block(&mut rec, Some(a), (0.5, 1.5));
    let inner = rec.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
    });
    let c = recorded_block(&mut rec, Some(a), (1.2, 2.2));
    // The inner's merged rows, as the inner's table names them.
    let at = rec.doc.clone();
    let inner_row = |fam: usize| {
        merged(
            inner,
            vec![
                from_a(&at, inner, own_face(&at, a, fam)),
                from_b(&at, inner, own_face(&at, b, fam)),
            ],
        )
    };
    let outer = rec.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![inner.into(), c.into()]),
    });
    let ev = run(&rec.doc);
    assert!(failure(&ev, outer).is_none(), "{:?}", failure(&ev, outer));
    let t = table(&ev, outer);
    for fam in 0..4 {
        let row = merged(
            outer,
            vec![
                from_a(
                    &rec.doc,
                    outer,
                    from_a(&rec.doc, inner, own_face(&rec.doc, a, fam)),
                ),
                from_a(
                    &rec.doc,
                    outer,
                    from_b(&rec.doc, inner, own_face(&rec.doc, b, fam)),
                ),
                from_b(&rec.doc, outer, own_face(&rec.doc, c, fam)),
            ],
        );
        assert!(
            matches!(t.lookup(&row), Some(Entry::Unique(_))),
            "the outer merge of family {fam} is not the flat row {row:?}"
        );
        // And the inner's merged row is retired into it: not a row of
        // the outer, wrapped or not.
        assert!(
            t.lookup(&from_a(&rec.doc, outer, inner_row(fam))).is_none(),
            "the inner merged row survived as an outer row"
        );
    }
    let merged = |kind| {
        t.iter()
            .filter(|(n, _)| n.kind == kind && matches!(n.path.first(), Some(RoleSeg::Merged(_))))
            .count()
    };
    assert_eq!(merged(EntityKind::Face), 4);
    assert_eq!(
        merged(EntityKind::Edge),
        4,
        "the four long edges are joined"
    );
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

/// **A member face another member contains whole is gone, and the
/// union fuses.** `a`'s x = 1 wall lies inside `big`, and `touch`'s
/// x = 1 wall rests on it inside `big`.
#[test]
fn a_member_face_contained_whole_by_another_fuses() {
    let doc = ProfileDoc::empty_derived("docm8_vanished", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, big) = block_on(doc, a, (0.5, 3.0), (-1.0, 2.0), -1.0, 3.0);
    let (doc, touch) = block_on(doc, a, (1.0, 1.2), (0.2, 0.8), 0.2, 0.4);
    let (docx, union) = crate::fixture::union_over(doc, &[a, big, touch]);
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
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
    let a = recorded_block(&mut rec, None, (0.0, 1.0));
    let b = recorded_block(&mut rec, Some(a), (0.5, 1.5));
    let inner = rec.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
    });
    let c = recorded_block(&mut rec, Some(a), (1.2, 2.2));
    let at = rec.doc.clone();
    let inner_row = |fam: usize| {
        merged(
            inner,
            vec![
                from_a(&at, inner, own_face(&at, a, fam)),
                from_b(&at, inner, own_face(&at, b, fam)),
            ],
        )
    };
    let outer = rec.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![inner.into(), c.into()]),
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
                from_a(
                    &rec.doc,
                    outer,
                    from_a(&rec.doc, inner, own_face(&rec.doc, a, fam)),
                ),
                from_a(
                    &rec.doc,
                    outer,
                    from_b(&rec.doc, inner, own_face(&rec.doc, b, fam)),
                ),
                from_b(&rec.doc, outer, own_face(&rec.doc, c, fam)),
            ],
        );
        // The consumed operand face: the inner merge, wrapped once.
        let consumed = from_a(&rec.doc, outer, inner_row(fam));
        match crate::fixture::resolve_at_mint(ctx, &consumed) {
            Resolution::Failed(f) => assert!(
                f.offers.contains(&outer_row),
                "family {fam}: the consumed inner merged face does not offer the outer row: {:?}",
                f.offers
            ),
            other => panic!("family {fam}: the consumed face resolved: {other:?}"),
        }
        // And a flat constituent — a name that was never a row of any
        // table — offers the same row.
        let constituent = from_a(
            &rec.doc,
            outer,
            from_a(&rec.doc, inner, own_face(&rec.doc, a, fam)),
        );
        match crate::fixture::resolve_at_mint(ctx, &constituent) {
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
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, inner) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    // A far block, unioned with the merge as operand B, so the merged
    // rows ride through as `mid:FromB(inner:Merged(..))`.
    let (doc, far) = block_on(doc, a, (8.0, 9.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, mid) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![far.into(), inner.into()]),
        },
    );
    let (doc, c) = block_on(doc, a, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, outer) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![mid.into(), c.into()]),
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
                    &doc,
                    outer,
                    from_b(&doc, mid, from_a(&doc, inner, own_face(&doc, a, fam))),
                ),
                from_a(
                    &doc,
                    outer,
                    from_b(&doc, mid, from_b(&doc, inner, own_face(&doc, b, fam))),
                ),
                from_b(&doc, outer, own_face(&doc, c, fam)),
            ],
        );
        assert!(
            matches!(t.lookup(&want), Some(Entry::Unique(_))),
            "the FromB pass-through did not mint flat for family {fam}: {want:?}"
        );
    }
}

/// R1's split fixture: `a` and `c` meet flush along x; `s` sits on
/// `a`'s top cap across its whole depth, flush with `a`'s two y-walls,
/// so folding `s` in fragments that cap.
pub(crate) fn split_fixture(doc: ProfileDoc) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block_on(doc, a, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    (doc, [a, c, s])
}

/// **A union's contact against a face the fold FRAGMENTS glues.**
///
/// `s` pokes through `a`'s end cap, so folding it in FRAGMENTS that
/// cap: the accumulation's rows for it are `[FromMember(a), Fragment]`
/// pairs, which the member-keying rule does not collapse to a member's
/// entity and no merge retired. `d` rests flush on `a`'s cap. The
/// margins decide that rest one carrier, so the fold glues it and the
/// union is one body.
#[test]
fn a_contact_against_a_fold_minted_fragment_glues() {
    let doc = ProfileDoc::empty_derived("docm8_fragment_refusal", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, s) = block_on(doc, a, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let (doc, d) = block_on(doc, a, (0.5, 0.9), (0.2, 0.8), 1.0, 0.4);
    let (docx, union) = crate::fixture::union_over(doc, &[a, s, d]);
    let ev = run(&docx);
    assert!(
        failure(&ev, union).is_none(),
        "the rest on the fragmented cap glues: {:?}",
        failure(&ev, union)
    );
}
