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

use crate::fixture::resolver::{PartStore, copy_key, placement_of, with_resolver};
use crate::fixture::{self, at_the_door_of, insert, len, mated_instance, on_frame, run, square, step};
use editor_core::eval::bound_pose;
use editor_core::pose::{PoseDef, PoseRole};
use editor_core::{
    BodyRead, CapEnd, ContactClass, CopyKey, DocEdit, DocumentId, EditError, EvalOptions,
    ExtrudeSide, Formula, Frame, MateFault, MateRef, Node, NodeErrorKind, NodeResult, Operand,
    OperandSlot, PoseValue, ProductError, ProfileDoc, RecipeNodeId, RoleSeg, Space, StableName,
    Target, WorldRead,
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

/// **Face `seg` of the part's block, as instance `instance` names it**:
/// the block's face under its part placement's copy, worn inside the
/// instance's `InPart`.
fn face_in(instance: RecipeNodeId, body: RecipeNodeId, seg: RoleSeg) -> StableName {
    let own = StableName {
        kind: editor_core::EntityKind::Face,
        node: body,
        path: vec![seg],
    };
    editor_core::FaceName::new(StableName {
        kind: editor_core::EntityKind::Face,
        node: placement_of(body),
        path: vec![RoleSeg::Placed {
            of: editor_core::NameRef::new(own),
        }],
    })
    .expect("a face")
    .in_part(instance)
    .into_name()
}

/// `b`'s bottom resting on `a`'s top: the rest a stack starts from,
/// leaving `b` free in the plane.
fn rest_on(body: RecipeNodeId, a: RecipeNodeId, b: RecipeNodeId) -> editor_core::Mate<Formula> {
    fixture::rest(
        fixture::head(face_in(b, body, RoleSeg::Cap(CapEnd::Start))),
        fixture::head(face_in(a, body, RoleSeg::Cap(CapEnd::End))),
        ContactClass::Rest,
    )
}

/// `b`'s wall `seg` flush with `a`'s: the two planes equated, outward
/// normals agreeing.
fn flush(
    body: RecipeNodeId,
    a: RecipeNodeId,
    b: RecipeNodeId,
    seg: RoleSeg,
) -> editor_core::Mate<Formula> {
    fixture::mate(
        fixture::plane(fixture::head(face_in(b, body, seg.clone()))),
        fixture::plane(fixture::head(face_in(a, body, seg))),
        ContactClass::Rest,
    )
}

/// **The two walls of the part's block a stack makes flush**: two
/// adjacent sides, whose planes with the rest pin the copy.
fn walls(label: &str) -> [RoleSeg; 2] {
    let (doc, body) = block(&format!("{label}-part"));
    [fixture::wall(&doc, body, 0), fixture::wall(&doc, body, 1)]
}

/// **`b` stacked on `a`**, square: `b` resting on `a`'s top and its two
/// walls flush with `a`'s — three plane mates that pin `b`, each
/// admitted to `b`'s bundle; their addresses, the rest first.
fn stack(
    doc: ProfileDoc,
    label: &str,
    body: RecipeNodeId,
    a: RecipeNodeId,
    b: RecipeNodeId,
) -> (ProfileDoc, [MateRef; 3]) {
    let [x, y] = walls(label);
    let (doc, rest) = fixture::constrain_at(doc, b, rest_on(body, a, b));
    let (doc, wx) = fixture::constrain_at(doc, b, flush(body, a, b, x));
    let (doc, wy) = fixture::constrain_at(doc, b, flush(body, a, b, y));
    (doc, [rest, wx, wy])
}

/// **One copy of `body` in the world at `pose`**: a placement of it,
/// pinned there over its own geometry.
fn place_at(doc: ProfileDoc, body: RecipeNodeId, pose: &Frame) -> (ProfileDoc, RecipeNodeId) {
    let (doc, placement) = insert(doc, Node::place(vec![body.into()]));
    (
        fixture::pin(doc, placement, pose, &EvalOptions::default()),
        placement,
    )
}

/// **A zero value on each freedom a plane mate leaves**: both slides
/// and the spin, so the copy's own coordinates agree with its target's
/// as far as the mate allows.
fn zero_values(doc: ProfileDoc, mate: MateRef) -> ProfileDoc {
    [
        (editor_core::Coord::Slide(editor_core::SlideAxis::X), len(0.0)),
        (editor_core::Coord::Slide(editor_core::SlideAxis::Y), len(0.0)),
        (editor_core::Coord::Spin, fixture::ang(0.0)),
    ]
    .into_iter()
    .fold(doc, |doc, (coord, by)| {
        step(
            doc,
            DocEdit::AddValue {
                placement: mate.placement,
                value: Box::new(editor_core::Value {
                    mate: mate.id,
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
        )
        .0
    })
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
    let (doc, second) = place_at(doc, body, &Frame::translation([3.0, 0.0, 0.0]));
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
/// product**: the second instance, stacked on the first, which is
/// pinned to the world.
#[test]
fn a_copy_pinned_against_a_pinned_copy_is_in_the_product() {
    let (doc, store, doc_ref, body) = host("s3b-chain");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, _) = stack(doc, "s3b-chain", body, a, b);
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
    let (doc, _) = stack(doc, "s3b-loose-pair", body, a, b);
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
    // A copy's bottom on a face, square to its target's coordinates:
    // the plane mate and a zero value on each freedom it leaves.
    let pin_on = |doc: ProfileDoc, at: RecipeNodeId, body: RecipeNodeId, target: Operand| {
        let (doc, mate) = fixture::constrain_at(
            doc,
            at,
            fixture::mate(
                fixture::plane(fixture::head(fixture::fname(
                    body,
                    RoleSeg::Cap(CapEnd::Start),
                ))),
                fixture::plane(target),
                ContactClass::Rest,
            ),
        );
        zero_values(doc, mate)
    };
    let (doc, l2) = insert(doc, Node::place(vec![b.into()]));
    let doc = pin_on(
        doc,
        l2,
        b,
        Operand::select(Operand::output(l, 0), vec![placed_cap]),
    );
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
    let face = crate::fixture::table(&ev, s)
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| n.kind == editor_core::EntityKind::Face)
        .expect("the subtract names a face");
    let (doc, p) = insert(doc, Node::place(vec![c.into()]));
    let doc = pin_on(doc, p, c, Operand::select(Operand::output(s, 0), vec![face]));
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

/// **A copy of no instance forms no lever its constraints do not ask
/// for**: a `Place` copy resting on the world's xy plane decides nothing
/// over a lever, so its bundle never asks the parts' reach, which has no
/// part to answer for it. The copy stands, loose in the rest's plane.
#[test]
fn a_place_copy_resting_on_the_world_asks_no_lever() {
    let (doc, body) = block("s3b-world-rest");
    let rest = editor_core::Mate {
        on: fixture::plane(fixture::head(fixture::fname(
            body,
            RoleSeg::Cap(CapEnd::Start),
        ))),
        to: Target::World(WorldRead::Flip {
            pose: Box::new(WorldRead::through(PoseRole::Xy)),
        }),
        class: ContactClass::Rest,
    };
    let (doc, place) = step(doc, DocEdit::place(vec![body.into()], vec![rest]));
    let place = place.expect("the placement mints");
    let ev = run(&doc, &EvalOptions::default());
    assert!(
        matches!(ev.result(place), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(place)
    );
    assert_eq!(ev.space(place), Space::Loose { root: place });
}

/// **A copy its bundle pins less than its body needs is a loose copy,
/// not a refusal**: a rest alone leaves the plane.
#[test]
fn an_under_pinned_copy_is_loose_not_refused() {
    let (doc, store, doc_ref, body) = host("s3b-under");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, _) = fixture::constrain(doc, rest_on(body, a, b));
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

// ---- B1: an instance's `on` reads its own copy ----

/// **An instance's `on` reads its own output, and its target may
/// not**: `on` is no pose dependency (it reads the part's face, which
/// the instance's motion carries), so a mate whose `on` is the
/// instance's own face admits; a mate whose target is the instance's
/// own output relates the copy to itself, a read of its own output the
/// door refuses as a cycle.
#[test]
fn an_instance_reads_its_own_copy_on_the_copy_side_alone() {
    let (doc, store, doc_ref, body) = host("s3b-own-copy");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    // `b`'s own start cap on `on`, `a`'s top on the target.
    let admitted = at_the_door_of(&doc, &editor_core::RefusingReach, b, rest_on(body, a, b));
    assert!(admitted.is_ok(), "{admitted:?}");
    // `b`'s own top as the target.
    let own = fixture::rest(
        fixture::head(face_in(b, body, RoleSeg::Cap(CapEnd::Start))),
        fixture::head(face_in(b, body, RoleSeg::Cap(CapEnd::End))),
        ContactClass::Rest,
    );
    match editor_core::apply(
        &doc,
        &DocEdit::AddConstraint {
            placement: b,
            mate: Box::new(own),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::WouldCycle { at }) => assert_eq!(at.id(), b),
        other => panic!("a mate of the copy to itself, not {other:?}"),
    }
}

// ---- Overconstraint ----

/// **A mate beyond the pin refuses where its placement evaluates**: the
/// second instance is pinned by its stack, and a second rest beyond it
/// is named with the rest it repeats.
#[test]
fn a_mate_beyond_the_pin_refuses_where_it_evaluates() {
    let (doc, store, doc_ref, body) = host("s3b-beyond");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, [rest, ..]) = stack(doc, "s3b-beyond", body, a, b);
    let (doc, beyond) = fixture::constrain_at(doc, b, rest_on(body, a, b));
    let ev = run(&doc, &o);
    match mate_fault(&ev, b) {
        MateFault::Overconstrained {
            placement,
            constraints,
            ..
        } => {
            assert_eq!(placement, b);
            assert_eq!(constraints, vec![rest.id, beyond.id]);
        }
        other => panic!("a mate beyond the pin refuses, not {other:?}"),
    }
}

/// **The solve names every constraint whose removal leaves the fold
/// unchanged, never "the later one"**: two identical rests, then the
/// two flush walls that pin: the bundle is pinned, and either rest
/// alone would have done.
#[test]
fn the_solve_names_every_redundant_constraint() {
    let (doc, store, doc_ref, body) = host("s3b-redundant");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, first) = fixture::constrain_at(doc, b, rest_on(body, a, b));
    let (doc, second) = fixture::constrain_at(doc, b, rest_on(body, a, b));
    let [x, y] = walls("s3b-redundant");
    let (doc, wx) = fixture::constrain_at(doc, b, flush(body, a, b, x));
    let (doc, wy) = fixture::constrain_at(doc, b, flush(body, a, b, y));
    let ev = run(&doc, &o);
    match mate_fault(&ev, b) {
        MateFault::Overconstrained {
            placement,
            constraints,
            ..
        } => {
            assert_eq!(placement, b);
            assert_eq!(constraints, vec![first.id, second.id]);
            assert!(!constraints.contains(&wx.id) && !constraints.contains(&wy.id));
        }
        other => panic!("overconstrained, not {other:?}"),
    }
}

// ---- The interims ----

/// **One bundle targets one space** (the stage's named interim,
/// `work/intent/a-bundle-folds-in-the-joined-space.md`): a bundle with
/// a rest on a copy in the world and one on a loose copy refuses
/// `TargetsDiffer` naming the bundle where it evaluates.
#[test]
fn a_bundle_of_two_targets_refuses() {
    let (doc, store, doc_ref, body) = host("s3b-two-targets");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, c) = insert(doc, mated_instance(doc_ref));
    let (doc, held) = fixture::constrain_at(doc, c, rest_on(body, a, c));
    let (doc, _) = fixture::constrain_at(doc, c, rest_on(body, b, c));
    let ev = run(&doc, &o);
    match mate_fault(&ev, c) {
        MateFault::TargetsDiffer {
            placement,
            constraints,
        } => {
            assert_eq!(placement, c);
            assert_eq!(constraints.first(), Some(&held.id));
        }
        other => panic!("two targets refuse, not {other:?}"),
    }
}

/// **A mate on a pinned copy is overconstrained**: on a copy its world
/// pin already pins, a rest on another copy in the world that holds
/// where the pin puts it is a mate the pin already fixes, named where
/// the copy evaluates.
#[test]
fn a_mate_on_a_pinned_copy_is_overconstrained() {
    let (doc, store, doc_ref, body) = host("s3b-pin-first");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
    // `b` pinned where the rest puts it, on `a`'s top.
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    let doc = fixture::pin(doc, b, &Frame::translation([0.0, 0.0, 1.0]), &o);
    let (doc, rest) = fixture::constrain_at(doc, b, rest_on(body, a, b));
    let ev = run(&doc, &o);
    match mate_fault(&ev, b) {
        MateFault::Overconstrained {
            placement,
            constraints,
            ..
        } => {
            assert_eq!(placement, b);
            assert_eq!(constraints, vec![rest.id]);
        }
        other => panic!("a mate on a pinned copy is overconstrained, not {other:?}"),
    }
}

/// **A constraint has its `on` on its copy**: one relating two other
/// copies refuses `NotOnTheCopy` at the door.
#[test]
fn a_constraint_off_its_copy_refuses() {
    let (doc, store, doc_ref, body) = host("s3b-off-copy");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, c) = insert(doc, mated_instance(doc_ref));
    match at_the_door_of(&doc, &editor_core::RefusingReach, c, rest_on(body, a, b)) {
        Err((at, MateFault::NotOnTheCopy { mate })) => {
            assert_eq!((at, mate.placement), (c, c));
        }
        other => panic!("a constraint off its copy refuses, not {other:?}"),
    }
}

// ---- An instance's copies ----

/// **A construction reads one copy of an instance by its key**, and a
/// raw construction stands at the identity of the document's one kind,
/// so a union of a pinned instance's copy and a block of this document
/// evaluates (the stage's interim until D).
#[test]
fn a_union_of_an_instances_copy_and_a_block_evaluates() {
    let (doc, store, doc_ref, part_body) = host("s3b-union");
    let o = with_resolver(store);
    let (doc, instance) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
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
    let ev = run(&doc, &o);
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
    let o = with_resolver(store);
    let (doc, instance) = fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o);
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
    let ev = run(&doc, &o);
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

/// **One copy of a nested instance is placed by the outer motion after
/// the inner one**: a part placed in a middle document at a translation,
/// the middle document instanced in an outer one at a quarter turn, and
/// a reader of that one copy sees the block turned about the outer
/// origin after it was moved, never moved after it was turned.
#[test]
fn a_nested_instances_copy_is_placed_outer_after_inner() {
    let mut store = PartStore::default();
    let (leaf, leaf_body) = store.insert_part(block("s3b-nested-leaf"), Tol::witness());
    let mid = ProfileDoc::empty(DocumentId::derive("s3b-nested-mid"), Tol::witness());
    let (mid, inner) = insert(
        mid,
        Node::instantiate_part_with(leaf, editor_core::InterfaceRecord::default()),
    );
    let mid = fixture::pin(
        mid,
        inner,
        &Frame::translation([5.0, 0.0, 0.0]),
        &with_resolver(store.clone()),
    );
    let mid_ref = store.insert(mid, Tol::witness());
    let top = ProfileDoc::empty(DocumentId::derive("s3b-nested-top"), Tol::witness());
    let (top, outer) = insert(
        top,
        Node::instantiate_part_with(mid_ref, editor_core::InterfaceRecord::default()),
    );
    let quarter_turn = Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };
    // The reader copies the one copy where the outer instance puts it:
    // one pin, read off that copy while the instance is loose, holds
    // the instance at the quarter turn and the reader where it reads.
    let at_turn = editor_core::pin_at(
        &top,
        &Operand::Var(top.output(outer, 0).expect("an instance defines its copies")),
        &quarter_turn,
        &with_resolver(store.clone()),
        Tol::witness(),
    )
    .unwrap_or_else(|refusal| panic!("the copy pins over its geometry: {refusal}"));
    let (top, _) = fixture::constrain_at(top, outer, at_turn.clone());
    let key = CopyKey::part(inner, copy_key(leaf_body));
    let (top, reader) = insert(
        top,
        Node::place(vec![BodyRead::keyed(Operand::Node(outer), key)]),
    );
    let (top, _) = fixture::constrain_at(top, reader, at_turn);
    let ev = run(&top, &with_resolver(store));
    let body = match ev.value(reader).map(|v| &v.payload) {
        Some(editor_core::ValuePayload::Body(b)) => b.clone(),
        other => panic!("one copy reads one body, not {other:?}"),
    };
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for (_, p) in body.points() {
        for (k, c) in p.to_array().into_iter().enumerate() {
            lo[k] = lo[k].min(c);
            hi[k] = hi[k].max(c);
        }
    }
    // Moved to x ∈ [4.5, 5.5], then turned: x ∈ [-0.5, 0.5],
    // y ∈ [4.5, 5.5]. Turned first and moved after, it would stand at
    // x ∈ [4.5, 5.5], y ∈ [-0.5, 0.5].
    let near = |a: [f64; 3], b: [f64; 3]| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-12);
    assert!(
        near(lo, [-0.5, 4.5, 0.0]) && near(hi, [0.5, 5.5, 1.0]),
        "the copy stands at {lo:?}..{hi:?}"
    );
}

/// **A `Place` over several shapes defines one output, its copies keyed
/// by read**: a read of one evaluates to that copy, and two reads of one
/// family key alike and refuse (FORK-DM4), before any mate is asked.
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
    let (doc, place) = insert(doc, Node::place(vec![first.into(), second.into()]));
    let doc = fixture::pinned(doc, place, &EvalOptions::default());
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
    let (doc, twice) = insert(
        doc,
        Node::place(vec![
            BodyRead::indexed(Operand::Node(pattern), vec![Formula::count(0)]),
            BodyRead::indexed(Operand::Node(pattern), vec![Formula::count(1)]),
        ]),
    );
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
    let (doc, placement) = place_at(doc, body, &Frame::translation([3.0, 0.0, 0.0]));
    let top = fixture::fname(body, RoleSeg::Cap(CapEnd::End));
    let plane = |carried: bool| {
        let face = Operand::Pose(Box::new(PoseDef::Plane {
            face: Operand::select(Operand::output(body, 0), vec![top.clone()]),
        }));
        if carried {
            Operand::Pose(Box::new(PoseDef::Carried {
                copy: Operand::output(placement, 0),
                pose: face.into(),
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
                tool: plane(carried).into(),
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
