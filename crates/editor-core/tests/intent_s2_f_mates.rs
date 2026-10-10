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

/// Which relative-freedom component `id` is in.
fn component_of(doc: &ProfileDoc, id: RecipeNodeId) -> Option<usize> {
    relative_freedom_components(doc)
        .iter()
        .position(|c| c.contains(&id))
}

/// **Test 20 (A9 over reads).** Two instances are relatively free until
/// a mate reads a face of each: its reads reach down to both instances,
/// so mate and instances are one component.
#[test]
fn a_mate_couples_its_two_instances_through_its_reads() {
    let (doc, [a, b], _, body) = pair("f-a9");
    let of = component_of;
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

/// **Test 20 (A9 over reads, without a mate).** A measure across two
/// unmated instances reads both, so it couples them as a mate does
/// (Q4); and two instances on one gauge stay one component, because the
/// gauge edges outlive A12's reading edges.
#[test]
fn a_measure_and_a_shared_gauge_couple_as_reads_do() {
    let (doc, [a, b], _, body) = pair("f-a9-measure");
    let (measured, measure) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            editor_core::SitedRef::at_mint(in_part(a, body, CapEnd::End)),
            editor_core::SitedRef::at_mint(in_part(b, body, CapEnd::Start)),
        ],
    );
    assert_eq!(
        component_of(&measured, a),
        component_of(&measured, b),
        "a measure across two instances couples them"
    );
    assert_eq!(component_of(&measured, measure), component_of(&measured, a));
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            editor_core::Placement::literal(&editor_core::Frame::translation([0.0, 0.0, 5.0])),
        ),
    );
    let mut doc = doc;
    for instance in [a, b] {
        doc = step(
            doc,
            DocEdit::SetGauge {
                node: instance,
                gauge: Some(g),
            },
        )
        .0;
    }
    assert_eq!(
        component_of(&doc, a),
        component_of(&doc, b),
        "two instances on one gauge are one component"
    );
    assert_eq!(component_of(&doc, g), component_of(&doc, a));
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
            SplitError::SeveredEdge {
                consumer,
                slot: Some(OperandSlot::Side(editor_core::MateSide::B)),
                input,
                consumer_is_cut: false,
            } if *consumer == doc.spoken(mate) && *input == doc.spoken(b)
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
    let ev = run(&doc, &with_resolver(store.clone()));
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
    // Its side stopping resolving is the evaluation's refusal, not the
    // gate's (review R1, MINOR-1): the mate is still no statement about
    // the product.
    let doc = ghost_side(doc, mate, b);
    let ev = run(&doc, &with_resolver(store));
    assert!(
        ev.node_error(mate).is_some(),
        "the evaluation refuses the side"
    );
    let gated = assemble(&doc, &ev, Tol::witness()).map(|_| ());
    assert!(gated.is_ok(), "the gate asks nothing of it: {gated:?}");
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

/// `mate`'s side `b` re-pointed at a face name `instance`'s table does
/// not carry: the selection's ladder refuses it.
fn ghost_side(doc: ProfileDoc, mate: RecipeNodeId, instance: RecipeNodeId) -> ProfileDoc {
    let ghost = editor_core::StableName {
        kind: editor_core::EntityKind::Face,
        node: instance,
        path: vec![editor_core::RoleSeg::InPart {
            of: editor_core::StableName {
                kind: editor_core::EntityKind::Face,
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(77)),
                path: vec![editor_core::RoleSeg::Cap(CapEnd::Start)],
            }
            .into(),
        }],
    };
    step(
        doc,
        DocEdit::SetParam {
            node: mate,
            slot: editor_core::SlotId::Operand(OperandSlot::Side(editor_core::MateSide::B)),
            value: editor_core::SlotValue::Read(fixture::head_at(instance, ghost).into()),
            fresh: Vec::new(),
        },
    )
    .0
}

/// **A side whose name stops resolving refuses the mate in the
/// selection's words, and the gate names the mate** rather than pass a
/// mate that speaks for nothing.
#[test]
fn a_mate_whose_side_no_longer_resolves_refuses_and_the_gate_says_so() {
    let (doc, [a, b], store, body) = pair("f-vanish");
    let doc = fixture::place_all(doc, &[a, b]);
    let (doc, mate) = insert(doc, seat(body, (a, a), (b, b)));
    let doc = ghost_side(doc, mate, b);
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

/// Two local cubes, `x` and `y`, a transform `t` lifting `x` by 5, and
/// `x` and `y` placed; a distance measure reads a face of `x` in
/// `body_of_x(x, t)` and a face of `y`. Answers the document and
/// `(x, t, the placement of x)`.
fn lifted_cube_measured(
    label: &str,
    body_of_x: impl Fn(RecipeNodeId, RecipeNodeId) -> RecipeNodeId,
    placed: impl Fn(RecipeNodeId, RecipeNodeId) -> RecipeNodeId,
) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let cube = |doc: ProfileDoc, x: f64| {
        let (doc, p) = on_frame(
            doc,
            [x, 0.0, 0.0],
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
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, x) = cube(doc, 0.0);
    let (doc, y) = cube(doc, 3.0);
    let (doc, t) = insert(
        doc,
        fixture::xform(x, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0),
    );
    let (doc, px) = fixture::place(doc, placed(x, t));
    let (doc, _) = fixture::place(doc, y);
    let (doc, _) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            editor_core::SitedRef::new(
                body_of_x(x, t),
                fixture::fname(x, editor_core::RoleSeg::Cap(CapEnd::End)),
            ),
            editor_core::SitedRef::new(
                y,
                fixture::fname(y, editor_core::RoleSeg::Cap(CapEnd::End)),
            ),
        ],
    );
    (doc, [x, t, px])
}

/// **A selection crosses a cut only through a placement of its own
/// body** (review R1, MAJOR-1). A measure reads `x`'s face in the
/// transform `t` over it, and only `x` is placed: the cut placement's
/// copy holds the name, but not where the measure reads it, so the
/// read is severed rather than re-anchored five units away. The
/// reverse — the face read in `x`, only `t` placed — likewise.
#[test]
fn a_selection_crosses_only_through_a_placement_of_its_own_body() {
    for (label, body_of_x, placed, severed) in [
        (
            "f-cross-through-t",
            (|_x, t| t) as fn(RecipeNodeId, RecipeNodeId) -> RecipeNodeId,
            (|x, _t| x) as fn(RecipeNodeId, RecipeNodeId) -> RecipeNodeId,
            (|_x, t| t) as fn(RecipeNodeId, RecipeNodeId) -> RecipeNodeId,
        ),
        ("f-cross-through-x", |x, _t| x, |_x, t| t, |x, _t| x),
    ] {
        let (doc, [x, t, px]) = lifted_cube_measured(label, body_of_x, placed);
        // `t` and everything it reads, and the placement.
        let mut cut = std::collections::BTreeSet::from([px]);
        let mut stack = vec![t];
        while let Some(n) = stack.pop() {
            if cut.insert(n) {
                stack.extend(doc.upstream(n));
            }
        }
        let err = editor_core::split(
            &doc,
            &cut,
            DocumentId::derive(&format!("{label}-part")),
            Tol::witness(),
            None,
        )
        .expect_err("the measure reads a body no cut placement places");
        let input = severed(x, t);
        assert!(
            matches!(
                &err,
                SplitError::SeveredEdge { input: i, consumer_is_cut: false, .. }
                    if *i == doc.spoken(input)
            ),
            "{label}: {err:?}"
        );
    }
}

/// **A selection of a pattern's copies is a mate side's alone** (review
/// R1, MAJOR-2). A side reads a face of copy 2 in the pattern's
/// `Bodies`, which a side admits; the document saves. A measure
/// reading that same selection by its variable is refused at the door,
/// so the editor never holds a document `persist::save` refuses.
#[test]
fn a_side_selection_of_a_patterns_copies_is_read_by_sides_alone() {
    let mut store = PartStore::default();
    let (leg_ref, leg) = store.insert_part(cube_part("f-shared-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(cube_part("f-shared-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("f-shared"), Tol::witness());
    let (doc, instance) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: instance.into(),
            count: Formula::count(4),
            kind: editor_core::PatternKind::Linear {
                direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, mate) = insert(
        doc,
        Node::Mate {
            a: fixture::head(fixture::in_copy(
                pattern,
                2,
                in_part(instance, leg, CapEnd::End),
            ))
            .into(),
            b: fixture::head(in_part(top, top_body, CapEnd::Start)).into(),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: frame([0.0, 0.0, 1.0]),
                b: frame([0.0, 0.0, 0.0]),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Opposed,
                clocking: None,
            },
        },
    );
    let Some(Node::Mate { a: side, .. }) = doc.node(mate) else {
        panic!("the mate is there")
    };
    let side = *side;
    // Named, so another reader may read it by its variable.
    let (doc, _) = step(
        doc,
        DocEdit::RenameVar {
            var: side.into(),
            name: Some(editor_core::VarName::from_static("leg_top")),
        },
    );
    assert!(
        editor_core::persist::save(&doc, &[], Tol::witness()).is_ok(),
        "a side reading a pattern's copy saves"
    );
    let measured = editor_core::measure(
        &doc,
        &[editor_core::MeasurePrimitive::Distance {
            a: editor_core::Operand::Var(side),
            b: editor_core::Operand::from(editor_core::SitedRef::new(
                top,
                in_part(top, top_body, CapEnd::Start),
            )),
        }],
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    assert!(
        matches!(
            &measured,
            Err(editor_core::EditError::SlotVarKind {
                found: editor_core::VarKind::Bodies,
                ..
            })
        ),
        "a measure reads no selection of a pattern's copies: {:?}",
        measured.map(|out| editor_core::persist::save(&out.doc, &[], Tol::witness()).map(|_| ()))
    );
}

/// **A selection crosses only to a copy that stands where its body did**
/// (review R2, M1). A measure reads a face of a local cube placed ten
/// up: the cube's world copy is posed, so the remainder re-anchored on
/// it would measure the world, not the body it read. The split refuses
/// `RemainderReadUncarried { Posed }`, as inline would refuse to put it
/// back.
#[test]
fn a_selection_crossing_a_posed_placement_refuses() {
    let (doc, [x, t, _]) = lifted_cube_measured("f-posed", |x, _| x, |_, t| t);
    // Re-place `x` itself, posed, and drop the transform's placement.
    let placed_t = doc
        .ids()
        .into_iter()
        .find(|&p| matches!(doc.node(p), Some(Node::PlaceInWorld { .. })) && doc.upstream(p).len() == 1
            && doc.node(doc.upstream(p)[0]).is_some_and(|n| matches!(n, Node::Transform { .. })))
        .expect("the transform is placed");
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: placed_t });
    let (doc, posed) = step(
        doc,
        DocEdit::place(
            editor_core::Operand::Node(x),
            Some(editor_core::Placement::literal(&editor_core::Frame::translation([
                0.0, 0.0, 10.0,
            ]))),
        ),
    );
    let posed = posed.expect("the placement mints");
    let mut cut = std::collections::BTreeSet::from([posed]);
    let mut stack = vec![t];
    while let Some(n) = stack.pop() {
        if cut.insert(n) {
            stack.extend(doc.upstream(n));
        }
    }
    let err = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("f-posed-part"),
        Tol::witness(),
        None,
    )
    .expect_err("the measure's copy is posed");
    assert!(
        matches!(
            &err,
            SplitError::RemainderReadUncarried {
                why: editor_core::Uncarried::Posed { placement },
                ..
            } if *placement == doc.spoken(posed)
        ),
        "{err:?}"
    );
}
