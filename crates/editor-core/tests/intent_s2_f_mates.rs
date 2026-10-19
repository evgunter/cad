//! **INTENT stage 2 F: a mate's sides read `Face` variables** (D10).
//!
//! A mate side is a selection of the body it is read in, so the
//! placement whose bundle holds the mate depends on what it reads as
//! every reader does: A9's partition runs over reads, a cut separating
//! a mate from a body it reads is the ordinary severed read, and the
//! at-rest gate mints a mate's declaration on the world copies of the
//! bodies its sides read.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::{
    AssemblyError, CapEnd, ContactClass, Coord, DocEdit, DocumentId, Formula, Mate, MateRef,
    MintRefusal, Node, OperandSlot, ProfileDoc, RecipeNodeId, RefusedRef, SlideAxis, SplitError,
    Value, assemble, product_recorded, relative_freedom_components,
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
        ContactClass::Rest,
    )
}

/// **`mate` added with a zero value on each freedom it leaves** — two
/// slides in its plane and the spin — so its copy stands square on the
/// target, pinned against it.
fn seated(doc: ProfileDoc, mate: Mate<Formula>) -> (ProfileDoc, MateRef) {
    let (doc, mate) = fixture::constrain(doc, mate);
    let doc = [
        (Coord::Slide(SlideAxis::X), len(0.0)),
        (Coord::Slide(SlideAxis::Y), len(0.0)),
        (Coord::Spin, fixture::ang(0.0)),
    ]
    .into_iter()
    .fold(doc, |doc, (coord, by)| {
        step(
            doc,
            DocEdit::AddValue {
                placement: mate.placement,
                value: Box::new(Value {
                    mate: mate.id,
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
        )
        .0
    });
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
        Node::instantiate_part(doc_ref),
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

/// **Test 20 (A9 over reads).** Two instances are relatively free until
/// a mate in the second's bundle reads a face of the first: the
/// placement's reads reach the first instance, so both are one
/// component.
#[test]
fn a_mate_couples_its_two_instances_through_its_reads() {
    let (doc, [a, b], _, body) = pair("f-a9");
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
}

/// **Test 21 (split across a mate).** A cut that takes the copy a kept
/// mate's target reads, which is loose and so no world placement,
/// refuses as the ordinary severed read, naming the mate's placement and
/// the copy.
#[test]
fn a_cut_severing_a_mates_read_refuses_as_a_severed_read() {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(cube_part("f-split-part"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("f-split"), Tol::witness());
    let (doc, a) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    // The mate places `a` onto `b`, which stays loose.
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

/// **Test 22 (minting on placed copies).** A mate between two
/// instances in the world mints its declaration on their two world
/// copies, and the gate certifies it.
#[test]
fn a_mate_between_placed_instances_mints_on_their_copies() {
    let (doc, [a, b], store, body) = pair("f-mint");
    let (doc, mate) = seated(doc, seat(body, (b, b), (a, a)));
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

/// **Test 22 (a member of no world copy).** A mate seating one loose
/// instance on another, whose copies a boolean reads, mints nothing: the
/// copies are in the first's own space, not the world, the mate relates
/// them there, and the product is the boolean's copy alone.
#[test]
fn a_mate_on_unplaced_operands_mints_nothing() {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(cube_part("f-unplaced-part"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("f-unplaced"), Tol::witness());
    let (doc, a) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
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
    let (doc, placed) = insert(doc, Node::place(vec![fused.into()]));
    let doc = fixture::pinned(doc, placed, &with_resolver(store.clone()));
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
    // Its side stopping resolving is the evaluation's refusal (review
    // R1, MINOR-1): it fails the mate's placement, and the union reading
    // that copy is poisoned through it, so the gate refuses the union's
    // placement poisoned there rather than mint anything of the mate.
    let doc = ghost_side(doc, mate, b);
    let ev = run(&doc, &with_resolver(store));
    assert!(
        ev.node_error(mate.placement).is_some(),
        "the evaluation refuses the side"
    );
    match assemble(&doc, &ev, Tol::witness()) {
        Err(AssemblyError::Product(refusal)) => assert!(
            matches!(
                *refusal,
                editor_core::ProductError::Root(editor_core::NodeStanding::Poisoned {
                    through,
                    ..
                }) if through == mate.placement
            ),
            "{refusal:?}"
        ),
        other => panic!("the gate refuses the poisoned product: {other:?}"),
    }
}

/// **An instance copied twice has two world copies**: its own, and a
/// placement copying its copies, pinned where they stand. A face read
/// in it answers two faces, and a declaration names one pair: the gate
/// refuses `Ambiguous` rather than pick a copy.
#[test]
fn a_side_whose_body_is_placed_twice_refuses_ambiguous() {
    let (doc, [a, b], store, body) = pair("f-twice");
    let opts = with_resolver(store);
    // The second copy's pin, read off `b`'s own copy while it is loose:
    // where the seat puts it, one up.
    let pin = editor_core::pin_at(
        &doc,
        &editor_core::Operand::Var(doc.output(b, 0).expect("an instance defines its copies")),
        &editor_core::Frame::translation([0.0, 0.0, 1.0]),
        &opts,
        Tol::witness(),
    )
    .unwrap_or_else(|refusal| panic!("the copy pins over its geometry: {refusal}"));
    let (doc, mate) = seated(doc, seat(body, (b, b), (a, a)));
    let (doc, again) = insert(doc, Node::place(vec![b.into()]));
    let (doc, _) = fixture::constrain_at(doc, again, pin);
    let ev = run(&doc, &opts);
    let (_, unminted) = fixture::mate_mints(&doc, &ev);
    assert_eq!(
        unminted,
        vec![MintRefusal::Reference {
            mate,
            side: editor_core::MateSide::On,
            name: Box::new(in_part(b, body, CapEnd::Start)),
            why: RefusedRef::Ambiguous { width: 2 },
        }]
    );
}

/// `mate`'s `on` re-pointed at the plane of a face name `instance`'s
/// table does not carry: the selection's ladder refuses it.
fn ghost_side(doc: ProfileDoc, mate: MateRef, instance: RecipeNodeId) -> ProfileDoc {
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
    // The placement is in the product, so the gather refuses it typed.
    match assemble(&doc, &ev, Tol::witness()) {
        Err(AssemblyError::Product(refusal)) => assert!(
            matches!(
                *refusal,
                editor_core::ProductError::Root(editor_core::NodeStanding::Failed { node })
                    if node == mate.placement
            ),
            "{refusal:?}"
        ),
        other => panic!("a mate with no value fails its placement at the gate: {other:?}"),
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

/// **A selection of a pattern's copies is a mate pose's alone** (review
/// R1, MAJOR-2). A mate's target reads the plane of a face of copy 2 in
/// the pattern's `Bodies`, which a pose admits; the document saves. A measure
/// reading that same selection by its variable is refused at the door,
/// so the editor never holds a document `persist::save` refuses.
#[test]
fn a_side_selection_of_a_patterns_copies_is_read_by_sides_alone() {
    let mut store = PartStore::default();
    let (leg_ref, leg) = store.insert_part(cube_part("f-shared-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(cube_part("f-shared-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("f-shared"), Tol::witness());
    let (doc, instance) =
        fixture::insert_pinned(doc, Node::instantiate_part(leg_ref), &with_resolver(store));
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
    let (doc, mate) = fixture::constrain(
        doc,
        fixture::rest(
            fixture::head(in_part(top, top_body, CapEnd::Start)),
            fixture::head(fixture::in_copy(
                pattern,
                2,
                in_part(instance, leg, CapEnd::End),
            )),
            ContactClass::Rest,
        ),
    );
    let editor_core::Target::Pose(target) = fixture::constraint(&doc, mate).to else {
        panic!("the mate's target is a pose")
    };
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
        "a pose reading a pattern's copy saves"
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
        .placements()
        .into_iter()
        .find(|&p| match doc.node(p) {
            Some(Node::Place { shapes, .. }) => {
                shapes.len() == 1 && doc.operation_of(shapes[0].read).is_some_and(|at| at == t)
            }
            _ => false,
        })
        .expect("the transform is placed");
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: placed_t });
    let (doc, posed) = insert(doc, Node::place(vec![x.into()]));
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
