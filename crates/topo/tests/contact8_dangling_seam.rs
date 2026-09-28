//! **A declared area-overlap cap contact merges.** Two blocks whose
//! coplanar caps are declared in contact and overlap in AREA meet
//! along a seam that bends at the overlap's corner. The merge output
//! stage joins each cap pair, deletes the seam edge the glue leaves
//! dangling together with its free end, and publishes one cap top and
//! bottom, so the result is a legal operand of the next boolean.
//!
//! The rows run the reproducer end to end (`a = [0,1]³`,
//! `f = [0.5,1.5]² × [0,1]`, then a third brick `c`), a seam that bends
//! twice around a hole, and the step refusal a planar group the merge
//! cannot glue earns.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, describe_as_intersections, flush_declarations, holed_block, prism_z};
use geom_core::Tol;
use topo::validate::{validate_closed, validate_geometric};
use topo::{
    Body, BooleanBody, BooleanDeclarations, BooleanError, BooleanResult, LoopBoundary,
    MergeCoplanarError, mass_properties, union_with, validate_pseudomanifold,
};

fn unwrap_body(r: BooleanResult<f64>) -> BooleanBody<f64> {
    let BooleanResult::Body(bb) = r else {
        panic!("an overlapping union cannot be Empty");
    };
    bb
}

/// The faces of `b` whose every outer-loop vertex lies at height `z`.
fn faces_at_height(b: &Body<f64>, z: f64) -> usize {
    b.faces()
        .filter(|(_, face)| {
            let LoopBoundary::Cycle { first } = b.get_loop(face.outer).unwrap().boundary else {
                return false;
            };
            b.loop_cycle(first).unwrap().into_iter().all(|he| {
                let v = b.get_half_edge(he).unwrap().start;
                b.get_point(b.get_vertex(v).unwrap().point).unwrap().z == z
            })
        })
        .count()
}

/// Tier 2, tier 3 and tier 3′ with the op's own contacts.
fn assert_green(bb: &BooleanBody<f64>, what: &str) {
    let tol = Tol::witness();
    assert_eq!(validate_closed(&bb.body), Ok(()), "{what}: tier 2");
    assert_eq!(validate_geometric(&bb.body, tol), Ok(()), "{what}: tier 3");
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts, tol),
        Ok(()),
        "{what}: tier 3′"
    );
}

/// `a ∪ f`, caps declared: each cap pair meets along a two-edge seam
/// bent 90° at (1, 1).
fn a_union_f() -> BooleanBody<f64> {
    let tol = Tol::witness();
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let f = brick::<f64>((0.5, 1.5), (0.5, 1.5), (0.0, 1.0), tol);
    let decls = flush_declarations(&a, &f, tol);
    assert_eq!(decls.coincident_faces.len(), 2, "both caps declared");
    unwrap_body(union_with(&a, &f, &decls, tol).expect("the declared area-overlap union runs"))
}

/// The third brick, touching both blocks.
fn c() -> Body<f64> {
    brick::<f64>((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), Tol::witness())
}

/// **The two cap pairs merge**: two `Merged` rows, one cap at each
/// height, nothing skipped, volume `1 + 1 − 0.25`.
#[test]
fn an_area_overlap_cap_contact_publishes_one_cap_top_and_bottom() {
    let bb = a_union_f();
    assert_eq!(
        bb.naming.merge_groups.len(),
        2,
        "{:?}",
        bb.naming.merge_groups
    );
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{:?}",
        bb.naming.merge_skipped
    );
    assert_eq!(faces_at_height(&bb.body, 1.0), 1, "one top cap");
    assert_eq!(faces_at_height(&bb.body, 0.0), 1, "one bottom cap");
    // Two octagonal caps and eight walls.
    assert_eq!(bb.body.faces().count(), 10);
    assert_eq!(
        mass_properties(&bb.body, Tol::witness()).unwrap().volume,
        1.75
    );
    assert_green(&bb, "a ∪ f");
}

/// **The merged union is a legal operand**: the next union, with its
/// own contacts declared, runs and skips nothing. The finished body is
/// the L-shaped hexagon's prism: six walls and two caps.
#[test]
fn the_merged_union_takes_a_third_brick_declared() {
    let tol = Tol::witness();
    let af = a_union_f();
    let c = c();
    let decls = flush_declarations(&af.body, &c, tol);
    let bb = unwrap_body(
        union_with(&af.body, &c, &decls, tol).expect("a merged union is a boolean operand"),
    );
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{:?}",
        bb.naming.merge_skipped
    );
    assert_eq!(bb.body.faces().count(), 8);
    assert_eq!(faces_at_height(&bb.body, 1.0), 1, "one top cap");
    assert_eq!(faces_at_height(&bb.body, 0.0), 1, "one bottom cap");
    assert_eq!(mass_properties(&bb.body, tol).unwrap().volume, 2.0);
    assert_green(&bb, "(a ∪ f) ∪ c");
}

/// **Without its declarations the next union refuses on a pair it
/// really has**: an undeclared coincidence ACROSS the operands — not a
/// same-operand pair left by the merge, which no declaration could
/// cover.
#[test]
fn the_merged_union_refuses_an_undeclared_third_brick_across_operands() {
    let tol = Tol::witness();
    let af = a_union_f();
    let err = union_with(&af.body, &c(), &BooleanDeclarations::none(), tol)
        .expect_err("c's flush faces are undeclared");
    let BooleanError::UndeclaredCoincidence { pair, .. } = &err else {
        panic!("expected an undeclared coincidence, got {err:?}");
    };
    assert_ne!(pair[0].0, pair[1].0, "a cross-operand pair: {err:?}");
}

/// **No contact record cites a deleted free end.** The pruning deletes
/// the corner vertex of each cap's bent seam; a record citing it is
/// consumed and drops under the strict rule, so every record the
/// result carries resolves, and tier 3′ reads them green
/// (`assert_green`). The rule itself, on a record that does cite a
/// pruned vertex, is `boolean::ops`' `a_record_citing_a_pruned_free_end_drops`.
#[test]
fn every_carried_record_resolves_after_the_pruning() {
    let bb = a_union_f();
    let live = |v| bb.body.get_vertex(v).is_some();
    let face = |f| bb.body.get_face(f).is_some();
    for c in &bb.contacts.vv {
        assert!(live(c.a) && live(c.b), "{c:?}");
    }
    for c in bb.contacts.a_on_b.iter().chain(&bb.contacts.b_on_a) {
        assert!(live(c.vertex) && face(c.face), "{c:?}");
    }
    // The corner the pruning deleted is no vertex of the result.
    let at_corner = bb.body.vertices().any(|(_, v)| {
        let p = bb.body.get_point(v.point).unwrap();
        (p.x, p.y) == (1.0, 1.0)
    });
    assert!(!at_corner, "the bent seam's corner went with its seam");
}

/// **A seam that bends around a hole.** A U-shaped block whose opening
/// is closed by a bar overlapping both arms in area: each cap pair
/// shares two bent seam chains, one from the outline to the hole on
/// each side. One chain's corner dangles once the faces are joined and
/// is pruned; the other chain has no free end, so it separates the
/// hole — `kemr` mints the ring — and its own corner then dangles and
/// is pruned in turn. Each cap is one face with one ring.
#[test]
fn a_bent_seam_around_a_hole_merges_to_one_ringed_cap() {
    let tol = Tol::witness();
    let u = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
        tol,
    )
    .body;
    let bar = brick::<f64>((0.5, 2.5), (2.0, 3.5), (0.0, 1.0), tol);
    let decls = flush_declarations(&u, &bar, tol);
    let bb = unwrap_body(union_with(&u, &bar, &decls, tol).expect("the declared union runs"));
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{:?}",
        bb.naming.merge_skipped
    );
    let ringed: Vec<usize> = bb
        .body
        .faces()
        .map(|(_, face)| face.rings.len())
        .filter(|&n| n > 0)
        .collect();
    assert_eq!(ringed, [1, 1], "each cap carries the hole as one ring");
    assert_eq!(faces_at_height(&bb.body, 1.0), 1, "one top cap");
    assert_eq!(faces_at_height(&bb.body, 0.0), 1, "one bottom cap");
    // 9 − 2 + 3 − 1: the U, the bar, their two corner squares.
    assert_eq!(mass_properties(&bb.body, tol).unwrap().volume, 9.0);
    assert_green(&bb, "U ∪ bar");
}

/// **A boolean refuses a planar group it was licensed to merge and
/// cannot glue**, with the merge's own reason, rather than ship two
/// coplanar caps the next boolean refuses. A through-hole plugged
/// exactly, every flush pair declared: each cap and the plug's cap
/// join, and the hole's four rim edges are a closed doubled cycle.
/// The pruning takes three of them and leaves the fourth with both
/// ends free — an isolated segment whose deletion leaves an empty ring
/// the Euler inventory has no operator to remove.
#[test]
fn a_planar_group_the_merge_cannot_glue_refuses_the_step() {
    let tol = Tol::witness();
    let mut block = holed_block::<f64>(3.0, &[1.5], tol);
    describe_as_intersections(&mut block, tol);
    let plug = brick::<f64>((1.0, 2.0), (0.5, 1.5), (0.0, 2.0), tol);
    let decls = flush_declarations(&block, &plug, tol);
    let err = union_with(&block, &plug, &decls, tol)
        .expect_err("a planar group the merge cannot glue refuses the step");
    println!("[plug] {err}");
    let BooleanError::Merge(reason) = &err else {
        panic!("expected the merge stage's refusal, got {err:?}");
    };
    assert!(
        matches!(reason, MergeCoplanarError::ResultNotClosed { .. }),
        "{reason:?}"
    );
}
