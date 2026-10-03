//! **A split tangent to a declared union's edge classifies the contact
//! with its material, and names both halves.**
//!
//! `a` and `b` meet flush along x, declared on all four families; `g`
//! is a slab at x = 1.2..1.3 that the plane y + z = 2 really cuts. The
//! plane touches the union along its top/far rim y = z = 1, a convex
//! edge whose material is all below. The rim stays an ordinary edge of
//! the below half and adds nothing to the section, so the split cuts
//! only the slab, and its halves name without a collision. Each control
//! is the contact on its own: the whole target lands below and the
//! above half is empty.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{insert, len, scl};
use editor_core::{Evaluation, Node, ProfileDoc, RecipeNodeId, SitedRef, SplitSide, ValuePayload};
use geom_core::Tol;

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

/// The volume of one half, `None` for an empty one.
fn volume(side: &SplitSide<f64>) -> Option<f64> {
    match side {
        SplitSide::Body(b) => Some(topo::mass_properties(b, Tol::witness()).unwrap().volume),
        SplitSide::Empty => None,
    }
}

/// The target and the split both evaluate; the halves' volumes.
fn halves(
    label: &str,
    ev: &Evaluation<f64>,
    target: RecipeNodeId,
    split: RecipeNodeId,
) -> (Option<f64>, Option<f64>) {
    for id in [target, split] {
        assert!(failure(ev, id).is_none(), "{label}: {:?}", failure(ev, id));
    }
    let ValuePayload::Split { above, below } = &ev.value(split).expect("the split").payload else {
        panic!("{label}: a split value");
    };
    (volume(above), volume(below))
}

fn near(got: Option<f64>, want: f64) -> bool {
    got.is_some_and(|v| (v - want).abs() <= 1e-12 * want)
}

/// `g` spans z = 0.5..3. Its part above the plane is 0.1 × 4.375; the
/// rest of the union, 2.2 − 0.4375, is below.
#[test]
fn a_tangent_split_of_a_fused_declared_union_cuts_only_the_slab() {
    let doc = ProfileDoc::empty_derived("emit_split_duplicate", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block(doc, (1.2, 1.3), (-1.0, 2.0), 0.5, 2.5);
    for (label, order) in [("[a, g, b]", [a, g, b]), ("[g, a, b]", [g, a, b])] {
        let (d, u) = declared_union(doc.clone(), &order, flush_pairs(&doc, (a, a), (b, b)));
        let (d, s) = split_of(d, u);
        let (above, below) = halves(label, &run(&d), u, s);
        assert!(near(above, 0.4375), "{label}: above {above:?}");
        assert!(near(below, 1.7625), "{label}: below {below:?}");
    }
}

/// The same plane against the contact on its own: `a`, `a ∪ b` in
/// both orders, and `a` with a slab away from the rim's section.
#[test]
fn the_tangent_contact_standing_alone_lands_below_whole() {
    let doc = ProfileDoc::empty_derived("emit_split_duplicate", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g0) = block(doc, (0.2, 0.3), (-1.0, 2.0), 0.5, 2.5);
    let (d, s) = split_of(doc.clone(), a);
    let (above, below) = halves("a", &run(&d), a, s);
    assert_eq!(above, None, "a");
    assert!(near(below, 1.0), "a: below {below:?}");
    for (label, order) in [("[a, b]", vec![a, b]), ("[b, a]", vec![b, a])] {
        let (d, u) = declared_union(doc.clone(), &order, flush_pairs(&doc, (a, a), (b, b)));
        let (d, s) = split_of(d, u);
        let (above, below) = halves(label, &run(&d), u, s);
        assert_eq!(above, None, "{label}");
        assert!(near(below, 1.5), "{label}: below {below:?}");
    }
    let (d, u) = declared_union(doc, &[a, g0], Vec::<(SitedRef, SitedRef)>::new());
    let (d, s) = split_of(d, u);
    let (above, below) = halves("[a, g0]", &run(&d), u, s);
    assert!(near(above, 0.4375), "[a, g0]: above {above:?}");
    assert!(near(below, 1.2625), "[a, g0]: below {below:?}");
}
