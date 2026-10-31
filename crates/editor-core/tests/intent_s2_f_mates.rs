//! **INTENT stage 2 F: a mate's sides read `Face` variables** (D10).
//!
//! A mate side is a selection of the body it is read in, so the
//! placement whose bundle holds the mate depends on what it reads as
//! every reader does: A9's partition runs over reads, a cut separating
//! a mate from a body it reads is the ordinary severed read, and the
//! at-rest census records the seat a mate makes between the world
//! copies of the bodies its sides read.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Dimension;
use editor_core::{
    CapEnd, Coord, DocEdit, DocumentId, Formula, Mate, MateRef, Node, OperandSlot, ProfileDoc,
    RecipeNodeId, SplitError, TargetAxis, product_recorded, relative_freedom_components,
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
        },
    )
}

/// A rest of `on`'s bottom on `to`'s top, each face read at the node it
/// names: `on` the copy it places.
fn seat(
    body: RecipeNodeId,
    (on_at, on): (RecipeNodeId, RecipeNodeId),
    (to_at, to): (RecipeNodeId, RecipeNodeId),
) -> Mate<Formula> {
    fixture::rest(
        fixture::head_at(on_at, in_part(on, body, CapEnd::Start)),
        fixture::head_at(to_at, in_part(to, body, CapEnd::End)),
    )
}

/// **`mate` added with a zero value on each freedom it leaves** — two
/// slides in its plane and the spin — so its copy stands square on the
/// target, pinned against it.
fn seated(doc: ProfileDoc, mate: Mate<Formula>) -> (ProfileDoc, MateRef) {
    let (doc, mate) = crate::fixture::constrain_with(
        doc,
        mate,
        &[
            (
                Coord::Slide(TargetAxis::X),
                Formula::zero(Dimension::Length),
            ),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    (doc, mate)
}

/// Two cube instances, the first pinned to the world and the second
/// placed by what its bundle comes to hold, and the store.
fn pair(label: &str) -> (ProfileDoc, [RecipeNodeId; 2], PartStore, RecipeNodeId) {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(cube_part(&format!("{label}-part")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = fixture::insert_pinned(
        doc,
        crate::fixture::mated_instance(doc_ref),
        &with_resolver(store.clone()),
    );
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    (doc, [a, b], store, body)
}

/// Which relative-freedom component `id` is in.
fn component_of(doc: &ProfileDoc, id: RecipeNodeId) -> Option<usize> {
    relative_freedom_components(doc)
        .iter()
        .position(|c| c.contains(&id))
}

/// **Test 20 (A9 over reads).** Two instances, each in a space of its
/// own, are relatively free until a mate in the second's bundle reads a
/// face of the first: the
/// placement's reads reach the first instance, so both are one
/// component.
#[test]
fn a_mate_couples_its_two_instances_through_its_reads() {
    let (doc, [a, b], _, body) = pair("f-a9");
    // Each in a space of its own: two copies in the world are each
    // pinned there.
    let (doc, _) = fixture::into_new_space(doc, a);
    let (doc, _) = fixture::into_new_space(doc, b);
    let of = component_of;
    assert_ne!(of(&doc, a), of(&doc, b), "unmated instances are free");
    let (doc, mate) = fixture::constrain(doc, seat(body, (b, b), (a, a)));
    assert_eq!(mate.placement, b, "the mate places the second instance");
    assert_eq!(of(&doc, a), of(&doc, b), "the mate couples them");
    assert!(
        doc.upstream(b).contains(&a),
        "the placement's upstream holds the copy its mate targets: {:?}",
        doc.upstream(b)
    );
}

/// **Test 20 (A9 over reads, without a mate).** A measure across two
/// unmated instances reads both, so it couples them as a mate does
/// (Q4).
#[test]
fn a_measure_couples_as_reads_do() {
    let (doc, [a, b], _, body) = pair("f-a9-measure");
    let (measured, measure) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            crate::fixture::sited_ref(in_part(a, body, CapEnd::End)),
            crate::fixture::sited_ref(in_part(b, body, CapEnd::Start)),
        ],
    );
    assert_eq!(
        component_of(&measured, a),
        component_of(&measured, b),
        "a measure across two instances couples them"
    );
    assert_eq!(component_of(&measured, measure), component_of(&measured, a));
}

/// **Test 21 (split across a mate).** A cut that takes the copy a kept
/// mate's target reads, which is in a space of its own and so no world
/// placement,
/// refuses as the ordinary severed read, naming the mate's placement and
/// the copy.
#[test]
fn a_cut_severing_a_mates_read_refuses_as_a_severed_read() {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(cube_part("f-split-part"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("f-split"), Tol::witness());
    let (doc, a) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    // The mate places `a` onto `b`, which stands in a space of its own.
    let (doc, _) = fixture::into_new_space(doc, b);
    let (doc, mate) = fixture::constrain(doc, seat(body, (a, a), (b, b)));
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
                slot: Some(OperandSlot::Side {
                    constraint,
                    side: editor_core::MateSide::To,
                }),
                input,
                consumer_is_cut: false,
            } if *consumer == doc.spoken(mate.placement)
                && *constraint == mate.id
                && *input == doc.spoken(b)
        ),
        "{err:?}"
    );
}

/// **Test 22 (a seat between placed copies).** A mate between two
/// instances in the world seats their two world copies, the census
/// records the seat, and the gate certifies it.
#[test]
fn a_mate_between_placed_instances_seats_their_copies() {
    let (doc, [a, b], store, body) = pair("f-mint");
    let (doc, _) = seated(doc, seat(body, (b, b), (a, a)));
    let ev = run(&doc, &with_resolver(store));
    let product = product_recorded(&doc, &ev, Tol::witness()).expect("the pair gathers");
    assert_eq!(
        product.contacts.face_face().count(),
        1,
        "{:?}",
        product.contacts
    );
    assert!(
        crate::fixture::could_look(&doc, &ev).is_ok(),
        "the seat certifies"
    );
}

/// **Test 22 (a member of no world copy).** A mate seating one instance
/// on another in a space of its own, whose copies a boolean reads, states nothing of the world: the
/// copies are in the first's own space, not the world, the mate relates
/// them there, and the product is the boolean's copy alone.
#[test]
fn a_mate_on_unplaced_operands_states_nothing_of_the_world() {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(cube_part("f-unplaced-part"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("f-unplaced"), Tol::witness());
    let (doc, a) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, _) = fixture::into_new_space(doc, a);
    let (doc, mate) = seated(doc, seat(body, (b, b), (a, a)));
    let (doc, fused) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                fixture::copy(a, body),
                fixture::copy(b, body),
            ]),
        },
    );
    let (doc, placed) = insert(doc, crate::fixture::stood(fused));
    let doc = fixture::pinned(doc, placed, &with_resolver(store.clone()));
    let ev = run(&doc, &with_resolver(store.clone()));
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
    // Its side stopping resolving is the evaluation's refusal (review
    // R1, MINOR-1): it fails the mate's placement, and the union reading
    // that copy is poisoned through it, so the gate refuses the union's
    // placement poisoned there.
    let doc = ghost_side(doc, mate, b);
    let ev = run(&doc, &with_resolver(store));
    assert!(
        ev.node_error(mate.placement).is_some(),
        "the evaluation refuses the side"
    );
    match product_recorded(&doc, &ev, Tol::witness()).map(|_| ()) {
        Err(refusal) => assert!(
            matches!(
                refusal,
                editor_core::ProductError::Read(editor_core::NodeStanding::Poisoned {
                    through,
                    ..
                }) if through == mate.placement
            ),
            "{refusal:?}"
        ),
        other => panic!("the gate refuses the poisoned product: {other:?}"),
    }
}

/// **A mate's `on` side is read in its own placement's copy alone.**
/// A cube instance `b`, pinned off to one side, is copied by a `Place`
/// `p` whose mate rests `b`'s bottom, read in `b`'s own copy, on the top
/// of the instance `a`. The face is read in `b`'s copy, but `p`'s fold
/// moves it: the mate's statement holds in `p`'s copy, and says nothing
/// of `b`'s own, which still stands where `b`'s pin puts it. The census's
/// seat row is the first rung's; the same row with its `on` cell read in
/// `b`'s own copy is not.
///
/// *Breaks if* the `on` side is read in any copy holding its face where
/// its body stands, as the `to` side is.
#[test]
fn a_mates_on_side_is_read_in_its_own_placements_copy_alone() {
    use editor_core::coincide::NamedCell;
    use editor_core::{Proof, Rung};
    let (doc, [a, b], store, body) = pair("f-on-own");
    let opts = with_resolver(store);
    let doc = fixture::pin(
        doc,
        b,
        &editor_core::Frame::translation([5.0, 0.0, 0.0]),
        &opts,
    );
    let (doc, p) = insert(doc, crate::fixture::stood(b));
    // The seat, slid back over `a` from where `b`'s pin stands its copy,
    // so `p`'s copy rests on `a` and clear of `b`'s own.
    let (doc, mate) = fixture::mated(
        doc,
        p,
        seat(body, (b, b), (a, a)),
        &[
            (Coord::Slide(TargetAxis::X), len(-5.0)),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    let ev = run(&doc, &opts);
    let product = product_recorded(&doc, &ev, Tol::witness()).expect("the cubes gather");
    let Some(seat) = product
        .coincidences
        .iter()
        .find(|row| row.row.relation == topo::Relation::SameOpposite)
    else {
        panic!(
            "the premise: the census decided the seat: {:?}",
            product.coincidences
        )
    };
    let door = |row: &editor_core::coincide::NamedCoincidence| {
        editor_core::coincide::prove(
            &doc,
            &[(mate.placement, std::slice::from_ref(row))],
            ev.resolver(),
            Tol::witness(),
            &editor_core::CancelToken::new(),
        )
        .expect("an uncanceled door")
        .remove(0)
        .remove(0)
    };
    assert_eq!(
        door(&seat.row),
        Proof::Structural(Rung::SameConstruction),
        "the seat, read in `p`'s copy"
    );
    let own = doc.output(b, 0).expect("an instance defines its copies");
    let mut other = seat.row.clone();
    for cell in &mut other.cells {
        if let NamedCell::Entity { input, name } = cell
            && doc.operation_of(*input) == Some(p)
        {
            // A copy names its faces as its source does.
            *name = name.clone();
            *input = own;
        }
    }
    assert_ne!(
        other, seat.row,
        "the premise: the row names `b` in `p`'s copy"
    );
    let proof = door(&other);
    assert!(
        !matches!(proof, Proof::Structural(Rung::SameConstruction)),
        "the same row read in `b`'s own copy is not the mate's: {proof:?}"
    );
}

/// `mate`'s `on` re-pointed at the plane of a face name `instance`'s
/// table does not carry: the selection's ladder refuses it.
fn ghost_side(doc: ProfileDoc, mate: MateRef, instance: RecipeNodeId) -> ProfileDoc {
    let ghost = editor_core::StableName {
        kind: editor_core::EntityKind::Face,
        node: RecipeNodeId::new(0, test_utils::refusal::tagged(77)),
        path: vec![editor_core::RoleSeg::Cap(CapEnd::Start)],
    };
    step(
        doc,
        DocEdit::SetParam {
            node: mate.placement,
            slot: editor_core::SlotId::Operand(OperandSlot::Side {
                constraint: mate.id,
                side: editor_core::MateSide::On,
            }),
            value: editor_core::SlotValue::Read(fixture::plane(fixture::head_at(instance, ghost))),
            fresh: Vec::new(),
        },
    )
    .0
}

/// **A face whose name stops resolving refuses the mate's placement
/// where it evaluates, in the selection's words, and the gate names
/// the placement** rather than pass a mate that speaks for nothing.
#[test]
fn a_mate_whose_side_no_longer_resolves_refuses_and_the_gate_says_so() {
    let (doc, [a, b], store, body) = pair("f-vanish");
    let (doc, mate) = seated(doc, seat(body, (b, b), (a, a)));
    let opts = with_resolver(store);
    assert!(
        run(&doc, &opts).node_error(mate.placement).is_none(),
        "the mate holds"
    );
    let doc = ghost_side(doc, mate, b);
    let ev = run(&doc, &opts);
    assert!(
        matches!(
            ev.node_error(mate.placement).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::Mate(fault))
                if matches!(&**fault, editor_core::MateFault::SideUnevaluated {
                    mate: named,
                    side: editor_core::MateSide::On,
                    refusal,
                } if *named == mate
                    && matches!(refusal.kind(), editor_core::NodeErrorKind::SelectResolve { .. }))
        ),
        "{:?}",
        ev.node_error(mate.placement)
    );
    // The placement's copy is in the product's `⊔`, so the gather
    // refuses it typed, poisoned through the placement.
    match product_recorded(&doc, &ev, Tol::witness()).map(|_| ()) {
        Err(refusal) => assert!(
            matches!(
                refusal,
                editor_core::ProductError::Read(editor_core::NodeStanding::Poisoned {
                    through,
                    ..
                }) if through == mate.placement
            ),
            "{refusal:?}"
        ),
        other => panic!("a mate with no value fails its placement at the gate: {other:?}"),
    }
}

/// Two local cubes, `x` and `y`, a placement `t` lifting `x` by 5 in
/// its own space, and `x` and `y` placed in the world; a distance
/// measure reads `x`'s two caps (a measure reads one space, and each
/// cube is in a space of its own). Answers the document
/// and `(x, t, the placement of x)`.
fn lifted_cube_measured(label: &str) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let cube = |doc: ProfileDoc, x: f64| {
        let (doc, p) = on_frame(
            doc,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x, 0.0), (x + 1.0, 0.0), (x + 1.0, 1.0), (x, 1.0)]],
        );
        insert(
            doc,
            Node::Extrude {
                profile: p.into(),
                distance: len(1.0),
            },
        )
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, x) = cube(doc, 0.0);
    let (doc, y) = cube(doc, 3.0);
    let (doc, t) = fixture::xform(doc, x, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0);
    let x_end = fixture::fname(x, editor_core::RoleSeg::Cap(CapEnd::End));
    let (doc, px) = fixture::place(doc, x);
    let (doc, _) = fixture::place(doc, y);
    let (doc, _) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            editor_core::SitedRef::new(x, x_end),
            editor_core::SitedRef::new(
                x,
                fixture::fname(x, editor_core::RoleSeg::Cap(CapEnd::Start)),
            ),
        ],
    );
    (doc, [x, t, px])
}

/// **A selection of a family's members reads the family whole** (review
/// R1, MAJOR-2: the door and `persist::save` agree). A mate's target
/// reads the plane of a face of member 2 of the family, which a pose
/// admits; the document saves. A measure reading that same selection by
/// its variable is one measure of member 2's face, admitted at the door,
/// and the document holding it saves too.
#[test]
fn a_selection_of_a_familys_members_reads_the_family_whole() {
    let mut store = PartStore::default();
    let (leg_ref, leg) = store.insert_part(cube_part("f-shared-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(cube_part("f-shared-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("f-shared"), Tol::witness());
    let (doc, instance) = fixture::insert_pinned(
        doc,
        crate::fixture::mated_instance(leg_ref),
        &with_resolver(store),
    );
    let (doc, k, pattern) = crate::fixture::linear(doc, instance, [1.0, 0.0, 0.0], 2.0, 4);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, mate) = fixture::constrain(
        doc,
        fixture::rest(
            fixture::head_at(top, in_part(top, top_body, CapEnd::Start)),
            fixture::head(fixture::in_copy(
                pattern,
                k,
                2,
                in_part(instance, leg, CapEnd::End),
            )),
        ),
    );
    let target = fixture::constraint(&doc, mate).to;
    let [side] = fixture::face_reads(&doc, target)[..] else {
        panic!("the mate's target reads one face")
    };
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
        "a pose reading a family's member saves"
    );
    let measured = editor_core::measure(
        &doc,
        &[editor_core::MeasurePrimitive::Distance {
            a: editor_core::Operand::Var(side),
            b: editor_core::Operand::from(crate::fixture::sited_ref(in_part(
                top,
                top_body,
                CapEnd::Start,
            ))),
        }],
        Tol::witness(),
    );
    let measured = measured.expect("a measure reads a selection of a family's members");
    assert!(
        measured
            .doc
            .node_reach(measured.measured.measures[0])
            .chain()
            .is_some_and(<[_]>::is_empty),
        "the selection reads the family whole, so the measure is one measure"
    );
    assert!(
        editor_core::persist::save(&measured.doc, &[], Tol::witness()).is_ok(),
        "the door's measure saves"
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
    let (doc, [x, t, px]) = lifted_cube_measured("f-posed");
    // Re-place `x` posed, in place of its world placement at the identity.
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: px });
    let (doc, posed) = insert(doc, crate::fixture::stood(x));
    let doc = fixture::pin(
        doc,
        posed,
        &editor_core::Frame::translation([0.0, 0.0, 10.0]),
        &editor_core::EvalOptions::default(),
    );
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
