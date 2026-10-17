//! INTENT stage 3 PR B: a placement is the bundle of mates
//! (`docs/INTENT-STAGE3-SPEC.md` §3, §10 rows 5, 6 and 12).
//!
//! A copy is made by a placement — a `Place` of the shapes it reads, or
//! an instance of a part — and pinned by its bundle of constraints, each
//! addressed by the placement and the id minted with it. A copy pinned,
//! transitively, against the world is in the product; one its bundle
//! pins less is a loose copy, a space of its own. The rows here pin the
//! representation; the solve's arithmetic is the mate suites'.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::resolver::{PartStore, copy_key, in_part, with_resolver};
use crate::fixture::{
    self, at_the_door_of, constrain, insert, len, mated_instance, on_frame, run, square, step,
};
use editor_core::eval::bound_pose;
use editor_core::pose::PoseDef;
use editor_core::{
    Alignment, AxisSense, BodyRead, CapEnd, ContactClass, CopyKey, DocEdit, DocumentId, EditError,
    EvalOptions, ExtrudeSide, Formula, MateFault, MateFrame, MatePrimitive, Node, NodeErrorKind,
    NodeResult, Operand, OperandSlot, Placement, PoseValue, ProductError, ProfileDoc, RecipeNodeId,
    RoleSeg, Space, StableName,
};
use geom_core::Tol;
use geom_core::UnitVec3;

// ---- Substrate ----

/// A unit block, `1 × 1 × 1` above the origin, in a document of its own,
/// and its body.
fn block(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
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

/// A host document, the store that resolves its part (a unit block
/// placed in its world), the part's reference and its body.
fn host(label: &str) -> (ProfileDoc, PartStore, editor_core::DocRef, RecipeNodeId) {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(block(&format!("{label}-part")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    (doc, store, doc_ref, body)
}

fn frame(origin: [f64; 3], axis: [f64; 3], reference: [f64; 3]) -> MateFrame<Formula> {
    MateFrame::authored(origin, axis, reference, Tol::witness()).expect("a definite frame")
}

fn z_up() -> MateFrame<Formula> {
    frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
}

/// A mate between the start caps of instances `a` and `b` of the part
/// whose body is `body`.
fn mate(
    body: RecipeNodeId,
    a: RecipeNodeId,
    b: RecipeNodeId,
    primitive: MatePrimitive,
    sense: AxisSense,
    (fa, fb): (MateFrame<Formula>, MateFrame<Formula>),
    clocking: Option<f64>,
) -> editor_core::Mate<Formula> {
    fixture::mate(
        fixture::head(in_part(a, body, CapEnd::Start)),
        fixture::head(in_part(b, body, CapEnd::Start)),
        ContactClass::Rest,
        Alignment {
            a: fa,
            b: fb,
            primitive,
            sense,
            clocking,
        },
    )
}

/// Coaxial about the shared +z, clocked at zero: a prismatic pair.
fn coaxial(body: RecipeNodeId, a: RecipeNodeId, b: RecipeNodeId) -> editor_core::Mate<Formula> {
    mate(
        body,
        a,
        b,
        MatePrimitive::Coaxial,
        AxisSense::Aligned,
        (z_up(), z_up()),
        Some(0.0),
    )
}

/// Seated on `a`'s top: the slide the coaxial leaves, closed.
fn seated(body: RecipeNodeId, a: RecipeNodeId, b: RecipeNodeId) -> editor_core::Mate<Formula> {
    mate(
        body,
        a,
        b,
        MatePrimitive::PlanarRest { offset: -1.0 },
        AxisSense::Opposed,
        (
            frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
            frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
        ),
        None,
    )
}

/// The fault the evaluation refused `placement` with.
fn mate_fault(ev: &editor_core::Evaluation<f64>, placement: RecipeNodeId) -> MateFault {
    match ev.result(placement) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("a mate refusal, not {other:?}"),
        },
        other => panic!("a failure, not {other:?}"),
    }
}

fn product_solids(doc: &ProfileDoc, ev: &editor_core::Evaluation<f64>) -> usize {
    editor_core::product_recorded(doc, ev, Tol::witness())
        .expect("the product gathers")
        .solid_copies
        .len()
}

// ---- Row 5: two placements, two copies ----

/// **Two `Place`s of one body, each against the world, are two copies**:
/// two outputs, two solids in the product, and each name of the body
/// resolving once per copy, under its placement.
#[test]
fn two_placements_of_one_body_are_two_copies() {
    let (doc, body) = block("s3b-two-copies");
    let (doc, first) = fixture::place(doc, body);
    let (doc, second) = step(
        doc,
        DocEdit::place_in_world(
            Operand::Node(body),
            Some(Placement::literal(&editor_core::Frame::translation([
                3.0, 0.0, 0.0,
            ]))),
        ),
    );
    let second = second.expect("a placement mints its node");
    assert_ne!(
        doc.output(first, 0),
        doc.output(second, 0),
        "one output each"
    );
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(product_solids(&doc, &ev), 2);
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    let cap = fixture::fname(body, RoleSeg::Cap(CapEnd::End));
    for placement in [first, second] {
        let copied = StableName {
            kind: cap.kind,
            node: placement,
            path: vec![RoleSeg::Placed {
                of: editor_core::NameRef::new(cap.clone()),
            }],
        };
        assert!(
            product.names.lookup(&copied).is_some(),
            "the cap resolves in {placement:?}'s copy"
        );
    }
}

// ---- Row 6: the product is the world's kind ----

/// **A copy pinned against a copy pinned against the world is in the
/// product**: the second instance, coaxial and seated on the first,
/// which is pinned to the world.
#[test]
fn a_copy_pinned_against_a_pinned_copy_is_in_the_product() {
    let (doc, store, doc_ref, body) = host("s3b-chain");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = fixture::constrain_through(doc, &reach, b, coaxial(body, a, b));
    let (doc, _) = fixture::constrain_through(doc, &reach, b, seated(body, a, b));
    let ev = run(&doc, &o);
    assert_eq!(ev.space(b), Space::World);
    assert_eq!(product_solids(&doc, &ev), 2);
}

/// **Two copies pinned only against each other are one loose space**:
/// not in the product, which `EmptyProduct` refuses naming the loose
/// copy, and gathered by itself for the at-rest census.
#[test]
fn copies_pinned_only_against_each_other_are_one_loose_space() {
    let (doc, store, doc_ref, body) = host("s3b-loose-pair");
    let (doc, a) = insert(doc, mated_instance(doc_ref));
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = fixture::constrain_through(doc, &reach, b, coaxial(body, a, b));
    let (doc, _) = fixture::constrain_through(doc, &reach, b, seated(body, a, b));
    let ev = run(&doc, &o);
    assert_eq!(ev.space(a), Space::Loose { root: a });
    assert_eq!(ev.space(b), Space::Loose { root: a });
    match editor_core::product_recorded(&doc, &ev, Tol::witness()) {
        Err(ProductError::EmptyProduct { loose, .. }) => assert_eq!(loose, vec![a]),
        other => panic!("an empty world naming the loose copy, not {other:?}"),
    }
    let spaces = editor_core::loose_spaces(&doc, &ev, Tol::witness());
    assert_eq!(spaces.len(), 1, "one loose space");
    assert_eq!(spaces[0].root, a);
    let gathered = spaces[0].gather.as_ref().expect("the loose space gathers");
    assert_eq!(gathered.solid_copies.len(), 2, "both copies, by themselves");
}

/// **A copy pinned against a construction lives in the construction's
/// space**: a subtract of two copies of one loose space is in that
/// space, so a copy pinned against one of its faces joins the space too.
/// It is neither in the world nor in the product, which refuses
/// `EmptyProduct` naming the space's root.
#[test]
fn a_copy_pinned_against_a_construction_of_a_loose_space_is_in_that_space() {
    let doc = ProfileDoc::empty(DocumentId::derive("s3b-loose-construction"), Tol::witness());
    let (doc, a) = block_at(doc, 0.5);
    let (doc, b) = block_at(doc, 0.2);
    let (doc, c) = block_at(doc, 0.1);
    let (doc, l) = step(doc, DocEdit::place(vec![a.into()], vec![]));
    let l = l.expect("the loose placement mints");
    let cap = fixture::fname(a, RoleSeg::Cap(CapEnd::Start));
    let placed_cap = StableName {
        kind: cap.kind,
        node: l,
        path: vec![RoleSeg::Placed {
            of: editor_core::NameRef::new(cap),
        }],
    };
    let pin_on = |target: editor_core::SideRead<Operand>| editor_core::Mate {
        a: target,
        b: editor_core::SideRead::Copy,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::on_part(Placement::IDENTITY),
            b: MateFrame::on_part(Placement::IDENTITY),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    };
    let on_l =
        editor_core::SideRead::Face(Operand::select(Operand::output(l, 0), vec![placed_cap]));
    let (doc, l2) = step(doc, DocEdit::place(vec![b.into()], vec![pin_on(on_l)]));
    let l2 = l2.expect("the pinned placement mints");
    let (doc, s) = insert(
        doc,
        Node::Subtract {
            from: Operand::output(l, 0).into(),
            tool: Operand::output(l2, 0).into(),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(
        ev.space(s),
        Space::Loose { root: l },
        "the subtract is in L's space"
    );
    let face = ev
        .value(s)
        .expect("the subtract evaluates")
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| n.kind == editor_core::EntityKind::Face)
        .expect("the subtract names a face");
    let on_s = editor_core::SideRead::Face(Operand::select(Operand::output(s, 0), vec![face]));
    let (doc, p) = step(doc, DocEdit::place(vec![c.into()], vec![pin_on(on_s)]));
    let p = p.expect("the placement on the construction mints");
    let ev = run(&doc, &EvalOptions::default());
    assert!(
        matches!(ev.result(p), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(p)
    );
    assert_eq!(
        ev.space(p),
        Space::Loose { root: l },
        "pinned in L's space, not the world"
    );
    match editor_core::product_recorded(&doc, &ev, Tol::witness()) {
        Err(ProductError::EmptyProduct { loose, .. }) => assert_eq!(loose, vec![l]),
        other => panic!("an empty world naming L's space, not {other:?}"),
    }
}

/// A square block of half-width `half` and unit height, its profile on
/// the XY frame at the origin.
fn block_at(doc: ProfileDoc, half: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, half)],
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

/// **A copy its bundle pins less than its body needs is a loose copy,
/// not a refusal**: coaxial alone leaves a slide.
#[test]
fn an_under_pinned_copy_is_loose_not_refused() {
    let (doc, store, doc_ref, body) = host("s3b-under");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, _) = constrain(doc, coaxial(body, a, b));
    let o = with_resolver(store);
    let ev = run(&doc, &o);
    assert!(
        matches!(ev.result(b), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(b)
    );
    assert_eq!(ev.space(b), Space::Loose { root: b });
    assert_eq!(product_solids(&doc, &ev), 1, "the pinned copy alone");
}

// ---- Row 12: the world ----

/// **The world is undeletable, minted with every document, and read by
/// placements alone**: deleting it refuses, a second one refuses, and a
/// construction cannot name it, since it defines no variable.
#[test]
fn the_world_is_undeletable_and_defines_nothing() {
    let doc = ProfileDoc::empty(DocumentId::derive("s3b-world"), Tol::witness());
    assert!(matches!(doc.node(RecipeNodeId::WORLD), Some(Node::World)));
    assert_eq!(doc.len(), 0, "the world is no node an author counts");
    let refused = editor_core::apply(
        &doc,
        &DocEdit::DeleteNode {
            id: RecipeNodeId::WORLD,
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    assert!(
        matches!(refused, Err(EditError::WorldIsUndeletable { .. })),
        "{refused:?}"
    );
    let second = fixture::insert_refused(&doc, Node::World);
    assert!(
        matches!(second, EditError::WorldIsUndeletable { .. }),
        "{second:?}"
    );
    let (doc, body) = block("s3b-world-reader");
    let read = fixture::insert_refused(
        &doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                body.into(),
                BodyRead::plain(Operand::Node(RecipeNodeId::WORLD)),
            ]),
        },
    );
    assert!(matches!(read, EditError::DefinesNothing { .. }), "{read:?}");
}

// ---- B1: an instance's copy side reads its own copy ----

/// **An instance's copy side reads its own output, and its target may
/// not**: the copy side is no pose dependency (it reads the part's face,
/// which the instance's motion carries), so a mate whose copy side is
/// the instance's own face admits; a mate whose target is the instance's
/// own output relates the copy to itself and refuses.
#[test]
fn an_instance_reads_its_own_copy_on_the_copy_side_alone() {
    let (doc, store, doc_ref, body) = host("s3b-own-copy");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    // `b`'s own start cap on the copy side, `a`'s on the target.
    let admitted = at_the_door_of(&doc, &reach, b, coaxial(body, a, b));
    assert!(admitted.is_ok(), "{admitted:?}");
    // `b`'s own output as the target, the copy itself on the copy side.
    let own = editor_core::Mate {
        a: editor_core::SideRead::Face(fixture::head(in_part(b, body, CapEnd::End)).into()),
        b: editor_core::SideRead::Copy,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: z_up(),
            b: MateFrame::on_part(Placement::IDENTITY),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    };
    match at_the_door_of(&doc, &reach, b, own) {
        Err((at, MateFault::SelfMate { instance, .. })) => {
            assert_eq!((at, instance), (b, b));
        }
        other => panic!("a mate of the copy to itself, not {other:?}"),
    }
}

// ---- Overconstraint ----

/// **A mate beyond the pin refuses at the door**: the second instance is
/// pinned by coaxial and seated, and a third mate names itself.
#[test]
fn a_mate_beyond_the_pin_refuses_at_the_door() {
    let (doc, store, doc_ref, body) = host("s3b-beyond");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = fixture::constrain_through(doc, &reach, b, coaxial(body, a, b));
    let (doc, _) = fixture::constrain_through(doc, &reach, b, seated(body, a, b));
    match at_the_door_of(&doc, &reach, b, coaxial(body, a, b)) {
        Err((at, MateFault::Overconstrained { placement, .. })) => {
            assert_eq!((at, placement), (b, b));
        }
        other => panic!("a mate beyond the pin refuses, not {other:?}"),
    }
}

/// **The solve names every constraint whose removal leaves the fold
/// unchanged, never "the later one"**: two identical coaxials, each
/// admitted against a prefix that pinned nothing, then the seat that
/// pins: the bundle is pinned, and either coaxial alone would have done.
#[test]
fn the_solve_names_every_redundant_constraint() {
    let (doc, store, doc_ref, body) = host("s3b-redundant");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, first) = fixture::constrain_through(doc, &reach, b, coaxial(body, a, b));
    let (doc, second) = fixture::constrain_through(doc, &reach, b, coaxial(body, a, b));
    let (doc, seat) = fixture::constrain_through(doc, &reach, b, seated(body, a, b));
    let ev = run(&doc, &o);
    match mate_fault(&ev, b) {
        MateFault::Overconstrained {
            placement,
            constraints,
            ..
        } => {
            assert_eq!(placement, b);
            assert_eq!(constraints, vec![first.id, second.id]);
            assert!(!constraints.contains(&seat.id));
        }
        other => panic!("overconstrained, not {other:?}"),
    }
}

// ---- The interims ----

/// **One bundle targets one copy** (the stage's named interim,
/// `work/intent/a-bundle-folds-in-the-joined-space.md`): a mate to a
/// second target refuses `TargetsDiffer` naming the bundle.
#[test]
fn a_bundle_of_two_targets_refuses() {
    let (doc, store, doc_ref, body) = host("s3b-two-targets");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, c) = insert(doc, mated_instance(doc_ref));
    let (doc, held) = constrain(doc, coaxial(body, a, c));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    match at_the_door_of(&doc, &reach, c, seated(body, b, c)) {
        Err((
            at,
            MateFault::TargetsDiffer {
                placement,
                constraints,
            },
        )) => {
            assert_eq!((at, placement), (c, c));
            assert_eq!(constraints.first(), Some(&held.id));
        }
        other => panic!("two targets refuse, not {other:?}"),
    }
}

/// **The pin is asked before the target**: on a copy its bundle
/// already pins (an instance's default world pin), a mate to another
/// copy is overconstrained, not a second target. The order the
/// unpinned bundle above meets is the other: `TargetsDiffer`.
#[test]
fn a_mate_on_a_pinned_copy_is_overconstrained_whatever_it_targets() {
    let (doc, store, doc_ref, body) = host("s3b-pin-first");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    match at_the_door_of(&doc, &reach, b, coaxial(body, a, b)) {
        Err((at, MateFault::Overconstrained { placement, .. })) => {
            assert_eq!((at, placement), (b, b));
        }
        other => panic!("a mate on a pinned copy is overconstrained, not {other:?}"),
    }
}

/// **A constraint has a side on its copy**: one relating two other
/// copies refuses `NotOnTheCopy`.
#[test]
fn a_constraint_off_its_copy_refuses() {
    let (doc, store, doc_ref, body) = host("s3b-off-copy");
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, c) = insert(doc, mated_instance(doc_ref));
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    match at_the_door_of(&doc, &reach, c, coaxial(body, a, b)) {
        Err((at, MateFault::NotOnTheCopy { mate })) => {
            assert_eq!((at, mate.placement), (c, c));
        }
        other => panic!("a constraint off its copy refuses, not {other:?}"),
    }
}

// ---- An instance's copies ----

/// **A construction reads one copy of an instance by its key**, and a
/// raw construction stands at the identity of the document's one kind,
/// so a union of an instance's copy and a block of this document
/// evaluates (the stage's interim until D).
#[test]
fn a_union_of_an_instances_copy_and_a_block_evaluates() {
    let (doc, store, doc_ref, part_body) = host("s3b-union");
    let (doc, instance) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, profile) = on_frame(
        doc,
        [3.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, other) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                fixture::copy(instance, part_body),
                other.into(),
            ]),
        },
    );
    let ev = run(&doc, &with_resolver(store));
    assert!(
        matches!(ev.result(union), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(union)
    );
}

/// **A whole read of an instance's copies at a body seat refuses at the
/// door, and a key the part delivers nothing at refuses at evaluation**,
/// typed, never re-pointed.
#[test]
fn an_instances_copies_are_read_one_by_key() {
    let (doc, store, doc_ref, part_body) = host("s3b-keyed");
    let (doc, instance) = insert(doc, Node::instantiate_part(doc_ref));
    let whole = fixture::insert_refused(
        &doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                instance.into(),
                fixture::copy(instance, part_body),
            ]),
        },
    );
    assert!(matches!(whole, EditError::SlotVarKind { .. }), "{whole:?}");
    let missing = CopyKey::part(RecipeNodeId::new(0, 1), copy_key(part_body));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                BodyRead::keyed(Operand::Node(instance), missing.clone()),
                fixture::copy(instance, part_body),
            ]),
        },
    );
    let ev = run(&doc, &with_resolver(store));
    match ev.result(union) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::NoSuchCopy { input, key } => {
                assert_eq!((*input, key.as_ref()), (instance, &missing));
            }
            other => panic!("no such copy, not {other:?}"),
        },
        other => panic!("a refusal, not {other:?}"),
    }
}

/// **A `Place` over several shapes defines one output, its copies keyed
/// by read**: a read of one evaluates to that copy, and two reads of one
/// family key alike and refuse (FORK-DM4).
#[test]
fn a_place_of_several_shapes_keys_its_copies_by_read() {
    let (doc, first) = block("s3b-place-several");
    let (doc, profile) = on_frame(
        doc,
        [3.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, second) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, place) = step(
        doc,
        DocEdit::place(
            vec![first.into(), second.into()],
            vec![editor_core::Mate::world_pin(Placement::IDENTITY)],
        ),
    );
    let place = place.expect("a placement mints its node");
    assert_eq!(
        doc.outputs(place).len(),
        1,
        "one output, however many shapes"
    );
    let second_read = doc.output(second, 0).expect("the block's body");
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                BodyRead::keyed(Operand::Node(place), CopyKey::Read(second_read)),
                first.into(),
            ]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    assert!(
        matches!(ev.result(union), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(union)
    );
    assert_eq!(product_solids(&doc, &ev), 2);

    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: first.into(),
            count: Formula::count(2),
            kind: editor_core::PatternKind::Linear {
                direction: [fixture::scl(0.0), fixture::scl(1.0), fixture::scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    let (doc, twice) = step(
        doc,
        DocEdit::place(
            vec![
                BodyRead::indexed(Operand::Node(pattern), vec![Formula::count(0)]),
                BodyRead::indexed(Operand::Node(pattern), vec![Formula::count(1)]),
            ],
            vec![editor_core::Mate::world_pin(Placement::IDENTITY)],
        ),
    );
    let twice = twice.expect("a placement mints its node");
    let ev = run(&doc, &EvalOptions::default());
    match ev.result(twice) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(e.kind, NodeErrorKind::CopiesShareAKey { .. }),
            "{:?}",
            e.kind
        ),
        other => panic!("two reads of one family refuse, not {other:?}"),
    }
}

// ---- Carried ----

/// **A carried pose is the pose its copy carries**: a block's top plane,
/// carried by a placement three units along x, is the top plane moved.
#[test]
fn a_carried_pose_moves_with_its_copy() {
    let (doc, body) = block("s3b-carried");
    let (doc, placement) = step(
        doc,
        DocEdit::place_in_world(
            Operand::Node(body),
            Some(Placement::literal(&editor_core::Frame::translation([
                3.0, 0.0, 0.0,
            ]))),
        ),
    );
    let placement = placement.expect("a placement mints its node");
    let top = fixture::fname(body, RoleSeg::Cap(CapEnd::End));
    let plane = |carried: bool| {
        let face = Operand::Pose(Box::new(PoseDef::Plane {
            face: Operand::select(Operand::output(body, 0), vec![top.clone()]),
        }));
        if carried {
            Operand::Pose(Box::new(PoseDef::Carried {
                copy: Operand::output(placement, 0),
                pose: face,
            }))
        } else {
            face
        }
    };
    let mut origins = Vec::new();
    for carried in [false, true] {
        let (doc, split) = insert(
            doc.clone(),
            Node::Split {
                target: Operand::output(placement, 0).into(),
                tool: plane(carried),
            },
        );
        let Some(Node::Split { tool, .. }) = doc.node(split) else {
            panic!("a split")
        };
        let tool = *tool;
        let ev = run(&doc, &EvalOptions::default());
        let value = bound_pose(&doc, &ev, OperandSlot::Tool, tool, Tol::witness());
        let Ok(PoseValue::Plane { origin, normal }) = value else {
            panic!("a plane, not {value:?}")
        };
        assert_eq!(UnitVec3::get(normal).z, 1.0, "the top's outward normal");
        origins.push(origin);
    }
    assert_eq!(origins[1].x - origins[0].x, 3.0, "moved with its copy");
    assert_eq!(origins[1].z, origins[0].z, "and no further");
}
