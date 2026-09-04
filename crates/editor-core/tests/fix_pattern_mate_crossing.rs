//! FIX — the split seam's crossing collector and A11's member
//! vocabulary (`split-crossings-skip-pattern-mate-ends`).
//!
//! The collector that fills an instance's [`InterfaceRecord`] asks ONE
//! predicate for "is this reference a member reference" — the same one
//! A12's reading edges and A11's clusters ask. These rows pin what that
//! identity buys, on the two shapes a pattern brings:
//!
//! - a mate whose end is a pattern-placed instance IS an edge, so it
//!   welds a cluster and AQ8's unreachability covers it — asserted in
//!   both directions, exactly as `asm_r2b_assembly::row5_a` does for a
//!   plain edge;
//! - a mate whose end is a NESTED pattern head is outside the
//!   vocabulary, welds nothing, and so genuinely reaches a cut's
//!   opposite sides — and the AQ8 (b)-SKIP ruling makes it contribute
//!   no crossing. This is the reachable construction that tells a gate
//!   asking the vocabulary apart from one merely matching a head's
//!   spelling.
//!
//! That SKIP ruling was made at the ASM-R2b review and is recorded in
//! `asm_r2b_assembly.rs`'s rows-5-and-6 header; `ASSEMBLY.md`'s AQ8
//! clause carries only the weld/`TornCluster` half, so the ruling is
//! cited here from where it actually lives rather than as ratified
//! design text.
//!
//! The whole-cluster cut also pins A4's recorded map over an
//! `Instance(i)` head: the node ids remap, the STRUCTURAL INDEX does
//! not.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeSet;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocRef, DocumentId, EntityKind, Expr,
    MateFrame, MatePrimitive, Node, PatternKind, ProfileDoc, RecipeNodeId, RoleSeg, StableName,
    content_pin, split,
};
use fixture::{insert, len, on_frame, scl, step};
use geom_core::Tol;

/// The extrude in a one-block part document (frame, profile, extrude).
const PART_BODY: RecipeNodeId = RecipeNodeId(2);

/// The unit cube `[0,1]³`, as a whole part document.
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

/// A face of `instance`'s part product — the plain member spelling.
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

/// A face of pattern copy `i` — the `Instance(i)` spelling, the PATTERN
/// node as head and the master's own name under the qualifier.
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

/// A determining `Rest` mate seating `b`'s bottom onto `a`.
fn seat(a: StableName, b: StableName) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
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

/// The copy index the pattern-headed rows mate to.
const COPY: u32 = 2;

/// Four legs, one top: `leg`, a linear `pattern` of it (count 4), a
/// `top`, and a seat mate from pattern copy [`COPY`] onto the top.
/// Returns `(doc, leg, pattern, top, mate)`.
///
/// A bare datum sits AHEAD of the cluster and is never cut, so the part
/// document's id space is offset from the host's — which is what makes
/// the recorded map's rewrite observable rather than an identity.
fn four_legs(
    label: &str,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, _kept) = insert(
        doc,
        fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, leg) = insert(doc, Node::instantiate_part(block_ref("fix-xs-leg")));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(4),
            kind: linear(2.0),
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(block_ref("fix-xs-top")));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                in_copy(pattern, COPY, in_part(leg, CapEnd::End)),
                in_part(top, CapEnd::Start),
            ),
        },
    );
    (doc, leg, pattern, top, mate.unwrap())
}

fn cut(ids: impl IntoIterator<Item = RecipeNodeId>) -> BTreeSet<RecipeNodeId> {
    ids.into_iter().collect()
}

fn crossings(doc: &ProfileDoc, instance: RecipeNodeId) -> &editor_core::InterfaceRecord {
    let Some(Node::InstantiatePart { interface, .. }) = doc.node(instance) else {
        panic!("the split minted an instance");
    };
    interface
}

/// INVARIANT: a pattern-placed mate end is a member end, so the mate is
/// an A12 EDGE and welds the pattern's INPUT instance to the other
/// member — which is what puts a pattern-headed mate under AQ8's
/// unreachability with plain edges rather than beside it.
#[test]
fn a_pattern_headed_mate_is_an_edge_and_welds_the_pattern_input_instance() {
    let (doc, leg, _pattern, top, mate) = four_legs("fix-xs-edge");
    assert_eq!(
        editor_core::reading_edges(&doc),
        vec![(mate, leg), (mate, top)],
        "the pattern-placed head reads through the pattern's input instance"
    );
    assert_eq!(
        editor_core::clusters(&doc),
        vec![vec![leg, top]],
        "the mate welds both members into ONE placement cluster"
    );
}

/// INVARIANT (AQ8, for a PATTERN-HEADED edge — the row5_a argument,
/// re-run on the member vocabulary): cutting either member of a
/// pattern-headed mate alone refuses `TornCluster`, and the
/// whole-cluster cut that IS accepted carries both ends, so its record
/// is empty. No accepted cut severs a pattern-headed mate edge.
#[test]
fn a_pattern_headed_mate_edge_cannot_cross_a_cut_and_split_says_so_both_ways() {
    let (doc, leg, pattern, top, mate) = four_legs("fix-xs-torn");

    // The gauge is the cluster's document-order-first instance and the
    // named instance is the first member on the far side of the tear,
    // so BOTH payload fields are asserted: a refusal that named the
    // wrong pair would still be a `TornCluster`, and the repair it
    // tells the caller to make ("widen the cut to the whole cluster")
    // is only actionable if the pair is right.
    for (what, ids, gauge_is_cut) in [
        ("the pattern side", cut([leg, pattern]), true),
        (
            "the pattern side with its mate",
            cut([leg, pattern, mate]),
            true,
        ),
        ("the other member", cut([top]), false),
        ("the other member with the mate", cut([top, mate]), false),
    ] {
        let refused = split(
            &doc,
            &ids,
            DocumentId::derive("fix-xs-torn-part"),
            Tol::witness(),
        )
        .expect_err("a torn cluster refuses");
        let editor_core::SplitError::TornCluster {
            gauge,
            instance,
            gauge_is_cut: cut_side,
        } = refused
        else {
            panic!("cutting {what} tears the cluster the mate welds: {refused:?}");
        };
        assert_eq!(
            (gauge, instance, cut_side),
            (leg, top, gauge_is_cut),
            "cutting {what} names the gauge, the instance across the tear, \
             and which side the gauge is on"
        );
    }

    // The pattern WITHOUT its input is a severed recipe edge, refused
    // before the cluster rule is ever reached — D-2 closure, and the
    // premise that keeps a pattern head's derivation nodes on the same
    // side as the member it resolves to.
    let severed = split(
        &doc,
        &cut([pattern]),
        DocumentId::derive("fix-xs-severed-part"),
        Tol::witness(),
    )
    .expect_err("a severed recipe edge refuses");
    let editor_core::SplitError::SeveredEdge {
        consumer,
        input,
        consumer_is_cut,
    } = severed
    else {
        panic!("cutting the pattern alone severs its input edge: {severed:?}");
    };
    assert_eq!(
        (consumer, input, consumer_is_cut),
        (pattern, leg, true),
        "the pattern is the cut-side consumer and its instance input is the severed end"
    );

    // The whole cluster: accepted, and nothing crosses.
    let out = split(
        &doc,
        &cut([leg, pattern, top, mate]),
        DocumentId::derive("fix-xs-whole-part"),
        Tol::witness(),
    )
    .expect("a whole-cluster cut splits");
    assert!(
        crossings(&out.remainder, out.instance).is_empty(),
        "both ends moved with the cut, so the mate says nothing about the seam"
    );
}

/// INVARIANT (A4's recorded map over an `Instance(i)` head): the map is
/// a correspondence between NODE ID SPACES and nothing else. The moved
/// mate's pattern-placed end has both its node ids rewritten — the
/// pattern head and the master under the qualifier — while the
/// STRUCTURAL INDEX crosses unchanged, because the pattern's own rule
/// moves verbatim and copy `i` denotes the same copy on both sides.
#[test]
fn the_recorded_map_rewrites_a_pattern_head_s_ids_and_never_its_copy_index() {
    let (doc, leg, pattern, top, mate) = four_legs("fix-xs-remap");
    let out = split(
        &doc,
        &cut([leg, pattern, top, mate]),
        DocumentId::derive("fix-xs-remap-part"),
        Tol::witness(),
    )
    .expect("a whole-cluster cut splits");

    let (new_leg, new_pattern, new_mate) = (
        out.node_map[&leg],
        out.node_map[&pattern],
        out.node_map[&mate],
    );
    assert_ne!(new_pattern, pattern, "the part mints its own id space");

    let Some(Node::Mate { a, .. }) = out.part.node(new_mate) else {
        panic!("the mate moved into the part");
    };
    assert_eq!(
        *a,
        in_copy(new_pattern, COPY, in_part(new_leg, CapEnd::End)),
        "ids remap through the recorded map; the copy index does not"
    );
    let RoleSeg::Instance { i, .. } = a.path[0] else {
        panic!("the head keeps its Instance(i) qualifier");
    };
    assert_eq!(i, COPY, "the structural index is not in the map's domain");
}

/// A NESTED pattern head — a pattern of a pattern — is outside A11's
/// member vocabulary, so the mate is not an edge, welds nothing, and
/// its two ends DO reach opposite sides of an accepted cut.
///
/// INVARIANT (AQ8 option (b), SKIP — ruled at the ASM-R2b review and
/// recorded in `asm_r2b_assembly.rs`'s rows-5-and-6 header, NOT in
/// `ASSEMBLY.md`'s AQ8 clause, which carries only the weld half):
/// such a mate contributes NO crossing however its names fall, because
/// it never solved and a record minted from it would be
/// trusted-at-rest state. This is the
/// row that separates a gate asking the member vocabulary from one
/// matching a head's SPELLING: admitting `Node::Pattern` by shape mints
/// a record here, for a mate the cluster graph never welded.
///
/// Whether a nested head should instead RESOLVE is a live design
/// question (issue 1411); this row pins today's answer at this door and
/// settles nothing about it.
#[test]
fn a_nested_pattern_head_reaches_the_seam_and_still_contributes_no_crossing() {
    let doc = ProfileDoc::empty(DocumentId::derive("fix-xs-nested"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(block_ref("fix-xs-n-leg")));
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(2),
            kind: linear(2.0),
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner,
            count: Expr::count(2),
            kind: linear(5.0),
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(block_ref("fix-xs-n-top")));
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                in_copy(outer, 1, in_copy(inner, 1, in_part(leg, CapEnd::End))),
                in_part(top, CapEnd::Start),
            ),
        },
    );

    // Not an edge: no reading edge at the nested head, and the two
    // members stay singleton clusters.
    assert_eq!(
        editor_core::clusters(&doc),
        vec![vec![leg], vec![top]],
        "a nested head welds nothing, which is what makes the cut below legal"
    );

    // And so the cut IS accepted, with the mate's ends on opposite
    // sides — the reachable seam a spelling-matched gate would mint on.
    let out = split(
        &doc,
        &cut([leg, inner, outer]),
        DocumentId::derive("fix-xs-nested-part"),
        Tol::witness(),
    )
    .expect("nothing tears: the cut is a union of whole clusters");
    assert!(
        crossings(&out.remainder, out.instance).is_empty(),
        "a mate that is not an edge says nothing about the seam (AQ8 SKIP)"
    );
}
