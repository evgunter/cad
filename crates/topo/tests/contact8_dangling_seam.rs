//! **An area-overlap cap contact merges.** Two blocks whose coplanar
//! caps are flush and overlap in AREA meet
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

use common::{brick, describe_as_intersections, finished, holed_block, prism_z};
use geom_core::Tol;
use topo::validate::{validate_closed, validate_geometric};
use topo::{
    AtRestBody, Body, BooleanBody, BooleanResult, LoopBoundary, mass_properties, union,
    validate_pseudomanifold,
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

/// `a ∪ f`, caps glued: each cap pair meets along a two-edge seam
/// bent 90° at (1, 1).
fn a_union_f() -> BooleanBody<f64> {
    let tol = Tol::witness();
    let a = finished(
        "a",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let f = finished(
        "f",
        brick::<f64>((0.5, 1.5), (0.5, 1.5), (0.0, 1.0), tol),
        tol,
    );
    unwrap_body(union(&a, &f, tol).expect("the area-overlap union runs"))
}

/// The third brick, touching both blocks.
fn c() -> AtRestBody<f64> {
    let tol = Tol::witness();
    finished(
        "c",
        brick::<f64>((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    )
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
        mass_properties(
            &bb.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        1.75
    );
    assert_green(&bb, "a ∪ f");
}

/// **The merged union is a legal operand**: the next union, gluing its
/// own contacts, runs and skips nothing. The finished body is
/// the L-shaped hexagon's prism: six walls and two caps.
#[test]
fn the_merged_union_takes_a_third_brick() {
    let tol = Tol::witness();
    let af = a_union_f();
    let c = c();
    let bb = unwrap_body(union(&af.body, &c, tol).expect("a merged union is a boolean operand"));
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{:?}",
        bb.naming.merge_skipped
    );
    assert_eq!(bb.body.faces().count(), 8);
    assert_eq!(faces_at_height(&bb.body, 1.0), 1, "one top cap");
    assert_eq!(faces_at_height(&bb.body, 0.0), 1, "one bottom cap");
    // Exact on purpose: every coordinate is dyadic and small, so each closed-form term and
    // their sum are exact in f64 (a zero pad states no rounding; here there is none).
    assert_eq!(
        mass_properties(&bb.body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume,
        2.0
    );
    assert_green(&bb, "(a ∪ f) ∪ c");
}

/// **The bent seam's corner is deleted, and no record survives to
/// cite it.** The union ships no contact records at all: every
/// reduction record here rests at a seam vertex the zip fused, and is
/// consumed there, before the merge runs. That is why the rule for a
/// pruned vertex (a record citing it lands on the face that swallowed
/// it) is pinned on the merge's real outcome in `boolean::ops`'
/// `a_record_citing_a_pruned_free_end_lands_on_the_face_that_swallowed_it`
/// rather than here.
#[test]
fn the_bent_seams_corner_is_deleted_and_no_record_survives() {
    let bb = a_union_f();
    assert_eq!(bb.contacts, Default::default(), "{:?}", bb.contacts);
    let at_corner = bb
        .body
        .vertex_points()
        .any(|(_, p)| (p.x, p.y) == (1.0, 1.0));
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
    let u = finished("U", u, tol);
    let bar = finished(
        "bar",
        brick::<f64>((0.5, 2.5), (2.0, 3.5), (0.0, 1.0), tol),
        tol,
    );
    let bb = unwrap_body(union(&u, &bar, tol).expect("the union runs"));
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
    // Exact on purpose: every coordinate is dyadic and small, so each closed-form term and
    // their sum are exact in f64 (a zero pad states no rounding; here there is none).
    assert_eq!(
        mass_properties(&bb.body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume,
        9.0
    );
    assert_green(&bb, "U ∪ bar");
}

/// A block with a unit-square through-hole, `[0,3]×[0,2]×[0,2]` less
/// `[1,2]×[0.5,1.5]`, described.
fn holed() -> AtRestBody<f64> {
    let tol = Tol::witness();
    let mut block = holed_block::<f64>(3.0, &[1.5], tol);
    describe_as_intersections(&mut block, tol);
    finished("the holed block", block, tol)
}

/// `holed() ∪ plug`, every flush pair glued, then a next union with
/// a brick flush on the block's `x = 3` wall: both must run, publish
/// no skip and stay green, at the volumes given.
fn assert_plug_merges(plug: Body<f64>, volume: f64, what: &str) {
    let tol = Tol::witness();
    let plug = finished(what, plug, tol);
    let block = holed();
    let bb = unwrap_body(
        union(&block, &plug, tol)
            .unwrap_or_else(|e| panic!("{what}: the plugged block merges, got {e:?}")),
    );
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{what}: {:?}",
        bb.naming.merge_skipped
    );
    assert_eq!(
        mass_properties(&bb.body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume,
        volume,
        "{what}"
    );
    assert_green(&bb, what);
    println!(
        "[plug] {what}: faces {} merge_groups {:?}",
        bb.body.faces().count(),
        bb.naming.merge_groups
    );
    let next = finished(
        "next",
        brick::<f64>((3.0, 4.0), (0.0, 2.0), (0.0, 2.0), tol),
        tol,
    );
    let nb = unwrap_body(
        union(&bb.body, &next, tol)
            .unwrap_or_else(|e| panic!("{what}: the result is an operand, got {e:?}")),
    );
    assert!(nb.naming.merge_skipped.is_empty(), "{what}: next union");
    assert_eq!(
        mass_properties(&nb.body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume,
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
    let join = topo::test_support::boolean_join_refusal(
        topo::BooleanOp::Union,
        &block,
        &plug,
        Tol::witness(),
    );
    assert!(
        matches!(join, Ok(None)),
        "the join builds the exact plug: got {join:?}"
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
/// face on both sides of the rim. The merged cap wears the plug's
/// description, the first-written operand's, as every glued pair does:
/// red if the hole's host keeps its own.
#[test]
fn a_plug_folded_first_merges_into_the_face_it_plugs() {
    let tol = Tol::witness();
    let plug = finished(
        "plug",
        brick::<f64>((1.0, 2.0), (0.5, 1.5), (0.0, 2.0), tol),
        tol,
    );
    let block = holed();
    let bb = unwrap_body(union(&plug, &block, tol).expect("the plug folded first merges"));
    assert!(
        bb.naming.merge_skipped.is_empty(),
        "{:?}",
        bb.naming.merge_skipped
    );
    assert_eq!(bb.body.faces().count(), 6);
    // Exact on purpose: every coordinate is dyadic and small, so each closed-form term and
    // their sum are exact in f64 (a zero pad states no rounding; here there is none).
    assert_eq!(
        mass_properties(&bb.body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume,
        12.0
    );
    assert_green(&bb, "plug ∪ block");
    // The result arena is the plug's clone, so its surface keys are the
    // plug's own.
    for z in [0.0, 2.0] {
        let description = |b: &Body<f64>| {
            let faces: Vec<_> = b
                .faces()
                .filter(|&(f, _)| face_height(b, f) == Some(z))
                .map(|(_, face)| (face.surface, face.sense))
                .collect();
            let [one] = faces[..] else {
                panic!("one cap at z = {z}: {faces:?}")
            };
            one
        };
        assert_eq!(
            description(&bb.body),
            description(&plug),
            "the cap at z = {z} wears the plug's description"
        );
    }
}

/// **The merged faces wear the first-written operand's description
/// where the other operand's face is arena-first.** `a` is built in a
/// fresh-keyed body, so its faces sit high in the face arena and `b`'s,
/// grafted into the slots the fixture freed, sit below them: every merge
/// group's arena-first face, and its survivor, is `b`'s (checked). Each
/// merged face (the two caps and the two walls flush across `x = 1`)
/// wears the surface and sense of `a`'s face it absorbed, the glue's
/// kept side; red if the merge keeps its arena-first member's.
#[test]
fn the_merged_faces_wear_the_first_operands_description_where_the_second_is_arena_first() {
    let tol = Tol::witness();
    let a = finished(
        "a",
        common::brick_with_fresh_keys::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let b = finished(
        "b",
        brick::<f64>((1.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let bb = unwrap_body(union(&a, &b, tol).expect("the side-by-side union runs"));
    let from_b: Vec<topo::FaceKey> = bb.naming.graft_faces.iter().map(|&(_, r)| r).collect();
    assert_eq!(
        bb.naming.merge_groups.len(),
        4,
        "{:?}",
        bb.naming.merge_groups
    );
    for (kept, absorbed) in &bb.naming.merge_groups {
        assert!(
            from_b.contains(kept),
            "b's face {kept:?} is arena-first and survives"
        );
        let [from_a] = absorbed[..] else {
            panic!("one absorbed face, a's: {absorbed:?}")
        };
        let a_face = a.get_face(from_a).expect("the absorbed face is a's");
        let merged = bb.body.get_face(*kept).expect("the survivor is live");
        assert_eq!(
            (merged.surface, merged.sense),
            (a_face.surface, a_face.sense),
            "the face merged into {kept:?} wears a's description"
        );
    }
    assert_eq!(
        mass_properties(&bb.body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume,
        2.0
    );
    assert_green(&bb, "a ∪ b, b arena-first");
}
