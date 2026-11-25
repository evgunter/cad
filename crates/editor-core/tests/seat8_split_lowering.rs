//! **The split moved onto the verb substrate and nothing observable
//! moved with it** — the SEAT-4/5/7 method, for the first verb whose
//! result is two-sided.
//!
//! `Node::Split` builds a `verbs::Split`, runs it through the split
//! door, takes its birth record out of the closed record channel and
//! its two sides out of the door's own out-type, and emits names from
//! the record. A re-plumbing's failure mode is a difference nobody
//! looks for — so:
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
//!
//! What already covers this and what these rows add is the same
//! division SEAT-4 recorded: `m10_p_fence` digests every body point's
//! bits corpus-wide and `lib_g16_corpus_name_digests` digests every
//! name table, so either would catch a lowering that changed geometry
//! or names — and neither says which document did it, and neither sees
//! an `Empty` side at all
//! (there is no body to digest and no name to emit for one).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use corpus::{Recorder, eval, failures};
use editor_core::{
    Node, NodeErrorKind, NodeResult, RecipeNodeId, SplitHalf, SplitSide, ValuePayload, persist,
};
use fixture::digest::digest;
use fixture::{len, square, tol};

/// The registered split documents, by name: a tilted plane through a
/// cylinder (curved section edges), a plane through a box with both
/// halves read by port and unioned back (the DM3 payoff), and the kitchen
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
/// halves read by port and unioned — saves, loads and re-saves
/// byte-identically. The corpus-wide round-trip covers the
/// same bytes; this is the per-document form beside the digest rows,
/// so a red here names the split rather than the registry.
#[test]
fn a_split_document_read_by_port_round_trips_byte_identical() {
    let doc = corpus::documents()
        .into_iter()
        .find(|d| d.name == "part_select")
        .expect("the document is registered");
    let first = persist::save(&doc.doc, tol()).expect("the document saves");
    let loaded = persist::load(&first, tol()).expect("its own bytes load back");
    assert!(
        loaded.bit_eq(&doc.doc),
        "the loaded document is not bit-identical to the registered one"
    );
    let second = persist::save(&loaded, tol()).expect("the loaded document re-saves");
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
/// through the verb lowering — bodies on both sides and name tables —
/// one committed number each.
///
/// The digest feeds each body's key-free reading (its faces' senses,
/// carriers and loops, its edges' curves and descriptions, its points'
/// bits), each split side under its role, and each name table, name by
/// name with the cell it denotes (`fixture::digest`'s module docs). It
/// feeds no arena key and no mint id, so a renumbered arena or a
/// re-minted id moves no row; a renamed field or a moved bit does.
/// They are goldens in the ordinary sense — when one moves the question
/// is whether the new behaviour is right, never how to restore the old
/// number — and the PR that re-cuts one says what moved and why.
#[test]
fn the_split_documents_evaluate_to_their_committed_digests() {
    let mut moved: Vec<String> = Vec::new();
    for (name, want) in [
        // Re-cut: PR 7299 places the x = -0.5 seam's split vertex at cross's root, z one ulp up onto the rounded crossing.
        ("cut_cylinder", 0x578d_6718_4b8b_6b9f_u64),
        // Re-cut: PR 7471's along-edge chord keeps the edge's own description, so the cut box's rim lines name the surface record their edges carried in.
        ("part_select", 0x93b9_dff5_cd18_00d3),
        ("kitchen_sink", 0x0027_ff77_60b2_466b),
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
        if got != want {
            moved.push(format!("{name}: {got:#018x} (want {want:#018x})"));
        }
    }
    assert!(
        moved.is_empty(),
        "these documents' evaluations moved — a side or the name table:\n{}",
        moved.join("\n")
    );
}

/// A unit cube on its own xy, the plane at height `z` with `+z` normal
/// read off its geometry, and the split of the one by the other — returns the
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
        profile: profile.into(),
        distance: len(1.0),
    });
    let tool = crate::fixture::plane_on(&r.doc, cube, [0.0, 0.0, z], [0.0, 0.0, 1.0]);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool,
    });
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
/// empty side into a phantom body — moves this number.
///
/// empty side into a phantom body — moves this number. The constant
/// reproduces on the extracted merge base (the empty path predates the
/// migration), so it is a differential pin, not a self-agreement.
///
/// RE-BLESSED for the axis-order orthonormal basis: the digest feeds
/// each surface's `Debug`, and every planar carrier's stored `u_ref`
/// is now `e_z × n` or `e_y × n` — whichever axis the comparison
/// `|n.z| ≤ max(|n.x|, |n.y|)/2` picks — divided by its own length.
/// The plane's LOCUS did not move — origin and normal are
/// bit-identical, which the STEP fixtures' record-level diff shows
/// directly — and the id-free body rows (`m4_pr8_corpus`'s exact mass
/// pins, `m5_pr8_bvh_diff`'s realized-vs-idealized bit equality) were
/// green across the change untouched.
///
/// RE-BLESSED for INTENT-LITERALS PR C (a slot holds a variable): every
/// node is minted from slots holding variable ids, so every id moved
/// and this digest feeds ids. No outcome or point moved:
/// `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
/// held untouched.
///
/// RE-BLESSED when an intrinsic edge description's two surfaces became a
/// set (`geom_brep::SurfacePair`), re-taken on the tree holding every
/// change above: the digest feeds each edge's description `Debug`,
/// which spells `pair: {a, b}` in key order where it spelled `s1`, `s2`
/// in the builder's order. No body moved: `m4_pr8_corpus`'s exact mass
/// pins and `m5_pr8_bvh_diff` held.
///
/// RE-BLESSED when a name came to be scoped by the variable holding its
/// body (FORK-DM4 unit 2): the split's one table became a table per
/// half, the empty half's empty, and its names spell no half.
///
/// RE-BLESSED for INTENT stage 3 D (Transform retires into a
/// placement): the split's tool is a plane read off the cube's own
/// geometry (`fixture::plane_on`) where it was a `Datum::Plane`, and the
/// profile carries no frame, so its nodes, their ids and the names they
/// mint moved.
///
/// RE-BLESSED for exact turns (INTENT stage 3 D): the tool is the
/// cube's end cap's plane slid along its normal (`fixture::plane_on`
/// reads a face with the stated normal) where it was a frame read off
/// the cube's edges moved by turns read off by `atan2`, so its pose's
/// nodes and the names the split mints moved.
///
/// RE-BLESSED when a minted plane's `u_ref` became a direction its
/// construction holds: the cube's caps are charted along its sketch's
/// `x` axis and its walls along their swept segments, where
/// `orthonormal_basis` picked both off the axes. No plane moved as a
/// point set.
///
/// RE-BLESSED for D′1 (a placement names its target space): each
/// construction defines its space beside its body, and the content key
/// holds a placement's target (`VERSION` 18), so ids drawn after a
/// construction and every value token moved. No point moved.
///
/// RE-BLESSED for a certificate that keeps its turn: the digest feeds
/// each edge curve's `Debug`, which now spells the certificate's `turn`.
/// No outcome or point moved.
///
/// RE-BLESSED for limb 2 for every carrier (PR 5728): check 2's turn
/// moved from the edge's `Certificate` onto the edge, which changes the
/// edge curve's `Debug` the digest feeds. With that `Debug` written in
/// main's shape every digest here reproduces main's value at all three
/// ε rows, so nothing else moved.
///
/// RE-BLESSED when `Intersection`'s witness became its selector
/// (`selector: Witness(p)` where it spelled `witness: p`), which changes
/// the edge curve's `Debug` the digest feeds. With that `Debug` written in
/// main's shape the digest reproduces main's value at all three ε rows,
/// so nothing else moved.
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
        got, 0x7b5c_023e_2261_485f,
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

/// **What a read of a half needs from the two-sided value, pinned where
/// it reads it.** Each port is one half by ROLE — above or below the
/// plane's normal (`SplitHalf::port`) — a read of an empty half refuses
/// typed (`EmptyHalf`, naming the split and the side), and a present
/// half is read as the split's own side. Those three are the contract
/// the split's out-type carries two role-tagged `Body | Empty` sides
/// for, and the reason it is not a list of bodies or a body with a side
/// marker.
#[test]
fn a_read_of_a_half_reads_the_two_sided_value_by_role() {
    let (mut r, split) = cube_split_at(5.0);
    // A copy of each half, loose: a reader of the half's value.
    let still = |half: SplitHalf| {
        crate::fixture::stood(editor_core::BodyRead::plain(editor_core::Operand::output(
            split,
            half.port(),
        )))
    };
    let above = r.insert(still(SplitHalf::Above));
    let below = r.insert(still(SplitHalf::Below));
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
    let read = match ev.value(below).map(|v| &v.payload) {
        Some(ValuePayload::Body(body)) => body,
        Some(ValuePayload::Instances(copies)) => {
            let [Some(read)] = &copies[..] else {
                panic!("one copy of the half: {}", copies.len());
            };
            read
        }
        _ => panic!("the present half is read: {:?}", ev.result(below).is_some()),
    };
    assert_eq!(
        (
            read.body().faces().count(),
            read.body().edges().count(),
            read.body().vertices().count()
        ),
        (
            side.body().faces().count(),
            side.body().edges().count(),
            side.body().vertices().count()
        ),
        "the reader holds the split's own side"
    );
}
