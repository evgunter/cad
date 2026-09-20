//! **Two emitter refusals an ordinary declared union reaches, and what
//! the author is told when it does.**
//!
//! `NamingError::Emission` means a mint-time fact disagreed with the
//! result body — a kernel bug. Neither refusal pinned here is one:
//! every recipe below is well formed, every member is an ordinary
//! solid, and the bodies the fold builds are sound. What is missing is
//! a naming RULE for the construction the fold produced, and these rows
//! pin that the refusal says so rather than sending the author to file
//! a bug against the kernel.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{fname, wall};

use editor_core::{NamingError, NodeErrorKind, ProfileDoc, RimShare, SitedRef};
use geom_core::Tol;

/// The sentence `NamingError::Emission` opens with, written out because
/// what this suite is about is a reader meeting it: a refusal reached
/// from a legal recipe must NOT read as a report that the result body
/// is wrong.
const BUG_FRAMING: &str = "inconsistent with the result body";

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass")
        .volume
}

/// **A seam vertex no rule names refuses as a missing rule.**
///
/// `a` and `c` meet flush along x on all four families; `s` is stacked
/// over both, declared against the two y-walls. Fold `a` in last and
/// the vertex where `s`'s seam meets the merged y-wall has neither one
/// operand-descended edge on each side nor two seam lines, so the
/// emitter's case analysis has no arm for it.
///
/// Nothing is wrong with this document, and the other four orders of
/// the same three members are the proof: two of them fuse to a body
/// with the volume the geometry says.
#[test]
fn a_seam_vertex_no_rule_names_is_a_missing_rule_not_a_kernel_bug() {
    let doc = ProfileDoc::empty_derived("wire_seam_vertex", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block(doc, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let pairs = {
        let mut v = flush_pairs((a, a), (c, c));
        for seg in [wall(0), wall(2)] {
            v.push((
                SitedRef::new(a, fname(a, seg.clone())),
                SitedRef::new(s, fname(s, seg)),
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
            Some(e @ NodeErrorKind::Naming(NamingError::SeamVertexParentage { .. })) => {
                e.to_string()
            }
            other => panic!("{order:?}: wanted the seam-vertex refusal, got {other:?}"),
        };
        assert!(
            !shown.contains(BUG_FRAMING),
            "{order:?}: a legal document was told the kernel is broken: {shown}"
        );
        assert!(
            shown.contains("seam vertex"),
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

/// **A rim that is not unique refuses as a missing rule, naming the
/// face pair.**
///
/// `a` and `b` meet flush along x with their caps declared, so the
/// fold's first step merges them; `g` is a slab that rises through the
/// merged top cap and out through both y-walls. At step 2 a seam chord
/// GUESSES that two faces it descended into the A operand carry its
/// rim, and the pair does not have one.
///
/// The premise is what makes this a missing rule rather than a bug, so
/// the row also pins WHICH caller raised it: `emit_sweep` asks the same
/// walk about a body it built itself and calls the same answer an
/// emission inconsistency.
///
/// The recipe is legal: the same declaration over the same two members
/// fuses on its own, and `g` is an ordinary overlapping solid declared
/// against nothing.
#[test]
fn a_rim_that_is_not_unique_is_a_missing_rule_not_a_kernel_bug() {
    let doc = ProfileDoc::empty_derived("wire_shared_rim", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block(doc, (0.7, 0.8), (-1.0, 2.0), 0.5, 3.0);
    let pairs = flush_pairs((a, a), (b, b));

    // The declaration on its own is well formed and fuses.
    let (docx, ab, _) = declared_union(doc.clone(), &[a, b], pairs.clone());
    let ev = run(&docx);
    assert!(failure(&ev, ab).is_none(), "{:?}", failure(&ev, ab));
    let v = volume(body_of(&ev, ab));
    assert!((v - 1.5).abs() < 1e-9, "the two members fuse: volume {v}");

    let (docx, union, _) = declared_union(doc, &[a, b, g], pairs);
    let ev = run(&docx);
    let shown = failure(&ev, union)
        .map(ToString::to_string)
        .unwrap_or_default();
    let (face, other) = match failure(&ev, union) {
        Some(NodeErrorKind::Naming(NamingError::SharedRim {
            face, other, found, ..
        })) => {
            assert_eq!(
                *found,
                RimShare::Several,
                "a split merged cap leaves the pair sharing TWO edges, not none"
            );
            (*face, *other)
        }
        other => panic!("wanted the shared-rim refusal, got {other:?}"),
    };
    assert_ne!(
        face, other,
        "the refusal names a PAIR; one face twice would be a different fact"
    );
    assert!(
        !shown.contains(BUG_FRAMING),
        "a legal document was told the kernel is broken: {shown}"
    );
    assert!(
        shown.contains(&format!("{face:?}")) && shown.contains(&format!("{other:?}")),
        "both faces of the pair reach the sentence: {shown}"
    );
    assert!(
        shown.contains("operand node"),
        "the keys are an OPERAND body's, so the sentence must say whose: {shown}"
    );
    assert!(
        !shown.contains("  "),
        "prose a human reads has no padded run in it: {shown}"
    );
}
