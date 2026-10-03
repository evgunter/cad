//! **A declared area-overlap cap contact merges.** Two blocks whose
//! coplanar caps are declared in contact and overlap in AREA meet
//! along a seam that bends at the overlap's corner. The merge output
//! stage joins each cap pair, deletes the seam edge the glue leaves
//! dangling together with its free end, and publishes one cap top and
//! bottom, so the result is a legal operand of the next boolean.
//!
//! The rows run the reproducer end to end (`a = [0,1]³`,
//! `f = [0.5,1.5]² × [0,1]`, then a third brick `c`), a seam that bends
//! twice around a hole, and a through-hole plugged three ways, whose
//! rim is a doubled cycle the pruning takes down to its last edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, describe_as_intersections, flush_declarations, holed_block, prism_z};
use geom_core::Tol;
use topo::validate::{validate_closed, validate_geometric};
use topo::{
    Body, BooleanBody, BooleanDeclarations, BooleanError, BooleanResult, LoopBoundary, Operand,
    mass_properties, union_with, validate_pseudomanifold,
};

fn unwrap_body(r: BooleanResult<f64>) -> BooleanBody<f64> {
    let BooleanResult::Body(bb) = r else {
        panic!("an overlapping union cannot be Empty");
    };
    bb
}

/// The height every outer-loop vertex of `f` lies at, if they share one.
fn face_height(b: &Body<f64>, f: topo::FaceKey) -> Option<f64> {
    let face = b.get_face(f)?;
    let LoopBoundary::Cycle { first } = b.get_loop(face.outer)?.boundary else {
        return None;
    };
    let zs: Vec<f64> = b
        .loop_cycle(first)?
        .into_iter()
        .map(|he| {
            let v = b.get_half_edge(he).unwrap().start;
            b.get_point(b.get_vertex(v).unwrap().point).unwrap().z
        })
        .collect();
    zs.iter().all(|&z| z == zs[0]).then_some(zs[0])
}

/// The faces of `b` whose every outer-loop vertex lies at height `z`.
fn faces_at_height(b: &Body<f64>, z: f64) -> usize {
    b.faces()
        .filter(|&(f, _)| face_height(b, f) == Some(z))
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
    // The refused pair is exactly one the declarations would have
    // covered: A's face and c's face, in that order, a flush pair.
    let [(Operand::A, fa), (Operand::B, fb)] = *pair else {
        panic!("a cross-operand (A, B) pair: {err:?}");
    };
    let flush = flush_declarations(&af.body, &c(), tol);
    assert!(
        flush
            .coincident_faces
            .iter()
            .any(|d| (d.a, d.b) == (fa, fb)),
        "({fa:?}, {fb:?}) is a flush pair of the two operands: {:?}",
        flush.coincident_faces
    );
    // ...and it is the two TOP caps: the merged union's octagonal top
    // and c's top, the first continuation the reduction's scan meets in
    // arena order.
    assert_eq!(face_height(&af.body, fa), Some(1.0), "{err:?}");
    assert_eq!(face_height(&c(), fb), Some(1.0), "{err:?}");
}

/// **The bent seam's corner is deleted, and no record survives to
/// cite it.** The union ships no contact records at all: every
/// reduction record here rests at a seam vertex the zip fused, and is
/// consumed there, before the merge runs. That is why the drop rule
/// for a pruned vertex is pinned on the merge's real outcome in
/// `boolean::ops`' `a_record_citing_a_pruned_free_end_drops` rather
/// than here.
#[test]
fn the_bent_seams_corner_is_deleted_and_no_record_survives() {
    let bb = a_union_f();
    assert_eq!(bb.contacts, Default::default(), "{:?}", bb.contacts);
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

/// A block with a unit-square through-hole, `[0,3]×[0,2]×[0,2]` less
/// `[1,2]×[0.5,1.5]`, described.
fn holed() -> Body<f64> {
    let tol = Tol::witness();
    let mut block = holed_block::<f64>(3.0, &[1.5], tol);
    describe_as_intersections(&mut block, tol);
    block
}

/// `holed() ∪ plug`, every flush pair declared, then a next union with
/// a brick flush on the block's `x = 3` wall: both must run, publish
/// no skip and stay green, at the volumes given.
fn assert_plug_merges(plug: Body<f64>, volume: f64, what: &str) {
    let tol = Tol::witness();
    let block = holed();
    let decls = flush_declarations(&block, &plug, tol);
    let bb = unwrap_body(
        union_with(&block, &plug, &decls, tol)
            .unwrap_or_else(|e| panic!("{what}: the plugged block merges, got {e:?}")),
    );
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{what}: {:?}",
        bb.naming.merge_skipped
    );
    assert_eq!(
        mass_properties(&bb.body, tol).unwrap().volume,
        volume,
        "{what}"
    );
    assert_green(&bb, what);
    println!(
        "[plug] {what}: faces {} merge_groups {:?}",
        bb.body.faces().count(),
        bb.naming.merge_groups
    );
    let next = brick::<f64>((3.0, 4.0), (0.0, 2.0), (0.0, 2.0), tol);
    let decls = flush_declarations(&bb.body, &next, tol);
    let nb = unwrap_body(
        union_with(&bb.body, &next, &decls, tol)
            .unwrap_or_else(|e| panic!("{what}: the result is an operand, got {e:?}")),
    );
    assert!(nb.naming.merge_skipped.is_empty(), "{what}: next union");
    assert_eq!(
        mass_properties(&nb.body, tol).unwrap().volume,
        volume + 4.0,
        "{what}: next union"
    );
    assert_green(&nb, &format!("{what}, next union"));
}

/// **A hole plugged exactly merges into whole caps.** Each cap and the
/// plug's cap join; the hole's rim edges are a closed doubled cycle
/// with no free end, so `kemr` cuts it off as a ring, the pruning then
/// takes its edges one free end at a time, and the last — both ends
/// free — goes with its lone vertex and the ring. The block is whole.
#[test]
fn an_exactly_plugged_hole_merges_to_whole_caps() {
    let plug = brick::<f64>((1.0, 2.0), (0.5, 1.5), (0.0, 2.0), Tol::witness());
    let block = holed();
    let decls = flush_declarations(&block, &plug, Tol::witness());
    let join = topo::test_support::boolean_join_refusal(
        topo::BooleanOp::Union,
        &block,
        &plug,
        &decls,
        Tol::witness(),
    );
    assert!(
        matches!(join, Ok(None)),
        "the join builds the exact plug, not the declared-REST zip: got {join:?}"
    );
    assert_plug_merges(plug, 12.0, "exact plug");
}

/// **A plug filling the hole's bottom half** merges the bottom cap
/// whole and leaves the top of the hole open as a pocket.
#[test]
fn a_half_plugged_hole_merges_its_bottom_cap() {
    let plug = brick::<f64>((1.0, 2.0), (0.5, 1.5), (0.0, 1.0), Tol::witness());
    assert_plug_merges(plug, 11.0, "bottom-half plug");
}

/// **An oversized plug**, overlapping the hole's rim in area on every
/// side: its caps' remainders are frames around the hole, and each cap
/// merges whole.
#[test]
fn an_oversized_plug_merges_to_whole_caps() {
    let plug = brick::<f64>((0.5, 2.5), (0.25, 1.75), (0.0, 2.0), Tol::witness());
    assert_plug_merges(plug, 12.0, "oversized plug");
}

/// **The plug as the FIRST operand.** The result arena is `A`'s, so the
/// plug's caps are arena-first in their cap groups and each lies in the
/// block's hole. The merge keeps the block's cap, the face with the
/// hole, and absorbs the plug's into it; kept the other way round, the
/// plug's cap would receive the block's ring and `kef` would find one
/// face on both sides of the rim.
#[test]
fn a_plug_folded_first_merges_into_the_face_it_plugs() {
    let tol = Tol::witness();
    let plug = brick::<f64>((1.0, 2.0), (0.5, 1.5), (0.0, 2.0), tol);
    let block = holed();
    let decls = flush_declarations(&plug, &block, tol);
    let bb =
        unwrap_body(union_with(&plug, &block, &decls, tol).expect("the plug folded first merges"));
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{:?}",
        bb.naming.merge_skipped
    );
    assert_eq!(bb.body.faces().count(), 6);
    assert_eq!(mass_properties(&bb.body, tol).unwrap().volume, 12.0);
    assert_green(&bb, "plug ∪ block");
}
