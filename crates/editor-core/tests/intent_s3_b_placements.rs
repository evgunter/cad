//! INTENT stage 3 PR B: a placement is the bundle of mates
//! (`docs/INTENT-STAGE3-SPEC.md` §3, §10 rows 5, 6 and 12).
//!
//! A copy is made by a placement — a `Place` of the shapes it reads, or
//! an instance of a part — and pinned by its bundle of constraints, each
//! addressed by the placement and the id minted with it, and placed into
//! the space it names. A copy placed into the world is in the product;
//! one placed into another space is in that space. The rows here pin the
//! representation; the solve's arithmetic is the mate suites'.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::resolver::{PartName, PartStore, with_resolver};
use crate::fixture::{
    self, at_the_door_of, insert, len, mated_instance, on_frame, run, square, step,
};
use editor_core::Dimension;
use editor_core::eval::bound_pose;
use editor_core::pose::PoseDef;
use editor_core::{
    BodyRead, CapEnd, DocEdit, DocumentId, EditError, EvalOptions, Formula, Frame, MateFault,
    MateRef, Node, NodeErrorKind, NodeResult, Operand, OperandSlot, PoseValue, ProductError,
    ProfileDoc, RecipeNodeId, RoleSeg, VarId,
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

/// **Face `seg` of the block, read at the copy `instance` carries it
/// in**: the block's own name, which every copy of it carries verbatim.
fn face_in(instance: RecipeNodeId, body: RecipeNodeId, seg: RoleSeg) -> PartName {
    fixture::resolver::in_part_of(editor_core::EntityKind::Face, instance, body, seg)
}

/// `b`'s bottom resting on `a`'s top: the rest a stack starts from,
/// leaving `b` free in the plane.
fn rest_on(body: RecipeNodeId, a: RecipeNodeId, b: RecipeNodeId) -> editor_core::Mate<Formula> {
    fixture::rest(
        fixture::head(face_in(b, body, RoleSeg::Cap(CapEnd::Start))),
        fixture::head(face_in(a, body, RoleSeg::Cap(CapEnd::End))),
    )
}

/// **`b` stacked on `a`**, square: `b` resting on `a`'s top, with a
/// zero on each freedom the rest leaves, so `b`'s own coordinates agree
/// with `a`'s as far as the rest allows — the rest's address.
fn stack(
    doc: ProfileDoc,
    body: RecipeNodeId,
    a: RecipeNodeId,
    b: RecipeNodeId,
) -> (ProfileDoc, MateRef) {
    let (doc, minted) = fixture::write_bundle(doc, b, zeroed(rest_on(body, a, b)));
    (doc, minted[0])
}

/// **One copy of `body` in the world at `pose`**: a placement of it,
/// pinned there over its own geometry.
fn place_by_values(
    doc: ProfileDoc,
    body: RecipeNodeId,
    pose: &Frame,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, placement) = insert(doc, crate::fixture::stood(body));
    (
        fixture::assemble_world_copies(fixture::pin(doc, placement, pose, &EvalOptions::default())),
        placement,
    )
}

/// **A plane mate and a zero value on each freedom it leaves**: both
/// slides and the spin, so the copy's own coordinates agree with its
/// target's as far as the mate allows. The whole bundle.
fn zeroed(mate: editor_core::Mate<Formula>) -> Vec<editor_core::Constraint<Formula>> {
    editor_core::Placed {
        mates: vec![mate],
        values: vec![
            (
                editor_core::Coord::Slide(editor_core::TargetAxis::X),
                len(0.0),
            ),
            (
                editor_core::Coord::Slide(editor_core::TargetAxis::Y),
                len(0.0),
            ),
            (editor_core::Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    }
    .bundle()
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
    let (doc, second) = place_by_values(doc, body, &Frame::translation([3.0, 0.0, 0.0]));
    assert_ne!(
        doc.output(first, 0),
        doc.output(second, 0),
        "one output each"
    );
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(product_solids(&doc, &ev), 2);
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    let cap = fixture::fname(body, RoleSeg::Cap(CapEnd::End));
    // The product's `⊔` keys each copy's names by the copy.
    for placement in [first, second] {
        let copied = fixture::product_name(&doc, &ev, (placement, cap.clone()));
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
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, _) = stack(doc, body, a, b);
    assert_eq!(doc.node_space(b), Some(VarId::WORLD));
    let ev = run(&doc, &o);
    assert_eq!(product_solids(&doc, &ev), 2);
}

/// **Two copies placed into a `new_space()` are in that space**: the
/// first at the identity there, the second stacked on it, so both are
/// in the new space, derived from the recipe, and neither is in the
/// product, which `EmptyProduct` refuses.
#[test]
fn copies_placed_into_a_new_space_are_in_it_not_the_world() {
    let (doc, store, doc_ref, body) = host("s3b-new-space-pair");
    let (doc, space) = insert(doc, Node::Space);
    let (doc, a) = insert(
        doc,
        Node::instantiate_part(doc_ref, Operand::Node(space), editor_core::stand().bundle()),
    );
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let o = with_resolver(store);
    let (doc, _) = stack(doc, body, a, b);
    let space = doc.output(space, 0).expect("a new space defines its space");
    assert_eq!(doc.node_space(a), Some(space));
    assert_eq!(
        doc.node_space(b),
        Some(space),
        "joins the space it mates into"
    );
    let ev = run(&doc, &o);
    for copy in [a, b] {
        assert!(
            matches!(ev.result(copy), Some(NodeResult::Ok(_))),
            "{:?}",
            ev.result(copy)
        );
    }
    assert!(
        matches!(
            editor_core::product_recorded(&doc, &ev, Tol::witness()),
            Err(ProductError::EmptyProduct)
        ),
        "an empty world"
    );
}

/// **A copy pinned against a body of another space joins that space**:
/// `L` places a block into a `new_space()`, a subtract of two copies in
/// that space is in it, and a copy pinned against one of its faces joins
/// it too. None is in the world, and the product refuses
/// `EmptyProduct`.
#[test]
fn a_copy_pinned_against_a_body_of_another_space_is_in_that_space() {
    let doc = ProfileDoc::empty(DocumentId::derive("s3b-space-construction"), Tol::witness());
    let (doc, a) = block_at(doc, 0.5);
    let (doc, b) = block_at(doc, 0.2);
    let (doc, c) = block_at(doc, 0.1);
    let (doc, space) = insert(doc, Node::Space);
    let (doc, l) = step(
        doc,
        DocEdit::place(a, Operand::Node(space), editor_core::stand().bundle()),
    );
    let l = l.expect("the placement mints");
    let space = doc.output(space, 0).expect("a new space defines its space");
    let cap = fixture::fname(a, RoleSeg::Cap(CapEnd::Start));
    let placed_cap = cap;
    // A copy's bottom on a face, square to its target's coordinates
    // under `orient`: the plane mate and a zero value on each freedom it
    // leaves.
    let pin_on = |doc: ProfileDoc,
                  at: RecipeNodeId,
                  body: RecipeNodeId,
                  target: Operand,
                  orient: editor_core::Orientation| {
        let mut mate = fixture::mate(
            fixture::plane(fixture::head(fixture::fname(
                body,
                RoleSeg::Cap(CapEnd::Start),
            ))),
            fixture::plane(target),
        );
        mate.orient = orient;
        fixture::write_bundle(doc, at, zeroed(mate)).0
    };
    let (doc, l2) = insert(doc, crate::fixture::stood(b));
    let doc = pin_on(
        doc,
        l2,
        b,
        Operand::select(Operand::output(l, 0), vec![placed_cap]),
        editor_core::Orientation::IDENTITY,
    );
    let (doc, s) = insert(
        doc,
        Node::Subtract {
            from: Operand::output(l, 0).into(),
            tool: Operand::output(l2, 0).into(),
        },
    );
    assert_eq!(
        doc.node_space(s),
        Some(space),
        "the subtract is in L's space"
    );
    let ev = run(&doc, &EvalOptions::default());
    let face = crate::fixture::table(&ev, s)
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| n.kind == editor_core::EntityKind::Face)
        .expect("the subtract names a face");
    let (doc, p) = insert(doc, crate::fixture::stood(c));
    let doc = pin_on(
        doc,
        p,
        c,
        Operand::select(Operand::output(s, 0), vec![face]),
        // The face faces up and the copy's bottom down: the copy upended.
        editor_core::Orientation::new(
            editor_core::SignedAxis::PlusX,
            editor_core::SignedAxis::MinusZ,
        )
        .expect("x and z lie along two axes"),
    );
    assert_eq!(
        doc.node_space(p),
        Some(space),
        "in L's space, not the world"
    );
    let ev = run(&doc, &EvalOptions::default());
    assert!(
        matches!(ev.result(p), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(p)
    );
    assert!(
        matches!(
            editor_core::product_recorded(&doc, &ev, Tol::witness()),
            Err(ProductError::EmptyProduct)
        ),
        "an empty world"
    );
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
        },
    )
}

/// **A copy of no instance forms no lever its constraints do not ask
/// for**: a `Place` copy resting on a floor's top decides nothing over a
/// lever, so its bundle never asks the parts' reach, which has no part
/// to answer for it. The copy stands in the world.
#[test]
fn a_place_copy_resting_on_the_world_asks_no_lever() {
    let (doc, body) = block("s3b-world-rest");
    let (doc, top, _) = fixture::floor(doc, Vec::new());
    let rest = editor_core::Mate {
        on: fixture::plane(fixture::head(fixture::fname(
            body,
            RoleSeg::Cap(CapEnd::Start),
        ))),
        to: fixture::flip(top),
        orient: editor_core::Orientation::IDENTITY,
    };
    let (doc, place) = step(
        doc,
        DocEdit::place(body, Operand::Var(VarId::WORLD), zeroed(rest)),
    );
    let place = place.expect("the placement mints");
    let ev = run(&doc, &EvalOptions::default());
    assert!(
        matches!(ev.result(place), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(place)
    );
    assert_eq!(doc.node_space(place), Some(VarId::WORLD));
}

/// **A bundle that pins less than its copy needs refuses at the door**
/// (D10 `[placement.placement]`; spec §10 row 18): a rest alone leaves
/// the plane, so writing it refuses `Unpinned` with the three freedoms
/// it leaves, and an empty bundle refuses with all six.
#[test]
fn an_under_pinned_bundle_refuses_at_the_door() {
    let (doc, store, doc_ref, body) = host("s3b-under");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let free = |edit: &DocEdit<editor_core::ProfileProgram>| match editor_core::apply(
        &doc,
        edit,
        Tol::witness(),
    ) {
        Err(EditError::MateRefused { fault, .. }) => match *fault {
            MateFault::Unpinned { placement, free } => {
                assert_eq!(placement, b);
                free
            }
            other => panic!("Unpinned, not {other:?}"),
        },
        other => panic!("a refusal at the door, not {other:?}"),
    };
    let rest = DocEdit::SetConstraints {
        placement: b,
        constraints: vec![editor_core::Constraint::of_mate(
            editor_core::ConstraintId::placeholder(1),
            rest_on(body, a, b),
        )],
        fresh: Vec::new(),
    };
    assert_eq!(free(&rest), 3, "a rest leaves two slides and a spin");
    let empty = DocEdit::SetConstraints {
        placement: b,
        constraints: Vec::new(),
        fresh: Vec::new(),
    };
    assert_eq!(free(&empty), 6, "an empty bundle pins nothing");
}

// ---- Row 12: the world ----

/// **The world is undeletable, minted with every document, and read by
/// placements alone**: deleting it refuses, a second one refuses, and a
/// construction cannot name it, since it defines only its space, which no
/// body seat reads.
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
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    // `b`'s own start cap on `on`, `a`'s top on the target.
    let whole = |mate| DocEdit::SetConstraints {
        placement: b,
        constraints: zeroed(mate),
        fresh: Vec::new(),
    };
    let admitted = editor_core::apply(&doc, &whole(rest_on(body, a, b)), Tol::witness());
    assert!(admitted.is_ok(), "{admitted:?}");
    // `b`'s own top as the target.
    let own = fixture::rest(
        fixture::head(face_in(b, body, RoleSeg::Cap(CapEnd::Start))),
        fixture::head(face_in(b, body, RoleSeg::Cap(CapEnd::End))),
    );
    match editor_core::apply(&doc, &whole(own), Tol::witness()) {
        Err(EditError::WouldCycle { at }) => assert_eq!(at.id(), b),
        other => panic!("a mate of the copy to itself, not {other:?}"),
    }
}

// ---- Overconstraint ----

/// **A mate beyond the pin refuses at the door**: the second instance
/// is pinned by its stack (a rest and its three values), and a second
/// rest beyond it fixes what they fix. Measuring nothing, the door names
/// the mates that repeat an equation, both rests, and none of the
/// values, which live inside what the mates leave.
#[test]
fn a_mate_beyond_the_pin_refuses_at_the_door() {
    let (doc, store, doc_ref, body) = host("s3b-beyond");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, rest) = stack(doc, body, a, b);
    match fixture::door_refusal(&doc, rest_on(body, a, b)) {
        MateFault::Overconstrained {
            placement,
            constraints,
            free,
        } => {
            assert_eq!(placement, b);
            assert!(constraints.contains(&rest.id), "{constraints:?}");
            assert_eq!(
                constraints.len(),
                2,
                "the two rests, and none of the values inside what they leave: \
                 {constraints:?}"
            );
            assert!(
                fixture::bundle(&doc, b).len() > 1,
                "the premise: values beside the rest"
            );
            assert_eq!(
                free, 0,
                "the bundle is written whole: the slide two planes in general position \
                 leave is written 0"
            );
        }
        other => panic!("a mate beyond the pin refuses, not {other:?}"),
    }
}

/// **The solve names every constraint any of whose equations the rest
/// already fix, never "the later one"**: two identical rests beside
/// each other past the door (the state a doctored save leaves) refuse
/// where the placement evaluates, naming both.
#[test]
fn the_solve_names_every_redundant_constraint() {
    let (doc, store, doc_ref, body) = host("s3b-redundant");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, first) = fixture::constrain_at(doc, b, rest_on(body, a, b));
    let (doc, second) = editor_core::test_support::beside_past_the_bundle_rule(
        &doc,
        b,
        rest_on(body, a, b),
        Tol::witness(),
    );
    let ev = run(&doc, &o);
    match mate_fault(&ev, b) {
        MateFault::Overconstrained {
            placement,
            constraints,
            ..
        } => {
            assert_eq!(placement, b);
            assert_eq!(constraints, vec![first.id, second]);
        }
        other => panic!("overconstrained, not {other:?}"),
    }
}

// ---- The interims ----

/// **One bundle targets one space** (D10 `[placement.spaces]`): a
/// bundle with a ball on a copy in the world and a direction on a copy
/// in another space — the one pair of mates the bundle rule admits
/// beside each other — reads two spaces, and the door refuses it
/// `SpaceMismatch` naming both, before any evaluation.
#[test]
fn a_bundle_of_two_targets_refuses() {
    let label = "s3b-two-targets";
    let (doc, store, doc_ref, body) = host(label);
    let (part, part_body) = block(&format!("{label}-part"));
    let corner = fixture::vpiece(&part, part_body, 0, 0);
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, space) = fixture::into_new_space(doc, b);
    let (doc, c) = insert(doc, mated_instance(doc_ref));
    let point = |at| {
        fixture::point_of(fixture::read_minted(fixture::resolver::in_part_of(
            editor_core::EntityKind::Vertex,
            at,
            body,
            RoleSeg::CapVertex(CapEnd::Start, corner),
        )))
    };
    let normal = |at| {
        editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Flip {
            pose: editor_core::pose::RoleRead::through(
                fixture::plane(fixture::head(face_in(at, body, RoleSeg::Cap(CapEnd::End)))),
                editor_core::pose::PoseRole::Normal,
            ),
        }))
    };
    let bundle = [
        fixture::mate(point(c), point(a)),
        fixture::mate(normal(c), normal(b)),
    ]
    .into_iter()
    .zip(1..)
    .map(|(mate, i)| {
        editor_core::Constraint::of_mate(editor_core::ConstraintId::placeholder(i), mate)
    })
    .collect();
    match editor_core::apply(
        &doc,
        &DocEdit::SetConstraints {
            placement: c,
            constraints: bundle,
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(EditError::SpaceMismatch { node, a, b }) => {
            assert_eq!(node.id(), c);
            assert_eq!((a.id(), b.id()), (VarId::WORLD, space));
        }
        other => panic!("two targets refuse at the door, not {other:?}"),
    }
}

/// **A slide beside a ball has no chart** (review finding 4 of F): a
/// ball and a direction on one target leave a turn about the direction
/// through the ball, and a slide on the direction mate would, through
/// the copy's lever, fix that turn, which is no repeat. The ball leaves
/// it no translation to set, and the solve does not settle a turn from
/// a slide, so the door refuses it `NoChart` (`SlideTurnsABall`), not
/// as an overconstraint.
#[test]
fn a_slide_beside_a_ball_refuses_as_having_no_chart() {
    let label = "s3b-slide-ball";
    let (doc, store, doc_ref, body) = host(label);
    let (part, part_body) = block(&format!("{label}-part"));
    let corner = fixture::vpiece(&part, part_body, 0, 0);
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, c) = insert(doc, mated_instance(doc_ref));
    let point = |at| {
        fixture::point_of(fixture::read_minted(fixture::resolver::in_part_of(
            editor_core::EntityKind::Vertex,
            at,
            body,
            RoleSeg::CapVertex(CapEnd::Start, corner),
        )))
    };
    let normal = |at| {
        editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Flip {
            pose: editor_core::pose::RoleRead::through(
                fixture::plane(fixture::head(face_in(at, body, RoleSeg::Cap(CapEnd::End)))),
                editor_core::pose::PoseRole::Normal,
            ),
        }))
    };
    // The ball, the direction, and a slide on the direction mate,
    // written whole: the bundle's own refusal.
    let refused = doc.apply(
        &DocEdit::SetConstraints {
            placement: c,
            constraints: vec![
                editor_core::Constraint::of_mate(
                    editor_core::ConstraintId::placeholder(1),
                    fixture::mate(point(c), point(a)),
                ),
                editor_core::Constraint::of_mate(
                    editor_core::ConstraintId::placeholder(2),
                    fixture::mate(normal(c), normal(a)),
                ),
                editor_core::Constraint::of_value(
                    editor_core::ConstraintId::placeholder(3),
                    editor_core::Value {
                        coord: editor_core::Coord::Slide(editor_core::TargetAxis::X),
                        by: len(0.1),
                    },
                ),
            ],
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    match refused {
        Err(EditError::MateRefused { fault, .. }) => match *fault {
            MateFault::NoChart { value, why } => {
                assert_eq!(value.placement, c);
                assert_eq!(why, editor_core::NoChartWhy::SlideTurnsABall);
                let said = MateFault::NoChart { value, why }.to_string();
                assert!(said.contains("set the turn as a"), "{said}");
            }
            other => panic!("a slide beside a ball has no chart, not {other:?}"),
        },
        other => panic!("the door refuses the slide, got {other:?}"),
    }
}

/// **A mate on a copy pinned by values takes the freedoms it fixes
/// from them** (D′2: the door rewrites the bundle whole): on a copy its
/// values alone pin, a rest on another copy holding where the values put
/// it is admitted, and the values it fixes go, the slides and the spin
/// it leaves kept as written. The copy stands where it stood.
///
/// *Breaks if* the door keeps a value the rest fixes, or drops one it
/// leaves.
#[test]
fn a_mate_on_a_copy_pinned_by_values_takes_the_freedoms_it_fixes() {
    use editor_core::{Coord, TargetAxis};
    let (doc, store, doc_ref, body) = host("s3b-pin-first");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    // `b` pinned where the rest puts it, on `a`'s top.
    let (doc, b) = insert(doc, crate::fixture::mated_instance(doc_ref));
    let doc = fixture::pin(doc, b, &Frame::translation([0.0, 0.0, 1.0]), &o);
    let before = fixture::solve(&doc, &o, Tol::witness()).placement(&doc, b);
    let (doc, rest) = fixture::at_the_door(&doc, rest_on(body, a, b))
        .unwrap_or_else(|(_, fault)| panic!("the rest is admitted: {fault}"));
    let bundle = fixture::bundle(&doc, b);
    assert_eq!(bundle[0].id, rest.id, "the rest leads the bundle");
    let kept: Vec<Coord> = bundle
        .iter()
        .filter_map(|c| c.value().map(|v| v.coord))
        .collect();
    assert_eq!(
        kept,
        [
            Coord::Slide(TargetAxis::X),
            Coord::Slide(TargetAxis::Y),
            Coord::SPIN,
        ],
        "the values the rest leaves, and none it fixes"
    );
    assert_eq!(
        fixture::solve(&doc, &o, Tol::witness()).placement(&doc, b),
        before,
        "the copy stands where it stood"
    );
}

/// **A constraint has its `on` on its copy**: one relating two other
/// copies refuses `NotOnTheCopy` at the door.
#[test]
fn a_constraint_off_its_copy_refuses() {
    let (doc, store, doc_ref, body) = host("s3b-off-copy");
    let o = with_resolver(store);
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, b) = insert(doc, mated_instance(doc_ref));
    let (doc, c) = insert(doc, mated_instance(doc_ref));
    match at_the_door_of(&doc, c, rest_on(body, a, b)) {
        Err((at, MateFault::NotOnTheCopy { mate })) => {
            assert_eq!((at, mate.placement), (c, c));
        }
        other => panic!("a constraint off its copy refuses, not {other:?}"),
    }
}

// ---- An instance's copies ----

/// **A construction reads one copy of an instance by its key**, and a
/// block of this document placed in the world joins the instance's
/// space, so a union of a pinned instance's copy and the block's copy
/// evaluates.
#[test]
fn a_union_of_an_instances_copy_and_a_block_evaluates() {
    let (doc, store, doc_ref, part_body) = host("s3b-union");
    let o = with_resolver(store);
    let (doc, instance) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, other) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, other) = fixture::in_world(
        doc,
        other,
        &editor_core::Frame::translation([3.0, 0.0, 0.0]),
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

/// **An instance's copy is one body, read whole, and a pick of a piece
/// its part's product does not hold refuses at evaluation**, typed,
/// never re-pointed: the part's product is one copy, so it holds no
/// piece.
#[test]
fn an_instances_copy_is_one_body_and_a_missing_piece_refuses() {
    let (doc, store, doc_ref, part_body) = host("s3b-keyed");
    let o = with_resolver(store);
    let (doc, instance) = fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o);
    let (doc, picked) = insert(
        doc,
        crate::fixture::stood(BodyRead::plain(Operand::pick(
            Operand::Node(instance),
            vec![VarId::new(0, 1)],
        ))),
    );
    let pick = match doc.node(picked) {
        Some(Node::Place { source, .. }) => source.read,
        other => panic!("a placement, not {other:?}"),
    };
    let ev = run(&doc, &o);
    match ev.result(picked) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::NoSuchPiece { pick: read, at } => {
                assert_eq!((*read, *at), (pick, 0));
            }
            other => panic!("no such piece, not {other:?}"),
        },
        other => panic!("a refusal, not {other:?}"),
    }
    let _ = part_body;
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
        Node::instantiate_part_with(
            leaf,
            editor_core::InterfaceRecord::default(),
            editor_core::Operand::Var(editor_core::VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let mid = fixture::pin(
        mid,
        inner,
        &Frame::translation([5.0, 0.0, 0.0]),
        &with_resolver(store.clone()),
    );
    let mid = crate::fixture::assemble_world_copies(mid);
    let mid_ref = store.insert(mid, Tol::witness());
    let top = ProfileDoc::empty(DocumentId::derive("s3b-nested-top"), Tol::witness());
    let (top, outer) = insert(
        top,
        Node::instantiate_part_with(
            mid_ref,
            editor_core::InterfaceRecord::default(),
            editor_core::Operand::Var(editor_core::VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let quarter_turn = Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };
    // The reader copies the one copy where the outer instance puts it:
    // values hold the instance at the quarter turn and the reader where
    // it reads.
    let top = fixture::pin(top, outer, &quarter_turn, &with_resolver(store.clone()));
    // The middle document's product is its one instance's copy, so
    // the outer copy is the block, placed twice.
    let _ = (inner, leaf_body);
    let (top, reader) = insert(top, crate::fixture::stood(Operand::Node(outer)));
    let ev = run(&top, &with_resolver(store));
    let body = match ev.value(reader).map(|v| &v.payload) {
        Some(editor_core::ValuePayload::Body(b)) => crate::fixture::in_space(b),
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

/// **A placement of a `⊔` copies one body, its pieces kept**: a pick of
/// one piece of the copy evaluates to that piece, and a pick through a
/// `∪` refuses at the door.
#[test]
fn a_place_of_an_assemble_picks_its_pieces() {
    let (doc, first) = block("s3b-place-several");
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(3.0, 0.0, 0.5)],
    );
    let (doc, second) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    // Both shapes in `first`'s space, so one `⊔` holds them both.
    let (doc, second) = fixture::slid(doc, first, second, [0.0; 3]);
    let (doc, both) = insert(
        doc,
        Node::Assemble {
            members: editor_core::Bodies::Spelled(vec![first.into(), second.into()]),
        },
    );
    let (doc, place) = insert(doc, crate::fixture::stood(both));
    let doc = fixture::pinned(doc, place, &EvalOptions::default());
    assert_eq!(doc.outputs(place).len(), 1, "one output, one copy");
    let second_read = doc.output(second, 0).expect("the block's copy");
    let first_read = doc.output(first, 0).expect("the block's body");
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                BodyRead::plain(Operand::pick(Operand::Node(place), vec![second_read])),
                BodyRead::plain(Operand::pick(Operand::Node(place), vec![first_read])),
            ]),
        },
    );
    let doc = editor_core::test_support::assemble_world_copies(doc, Tol::witness());
    let ev = run(&doc, &EvalOptions::default());
    assert!(
        matches!(ev.result(union), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(union)
    );
    let (doc, joined) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![first.into(), second.into()]),
        },
    );
    let (doc, through) = insert(doc, crate::fixture::stood(joined));
    let refused = fixture::insert_refused(
        &doc,
        crate::fixture::stood(BodyRead::plain(Operand::pick(
            Operand::Node(through),
            vec![first_read],
        ))),
    );
    assert!(
        matches!(refused, EditError::PickPath { at: Some(0), .. }),
        "{refused:?}"
    );
}

// ---- Carried ----

/// **A carried pose is the pose its copy carries**: a block's top plane,
/// carried by a placement three units along x and two up, is the top
/// plane moved two up (its slide along x is in-plane, which a plane
/// forgets) — each read where it splits, the block's in the block's own space and
/// the carried one in the world its copy stands in.
#[test]
fn a_carried_pose_moves_with_its_copy() {
    let (doc, body) = block("s3b-carried");
    let (doc, placement) = place_by_values(doc, body, &Frame::translation([3.0, 0.0, 2.0]));
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
    let mut offsets = Vec::new();
    for carried in [false, true] {
        let (doc, split) = insert(
            doc.clone(),
            Node::Split {
                // Each pose split in its own space: the block's face
                // splits the block, the carried face its copy.
                target: Operand::output(if carried { placement } else { body }, 0).into(),
                tool: plane(carried).into(),
            },
        );
        let Some(Node::Split { tool, .. }) = doc.node(split) else {
            panic!("a split")
        };
        let tool = *tool;
        let ev = run(&doc, &EvalOptions::default());
        let value = bound_pose(&doc, &ev, OperandSlot::Tool, tool, Tol::witness());
        let Ok(PoseValue::Plane { normal, offset }) = value else {
            panic!("a plane, not {value:?}")
        };
        assert_eq!(UnitVec3::get(normal).z, 1.0, "the top's outward normal");
        offsets.push(offset);
    }
    assert_eq!(offsets[1] - offsets[0], 2.0, "moved with its copy");
}
