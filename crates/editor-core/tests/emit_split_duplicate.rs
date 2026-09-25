//! **A split tangent to a declared union's edge refuses as a degenerate
//! section, never as a duplicate name.**
//!
//! `a` and `b` meet flush along x, declared on all four families; `g`
//! is a slab at x = 1.2..1.3 that the plane y + z = 2 really cuts. The
//! plane touches the union along its top/far rim y = z = 1. The
//! split's direct run refuses that contact on area, as it refuses the
//! contact standing alone (the controls below); its pinch lane then
//! reran under the mirrored plane, where the contact joined the slab's
//! section as a spur and the run SUCCEEDED — the Below half carried two
//! copies of each vertex along the rim, and the split's names collided
//! (`Naming(Duplicate)`). The join refuses the spur now, so the direct
//! run's `DegenerateSection` is what the split reports.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{insert, len, scl};
use editor_core::{Node, NodeErrorKind, ProfileDoc, RecipeNodeId, SitedRef};
use geom_core::Tol;
use topo::{SplitError, SplitJoinError};

fn split_of(doc: ProfileDoc, target: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(1.0), len(1.0)],
            normal: [scl(0.0), scl(h), scl(h)],
        }),
    );
    insert(
        doc,
        Node::Split {
            target,
            tool: plane,
        },
    )
}

/// The target fuses and the split refuses `DegenerateSection`.
fn refuses_degenerate(label: &str, doc: &ProfileDoc, target: RecipeNodeId, split: RecipeNodeId) {
    let ev = run(doc);
    assert!(
        failure(&ev, target).is_none(),
        "{label}: the target refused: {:?}",
        failure(&ev, target)
    );
    let got = failure(&ev, split);
    assert!(
        matches!(
            got,
            Some(NodeErrorKind::Split(SplitError::Join(
                SplitJoinError::DegenerateSection { .. }
            )))
        ),
        "{label}: {got:?}"
    );
}

#[test]
fn a_tangent_split_of_a_fused_declared_union_refuses_degenerate_not_duplicate() {
    let doc = ProfileDoc::empty_derived("emit_split_duplicate", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block(doc, (1.2, 1.3), (-1.0, 2.0), 0.5, 2.5);
    for (label, order) in [("[a, g, b]", [a, g, b]), ("[g, a, b]", [g, a, b])] {
        let (d, u, _) = declared_union(doc.clone(), &order, flush_pairs((a, a), (b, b)));
        let (d, s) = split_of(d, u);
        refuses_degenerate(label, &d, u, s);
    }
}

/// The same plane against the contact on its own: `a`, `a ∪ b` in
/// both orders, and `a` with a slab away from the rim's section.
#[test]
fn the_tangent_contact_standing_alone_refuses_degenerate() {
    let doc = ProfileDoc::empty_derived("emit_split_duplicate", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g0) = block(doc, (0.2, 0.3), (-1.0, 2.0), 0.5, 2.5);
    let (d, s) = split_of(doc.clone(), a);
    refuses_degenerate("a", &d, a, s);
    for (label, order) in [("[a, b]", vec![a, b]), ("[b, a]", vec![b, a])] {
        let (d, u, _) = declared_union(doc.clone(), &order, flush_pairs((a, a), (b, b)));
        let (d, s) = split_of(d, u);
        refuses_degenerate(label, &d, u, s);
    }
    let (d, u, _) = declared_union(doc, &[a, g0], Vec::<(SitedRef, SitedRef)>::new());
    let (d, s) = split_of(d, u);
    refuses_degenerate("[a, g0]", &d, u, s);
}
