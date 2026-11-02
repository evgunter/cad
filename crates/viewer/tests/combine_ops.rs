//! **Combining bodies, replayed headlessly** (GAUTH-4): the Phase-B op
//! vocabulary driving the real `DocSession` with no renderer — the
//! boolean / split / transform / pattern doors with their typed
//! refusals, the tool values that feed them, and the rule that only
//! one modal tool is open.
//!
//! # The two boxes every geometric row is about
//!
//! [`two_boxes`] authors an overlapping pair through the creation
//! vocabulary alone (rectangle profile → extrude, and a second one
//! placed by [`SessionOp::AddTransform`]): a 40 × 20 × 10 mm block and
//! a 20 × 10 × 6 mm block offset so that they overlap in a
//! 5 × 10 × 6 mm brick and share NO plane. The constants below are
//! that arrangement's closed form, and every boolean row asserts
//! against them rather than against a recorded number — a row goes red
//! when the kernel's answer stops being the arithmetic one, which is
//! the only thing worth pinning here.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::AuthoredNode;
use pncad::document::Bodies;
use pncad::topo::BooleanOp;
use test_utils::refusal::tagged;
use viewer::session::BooleanSpec;

use common::drawn::spacing_of;
use common::{
    ang, body_volume, len, len3, near, no_translation, scl3, session_insert, shape, translation,
};
use pncad::document::BooleanValue;
use pncad::document::Speaker;
use pncad::document::SplitSide;
use pncad::document::{
    Axis3, Dimension, DimensionError, Doc, DocEdit, EditError, Expr, Formula, LoopProgram, Node,
    NodeError, NodeErrorKind, NodeResult, NodeStanding, Operand, OperandSlot, ProfileProgram,
    RecipeNodeId, SlotId,
};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use pncad::select::SplitHalf;
use viewer::combine::{
    BooleanTool, DUPLICATE_GAP, DuplicateFault, MEASURE_CHORD, PatternOutputChoice, PatternTool,
    STEP_DIRECTION, SplitTool, TransformTool, denotes_body,
};
use viewer::pickindex::PickKinds;
use viewer::seats::{Seat, SeatError, SeatEvent, seat_line};
use viewer::session::{
    DocSession, GestureFrameFault, NodeKindWanted, PartSelectSpec, PatternRuleSpec, ProfileShape,
    Refusal, Selection, SessionOp,
};
use viewer::tools::{ToolKind, Tools};

/// The big block: 40 × 20 × 10 mm, extruded up from the sketch plane.
const A: [f64; 3] = [0.04, 0.02, 0.01];
/// The small block: 20 × 10 × 6 mm, before it is placed.
const B: [f64; 3] = [0.02, 0.01, 0.006];
/// Where the small block is placed — chosen so the two overlap in a
/// brick and no face of one is coplanar with a face of the other.
const OFFSET: [f64; 3] = [0.025, 0.002, 0.002];
/// The volume of the overlap brick: 5 × 10 × 6 mm.
const OVERLAP: f64 = 0.005 * 0.01 * 0.006;

/// A session over a throwaway document.
fn session(tol: Tol) -> DocSession {
    crate::common::inline_without_door(Doc::empty_derived("combine-start", tol), tol)
}

/// The two overlapping blocks (module docs), the second placed by a
/// transform, each then placed into the world by hand (a new body lands
/// unplaced). Answers the session and the two BODY nodes, in the order
/// the module's constants name them.
fn two_boxes(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let mut session = session(tol);
    let a = common::xy_box_in(&mut session, A);
    let raw_b = common::xy_box_in(&mut session, B);
    let b = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: raw_b.into(),
            translation: len3(OFFSET),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    let (session, _) = common::world_copy_in(&session, a, tol);
    let (session, _) = common::world_copy_in(&session, b, tol);
    (session, a, b)
}

/// A split node's two sides, by volume, with the seam pumped — an
/// empty side reading as zero.
fn split_volumes(session: &mut DocSession, split: RecipeNodeId, tol: Tol) -> (f64, f64) {
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let ValuePayload::Split { above, below } =
        &eval.value(split).expect("the split evaluated").payload
    else {
        panic!("a split evaluates to its two sides");
    };
    let volume_of = |side: &SplitSide<f64>| match side {
        SplitSide::Body(body) => {
            pncad::topo::test_support::pinned_volume(
                "mass properties",
                &pncad::topo::AtRestBody::validate(pncad::topo::Body::clone(body.body()), tol)
                    .expect("the measured body passes tier 3"),
                tol,
            )
            .volume
        }
        SplitSide::Empty => 0.0,
    };
    (volume_of(above), volume_of(below))
}

/// A body's volume, with the closed form it must match.
fn assert_volume(session: &mut DocSession, node: RecipeNodeId, want: f64, tol: Tol) {
    let got = body_volume(session, node, tol);
    assert!(near(got, want), "volume {got} vs {want}");
}

/// **The acceptance row**: a two-body boolean authored from nothing,
/// evaluated, saved, reloaded and re-evaluated. The two boxes share no
/// plane, and the union lands as one insert.
#[test]
fn a_two_body_union_authors_evaluates_saves_and_reloads() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    let union = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![a.into(), b.into()]),
        },
    );
    assert!(matches!(
        session.committed_doc().node(union),
        Some(Node::Union { .. })
    ));
    let va = A[0] * A[1] * A[2];
    let vb = B[0] * B[1] * B[2];
    let volume = body_volume(&mut session, union, tol);
    assert!(near(volume, va + vb - OVERLAP), "union volume {volume}");

    let dir = common::tempdir("gauth4-union");
    let path = dir.join("union.pncad");
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none(),
        "save"
    );
    let authored = session.committed_doc().clone();
    assert!(
        session.perform(SessionOp::Open(path)).refusal.is_none(),
        "open"
    );
    assert!(
        session.committed_doc().bit_eq(&authored),
        "the reloaded document is the authored one, bit for bit"
    );
    let reloaded = body_volume(&mut session, union, tol);
    assert_eq!(
        volume.to_bits(),
        reloaded.to_bits(),
        "same solid after reload"
    );
}

/// **Three picks, one union**: the boolean tool fed three body picks
/// through the selection stream commits ONE union node spelling all
/// three members, in pick order, each as the read of the body picked —
/// and it is the arithmetic union of the three.
#[test]
fn a_three_pick_union_commits_one_union_of_three_spelled_members() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    // A third block well clear of both.
    let raw_c = common::xy_box_in(&mut session, B);
    let c = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: raw_c.into(),
            translation: translation([0.0, 0.05, 0.0]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    let mut tools = Tools::new();
    tools.open(ToolKind::Boolean);
    let picks: Vec<_> = [a, b, c]
        .into_iter()
        .map(|node| SessionOp::Select(Selection::Node(node)))
        .collect();
    assert!(
        tools
            .feed(
                session.committed_doc(),
                Err(viewer::pickcache::NotIndexed::Absent),
                &picks
            )
            .is_empty(),
        "every pick landed"
    );
    let tool = tools.boolean().expect("the boolean tool is open");
    assert_eq!(
        tool.operation(),
        BooleanOp::Union,
        "the tool opens on union"
    );
    let op = tool.op().expect("three members are held");
    assert!(tools.commits_open_tool(&op), "the union is the tool's edit");
    let steps = session.history().len();
    let union = session_insert(&mut session, op);
    assert_eq!(session.history().len(), steps + 1, "one action, one step");
    let doc = session.committed_doc();
    let Some(Node::Union {
        members: Bodies::Spelled(members),
    }) = doc.node(union)
    else {
        panic!("one union node: {:?}", doc.node(union));
    };
    assert_eq!(
        members.iter().map(|m| Some(m.read)).collect::<Vec<_>>(),
        [a, b, c].map(|n| doc.output(n, 0)),
        "three spelled members, the picks' reads in pick order"
    );
    assert_eq!(
        common::world(doc),
        [union],
        "the union takes all three members' place in the world"
    );
    let va = A[0] * A[1] * A[2];
    let vb = B[0] * B[1] * B[2];
    assert_volume(&mut session, union, va + 2.0 * vb - OVERLAP, tol);
}

/// **Nothing places as a side effect, and the combine gesture takes its
/// operands' place in the world** (spec §9 row 9, the viewer's half).
///
/// Two placed bodies `a`, `b` (`b` moved): the combine gesture leaves
/// the world `[c]` — `b` placed against `a` where it stands, `a`'s
/// placement re-pointed to `c` through the slot door and `b`'s
/// deleted, in ONE recorded action — and one undo restores
/// `[a, b]` bit for bit. Breaks if the gesture leaves the boolean
/// unplaced (the world stays `[a, b]`), places it beside them, or
/// records the world edits as a second action.
#[test]
fn the_combine_gesture_re_points_the_world_in_one_action() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    session.pump();
    let before = session.committed_doc().clone();
    assert_eq!(common::world(&before), [a, b], "both operands are placed");
    let [a_placement, b_placement] = crate::common::drawn_copies(&before)[..] else {
        panic!("two copies: {:?}", crate::common::drawn_copies(&before));
    };
    let outcome = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![a.into(), b.into()]),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let union = outcome.minted[0];
    assert!(
        matches!(
            &outcome.committed[..],
            [
                replaced @ ..,
                DocEdit::InsertNode { .. },
                DocEdit::SetParam {
                    node,
                    slot: pncad::document::SlotId::Operand(OperandSlot::Source),
                    value: pncad::document::SlotValue::Read(pncad::document::Operand::Node(read)),
                    ..
                },
                DocEdit::DeleteNode { id },
            ] if *node == a_placement
                && *read == union
                && *id == b_placement
                && replaced.iter().all(|edit| matches!(
                    edit,
                    DocEdit::SetConstraints { placement, .. }
                        | DocEdit::SetParam { node: placement, .. } if *placement == b
                ))
        ),
        "b, moved, placed against a where it stands; the boolean; a's \
         placement re-pointed to it, b's deleted: {:?}",
        outcome.committed
    );
    let doc = session.committed_doc();
    assert_eq!(
        crate::common::drawn_copies(doc),
        [a_placement],
        "a's placement stands"
    );
    assert_eq!(common::world(doc), [union], "and places the boolean");
    let va = A[0] * A[1] * A[2];
    let vb = B[0] * B[1] * B[2];
    let drawn = drawn_volume(&mut session, tol);
    assert!(
        near(drawn, va + vb - OVERLAP),
        "the drawn product is the union, once: {drawn}"
    );

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo restores [a, b], bit for bit"
    );
}

/// **A combine reads its operands in their spaces, not in the world's
/// picture of them.** `a`'s world placement is slid 0.1 m along `x`,
/// so `a`'s space stands off `b`'s in the world and the two drawn
/// copies no longer overlap. The union still takes `b` into `a`'s
/// space with its values as written — it stands on `a`'s axes as it
/// stood on its own body's — so the union overlaps as the two bodies
/// do in their own coordinates, and the result is drawn where `a`'s
/// placement puts it: `b`'s world position moves with `a`'s. Keeping
/// `b`'s world position instead would compose the two world bundles
/// into numbers, a frame read back.
///
/// *Breaks if* the combine composes `a`'s world placement into `b`'s
/// values (the union would be disjoint).
#[test]
fn a_combine_takes_the_operand_into_the_first_ones_space_by_its_coordinates() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    session.pump();
    let [a_placement, _] = crate::common::drawn_copies(session.committed_doc())[..] else {
        panic!("two copies");
    };
    let slide_x = pncad::document::Coord::Slide(pncad::document::TargetAxis::X);
    let moved = session.perform(SessionOp::SetSlot {
        node: a_placement,
        slot: SlotId::ValueSlide {
            constraint: session
                .committed_doc()
                .node(a_placement)
                .and_then(|node| match node {
                    Node::Place { constraints, .. } => constraints
                        .iter()
                        .find(|c| c.value().is_some_and(|v| v.coord == slide_x))
                        .map(|c| c.id),
                    _ => None,
                })
                .expect("a's placement writes a slide along x"),
        },
        value: viewer::props::SlotValue::of(Dimension::Length, 0.1)
            .expect("a finite length is a value"),
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    session.pump();
    let va = A[0] * A[1] * A[2];
    let vb = B[0] * B[1] * B[2];
    let drawn = drawn_volume(&mut session, tol);
    assert!(
        near(drawn, va + vb),
        "drawn apart, the copies are disjoint: {drawn}"
    );
    let outcome = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![a.into(), b.into()]),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let drawn = drawn_volume(&mut session, tol);
    assert!(
        near(drawn, va + vb - OVERLAP),
        "the union overlaps as the bodies do in their own coordinates: {drawn}"
    );
}

/// **A combine of an operand nothing places places nothing of it**:
/// the first placed operand's placements take the result, and an
/// unplaced one has none to give.
#[test]
fn a_combine_takes_the_placement_of_the_operand_the_world_places() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    let a_placement = common::copy_of(session.committed_doc(), a);
    let unplaced = session.perform(SessionOp::DeleteNode { node: a_placement });
    assert!(unplaced.refusal.is_none(), "{:?}", unplaced.refusal);
    assert_eq!(
        common::world(session.committed_doc()),
        [b],
        "only b is placed"
    );
    let union = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![a.into(), b.into()]),
        },
    );
    assert_eq!(
        common::world(session.committed_doc()),
        [union],
        "b's placement takes the result: the world still shows its material"
    );
}

/// **A feature gesture re-points every placement of its target, and a
/// target nothing places places nothing.** Two identity placements of
/// one body are two copies (A10); a transform of the body takes both.
#[test]
fn a_feature_re_points_each_placement_of_its_target() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("combine-two-copies", tol);
    let (doc, profile) = common::square_profile(&doc, A[0], tol);
    let (doc, body) = common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(A[2]),
        },
        tol,
    );
    let (doc, _) = common::placed(&doc, body, tol);
    let (doc, _) = common::placed(&doc, body, tol);
    let mut session = DocSession::inline(doc, tol);
    let moved = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: body.into(),
            translation: translation([0.0, 0.0, A[2] * 4.0]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    assert_eq!(
        common::world(session.committed_doc()),
        [moved, moved],
        "both copies now place the transform"
    );

    let world = common::world(session.committed_doc());
    let outcome = session.perform(SessionOp::AddTransform {
        input: body.into(),
        translation: len3(OFFSET),
        rotation_axis: scl3([0.0, 0.0, 1.0]),
        rotation_angle: Formula::zero(Dimension::Angle),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        matches!(&outcome.committed[..], [DocEdit::InsertNode { .. }]),
        "the body is no longer placed, so its second transform re-points nothing: \
         its placement, inserted with its bundle: {:?}",
        outcome.committed
    );
    // Nothing reads the second transform's copy, so it is drawn, in the
    // body's own space, beside the two copies it did not touch. A copy
    // into the space of what it copies is no gesture's copy, so a pick on
    // it is on the transform itself.
    let second = outcome.minted[0];
    assert_eq!(
        common::world(session.committed_doc()),
        [world, vec![second]].concat()
    );
}

/// **Difference is not commutative, and the door's operand order is
/// what decides it**: the same two picks in the two orders author two
/// different solids, each the arithmetic one.
#[test]
fn subtraction_is_not_commutative_in_the_authored_order() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    let va = A[0] * A[1] * A[2];
    let vb = B[0] * B[1] * B[2];

    let a_minus_b = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: a.into(),
                tool: b.into(),
            },
        },
    );
    let b_minus_a = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: b.into(),
                tool: a.into(),
            },
        },
    );
    let first = body_volume(&mut session, a_minus_b, tol);
    let second = body_volume(&mut session, b_minus_a, tol);
    assert!(near(first, va - OVERLAP), "a ∖ b volume {first}");
    assert!(near(second, vb - OVERLAP), "b ∖ a volume {second}");
    assert!(
        (first - second).abs() > OVERLAP,
        "the two orders are genuinely different solids"
    );

    // The intersection is the overlap brick either way — the operand
    // order is data for subtraction and for nothing else here.
    let meet = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Intersect(vec![a.into(), b.into()]),
        },
    );
    assert_volume(&mut session, meet, OVERLAP, tol);
}

/// Every operand wants a body, and a body may be listed twice: a
/// repeated read GLUES, so `A ∪ A` is `A` and `A ∖ A` the typed empty
/// body — documents the door commits like any other.
#[test]
fn the_boolean_door_refuses_a_non_body_operand_and_glues_a_repeated_body() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let a = common::xy_box_in(&mut session, A);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius: 0.01,
            })],
        },
    );
    // A profile and an id the document never held are each "not a
    // body" at any operand.
    for wrong in [profile, RecipeNodeId::new(0, tagged(999))] {
        for spec in [
            BooleanSpec::Union(vec![wrong.into(), a.into()]),
            BooleanSpec::Intersect(vec![a.into(), a.into(), wrong.into()]),
            BooleanSpec::Subtract {
                from: a.into(),
                tool: wrong.into(),
            },
        ] {
            let refused = session.perform(SessionOp::AddBoolean { spec });
            assert!(
                matches!(
                    &refused.refusal,
                    Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body })
                        if node.id() == wrong
                ),
                "{:?}",
                refused.refusal
            );
        }
    }
    // One body listed twice: accepted, and glued.
    let va = A[0] * A[1] * A[2];
    let doubled = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![a.into(), a.into()]),
        },
    );
    assert_volume(&mut session, doubled, va, tol);
    let nothing = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: a.into(),
                tool: a.into(),
            },
        },
    );
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    assert!(
        matches!(
            eval.value(nothing).map(|v| &v.payload),
            Some(ValuePayload::Boolean(BooleanValue::Empty))
        ),
        "A ∖ A is the typed empty body"
    );
}

/// A split's target seat wants a body, its plane is numbers in the
/// target's own coordinates, and what it authors cuts.
#[test]
fn the_split_door_takes_a_body_and_a_plane_in_its_coordinates() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let cut = 0.004;
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius: 0.01,
            })],
        },
    );
    let refused = session.perform(SessionOp::AddSplit {
        target: profile.into(),
        origin: len3([0.0, 0.0, cut]),
        normal: scl3([0.0, 0.0, 1.0]),
    });
    assert!(
        matches!(
            &refused.refusal,
            Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body }) if node.id() == profile
        ),
        "{:?}",
        refused.refusal
    );
    // A normal that names no direction refuses typed, before any edit.
    let states = session.history().len();
    let refused = session.perform(SessionOp::AddSplit {
        target: body.into(),
        origin: len3([0.0, 0.0, cut]),
        normal: scl3([0.0; 3]),
    });
    assert!(
        matches!(
            refused.refusal.as_ref(),
            Some(Refusal::Frame(fault)) if matches!(**fault, GestureFrameFault::Direction { .. })
        ),
        "{:?}",
        refused.refusal
    );
    assert_eq!(session.history().len(), states, "a refusal records nothing");

    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, 0.0, cut]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
    );
    let (up, down) = split_volumes(&mut session, split, tol);
    let face = A[0] * A[1];
    assert!(near(up, face * (A[2] - cut)), "above the cut: {up}");
    assert!(near(down, face * cut), "below the cut: {down}");
}

/// A split's OUTPUT is two bodies, so it is not a body seat's pick —
/// the door says so rather than letting the operand mismatch surface
/// after the edit lands. A family is not refused: a reader of it is
/// evaluated once per member, and a union's member seat reads it
/// whole.
#[test]
fn several_bodies_are_not_one_body_at_a_seat() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, 0.0, 0.004]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
    );
    {
        let wrong = split;
        for op in [
            SessionOp::AddBoolean {
                spec: BooleanSpec::Union(vec![wrong.into(), body.into()]),
            },
            SessionOp::AddTransform {
                input: wrong.into(),
                translation: no_translation(),
                rotation_axis: scl3([0.0, 0.0, 1.0]),
                rotation_angle: Formula::zero(Dimension::Angle),
            },
            SessionOp::AddSplit {
                target: wrong.into(),
                origin: len3([0.0, 0.0, 0.004]),
                normal: scl3([0.0, 0.0, 1.0]),
            },
        ] {
            let refused = session.perform(op);
            assert!(
                matches!(
                    &refused.refusal,
                    Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body })
                        if node.id() == wrong
                ),
                "{:?}",
                refused.refusal
            );
        }
    }
}

/// The transform door places a body rigidly: a placement of it into its
/// own space, standing at the frame the form's numbers make, by values
/// on the space's axes the property panel then owns.
#[test]
fn the_transform_door_places_a_body_with_values() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let placed = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: body.into(),
            translation: len3(OFFSET),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: ang(core::f64::consts::FRAC_PI_2),
        },
    );
    // A rigid placement preserves the volume, and the copy stands at
    // the frame the form's numbers make, in its body's own space.
    assert_volume(&mut session, placed, A[0] * A[1] * A[2], tol);
    let doc = session.committed_doc();
    assert!(
        matches!(doc.node(placed), Some(Node::Place { .. })),
        "the transform is a placement"
    );
    let want = pncad::document::Frame::rotate_then_translate(
        [0.0, 0.0, 1.0],
        core::f64::consts::FRAC_PI_2,
        OFFSET,
        common::band(),
    )
    .expect("a rotation axis with a direction");
    let poses = pncad::document::solve_document(doc, &pncad::document::EvalOptions::default(), tol);
    let at = poses
        .relative(placed)
        .expect("the copy stands in its body's space");
    let close = |a: f64, b: f64| (a - b).abs() <= 1.0e-12;
    assert!(
        at.translation
            .iter()
            .zip(want.translation)
            .all(|(a, b)| close(*a, b))
            && at
                .columns
                .iter()
                .flatten()
                .zip(want.columns.iter().flatten())
                .all(|(a, b)| close(*a, *b)),
        "the copy stands at {at:?}, not {want:?}"
    );
    // Its values are the placement's slots: the spin about the space's
    // `z` the rotation axis names, and the three slides.
    for (coord, dim) in [
        (
            pncad::document::Coord::Spin(Some(pncad::document::TargetAxis::Z)),
            Dimension::Angle,
        ),
        (
            pncad::document::Coord::Slide(pncad::document::TargetAxis::X),
            Dimension::Length,
        ),
        (
            pncad::document::Coord::Slide(pncad::document::TargetAxis::Y),
            Dimension::Length,
        ),
        (
            pncad::document::Coord::Slide(pncad::document::TargetAxis::Z),
            Dimension::Length,
        ),
    ] {
        let slot = crate::fixture::value_slot(doc, placed, coord);
        let expr = doc
            .slot_expansion(placed, slot)
            .expect("the value slot is there");
        assert_eq!(expr.dim(), dim, "{coord:?}");
    }

    // A non-finite field never reaches the document, and now cannot
    // reach the OP either: the literal door refuses it where the
    // expression is built, so the op has no spelling for it.
    assert!(matches!(
        Formula::literal(f64::NAN, Dimension::Length),
        Err(DimensionError::NonFiniteLiteral)
    ));

    // What the door still refuses is a wrong-DIMENSION expression —
    // an angle where a translation belongs — typed, before any edit.
    let states = session.history().len();
    let refused = session.perform(SessionOp::AddTransform {
        input: body.into(),
        translation: [ang(0.0), len(0.0), len(0.0)],
        rotation_axis: scl3([0.0, 0.0, 1.0]),
        rotation_angle: Formula::zero(Dimension::Angle),
    });
    assert!(
        matches!(
            refused.refusal.as_ref(),
            Some(Refusal::Frame(fault)) if matches!(**fault, GestureFrameFault::Dimension { .. })
        ),
        "{:?}",
        refused.refusal
    );
    assert_eq!(session.history().len(), states, "a refusal records nothing");
}

/// The pattern door declares an index of the count it was handed, and
/// both rules author what they say: a row a placement with no mate, a
/// ring one with one ([`pncad::pattern::recognise`]).
#[test]
fn the_pattern_door_declares_an_index_of_its_count() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let linear = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 3,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
    );
    // The count is the index's, an exact Count literal, and the rule is
    // the one the form offered.
    let count = common::family_count(session.committed_doc(), linear);
    assert_eq!(count.dim(), Dimension::Count);
    assert!(count.bit_eq(&Formula::count(3)));
    assert!(matches!(
        pncad::pattern::recognise(session.committed_doc(), linear),
        Some(pncad::pattern::Recognised::Linear { .. })
    ));
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let ValuePayload::Instances(instances) =
        &eval.value(linear).expect("the family evaluated").payload
    else {
        panic!("a family evaluates to its members");
    };
    assert_eq!(instances.len(), 3, "three members, unfused");

    // The circular rule's axis is a point and a direction in the
    // body's own coordinates, read off its geometry; one that names no
    // direction refuses typed.
    let circular = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 4,
            rule: PatternRuleSpec::Circular {
                through: len3([0.0; 3]),
                direction: scl3([0.0, 0.0, 1.0]),
                step: ang(core::f64::consts::FRAC_PI_2),
            },
        },
    );
    assert!(matches!(
        pncad::pattern::recognise(session.committed_doc(), circular),
        Some(pncad::pattern::Recognised::Circular { .. })
    ));
    let refused = session.perform(SessionOp::AddPattern {
        input: body.into(),
        count: 4,
        rule: PatternRuleSpec::Circular {
            through: len3([0.0; 3]),
            direction: scl3([0.0; 3]),
            step: ang(1.0),
        },
    });
    assert!(
        matches!(
            refused.refusal.as_ref(),
            Some(Refusal::Frame(fault)) if matches!(**fault, GestureFrameFault::Direction { .. })
        ),
        "{:?}",
        refused.refusal
    );
}

/// **The fused door is the affordance the boolean's refusal points
/// at**: the same form as the pattern door — same prototype, same
/// count, same rule — minting the family and a `Union` over it, ONE
/// body a boolean seat consumes where a family's several members are
/// refused.
#[test]
fn the_fused_door_mints_one_body_a_boolean_seat_takes() {
    let tol = Tol::witness();
    let (mut session, part, proto) = two_boxes(tol);
    let fused = session_insert(
        &mut session,
        SessionOp::AddFusedPattern {
            input: proto.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
    );
    // The union reads the family whole, and the family's count is the
    // index's.
    let Some(Node::Union {
        members: Bodies::Family(family),
    }) = session.committed_doc().node(fused)
    else {
        panic!("the fused door mints a union over the family");
    };
    let family = session
        .committed_doc()
        .operation_of(family.read)
        .expect("the family is a node");
    let count = common::family_count(session.committed_doc(), family);
    assert!(count.bit_eq(&Formula::count(2)));
    assert!(matches!(
        pncad::pattern::recognise(session.committed_doc(), family),
        Some(pncad::pattern::Recognised::Linear { .. })
    ));

    // ONE body out, and it is the union of the prototype at both
    // placements — where the same form's other door answers with two
    // separate members.
    session.pump();
    {
        let eval = session.evaluation().expect("the inline seam landed");
        let value = eval.value(fused).expect("the union evaluated");
        assert!(
            matches!(
                value.payload,
                ValuePayload::Boolean(BooleanValue::Body { .. })
            ),
            "a union of a family evaluates to one body, not to members"
        );
    }
    let vb = B[0] * B[1] * B[2];
    let fused_volume = body_volume(&mut session, fused, tol);
    assert!(
        near(fused_volume, 2.0 * vb),
        "two disjoint copies of the prototype: {fused_volume}"
    );

    // And the seat that refuses a family takes this: the fused group
    // merges into the part through the ordinary boolean door.
    let union = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![part.into(), fused.into()]),
        },
    );
    let va = A[0] * A[1] * A[2];
    let volume = body_volume(&mut session, union, tol);
    assert!(
        near(volume, va + 2.0 * vb - OVERLAP),
        "the part with the group fused into it: {volume}"
    );
}

/// **Overlapping copies fuse**: the union judges the pairs of members
/// that meet, so a step shorter than the prototype is wide builds one
/// body — the copies' hull along the step — exactly as the unfused
/// family over the same rule builds its members.
#[test]
fn overlapping_copies_fuse_into_one_body() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let proto = common::xy_box_in(&mut session, B);
    // A step shorter than the prototype is wide: every pair of copies
    // meets.
    let crowded = session_insert(
        &mut session,
        SessionOp::AddFusedPattern {
            input: proto.into(),
            count: 3,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(B[0] / 4.0),
            },
        },
    );
    let loose = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: proto.into(),
            count: 3,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(B[0] / 4.0),
            },
        },
    );
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    assert!(
        eval.value(loose).is_some(),
        "the unfused family over the same rule evaluates"
    );
    let volume = body_volume(&mut session, crowded, tol);
    assert!(
        near(volume, 1.5 * B[0] * B[1] * B[2]),
        "three copies a quarter-width apart span one and a half widths: {volume}"
    );
}

/// **The new op round-trips**: authored, saved, reopened through the
/// session's own doors, the document is the same one bit for bit and
/// the same solid comes back out of it.
///
/// A `SessionOp` joins the REPLAY vocabulary, so this is the row that
/// says the file carries the fused node rather than only the session
/// that authored it.
#[test]
fn a_fused_pattern_round_trips_through_save_and_open() {
    let tol = Tol::witness();
    let mut authoring = session(tol);
    let proto = common::xy_box_in(&mut authoring, B);
    let fused = session_insert(
        &mut authoring,
        SessionOp::AddFusedPattern {
            input: proto.into(),
            count: 3,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
    );
    let volume = body_volume(&mut authoring, fused, tol);
    let authored = authoring.committed_doc().clone();

    let dir = common::tempdir("gauth4-placed-union");
    let path = dir.join("fused.pncad");
    assert!(
        authoring
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none(),
        "save"
    );
    // A FRESH session, so nothing of the authoring one is carried
    // into the answer.
    let mut reopened = session(tol);
    assert!(
        reopened.perform(SessionOp::Open(path)).refusal.is_none(),
        "open"
    );
    assert!(
        reopened.committed_doc().bit_eq(&authored),
        "the reopened document is the authored one, bit for bit"
    );
    let reloaded = body_volume(&mut reopened, fused, tol);
    assert_eq!(
        volume.to_bits(),
        reloaded.to_bits(),
        "same solid after reload"
    );
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// **Undo then redo returns the document to the same state**, bit for
/// bit, with the fused node in it and its solid unchanged — the other
/// half of joining the replay vocabulary.
#[test]
fn undo_and_redo_walk_the_fused_pattern_out_and_back() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let proto = common::xy_box_in(&mut session, B);
    let before = session.committed_doc().clone();
    let states = session.history().len();
    let fused = session_insert(
        &mut session,
        SessionOp::AddFusedPattern {
            input: proto.into(),
            count: 3,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
    );
    let after = session.committed_doc().clone();
    let volume = body_volume(&mut session, fused, tol);
    assert_eq!(session.history().len(), states + 1, "one op, one state");

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().node(fused).is_none(),
        "the undo took the fused node out"
    );
    assert!(
        session.committed_doc().bit_eq(&before),
        "and left the document it was inserted into, bit for bit"
    );
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&after),
        "the redo puts back exactly what the op committed"
    );
    let again = body_volume(&mut session, fused, tol);
    assert_eq!(volume.to_bits(), again.to_bits(), "the same solid, redone");
    assert_eq!(
        session.history().len(),
        states + 1,
        "the walk destroyed nothing and minted nothing"
    );
}

/// The pattern form's output choice read as a value: both labels, the
/// default, and which op each one commits — the chrome's radio row
/// checked without a window.
#[test]
fn the_output_choice_names_both_doors_and_defaults_to_instances() {
    assert_eq!(
        PatternOutputChoice::ALL.map(|(output, label)| (output, label)),
        [
            (PatternOutputChoice::Instances, "instances"),
            (PatternOutputChoice::Fused, "fused"),
        ]
    );
    assert_eq!(
        PatternOutputChoice::default(),
        PatternOutputChoice::Instances
    );

    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, B);
    let mut tool = PatternTool::new();
    tool.pick(session.committed_doc(), body);
    for (output, label) in PatternOutputChoice::ALL {
        let op = tool
            .linear_op(output, 2, scl3([1.0, 0.0, 0.0]), len(0.05))
            .expect("the body seat is filled");
        let minted = matches!(
            (output, &op),
            (PatternOutputChoice::Instances, SessionOp::AddPattern { .. })
                | (
                    PatternOutputChoice::Fused,
                    SessionOp::AddFusedPattern { .. }
                )
        );
        assert!(minted, "{label} commits its own op: {op:?}");
    }
}

/// Every door that takes an existing BODY refuses mid-gesture and
/// records nothing when it refuses — the creation vocabulary's rule,
/// held by GAUTH-4's four arms and GAUTH-5's two.
///
/// The list below is hand-written, which is the reason it is worth
/// saying what it is a list OF: every `SessionOp` arm whose seat is a
/// node that must already exist. A door added to that family and not
/// added here is a door with no gesture row and no
/// nothing-was-recorded row, which is exactly how the blend doors
/// first shipped.
#[test]
fn a_refusal_at_any_body_seated_door_leaves_no_history_state() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    // A node of the wrong kind at every body seat: a profile.
    let plane = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![common::rectangle_loop([0.0, 0.0], 0.01, 0.01)],
        },
    );
    let other = common::xy_box_in(&mut session, B);
    // Real edge names for the two blend doors: their seat refusals are
    // about the TARGET, so the selection has to be the kind of thing a
    // user would actually have picked rather than an empty vector that
    // could refuse for its own reason.
    let edges = {
        session.pump();
        let eval = session.evaluation().expect("the inline seam landed");
        let edges = pncad::select::all_edges(eval, body);
        assert!(!edges.is_empty(), "the box has edges");
        edges
    };
    let states = session.history().len();
    let doc = session.committed_doc().clone();

    // A gesture in flight closes every door here.
    assert!(
        session
            .perform(SessionOp::BeginGesture {
                node: body,
                slot: SlotId::Distance,
            })
            .refusal
            .is_none(),
        "the gesture opened"
    );
    for op in [
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![body.into(), other.into()]),
        },
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0; 3]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
        SessionOp::AddTransform {
            input: body.into(),
            translation: no_translation(),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
        SessionOp::AddFusedPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
        SessionOp::AddFillet {
            target: body,
            radius: len(0.001),
            selection: edges.clone(),
        },
        SessionOp::AddChamfer {
            target: body,
            distance: len(0.001),
            selection: edges.clone(),
        },
    ] {
        let refused = session.perform(op);
        assert!(
            matches!(refused.refusal, Some(Refusal::GestureInFlight)),
            "{:?}",
            refused.refusal
        );
    }
    assert!(session.perform(SessionOp::CancelGesture).refusal.is_none());

    // The SEAT refusals record nothing either — a wrong-kind pick at
    // any of the doors, a union member and a subtraction's tool among
    // them — nor does a plane or an axis that names no direction.
    for op in [
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![plane.into(), body.into()]),
        },
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: body.into(),
                tool: plane.into(),
            },
        },
        SessionOp::AddSplit {
            target: plane.into(),
            origin: len3([0.0; 3]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0; 3]),
            normal: scl3([0.0; 3]),
        },
        SessionOp::AddTransform {
            input: plane.into(),
            translation: no_translation(),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
        SessionOp::AddPattern {
            input: plane.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Circular {
                through: len3([0.0; 3]),
                direction: scl3([0.0; 3]),
                step: ang(1.0),
            },
        },
        SessionOp::AddFusedPattern {
            input: plane.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
        SessionOp::AddFusedPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Circular {
                through: len3([0.0; 3]),
                direction: scl3([0.0; 3]),
                step: ang(1.0),
            },
        },
        SessionOp::AddFillet {
            target: plane,
            radius: len(0.001),
            selection: edges.clone(),
        },
        SessionOp::AddChamfer {
            target: plane,
            distance: len(0.001),
            selection: edges.clone(),
        },
    ] {
        let refused = session.perform(op);
        assert!(
            matches!(
                refused.refusal,
                Some(Refusal::WrongNodeKind { .. } | Refusal::Edit(_) | Refusal::Frame(_))
            ),
            "{:?}",
            refused.refusal
        );
        assert!(refused.committed.is_empty(), "and commits nothing");
    }
    assert_eq!(session.history().len(), states, "nothing was recorded");
    assert!(session.committed_doc().bit_eq(&doc), "and nothing changed");
}

/// **The authored path admits any count, and the evaluation judges it**
/// — the door does not judge the count: the form declines to offer a
/// count below zero, and the op vocabulary is not narrowed to what the
/// form offers, because the index's count can be written to the same
/// number afterwards. A count of zero is a family of no member, which
/// evaluates; a count below zero refuses typed on the family's own
/// badge (`IndexCountOutOfRange`).
#[test]
fn a_zero_count_is_a_family_of_no_member_and_a_negative_one_refuses() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    for count in [0, -3] {
        let pattern = session_insert(
            &mut session,
            SessionOp::AddPattern {
                input: body.into(),
                count,
                rule: PatternRuleSpec::Linear {
                    direction: scl3([1.0, 0.0, 0.0]),
                    spacing: len(0.05),
                },
            },
        );
        session.pump();
        let eval = session.evaluation().expect("the inline seam landed");
        if count == 0 {
            let members = eval
                .value(pattern)
                .unwrap_or_else(|| panic!("a family of count {count} evaluates"))
                .members
                .as_ref()
                .map_or(0, |members| members.members.len());
            assert_eq!(members, 0, "a family of count {count} has no member");
        } else {
            assert!(
                matches!(
                    eval.result(pattern),
                    Some(pncad::document::NodeResult::Failed(
                        pncad::document::NodeError {
                            kind: pncad::document::NodeErrorKind::IndexCountOutOfRange {
                                count: -3,
                                ..
                            },
                            ..
                        }
                    ))
                ),
                "a family of count {count} refuses typed: {:?}",
                eval.result(pattern)
            );
        }
    }
}

/// Each tool holds its picks by ROLE, refuses typed until its seats
/// are filled, and drops a pick whose node vanished — naming the seat
/// it emptied.
#[test]
fn each_combining_tool_holds_its_picks_and_survives_a_vanished_one() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);

    // A union holds members in pick order, refuses typed until one is
    // held, and commits whatever it holds from one member on.
    let mut boolean = BooleanTool::new();
    assert!(matches!(
        boolean.op(),
        Err(SeatError::Empty { seat: Seat::Member })
    ));
    boolean.pick(session.committed_doc(), a);
    assert!(matches!(
        boolean.op(),
        Ok(SessionOp::AddBoolean { spec: BooleanSpec::Union(members), .. }) if members == [Operand::from(a)]
    ));
    boolean.pick(session.committed_doc(), b);
    boolean.pick(session.committed_doc(), a);
    assert_eq!(boolean.picks(), [a, b, a], "every pick appends");

    // The subtraction's two seats: the first is the body KEPT, and a
    // further pick replaces the SECOND, so the kept body stays where it
    // was put. Changing the operation re-seats the first two members.
    boolean.set_operation(BooleanOp::Subtract);
    assert_eq!(
        boolean.picks(),
        [a, b],
        "re-seated from the first two members"
    );
    boolean.pick(session.committed_doc(), a);
    assert_eq!(boolean.picks(), [a, a]);
    boolean.pick(session.committed_doc(), b);
    assert!(matches!(
        boolean.op(),
        Ok(SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: first,
                tool: second,
            },
        }) if first == Operand::from(a) && second == Operand::from(b)
    ));

    // Deleting the REMOVED body's node empties that seat and leaves
    // the kept one where it is — no promotion between roles.
    assert!(
        session
            .perform(SessionOp::DeleteNode { node: b })
            .refusal
            .is_none()
    );
    let events = boolean.reconcile(session.committed_doc());
    assert!(
        matches!(
            events.as_slice(),
            [SeatEvent::PickLost {
                seat: Seat::SubtractTool,
                node
            }] if node.id() == b
        ),
        "{events:?}"
    );
    assert_eq!(boolean.picks(), [a]);
    assert!(matches!(
        boolean.op(),
        Err(SeatError::Empty {
            seat: Seat::SubtractTool
        })
    ));
    // The next pick refills the empty seat rather than displacing the
    // held one.
    boolean.pick(session.committed_doc(), a);
    assert_eq!(boolean.picks(), [a, a]);

    // The split tool's one seat is its target; the plane is fields.
    let mut split = SplitTool::new();
    assert!(matches!(
        split.op(len3([0.0; 3]), scl3([0.0, 0.0, 1.0])),
        Err(SeatError::Empty {
            seat: Seat::SplitTarget
        })
    ));
    split.pick(session.committed_doc(), a);
    assert!(matches!(
        split.op(len3([0.0; 3]), scl3([0.0, 0.0, 1.0])),
        Ok(SessionOp::AddSplit { .. })
    ));

    let mut transform = TransformTool::new();
    assert!(matches!(
        transform.op(
            no_translation(),
            scl3([0.0, 0.0, 1.0]),
            Formula::zero(Dimension::Angle)
        ),
        Err(SeatError::Empty {
            seat: Seat::TransformBody
        })
    ));
    transform.pick(session.committed_doc(), b);
    let events = transform.reconcile(session.committed_doc());
    assert!(
        matches!(
            events.as_slice(),
            [SeatEvent::PickLost {
                seat: Seat::TransformBody,
                ..
            }]
        ),
        "{events:?}"
    );
    assert_eq!(transform.input(), None);

    // The pattern tool's one seat is its body, whichever rule; a
    // circular rule's axis is fields.
    let mut pattern = PatternTool::new();
    pattern.pick(session.committed_doc(), a);
    assert!(matches!(
        pattern.linear_op(
            PatternOutputChoice::Instances,
            3,
            scl3([1.0, 0.0, 0.0]),
            len(0.05)
        ),
        Ok(SessionOp::AddPattern { count: 3, .. })
    ));
    assert!(matches!(
        pattern.circular_op(
            PatternOutputChoice::Instances,
            3,
            len3([0.0; 3]),
            scl3([0.0, 0.0, 1.0]),
            ang(1.0)
        ),
        Ok(SessionOp::AddPattern { count: 3, .. })
    ));
    // The output choice picks the op and changes nothing else: the
    // same seat, the same count, the same rule — so an empty seat is
    // the error whichever node the form would have minted.
    assert!(matches!(
        pattern.linear_op(
            PatternOutputChoice::Fused,
            3,
            scl3([1.0, 0.0, 0.0]),
            len(0.05)
        ),
        Ok(SessionOp::AddFusedPattern { count: 3, .. })
    ));
    pattern.clear();
    assert_eq!(pattern.input(), None);
    for output in [PatternOutputChoice::Instances, PatternOutputChoice::Fused] {
        assert!(matches!(
            pattern.circular_op(output, 3, len3([0.0; 3]), scl3([0.0, 0.0, 1.0]), ang(1.0)),
            Err(SeatError::Empty {
                seat: Seat::PatternBody
            })
        ));
    }
}

/// Which tools are actually holding state, read through the per-tool
/// accessors rather than through `open_kind`.
///
/// `open_kind` answers with the ONE open tool's kind, so it cannot
/// tell a leftover from the tool that displaced it: it reads the
/// single `Option<OpenTool>`, and a second tool left behind has no
/// spelling in that value at all. The per-tool accessors are the door
/// the chrome uses and each answers about its own state, so a leftover
/// shows up here as a second `true`. The exclusivity row asserts on
/// this instead.
///
/// The array is `ToolKind::ALL`-wide and its position per kind is the
/// kind's position in `ALL`, so the exclusivity row covers every pair
/// with no count and no order written out twice.
fn open_flags(tools: &Tools) -> [bool; ToolKind::ALL.len()] {
    ToolKind::ALL.map(|kind| match kind {
        ToolKind::Mate => tools.mate().is_some(),
        ToolKind::Revolve => tools.revolve().is_some(),
        ToolKind::Boolean => tools.boolean().is_some(),
        ToolKind::Split => tools.split().is_some(),
        ToolKind::Transform => tools.transform().is_some(),
        ToolKind::Pattern => tools.pattern().is_some(),
        ToolKind::Blend => tools.blend().is_some(),
        ToolKind::Part => tools.part().is_some(),
        ToolKind::Duplicate => tools.duplicate().is_some(),
        ToolKind::Mirror => tools.mirror().is_some(),
    })
}

/// **One modal tool at a time**, and the rule is the tool set's rather
/// than each activation site's: opening any tool closes every other,
/// whichever was open.
#[test]
fn only_one_modal_tool_is_open_at_a_time() {
    let mut tools = Tools::new();
    assert_eq!(
        open_flags(&tools),
        [false; ToolKind::ALL.len()],
        "nothing is open to start"
    );
    for opened in ToolKind::ALL {
        for previous in ToolKind::ALL {
            tools.open(previous);
            tools.open(opened);
            assert_eq!(
                tools.open_kind(),
                Some(opened),
                "opening {opened:?} over {previous:?}"
            );
            let want = ToolKind::ALL.map(|kind| kind == opened);
            assert_eq!(
                open_flags(&tools),
                want,
                "opening {opened:?} over {previous:?} leaves ONLY {opened:?} holding state"
            );
        }
    }
    tools.close();
    assert_eq!(
        open_flags(&tools),
        [false; ToolKind::ALL.len()],
        "close empties every seat"
    );
}

/// **Every seat's wanted kind is the one its own door refuses by.**
///
/// The routing in `viewer::seats` and the gate in `viewer::session`
/// share a CLASSIFIER (`session::admits`) but not a table: which kind
/// a seat wants is stated at `Seat::wants` and again at each door's
/// `require_kind` call. This row is what holds them together — for
/// every seat, its op is driven with a node of the wrong kind and the
/// refusal has to name exactly `seat.wants()`.
///
/// It catches drift in both directions. A `wants()` that disagreed
/// with its door would either pick a "wrong" node the door happily
/// accepts (no refusal, and the row fails) or draw a refusal naming a
/// different kind (and the row fails). `Seat::ALL` is what makes it a
/// sweep rather than a list somebody remembers to extend — and it is
/// projected from `Seat`'s own declaration, so a seat this loop misses
/// is a seat the enum does not have.
#[test]
fn every_seats_wanted_kind_is_the_one_its_door_refuses_by() {
    let tol = Tol::witness();
    let (mut session, body, _) = two_boxes(tol);
    let profile = common::rectangle_in(&mut session, 0.01, 0.01);
    // The two multi-body values the part selectors read. Each is a
    // legal node of SOME kind, so what the part seats refuse is the
    // kind and never the absence — and each is the OTHER's near miss.
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, 0.0, 0.005]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
    );
    let pattern = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
    );
    let right = |wanted: NodeKindWanted| match wanted {
        NodeKindWanted::Body => body,
        NodeKindWanted::Profile => profile,
        NodeKindWanted::Instances => pattern,
    };
    let wrong = |wanted: NodeKindWanted| match wanted {
        NodeKindWanted::Body => profile,
        NodeKindWanted::Profile => body,
        // The near miss the part selector has is a split: both are
        // several bodies, and a split's are read by port, not picked.
        NodeKindWanted::Instances => split,
    };

    for seat in Seat::ALL {
        // Every OTHER seat of the same op is filled correctly, so the
        // refusal can only be about the seat under test.
        let filled = |other: Seat| {
            if other == seat {
                wrong(other.wants())
            } else {
                right(other.wants())
            }
        };
        let op = match seat {
            Seat::RevolveProfile => SessionOp::AddRevolve {
                profile: filled(Seat::RevolveProfile),
                axis_through: [
                    Formula::zero(Dimension::Length),
                    Formula::zero(Dimension::Length),
                ],
                axis_heading: Formula::turn_over(4).expect("a quarter turn"),
                angle: ang(core::f64::consts::TAU),
            },
            // A member beside one filled correctly, so the refusal is
            // about the member under test.
            Seat::Member => SessionOp::AddBoolean {
                spec: BooleanSpec::Union(vec![body.into(), (filled(Seat::Member)).into()]),
            },
            Seat::SubtractFrom | Seat::SubtractTool => SessionOp::AddBoolean {
                spec: BooleanSpec::Subtract {
                    from: (filled(Seat::SubtractFrom)).into(),
                    tool: (filled(Seat::SubtractTool)).into(),
                },
            },
            Seat::SplitTarget => SessionOp::AddSplit {
                target: filled(Seat::SplitTarget).into(),
                origin: len3([0.0, 0.0, 0.005]),
                normal: scl3([0.0, 0.0, 1.0]),
            },
            Seat::TransformBody => SessionOp::AddTransform {
                input: filled(Seat::TransformBody).into(),
                translation: no_translation(),
                rotation_axis: scl3([0.0, 0.0, 1.0]),
                rotation_angle: Formula::zero(Dimension::Angle),
            },
            Seat::PartInstance => SessionOp::AddPart {
                of: filled(Seat::PartInstance),
                select: PartSelectSpec::Instance(0),
            },
            Seat::DuplicateBody => SessionOp::Duplicate {
                input: filled(Seat::DuplicateBody).into(),
                beside: None,
            },
            // The plane a face of the right body, so the refusal is
            // about the body seat.
            Seat::MirrorBody => SessionOp::AddMirror {
                body: filled(Seat::MirrorBody).into(),
                plane: viewer::session::FaceSelection {
                    name: pncad::prelude::StableName {
                        kind: pncad::prelude::EntityKind::Face,
                        node: body,
                        path: vec![pncad::select::RoleSeg::Cap(pncad::select::CapEnd::End)],
                    },
                    node: body,
                    body: 0,
                },
            },
            Seat::PatternBody => SessionOp::AddPattern {
                input: filled(Seat::PatternBody).into(),
                count: 3,
                rule: PatternRuleSpec::Circular {
                    through: len3([0.0; 3]),
                    direction: scl3([0.0, 0.0, 1.0]),
                    step: ang(1.0),
                },
            },
        };
        let out = session.perform(op);
        match out.refusal {
            Some(Refusal::WrongNodeKind { node, wanted }) => {
                assert_eq!(
                    wanted,
                    seat.wants(),
                    "the {} door refuses by {wanted:?}, but the seat says {:?}",
                    seat.name(),
                    seat.wants(),
                );
                assert_eq!(
                    node.id(),
                    wrong(seat.wants()),
                    "it names the node it refused"
                );
            }
            other => panic!(
                "the {} seat took a {:?} without refusing: {other:?}",
                seat.name(),
                seat.wants(),
            ),
        }
    }
}

/// The routing is NARROW: a pick both seats admit follows the plain
/// rule, so a subtraction's operands still fill in order and a third
/// pick still replaces the second — which is what makes `A ∖ B`'s
/// kept body stay where it was put.
///
/// The other half of the same narrowness: a pick NEITHER seat admits
/// is not steered anywhere. It lands where the plain rule puts it and
/// refuses typed at the commit door, which is the refusal's job.
#[test]
fn a_pick_both_seats_admit_and_a_pick_neither_does_follow_the_plain_rule() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    let doc = session.committed_doc();

    let mut boolean = BooleanTool::new();
    boolean.set_operation(BooleanOp::Subtract);
    boolean.pick(doc, a);
    boolean.pick(doc, b);
    boolean.pick(doc, a);
    assert_eq!(
        boolean.picks(),
        [a, a],
        "two body seats: the third pick replaces the second, as before",
    );

    // A profile is no body, so the split tool has no seat to steer it
    // to: it replaces the one pick, as the plain rule says.
    let profile = common::rectangle_in(&mut session, 0.01, 0.01);
    let doc = session.committed_doc();
    let mut split = SplitTool::new();
    split.pick(doc, a);
    split.pick(doc, profile);
    assert_eq!(
        split.target(),
        Some(profile),
        "an unroutable pick lands where the plain rule puts it",
    );
    let op = split
        .op(len3([0.0, 0.0, 0.005]), scl3([0.0, 0.0, 1.0]))
        .expect("the seat is filled");
    let refused = session.perform(op);
    assert!(
        matches!(
            &refused.refusal,
            Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body }) if node.id() == profile
        ),
        "and is refused at the commit door, not here: {:?}",
        refused.refusal
    );
}

/// **A one-seat tool has ONE seat.** Its role is named twice so the
/// pick rule degenerates to "replace", and this is the row that holds
/// it there: the rule underneath fills the first EMPTY slot, so
/// without the arity arm a second pick lands in a slot nothing reads
/// and the transform tool's own door ("a second pick replaces the
/// first") stops being true.
#[test]
fn a_second_pick_replaces_a_one_seat_tools_body() {
    let tol = Tol::witness();
    let (session, a, b) = two_boxes(tol);
    let doc = session.committed_doc();

    let mut transform = TransformTool::new();
    transform.pick(doc, a);
    assert_eq!(transform.input(), Some(a));
    transform.pick(doc, b);
    assert_eq!(transform.input(), Some(b), "the second pick replaces");

    // The survival step sees one seat, so a vanished pick is reported
    // once rather than twice.
    let mut session = session;
    let out = session.perform(SessionOp::DeleteNode { node: b });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let events = transform.reconcile(session.committed_doc());
    assert_eq!(events.len(), 1, "one seat, one drop: {events:?}");
}

/// The open tool consumes the ordinary selection stream at the
/// vocabulary it was written against, and a closed one consumes
/// nothing.
#[test]
fn the_open_tool_consumes_the_selection_stream() {
    let tol = Tol::witness();
    let (session, a, b) = two_boxes(tol);
    let picks = vec![
        SessionOp::Select(Selection::Node(a)),
        SessionOp::Select(Selection::Node(b)),
    ];

    let mut tools = Tools::new();
    // Nothing open: the stream reaches nothing.
    assert!(
        tools
            .feed(
                session.committed_doc(),
                Err(viewer::pickcache::NotIndexed::Absent),
                &picks
            )
            .is_empty(),
        "every pick landed"
    );
    assert_eq!(tools.open_kind(), None);

    tools.open(ToolKind::Boolean);
    assert!(
        tools
            .feed(
                session.committed_doc(),
                Err(viewer::pickcache::NotIndexed::Absent),
                &picks
            )
            .is_empty(),
        "every pick landed"
    );
    let boolean = tools.boolean().expect("the boolean tool is open");
    assert_eq!(boolean.picks(), [a, b]);

    // Opening another tool starts it EMPTY — the picks belonged to
    // the tool that held them.
    tools.open(ToolKind::Pattern);
    assert_eq!(tools.pattern().and_then(|tool| tool.input()), None);
    assert!(
        tools
            .feed(
                session.committed_doc(),
                Err(viewer::pickcache::NotIndexed::Absent),
                &picks[..1]
            )
            .is_empty(),
        "the pick landed"
    );
    assert_eq!(tools.pattern().and_then(|tool| tool.input()), Some(a));

    // The survival step reaches whichever tool is open, from the
    // document alone.
    let mut session = session;
    assert!(
        session
            .perform(SessionOp::DeleteNode { node: a })
            .refusal
            .is_none()
    );
    // `session.doc()` — the SHOWN document, which is what the
    // application hands this call every frame. It is the committed one
    // except under a gesture, and a tool holding picks under a gesture
    // must still see a delete the gesture's scratch does not contain.
    let notices = tools.reconcile(session.doc(), session.landed_pair());
    assert_eq!(notices.len(), 1, "{notices:?}");
    let sentence = notices[0].to_string();
    assert!(
        sentence.starts_with("pattern tool: "),
        "a notice names its tool: {sentence}"
    );
    assert!(
        sentence.contains("patterned body"),
        "and the seat it emptied: {sentence}"
    );
    assert_eq!(tools.pattern().and_then(|tool| tool.input()), None);
}

/// The seat line names WHICH pick is which — the fact that decides
/// what a subtraction removes — and says so before anything is picked.
///
/// Read off each tool's own seats, so the roles and their order are
/// the tool's: a one-seat tool says one item, not its role twice.
#[test]
fn the_seat_line_names_the_roles() {
    let doc = Doc::empty_derived("seat-line", Tol::witness());
    let mut boolean = BooleanTool::new();
    boolean.set_operation(BooleanOp::Subtract);
    assert_eq!(boolean.line(Speaker::of(&doc)), "none picked");
    boolean.pick(&doc, RecipeNodeId::new(0, tagged(3)));
    // The empty document holds neither pick, so each is spoken by its
    // tag alone.
    assert_eq!(
        boolean.line(Speaker::of(&doc)),
        "body kept: node 000000000003; body removed: none picked"
    );
    let mut transform = TransformTool::new();
    transform.pick(&doc, RecipeNodeId::new(0, tagged(7)));
    assert_eq!(
        seat_line(transform.seats(), Speaker::of(&doc)),
        "transformed body: node 000000000007"
    );
}

/// **A dropped pick is called what the panel called it**: the drop
/// notice names the seat and the pick in the words the seat line said
/// them in on the frame before — read off [`seat_line`]'s own output,
/// so a panel and a notice that came to call one held node two things
/// go red here — and a node the document does not hold is spoken by
/// its tag, `node 000000000004`.
#[test]
fn a_lost_picks_notice_names_the_node_as_the_seat_line_does() {
    let doc = Doc::empty_derived("seat-drop", Tol::witness());
    let mut boolean = BooleanTool::new();
    boolean.set_operation(BooleanOp::Subtract);
    boolean.pick(&doc, RecipeNodeId::new(0, tagged(3)));
    boolean.pick(&doc, RecipeNodeId::new(0, tagged(4)));
    let line = boolean.line(Speaker::of(&doc));
    let events = boolean.reconcile(&doc);
    assert_eq!(events.len(), 2, "neither node is in the empty document");
    for event in &events {
        let SeatEvent::PickLost { seat, .. } = event;
        let role = seat.name();
        let held = line
            .split("; ")
            .find_map(|item| item.strip_prefix(&format!("{role}: ")))
            .unwrap_or_else(|| panic!("the line names the {role} seat: {line:?}"));
        assert!(
            event
                .to_string()
                .starts_with(&format!("the {role} pick ({held}) ")),
            "the notice {event} calls the pick what the line {line:?} called it"
        );
    }
    assert_eq!(
        SeatEvent::PickLost {
            seat: Seat::SubtractTool,
            node: viewer::test_support::spoken(RecipeNodeId::new(0, tagged(4)), None),
        }
        .to_string(),
        "the body removed pick (node 000000000004) is no longer in the document; the tool \
         dropped it"
    );
}

/// **A tool closes on the COMMIT, not on the click**, and the rule is
/// asked of the OPEN TOOL rather than of the op alone: an op a
/// different tool authors — or one no tool does — closes nothing.
#[test]
fn a_tool_closes_on_its_own_committed_edit() {
    let edits = [
        (
            ToolKind::Boolean,
            SessionOp::AddBoolean {
                spec: BooleanSpec::Union(vec![
                    (RecipeNodeId::new(0, tagged(1))).into(),
                    (RecipeNodeId::new(0, tagged(2))).into(),
                ]),
            },
        ),
        (
            ToolKind::Split,
            SessionOp::AddSplit {
                target: (RecipeNodeId::new(0, tagged(1))).into(),
                origin: len3([0.0; 3]),
                normal: scl3([0.0, 0.0, 1.0]),
            },
        ),
        (
            ToolKind::Transform,
            SessionOp::AddTransform {
                input: (RecipeNodeId::new(0, tagged(1))).into(),
                translation: no_translation(),
                rotation_axis: scl3([0.0, 0.0, 1.0]),
                rotation_angle: Formula::zero(Dimension::Angle),
            },
        ),
        (
            ToolKind::Pattern,
            SessionOp::AddPattern {
                input: (RecipeNodeId::new(0, tagged(1))).into(),
                count: 2,
                rule: PatternRuleSpec::Linear {
                    direction: scl3([1.0, 0.0, 0.0]),
                    spacing: len(0.05),
                },
            },
        ),
        (
            ToolKind::Revolve,
            SessionOp::AddRevolve {
                profile: RecipeNodeId::new(0, tagged(1)),
                axis_through: [
                    Formula::zero(Dimension::Length),
                    Formula::zero(Dimension::Length),
                ],
                axis_heading: Formula::turn_over(4).expect("a quarter turn"),
                angle: ang(1.0),
            },
        ),
    ];
    let mut tools = Tools::new();
    for (author, op) in &edits {
        for open in ToolKind::ALL {
            tools.open(open);
            assert_eq!(
                tools.commits_open_tool(op),
                open == *author,
                "{op:?} closes {author:?} and nothing else"
            );
        }
    }
    // A creation op no modal tool authors closes nothing, whatever is
    // open — the mate tool included, which closes at its own click.
    for op in [
        SessionOp::AddExtrude {
            profile: RecipeNodeId::new(0, tagged(1)),
            distance: len(0.01),
        },
        SessionOp::Undo,
    ] {
        for open in ToolKind::ALL {
            tools.open(open);
            assert!(!tools.commits_open_tool(&op), "{op:?} with {open:?} open");
        }
    }
    tools.close();
    for (_, op) in &edits {
        assert!(
            !tools.commits_open_tool(op),
            "with nothing open there is nothing to close"
        );
    }
}

/// **The body seat is the operand door's body slot**, and this row is
/// what holds that rather than the prose at
/// [`viewer::combine::denotes_body`].
///
/// The check drives the REAL door: each candidate node is offered as a
/// `Node::Shell`'s target, a slot that reads one `Body`, and the edit
/// door's answer is the verdict — a kind refusal (`SlotVarKind`, or
/// `AmbiguousOutput` for a node of two bodies named alone) means the
/// door refused the candidate as an operand, and acceptance means it
/// did not. Both directions are asserted as one equality: a kind the
/// seat admits that the door refuses is a lie the user meets at the
/// commit, and a kind the seat refuses that the door would take is
/// silent capability loss.
///
/// **What it does not reach**, stated rather than implied: six of the
/// twenty-two node kinds are absent. `Mate`, `Measure` and `Assertion`
/// need substrate this row does not build (a solved assembly, a
/// measured expression) and define no body; `InstantiatePart` needs a
/// resolver with a sibling document on disk, so its body is exercised
/// by the assembly suites instead; the two tube kinds define a body
/// and evaluate through their own doors, exercised by
/// `lib_tube_node`. The sixteen that ARE here — `Part` counted once
/// for its two selectors — include every kind whose classification is
/// load-bearing for this unit.
#[test]
fn the_body_seat_is_the_operand_doors_body_slot() {
    let tol = Tol::witness();
    let mut doc = Doc::empty_derived("operand-door", tol);
    // The substrate every candidate is built out of.
    let (next, profile) = common::inserted(&doc, common::square(0.02), tol);
    doc = next;
    let (next, profile_b) = common::inserted(&doc, common::square(0.02), tol);
    doc = next;
    let (next, ring) = common::inserted(
        &doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::circle(0.05, 0.0, 0.01).expect("finite circle")],
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    doc = next;
    let (next, body) = common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: common::len(0.01),
        },
        tol,
    );
    doc = next;
    // A SECOND independently minted body, stood clear of the first,
    // for the n-ary union candidate, so the union's members share no
    // contact and the row asks only the seat's question. They need not
    // be independently minted — a union keys its names on the member EDGE,
    // so a body and a placement of it are two members and name fine
    // (`docm3_union::two_placements_of_one_prototype_are_two_members`);
    // two extrudes are simply the clearest thing to stand apart.
    let (next, extruded_b) = common::inserted(
        &doc,
        Node::Extrude {
            profile: profile_b.into(),
            distance: common::len(0.01),
        },
        tol,
    );
    doc = next;
    let (next, body_b) = crate::fixture::slid(doc, body, extruded_b, [0.1, 0.0, 0.0]);
    doc = next;
    let (next, other) = crate::fixture::slid(doc, body, body, [0.01, 0.002, 0.002]);
    doc = next;
    // The split's tool: a plane read off the body's own geometry.
    let plane = crate::fixture::plane_on(&doc, body, [0.0, 0.0, 0.005], [0.0, 0.0, 1.0]);
    // A family for the member-read candidate to read: SEVERAL bodies
    // (refused at the seat below, as a candidate in its own right), and
    // a read of one member of it is one.
    let (next, pattern_of_body) =
        common::row_of_copies(&doc, body, 2, (pncad::document::TargetAxis::X, 0.05), tol);
    doc = next;
    let pattern_index = common::family_index(&doc, pattern_of_body);
    let pattern_name = doc.var_name(pattern_index).cloned().expect("a named index");

    // A real edge of the box, for the two blends: an empty blend
    // selection is an AUTHORING refusal, so a minimal fillet has to
    // name an edge, and the shipped `all_edges` door is where a user
    // would get one too.
    // ALL of them: a blend of one box edge terminates at a trivalent
    // corner whose other two edges were not requested, which the
    // kernel refuses by name — the whole-body blend is the shape a
    // minimal instance has to take.
    let edges = {
        let mut probe = DocSession::inline(doc.clone(), tol);
        probe.pump();
        let eval = probe.evaluation().expect("the inline seam landed");
        let edges = pncad::select::all_edges(eval, body);
        assert!(!edges.is_empty(), "the box has edges");
        edges
    };

    // One candidate per node kind this row can build, with the seat's
    // answer beside it. The seat's answer is READ, never restated: a
    // kind that moves sides moves in one place.
    let candidates: Vec<(&str, AuthoredNode)> = vec![
        ("profile", common::square(0.01)),
        (
            "extrude",
            Node::Extrude {
                profile: profile.into(),
                distance: common::len(0.004),
            },
        ),
        (
            "revolve",
            Node::Revolve {
                profile: ring.into(),
                // The revolve's axis is its own: a line in the ring's
                // plane, its +y through (0, 0).
                axis_through: [
                    Formula::zero(Dimension::Length),
                    Formula::zero(Dimension::Length),
                ],
                axis_heading: Formula::turn_over(4).expect("a quarter turn"),
                angle: common::ang(core::f64::consts::TAU),
            },
        ),
        (
            "subtract",
            Node::Subtract {
                from: body.into(),
                tool: other.into(),
            },
        ),
        // The n-ary union at its minimal size. Its two members are
        // `body` and the SECOND extrude's placement rather than `other`
        // because those two stand CLEAR of each other, so the candidate
        // asks only the seat's question. `other` is a placement of
        // `body`, which a union names perfectly well — the member edge
        // tells the two apart — but it sits where `body` is.
        (
            "union",
            Node::Union {
                members: Bodies::Spelled(vec![body.into(), body_b.into()]),
            },
        ),
        (
            "intersect",
            Node::Intersect {
                members: Bodies::Spelled(vec![body.into(), body_b.into()]),
            },
        ),
        (
            "placement",
            crate::fixture::stood(pncad::document::BodyRead::plain(Operand::from(body))),
        ),
        (
            "split",
            Node::Split {
                target: body.into(),
                tool: plane.clone(),
            },
        ),
        (
            "pattern",
            pncad::pattern::linear_pattern(
                &pattern_name,
                pncad::document::BodyRead::plain(Operand::from(body)),
                Operand::Var(doc.node_space(body).expect("the body's space")),
                &[(pncad::document::TargetAxis::X, common::len(0.05))],
            )
            .expect("a length pitch"),
        ),
        // ONE body out of a family: the indexed read is what makes one
        // of several bodies a body at a seat.
        (
            "part of a pattern",
            viewer::combine::part_node(&doc, pattern_of_body, PartSelectSpec::Instance(1))
                .expect("the pattern is in its body's space"),
        ),
        (
            "loft",
            Node::Loft {
                first: profile.into(),
                then: vec![pncad::document::LoftSection {
                    profile: profile_b.into(),
                    station: vec![pncad::document::StationMove::Slide {
                        along: Axis3::Z,
                        by: common::len(0.01),
                    }],
                }],
                v_degree: Formula::count(1),
            },
        ),
        (
            "fillet",
            Node::fillet(body, common::len(0.001), edges.clone()),
        ),
        (
            "chamfer",
            Node::chamfer(body, common::len(0.001), edges.clone()),
        ),
        (
            "family union",
            Node::Union {
                members: Bodies::Family(pncad::document::BodyRead::plain(Operand::from(
                    pattern_of_body,
                ))),
            },
        ),
        (
            "sweep",
            Node::Sweep {
                profile: profile.into(),
                path: profile_b.into(),
                stations: Formula::count(8),
                v_degree: Formula::count(3),
            },
        ),
    ];

    for (name, node) in candidates {
        let (with_candidate, candidate) = common::inserted(&doc, node, tol);
        let admitted = denotes_body(&with_candidate, candidate);
        let probe = pncad::document::apply(
            &with_candidate,
            &pncad::document::DocEdit::InsertNode {
                node: Box::new(Node::shell(candidate, common::len(0.001), Vec::new())),
                fresh: Vec::new(),
            },
            tol,
        );
        let refused_as_operand = matches!(
            probe,
            Err(EditError::SlotVarKind { .. } | EditError::AmbiguousOutput { .. })
        );
        assert_eq!(
            admitted,
            !refused_as_operand,
            "the seat and the operand door disagree about a {name}: {:?}",
            probe.err()
        );
    }
}

/// **An open tool narrows what the cursor may pick to what that tool
/// can actually use**, and a tool that can use either kind narrows
/// nothing.
///
/// Two tools narrow, in opposite directions: the mate tool's alignment
/// frames come off FACE geometry, and the blend tool blends EDGES. The
/// seated tools hold node picks, which a face and an edge answer
/// equally well, so they leave the bare cursor's rule alone.
///
/// The expectation is a match rather than a comparison against one
/// named kind: a new tool has to state which side of this it is on
/// before the row compiles.
#[test]
fn each_tool_narrows_the_cursor_to_what_it_can_use() {
    let mut tools = Tools::new();
    assert_eq!(tools.pick_kinds(), PickKinds::Any, "the bare cursor's rule");
    for kind in ToolKind::ALL {
        tools.open(kind);
        let want = match kind {
            // The mirror tool's plane is a face, and its body is
            // picked by one too.
            ToolKind::Mate | ToolKind::Mirror => PickKinds::FacesOnly,
            ToolKind::Blend => PickKinds::EdgesOnly,
            ToolKind::Revolve
            | ToolKind::Boolean
            | ToolKind::Split
            | ToolKind::Transform
            | ToolKind::Pattern
            | ToolKind::Part
            | ToolKind::Duplicate => PickKinds::Any,
        };
        assert_eq!(tools.pick_kinds(), want, "{kind:?}");
    }
    tools.close();
    assert_eq!(tools.pick_kinds(), PickKinds::Any);
}

// ---------------------------------------------------------------
// AUTH-4: projecting one body out of several, and duplicating one.
// ---------------------------------------------------------------

/// The world's volume — the copies placed into the world, gathered as
/// one `⊔`, as one number.
///
/// **The world and not a node**, because every row below this line
/// is about what the world holds rather than about what one node
/// evaluates to, and the two differ as soon as a body is authored and
/// not placed.
/// The gather is of the landed document with its product set to the
/// world's copies, so the session's own document is not edited.
fn drawn_volume(session: &mut DocSession, tol: Tol) -> f64 {
    session.pump();
    let (landed, _) = session.landed_pair().expect("a run landed");
    let doc = crate::fixture::assemble_world_copies(landed.clone());
    let ev = pncad::document::evaluate::<f64>(
        &doc,
        None,
        &pncad::document::CancelToken::new(),
        &pncad::document::EvalOptions::default(),
        tol,
    );
    let body = pncad::document::product(&doc, &ev, tol).expect("the world gathers");
    pncad::topo::test_support::pinned_volume(
        "mass properties",
        &pncad::topo::AtRestBody::validate(pncad::topo::Body::clone(&body), tol)
            .expect("the measured body passes tier 3"),
        tol,
    )
    .volume
}

/// A box, and a pattern of two of it stepped clear along +x.
fn box_and_pattern(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let pattern = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(A[0] * 2.0),
            },
        },
    );
    (session, body, pattern)
}

/// **A split's half is a body at a seat, read by its port**: a
/// transform's placement of the split's `below` output reads that
/// output's variable, and its body is the lower half.
#[test]
fn a_split_half_is_read_by_its_port_at_a_body_seat() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, 0.0, A[2] / 4.0]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
    );
    let (above, below) = split_volumes(&mut session, split, tol);
    let moved = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: pncad::document::Operand::output(split, SplitHalf::Below.port()),
            translation: no_translation(),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    let doc = session.committed_doc();
    assert!(
        matches!(
            doc.node(moved),
            Some(Node::Place { source, .. })
                if Some(source.read) == doc.output(split, SplitHalf::Below.port())
        ),
        "the transform's placement reads the split's below output"
    );
    let read = body_volume(&mut session, moved, tol);
    assert!(near(read, below), "the lower half {read} vs {below}");
    assert!(
        !near(read, above),
        "and it is the lower half, not the upper {above}",
    );
}

/// **The index crosses the door as an `i64` and lands as an exact
/// Count literal** — [`SessionOp::AddPattern`]'s rule for a count, which
/// a member read's index is.
///
/// The assertion is on the LITERAL the door minted, not on the
/// arithmetic: a door that lowered the index through a continuous
/// expression could still select the right body and would still be
/// the thing spec D3 forbids.
#[test]
fn a_part_indexes_a_patterns_instances_by_an_exact_count() {
    let tol = Tol::witness();
    let (mut session, _body, pattern) = box_and_pattern(tol);
    let part = session_insert(
        &mut session,
        SessionOp::AddPart {
            of: pattern,
            select: PartSelectSpec::Instance(1),
        },
    );
    let Some(Node::Place { source, .. }) = session.committed_doc().node(part) else {
        panic!("the door authored a member read");
    };
    let [index] = source.at[..] else {
        panic!("one index per index of the family: {:?}", source.at);
    };
    assert_eq!(
        Formula::from(
            session
                .committed_doc()
                .written(&Expr::var(index, Dimension::Count))
        ),
        Formula::count(1),
        "the index is the exact Count literal, not a continuous one",
    );
    let one = A[0] * A[1] * A[2];
    let projected = body_volume(&mut session, part, tol);
    assert!(near(projected, one), "one member {projected} vs {one}");
}

/// **An index comes out of a pattern**, and the door refuses a split by
/// naming the kind the selector wanted: a split's halves are its
/// outputs, read by port, never picked by index.
///
/// A fact about the COMMITTED document, so it is the door's question
/// and not evaluation's, the rule [`SessionOp::AddFillet`] states for
/// its target's kind.
#[test]
fn the_part_door_refuses_a_split() {
    let tol = Tol::witness();
    let (mut session, body, _) = box_and_pattern(tol);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, 0.0, A[2] / 4.0]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
    );
    let before = session.committed_doc().clone();

    let instance_of_a_split = session.perform(SessionOp::AddPart {
        of: split,
        select: PartSelectSpec::Instance(0),
    });
    assert!(matches!(
        instance_of_a_split.refusal,
        Some(Refusal::WrongNodeKind {
            wanted: NodeKindWanted::Instances,
            ..
        })
    ));
    assert!(
        session.committed_doc().bit_eq(&before),
        "the refusal moved nothing",
    );
}

/// **A pattern takes its body over, member 0 standing where the body
/// stood** (`the-pattern-tool-stands-member-zero-where-its-body-stands`):
/// of a body nothing copies, the family is drawn in the body's own
/// space and the body is not, so no member is drawn over it; of a body
/// placed into the world, its copy is re-pointed to the family, so the
/// world draws both members there, apart. A member read is the node
/// alone, placed nowhere.
#[test]
fn a_pattern_takes_its_body_over_and_a_member_read_places_nothing() {
    let tol = Tol::witness();
    let (mut session, body, pattern) = box_and_pattern(tol);
    let one = A[0] * A[1] * A[2];
    let doc = session.committed_doc();
    assert_eq!(
        viewer::world::drawn_nodes(doc),
        [pattern],
        "the family is drawn and the body it replicates is not"
    );
    let Some(Node::Place { source, .. }) = doc.node(pattern) else {
        panic!("the pattern is a placement");
    };
    assert_eq!(
        doc.node_space(pattern),
        doc.node_space(body),
        "the family stands in the body's own space"
    );
    assert_eq!(
        Some(source.read),
        doc.output(body, 0),
        "of the body it replicates"
    );

    let part = session.perform(SessionOp::AddPart {
        of: pattern,
        select: PartSelectSpec::Instance(1),
    });
    assert!(part.refusal.is_none(), "{:?}", part.refusal);
    assert_eq!(
        part.committed.len(),
        1,
        "the member read alone, no world placement: {:?}",
        part.committed
    );

    // Placed into the world by hand, the body's copy takes the family.
    let mut session = session_with_box_in_world(tol);
    let (body, copy) = session.1;
    let pattern = session_insert(
        &mut session.0,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(A[0] * 2.0),
            },
        },
    );
    let session = &mut session.0;
    let doc = session.committed_doc();
    let Some(Node::Place { source, .. }) = doc.node(copy) else {
        panic!("the world copy stays a placement");
    };
    assert_eq!(
        Some(source.read),
        doc.output(pattern, 0),
        "the world copy now copies the family"
    );
    let both = drawn_volume(session, tol);
    assert!(
        near(both, one * 2.0),
        "both members drawn in the world: {both}"
    );
    assert_eq!(at_rest_findings(session), 0, "and they do not overlap");
}

/// Box `A` placed into the world by hand: the session, the box and its
/// world copy.
fn session_with_box_in_world(tol: Tol) -> (DocSession, (RecipeNodeId, RecipeNodeId)) {
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let (session, copy) = common::world_copy_in(&session, body, tol);
    (session, (body, copy))
}

/// **A duplicate of a body nothing copies is two copies in its own
/// space**: a stand-in by the identity and one stepped clear, in one
/// action, the stepped one the copy made — so the body is read by
/// both and the picture draws the two copies, apart.
#[test]
fn duplicating_an_unplaced_body_writes_a_stand_in_and_a_stepped_copy() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let own = session.committed_doc().node_space(body);
    let outcome = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [stand_in, stepped] = outcome.minted[..] else {
        panic!("two inserts mint two ids: {:?}", outcome.minted);
    };
    assert_eq!(
        outcome.made,
        Some(stepped),
        "the copy made is the stepped one"
    );
    let doc = session.committed_doc();
    for copy in [stand_in, stepped] {
        let Some(Node::Place { source, .. }) = doc.node(copy) else {
            panic!("the duplicate is a placement");
        };
        assert_eq!(Some(source.read), doc.output(body, 0), "of the body");
        assert_eq!(doc.node_space(copy), own, "in the body's own space");
    }
    assert!(
        common::solve(&session, doc, tol)
            .relative(stand_in)
            .is_some_and(|at| at.is_identity_bits()),
        "the stand-in stands where the body stood"
    );
    assert!(
        spacing_of(&session, stepped) > A[0],
        "the stepped one clears it"
    );
    assert_eq!(
        crate::common::drawn_copies(doc),
        [stand_in, stepped],
        "the body is read by both, and both copies are drawn"
    );
    assert_eq!(at_rest_findings(&mut session), 0, "and they do not overlap");
}

/// **A duplicate of a copy in the world is one more copy there**: the
/// body placed into the world by hand, then duplicated — one placement
/// of the body into the world beside the copy, which the world's volume
/// doubles to.
#[test]
fn duplicating_a_world_copy_places_one_more_copy_beside_it() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let (mut session, original) = common::world_copy_in(&session, body, tol);
    let one = A[0] * A[1] * A[2];
    assert!(near(drawn_volume(&mut session, tol), one), "one to start");

    let outcome = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [copy] = outcome.minted[..] else {
        panic!("one insert: {:?}", outcome.minted);
    };
    let doc = session.committed_doc();
    let Some(Node::Place { source, .. }) = doc.node(copy) else {
        panic!("the duplicate is a placement");
    };
    assert_eq!(
        Some(source.read),
        doc.output(body, 0),
        "of the body it duplicates, not of its copy"
    );
    assert_eq!(
        doc.node_space(copy),
        Some(pncad::document::VarId::WORLD),
        "into the copy's space"
    );
    assert_eq!(
        crate::common::drawn_copies(doc),
        [original, copy],
        "the original's placement is untouched, the copy's after it",
    );
    let both = drawn_volume(&mut session, tol);
    assert!(near(both, one * 2.0), "two copies drawn {both}");
    // Doubled volume alone cannot tell two copies from two copies
    // sharing space — the product gathers shared space twice — so the
    // check the picture itself raises is the witness that they do not
    // meet.
    assert_eq!(at_rest_findings(&mut session), 0, "and they do not overlap");
}

/// **A duplicate saved and reopened is the document it left**, and undo
/// ends at the open: the file is the document's state, so the reopened
/// session has no history in which the duplicate's edits could come
/// apart.
#[test]
fn a_duplicate_saved_and_reopened_has_nothing_to_undo() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let (mut session, _) = common::world_copy_in(&session, body, tol);
    // A duplicate steps clear of the drawn body, so a run lands first.
    drawn_volume(&mut session, tol);
    let outcome = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let world = common::world(session.committed_doc());
    assert_eq!(world.len(), 2, "the original and its copy");

    let dir = common::tempdir("vseam-duplicate-reopen");
    let path = dir.join("duplicated.pncad");
    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    let opened = session.perform(SessionOp::Open(path));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    assert_eq!(
        common::world(session.committed_doc()),
        world,
        "both copies reopen"
    );
    assert!(
        matches!(
            session.perform(SessionOp::Undo).refusal,
            Some(Refusal::NothingToDo { .. })
        ),
        "undo ends at the open"
    );
    assert_eq!(
        common::world(session.committed_doc()),
        world,
        "and the refused undo drew no single copy"
    );
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// **One gesture, one undo**: the two inserts are one action, so one
/// `Undo` puts the document back exactly as it was.
#[test]
fn duplicating_is_one_undo() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let before = session.committed_doc().clone();
    assert!(
        session
            .perform(SessionOp::Duplicate {
                input: body.into(),
                beside: None,
            })
            .refusal
            .is_none()
    );
    assert!(
        session.perform(SessionOp::Undo).refusal.is_none(),
        "one undo",
    );
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo puts back both inserts, bit for bit",
    );
}

/// **The payoff**: the copy moves and the stand-in stays put, both
/// still drawn — a duplicate of a body nothing copies is two copies of
/// it in its own space, so a transform of the stepped one takes that
/// copy alone.
#[test]
fn a_duplicates_copy_moves_on_its_own() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let one = A[0] * A[1] * A[2];
    let outcome = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let copy = outcome.made.expect("the stepped copy");
    let [stand_in, stepped] = outcome.minted[..] else {
        panic!("a stand-in and a stepped copy: {:?}", outcome.minted);
    };
    assert_eq!(stepped, copy, "the copy made is the stepped one");

    let placed = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: copy.into(),
            translation: translation([0.0, A[1] * 3.0, 0.0]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    assert_eq!(
        crate::common::drawn_copies(session.committed_doc()),
        [stand_in, placed],
        "the transform took the stepped copy; the stand-in is untouched",
    );
    session.pump();
    // And the two are apart in SPACE, not merely in the tree: a
    // plane between them cuts each entirely to one side. `split` is
    // the probe because it is a door this suite already drives, and
    // an empty side is a typed value rather than a number to compare.
    let cut_copy = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: placed.into(),
            origin: len3([0.0, A[1] * 2.0, 0.0]),
            normal: scl3([0.0, 1.0, 0.0]),
        },
    );
    let (copy_above, copy_below) = split_volumes(&mut session, cut_copy, tol);
    assert!(
        near(copy_above, one),
        "the moved copy is wholly above the plane"
    );
    assert_eq!(copy_below, 0.0, "and nothing of it is below");

    let cut_original = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, A[1] * 2.0, 0.0]),
            normal: scl3([0.0, 1.0, 0.0]),
        },
    );
    let (original_above, original_below) = split_volumes(&mut session, cut_original, tol);
    assert_eq!(original_above, 0.0, "the original did not move");
    assert!(near(original_below, one), "it is wholly below the plane");
}

/// **The part seats track the part door**, in both directions, over
/// every node kind this suite can build — the shape
/// `the_body_seat_is_the_operand_doors_body_slot` takes for the
/// single-body operand door.
///
/// The check drives the REAL doors: each candidate is offered to a
/// member read ([`viewer::combine::part_node`]), and the verdict is a
/// refusal at the edit door (the read's rank) or the read's own
/// `WrongOperand` (evaluation refusing the VALUE as no family). A
/// placement of a family is a family under the same index, so the
/// instances seat takes it as the door does.
#[test]
fn the_part_seats_track_the_part_door() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, 0.0, A[2] / 4.0]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
    );
    let pattern = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(A[0] * 2.0),
            },
        },
    );
    // A placement of the family, authored past the viewer's own body
    // seat because that seat refuses a family.
    let mut doc = session.committed_doc().clone();
    let placed_pattern = common::insert_into(
        &mut doc,
        crate::fixture::stood(pncad::document::BodyRead::plain(Operand::from(pattern))),
        tol,
    );

    let candidates = [
        ("the split", split),
        ("the pattern", pattern),
        ("the body", body),
        ("a placement of the pattern", placed_pattern),
    ];
    for (name, candidate) in candidates {
        {
            let wanted = NodeKindWanted::Instances;
            let admitted = viewer::session::admits(&doc, candidate, wanted);
            // A candidate in no space has no member read to write.
            let Some(read) =
                viewer::combine::part_node(&doc, candidate, PartSelectSpec::Instance(0))
            else {
                assert!(
                    !admitted,
                    "the {wanted:?} seat admits {name}, which is in no space"
                );
                continue;
            };
            let refused = match pncad::document::apply(
                &doc,
                &pncad::document::DocEdit::InsertNode {
                    node: Box::new(read),
                    fresh: Vec::new(),
                },
                tol,
            ) {
                Err(_) => true,
                Ok(applied) => {
                    let part = applied.record.minted.expect("an insert mints its node");
                    let mut probe = DocSession::inline(applied.doc, tol);
                    probe.pump();
                    let eval = probe.evaluation().expect("the inline seam landed");
                    matches!(
                        eval.result(part).and_then(NodeResult::error),
                        Some(NodeError {
                            kind: NodeErrorKind::WrongOperand { .. },
                            ..
                        })
                    )
                }
            };
            assert_eq!(
                admitted, !refused,
                "the {wanted:?} seat and the part door disagree about {name}"
            );
        }
    }
}

/// **A body seat reads a split's half by its port**: a pick on a
/// placed half's copy seats that half, and a split picked in the tree
/// waits for its half — the commit refuses typed until one is chosen,
/// and then reads that port.
#[test]
fn a_body_seat_reads_a_split_half_by_its_port() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, B);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, 0.0, B[2] / 4.0]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
    );
    let below = pncad::document::Operand::output(split, SplitHalf::Below.port());
    let (doc, placed) = common::placed_read(session.committed_doc(), below.clone(), tol);

    // A pick on the placed half's copy seats the half it places.
    let mut tool = TransformTool::new();
    tool.pick_at(&doc, placed, Some(0));
    assert_eq!(tool.input(), Some(split));
    assert_eq!(tool.seats().read(0), Some(below.clone()));

    // A tree pick of the split names neither half: the commit refuses
    // naming the seat, and reads the half once it is chosen.
    let mut tool = TransformTool::new();
    tool.pick(&doc, split);
    assert!(tool.seats().awaits_half(0));
    let op = |tool: &TransformTool| {
        tool.op(
            no_translation(),
            scl3([0.0, 0.0, 1.0]),
            Formula::zero(Dimension::Angle),
        )
    };
    assert!(matches!(
        op(&tool),
        Err(SeatError::HalfUnchosen {
            seat: Seat::TransformBody
        })
    ));
    tool.choose_half(0, SplitHalf::Below);
    assert!(matches!(
        op(&tool),
        Ok(SessionOp::AddTransform { input, .. }) if input == below
    ));
    assert!(
        seat_line(tool.seats(), Speaker::of(&doc)).ends_with("below half"),
        "{}",
        seat_line(tool.seats(), Speaker::of(&doc))
    );
}

/// **Each of the two tools closes on its own committed edit**, and on
/// nothing else — the rule that makes a tool modal.
#[test]
fn the_part_and_duplicate_tools_close_on_their_own_edits() {
    let node = RecipeNodeId::new(0, tagged(1));
    let part = SessionOp::AddPart {
        of: node,
        select: PartSelectSpec::Instance(0),
    };
    let duplicate = SessionOp::Duplicate {
        input: node.into(),
        beside: None,
    };
    let mut tools = Tools::new();

    tools.open(ToolKind::Part);
    assert!(tools.commits_open_tool(&part));
    assert!(!tools.commits_open_tool(&duplicate));

    tools.open(ToolKind::Duplicate);
    assert!(tools.commits_open_tool(&duplicate));
    assert!(!tools.commits_open_tool(&part));
}

// ---------------------------------------------------------------
// What a VIEWPORT pick seats, and where a duplicate lands.
// ---------------------------------------------------------------

/// A face picked through the session's real ray path, looking straight
/// down at `(x, y)` — the viewport's own door, not a hand-built
/// selection.
fn picked_from_above(session: &DocSession, x: f64, y: f64) -> Selection {
    Selection::Face(common::asm::pick_face(session, &common::asm::down_at(x, y)))
}

/// A duplicate's step, read back off the committed stepped placement:
/// its slide along each target axis, which the door authors literal.
fn step_of(session: &DocSession, copy: RecipeNodeId) -> [f64; 3] {
    use pncad::document::{Coord, TargetAxis};
    let doc = session.committed_doc();
    let Some(Node::Place { constraints, .. }) = doc.node(copy) else {
        panic!("a duplicate is a stepped placement");
    };
    [TargetAxis::X, TargetAxis::Y, TargetAxis::Z].map(|axis| {
        constraints
            .iter()
            .filter_map(|c| c.value())
            .find(|value| value.coord == Coord::Slide(axis))
            .map(|value| doc.written(&Expr::var(value.by, Dimension::Length)))
            .filter(|written| !written.is_zero())
            .map_or(0.0, |written| {
                written.literal_value().expect("a literal slide")
            })
    })
}

/// The at-rest findings the landed checks report.
fn at_rest_findings(session: &mut DocSession) -> usize {
    session.pump();
    let report = session.checks().expect("the registry ran");
    report
        .findings
        .iter()
        .filter(|f| f.check == pncad::document::CheckId::AtRest)
        .count()
}

/// Box `A` duplicated, and the copy moved `lift` along +y — the
/// workflow the gesture exists for. Answers the session, the moved
/// copy's node and where its centre now is in plan.
fn a_moved_copy(tol: Tol, lift: f64) -> (DocSession, RecipeNodeId, [f64; 2]) {
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let outcome = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let copy = outcome.made.expect("the stepped copy");
    let step = spacing_of(&session, copy);
    let moved = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: copy.into(),
            translation: translation([0.0, lift, 0.0]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    session.pump();
    (session, moved, [step, lift])
}

/// **A viewport click seats the body that was DRAWN, not the feature
/// that made the face** — for every tool whose seat takes a body.
///
/// The picked face on a moved copy was minted by the ORIGINAL extrude
/// (a transform mints no name), so `Selection::node` — the feature
/// tree's question — answers the extrude, which sits where the
/// original is. A tool that took that answer would author against a
/// body the user did not click.
#[test]
fn a_viewport_pick_seats_the_drawn_body_in_every_body_seat() {
    let tol = Tol::witness();
    let (session, moved, [x, y]) = a_moved_copy(tol, 0.05);
    let picked = picked_from_above(&session, x, y);
    let Selection::Face(face) = &picked else {
        unreachable!("the helper answers a face")
    };
    assert_eq!(
        face.node,
        common::drawn_node(session.committed_doc(), moved),
        "the ray met the moved copy, drawn by its placement"
    );
    assert_ne!(
        face.feature(),
        moved,
        "and the face's FEATURE is the extrude upstream — the two answers differ here",
    );
    let doc = session.committed_doc();
    for kind in ToolKind::ALL {
        let mut tools = Tools::new();
        tools.open(kind);
        let _ = tools.feed(
            doc,
            Err(viewer::pickcache::NotIndexed::Absent),
            &[SessionOp::Select(picked.clone())],
        );
        let held = match kind {
            ToolKind::Boolean => tools.boolean().and_then(|t| t.picks().first().copied()),
            ToolKind::Split => tools.split().and_then(|t| t.target()),
            ToolKind::Transform => tools.transform().and_then(|t| t.input()),
            ToolKind::Pattern => tools.pattern().and_then(|t| t.input()),
            ToolKind::Duplicate => tools.duplicate().and_then(|t| t.input()),
            ToolKind::Mirror => tools.mirror().and_then(|t| t.input()),
            // Seats a split or a pattern, never a body: its own row
            // below picks one.
            ToolKind::Part => continue,
            // Faces and edges, taken whole rather than as a node; and
            // a profile and an axis, which a face pick never is.
            ToolKind::Mate | ToolKind::Blend | ToolKind::Revolve => continue,
        };
        assert_eq!(
            held,
            Some(moved),
            "the {kind:?} tool seats the body the drawn copy places"
        );
    }
}

/// **Duplicating the copy you just moved duplicates THAT copy** —
/// Ev's workflow, end to end through a real pick and the tool's own
/// commit door.
#[test]
fn duplicating_a_moved_copy_picked_in_the_viewport_duplicates_the_copy() {
    let tol = Tol::witness();
    let (mut session, moved, [x, y]) = a_moved_copy(tol, 0.05);
    let picked = picked_from_above(&session, x, y);
    let mut tools = Tools::new();
    tools.open(ToolKind::Duplicate);
    let _ = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[SessionOp::Select(picked)],
    );
    let op = tools
        .duplicate()
        .expect("open")
        .op()
        .expect("the pick filled the seat");
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let copy = outcome.minted[0];
    let Some(Node::Place { source, .. }) = session.committed_doc().node(copy) else {
        panic!("a placement");
    };
    assert_eq!(
        Some(source.read),
        session.committed_doc().output(moved, 0),
        "the duplicate copies the moved copy"
    );
    assert_eq!(
        at_rest_findings(&mut session),
        0,
        "and no copy landed on top of another",
    );
}

/// **The part tool seats a pattern picked in the tree**: a pick on a
/// family of copies is on the family, not on the body it copies.
#[test]
fn the_part_tool_seats_a_pattern_picked_in_the_tree() {
    let tol = Tol::witness();
    let (mut session, _body, pattern) = box_and_pattern(tol);
    session.pump();
    let mut tools = Tools::new();
    tools.open(ToolKind::Part);
    let _ = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[SessionOp::Select(Selection::Node(pattern))],
    );
    let tool = tools.part().expect("open");
    assert_eq!(
        tool.pattern(),
        Some(pattern),
        "the pattern lands in the seat"
    );
    assert!(matches!(
        tool.instance_op(1),
        Ok(SessionOp::AddPart { of, .. }) if of == pattern
    ));
}

/// **A duplicate lands clear of its original** — no at-rest
/// finding, which is the check the picture actually raises when two
/// bodies share space. Doubled volume cannot show it: the product
/// gathers overlapping space twice.
#[test]
fn a_duplicate_lands_clear_of_its_original() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let outcome = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        at_rest_findings(&mut session),
        0,
        "the copies do not overlap"
    );
    let step = spacing_of(&session, outcome.made.expect("the stepped copy"));
    assert!(
        step > A[0],
        "the step clears the body's own width along the step: {step} vs {}",
        A[0],
    );
}

/// **The step keeps the note's promise** — clear of the original by
/// AT LEAST [`DUPLICATE_GAP`] of the body's width — and it is the
/// rule's exact number against box `A`'s closed form: its width along
/// +x, inflated by twice the measuring chord plus the kernel ε (the
/// tessellation's conservative bound — the box's floor mesh is its
/// eight corners, so its diagonal is exact), then the gap on top.
///
/// And for a plate THIN along the step, where the inflation is large
/// against the width: the promise is a floor, so the step may exceed
/// a quarter-width by a lot, and must never fall short of it.
#[test]
fn a_duplicate_keeps_the_notes_promise() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let outcome = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let diagonal = (A[0] * A[0] + A[1] * A[1] + A[2] * A[2]).sqrt();
    let chord = diagonal * MEASURE_CHORD;
    let want = (A[0] + 2.0 * (chord + tol.eps())) * (1.0 + DUPLICATE_GAP);
    let got = spacing_of(&session, outcome.made.expect("the stepped copy"));
    assert!(near(got, want), "spacing {got} vs the rule's {want}");
    assert!(
        got >= A[0] * (1.0 + DUPLICATE_GAP),
        "at least a quarter-width clear"
    );
    let step = step_of(&session, outcome.made.expect("the stepped copy"));
    assert_eq!(
        step.map(|c| c / got),
        STEP_DIRECTION,
        "along the one stepping direction"
    );

    // A 1 × 100 × 100 mm plate, thin along +x.
    let plate = [0.001, 0.1, 0.1];
    let mut session = self::session(tol);
    let body = common::xy_box_in(&mut session, plate);
    session.pump();
    let outcome = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let got = spacing_of(&session, outcome.made.expect("the stepped copy"));
    assert!(
        got >= plate[0] * (1.0 + DUPLICATE_GAP),
        "the floor holds on a thin body: {got}"
    );
    assert_eq!(
        at_rest_findings(&mut session),
        0,
        "and the plates do not overlap"
    );
}

/// **A body whose value is several bodies is refused, typed** — a
/// placement of a family, which is several bodies again. Its variable
/// is a body's, so the body seat takes it, and the duplicate door
/// refuses it by its value (`DuplicateFault::NotOneBody`).
#[test]
fn duplicating_a_several_body_value_is_refused() {
    let tol = Tol::witness();
    let (session, _body, pattern) = box_and_pattern(tol);
    // Past the viewer's own body seat, which refuses the pattern
    // itself: a loaded document can hold this shape.
    let mut doc = session.committed_doc().clone();
    let placed = common::insert_into(
        &mut doc,
        crate::fixture::stood(pncad::document::BodyRead::plain(Operand::from(pattern))),
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::SetLabel {
        node: placed,
        label: Some(pncad::document::Label::new("moved").expect("a label")),
    });
    session.pump();
    let before = session.committed_doc().clone();
    let out = session.perform(SessionOp::Duplicate {
        input: placed.into(),
        beside: None,
    });
    let said = out
        .refusal
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_default();
    assert!(
        said.starts_with("Place \"moved\" ("),
        "it names the input as labelled: {said}"
    );
    assert!(
        matches!(
            &out.refusal,
            Some(Refusal::Duplicate(DuplicateFault::NotOneBody { input })) if input.id() == placed
        ),
        "{:?}",
        out.refusal
    );
    assert!(session.committed_doc().bit_eq(&before), "nothing committed");
}

/// **A failed body has nothing to copy, and the refusal says its
/// standing**: which node, what state, where the repair is.
#[test]
fn duplicating_a_failed_body_says_its_standing() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    assert!(
        session
            .perform(SessionOp::SetSlot {
                node: body,
                slot: SlotId::Distance,
                value: viewer::props::SlotValue::of(Dimension::Length, 0.0)
                    .expect("a finite length is a value"),
            })
            .refusal
            .is_none()
    );
    session.pump();
    let before = session.committed_doc().clone();
    let out = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    let standing = NodeStanding::Failed { node: body };
    let Some(Refusal::Duplicate(fault)) = &out.refusal else {
        panic!("a duplicate refusal, got {:?}", out.refusal);
    };
    assert!(
        matches!(fault, DuplicateFault::NoValue { standing: carried, .. } if *carried == standing),
        "{fault:?}"
    );
    assert_eq!(
        fault.to_string(),
        format!(
            "there is no body to copy: {}",
            pncad::document::spoken_by(&standing, session.committed_doc())
        )
    );
    assert!(session.committed_doc().bit_eq(&before), "nothing committed");
}

/// **Nothing landed, nothing to measure** — the step is read off the
/// body, so before the first landing the door says so rather than
/// guessing a length.
#[test]
fn duplicating_before_anything_has_landed_is_refused() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let out = session.perform(SessionOp::Duplicate {
        input: body.into(),
        beside: None,
    });
    assert!(
        matches!(
            out.refusal,
            Some(Refusal::Duplicate(DuplicateFault::NotLanded))
        ),
        "{:?}",
        out.refusal
    );
}

/// **A split half clicked in the viewport lands in a body seat by its
/// port** — where the world places a half by its port, a pick on that
/// copy is a read of the half it places.
#[test]
fn a_body_seat_takes_a_split_half_clicked_in_the_viewport() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, 0.0, A[2] / 2.0]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
    );
    // The world places the two halves: the split took the box over.
    let mut doc = session.committed_doc().clone();
    for port in [0, 1] {
        doc = common::placed_read(&doc, pncad::document::Operand::output(split, port), tol).0;
    }
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let picked = picked_from_above(&session, 0.0, 0.0);
    let mut tools = Tools::new();
    tools.open(ToolKind::Transform);
    let _ = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[SessionOp::Select(picked)],
    );
    let tool = tools.transform().expect("open");
    assert_eq!(tool.input(), Some(split));
    assert_eq!(
        tool.seats().read(0),
        Some(pncad::document::Operand::output(
            split,
            SplitHalf::Above.port()
        )),
        "the half the ray met from above"
    );
}

/// **A duplicate is not measured off a stale picture.** A box turned
/// 90° about y, so its extrude distance is its width along +x, lands;
/// then the distance grows tenfold and — before that edit lands — the
/// box is duplicated. The landed evaluation still holds the OLD body,
/// so a step read off it would be the old width and the copy would
/// land inside the grown original. The door refuses instead, typed.
#[test]
fn a_duplicate_is_not_measured_off_a_picture_older_than_the_document() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let extrude = common::xy_box_in(&mut session, [0.01, 0.02, 0.01]);
    let turned = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: extrude.into(),
            translation: no_translation(),
            rotation_axis: scl3([0.0, 1.0, 0.0]),
            rotation_angle: ang(core::f64::consts::FRAC_PI_2),
        },
    );
    session.pump();
    let grown = session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: viewer::props::SlotValue::Continuous(0.1),
    });
    assert!(grown.refusal.is_none(), "{:?}", grown.refusal);
    assert!(
        session.busy(),
        "the premise: the picture is older than the document"
    );
    let before = session.committed_doc().clone();
    let out = session.perform(SessionOp::Duplicate {
        input: turned.into(),
        beside: None,
    });
    assert!(
        matches!(out.refusal, Some(Refusal::Duplicate(DuplicateFault::Stale))),
        "{:?}",
        out.refusal
    );
    assert!(session.committed_doc().bit_eq(&before), "nothing committed");
    // And once the edit lands, the same gesture measures the grown body
    // and clears it.
    session.pump();
    let out = session.perform(SessionOp::Duplicate {
        input: turned.into(),
        beside: None,
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(
        spacing_of(&session, out.made.expect("the stepped copy")) > 0.1,
        "the step clears the grown width"
    );
    assert_eq!(
        at_rest_findings(&mut session),
        0,
        "and the copies do not overlap"
    );
}

/// **A union across two flush contacts lands as one action.** The
/// block has a channel cut across its top, so its top is TWO faces, and
/// a boss bridging the channel rests on both. The union commits one
/// step. Red if it refuses, or lands anything but the channelled block
/// plus the boss.
#[test]
fn a_union_across_two_flush_contacts_lands_as_one_action() {
    const CHANNEL: [f64; 3] = [0.01, 0.04, 0.01];
    const CHANNEL_DROP: f64 = 0.005;
    const BOSS: [f64; 3] = [0.03, 0.01, 0.004];
    let tol = Tol::witness();
    let mut session = session(tol);
    let block = common::xy_box_in(&mut session, common::BOSS_BLOCK);
    // The channel: through the block's whole depth and 5 mm into its
    // top, with no face coplanar with one of the block's.
    let raw = common::xy_box_in(&mut session, CHANNEL);
    let channel = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: raw.into(),
            translation: translation([0.0, 0.0, CHANNEL_DROP]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    let channelled = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: block.into(),
                tool: channel.into(),
            },
        },
    );
    // The boss, stood on the block's top: its own box, moved up by the
    // block's height.
    let (_, raw_boss) = common::box_in(&mut session, BOSS);
    let boss = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: raw_boss.into(),
            translation: translation([0.0, 0.0, common::BOSS_BLOCK[2]]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    session.pump();

    let steps = session.history().len();
    let outcome = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![channelled.into(), boss.into()]),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [union] = outcome.minted[..] else {
        panic!("one union: {:?}", outcome.minted);
    };
    assert_eq!(session.history().len(), steps + 1, "one action, one step");
    let [width, depth, height] = common::BOSS_BLOCK;
    let cut = CHANNEL[0] * depth * (height - CHANNEL_DROP);
    let want = width * depth * height - cut + BOSS[0] * BOSS[1] * BOSS[2];
    let got = body_volume(&mut session, union, tol);
    assert!(
        near(got, want),
        "channelled block plus boss: {got} vs {want}"
    );
}

/// **The boolean tool's picks carry their ports.** Two tree picks of a
/// split wait for their halves (`HalfUnchosen` until chosen); the chosen
/// ports survive a change of operation both ways, members to seats and
/// back; a pending pick survives the change too; and the union of the
/// two halves commits.
#[test]
fn the_boolean_tools_picks_carry_their_ports_across_the_operation() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, B);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: len3([0.0, 0.0, B[2] / 4.0]),
            normal: scl3([0.0, 0.0, 1.0]),
        },
    );
    let doc = session.committed_doc().clone();
    let port = |h: SplitHalf| Operand::output(split, h.port());
    let mut tool = BooleanTool::new();
    tool.pick(&doc, split);
    tool.pick(&doc, split);
    assert_eq!(tool.awaiting_half(), vec![0, 1], "both picks wait");
    assert!(matches!(
        tool.spec(),
        Err(SeatError::HalfUnchosen { seat: Seat::Member })
    ));
    tool.choose_half(1, SplitHalf::Below);
    assert_eq!(tool.awaiting_half(), vec![0], "the second chose");
    tool.choose_half(0, SplitHalf::Above);
    assert_eq!(
        tool.spec().expect("both chosen"),
        BooleanSpec::Union(vec![port(SplitHalf::Above), port(SplitHalf::Below)])
    );
    tool.set_operation(BooleanOp::Subtract);
    assert_eq!(
        tool.spec().expect("seated"),
        BooleanSpec::Subtract {
            from: port(SplitHalf::Above),
            tool: port(SplitHalf::Below),
        },
        "members to seats keep their ports"
    );
    tool.set_operation(BooleanOp::Intersect);
    assert_eq!(
        tool.spec().expect("members"),
        BooleanSpec::Intersect(vec![port(SplitHalf::Above), port(SplitHalf::Below)]),
        "seats to members keep their ports"
    );
    let mut pending = BooleanTool::new();
    pending.pick(&doc, split);
    pending.set_operation(BooleanOp::Subtract);
    assert_eq!(
        pending.awaiting_half(),
        vec![0],
        "a pending pick stays pending"
    );
    pending.choose_half(0, SplitHalf::Below);
    pending.pick(&doc, body);
    assert_eq!(
        pending.spec().expect("both seated"),
        BooleanSpec::Subtract {
            from: port(SplitHalf::Below),
            tool: body.into(),
        }
    );
    let outcome = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![port(SplitHalf::Above), port(SplitHalf::Below)]),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}

/// **A session op's body read that names a variable is read as that
/// variable's output, and one that names no output refuses typed**: a
/// duplicate of a body read by its variable lands, and a transform of a
/// selection refuses `NotABodyRead`.
#[test]
fn a_body_read_by_variable_lands_and_a_selection_refuses_typed() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let var = session
        .committed_doc()
        .read_of_node(body)
        .expect("a box defines one body");
    let duplicated = session.perform(SessionOp::Duplicate {
        input: Operand::Var(var),
        beside: None,
    });
    assert!(duplicated.refusal.is_none(), "{:?}", duplicated.refusal);
    let refused = session.perform(SessionOp::AddTransform {
        input: Operand::Select {
            body: Box::new(body.into()),
            names: Vec::new(),
        },
        translation: no_translation(),
        rotation_axis: scl3([0.0, 0.0, 1.0]),
        rotation_angle: Formula::zero(Dimension::Angle),
    });
    assert!(
        matches!(refused.refusal, Some(Refusal::NotABodyRead { .. })),
        "{:?}",
        refused.refusal
    );
}

/// **Taking over one half of a split leaves the other's copy**: two
/// placements draw the two halves of a split, and a transform of the
/// lower half re-points the placement copying that half alone, so the
/// upper half is still drawn and the lower half is drawn once.
#[test]
fn taking_over_one_half_of_a_split_leaves_the_other() {
    let tol = Tol::witness();
    let (session, _, split) = common::drawn::split_box(A, tol);
    let at = common::drawn::translated([0.1, 0.0, 0.0]);
    let (session, upper) = common::drawn::half_placed(&session, split, SplitHalf::Above, &at, tol);
    let (mut session, lower) =
        common::drawn::half_placed(&session, split, SplitHalf::Below, &at, tol);
    let doc = session.committed_doc();
    let above = doc.output(split, SplitHalf::Above.port());
    let below = Operand::output(split, SplitHalf::Below.port());
    let moved = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: below,
            translation: translation([0.0, 0.0, -0.02]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    let doc = session.committed_doc();
    let read = |placement| match doc.node(placement) {
        Some(Node::Place { source, .. }) => Some(source.read),
        other => panic!("the placement stands, not {other:?}"),
    };
    assert_eq!(
        [read(upper), read(lower)],
        [above, doc.output(moved, 0)],
        "the upper half kept, the lower half taken over by the transform"
    );
}

/// **A combine that would strand a pick refuses at the takeover door,
/// typed, and records nothing.** A `⊔` of two placed boxes is placed in
/// the world, and a pick of its first piece is placed beside it. The
/// union of the `⊔` with the second box takes the `⊔`'s world copy over
/// onto the union, which holds no pieces; a pick is never re-pointed,
/// so the takeover refuses rather than leave the pick naming a piece
/// its copy no longer holds.
#[test]
fn a_combine_that_would_strand_a_pick_refuses_typed() {
    use pncad::document::{BodyRead, VarId};
    let tol = Tol::witness();
    let mut session = session(tol);
    let a = common::xy_box_in(&mut session, A);
    let b = common::xy_box_in(&mut session, B);
    // Both placed into the world by hand.
    let (doc, _) = common::world_copy(session.committed_doc(), a, tol);
    let (doc, _) = common::world_copy(&doc, b, tol);
    let copy = |body| {
        doc.output(common::copy_of(&doc, body), 0)
            .expect("a world copy")
    };
    let (a_copy, b_copy) = (copy(a), copy(b));
    let place = |read: Operand| {
        Node::place(
            BodyRead::plain(read),
            Operand::Var(VarId::WORLD),
            pncad::document::stand().bundle(),
        )
    };
    let (doc, seat) = crate::fixture::insert(
        doc.clone(),
        Node::Assemble {
            members: Bodies::Spelled(vec![
                BodyRead::plain(Operand::Var(a_copy)),
                BodyRead::plain(Operand::Var(b_copy)),
            ]),
        },
    );
    let (doc, held) = crate::fixture::insert(doc, place(Operand::Node(seat)));
    let (doc, _) =
        crate::fixture::insert(doc, place(Operand::pick(Operand::Node(held), vec![a_copy])));
    let mut session = crate::common::inline_without_door(doc, tol);
    session.pump();
    let steps = session.history().len();
    let out = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![seat.into(), b.into()]),
    });
    assert!(
        matches!(
            &out.refusal,
            Some(Refusal::StrandedPick { copy, .. }) if copy.id() == held
        ),
        "{:?}",
        out.refusal
    );
    assert_eq!(session.history().len(), steps, "nothing recorded");
}

/// **The split plane's verdict is the door's, asked before commit**: the
/// split the panel would commit, staged and evaluated beside the
/// picture, read as its tree row would read it, with nothing recorded.
/// A plane through the origin with normal z lies on the block's bottom
/// face and leaves the below half empty; a plane above the block misses
/// it; a plane through the block's centre cuts it in two. That the
/// panel seeds that centre is `drive.rs`'s
/// `the_split_tool_seeds_its_plane_through_the_picked_plate_and_draws_it_there`.
///
/// *Breaks if* staging records an edit, or the verdict stops being the
/// split's own.
#[test]
fn a_staged_split_says_whether_its_plane_cuts_the_body() {
    use viewer::session::{Creation, Staged};
    use viewer::tree::{Emptiness, split_reading};

    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let states = session.history().len();
    let verdict = |session: &mut DocSession, origin: [f64; 3]| {
        let creation = Creation::of(SessionOp::AddSplit {
            target: body.into(),
            origin: len3(origin),
            normal: scl3([0.0, 0.0, 1.0]),
        })
        .expect("a split creates a node");
        assert!(
            session.staged_for(&creation).is_none(),
            "{origin:?}: nothing is staged for a plane not yet asked about"
        );
        let staged = session.perform(SessionOp::Stage {
            creation: creation.clone(),
        });
        assert!(
            staged.refusal.is_none() && staged.committed.is_empty(),
            "{origin:?}: a stage refuses nothing and commits nothing: {staged:?}"
        );
        session.pump();
        match session.staged_for(&creation) {
            Some(Staged::Evaluated {
                node, evaluation, ..
            }) => split_reading(evaluation, node).expect("the staged split evaluates"),
            other => panic!("{origin:?}: the staged run landed inline: {other:?}"),
        }
    };
    assert_eq!(
        verdict(&mut session, [0.0; 3]),
        Some(Emptiness::Half(SplitHalf::Below)),
        "the old default lies on the bottom face"
    );
    assert_eq!(
        verdict(&mut session, [0.0, 0.0, 1.0]),
        Some(Emptiness::Half(SplitHalf::Above)),
        "a plane above the block misses it"
    );
    let centre = {
        let (doc, evaluation) = session.landed_pair().expect("landed");
        let source = viewer::splitplane::source(doc, evaluation, &body.into()).expect("a body");
        viewer::splitplane::centre(source).expect("the block has corners")
    };
    assert!(
        near(centre[2], 0.5 * A[2]),
        "the seed is the block's centre: {centre:?}"
    );
    assert_eq!(
        verdict(&mut session, centre),
        None,
        "the seeded plane cuts the block in two"
    );
    assert_eq!(session.history().len(), states, "staging recorded nothing");
}

/// **The split plane is drawn under the part its target was picked
/// on**, through the path the viewport takes (`splitplane::drawn`, over
/// the tools, the session and a built pick index). A viewport pick on
/// the block's world copy keeps that copy, and the plane is drawn where
/// it stands. A tree pick hit no part, so nothing is drawn. A target
/// whose frame is a map (a transform's copy, translated, so its body's
/// `in_space` is `Some`) is drawn where its own copy stands, the
/// translation applied once.
///
/// *Breaks if* the seat forgets the part, the preview stops reading the
/// index's drawn map, or a target's own frame is applied twice or not
/// at all.
#[test]
fn the_split_plane_is_drawn_under_the_picked_copy() {
    use pncad::prelude::StableName;
    use viewer::session::FaceSelection;

    let tol = Tol::witness();
    let z = [0.0, 0.0, 1.0];
    let pick_on = |copy: RecipeNodeId| {
        SessionOp::Select(Selection::Face(FaceSelection {
            name: StableName {
                kind: pncad::prelude::EntityKind::Face,
                node: copy,
                path: Vec::new(),
            },
            node: copy,
            body: 0,
        }))
    };
    let drawn = |session: &DocSession, picked: SessionOp| {
        let index = common::plate_index(session);
        let mut tools = Tools::new();
        tools.open(ToolKind::Split);
        let declined = tools.feed(session.committed_doc(), Ok(&index), &[picked]);
        assert!(declined.is_empty(), "{declined:?}");
        let tool = tools.split().expect("the split tool is open");
        let (doc, evaluation) = session.landed_pair().expect("landed");
        let target = tool.seats().read(0).expect("a target");
        let source = viewer::splitplane::source(doc, evaluation, &target).expect("a body");
        let centre = viewer::splitplane::centre(source).expect("corners");
        let plane = viewer::splitplane::drawn(
            &viewer::splitplane::Drawing {
                seats: tool.seats(),
                landed: (doc, evaluation),
                display: &session.display_view(),
                index: &index,
            },
            centre,
            z,
        );
        (tool, source.clone(), centre, plane)
    };

    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    // Placed into the world by hand: a new body lands unplaced.
    let (mut session, copy) = common::world_copy_in(&session, body, tol);
    let (tool, _, centre, plane) = drawn(&session, pick_on(copy));
    assert_eq!(
        tool.target(),
        Some(body),
        "a pick on the copy seats the body"
    );
    assert_eq!(
        tool.seats().through(0),
        Some(copy),
        "the seat keeps the part it hit"
    );
    let plane = plane.expect("the plane is drawn on the picked copy");
    assert!(
        (plane.centre.z - 0.5 * A[2]).abs() < 1e-12 && near(centre[2], 0.5 * A[2]),
        "the plane is drawn at the block's mid-height: {:?}",
        plane.centre
    );
    let (tool, _, _, plane) = drawn(&session, SessionOp::Select(Selection::Node(body)));
    assert_eq!(tool.seats().through(0), None, "a tree pick hits no part");
    assert!(plane.is_none(), "a tree pick draws no plane");

    let lift = 4.0 * A[2];
    let moved = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: body.into(),
            translation: translation([0.0, 0.0, lift]),
            rotation_axis: scl3(z),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    );
    session.pump();
    let [copy] = viewer::world::placements_of(session.committed_doc(), moved)[..] else {
        panic!("the block's world copy now places the transform")
    };
    let (tool, source, centre, plane) = drawn(&session, pick_on(copy));
    assert_eq!(
        tool.target(),
        Some(moved),
        "the pick seats the transform's copy"
    );
    assert!(
        pncad::document::in_space(&source).is_some(),
        "the transform's body is framed by a map"
    );
    assert!(
        near(centre[2], 0.5 * A[2] + lift),
        "the seed is in the space the copy is built in: {centre:?}"
    );
    let plane = plane.expect("the plane is drawn on the lifted copy");
    assert!(
        (plane.centre.z - (0.5 * A[2] + lift)).abs() < 1e-12,
        "the lift is applied once: {:?}",
        plane.centre
    );
}

/// **A staged verdict is not carried across a document replacement**
/// (review of PR 6240): the stage is keyed on the creation and the
/// history state, and a history id is an arena position — every
/// freshly opened document's root is the same id. A split staged on a
/// file must not answer for that file re-opened after it was edited
/// and saved elsewhere: same node ids, same root id, a flatter block.
///
/// *Breaks if* `Open`/`NewDocument` leave the old stage standing.
#[test]
fn a_staged_verdict_does_not_survive_reopening_an_edited_file() {
    use viewer::session::{Creation, Staged};
    use viewer::tree::{Emptiness, split_reading};

    let tol = Tol::witness();
    let path = common::tempdir("staged-across-open").join("block.pncad");
    let mut writer = session(tol);
    let body = common::xy_box_in(&mut writer, A);
    writer.pump();
    let saved = writer.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "the tall block saves: {saved:?}");

    let creation = Creation::of(SessionOp::AddSplit {
        target: body.into(),
        origin: len3([0.0, 0.0, 0.5 * A[2]]),
        normal: scl3([0.0, 0.0, 1.0]),
    })
    .expect("a split creates a node");
    let verdict = |session: &DocSession| match session.staged_for(&creation) {
        Some(Staged::Evaluated {
            node, evaluation, ..
        }) => Some(split_reading(evaluation, node).expect("the staged split evaluates")),
        None => None,
        other => panic!("the staged run landed inline: {other:?}"),
    };

    let mut reader = session(tol);
    assert!(
        reader
            .perform(SessionOp::Open(path.clone()))
            .refusal
            .is_none()
    );
    reader.pump();
    reader.perform(SessionOp::Stage {
        creation: creation.clone(),
    });
    reader.pump();
    assert_eq!(
        verdict(&reader),
        Some(None),
        "mid-height cuts the tall block"
    );

    // The writer flattens the block and saves over the file.
    let flattened = writer.perform(SessionOp::SetSlot {
        node: body,
        slot: SlotId::Distance,
        value: viewer::props::SlotValue::of(Dimension::Length, 0.2 * A[2])
            .expect("a finite length is a value"),
    });
    assert!(flattened.refusal.is_none(), "{flattened:?}");
    writer.pump();
    let saved = writer.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "the flat block saves: {saved:?}");

    assert!(reader.perform(SessionOp::Open(path)).refusal.is_none());
    reader.pump();
    assert_eq!(
        verdict(&reader),
        None,
        "nothing is staged on the re-opened (flat) block yet; the tall block's \
         verdict would say the plane cuts it"
    );
    reader.perform(SessionOp::Stage {
        creation: creation.clone(),
    });
    reader.pump();
    assert_eq!(
        verdict(&reader),
        Some(Some(Emptiness::Half(SplitHalf::Above))),
        "staged afresh, the plane passes over the flat block"
    );
}
