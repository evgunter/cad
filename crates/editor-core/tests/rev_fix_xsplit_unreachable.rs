//! FIX review probe — AQ8 unreachability, over EVERY cut set.
//!
//! The unit's own rows check the claim "no accepted cut severs a mate
//! EDGE" against four hand-constructed cuts. That is a sample, and the
//! item file discloses it as one. These rows close it by exhaustion:
//! for a document carrying the three head shapes a pattern brings, run
//! [`split`] over EVERY subset of the recipe and assert that no
//! accepted cut ever mints an interface crossing.
//!
//! The edge notion is re-derived here from the public
//! [`editor_core::reading_edges`] rather than from the collector's own
//! gate, so the row goes red if the two ever disagree — which is the
//! whole content of the change under review.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeSet;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocRef, DocumentId, EntityKind, Expr,
    MateFrame, MatePrimitive, Node, PatternKind, ProfileDoc, RecipeNodeId, RoleSeg, SitedRef,
    StableName, content_pin, derivation_nodes, split,
};
use fixture::{insert, len, on_frame, scl, step};
use geom_core::Tol;

const PART_BODY: RecipeNodeId = RecipeNodeId(2);

fn block(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

fn block_ref(label: &str) -> DocRef {
    let doc = block(label);
    let pin = content_pin(&doc, Tol::witness()).unwrap();
    DocRef { id: doc.id(), pin }
}

fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

fn in_copy(pattern: RecipeNodeId, i: u32, master: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: pattern,
        path: vec![RoleSeg::Instance {
            i,
            of: Box::new(master),
        }],
    }
}

fn mate_frame(origin: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

fn seat(a: StableName, b: StableName) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: SitedRef::at_mint(a),
        b: SitedRef::at_mint(b),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: mate_frame([0.0, 0.0, 1.0]),
            b: mate_frame([0.0, 0.0, 0.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

fn linear(spacing: f64) -> PatternKind {
    PatternKind::Linear {
        direction: [scl(1.0), scl(0.0), scl(0.0)],
        spacing: len(spacing),
    }
}

/// What one exhaustive sweep saw, so a row can refuse a VACUOUS pass.
struct Sweep {
    accepted: usize,
    refused: usize,
    /// Accepted cuts carrying a remainder mate whose two ends land on
    /// opposite sides — the shape a crossing would be minted from.
    straddling_mates: usize,
}

/// Runs [`split`] over every non-empty subset of `doc`'s recipe and
/// asserts the AQ8 invariant on each accepted one: an interface record
/// is always empty, and any remainder mate whose ends straddle the cut
/// is NOT an A12 edge.
fn sweep_every_cut(doc: &ProfileDoc, label: &str) -> Sweep {
    let ids: Vec<RecipeNodeId> = doc.order().to_vec();
    assert!(ids.len() <= 12, "2^n: keep the recipe small");
    // A mate is an EDGE iff BOTH its heads resolve to members — which
    // the public A12 walk reports as two edges out of the mate.
    let edge_count = |m: RecipeNodeId| {
        editor_core::reading_edges(doc)
            .into_iter()
            .filter(|&(mate, _)| mate == m)
            .count()
    };
    let mut seen = Sweep {
        accepted: 0,
        refused: 0,
        straddling_mates: 0,
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
            &cut,
            DocumentId::derive(&format!("{label}-part-{mask}")),
            Tol::witness(),
        ) else {
            seen.refused += 1;
            continue;
        };
        seen.accepted += 1;
        let Some(Node::InstantiatePart { interface, .. }) = out.remainder.node(out.instance) else {
            panic!("the split minted an instance");
        };
        assert!(
            interface.is_empty(),
            "{label}: cut {cut:?} was ACCEPTED and minted {} crossing(s) — \
             AQ8 unreachability is BROKEN: {:?}",
            interface.crossings.len(),
            interface.crossings
        );
        for &id in doc.order() {
            if cut.contains(&id) {
                continue;
            }
            let Some(Node::Mate { a, b, .. }) = doc.node(id) else {
                continue;
            };
            let inside = |n: &StableName| derivation_nodes(n).is_subset(&cut);
            if inside(&a.name) != inside(&b.name) {
                seen.straddling_mates += 1;
                assert_ne!(
                    edge_count(id),
                    2,
                    "{label}: cut {cut:?} was ACCEPTED with mate {id:?} — an A12 EDGE — \
                     straddling it. The cluster precondition should have refused."
                );
            }
        }
    }
    seen
}

/// Three head shapes in one recipe: a PATTERN-PLACED head (a member,
/// so an edge), a NESTED pattern head (not a member, so not an edge),
/// and plain instance heads.
fn three_shapes() -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive("rev-xs-shapes"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(block_ref("rev-xs-a")));
    let (doc, pa) = insert(
        doc,
        Node::Pattern {
            input: a,
            count: Expr::count(3),
            kind: linear(2.0),
        },
    );
    let (doc, b) = insert(doc, Node::instantiate_part(block_ref("rev-xs-b")));
    let (doc, c) = insert(doc, Node::instantiate_part(block_ref("rev-xs-c")));
    let (doc, pc) = insert(
        doc,
        Node::Pattern {
            input: c,
            count: Expr::count(2),
            kind: linear(3.0),
        },
    );
    let (doc, npc) = insert(
        doc,
        Node::Pattern {
            input: pc,
            count: Expr::count(2),
            kind: linear(7.0),
        },
    );
    // Pattern-placed head onto a plain one: an EDGE, welds a—b.
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                in_copy(pa, 1, in_part(a, CapEnd::End)),
                in_part(b, CapEnd::Start),
            ),
        },
    );
    // Nested head onto a plain one: NOT an edge, welds nothing.
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                in_copy(npc, 1, in_copy(pc, 1, in_part(c, CapEnd::End))),
                in_part(b, CapEnd::End),
            ),
        },
    );
    doc
}

/// The adversary the four-cut sample cannot reach: a pattern-placed
/// head whose MASTER names a FOREIGN instance. Its member is the
/// pattern's INPUT (`a`), but its derivation set is `{pattern, c}` —
/// so "the name is inside the cut" and "the member is inside the cut"
/// are computed from different nodes, and this is where they could
/// diverge.
fn foreign_master() -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive("rev-xs-foreign"), Tol::witness());
    let (doc, _datum) = insert(
        doc,
        fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, a) = insert(doc, Node::instantiate_part(block_ref("rev-xs-f-a")));
    let (doc, pa) = insert(
        doc,
        Node::Pattern {
            input: a,
            count: Expr::count(3),
            kind: linear(2.0),
        },
    );
    let (doc, c) = insert(doc, Node::instantiate_part(block_ref("rev-xs-f-c")));
    let (doc, d) = insert(doc, Node::instantiate_part(block_ref("rev-xs-f-d")));
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                in_copy(pa, 2, in_part(c, CapEnd::End)),
                in_part(d, CapEnd::Start),
            ),
        },
    );
    doc
}

/// INVARIANT (AQ8, exhaustively): over EVERY subset of the recipe, no
/// cut `split` accepts ever mints an interface crossing, and no
/// accepted cut leaves an A12 EDGE straddling it.
#[test]
fn no_cut_whatsoever_severs_a_mate_edge_or_mints_a_crossing() {
    let seen = sweep_every_cut(&three_shapes(), "shapes");
    assert!(seen.accepted > 0, "the sweep accepted nothing: vacuous");
    assert!(
        seen.straddling_mates > 0,
        "no accepted cut ever put a mate's ends on opposite sides — the row \
         cannot go red, and the nested-head seam is unexercised"
    );
    eprintln!(
        "shapes: {} accepted, {} refused, {} straddling remainder mates",
        seen.accepted, seen.refused, seen.straddling_mates
    );
}

/// INVARIANT: the same, for the head whose derivation set and whose
/// MEMBER are different nodes. The D-2 closure rule ties the pattern
/// to its input, which is what keeps the two in step.
#[test]
fn a_pattern_head_whose_master_names_a_foreign_instance_still_cannot_cross() {
    let seen = sweep_every_cut(&foreign_master(), "foreign");
    assert!(seen.accepted > 0, "the sweep accepted nothing: vacuous");
    eprintln!(
        "foreign: {} accepted, {} refused, {} straddling remainder mates",
        seen.accepted, seen.refused, seen.straddling_mates
    );
}
