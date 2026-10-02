//! **An emitter refusal an ordinary declared union reaches, and what
//! the author is told when it does — and one it no longer reaches.**
//!
//! `NamingError::Emission` means a mint-time fact disagreed with the
//! result body — a kernel bug. The refusal pinned here is not one: the
//! recipe is well formed, every member is an ordinary solid, and the
//! bodies the fold builds are sound. What is missing is a naming RULE
//! for the construction the fold produced, and the row pins that the
//! refusal says so rather than sending the author to file a bug against
//! the kernel. The second row is a construction that once refused the
//! same way and now has its rule. The last three run a declaration whose
//! faces overlap in area, under a covering block, in every member order:
//! no fold step may refuse a contact the pairwise judgement passed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{
    block, declared_union, declared_union_classed, failure, flush_pairs, run,
};
use crate::docm8_flat_merged::split_fixture;
use crate::fixture::{fname, wall};

use editor_core::{
    BooleanCoincidence, CapEnd, Diagnosis, FoldConsumption, NamingError, NodeErrorKind, ProfileDoc,
    RecipeNodeId, ResolveError, RoleSeg, SitedRef,
};
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
/// `a` and `c` meet flush along x on all four families; `s` rises
/// through `a`'s top cap across its depth, declared against `a`'s two
/// y-walls. Fold `a` in last and `c` and `s` are an assembly of two
/// bodies when it joins: the declared y-walls glue a face of EACH into
/// one merged wall together with `a`'s, and a seam chord bordering
/// that wall reads through to its A-side constituent, of which there
/// are two. Nothing picks the one the chord lies on.
///
/// Nothing is wrong with this document, and the other four orders of
/// the same three members are the proof: two of them fuse to a body
/// with the volume the geometry says.
#[test]
fn a_merged_face_with_several_constituents_is_a_missing_rule_not_a_kernel_bug() {
    let doc = ProfileDoc::empty_derived("wire_seam_vertex", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block(doc, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let pairs = {
        let mut v = flush_pairs(&doc, (a, a), (c, c));
        for seg in [0, 2] {
            v.push((
                SitedRef::new(a, fname(a, wall(&doc, a, seg))),
                SitedRef::new(s, fname(s, wall(&doc, s, seg))),
            ));
        }
        v
    };

    // The orders that fold `a` in last, and only those.
    for order in [vec![c, s, a], vec![s, c, a]] {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs.clone());
        let ev = run(&docx);
        let shown = match failure(&ev, union) {
            // Rendered at the NODE boundary, which is the only route by
            // which an emitter refusal reaches a human.
            Some(
                e @ NodeErrorKind::Naming(NamingError::MergedChordConstituents {
                    several: 2, ..
                }),
            ) => e.to_string(),
            other => panic!("{order:?}: wanted the merged-chord refusal, got {other:?}"),
        };
        assert!(
            !shown.contains(BUG_FRAMING),
            "{order:?}: a legal document was told the kernel is broken: {shown}"
        );
        assert!(
            shown.contains("merged face"),
            "{order:?}: the refusal must name the construction: {shown}"
        );
        assert!(
            !shown.contains("  "),
            "{order:?}: prose a human reads has no padded run in it: {shown}"
        );
    }

    // The same three members, folded the other way round, build a body.
    // Without this the row above could be pinning a malformed recipe.
    for order in [vec![a, c, s], vec![c, a, s]] {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs.clone());
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: this document fuses, so the refusal above is about \
             the fold order and not about the recipe"
        );
        let v = volume(body_of(&ev, union));
        assert!((v - 1.6).abs() < 1e-9, "{order:?}: volume {v}");
    }
}

/// **A rim that comes in several pieces no longer refuses.**
///
/// `a` and `b` meet flush along x with their caps declared, so the
/// fold's first step merges them; `g` is a slab that rises through the
/// merged top cap and out through both y-walls. At step 2 a seam chord
/// descends into two faces of the A operand that share their common
/// line in several pieces, and the chord lies within exactly one of
/// them, which names it (`emit_topo`'s `rim_holding`). This document
/// refused `SharedRim { found: Several }` until that rule existed; the
/// refusal's sentence stays pinned in `display_contract`.
///
/// The recipe is legal: the same declaration over the same two members
/// fuses on its own, and `g` is an ordinary overlapping solid declared
/// against nothing — and the three together fuse to the volume the
/// geometry says.
#[test]
fn a_rim_in_several_pieces_is_named_not_refused() {
    let doc = ProfileDoc::empty_derived("wire_shared_rim", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block(doc, (0.7, 0.8), (-1.0, 2.0), 0.5, 3.0);
    let pairs = flush_pairs(&doc, (a, a), (b, b));

    let (docx, union, _) = declared_union(doc, &[a, b, g], pairs);
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
    /// `NamingError::SeamVertexParentage`: the seam-vertex pass has no
    /// parentage rule for a vertex
    /// (`work/wire/a-merged-face-with-several-same-side-constituents-has-no-chord-rule`).
    SeamVertex,
    /// `NamingError::MergedChordConstituents`, the same row's chord rule.
    MergedChord,
    /// A declared face a later member split before its pair's step
    /// (`work/gather/member-space-look-through-stops-at-splits-containment-and-fragmented-merges`).
    Split,
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

/// Each wall pair as the continuation it is: the fixtures' walls carry
/// on into one another with aligned senses.
fn continuations(
    pairs: &[(SitedRef, SitedRef)],
) -> Vec<((SitedRef, SitedRef), BooleanCoincidence)> {
    pairs
        .iter()
        .map(|p| (p.clone(), BooleanCoincidence::Continuation))
        .collect()
}

/// Runs the declared union of `members` in every order and checks the
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
    pairs: &[((SitedRef, SitedRef), BooleanCoincidence)],
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
            let (docx, union, _) = declared_union_classed(doc.clone(), &order, pairs.to_vec());
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
                Some(NodeErrorKind::Naming(NamingError::SeamVertexParentage { .. })) => {
                    Seen::SeamVertex
                }
                Some(NodeErrorKind::Naming(NamingError::MergedChordConstituents { .. })) => {
                    Seen::MergedChord
                }
                Some(NodeErrorKind::DeclareResolve { error })
                    if matches!(
                        &**error,
                        ResolveError::Vanished {
                            diagnosis: Diagnosis::ConsumedByFold {
                                by: FoldConsumption::Split
                            },
                            ..
                        }
                    ) =>
                {
                    Seen::Split
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
/// whole depth, declared against `a`'s two y-walls, which it overlaps
/// in area; `big` swallows `a`'s top cap and the part of `s` above
/// z = 0.8. Folding `a` into an accumulation that holds `s` glues each
/// declared wall pair into one merged face, so the step after it meets
/// no coplanar pair inside one operand, and no fold step judges a
/// contact the pairwise pre-pass did not.
fn area_overlap_fixture(
    doc: ProfileDoc,
) -> (ProfileDoc, [RecipeNodeId; 3], Vec<(SitedRef, SitedRef)>) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block(doc, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let (doc, big) = block(doc, (-1.0, 2.0), (-1.0, 2.0), 0.8, 1.4);
    let pairs = [0, 2]
        .into_iter()
        .map(|seg| {
            (
                SitedRef::new(a, fname(a, wall(&doc, a, seg))),
                SitedRef::new(s, fname(s, wall(&doc, s, seg))),
            )
        })
        .collect();
    (doc, [a, s, big], pairs)
}

/// `big` (12.6) plus `a` below z = 0.8 (0.8); the rest of `a` and all
/// of `s` lie inside the two.
const AREA_OVERLAP_VOLUME: f64 = 13.4;

#[test]
fn no_order_of_an_area_overlap_declaration_under_a_covering_block_refuses_a_fold_contact() {
    use Seen::{Fused, SeamVertex};
    let doc = ProfileDoc::empty_derived("wire_fold_contact_3", Tol::witness());
    let (doc, [a, s, big], pairs) = area_overlap_fixture(doc);
    every_order(
        &doc,
        &[("a", a), ("s", s), ("big", big)],
        &continuations(&pairs),
        AREA_OVERLAP_VOLUME,
        &[
            ("a,s,big", Fused),
            ("a,big,s", SeamVertex),
            ("s,a,big", Fused),
            ("s,big,a", Fused),
            ("big,a,s", SeamVertex),
            ("big,s,a", Fused),
        ],
    );
}

/// The same three, plus `p` resting on `a`'s top cap inside `big`,
/// declared against that cap.
#[test]
fn no_order_of_the_area_overlap_union_with_a_fourth_member_refuses_a_fold_contact() {
    use Seen::{Fused, SeamVertex, Split};
    let doc = ProfileDoc::empty_derived("wire_fold_contact_4", Tol::witness());
    let (doc, [a, s, big], pairs) = area_overlap_fixture(doc);
    let (doc, p) = block(doc, (0.6, 0.9), (0.2, 0.8), 1.0, 0.2);
    // `p` rests on `a`'s top cap: the two caps face each other, a
    // `Rest` contact.
    let mut pairs = continuations(&pairs);
    pairs.push((
        (
            SitedRef::new(a, fname(a, RoleSeg::Cap(CapEnd::End))),
            SitedRef::new(p, fname(p, RoleSeg::Cap(CapEnd::Start))),
        ),
        BooleanCoincidence::REST,
    ));
    every_order(
        &doc,
        &[("a", a), ("s", s), ("big", big), ("p", p)],
        &pairs,
        AREA_OVERLAP_VOLUME,
        &[
            ("a,s,big,p", Fused),
            ("a,s,p,big", Split),
            ("a,big,s,p", SeamVertex),
            ("a,big,p,s", SeamVertex),
            ("a,p,s,big", Fused),
            ("a,p,big,s", SeamVertex),
            ("s,a,big,p", Fused),
            ("s,a,p,big", Split),
            ("s,big,a,p", Fused),
            ("s,big,p,a", Fused),
            ("s,p,a,big", Fused),
            ("s,p,big,a", Fused),
            ("big,a,s,p", SeamVertex),
            ("big,a,p,s", SeamVertex),
            ("big,s,a,p", Fused),
            ("big,s,p,a", Fused),
            ("big,p,a,s", SeamVertex),
            ("big,p,s,a", Fused),
            ("p,a,s,big", Fused),
            ("p,a,big,s", SeamVertex),
            ("p,s,a,big", Fused),
            ("p,s,big,a", Fused),
            ("p,big,a,s", SeamVertex),
            ("p,big,s,a", Fused),
        ],
    );
}

/// R1's split fixture (`a` and `c` flush along x, `s` declared against
/// `a`'s y-walls) under a block covering `a`'s top cap: a fourth
/// member over the same area-overlap declaration.
#[test]
fn no_order_of_the_split_fixture_under_a_covering_block_refuses_a_fold_contact() {
    use Seen::{Fused, MergedChord, SeamVertex, Split};
    let doc = ProfileDoc::empty_derived("wire_fold_contact_split", Tol::witness());
    let (doc, [a, c, s], pairs) = split_fixture(doc);
    let (doc, big) = block(doc, (-0.5, 1.2), (-0.5, 1.5), 0.8, 1.2);
    // `big` (4.08) plus `a ∪ c` (1.5) less their overlap (1.2 × 1 × 0.2).
    every_order(
        &doc,
        &[("a", a), ("c", c), ("s", s), ("big", big)],
        &continuations(&pairs),
        5.34,
        &[
            ("a,c,s,big", Fused),
            ("a,c,big,s", SeamVertex),
            ("a,s,c,big", Split),
            ("a,s,big,c", SeamVertex),
            ("a,big,c,s", SeamVertex),
            ("a,big,s,c", SeamVertex),
            ("c,a,s,big", Fused),
            ("c,a,big,s", SeamVertex),
            ("c,s,a,big", MergedChord),
            ("c,s,big,a", SeamVertex),
            ("c,big,a,s", SeamVertex),
            ("c,big,s,a", SeamVertex),
            ("s,a,c,big", Split),
            ("s,a,big,c", SeamVertex),
            ("s,c,a,big", MergedChord),
            ("s,c,big,a", SeamVertex),
            ("s,big,a,c", SeamVertex),
            ("s,big,c,a", SeamVertex),
            ("big,a,c,s", SeamVertex),
            ("big,a,s,c", SeamVertex),
            ("big,c,a,s", SeamVertex),
            ("big,c,s,a", SeamVertex),
            ("big,s,a,c", SeamVertex),
            ("big,s,c,a", SeamVertex),
        ],
    );
}
