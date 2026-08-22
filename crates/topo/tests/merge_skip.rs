//! The declared-merge SKIP lane (M4 PR 5 review F1/F2, reviewer's
//! probe adopted as the pin): flush caps DECLARED on an otherwise
//! ordinary partial overlap (walls offset). The cap groups license,
//! the chain-shaped shared seam edges put them outside the
//! never-elide inventory, and the merge SKIPS them — the result must
//! still be FULLY honest:
//!
//! - tier 2 AND tier 3 green (F1: the skipped faces' in-plane cut
//!   edges are re-described against the ACTUAL adjacency — no stale
//!   `Intersection`/`Seam` rows citing no-longer-adjacent surfaces);
//! - the skip is VISIBLE (F2): `BooleanNaming::merge_skipped` carries
//!   the group's faces and the real refusing diagnostics (F3), never
//!   a write-only record or a generic label.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{flush_declarations, prism_z};
use geom_core::Tol;
use topo::validate::{validate_closed, validate_geometric};
use topo::{Body, BooleanResult, mass_properties, union_with, validate_pseudomanifold};

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// The probe: a = [0,1]³, b = [0.5,1.5]×[0.25,1.25]×[0,1] — caps flush
/// (declared), walls offset (transversal). Exact dyadic volume:
/// 1 + 1 − (0.5 · 0.75 · 1).
#[test]
fn skipped_declared_merge_is_tier3_green_and_visible() {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick((0.5, 1.5), (0.25, 1.25), (0.0, 1.0));
    let decls = flush_declarations(&a, &b);
    assert_eq!(decls.coincident_faces.len(), 2, "both caps declared");
    let r = union_with(&a, &b, &decls, Tol::witness()).expect("declared flush-caps union runs");
    let BooleanResult::Body(bb) = r else {
        panic!("overlapping union cannot be Empty");
    };
    assert_eq!(
        mass_properties(&bb.body, Tol::witness()).unwrap().volume,
        1.0 + 1.0 - 0.5 * 0.75,
        "exact dyadic volume"
    );
    assert_eq!(validate_closed(&bb.body), Ok(()), "tier 2");
    // F1: the skip lane must not ship stale descriptions — tier 3
    // green at rest, and 3′ with the op's own contacts.
    assert_eq!(
        validate_geometric(&bb.body, Tol::witness()),
        Ok(()),
        "tier 3 (F1 pin)"
    );
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
        Ok(()),
        "tier 3′"
    );
    // F2/F3: the skip is visible with faces + the REAL diagnostics.
    assert!(
        !bb.naming.merge_skipped.is_empty(),
        "the licensed-then-skipped cap groups must be recorded"
    );
    for skip in &bb.naming.merge_skipped {
        assert!(!skip.faces.is_empty(), "skip records name their faces");
        assert!(
            skip.reason.contains("never-elide") || skip.reason.contains("refused:"),
            "skip reason carries the actual diagnostics, got {:?}",
            skip.reason
        );
        // Every recorded face is live in the result (the record
        // describes the shipped body, not a dead intermediate).
        for f in &skip.faces {
            assert!(bb.body.get_face(*f).is_some(), "skip face {f:?} live");
        }
    }
    // No cap group glued (merge_groups may still carry OTHER glue;
    // here there is none).
    assert!(bb.naming.merge_groups.is_empty());
}
