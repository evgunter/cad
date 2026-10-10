//! **INTENT stage 2 F: a mate's sides read `Face` variables** (D10).
//!
//! A mate side is a selection of the body it is read in, so a mate
//! depends on what it reads as every reader does: A9's partition runs
//! over reads, a cut separating a mate from a body it reads is the
//! ordinary severed read, and the at-rest gate mints a mate's
//! declaration on the world copies of the bodies its sides read.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::{
    Alignment, AssemblyError, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, Formula,
    MateFrame, MatePrimitive, MintRefusal, Node, OperandSlot, ProfileDoc, RecipeNodeId, RefusedRef,
    SplitError, assemble, product_recorded, relative_freedom_components,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, on_frame, run, step};
use geom_core::Tol;

/// A unit cube as a part document, and its body.
fn cube_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

fn frame(origin: [f64; 3]) -> MateFrame<Formula> {
    MateFrame::authored(origin, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], Tol::witness())
        .expect("a definite frame")
}

/// A rest mate seating `b`'s bottom on `a`'s top, each side read at the
/// node it names.
fn seat(
    body: RecipeNodeId,
    (a_at, a): (RecipeNodeId, RecipeNodeId),
    (b_at, b): (RecipeNodeId, RecipeNodeId),
) -> editor_core::AuthoredNode {
    Node::Mate {
        a: fixture::head_at(a_at, in_part(a, body, CapEnd::End)).into(),
        b: fixture::head_at(b_at, in_part(b, body, CapEnd::Start)).into(),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame([0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// Two cube instances, the first rooting the group, and the store.
fn pair(label: &str) -> (ProfileDoc, [RecipeNodeId; 2], PartStore, RecipeNodeId) {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(cube_part(&format!("{label}-part")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    (doc, [a, b], store, body)
}

/// **Test 20 (A9 over reads).** Two instances are relatively free until
/// a mate reads a face of each: its reads reach down to both instances,
/// so mate and instances are one component.
#[test]
fn a_mate_couples_its_two_instances_through_its_reads() {
    let (doc, [a, b], _, body) = pair("f-a9");
    let of = |doc: &ProfileDoc, id| {
        relative_freedom_components(doc)
            .iter()
            .position(|c| c.contains(&id))
    };
    assert_ne!(of(&doc, a), of(&doc, b), "unmated instances are free");
    let (doc, mate) = insert(doc, seat(body, (a, a), (b, b)));
    assert_eq!(of(&doc, a), of(&doc, b), "the mate couples them");
    assert_eq!(of(&doc, mate), of(&doc, a), "and is in their component");
    assert_eq!(
        doc.upstream(mate),
        vec![a, b],
        "a mate's upstream is the bodies its sides read, in side order"
    );
}

/// **Test 21 (split across a mate).** A cut that takes the body a kept
/// mate's side reads, which no cut placement places, refuses as the
/// ordinary severed read, naming the mate and the body.
#[test]
fn a_cut_severing_a_mates_read_refuses_as_a_severed_read() {
    let (doc, [a, b], _, body) = pair("f-split");
    // `a` on a gauge of its own, so the mate declares rather than welds
    // a group the cut would tear.
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            editor_core::Placement::literal(&editor_core::Frame::translation([0.0, 0.0, 5.0])),
        ),
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetGauge {
            node: a,
            gauge: Some(g),
        },
    );
    let doc = fixture::place_all(doc, &[a]);
    let (doc, mate) = insert(doc, seat(body, (a, a), (b, b)));
    let err = editor_core::split(
        &doc,
        &[b].into_iter().collect(),
        DocumentId::derive("f-split-cell"),
        Tol::witness(),
        None,
    )
    .expect_err("the mate's side reads the cut body");
    assert!(
        matches!(
            &err,
            SplitError::SeveredEdge { consumer, input, consumer_is_cut: false }
                if *consumer == doc.spoken(mate) && *input == doc.spoken(b)
        ),
        "{err:?}"
    );
}

/// **Test 22 (minting on placed copies).** A mate between two placed
/// instances mints its declaration on their two world copies, and the
/// gate certifies it.
#[test]
fn a_mate_between_placed_instances_mints_on_their_copies() {
    let (doc, [a, b], store, body) = pair("f-mint");
    let doc = fixture::place_all(doc, &[a, b]);
    let (doc, mate) = insert(doc, seat(body, (a, a), (b, b)));
    let ev = run(&doc, &with_resolver(store));
    let (minted, unminted) = fixture::mate_mints(&doc, &ev);
    assert_eq!(
        minted.iter().map(|d| d.mate).collect::<Vec<_>>(),
        vec![mate],
        "{unminted:?}"
    );
    assert!(unminted.is_empty(), "{unminted:?}");
    assert!(
        assemble(&doc, &ev, Tol::witness()).is_ok(),
        "the declared rest certifies"
    );
}

/// **Test 22 (a member no placement reads).** A mate whose sides read
/// a boolean's operands, which no placement reads, mints nothing: it
/// relates the operands in the workbench, which is the boolean's
/// business, and the product is the boolean's copy alone.
#[test]
fn a_mate_on_unplaced_operands_mints_nothing() {
    let (doc, [a, b], store, body) = pair("f-unplaced");
    let (doc, mate) = insert(doc, seat(body, (a, a), (b, b)));
    let (doc, fused) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
        },
    );
    let doc = fixture::place_all(doc, &[fused]);
    let ev = run(&doc, &with_resolver(store));
    fixture::assert_mints_nothing(&doc, &ev, mate);
    assert_eq!(
        product_recorded(&doc, &ev, Tol::witness())
            .expect("the union gathers")
            .solid_copies
            .iter()
            .map(|s| s.node)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        1,
        "the product is the union's one copy"
    );
}

/// **A body placed twice has two world copies**, so a side read in it
/// answers two faces, and a declaration names one pair: the gate
/// refuses `Ambiguous` rather than pick a copy.
#[test]
fn a_side_whose_body_is_placed_twice_refuses_ambiguous() {
    let (doc, [a, b], store, body) = pair("f-twice");
    let doc = fixture::place_all(doc, &[a, b, b]);
    let (doc, mate) = insert(doc, seat(body, (a, a), (b, b)));
    let ev = run(&doc, &with_resolver(store));
    let (_, unminted) = fixture::mate_mints(&doc, &ev);
    assert_eq!(
        unminted,
        vec![MintRefusal::Reference {
            mate,
            side: editor_core::MateSide::B,
            name: Box::new(in_part(b, body, CapEnd::Start)),
            why: RefusedRef::Ambiguous { width: 2 },
        }]
    );
}

/// **A side whose name stops resolving refuses the mate in the
/// selection's words, and the gate names the mate** rather than pass a
/// mate that speaks for nothing.
#[test]
fn a_mate_whose_side_no_longer_resolves_refuses_and_the_gate_says_so() {
    let (doc, [a, b], store, body) = pair("f-vanish");
    let doc = fixture::place_all(doc, &[a, b]);
    let (doc, mate) = insert(doc, seat(body, (a, a), (b, b)));
    // The side re-pointed at a face name the instance's table does not
    // carry: the selection's ladder refuses it.
    let ghost = editor_core::StableName {
        kind: editor_core::EntityKind::Face,
        node: b,
        path: vec![editor_core::RoleSeg::InPart {
            of: editor_core::StableName {
                kind: editor_core::EntityKind::Face,
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(77)),
                path: vec![editor_core::RoleSeg::Cap(CapEnd::Start)],
            }
            .into(),
        }],
    };
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: mate,
            slot: editor_core::SlotId::Operand(OperandSlot::Side(editor_core::MateSide::B)),
            value: editor_core::SlotValue::Read(fixture::head_at(b, ghost).into()),
            fresh: Vec::new(),
        },
    );
    let ev = run(&doc, &with_resolver(store));
    assert!(
        matches!(
            ev.node_error(mate).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::SelectResolve {
                slot: OperandSlot::Side(editor_core::MateSide::B),
                ..
            })
        ),
        "{:?}",
        ev.node_error(mate)
    );
    match assemble(&doc, &ev, Tol::witness()) {
        Err(AssemblyError::Mint { refusals }) => assert!(
            matches!(refusals.as_slice(), [MintRefusal::Unevaluated { mate: m, .. }] if *m == mate),
            "{refusals:?}"
        ),
        other => panic!("a mate with no value refuses at the gate: {other:?}"),
    }
}
