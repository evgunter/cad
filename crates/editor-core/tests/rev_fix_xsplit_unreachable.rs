//! FIX review probe — AQ8's crossing gate, over EVERY cut set.
//!
//! A kept constraint whose two face reads land on opposite sides of a
//! cut is a kept placement pinned against the part, and the split
//! records it as one interface crossing — but only when both its sides
//! resolve to members (AQ8 option (b): a dangling side contributes
//! nothing). Hand-built cuts sample that claim; these rows close it by
//! exhaustion: for a document carrying the three head shapes a family
//! brings, run [`split`] over EVERY subset of the recipe and assert
//! that each accepted cut's record holds exactly the straddling
//! constraints that weld two members.
//!
//! Whether a constraint welds its two members is re-derived here from
//! the member walk (`fixture::mate_members`) rather than from the
//! collector's own gate, so the row goes red if the two ever disagree.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fix_pattern_mate_crossing::{block_ref, blocks_reach, pinned_block, seat};
use crate::fixture;

use std::collections::BTreeSet;

use editor_core::{
    CapEnd, DocumentId, EvalOptions, Formula, MateRef, Node, ProfileDoc, RecipeNodeId, StableName,
    derivation_nodes, split,
};
use fixture::resolver::in_part;
use fixture::{in_copy, in_member, insert};
use geom_core::Tol;

/// Whether `id` is a world node: a `Place`, which rides with what it
/// places — a family in the world is one, its members in the product.
fn of_the_world(doc: &ProfileDoc, id: RecipeNodeId) -> bool {
    matches!(doc.node(id), Some(Node::Place { .. }))
}

/// `cut` and every placement of a cut body.
fn with_world(doc: &ProfileDoc, cut: &BTreeSet<RecipeNodeId>) -> BTreeSet<RecipeNodeId> {
    fixture::with_placements(doc, cut)
}

/// The face name each side of a stored rest reads, `on`'s first; `None`
/// for a mate that rests no two faces.
fn rested_names(
    doc: &ProfileDoc,
    mate: &editor_core::Mate,
) -> Option<((RecipeNodeId, StableName), (RecipeNodeId, StableName))> {
    let face = |pose| match fixture::face_reads(doc, pose)[..] {
        [one] => Some((
            doc.operation_of(doc.selection(one)?.body)?,
            fixture::side_name(doc, one),
        )),
        _ => None,
    };
    Some((face(mate.on)?, face(mate.to)?))
}

/// What one exhaustive sweep saw, so a row can refuse a VACUOUS pass.
struct Sweep {
    accepted: usize,
    refused: usize,
    /// Kept constraints of accepted cuts whose two ends land on
    /// opposite sides — the shape a crossing is minted from.
    straddling_mates: usize,
    /// Crossings the accepted cuts minted.
    crossings: usize,
}

/// Runs [`split`] over every non-empty subset of `doc`'s recipe (its
/// world placements ride with what they place,
/// [`with_world`]) and asserts the AQ8 invariant on each accepted one:
/// the interface record holds one crossing per kept constraint whose
/// ends straddle the cut and weld two members, and none for a
/// straddling constraint with a dangling side.
fn sweep_every_cut((doc, o): &(ProfileDoc, EvalOptions), label: &str) -> Sweep {
    let ids: Vec<RecipeNodeId> = doc
        .ids()
        .into_iter()
        .filter(|&id| !of_the_world(doc, id))
        .collect();
    assert!(ids.len() <= 12, "2^n: keep the recipe small");
    // A constraint welds two members iff BOTH its sides resolve to
    // members.
    let edge_count = |m: MateRef| {
        crate::fixture::mate_members(doc)
            .into_iter()
            .filter(|&(mate, _)| mate == m)
            .count()
    };
    let mut seen = Sweep {
        accepted: 0,
        refused: 0,
        straddling_mates: 0,
        crossings: 0,
    };
    for mask in 1u32..(1u32 << ids.len()) {
        let cut: BTreeSet<RecipeNodeId> = ids
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, &id)| id)
            .collect();
        let Ok(out) = split(
            doc,
            &with_world(doc, &cut),
            DocumentId::derive(&format!("{label}-part-{mask}")),
            Tol::witness(),
            o.resolver.as_ref(),
        ) else {
            seen.refused += 1;
            continue;
        };
        seen.accepted += 1;
        let Some(Node::InstantiatePart { interface, .. }) = out.remainder.node(out.instance) else {
            panic!("the split minted an instance");
        };
        let mut welding = 0;
        for placement in doc.placements() {
            if cut.contains(&placement) {
                continue;
            }
            for c in fixture::bundle(doc, placement) {
                let Some((on, to)) = c.mate().and_then(|m| rested_names(doc, m)) else {
                    continue;
                };
                // A name an instance's copy carries is spelled in its
                // part's ids, which no cut of this document holds: it
                // is inside where the node it is read at is.
                let foreign = |(_, n): &(RecipeNodeId, StableName)| {
                    derivation_nodes(n).iter().any(|&id| doc.node(id).is_none())
                };
                let inside = |side: &(RecipeNodeId, StableName)| {
                    if foreign(side) {
                        cut.contains(&side.0)
                    } else {
                        derivation_nodes(&side.1).is_subset(&cut)
                    }
                };
                let outer = match (inside(&on), inside(&to)) {
                    (false, true) => &on,
                    (true, false) => &to,
                    _ => continue,
                };
                seen.straddling_mates += 1;
                // A crossing whose outer side is a part's name, read in a
                // kept instance's copy, has no spelling a record can hold
                // (issue `a-part-name-in-the-host-is-spelled-in-the-parts-ids`):
                // it welds two members and goes unrecorded.
                if edge_count(MateRef::new(placement, c.id)) == 2 && !foreign(outer) {
                    welding += 1;
                }
            }
        }
        seen.crossings += interface.crossings.len();
        assert_eq!(
            interface.crossings.len(),
            welding,
            "{label}: cut {cut:?} was ACCEPTED and minted {} crossing(s), but {welding} kept \
             constraint(s) straddle it welding two members — AQ8's gate and the member walk \
             DISAGREE: {:?}",
            interface.crossings.len(),
            interface.crossings
        );
    }
    seen
}

/// Three head shapes in one recipe: a FAMILY head (a member, so an
/// edge), a family-of-families head its name UNDERQUALIFIES (not a
/// member, so not an edge — a member under both its indices IS one),
/// and plain instance heads.
fn three_shapes() -> (ProfileDoc, EvalOptions) {
    let o = blocks_reach(&["rev-xs-a", "rev-xs-b", "rev-xs-c", "rev-xs-e"]);
    let doc = ProfileDoc::empty(DocumentId::derive("rev-xs-shapes"), Tol::witness());
    let (doc, a, a_body) = pinned_block(doc, "rev-xs-a");
    let (doc, ka, pa) = fixture::linear(doc, a, [1.0, 0.0, 0.0], 2.0, 3);
    let (b_ref, b_body) = block_ref("rev-xs-b");
    let (doc, b) = insert(doc, fixture::mated_instance(b_ref));
    let (doc, c, c_body) = pinned_block(doc, "rev-xs-c");
    let (doc, i, pc) = fixture::linear(doc, c, [1.0, 0.0, 0.0], 3.0, 2);
    let (doc, j) = fixture::index_of(doc, "j", Formula::count(2), Some(i));
    let (doc, npc) = fixture::family_of(doc, pc, fixture::slides_along(j, [0.0, 1.0, 0.0], 7.0));
    // A plain instance seated on a family head: an EDGE, welds a's
    // family—b, and `b`'s placement.
    let (doc, _) = seat(
        doc,
        b,
        in_copy(pa, ka, 1, in_part(a, a_body, CapEnd::End)),
        in_part(b, b_body, CapEnd::Start),
    );
    // A head the name UNDERQUALIFIES — a `Member` keying `j` alone of
    // a family over `(i, j)`, which is the name such a table never
    // mints: NOT an edge, welds nothing. The door is structural, so it
    // is admitted as written, on an instance of its own.
    let (e_ref, e_body) = block_ref("rev-xs-e");
    let (doc, e) = insert(doc, fixture::mated_instance(e_ref));
    let (doc, _) = seat(
        doc,
        e,
        in_member(npc, vec![(j, 1)], in_part(c, c_body, CapEnd::End)),
        in_part(e, e_body, CapEnd::End),
    );
    (doc, o)
}

/// The adversary the four-cut sample cannot reach: a family head whose
/// MASTER was read in a FOREIGN instance. An instance's copy carries
/// its part's names verbatim (D′3), and `c` is an instance of the same
/// block part as `a`, so the master is spelled in that part's ids, as
/// `a`'s is: the head resolves to the family's member and welds it to
/// `d`. Its outer side, `d`'s face, is a part's
/// name read in a kept instance's copy, which no crossing can record
/// yet, so nothing crosses.
fn foreign_master() -> (ProfileDoc, EvalOptions) {
    let o = blocks_reach(&["rev-xs-f-a", "rev-xs-f-c", "rev-xs-f-d"]);
    let doc = ProfileDoc::empty(DocumentId::derive("rev-xs-foreign"), Tol::witness());
    let (doc, a, _) = pinned_block(doc, "rev-xs-f-a");
    let (doc, ka, pa) = fixture::linear(doc, a, [1.0, 0.0, 0.0], 2.0, 3);
    let (doc, c, c_body) = pinned_block(doc, "rev-xs-f-c");
    let (d_ref, d_body) = block_ref("rev-xs-f-d");
    let (doc, d) = insert(doc, fixture::mated_instance(d_ref));
    // The master is read in `c`, and spelled in the block part's ids;
    // the door is structural, so it is admitted as written.
    let (doc, _) = seat(
        doc,
        d,
        in_copy(pa, ka, 2, in_part(c, c_body, CapEnd::End)),
        in_part(d, d_body, CapEnd::Start),
    );
    (doc, o)
}

/// INVARIANT (AQ8, exhaustively): over EVERY subset of the recipe, the
/// crossings a cut `split` accepts mints are exactly the kept
/// constraints welding two members across it, and no accepted cut
/// leaves a straddling constraint whose head is no member: the
/// underqualified head's cuts refuse.
#[test]
fn every_cut_mints_exactly_its_straddling_member_edges() {
    let seen = sweep_every_cut(&three_shapes(), "shapes");
    assert!(seen.accepted > 0, "the sweep accepted nothing: vacuous");
    // The underqualified head names nothing the family's copy holds
    // (its members are keyed by both indices), so a cut that would put
    // it on one side and its constraint on the other cannot carry it
    // and refuses: every straddle an accepted cut leaves is a member
    // edge, recorded.
    assert!(
        seen.refused > 0,
        "no cut refused: the underqualified head is unexercised"
    );
    assert_eq!(
        seen.straddling_mates, seen.crossings,
        "an accepted cut left a straddling constraint that is no member edge"
    );
    eprintln!(
        "shapes: {} accepted, {} refused, {} straddling remainder constraints, {} crossings",
        seen.accepted, seen.refused, seen.straddling_mates, seen.crossings
    );
}

/// INVARIANT: the same, for a head whose master was read in a foreign
/// instance. The D-2 closure rule ties the family to its source, which
/// is what keeps "the name is inside the cut" and "the member is inside
/// the cut" in step.
#[test]
fn a_family_head_whose_master_names_a_foreign_instance_still_cannot_cross() {
    let seen = sweep_every_cut(&foreign_master(), "foreign");
    assert!(seen.accepted > 0, "the sweep accepted nothing: vacuous");
    assert_eq!(
        seen.crossings, 0,
        "the outer side is a part's name in a kept instance's copy, so nothing crosses"
    );
    eprintln!(
        "foreign: {} accepted, {} refused, {} straddling remainder constraints, {} crossings",
        seen.accepted, seen.refused, seen.straddling_mates, seen.crossings
    );
}
