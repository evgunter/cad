//! The declared-merge L-corner (M4 PR 5 review F1/F2's probe): flush
//! caps DECLARED on an otherwise ordinary partial overlap (walls
//! offset). Each cap group is licensed by its declaration, and its two
//! faces share a seam that bends 90° at the overlap's re-entrant
//! corner. The merge joins them, and the seam edge the glue leaves
//! dangling goes together with its free end — the corner — at any
//! angle, so both caps merge and nothing is skipped. The result must
//! be FULLY honest:
//!
//! - tier 2, tier 3 and tier 3′ green: the merged caps' in-plane cut
//!   edges are described against the ACTUAL adjacency (no stale
//!   `Intersection`/`Seam` rows citing no-longer-adjacent surfaces);
//! - the merge is VISIBLE: `BooleanNaming::merge_groups` carries both
//!   cap groups and `merge_skipped` is empty — a planar group the
//!   merge could not glue would refuse the step, never ship.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, flush_declarations};
use geom_core::Tol;
use topo::validate::{validate_closed, validate_geometric};
use topo::{BooleanResult, mass_properties, union_with, validate_pseudomanifold};

/// The probe: a = [0,1]³, b = [0.5,1.5]×[0.25,1.25]×[0,1] — caps flush
/// (declared), walls offset (transversal). Exact dyadic volume:
/// 1 + 1 − (0.5 · 0.75 · 1).
#[test]
fn declared_l_corner_caps_merge_and_stay_tier3_green() {
    let tol = Tol::witness();
    let a = finished(
        "a",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let b = finished(
        "b",
        brick::<f64>((0.5, 1.5), (0.25, 1.25), (0.0, 1.0), tol),
        tol,
    );
    let decls = flush_declarations(&a, &b, Tol::witness());
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
    assert_eq!(
        validate_geometric(&bb.body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
        Ok(()),
        "tier 3′"
    );
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "both licensed cap groups glue: {:?}",
        bb.naming.merge_skipped
    );
    // Both caps merged: one group per cap, each absorbing b's cap
    // remainder into a's cap.
    assert_eq!(
        bb.naming.merge_groups.len(),
        2,
        "{:?}",
        bb.naming.merge_groups
    );
    for (kept, absorbed) in &bb.naming.merge_groups {
        assert!(bb.body.get_face(*kept).is_some(), "kept face {kept:?} live");
        assert_eq!(absorbed.len(), 1, "{absorbed:?}");
    }
    // Two octagonal caps and eight walls.
    assert_eq!(bb.body.faces().count(), 10);
}
