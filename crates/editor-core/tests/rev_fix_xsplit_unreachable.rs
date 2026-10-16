//! FIX review probe — AQ8's crossing gate, over EVERY cut set.
//!
//! A kept constraint whose two face reads land on opposite sides of a
//! cut is a kept placement pinned against the part, and the split
//! records it as one interface crossing — but only when both its sides
//! resolve to members (AQ8 option (b): a dangling side contributes
//! nothing). Hand-built cuts sample that claim; these rows close it by
//! exhaustion: for a document carrying the three head shapes a pattern
//! brings, run [`split`] over EVERY subset of the recipe and assert
//! that each accepted cut's record holds exactly the straddling
//! constraints that weld two members.
//!
//! Whether a constraint welds its two members is re-derived here from
//! the member walk (`fixture::mate_members`) rather than from the
//! collector's own gate, so the row goes red if the two ever disagree.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use std::collections::BTreeSet;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocRef, DocumentId, Formula, Mate, MateFrame,
    MatePrimitive, MateRef, Node, PatternKind, ProfileDoc, RecipeNodeId, SideRead, StableName,
    derivation_nodes, split,
};
use fixture::resolver::in_part;
use fixture::{in_copy, insert, len, on_frame, scl};
use geom_core::Tol;

/// The unit cube `[0,1]³`, as a whole part document, and its body.
fn block(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

/// [`block`] as a reference, and its body, placed in the part's world
/// ([`fixture::resolver::PartStore::insert_part`], which is what lets
/// [`in_part`] spell its caps through the placement).
fn block_ref(label: &str) -> (DocRef, RecipeNodeId) {
    fixture::resolver::PartStore::new().insert_part(block(label), Tol::witness())
}

/// **The world the product is** (A10): each `(pattern, count)`'s copies
/// placed through a `Part` per copy. The instances are placements of
/// their own.
fn world(doc: ProfileDoc, patterns: &[(RecipeNodeId, i64)]) -> ProfileDoc {
    patterns.iter().fold(doc, |doc, &(pattern, count)| {
        (0..count).fold(doc, |doc, i| {
            let (doc, copy) = insert(
                doc,
                Node::Part {
                    of: pattern.into(),
                    select: editor_core::PartSelect::Instance(Formula::count(i)),
                },
            );
            fixture::place(doc, copy).0
        })
    })
}

/// Whether `id` is one of [`world`]'s nodes: a `Place`, or a `Part`
/// selecting a pattern copy for one.
fn of_the_world(doc: &ProfileDoc, id: RecipeNodeId) -> bool {
    matches!(doc.node(id), Some(Node::Place { .. } | Node::Part { .. }))
}

/// `cut` and the world copies of what it moves: each `Part` selecting a
/// copy of a cut pattern, then every placement of a cut body.
fn with_world(doc: &ProfileDoc, cut: &BTreeSet<RecipeNodeId>) -> BTreeSet<RecipeNodeId> {
    let mut out = cut.clone();
    for id in doc.ids() {
        if let Some(Node::Part { of, .. }) = doc.node(id)
            && doc.operation_of(*of).is_some_and(|at| out.contains(&at))
        {
            out.insert(id);
        }
    }
    fixture::with_placements(doc, &out)
}

fn mate_frame(origin: [f64; 3]) -> MateFrame<Formula> {
    MateFrame::authored(
        origin,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

fn seat(a: StableName, b: StableName) -> Mate<Formula> {
    fixture::mate(
        fixture::head(a),
        fixture::head(b),
        ContactClass::Rest,
        Alignment {
            a: mate_frame([0.0, 0.0, 1.0]),
            b: mate_frame([0.0, 0.0, 0.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    )
}

fn linear(spacing: f64) -> PatternKind<Formula> {
    PatternKind::Linear {
        direction: [scl(1.0), scl(0.0), scl(0.0)],
        spacing: len(spacing),
    }
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
/// world placements and copy `Part`s ride with what they place,
/// [`with_world`]) and asserts the AQ8 invariant on each accepted one:
/// the interface record holds one crossing per kept constraint whose
/// ends straddle the cut and weld two members, and none for a
/// straddling constraint with a dangling side.
fn sweep_every_cut(doc: &editor_core::ProfileDoc, label: &str) -> Sweep {
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
            None,
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
                let (SideRead::Face(a), SideRead::Face(b)) = (&c.mate.a, &c.mate.b) else {
                    continue;
                };
                let inside = |n: &StableName| derivation_nodes(n).is_subset(&cut);
                if inside(&fixture::side_name(doc, *a)) != inside(&fixture::side_name(doc, *b)) {
                    seen.straddling_mates += 1;
                    if edge_count(MateRef::new(placement, c.id)) == 2 {
                        welding += 1;
                    }
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

/// Three head shapes in one recipe: a PATTERN-PLACED head (a member,
/// so an edge), a nested pattern head its name UNDERQUALIFIES (not a
/// member, so not an edge — a nested copy under its own name IS one),
/// and plain instance heads.
fn three_shapes() -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive("rev-xs-shapes"), Tol::witness());
    let (a_ref, a_body) = block_ref("rev-xs-a");
    let (doc, a) = insert(doc, Node::instantiate_part(a_ref));
    let (doc, pa) = insert(
        doc,
        Node::Pattern {
            input: a.into(),
            count: Formula::count(3),
            kind: linear(2.0),
        },
    );
    let (b_ref, b_body) = block_ref("rev-xs-b");
    let (doc, b) = insert(doc, fixture::mated_instance(b_ref));
    let (c_ref, c_body) = block_ref("rev-xs-c");
    let (doc, c) = insert(doc, Node::instantiate_part(c_ref));
    let (doc, pc) = insert(
        doc,
        Node::Pattern {
            input: c.into(),
            count: Formula::count(2),
            kind: linear(3.0),
        },
    );
    let (doc, npc) = insert(
        doc,
        Node::Pattern {
            input: pc.into(),
            count: Formula::count(2),
            kind: linear(7.0),
        },
    );
    // A plain instance seated on a pattern-placed head: an EDGE, welds
    // a—b, and `b`'s placement.
    let (doc, _) = fixture::constrain_at(
        doc,
        b,
        seat(
            in_copy(pa, 1, in_part(a, a_body, CapEnd::End)),
            in_part(b, b_body, CapEnd::Start),
        ),
    );
    // A head the name UNDERQUALIFIES — one `Instance(i)` over a
    // two-level nest, which is the name such a table never mints:
    // NOT an edge, welds nothing. The constraint door refuses such a
    // head, so it is authored the way one arises after the add
    // (`constrain_with_dangling_side`), on an instance of its own.
    let (e_ref, e_body) = block_ref("rev-xs-e");
    let (doc, e) = insert(doc, fixture::mated_instance(e_ref));
    let (doc, _) = crate::fix_pattern_mate_crossing::constrain_with_dangling_side(
        doc,
        seat(
            in_copy(npc, 1, in_part(c, c_body, CapEnd::End)),
            in_part(e, e_body, CapEnd::End),
        ),
        editor_core::MateSide::A,
        e,
        e_body,
    );
    world(doc, &[(pa, 3), (npc, 4)])
}

/// The adversary the four-cut sample cannot reach: a pattern-placed
/// head whose MASTER names a FOREIGN instance. Its member is the
/// pattern's INPUT (`a`), but its derivation set is `{pattern, c}` —
/// so "the name is inside the cut" and "the member is inside the cut"
/// are computed from different nodes, and this is where they could
/// diverge.
fn foreign_master() -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive("rev-xs-foreign"), Tol::witness());
    let (doc, _datum) = insert(doc, fixture::xy_frame());
    let (a_ref, _) = block_ref("rev-xs-f-a");
    let (doc, a) = insert(doc, Node::instantiate_part(a_ref));
    let (doc, pa) = insert(
        doc,
        Node::Pattern {
            input: a.into(),
            count: Formula::count(3),
            kind: linear(2.0),
        },
    );
    let (c_ref, c_body) = block_ref("rev-xs-f-c");
    let (doc, c) = insert(doc, Node::instantiate_part(c_ref));
    let (d_ref, d_body) = block_ref("rev-xs-f-d");
    let (doc, d) = insert(doc, fixture::mated_instance(d_ref));
    // The head resolves to no member (the walk reaches `a` under a
    // name whose master is `c`), which the insert door refuses: it is
    // authored the way such a head arises after insert.
    let (doc, _) = crate::fix_pattern_mate_crossing::constrain_with_dangling_side(
        doc,
        seat(
            in_copy(pa, 2, in_part(c, c_body, CapEnd::End)),
            in_part(d, d_body, CapEnd::Start),
        ),
        editor_core::MateSide::A,
        d,
        d_body,
    );
    world(doc, &[(pa, 3)])
}

/// INVARIANT (AQ8, exhaustively): over EVERY subset of the recipe, the
/// crossings a cut `split` accepts mints are exactly the kept
/// constraints welding two members across it.
#[test]
fn every_cut_mints_exactly_its_straddling_member_edges() {
    let seen = sweep_every_cut(&three_shapes(), "shapes");
    assert!(seen.accepted > 0, "the sweep accepted nothing: vacuous");
    assert!(
        seen.straddling_mates > seen.crossings,
        "no accepted cut ever put a dangling constraint's ends on opposite \
         sides — the non-member seam is unexercised"
    );
    eprintln!(
        "shapes: {} accepted, {} refused, {} straddling remainder constraints, {} crossings",
        seen.accepted, seen.refused, seen.straddling_mates, seen.crossings
    );
}

/// INVARIANT: the same, for the head whose derivation set and whose
/// MEMBER are different nodes. The D-2 closure rule ties the pattern
/// to its input, which is what keeps the two in step.
#[test]
fn a_pattern_head_whose_master_names_a_foreign_instance_still_cannot_cross() {
    let seen = sweep_every_cut(&foreign_master(), "foreign");
    assert!(seen.accepted > 0, "the sweep accepted nothing: vacuous");
    assert_eq!(
        seen.crossings, 0,
        "the dangling head welds nothing, so nothing crosses"
    );
    eprintln!(
        "foreign: {} accepted, {} refused, {} straddling remainder constraints, {} crossings",
        seen.accepted, seen.refused, seen.straddling_mates, seen.crossings
    );
}
