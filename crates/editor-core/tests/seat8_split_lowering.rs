//! **The split moved onto the verb substrate and nothing observable
//! moved with it** — the SEAT-4/5/7 method, for the first verb whose
//! result is two-sided.
//!
//! `Node::Split` now builds a `verbs::Verb`, runs it through the split
//! door, takes its birth record out of the closed record channel and
//! its two sides out of the door's own out-type, stamps both sides in
//! one provenance index space, and emits names from the record. That
//! is a re-plumbing, and a re-plumbing's failure mode is a difference
//! nobody looks for — so:
//!
//! - **The wire format**: a registered document carrying a split AND
//!   two projections off it saves, loads and re-saves byte-identically.
//! - **The evaluation**: every split-carrying corpus document's bodies
//!   and name tables digest to committed constants, per document, so a
//!   red says WHICH document moved — and the digest feeds BOTH sides of
//!   the split's value, with an empty side as its own token.
//! - **The empty side is a channel that can red**: the corpus's three
//!   split documents all cut through their operand, so no registered
//!   evaluation ever takes the `Empty` arm; an in-suite document whose
//!   plane misses the body is pinned so that token is fed by a real
//!   value and not just written into the feed (the lesson the boolean's
//!   typed-empty and contacts channels taught, twice).
//! - **What the projection needs from the two-sided value** is pinned
//!   where the projection reads it: a half selected by ROLE, an empty
//!   half refused typed, a present half handed on as the split's own
//!   body.
//! - **The one-index-space stamping** across both halves is a guarded
//!   property here, not a comment in the lowering.
//!
//! What already covers this and what these rows add is the same
//! division SEAT-4 recorded: `m10_p_fence` digests every body point's
//! bits corpus-wide and `lib_g16_corpus_name_digests` digests every
//! name table, so either would catch a lowering that changed geometry
//! or names — and neither says which document did it, neither reaches
//! the provenance tables, and neither sees an `Empty` side at all
//! (there is no body to digest and no name to emit for one).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::corpus;
use crate::fixture;

use corpus::{Recorder, eval, failures};
use editor_core::{
    Datum, Node, NodeErrorKind, NodeResult, PartSelect, ProfileDoc, RecipeNodeId, SplitHalf,
    SplitSide, ValuePayload, persist,
};
use fixture::digest::digest;
use fixture::{len, scl, square};
use geom_core::Tol;
use topo::{Body, SourceExpr};

fn tol() -> Tol {
    Tol::witness()
}

/// The registered split documents, by name: a tilted plane through a
/// cylinder (curved section edges), a plane through a box with both
/// halves projected and unioned back (the DM3 payoff), and the kitchen
/// sink's mid-height cut of a declared union. Every one cuts through
/// its operand: two bodies out, no empty side anywhere — measured by
/// `the_corpus_has_no_split_with_an_empty_side`, not read off.
const SPLIT_DOCUMENTS: [&str; 3] = ["cut_cylinder", "part_select", "kitchen_sink"];

/// Every split value in an evaluation, as `(above is empty, below is
/// empty)`.
fn split_sides(ev: &editor_core::Evaluation<f64>) -> Vec<(bool, bool)> {
    ev.order
        .iter()
        .filter_map(|id| match ev.value(*id).map(|v| &v.payload) {
            Some(ValuePayload::Split { above, below }) => Some((
                matches!(above, SplitSide::Empty),
                matches!(below, SplitSide::Empty),
            )),
            _ => None,
        })
        .collect()
}

/// **The corpus's split documents are exactly [`SPLIT_DOCUMENTS`], and
/// none of their splits has an empty side** — a measurement, so that a
/// fourth registered split, or a plane that stops cutting through, is
/// noticed here rather than read off a comment. The empty-side row
/// below exists BECAUSE this count is zero: the day it is not, the
/// in-suite document is no longer the only input feeding that token
/// and the registered one should carry the pin.
#[test]
fn the_corpus_has_no_split_with_an_empty_side() {
    let mut with_split: Vec<&str> = Vec::new();
    let mut empty_sides = 0usize;
    for doc in corpus::documents() {
        let ev = eval::<f64>(&doc.doc);
        let sides = split_sides(&ev);
        if !sides.is_empty() {
            with_split.push(doc.name);
        }
        empty_sides += sides
            .iter()
            .filter(|(above, below)| *above || *below)
            .count();
    }
    with_split.sort_unstable();
    let mut expected = SPLIT_DOCUMENTS.to_vec();
    expected.sort_unstable();
    assert_eq!(
        with_split, expected,
        "the registry's split-carrying documents are not the ones this suite pins"
    );
    assert_eq!(
        empty_sides, 0,
        "a registered split now has an empty side; move the empty-side pin onto it"
    );
}

/// **The wire format is untouched**: `part_select` — a split with BOTH
/// halves projected off it by `Node::Part` and unioned — saves, loads
/// and re-saves byte-identically. The corpus-wide round-trip covers the
/// same bytes; this is the per-document form beside the digest rows,
/// so a red here names the split rather than the registry.
#[test]
fn a_split_document_with_projections_round_trips_byte_identical() {
    let doc = corpus::documents()
        .into_iter()
        .find(|d| d.name == "part_select")
        .expect("the document is registered");
    let snapshot = ProfileDoc::empty_derived("seat8_split_roundtrip", tol());
    let first = persist::save(&snapshot, &doc.edits, tol()).expect("the document saves");
    let loaded = persist::load(&first, tol()).expect("its own bytes load back");
    assert_eq!(loaded.edits, doc.edits, "the edit log did not survive");
    let second = persist::save(&loaded.snapshot, &loaded.edits, tol())
        .expect("the loaded document re-saves");
    assert_eq!(
        first, second,
        "a split document does not round-trip byte-identically"
    );
}

// The digest these rows pin with is `fixture::digest::digest` — ONE feed
// for every verb-migration suite, stated at that home; the `Split` arm
// (each side under its ROLE token, an empty side as its own token) is
// this unit's addition to it.

/// **The registered split documents' evaluations are bit-identical**
/// through the verb lowering — bodies on both sides, provenance stamps,
/// name tables — one committed number each.
///
/// The numbers were taken on this branch and re-taken on the extracted
/// merge base with this file and the shared feed copied onto it; all
/// three reproduce there. That differential is what "nothing observable moved" means;
/// without it the constants would only say the branch agrees with
/// itself. They are goldens in the ordinary sense — when one moves the
/// question is whether the new behaviour is right, never how to restore
/// the old number.
#[test]
fn the_split_documents_evaluate_to_their_committed_digests() {
    for (name, want) in [
        ("cut_cylinder", 0xeaea_81fa_b3df_29e3_u64),
        ("part_select", 0xd31a_b4c8_da48_2cd5),
        ("kitchen_sink", 0x8826_0b67_1ded_0c08),
    ] {
        assert!(SPLIT_DOCUMENTS.contains(&name));
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("the document is registered");
        let ev = eval::<f64>(&doc.doc);
        let bad = failures(&ev);
        assert!(bad.is_empty(), "{name} failed to evaluate: {bad:?}");
        let split = ev
            .order
            .iter()
            .copied()
            .find(|id| {
                matches!(
                    ev.value(*id).map(|v| &v.payload),
                    Some(ValuePayload::Split { .. })
                )
            })
            .expect("the document carries a split value");
        let Some(ValuePayload::Split { above, below }) = ev.value(split).map(|v| &v.payload) else {
            unreachable!("found above");
        };
        assert!(
            matches!((above, below), (SplitSide::Body(_), SplitSide::Body(_))),
            "{name}'s plane no longer cuts through its operand"
        );
        let got = digest(&ev);
        println!("seat8 {name}: {got:#018x}");
        assert_eq!(
            got, want,
            "{name}'s evaluation moved — a side, its stamps or the name table"
        );
    }
}

/// A unit cube on the xy frame, a plane datum at height `z` with `+z`
/// normal, and the split of the one by the other — returns the
/// recorder and the split node.
fn cube_split_at(z: f64) -> (Recorder, RecipeNodeId) {
    let mut r = Recorder::new();
    let profile = r.profile(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.5, 0.5, 0.5)],
    );
    let cube = r.insert(Node::Extrude {
        profile,
        distance: len(1.0),
    });
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(z)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    let split = r.insert(Node::Split { target: cube, tool });
    (r, split)
}

/// **The empty side is pinned by an input that PRODUCES it** — a plane
/// clear of the cube, authored in-suite because no registered document
/// has one and the empty path needs none of the corpus's other rows
/// (no mass to pin on a side with no material).
///
/// The value is asserted to actually BE `Empty` above and a body below
/// before the constant is checked, so the row cannot go vacuous if the
/// fixture drifts. With that, perturbing the digest's empty-side token
/// reds THIS row while the three document constants stand, and a
/// lowering that turned the typed empty into anything else — or an
/// empty side into a phantom body — moves this number. The constant
/// reproduces on the extracted merge base (the empty path predates the
/// migration), so it is a differential pin, not a self-agreement.
#[test]
fn a_split_with_an_empty_side_evaluates_to_its_committed_digest() {
    let (r, split) = cube_split_at(5.0);
    let ev = eval::<f64>(&r.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "the fixture evaluates: {bad:?}");
    let Some(ValuePayload::Split { above, below }) = ev.value(split).map(|v| &v.payload) else {
        panic!("the split node has no split value");
    };
    assert!(
        matches!(above, SplitSide::Empty),
        "a plane above the cube leaves nothing above it; the fixture no longer produces the empty side"
    );
    assert!(
        matches!(below, SplitSide::Body(_)),
        "everything is below the plane"
    );
    let got = digest(&ev);
    println!("seat8 empty_side: {got:#018x}");
    assert_eq!(
        got, 0xb826_1654_e18b_d14a,
        "the empty-side evaluation moved — side token, body or name table"
    );
}

/// The typed root cause of one node, or a loud failure.
fn error_of(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => &e.kind,
        other => panic!("node {id:?} did not fail typed: {other:?}"),
    }
}

/// **What the projection needs from the two-sided value, pinned where
/// it reads it.** `Node::Part` selects a half by ROLE — above or below
/// the plane's normal, never an index — refuses an empty half typed
/// (`EmptyHalf`, naming the split and the side), and hands a present
/// half on as the split's OWN body: the same `Arc`, no clone, no
/// re-stamp. Those three are the contract the split's out-type carries
/// two role-tagged `Body | Empty` sides for, and the reason it is not
/// a list of bodies or a body with a side marker.
#[test]
fn the_projection_reads_the_two_sided_value_by_role() {
    let (mut r, split) = cube_split_at(5.0);
    let above = r.insert(Node::Part {
        of: split,
        select: PartSelect::SplitHalf(SplitHalf::Above),
    });
    let below = r.insert(Node::Part {
        of: split,
        select: PartSelect::SplitHalf(SplitHalf::Below),
    });
    let ev = eval::<f64>(&r.doc);
    assert!(
        matches!(
            error_of(&ev, above),
            NodeErrorKind::EmptyHalf { input, half: SplitHalf::Above } if *input == split
        ),
        "the empty half must refuse typed, naming the split and the side: {:?}",
        error_of(&ev, above)
    );
    let Some(ValuePayload::Split {
        below: SplitSide::Body(side),
        ..
    }) = ev.value(split).map(|v| &v.payload)
    else {
        panic!("the split's below side is a body");
    };
    let Some(ValuePayload::Body(projected)) = ev.value(below).map(|v| &v.payload) else {
        panic!(
            "the present half projects to a body: {:?}",
            ev.nodes.get(&below)
        );
    };
    assert!(
        Arc::ptr_eq(side, projected),
        "the projection must hand on the split's own body, not a copy"
    );
}

/// Every minted index this node stamped on `body`'s descriptions —
/// surfaces, curves and points — as one set.
fn minted_indices(body: &Body<f64>, node: RecipeNodeId) -> Vec<u32> {
    let mut out = Vec::new();
    let sources = body
        .surfaces()
        .map(|(k, _)| body.surface_source(k))
        .chain(body.curves().map(|(k, _)| body.curve_source(k)))
        .chain(body.points().map(|(k, _)| body.point_source(k)));
    for source in sources.flatten() {
        if source.node == node.0 {
            let SourceExpr::Minted { index } = source.expr else {
                panic!("a split stamps minted sources only, found {source:?}");
            };
            out.push(index);
        }
    }
    out
}

/// **Both halves are stamped in ONE index space** — the property the
/// lowering's counter carry exists for, guarded rather than commented.
///
/// Each half's section plane is its own description with its own
/// outward normal, and the two are the operands of any boolean that
/// joins the halves back together (`part_select` does exactly that):
/// a source shared between them would read as one plane at that
/// boolean's coincidence rung while the bits say two. So every index
/// this node minted on the above half and every index on the below
/// half are distinct, and together they are the contiguous range a
/// single stamp pass numbers. A lowering that restarted the counter
/// on the second half — the red-first mutation this row was written
/// against — gives both section planes index 0 and fails here, and
/// fails the digest rows above with it.
#[test]
fn both_halves_are_stamped_in_one_index_space() {
    let (r, split) = cube_split_at(0.5);
    let ev = eval::<f64>(&r.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "the fixture evaluates: {bad:?}");
    let Some(ValuePayload::Split {
        above: SplitSide::Body(above),
        below: SplitSide::Body(below),
    }) = ev.value(split).map(|v| &v.payload)
    else {
        panic!("the mid-plane cut yields two bodies");
    };
    let (a, b) = (minted_indices(above, split), minted_indices(below, split));
    assert!(
        !a.is_empty() && !b.is_empty(),
        "each half carries something this node minted (its section plane at least): {a:?} / {b:?}"
    );
    let a_set: BTreeSet<u32> = a.iter().copied().collect();
    let b_set: BTreeSet<u32> = b.iter().copied().collect();
    assert_eq!(
        a_set.len(),
        a.len(),
        "the above half repeats an index: {a:?}"
    );
    assert_eq!(
        b_set.len(),
        b.len(),
        "the below half repeats an index: {b:?}"
    );
    let shared: Vec<u32> = a_set.intersection(&b_set).copied().collect();
    assert!(
        shared.is_empty(),
        "the two halves share minted indices {shared:?} — one source over two geometries"
    );
    let all: BTreeSet<u32> = a_set.union(&b_set).copied().collect();
    let expected: BTreeSet<u32> = (0..u32::try_from(all.len()).unwrap()).collect();
    assert_eq!(
        all, expected,
        "the two halves' indices are not one contiguous space from 0"
    );
}
