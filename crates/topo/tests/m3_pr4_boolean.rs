//! M3 PR 4 acceptance: boolean reduction + classification fixtures
//! (M3-PLAN PR item 4; censuses hand-traced in the PR derivation).
//! Every scenario is generic over `T` and runs at f64 (all ε rows via
//! CI) and on the interval lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{flush_declarations, prism_z};
use geom_core::Decide;
use geom_core::Tol;
use topo::test_support::arena_counts;
use topo::{
    Body, BooleanError, BooleanOp, BooleanReduction, boolean_reduce, boolean_reduce_declared,
    validate,
};

fn brick<T: Decide + geom_core::Bounds>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn reduce_ok<T: Decide + geom_core::Bounds>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
) -> BooleanReduction<T> {
    let before = (arena_counts(a), arena_counts(b));
    // M4 PR 5: intended flush contacts are DECLARED (the test author's
    // recipe intent); value-equality alone no longer classifies.
    let red = boolean_reduce_declared(op, a, b, &flush_declarations(a, b), Tol::witness()).unwrap();
    // Operands functionally untouched: every topology arena, not a
    // three-component sample of them.
    assert_eq!((arena_counts(a), arena_counts(b)), before);
    // Annotated clones stay tier-1 valid through every transient.
    validate(&red.a).unwrap();
    validate(&red.b).unwrap();
    red
}

/// Acceptance (1): the canonical two-brick corner overlap (TOG
/// Fig. 15.4 analogue). Hand-traced census: 3 A-edge pierces into B, 3
/// B-edge pierces into A, no v-v; each pierce mints one piercing-side
/// null edge + one ring strut in the pierced face (the vtxfacclassify
/// ring sequence), all correspondence-keyed. Op-independent here (no
/// Eq. 15.3 row is hit).
fn two_bricks<T: Decide + geom_core::Bounds>(op: BooleanOp) {
    let a = brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<T>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
    let red = reduce_ok(op, &a, &b);
    assert_eq!(red.contacts.vv.len(), 0);
    assert_eq!(red.contacts.a_on_b.len(), 3);
    assert_eq!(red.contacts.b_on_a.len(), 3);
    assert_eq!(red.null_pairs.len(), 6);
    assert_eq!(red.null_edges.len(), 12);
    assert_eq!(red.pierce_rings.len(), 6);
    let struts = red.null_edges.iter().filter(|e| e.dangling).count();
    assert_eq!(struts, 6); // the six pierced-face ring struts
    // Determinism (D9): bitwise-identical record dumps on replay.
    let red2 = reduce_ok(op, &a, &b);
    assert_eq!(
        format!("{:?}", red.contacts),
        format!("{:?}", red2.contacts)
    );
    assert_eq!(
        format!("{:?}", red.null_pairs),
        format!("{:?}", red2.null_pairs)
    );
}

#[test]
fn two_bricks_all_ops() {
    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        two_bricks::<f64>(op);
    }
}

/// Acceptance (2): Fig. 15.1-style coplanar overlap — B stacked on A
/// sharing the full plane z = 2 with identical footprints: four
/// corner-to-corner v-v contacts, every A-top edge collinear with a
/// B-bottom edge (Tables II/III rows live), Eq. 15.3's ⁻ row decides
/// (opposite orientation). Union sees crossings (the stacked bodies
/// merge through the shared plane); the census is pinned per op.
fn stacked_bricks<T: Decide + geom_core::Bounds>(op: BooleanOp, expect_pairs_nonzero: bool) {
    let a = brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<T>((0.0, 2.0), (0.0, 2.0), (2.0, 4.0));
    let red = reduce_ok(op, &a, &b);
    assert_eq!(red.contacts.vv.len(), 4);
    assert_eq!(red.contacts.a_on_b.len(), 0);
    assert_eq!(red.contacts.b_on_a.len(), 0);
    assert_eq!(
        !red.null_pairs.is_empty(),
        expect_pairs_nonzero,
        "op {op:?}: pairs {:?}",
        red.null_pairs.len()
    );
}

#[test]
fn stacked_bricks_full_coplanar_face() {
    // ⁻ row: ∪ lumps both sides In (the shared wall dissolves — every
    // corner is a genuine crossing of the merged boundary); ∩ and ∖
    // lump both Out (the shared wall is the entire interaction — the
    // sectors cancel; contact only).
    stacked_bricks::<f64>(BooleanOp::Union, true);
    stacked_bricks::<f64>(BooleanOp::Intersect, false);
    stacked_bricks::<f64>(BooleanOp::Subtract, false);
}

/// Acceptance (3): vertex-vertex corner kiss — the sector intersection
/// search finds NO crossing (the cones touch at one point); the
/// declared v-v contact is the entire result. Near-miss variants: a gap
/// inside the sliver band escalates typed; a definite gap is clean.
fn corner_kiss<T: Decide + geom_core::Bounds>() {
    let a = brick::<T>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<T>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        let red = reduce_ok(op, &a, &b);
        assert_eq!(red.contacts.vv.len(), 1);
        assert!(red.null_pairs.is_empty());
        assert!(red.null_edges.is_empty());
    }
}

#[test]
fn corner_kiss_touch_and_near_miss() {
    corner_kiss::<f64>();
    let eps = geom_core::Tol::witness().get().eps;
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    // In-band gap (3ε with K = 10): a genuine sliver — typed
    // escalation, never a silent contact and never a silent miss (F6).
    let g = 1.0 + 3.0 * eps;
    let b = brick::<f64>((g, 2.0), (g, 2.0), (g, 2.0));
    let err = boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness()).unwrap_err();
    assert!(
        matches!(
            err,
            BooleanError::Escalated { .. } | BooleanError::UndeclaredCoincidence { .. }
        ),
        "{err:?}"
    );
    // Definite gap (1000ε): clean miss, no contacts at all.
    let g = 1.0 + 1000.0 * eps;
    let b = brick::<f64>((g, 2.0), (g, 2.0), (g, 2.0));
    let red = boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness()).unwrap();
    assert!(red.contacts.vv.is_empty());
    assert!(red.contacts.a_on_b.is_empty());
    assert!(red.null_edges.is_empty());
}

/// Acceptance (4): skew touching edges — an A-edge crossing a B-edge at
/// an interior point of both (the OnEdge lane splits BOTH edges into a
/// declared v-v pair). Census hand-traced: two such crossings, plus one
/// vertex-on-face contact per side in the shared tangent plane z = 2.
fn skew_edge_cross<T: Decide + geom_core::Bounds>(op: BooleanOp) -> BooleanReduction<T> {
    let a = brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<T>((1.5, 3.5), (0.5, 2.5), (2.0, 4.0));
    let red = reduce_ok(op, &a, &b);
    assert_eq!(red.contacts.vv.len(), 2, "op {op:?}");
    assert_eq!(red.contacts.a_on_b.len(), 1);
    assert_eq!(red.contacts.b_on_a.len(), 1);
    red
}

#[test]
fn skew_edge_cross_all_ops() {
    // The bodies touch along the plane z = 2 only; for ∩ and ∖ the
    // Eq. 15.3 ⁻ row classifies every touching sector Out ⇒ no
    // surgery; for ∪ the coplanar contacts are genuine boundary
    // crossings of the merged solid.
    let red = skew_edge_cross::<f64>(BooleanOp::Union);
    assert!(!red.null_pairs.is_empty());
    let red = skew_edge_cross::<f64>(BooleanOp::Intersect);
    assert!(red.null_pairs.is_empty());
    let red = skew_edge_cross::<f64>(BooleanOp::Subtract);
    assert!(red.null_pairs.is_empty());
}

/// Acceptance (5): vertex-on-face — tangential rest (a brick standing
/// with one corner vertex on the interior of a bigger brick's top
/// face... realized as a small brick standing ON the plane through a
/// corner contact would be face-on-face; the honest prismatic
/// tangential v-on-f is the corner of B resting ON A's top face plane
/// with B entirely above: contact recorded, NO surgery). The piercing
/// variant of vertex-on-face (ring insertion, tier 1 through the
/// transient) is exercised six-fold by `two_bricks_all_ops` — every
/// pierce vertex is a v-on-f contact classified through
/// `vtxfacclassify` with the mev→kemr→ring-strut sequence, and
/// `reduce_ok` validates tier 1 on both annotated bodies.
#[test]
fn vertex_on_face_tangential_rest() {
    // B's corner (1,1,2) rests on A's top face z=2 interior.
    let a = brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 2.0));
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (2.0, 4.0));
    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        let red = reduce_ok(op, &a, &b);
        // All four bottom corners of B rest on A's face.
        assert_eq!(red.contacts.b_on_a.len(), 4, "op {op:?}");
        assert_eq!(red.contacts.vv.len(), 0);
        // ∩/∖: tangential — no surgery. ∪: the resting wall is a
        // boundary crossing of the merged solid (⁻ row lumps In).
        if matches!(op, BooleanOp::Intersect | BooleanOp::Subtract) {
            assert!(red.null_edges.is_empty(), "op {op:?}");
            assert!(red.pierce_rings.is_empty());
        } else {
            assert_eq!(red.pierce_rings.len(), 4);
        }
    }
}

/// Acceptance (6): edge-on-edge collinear overlap — B stacked on A with
/// a partial x-overlap: two of A's top edges cross B-bottom edges at
/// interior points DISCOVERED VIA NONCOPLANAR NEIGHBOR FACES (the
/// coplanar edge-face pair itself is skipped — the documented catch),
/// and the shared plane's collinear edge segments put the Tables II/III
/// edge-edge machinery live at every minted v-v pair.
fn collinear_overlap<T: Decide + geom_core::Bounds>(op: BooleanOp) -> BooleanReduction<T> {
    let a = brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<T>((1.0, 3.0), (0.0, 2.0), (2.0, 4.0));
    reduce_ok(op, &a, &b)
}

#[test]
fn collinear_edge_overlap() {
    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        let red = collinear_overlap::<f64>(op);
        // Hand census: A top corners (2,0,2),(2,2,2) lie ON B-bottom
        // edges (split → v-v); B bottom corners (1,0,2),(1,2,2) lie ON
        // A-top edges (split → v-v). The y-running edges at x ∈ [1,2]
        // are collinear overlaps bounded by those pairs.
        assert_eq!(red.contacts.vv.len(), 4, "op {op:?}");
        assert!(red.contacts.a_on_b.is_empty());
        assert!(red.contacts.b_on_a.is_empty());
        if matches!(op, BooleanOp::Union) {
            assert!(!red.null_pairs.is_empty());
        } else {
            assert!(red.null_pairs.is_empty(), "op {op:?}");
        }
    }
}

/// F5 gate: a curved operand refuses typed.
#[test]
fn curved_operand_refuses() {
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let mut b = brick::<f64>((2.0, 3.0), (0.0, 1.0), (0.0, 1.0));
    let cube = common::geometric_cube::<f64>();
    // A genuinely curved body is not in the prismatic corpus; instead
    // gate on scaffolding: a mid-surgery operand refuses.
    let _ = cube;
    let he = b.vertices().next().and_then(|(_, v)| v.emanating).unwrap();
    b.mev_null(
        topo::MevSite::Fan { he1: he, he2: he },
        topo::NewVertexSide::Above,
    )
    .unwrap();
    let err = boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, BooleanError::ScaffoldingOperand { .. }),
        "{err:?}"
    );
}

/// F7 gate: a non-maximal operand (declared-coplanar adjacent faces)
/// refuses typed. Built by splitting a brick face with a real edge
/// between two same-plane faces (mef through the middle of the top
/// face with the same plane description).
#[test]
fn non_maximal_operand_refuses() {
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let p = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)], 0.0, 1.0);
    let mut b = p.body;
    // Split the top face by a chord between the two top rim vertices
    // above (0,0) and... use mev+mef with FaceSurface::Same to make an
    // adjacent same-key coplanar pair.
    let top = p.top_face;
    let outer = b.get_face(top).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        panic!()
    };
    let cycle = b.loop_cycle(first).unwrap();
    let he1 = cycle[0];
    let he2 = cycle[2];
    let p0 = *b
        .get_point(
            b.get_vertex(b.get_half_edge(he1).unwrap().start)
                .unwrap()
                .point,
        )
        .unwrap();
    let p1 = *b
        .get_point(
            b.get_vertex(b.get_half_edge(he2).unwrap().start)
                .unwrap()
                .point,
        )
        .unwrap();
    b.mef(
        topo::MefSite::Chords { he1, he2 },
        common::line(p0, p1),
        topo::FaceSurface::Inherit,
        Tol::witness(),
    )
    .unwrap();
    let err = boolean_reduce(BooleanOp::Union, &a, &b, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, BooleanError::NonMaximalFaces { .. }),
        "{err:?}"
    );
}

// ---- Interval lane (the same scenarios at T = Interval). ----
#[cfg(feature = "interval")]
mod interval {
    use super::*;
    use geom_core::Interval;

    #[test]
    fn two_bricks_interval() {
        two_bricks::<Interval>(BooleanOp::Union);
    }

    #[test]
    fn corner_kiss_interval() {
        corner_kiss::<Interval>();
    }

    #[test]
    fn stacked_interval() {
        stacked_bricks::<Interval>(BooleanOp::Intersect, false);
    }

    #[test]
    fn skew_interval() {
        let _ = skew_edge_cross::<Interval>(BooleanOp::Subtract);
    }
}
