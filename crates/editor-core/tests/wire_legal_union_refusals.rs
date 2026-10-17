//! **An emitter refusal an ordinary union reaches, and what
//! the author is told when it does — and one it no longer reaches.**
//!
//! `NamingError::Emission` means a mint-time fact disagreed with the
//! result body — a kernel bug. The refusal pinned here is not one: the
//! recipe is well formed, every member is an ordinary solid, and the
//! bodies the fold builds are sound. What is missing is a naming RULE
//! for the construction the fold produced, and the row pins that the
//! refusal says so rather than sending the author to file a bug against
//! the kernel. The next two rows are constructions that once refused
//! and now name: a merged wall whose edges are pieces of member edges,
//! and a rim in several pieces. The last three run members whose
//! coincident faces overlap in area, under a covering block, in every
//! member order: no fold step may refuse a contact the pairwise
//! judgement passed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::corpus::body_of;
use crate::docm8_flat_merged::split_fixture;

use editor_core::{NamingError, NodeErrorKind, ProfileDoc, RecipeNodeId};
use geom_core::Tol;

/// The sentence every emission-bug refusal opens with, written out
/// because what this suite is about is a reader meeting it: a refusal
/// reached from a legal recipe must NOT read as a kernel bug report.
const BUG_FRAMING: &str =
    "name emission found a kernel bug — an invariant it relies on does not hold";

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass")
        .volume
}

/// **A merged face no rule reads a seam chord through refuses as a
/// missing rule.**
///
/// A plate and a cylinder boss sunk into it, its wall drawn as two
/// semicircular pieces, joined by a pair union; a slab subtracted across
/// the boss along y. The slab cuts each wall piece in two, and the
/// subtract merges the two cosurface pieces on each side of the slot, so
/// each merged wall face holds both boss pieces. The chord where a slab
/// wall meets it is a join's own and reads through to its A-side
/// constituent, of which there are two. Nothing picks the one the chord
/// lies on.
///
/// Nothing is wrong with this document: the boss joins the plate, and
/// the slot along x across the same boss names
/// (`emit_union_borders::a_slot_along_x_across_a_sunk_boss_names`).
#[test]
fn a_merged_face_with_several_constituents_is_a_missing_rule_not_a_kernel_bug() {
    let doc = ProfileDoc::empty_derived("wire_merged_chord", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, boss) = crate::emit_union_borders::cylinder(
        doc,
        [1.5, 1.5, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        1.0,
    );
    let (doc, slab) = block(doc, (1.4, 1.6), (-1.0, 4.0), 0.8, 1.2);
    let (doc, joined) = crate::fixture::insert(
        doc,
        editor_core::Node::Union {
            members: editor_core::Bodies::Spelled(vec![plate.into(), boss.into()]),
        },
    );
    let (doc, cut) = crate::fixture::insert(
        doc,
        editor_core::Node::Subtract {
            from: joined.into(),
            tool: slab.into(),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, joined).is_none(), "the boss joins the plate");
    // Rendered at the NODE boundary, which is the only route by which an
    // emitter refusal reaches a human.
    let shown = match failure(&ev, cut) {
        Some(
            e @ NodeErrorKind::Naming(NamingError::MergedChordConstituents { several: 2, .. }),
        ) => e.to_string(),
        other => panic!("wanted the merged-chord refusal, got {other:?}"),
    };
    assert!(
        !shown.contains(BUG_FRAMING),
        "a legal document was told the kernel is broken: {shown}"
    );
    assert!(
        shown.contains("merged face"),
        "the refusal must name the construction: {shown}"
    );
    assert!(
        !shown.contains("  "),
        "prose a human reads has no padded run in it: {shown}"
    );
}

/// **A split piece the fold's kills cut off is a piece, not a chord.**
///
/// `a` and `c` meet flush along x on all four families; `s` rises
/// through `a`'s top cap across its depth, flush with `a`'s two
/// y-walls. Folding `a` in last glues a face of `c` and of `s` into one
/// merged wall with `a`'s. The edges along that wall are pieces of
/// `a`'s and `s`'s own edges whose split lineage runs through the
/// stretches the fold killed, so they are named as those pieces, and
/// every order fuses to the volume the geometry says with every member
/// edge it names lying on its source. Folding `a` in last refused
/// `MergedChordConstituents` while the lineage stopped at the killed
/// stretches and the edges were read as chords through the merged wall.
#[test]
fn a_split_fixture_with_a_merged_wall_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("wire_seam_vertex", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block(doc, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    for order in [
        vec![a, c, s],
        vec![a, s, c],
        vec![c, a, s],
        vec![c, s, a],
        vec![s, a, c],
        vec![s, c, a],
    ] {
        let (docx, union) = crate::fixture::union_over(doc.clone(), &order);
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: the union fuses: {:?}",
            failure(&ev, union)
        );
        let v = volume(body_of(&ev, union));
        assert!((v - 1.6).abs() < 1e-9, "{order:?}: volume {v}");
        crate::emit_shared_rim_several::every_member_edge_lies_on_its_source(
            &docx,
            &ev,
            union,
            &order,
            &format!("{order:?}"),
        );
    }
}

/// **A rim that comes in several pieces no longer refuses.**
///
/// `a` and `b` meet flush along x, so the fold's first step merges
/// their caps; `g` is a slab that rises through the
/// merged top cap and out through both y-walls. At step 2 an edge along
/// the merged cap's rim is a piece of one of the pieces step 1 left on
/// that line, split off around a stretch `g` killed, and is named as
/// that piece through its lineage. This document refused
/// `SharedRim { found: Several }` while the lineage stopped at the
/// killed stretch and the edge was read as a chord between two faces
/// sharing several pieces; the refusal's sentence stays pinned in
/// `display_contract`.
///
/// The recipe is legal: the same two members fuse on their own, `g` is
/// an ordinary overlapping solid, and the three together fuse to the
/// volume the geometry says.
#[test]
fn a_rim_in_several_pieces_is_named_not_refused() {
    let doc = ProfileDoc::empty_derived("wire_shared_rim", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block(doc, (0.7, 0.8), (-1.0, 2.0), 0.5, 3.0);

    let (docx, union) = crate::fixture::union_over(doc, &[a, b, g]);
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    // a ∪ b is 1.5; g (z = 0.5..3.5) adds 0.1 × 3 × 3 less its
    // 0.1 × 1 × 0.5 inside.
    let v = volume(body_of(&ev, union));
    assert!((v - 2.35).abs() < 1e-9, "volume {v}");
}

/// What one member order of a union came to. Every refusal here is a
/// missing rule another row owns; none is the document's fault.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Seen {
    /// One body, at the volume the geometry says.
    Fused,
}

/// Every ordering of `0..n`.
fn orders(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    for first in 0..n {
        for rest in orders(n - 1) {
            let mut o = vec![first];
            o.extend(rest.into_iter().map(|k| k + usize::from(k >= first)));
            out.push(o);
        }
    }
    out
}

/// Runs the union of `members` in every order and checks the
/// outcomes against `want`, one line per order: an order that fuses
/// must reach `fused_volume`, and an order that refuses must refuse with a
/// missing rule some other row owns.
///
/// The refusal that must never appear is the fold's own contact verdict:
/// `judge_pairwise_contact` passed every member pair before the fold,
/// so a fold step that refuses a contact would be judging one no pair
/// has (`fold_step_refusal`).
fn every_order(
    doc: &ProfileDoc,
    members: &[(&str, RecipeNodeId)],
    fused_volume: f64,
    want: &[(&str, Seen)],
) {
    let mut seen: Vec<(String, Seen)> = orders(members.len())
        .into_iter()
        .map(|o| {
            let label = o
                .iter()
                .map(|&k| members[k].0)
                .collect::<Vec<_>>()
                .join(",");
            let order: Vec<RecipeNodeId> = o.iter().map(|&k| members[k].1).collect();
            let (docx, union) = crate::fixture::union_over(doc.clone(), &order);
            let ev = run(&docx);
            let outcome = match failure(&ev, union) {
                None => {
                    let v = volume(body_of(&ev, union));
                    assert!(
                        (v - fused_volume).abs() < 1e-9,
                        "[{label}]: volume {v}, want {fused_volume}"
                    );
                    Seen::Fused
                }
                Some(other) => panic!("[{label}]: a legal union refused {other:?}"),
            };
            (label, outcome)
        })
        .collect();
    let mut want: Vec<(String, Seen)> = want.iter().map(|&(o, s)| (o.to_owned(), s)).collect();
    seen.sort();
    want.sort();
    assert_eq!(seen, want, "the outcome of some member order moved");
}

/// `a` is a unit block; `s` rises through `a`'s top cap across its
/// whole depth, flush with `a`'s two y-walls, which it overlaps in
/// area; `big` swallows `a`'s top cap and the part of `s` above
/// z = 0.8. Folding `a` into an accumulation that holds `s` glues each
/// coincident wall pair into one merged face, so the step after it meets
/// no coplanar pair inside one operand, and no fold step judges a
/// contact the pairwise pre-pass did not.
fn area_overlap_fixture(doc: ProfileDoc) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block(doc, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let (doc, big) = block(doc, (-1.0, 2.0), (-1.0, 2.0), 0.8, 1.4);
    (doc, [a, s, big])
}

/// `big` (12.6) plus `a` below z = 0.8 (0.8); the rest of `a` and all
/// of `s` lie inside the two.
const AREA_OVERLAP_VOLUME: f64 = 13.4;

#[test]
fn no_order_of_an_area_overlap_under_a_covering_block_refuses_a_fold_contact() {
    use Seen::Fused;
    let doc = ProfileDoc::empty_derived("wire_fold_contact_3", Tol::witness());
    let (doc, [a, s, big]) = area_overlap_fixture(doc);
    // Every order fuses: the vertex whose half-decided parentage refused
    // two of them was a cut vertex the output stage now joins away.
    every_order(
        &doc,
        &[("a", a), ("s", s), ("big", big)],
        AREA_OVERLAP_VOLUME,
        &[
            ("a,s,big", Fused),
            ("a,big,s", Fused),
            ("s,a,big", Fused),
            ("s,big,a", Fused),
            ("big,a,s", Fused),
            ("big,s,a", Fused),
        ],
    );
}

/// The same three, plus `p` resting on `a`'s top cap inside `big`.
#[test]
fn no_order_of_the_area_overlap_union_with_a_fourth_member_refuses_a_fold_contact() {
    use Seen::Fused;
    let doc = ProfileDoc::empty_derived("wire_fold_contact_4", Tol::witness());
    let (doc, [a, s, big]) = area_overlap_fixture(doc);
    let (doc, p) = block(doc, (0.6, 0.9), (0.2, 0.8), 1.0, 0.2);
    every_order(
        &doc,
        &[("a", a), ("s", s), ("big", big), ("p", p)],
        AREA_OVERLAP_VOLUME,
        &[
            ("a,s,big,p", Fused),
            ("a,s,p,big", Fused),
            ("a,big,s,p", Fused),
            ("a,big,p,s", Fused),
            ("a,p,s,big", Fused),
            ("a,p,big,s", Fused),
            ("s,a,big,p", Fused),
            ("s,a,p,big", Fused),
            ("s,big,a,p", Fused),
            ("s,big,p,a", Fused),
            ("s,p,a,big", Fused),
            ("s,p,big,a", Fused),
            ("big,a,s,p", Fused),
            ("big,a,p,s", Fused),
            ("big,s,a,p", Fused),
            ("big,s,p,a", Fused),
            ("big,p,a,s", Fused),
            ("big,p,s,a", Fused),
            ("p,a,s,big", Fused),
            ("p,a,big,s", Fused),
            ("p,s,a,big", Fused),
            ("p,s,big,a", Fused),
            ("p,big,a,s", Fused),
            ("p,big,s,a", Fused),
        ],
    );
}

/// R1's split fixture (`a` and `c` flush along x, `s` flush with `a`'s
/// y-walls) under a block covering `a`'s top cap: a fourth member over
/// the same area overlap.
#[test]
fn no_order_of_the_split_fixture_under_a_covering_block_refuses_a_fold_contact() {
    use Seen::Fused;
    let doc = ProfileDoc::empty_derived("wire_fold_contact_split", Tol::witness());
    let (doc, [a, c, s]) = split_fixture(doc);
    let (doc, big) = block(doc, (-0.5, 1.2), (-0.5, 1.5), 0.8, 1.2);
    // `big` (4.08) plus `a ∪ c` (1.5) less their overlap (1.2 × 1 × 0.2).
    every_order(
        &doc,
        &[("a", a), ("c", c), ("s", s), ("big", big)],
        5.34,
        &[
            ("a,c,s,big", Fused),
            ("a,c,big,s", Fused),
            ("a,s,c,big", Fused),
            ("a,s,big,c", Fused),
            ("a,big,c,s", Fused),
            ("a,big,s,c", Fused),
            ("c,a,s,big", Fused),
            ("c,a,big,s", Fused),
            ("c,s,a,big", Fused),
            ("c,s,big,a", Fused),
            ("c,big,a,s", Fused),
            ("c,big,s,a", Fused),
            ("s,a,c,big", Fused),
            ("s,a,big,c", Fused),
            ("s,c,a,big", Fused),
            ("s,c,big,a", Fused),
            ("s,big,a,c", Fused),
            ("s,big,c,a", Fused),
            ("big,a,c,s", Fused),
            ("big,a,s,c", Fused),
            ("big,c,a,s", Fused),
            ("big,c,s,a", Fused),
            ("big,s,a,c", Fused),
            ("big,s,c,a", Fused),
        ],
    );
}
