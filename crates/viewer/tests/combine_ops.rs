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
//! placed and moved by the Transform tool, Place then the editor): a
//! 40 × 20 × 10 mm block and
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
use pncad::document::BodyRead;
use pncad::topo::BooleanOp;
use test_utils::refusal::tagged;
use viewer::session::BooleanSpec;

use common::{ang, body_volume, circle_at, len, near, session_insert};
use pncad::document::BooleanValue;
use pncad::document::Speaker;
use pncad::document::SplitSide;
use pncad::document::{
    Axis3, Dimension, DimensionError, Doc, DocEdit, EditError, Expr, Formula, LoopProgram, Node,
    NodeError, NodeErrorKind, NodeResult, Operand, OperandSlot, ProfileProgram, RecipeNodeId,
    SlotId,
};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use pncad::select::SplitHalf;
use viewer::combine::{BooleanTool, PatternOutputChoice, PatternTool, SplitTool, denotes_body};
use viewer::pickindex::PickKinds;
use viewer::seats::{Seat, SeatError, SeatEvent, seat_line};
use viewer::session::{
    DocSession, GestureFrameFault, NodeKindWanted, PartSelectSpec, PatternRuleSpec, Refusal,
    Selection, SessionOp,
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

/// The two overlapping blocks (module docs), the second moved by the
/// Transform tool and its copy's "into" row set to the first block's
/// space (a combine moves no copy), each then given a world copy by
/// hand, the product left empty: a document these rows combine over,
/// which the product door would not write — it moves the Transform
/// tool's placement, which nothing reads, into the world rather than copy
/// it. Answers the session and the two BODY nodes, in the order the
/// module's constants name them.
fn two_boxes(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let mut session = session(tol);
    let a = common::xy_box_in(&mut session, A);
    let raw_b = common::xy_box_in(&mut session, B);
    let b = common::moved_to(&mut session, raw_b, OFFSET);
    let home = session
        .committed_doc()
        .node_space(a)
        .expect("a is in a space");
    common::into_space(&mut session, b, home);
    let (session, _) = common::world_copy_by_hand_in(&session, a, tol);
    let (session, _) = common::world_copy_by_hand_in(&session, b, tol);
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
                &pncad::topo::finish(pncad::topo::Body::clone(body.body()), tol)
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
    let c = common::moved_to(&mut session, raw_c, [0.0, 0.05, 0.0]);
    let home = session.committed_doc().node_space(a).expect("a's space");
    common::into_space(&mut session, c, home);
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
        [a, b],
        "the union of the bodies picked as themselves writes no placement"
    );
    assert_eq!(
        viewer::world::repoint_offer(doc, union).map(|offer| offer.body),
        doc.output(a, 0),
        "and offers to re-point a's placements onto it"
    );
    let va = A[0] * A[1] * A[2];
    let vb = B[0] * B[1] * B[2];
    assert_volume(&mut session, union, va + 2.0 * vb - OVERLAP, tol);
}

/// **Nothing places as a side effect, and a combine of two copies
/// picked in the viewport is their union, where they stand** (spec §9
/// row 9, the viewer's half; Ev, PR 6353).
///
/// Two placed bodies `a`, `b` (`b` moved), their world copies picked:
/// the combine reads the two copies, so it is one union in the world and
/// nothing else — no placement re-pointed, none deleted — and the world
/// draws the union alone, its operands read: the union of the two as
/// they stand. One undo restores
/// `[a, b]` bit for bit. Breaks if the gesture reads through a copy to
/// its body, writes a placement, or records more than one action.
#[test]
fn a_combine_of_two_copies_is_their_union_and_writes_no_placement() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    session.pump();
    let before = session.committed_doc().clone();
    assert_eq!(common::world(&before), [a, b], "both operands are placed");
    let [a_placement, b_placement] = crate::common::drawn_copies(&before)[..] else {
        panic!("two copies: {:?}", crate::common::drawn_copies(&before));
    };
    let outcome = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![a_placement.into(), b_placement.into()]),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let union = outcome.minted[0];
    assert!(
        matches!(&outcome.committed[..], [DocEdit::InsertNode { .. }]),
        "the union alone: {:?}",
        outcome.committed
    );
    let doc = session.committed_doc();
    assert_eq!(
        viewer::world::drawn_nodes(doc),
        [union],
        "the union is drawn, its operands read"
    );
    assert!(
        crate::common::drawn_copies(doc).is_empty(),
        "and no copy is drawn beside it"
    );
    let va = A[0] * A[1] * A[2];
    let vb = B[0] * B[1] * B[2];
    assert_volume(&mut session, union, va + vb - OVERLAP, tol);

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo restores [a, b], bit for bit"
    );
}

/// **A combine reads its operands in their spaces, not in the world's
/// picture of them.** `a`'s world placement is slid 0.1 m along `x`,
/// so `a`'s space stands off `b`'s in the world and the two drawn
/// copies no longer overlap. The union of the bodies picked as
/// themselves still takes `b` into `a`'s space with its values as
/// written — it stands on `a`'s axes as it stood on its own body's — so
/// the union overlaps as the two bodies do in their own coordinates.
/// Keeping `b`'s world position instead would compose the two world
/// bundles into numbers, a frame read back.
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
    let slide_x = pncad::document::Coord::slide(pncad::document::TargetAxis::X);
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
    let union = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![a.into(), b.into()]),
        },
    );
    assert_volume(&mut session, union, va + vb - OVERLAP, tol);
}

/// **A combine of bodies picked as themselves places nothing, and
/// offers the re-point of the first operand the world places**: `a`
/// unplaced and `b` placed, their union writes no placement, and offers
/// to re-point `b`'s placement onto it (Ev, PR 6353); accepted, the
/// world shows the union where `b`'s copy stood.
#[test]
fn a_combine_offers_the_re_point_of_the_operand_the_world_places() {
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
    let doc = session.committed_doc();
    assert_eq!(common::world(doc), [b], "the union writes no placement");
    let offer = viewer::world::repoint_offer(doc, union).expect("the union offers the re-point");
    assert_eq!(Some(offer.body), doc.output(b, 0), "of b's placement");
    let out = session.perform(SessionOp::RepointPlacements {
        body: pncad::document::Operand::Var(offer.body),
        onto: union,
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert_eq!(
        common::world(session.committed_doc()),
        [union],
        "accepted, b's placement places the union"
    );
}

/// **A pattern of a body picked as itself re-points no placement**:
/// two identity placements of one body are two copies (A10); a pattern
/// of the body writes its index and its family and nothing else, and
/// both copies still place the body (Ev, PR 6353). A pattern is itself
/// a placement of the body, so it offers no re-point.
#[test]
fn a_pattern_of_a_body_picked_as_itself_re_points_no_placement() {
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
    let (doc, first) = common::placed(&doc, body, tol);
    let (doc, second) = common::placed(&doc, body, tol);
    let mut session = DocSession::inline(doc, tol);
    let row = || SessionOp::AddPattern {
        input: body.into(),
        count: 2,
        rule: PatternRuleSpec::Linear {
            direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
            spacing: len(A[0] * 2.0),
        },
    };
    let body_read = session.committed_doc().output(body, 0);
    for pass in ["first", "second"] {
        let outcome = session.perform(row());
        assert!(outcome.refusal.is_none(), "{pass}: {:?}", outcome.refusal);
        assert!(
            matches!(
                &outcome.committed[..],
                [DocEdit::DeclareVar { .. }, DocEdit::InsertNode { .. }]
            ),
            "{pass}: its index declared and its family inserted: {:?}",
            outcome.committed
        );
        let doc = session.committed_doc();
        assert_eq!(
            [first, second].map(|copy| doc.copied_read(copy)),
            [body_read; 2],
            "{pass}: the copies still place the body"
        );
        let family = outcome.made.expect("the family");
        assert_eq!(
            viewer::world::repoint_offer(doc, family),
            None,
            "{pass}: a pattern offers no re-point"
        );
    }
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
            loops: vec![circle_at([0.0, 0.0], 0.01)],
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

/// A split's target seat wants a body, its plane is read in the
/// target's own space, and what it authors cuts.
#[test]
fn the_split_door_takes_a_body_and_a_plane_in_its_space() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let cut = 0.004;
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![circle_at([0.0, 0.0], 0.01)],
        },
    );
    let refused = session.perform(SessionOp::AddSplit {
        target: profile.into(),
        plane: common::cut_across(&session, profile, cut),
    });
    assert!(
        matches!(
            &refused.refusal,
            Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body }) if node.id() == profile
        ),
        "{:?}",
        refused.refusal
    );
    // A plane read in another body's space refuses typed, before any
    // edit.
    let other = common::xy_box_in(&mut session, A);
    let states = session.history().len();
    let refused = session.perform(SessionOp::AddSplit {
        target: body.into(),
        plane: common::cut_across(&session, other, cut),
    });
    assert!(
        matches!(
            refused.refusal.as_ref(),
            Some(Refusal::Edit(error)) if matches!(**error, EditError::SpaceMismatch { .. })
        ),
        "{:?}",
        refused.refusal
    );
    assert_eq!(session.history().len(), states, "a refusal records nothing");

    let plane = common::cut_across(&session, body, cut);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    let (up, down) = split_volumes(&mut session, split, tol);
    let face = A[0] * A[1];
    assert!(near(up, face * (A[2] - cut)), "above the cut: {up}");
    assert!(near(down, face * cut), "below the cut: {down}");
}

/// The L block's footprint, metres: a 40 × 10 mm leg along x and a
/// 10 × 10 mm leg standing on its left end, so its two inner walls are
/// the planes `y = 10 mm` and `x = 10 mm`, each cutting the other leg.
const L_FOOTPRINT: [(f64, f64); 6] = [
    (0.0, 0.0),
    (0.04, 0.0),
    (0.04, 0.01),
    (0.01, 0.01),
    (0.01, 0.02),
    (0.0, 0.02),
];
/// Its height.
const L_HEIGHT: f64 = 0.01;

/// **The split tool cuts along a picked face's plane, and its plane
/// re-picks** (`crates/viewer/README.md`, "The seated tools whose values
/// have a home"). An L block's inner wall at `y = 10 mm`, picked in the
/// viewport, seats the block and the plane at once, and the committed
/// split cuts the block there: 10 × 10 mm on one side, 40 × 10 mm on
/// the other. Re-picking the plane moves the cut to the inner wall at
/// `x = 10 mm` (20 × 10 and 30 × 10 mm): before commit, by the tool's
/// next face pick; after it, by the split's tool slot written through
/// the operand slot door (`DocEdit::SetParam`).
///
/// *Breaks if* a face pick stops seating the plane, the split stops
/// cutting along the face's plane, or the slot door stops re-pointing
/// it.
#[test]
fn the_split_tool_cuts_along_a_picked_face_and_its_plane_re_picks() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![LoopProgram::polygon(L_FOOTPRINT).expect("finite corners")],
        },
    );
    let block = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(L_HEIGHT),
        },
    );
    session.pump();
    let index = common::plate_index(&session);
    let mid = 0.5 * L_HEIGHT;
    let y_wall = common::displayed_face_at(&session, &index, &common::along_y(-1.0, 0.025, mid));
    let x_wall = common::displayed_face_at(&session, &index, &common::along_x(-1.0, 0.015, mid));
    let leg = 0.01 * 0.01 * L_HEIGHT;
    let feed = |tools: &mut Tools, session: &DocSession, face: &viewer::session::FaceSelection| {
        let declined = tools.feed(
            session.committed_doc(),
            Err(viewer::pickcache::NotIndexed::Absent),
            &[SessionOp::Select(Selection::Face(face.clone()))],
        );
        assert!(declined.is_empty(), "the pick is taken: {declined:?}");
    };

    let mut tools = Tools::new();
    tools.open(ToolKind::Split);
    feed(&mut tools, &session, &y_wall);
    let tool = tools.split().expect("the split tool is open");
    assert_eq!(tool.target(), Some(block), "the face pick seats its body");
    assert_eq!(tool.face(), Some(&y_wall), "and is the plane");
    let op = tool.op().expect("a target and a plane are held");
    assert!(tools.commits_open_tool(&op), "the split is the tool's edit");
    let split = session_insert(&mut session, op);
    // The face's plane faces out of the face: `above` (its port,
    // `SplitHalf::Above`) is the side the wall faces, the standing leg.
    let (above, below) = split_volumes(&mut session, split, tol);
    assert!(
        near(above, leg) && near(below, 4.0 * leg),
        "the cut is the inner wall at y = 10 mm, above it the standing leg: {above} / {below}"
    );

    // Re-picked before commit: the next face pick is the plane, and the
    // target stays.
    feed(&mut tools, &session, &x_wall);
    let tool = tools.split().expect("the split tool is open");
    assert_eq!(tool.target(), Some(block), "a re-pick keeps the target");
    assert_eq!(tool.face(), Some(&x_wall), "and moves the plane");

    // Re-picked after commit, through the operand slot door.
    let doc = session.committed_doc().clone();
    let plane = pncad::document::Operand::Pose(Box::new(pncad::document::pose::PoseDef::Plane {
        face: Operand::select(block, vec![x_wall.name.clone()]),
    }));
    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetParam {
            node: split,
            slot: SlotId::Operand(OperandSlot::Tool),
            value: pncad::document::SlotValue::Read(plane),
            fresh: Vec::new(),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    let (above, below) = split_volumes(&mut session, split, tol);
    assert!(
        near(above, 3.0 * leg) && near(below, 2.0 * leg),
        "the re-picked cut is the inner wall at x = 10 mm, above it the long leg's end: \
         {above} / {below}"
    );
}

/// **A row's default spacing is the clearing step rounded up to a round
/// figure in the working unit** (`combine::row_spacing`): at least the
/// step, so the copies still clear, and one significant figure in the
/// unit the person works in, as a blend's default size is.
///
/// Red if the spacing is written as the raw measure, rounded down, or
/// rounded in another unit than the working one.
#[test]
fn a_rows_default_spacing_is_its_step_clear_of_rounded_up() {
    use pncad::quantity::{CM, IN, M, MM};
    let doc = Doc::<ProfileProgram>::empty_derived("row-spacing", Tol::witness());
    let at = |clears: f64, unit| {
        doc.unparse(&viewer::combine::row_spacing(clears, unit).expect("a finite step"))
    };
    let clears = 0.051_790_071_130_842_12;
    assert_eq!(
        [
            at(clears, MM),
            at(clears, CM),
            at(clears, M),
            at(clears, IN)
        ],
        ["60 mm", "6 cm", "0.06 m", "3 in"],
        "rounded up, at one significant figure in the working unit"
    );
    assert_eq!(at(0.05, MM), "50 mm", "a round step stays as it is");
    // A step just past a round figure goes to the next one: the round
    // figure would read back below it.
    for (clears, unit, want) in [
        (3.000_000_000_000_000_3e-4, MM, "0.4 mm"),
        (0.0762, IN, "4 in"),
    ] {
        let spacing = viewer::combine::row_spacing(clears, unit).expect("a positive step");
        assert_eq!(doc.unparse(&spacing), want, "{clears} m in {unit:?}");
        assert!(
            spacing.literal_value().expect("a written length") >= clears,
            "{clears} m: never below the step"
        );
    }
    for clears in [0.0, -0.01, f64::NAN] {
        assert!(
            matches!(
                viewer::combine::row_spacing(clears, MM),
                Err(viewer::combine::RowSpacingFault::NotPositive { .. })
            ),
            "{clears} m is no step to space a row by"
        );
    }
}

/// **A split half splits** (`work/doors/a-split-half-cannot-be-split.md`):
/// the block split at half its height, its lower half split again at a
/// quarter, each side of the second split a quarter of the block. The
/// plane is the block's start cap slid up, which the half, in the
/// block's space, is cut along.
///
/// Red if a split refuses a half for a target, or reads the whole
/// split's value where the seat names one half.
#[test]
fn a_split_half_splits() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let plane = common::cut_across(&session, body, A[2] / 2.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    let plane = common::cut_across(&session, body, A[2] / 4.0);
    let again = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: Operand::output(split, SplitHalf::Below.port()),
            plane,
        },
    );
    let quarter = A.iter().product::<f64>() / 4.0;
    let (above, below) = split_volumes(&mut session, again, tol);
    assert!(
        near(above, quarter) && near(below, quarter),
        "the lower half cut in two quarters: {above} / {below}"
    );
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
    let plane = common::cut_across(&session, body, 0.004);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    {
        let wrong = split;
        for op in [
            SessionOp::AddBoolean {
                spec: BooleanSpec::Union(vec![wrong.into(), body.into()]),
            },
            SessionOp::place_where_it_stands(session.committed_doc(), wrong.into())
                .expect("a body in a space"),
            SessionOp::AddSplit {
                target: wrong.into(),
                plane: common::cut_across(&session, wrong, 0.004),
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

/// **The Transform tool places a body rigidly**: on a body nothing
/// places, Place with no mate, into the body's own space, and the editor
/// on the placement it made; the slides and the spin about `z` typed on
/// its value rows, the copy stands at the frame they make, and the
/// values are slots the property panel then owns.
#[test]
fn the_transform_door_places_a_body_with_values() {
    use pncad::document::{Coord, TargetAxis};
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let mut values = vec![(
        Coord::Spin(Some(TargetAxis::Z)),
        ang(core::f64::consts::FRAC_PI_2),
    )];
    values.extend(common::slides(OFFSET));
    let op = common::transformed(&mut session, body, &values);
    let SessionOp::SetBundle {
        placement: placed,
        constraints,
        ..
    } = &op
    else {
        panic!("the editor on the placement Place made writes its bundle: {op:?}")
    };
    let placed = *placed;
    assert!(
        constraints.iter().all(|c| c.mate().is_none()),
        "no mate: {op:?}"
    );
    let doc = session.committed_doc();
    assert!(
        matches!(doc.node(placed), Some(Node::Place { source, target, .. })
            if Some(source.read) == doc.read_of_node(body)
                && Some(*target) == doc.operand_space(&body.into())),
        "Place placed the body into its own space"
    );
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    // A rigid placement preserves the volume, and the copy stands at
    // the frame the typed numbers make, in its body's own space.
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
    let poses = pncad::document::solve_document(
        doc,
        &pncad::document::EvalOptions::default(),
        &pncad::document::CancelToken::new(),
        tol,
    );
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
            pncad::document::Coord::slide(pncad::document::TargetAxis::X),
            Dimension::Length,
        ),
        (
            pncad::document::Coord::slide(pncad::document::TargetAxis::Y),
            Dimension::Length,
        ),
        (
            pncad::document::Coord::slide(pncad::document::TargetAxis::Z),
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

    // What a value row still refuses is a wrong-DIMENSION quantity —
    // an angle where a slide belongs — typed, before the draft holds it.
    assert!(matches!(
        viewer::matetool::typed_value(
            session.committed_doc(),
            Coord::slide(TargetAxis::X),
            "turn/4"
        ),
        Err(viewer::matetool::TypedValueError::Dimension {
            found: Dimension::Angle,
            ..
        })
    ));
}

/// **The Transform tool opens the placement editor on its pick**, in
/// the tool's place and with no mate row: on a copy, that copy's
/// placement, its bundle as written; on a body picked as itself, Place —
/// whether nothing places it, one copy does, or two do — so a body shown
/// again beside its copy is placed a second time the way it was placed
/// the first.
#[test]
fn the_transform_tool_opens_the_placement_editor_on_its_pick() {
    use viewer::matetool::MateTool;
    let tol = Tol::witness();
    let (mut session, a, _) = two_boxes(tol);
    let fresh = common::xy_box_in(&mut session, B);
    let doc = session.committed_doc().clone();
    let opened = |doc: &Doc<ProfileProgram>, node: RecipeNodeId| {
        let mut tools = Tools::new();
        tools.open(ToolKind::Transform);
        let notices = tools.feed(
            doc,
            Err(viewer::pickcache::NotIndexed::Absent),
            &[SessionOp::Select(Selection::Node(node))],
        );
        (tools, notices)
    };
    let placed = |doc: &Doc<ProfileProgram>, body: RecipeNodeId| {
        let (mut tools, notices) = opened(doc, body);
        assert!(notices.is_empty(), "{notices:?}");
        let editor = tools.mate().expect("the editor opened in the tool's place");
        assert!(editor.door().is_none(), "no placement until Place lands");
        assert_eq!(
            tools.take_place(),
            Some(
                SessionOp::place_where_it_stands(session.committed_doc(), body.into())
                    .expect("a body in a space")
            ),
            "a pick on {body:?} as itself asks for Place"
        );
    };

    placed(&doc, fresh);
    let copy = common::copy_of(&doc, a);
    let edit = MateTool::edit(&doc, copy).expect("a copy is a placement");
    let (mut tools, notices) = opened(&doc, copy);
    assert!(notices.is_empty(), "{notices:?}");
    assert_eq!(
        tools.mate(),
        Some(&edit),
        "a pick on the copy opens its placement, its bundle as written"
    );
    assert_eq!(tools.take_place(), None, "and asks for no Place");
    placed(&doc, a);
    let (doc, _) = common::inserted(&doc, common::world_place(a.into()), tol);
    placed(&doc, a);
}

/// **The Transform tool opens no door on what is no body**, typed: a
/// profile is not placeable, a pattern's family is moved by its rule
/// (the body it reads, picked as itself, is a body and is placed), and
/// a split picked whole waits for the half that is the body, whose
/// choice then asks for that half's Place.
#[test]
fn the_transform_tool_refuses_what_is_no_body_and_asks_a_split_for_its_half() {
    use viewer::matetool::MateToolEvent;
    use viewer::tools::ToolNotice;
    let tol = Tol::witness();
    let mut session = session(tol);
    let (profile, body) = common::box_in(&mut session, A);
    let plane = common::cut_across(&session, body, A[2] / 2.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    let patterned = common::xy_box_in(&mut session, B);
    let family = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: patterned.into(),
            count: 3,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(B[0] * 2.0),
            },
        },
    );
    let doc = session.committed_doc().clone();
    let picked = |node: RecipeNodeId| {
        let mut tools = Tools::new();
        tools.open(ToolKind::Transform);
        let notices = tools.feed(
            &doc,
            Err(viewer::pickcache::NotIndexed::Absent),
            &[SessionOp::Select(Selection::Node(node))],
        );
        (tools, notices)
    };

    let (tools, notices) = picked(profile);
    assert!(
        matches!(&notices[..], [ToolNotice::Mate(MateToolEvent::NotPlaceable { node })] if node.id() == profile),
        "a profile: {notices:?}"
    );
    assert_eq!(tools.open_kind(), Some(ToolKind::Transform));

    let (tools, notices) = picked(family);
    assert!(
        matches!(&notices[..], [ToolNotice::Mate(MateToolEvent::PatternFamily { family: f })] if f.id() == family),
        "the family is moved by its pattern's rule: {notices:?}"
    );
    assert!(tools.mate().is_none(), "no door opens on the family");
    // The body the pattern reads, picked as itself, is a body: Place.
    let (mut tools, notices) = picked(patterned);
    assert!(notices.is_empty(), "{notices:?}");
    assert_eq!(
        tools.take_place(),
        Some(
            SessionOp::place_where_it_stands(session.committed_doc(), patterned.into())
                .expect("a body in a space")
        ),
        "the patterned body picked as itself is placed"
    );

    let (mut tools, notices) = picked(split);
    assert!(notices.is_empty(), "{notices:?}");
    assert_eq!(
        tools
            .transform()
            .and_then(|tool| tool.awaiting_half())
            .map(|s| s.id()),
        Some(split),
        "the split waits for its half"
    );
    let notices = tools.transform_half(&doc, SplitHalf::Below);
    assert!(notices.is_empty(), "{notices:?}");
    assert_eq!(
        tools.take_place(),
        Some(
            SessionOp::place_where_it_stands(
                session.committed_doc(),
                Operand::output(split, SplitHalf::Below.port())
            )
            .expect("a body in a space")
        ),
        "the chosen half's Place"
    );
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
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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

    // The circular rule's axis is one of the space's axes, or picked
    // geometry's; a flat face picked for a ring's axis has none, and
    // refuses typed.
    let circular = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 4,
            rule: PatternRuleSpec::Circular {
                axis: viewer::PatternDirection::Axis(pncad::document::TargetAxis::Z),
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
            axis: viewer::PatternDirection::Face(end_cap(body)),
            step: ang(1.0),
        },
    });
    assert!(
        matches!(
            refused.refusal.as_ref(),
            Some(Refusal::Frame(fault)) if matches!(**fault, GestureFrameFault::NoHeading { .. })
        ),
        "{:?}",
        refused.refusal
    );
}

/// The end cap of the extrude `body`, as a viewport pick on it selects
/// it.
fn end_cap(body: RecipeNodeId) -> viewer::session::FaceSelection {
    viewer::session::FaceSelection {
        name: pncad::prelude::StableName {
            kind: pncad::prelude::EntityKind::Face,
            node: body,
            path: vec![pncad::select::RoleSeg::Cap(pncad::select::CapEnd::End)],
        },
        node: body,
        body: 0,
    }
}

/// Each member's offset from member 0 of the family `pattern`
/// evaluated to, in the space the family stands in.
fn member_offsets(session: &mut DocSession, pattern: RecipeNodeId) -> Vec<[f64; 3]> {
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let ValuePayload::Instances(instances) =
        &eval.value(pattern).expect("the family evaluated").payload
    else {
        panic!("a family evaluates to its members");
    };
    let instances = common::every_body(instances);
    instances
        .iter()
        .map(|member| {
            instances[0]
                .relative(member)
                .expect("members of one space relate")
                .map_or([0.0; 3], |motion| {
                    let o = motion.origin();
                    [o.x, o.y, o.z]
                })
        })
        .collect()
}

/// A row of `count` members of a fresh block, stepped `spacing` along
/// the target axis `along`, and the session holding it; `declare` names
/// a length variable declared first, at 50 mm, for `spacing` to read.
fn a_row(
    along: pncad::document::TargetAxis,
    declare: Option<&str>,
    spacing: Formula,
    count: i64,
) -> (DocSession, RecipeNodeId) {
    let mut session = session(Tol::witness());
    let body = common::xy_box_in(&mut session, A);
    if let Some(name) = declare {
        let declared = session.perform(SessionOp::DeclareVar {
            name: pncad::document::VarName::new(name).expect("a name"),
            value: pncad::document::FreeVar::continuous(Dimension::Length, 0.05),
        });
        assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    }
    let row = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(along),
                spacing,
            },
        },
    );
    (session, row)
}

/// Whether member `k` of the row `row` stands `k·pitch` from member 0
/// along the unit of `typed`, for every `k`; the message names the
/// first that does not.
fn stepped(session: &mut DocSession, row: RecipeNodeId, typed: [f64; 3], pitch: f64) {
    let norm = typed.iter().map(|c| c * c).sum::<f64>().sqrt();
    for (k, got) in (0u32..).zip(member_offsets(session, row)) {
        let want = typed.map(|c| c / norm * pitch * f64::from(k));
        assert!(
            got.iter().zip(want).all(|(g, w)| (g - w).abs() < 1e-12),
            "direction {typed:?}, pitch {pitch}: member {k} stands at {got:?}, not {want:?}"
        );
    }
}

/// **A row along an axis steps its spacing, and its pitch reads the
/// variable it was typed as**: member `k` stands `k·pitch` along `x`,
/// and still does after `pitch` is set again.
#[test]
fn a_row_along_an_axis_steps_its_spacing_and_reads_its_variable() {
    use pncad::document::TargetAxis;
    let (mut session, row) = a_row(TargetAxis::X, None, len(0.05), 3);
    assert_eq!(member_offsets(&mut session, row).len(), 3, "three members");
    stepped(&mut session, row, [1.0, 0.0, 0.0], 0.05);

    let pitch_name = pncad::document::VarName::new("pitch").expect("a name");
    let (mut session, row) = a_row(
        TargetAxis::X,
        Some("pitch"),
        Formula::named(pitch_name, Dimension::Length),
        2,
    );
    let pitch = common::var_of(session.committed_doc(), "pitch");
    for at in [0.05, 0.08] {
        let moved = session.perform(SessionOp::SetVariable {
            var: pitch,
            value: viewer::props::SlotValue::Continuous(at),
        });
        assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
        stepped(&mut session, row, [1.0, 0.0, 0.0], at);
    }
}

/// A session over a prism `HEIGHT` tall on the parallelogram with a side
/// along `x` and a side along `(lean, 1)`, with `lean` a free length
/// variable in millimetres-per-millimetre (the side's top corner is
/// `lean` along `x` per unit up `y`), and the prism.
fn slanted_prism(lean: f64) -> (DocSession, RecipeNodeId) {
    use pncad::document::{Formula, LoopProgram, ProgramStep, ProgramTarget};
    const SIDE: f64 = 0.01;
    const HEIGHT: f64 = 0.004;
    let tol = Tol::witness();
    let mut session = session(tol);
    let declared = session.perform(SessionOp::DeclareVar {
        name: pncad::document::VarName::new("lean").expect("a name"),
        value: pncad::document::FreeVar::continuous(Dimension::Length, lean * SIDE),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    let lean_by = || {
        Formula::named(
            pncad::document::VarName::new("lean").expect("a name"),
            Dimension::Length,
        )
    };
    let at = |x: Formula, y: f64| ProgramTarget::Point([x, len(y)]);
    let plus = |a: f64| Formula::add(len(a), lean_by()).expect("two lengths");
    let outline = LoopProgram::Chain(vec![
        ProgramStep::At(common::len2([0.0, 0.0])),
        ProgramStep::LineTo(at(len(SIDE), 0.0)),
        ProgramStep::LineTo(at(plus(SIDE), SIDE)),
        ProgramStep::LineTo(at(plus(0.0), SIDE)),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![outline],
        },
    );
    let prism = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(HEIGHT),
        },
    );
    (session, prism)
}

/// **The prism's straight edge running along `dir`**, as a viewport
/// pick on it selects it ([`edge_where`]).
fn edge_along(
    session: &mut DocSession,
    prism: RecipeNodeId,
    dir: [f64; 3],
) -> viewer::session::EdgeSelection {
    edge_where(session, prism, pncad::select::CurveKind::Line, dir, |_| {
        true
    })
}

/// A row of three of the prism, stepped 50 mm along its edge `edge`.
fn row_along(
    session: &mut DocSession,
    prism: RecipeNodeId,
    edge: viewer::session::EdgeSelection,
) -> RecipeNodeId {
    session_insert(
        session,
        SessionOp::AddPattern {
            input: prism.into(),
            count: 3,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Edge(edge),
                spacing: len(0.05),
            },
        },
    )
}

/// **A row steps its spacing along its direction, whatever the
/// direction**: picked as a prism's edge along `(1, 1, 0)`, member `k`
/// stands `k·spacing` along the unit direction, where a slide per
/// component stepped `spacing·√2`.
#[test]
fn a_row_steps_its_spacing_along_an_oblique_direction() {
    let (mut session, prism) = slanted_prism(1.0);
    let edge = edge_along(&mut session, prism, [1.0, 1.0, 0.0]);
    let row = row_along(&mut session, prism, edge);
    assert_eq!(member_offsets(&mut session, row).len(), 3, "three members");
    stepped(&mut session, row, [1.0, 1.0, 0.0], 0.05);
}

/// **A diagonal row follows the edge it was picked on when the edge is
/// edited**: the prism's slanted side re-leaned from `(1, 1)` to `(2, 1)`,
/// the row steps along the new side, its spacing unchanged.
#[test]
fn a_diagonal_row_follows_its_edited_edge() {
    let (mut session, prism) = slanted_prism(1.0);
    let edge = edge_along(&mut session, prism, [1.0, 1.0, 0.0]);
    let row = row_along(&mut session, prism, edge);
    stepped(&mut session, row, [1.0, 1.0, 0.0], 0.05);
    let lean = common::var_of(session.committed_doc(), "lean");
    let moved = session.perform(SessionOp::SetVariable {
        var: lean,
        value: viewer::props::SlotValue::Continuous(0.02),
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    stepped(&mut session, row, [2.0, 1.0, 0.0], 0.05);
}

/// **The edge of `node` the pick would select, as `keep` keeps one**:
/// the first of its edges whose carrier is of kind `kind`, runs along
/// `dir` and passes `keep`, read off the landed evaluation.
fn edge_where(
    session: &mut DocSession,
    node: RecipeNodeId,
    kind: pncad::select::CurveKind,
    dir: [f64; 3],
    keep: impl Fn(&pncad::select::Pose<f64>) -> bool,
) -> viewer::session::EdgeSelection {
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let node = common::drawn_node(session.committed_doc(), node);
    let norm = dir.iter().map(|c| c * c).sum::<f64>().sqrt();
    let unit = dir.map(|c| c / norm);
    let name = pncad::select::all_edges(eval, node)
        .into_iter()
        .find(|name| {
            pncad::select::edge_carrier_kind(eval, node, name).is_ok_and(|k| k == kind)
                && pncad::select::edge_frame(eval, node, name).is_ok_and(|pose| {
                    let a = [pose.axis.x, pose.axis.y, pose.axis.z];
                    let n = a.iter().map(|c| c * c).sum::<f64>().sqrt();
                    let along: f64 = a.iter().zip(unit).map(|(c, u)| c / n * u).sum();
                    (along.abs() - 1.0).abs() < 1e-12 && keep(&pose)
                })
        })
        .unwrap_or_else(|| panic!("{node:?} has a {kind:?} edge along {dir:?}"));
    viewer::session::EdgeSelection {
        name,
        node,
        body: 0,
    }
}

/// Each member of the family `pattern`'s motion from member 0, as its
/// turn's columns and its origin, in the family's space.
fn member_motions(
    session: &mut DocSession,
    pattern: RecipeNodeId,
) -> Vec<pncad::geom_core::linalg::Affine3<f64>> {
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let ValuePayload::Instances(instances) =
        &eval.value(pattern).expect("the family evaluated").payload
    else {
        panic!("a family evaluates to its members");
    };
    let instances = common::every_body(instances);
    instances
        .iter()
        .map(|member| {
            instances[0]
                .relative(member)
                .expect("members of one space relate")
                .map_or_else(pncad::geom_core::linalg::Affine3::identity, |m| {
                    m.to_affine()
                })
        })
        .collect()
}

/// **A ring about a tilted axis stands at one height along it**: the
/// prism ringed about its own slanted edge, along `(1, 1, 0)` and
/// missing the origin, turns each member about that line, so every
/// point of the prism keeps its height along the axis and its distance
/// from it — a ring, where a slide held on a target axis made a helix.
#[test]
fn a_ring_about_a_tilted_axis_stands_at_one_height() {
    let (mut session, prism) = slanted_prism(1.0);
    let edge = edge_where(
        &mut session,
        prism,
        pncad::select::CurveKind::Line,
        [1.0, 1.0, 0.0],
        |pose| {
            let o = pose.origin;
            (o.x - o.y).abs() > 1e-6
        },
    );
    let line = {
        let eval = session.evaluation().expect("the inline seam landed");
        pncad::select::edge_frame(eval, edge.node, &edge.name).expect("a line")
    };
    let ring = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: prism.into(),
            count: 4,
            rule: PatternRuleSpec::Circular {
                axis: viewer::PatternDirection::Edge(edge),
                step: ang(core::f64::consts::FRAC_PI_2),
            },
        },
    );
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let u = pncad::geom_core::linalg::Vec3::new(h, h, 0.0);
    let foot = line.origin;
    // A probe point of the prism off the axis: its corner at the origin.
    let p = pncad::geom_core::linalg::Point3::origin();
    let height = |q: pncad::geom_core::linalg::Point3<f64>| (q - foot).dot(u);
    let reach = |q: pncad::geom_core::linalg::Point3<f64>| {
        let d = q - foot;
        (d - u * d.dot(u)).norm()
    };
    let motions = member_motions(&mut session, ring);
    assert_eq!(motions.len(), 4, "four members");
    for (k, m) in motions.iter().enumerate() {
        let q = m.transform_point(p);
        assert!(
            (height(q) - height(p)).abs() < 1e-12,
            "member {k} stands {} along the axis, not {}",
            height(q),
            height(p)
        );
        assert!(
            (reach(q) - reach(p)).abs() < 1e-12,
            "member {k} stands {} from the axis, not {}",
            reach(q),
            reach(p)
        );
    }
    assert!(
        (motions[2].transform_point(p) - p).norm() > 1e-3,
        "the half-turned member moved: the probe is off the axis"
    );
}

/// **A bolt circle turns about another body's axis, with no mate**: a
/// bolt block beside a flange cylinder in its space, ringed about the
/// flange's round edge's axis — a spin about an axis read off the
/// flange — stands its members on a circle about the flange's centre.
#[test]
fn a_bolt_circle_turns_about_another_bodys_axis() {
    const CENTRE: [f64; 2] = [0.03, 0.01];
    let tol = Tol::witness();
    let mut session = session(tol);
    let flange_profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![common::circle_at(CENTRE, 0.02)],
        },
    );
    let flange = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: flange_profile,
            distance: len(0.005),
        },
    );
    let raw_bolt = common::xy_box_in(&mut session, [0.002, 0.002, 0.008]);
    let space = session
        .committed_doc()
        .node_space(flange)
        .expect("the flange's space");
    // The bolt placed into the flange's space beside it, as written.
    let (doc, bolt) = common::inserted(
        session.committed_doc(),
        Node::place(
            pncad::document::BodyRead::plain(Operand::from(raw_bolt)),
            Operand::Var(space),
            pncad::document::Placed::unmated(common::slides([0.045, 0.01, 0.0])).bundle(),
        ),
        tol,
    );
    let mut session = crate::common::inline_without_door(doc, tol);
    let rim = edge_where(
        &mut session,
        flange,
        pncad::select::CurveKind::Circle,
        [0.0, 0.0, 1.0],
        |_| true,
    );
    let bolts = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: bolt.into(),
            count: 4,
            rule: PatternRuleSpec::Circular {
                axis: viewer::PatternDirection::Edge(rim),
                step: ang(core::f64::consts::FRAC_PI_2),
            },
        },
    );
    assert!(matches!(
        pncad::pattern::recognise(session.committed_doc(), bolts),
        Some(pncad::pattern::Recognised::Circular { .. })
    ));
    // Member 0's centre is the bolt's: 15 mm along x from the flange's.
    // A member's motion is from member 0 in the bolt's own coordinates,
    // which its placement slides to where it stands.
    let placed = pncad::geom_core::linalg::Vec3::new(0.045, 0.01, 0.0);
    let p = pncad::geom_core::linalg::Point3::new(0.0, 0.0, 0.004);
    let c = pncad::geom_core::linalg::Point3::new(CENTRE[0], CENTRE[1], 0.004);
    let quarter = |k: usize| {
        let (s, co) = (core::f64::consts::FRAC_PI_2 * k as f64).sin_cos();
        pncad::geom_core::linalg::Point3::new(c.x + 0.015 * co, c.y + 0.015 * s, c.z)
    };
    for (k, m) in member_motions(&mut session, bolts).iter().enumerate() {
        let q = m.transform_point(p) + placed;
        assert!(
            (q - quarter(k)).norm() < 1e-12,
            "bolt {k} stands at {q:?}, not {:?}",
            quarter(k)
        );
    }
}

/// **A ring of a moved body's copy reads its axis where the copy
/// stands**: the second block, moved by the Transform tool and placed
/// into the world, ringed about its own vertical edge picked on the
/// world copy — the ring reads the copy it was picked on (Ev, PR 6353),
/// and the copy's axis in the copied block's own coordinates, those a
/// placement's values read its source in, so no mate reads a body the
/// placement does not copy.
#[test]
fn a_ring_of_a_moved_body_turns_about_its_picked_edge() {
    let (mut session, _, proto) = two_boxes(Tol::witness());
    let edge = edge_where(
        &mut session,
        proto,
        pncad::select::CurveKind::Line,
        [0.0, 0.0, 1.0],
        |_| true,
    );
    let edge_name = edge.name.clone();
    let copy = edge.node;
    let outcome = session.perform(SessionOp::AddPattern {
        input: copy.into(),
        count: 4,
        rule: PatternRuleSpec::Circular {
            axis: viewer::PatternDirection::Edge(edge),
            step: ang(core::f64::consts::FRAC_PI_2),
        },
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let ring = outcome.made.expect("the ring is made");
    // The picked line in the coordinates its members' motions are in:
    // the copied block's own, read off the block the copies copy.
    let mut raw = copy;
    while let Some(source) = viewer::world::copy_source(session.committed_doc(), raw, 0) {
        raw = source.operation;
    }
    let line = {
        let eval = session.evaluation().expect("the inline seam landed");
        pncad::select::edge_frame(eval, raw, &edge_name).expect("the picked edge's line")
    };
    let on_line = [line.origin, line.origin + line.axis];
    let motions = member_motions(&mut session, ring);
    assert_eq!(motions.len(), 4, "four members");
    for (k, m) in motions.iter().enumerate() {
        for p in on_line {
            assert!(
                (m.transform_point(p) - p).norm() < 1e-12,
                "member {k} keeps the picked line where it stands: {p:?}"
            );
        }
    }
    let centre = pncad::geom_core::linalg::Point3::origin();
    assert!(
        (motions[2].transform_point(centre) - centre).norm() > 1e-3,
        "the half-turned member moved: the block's centre is off the line"
    );
}

/// **The placement editor keeps a ring's slide along its axis on a
/// re-edit**: a ring of a cylinder about its own axis, written by the
/// pattern writer with one `Axis` mate and a slide along the mated
/// direction, opened in the editor, proposes the loaded slide along the
/// mated direction as it is, not a slide along a target axis, and drops
/// nothing.
#[test]
fn the_placement_editor_keeps_a_rings_mated_slide() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![common::circle_at([0.0, 0.0], 0.01)],
        },
    );
    let cylinder = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.005),
        },
    );
    session.pump();
    let doc = session.committed_doc().clone();
    let side = {
        let eval = session.evaluation().expect("the inline seam landed");
        pncad::select::all_faces(eval, cylinder)
            .into_iter()
            .find(|name| {
                pncad::select::face_carrier_kind(eval, cylinder, name)
                    .is_ok_and(|k| k == pncad::geom::SurfaceKind::Cylinder)
            })
            .expect("the cylinder has a round side")
    };
    let axis = pncad::document::Operand::Pose(Box::new(pncad::document::pose::PoseDef::Axis {
        of: pncad::document::Operand::select(pncad::document::Operand::Node(cylinder), vec![side]),
    }));
    let k = viewer::combine::fresh_index(&doc);
    let doc = pncad::document::apply(
        &doc,
        &pncad::pattern::index_decl(k.clone(), Formula::count(3), None),
        tol,
    )
    .expect("an index declares")
    .doc;
    let space = doc.node_space(cylinder).expect("the cylinder's space");
    let ring = pncad::pattern::circular_pattern(
        &doc,
        &k,
        pncad::document::BodyRead::plain(pncad::document::Operand::Node(cylinder)),
        pncad::document::Operand::Var(space),
        pncad::pattern::RingAxis::Mated {
            on: axis.clone(),
            to: axis,
            slide: len(0.002),
        },
        pncad::pattern::RingStep::Full(Formula::count(3)),
    )
    .expect("a ring");
    let (doc, ring) = common::inserted(&doc, ring, tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let (doc, ev) = session.landed_pair().expect("landed");
    let draft = viewer::matetool::MateTool::edit(doc, ring)
        .expect("the ring is a placement")
        .proposal(doc, ev, tol)
        .expect("the loaded bundle proposes");
    let coords: Vec<pncad::document::Coord<pncad::document::Operand>> =
        draft.values.iter().map(|row| row.coord.clone()).collect();
    assert!(
        coords.contains(&pncad::document::Coord::SLIDE)
            && !coords
                .iter()
                .any(|c| matches!(c, pncad::document::Coord::Slide(Some(_)))),
        "the slide along the mated direction is kept as it is: {coords:?}"
    );
    assert!(
        draft.dropped.is_empty(),
        "nothing drops: {:?}",
        draft.dropped
    );
}

/// **A skewed grid is a parallelogram lattice**: two indices, each
/// stepping along one of the prism's two sides, `x` and `(1, 1, 0)`,
/// written by the pattern writer the gesture writes through, each
/// member the sum of its two steps.
#[test]
fn a_skewed_grid_steps_along_both_its_picked_sides() {
    let (mut session, prism) = slanted_prism(1.0);
    let edge = edge_along(&mut session, prism, [1.0, 1.0, 0.0]);
    let doc = session.committed_doc();
    let (body, name) = (
        pncad::document::Operand::output(edge.node, 0),
        edge.name.clone(),
    );
    let read = pncad::document::pose::RoleRead::plain(pncad::document::Operand::Pose(Box::new(
        pncad::document::pose::PoseDef::Axis {
            of: pncad::document::Operand::select(body, vec![name]),
        },
    )));
    let (i, j) = (
        pncad::document::VarName::new("i").expect("a name"),
        pncad::document::VarName::new("j").expect("a name"),
    );
    let mut doc = doc.clone();
    for (name, count, within) in [(&i, 2, None), (&j, 3, Some(&i))] {
        doc = pncad::document::apply(
            &doc,
            &pncad::pattern::index_decl(name.clone(), Formula::count(count), within),
            Tol::witness(),
        )
        .expect("an index declares")
        .doc;
    }
    let space = doc.node_space(prism).expect("the prism's space");
    let grid = pncad::pattern::grid(
        (
            &i,
            (
                pncad::pattern::RowDirection::Axis(pncad::document::TargetAxis::X),
                len(0.03),
            ),
        ),
        (&j, (pncad::pattern::RowDirection::Read(read), len(0.05))),
        pncad::document::BodyRead::plain(pncad::document::Operand::from(prism)),
        pncad::document::Operand::Var(space),
        Some(pncad::document::slides_across(&[
            pncad::geom_core::linalg::Vec3::new(1.0, 0.0, 0.0),
            pncad::geom_core::linalg::Vec3::new(1.0, 1.0, 0.0),
        ])),
    )
    .expect("length pitches");
    let (doc, grid) = common::inserted(&doc, grid, Tol::witness());
    let mut session = DocSession::inline(doc, Tol::witness());
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let ValuePayload::Instances(members) = &eval.value(grid).expect("the grid evaluated").payload
    else {
        panic!("a grid evaluates to its members");
    };
    let members = common::every_body(members);
    assert_eq!(members.len(), 6, "two by three members");
    let h = core::f64::consts::FRAC_1_SQRT_2;
    for (n, member) in members.iter().enumerate() {
        let (a, b) = ((n / 3) as f64, (n % 3) as f64);
        let want = [0.03 * a + 0.05 * b * h, 0.05 * b * h, 0.0];
        let o = members[0]
            .relative(member)
            .expect("members of one space relate")
            .map_or([0.0; 3], |m| {
                let o = m.origin();
                [o.x, o.y, o.z]
            });
        assert!(
            o.iter().zip(want).all(|(g, w)| (g - w).abs() < 1e-12),
            "member {n} stands at {o:?}, not {want:?}"
        );
    }
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
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
            .linear_op(
                output,
                2,
                viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                len(0.05),
            )
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
            plane: common::cut_across(&session, body, 0.0),
        },
        SessionOp::place_where_it_stands(session.committed_doc(), body.into())
            .expect("a body in a space"),
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(0.05),
            },
        },
        SessionOp::AddFusedPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
    // them — nor does an axis that names no direction.
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
            plane: common::cut_across(&session, plane, 0.0),
        },
        SessionOp::Place {
            body: plane.into(),
            target: Operand::Var(pncad::document::VarId::WORLD),
        },
        SessionOp::AddPattern {
            input: plane.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(0.05),
            },
        },
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Circular {
                axis: viewer::PatternDirection::Face(end_cap(body)),
                step: ang(1.0),
            },
        },
        SessionOp::AddFusedPattern {
            input: plane.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(0.05),
            },
        },
        SessionOp::AddFusedPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Circular {
                axis: viewer::PatternDirection::Face(end_cap(body)),
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
                    direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
        Ok(SessionOp::AddBoolean { spec: BooleanSpec::Union(members), .. }) if members == [BodyRead::from(a)]
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
        }) if first == BodyRead::from(a) && second == BodyRead::from(b)
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

    // The split tool's one seat is its target, and its plane a pick.
    let mut split = SplitTool::new();
    assert!(matches!(
        split.op(),
        Err(SeatError::Empty {
            seat: Seat::SplitTarget
        })
    ));
    split.pick(session.committed_doc(), a);
    assert!(matches!(
        split.op(),
        Err(SeatError::NoFace {
            role: viewer::seats::FaceRole::SplitPlane
        })
    ));
    split.pick_plane(session.committed_doc(), end_cap(a));
    assert!(matches!(
        split.op(),
        Ok(SessionOp::AddSplit {
            plane: viewer::session::SplitPlane::Face(_),
            ..
        })
    ));

    // The pattern tool's one seat is its body, whichever rule; its
    // direction, or its ring's axis, is a target axis or a pick.
    let mut pattern = PatternTool::new();
    pattern.pick(session.committed_doc(), a);
    assert!(matches!(
        pattern.linear_op(
            PatternOutputChoice::Instances,
            3,
            viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
            len(0.05)
        ),
        Ok(SessionOp::AddPattern { count: 3, .. })
    ));
    assert!(matches!(
        pattern.circular_op(
            PatternOutputChoice::Instances,
            3,
            viewer::PatternDirection::Axis(pncad::document::TargetAxis::Z),
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
            viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
            len(0.05)
        ),
        Ok(SessionOp::AddFusedPattern { count: 3, .. })
    ));
    pattern.clear();
    assert_eq!(pattern.input(), None);
    for output in [PatternOutputChoice::Instances, PatternOutputChoice::Fused] {
        assert!(matches!(
            pattern.circular_op(
                output,
                3,
                viewer::PatternDirection::Axis(pncad::document::TargetAxis::Z),
                ang(1.0)
            ),
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
        ToolKind::Boolean => tools.boolean().is_some(),
        ToolKind::Split => tools.split().is_some(),
        ToolKind::Transform => tools.transform().is_some(),
        ToolKind::Pattern => tools.pattern().is_some(),
        ToolKind::Blend => tools.blend().is_some(),
        ToolKind::Part => tools.part().is_some(),
        ToolKind::Mirror => tools.mirror().is_some(),
        ToolKind::Clearance => tools.clearance().is_some(),
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
    let plane = common::cut_across(&session, body, 0.005);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    let pattern = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
                plane: common::cut_across(&session, filled(Seat::SplitTarget), 0.005),
            },
            Seat::PartInstance => SessionOp::AddPart {
                of: filled(Seat::PartInstance),
                select: PartSelectSpec::Instance(0),
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
            // The other seat a body of the right kind, so the refusal
            // is about the seat under test.
            Seat::ClearedFirst | Seat::ClearedSecond => SessionOp::AddClearance {
                a: viewer::session::ClearedSeat::Body(filled(Seat::ClearedFirst).into()),
                b: viewer::session::ClearedSeat::Body(filled(Seat::ClearedSecond).into()),
                bounds: pncad::document::ClearanceBounds::AtLeast(Formula::zero(Dimension::Length)),
            },
            Seat::PatternBody => SessionOp::AddPattern {
                input: filled(Seat::PatternBody).into(),
                count: 3,
                rule: PatternRuleSpec::Circular {
                    axis: viewer::PatternDirection::Axis(pncad::document::TargetAxis::Z),
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
    split.pick_plane(doc, end_cap(a));
    split.pick(doc, profile);
    assert_eq!(
        split.target(),
        Some(profile),
        "an unroutable pick lands where the plain rule puts it",
    );
    let op = split.op().expect("the seat and the plane are filled");
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
/// and the pattern tool's own door ("a second pick replaces the
/// first") stops being true.
#[test]
fn a_second_pick_replaces_a_one_seat_tools_body() {
    let tol = Tol::witness();
    let (session, a, b) = two_boxes(tol);
    let doc = session.committed_doc();

    let mut pattern = PatternTool::new();
    pattern.pick(doc, a);
    assert_eq!(pattern.input(), Some(a));
    pattern.pick(doc, b);
    assert_eq!(pattern.input(), Some(b), "the second pick replaces");

    // The survival step sees one seat, so a vanished pick is reported
    // once rather than twice.
    let mut session = session;
    let out = session.perform(SessionOp::DeleteNode { node: b });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let events = pattern.reconcile(session.committed_doc());
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
    let mut pattern = PatternTool::new();
    pattern.pick(&doc, RecipeNodeId::new(0, tagged(7)));
    assert_eq!(
        seat_line(pattern.seats(), Speaker::of(&doc)),
        "patterned body: node 000000000007"
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
                plane: viewer::session::SplitPlane::Face(end_cap(RecipeNodeId::new(0, tagged(1)))),
            },
        ),
        (
            ToolKind::Pattern,
            SessionOp::AddPattern {
                input: (RecipeNodeId::new(0, tagged(1))).into(),
                count: 2,
                rule: PatternRuleSpec::Linear {
                    direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                    spacing: len(0.05),
                },
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
        SessionOp::AddRevolve {
            profile: RecipeNodeId::new(0, tagged(1)),
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(1.0),
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
                &[(
                    pncad::pattern::RowDirection::Axis(pncad::document::TargetAxis::X),
                    common::len(0.05),
                )],
                None,
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
                    id: None,
                    profile: profile_b.into(),
                    station: vec![pncad::document::StationMove::Slide {
                        along: Axis3::Z,
                        by: common::len(0.01),
                    }],
                }],
                v_degree: Formula::count(1),
                first_id: None,
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
            // The mirror's and the split's plane is a face, and their
            // body is picked by one too; the clearance tool seats
            // faces, and bodies from the tree.
            ToolKind::Mate | ToolKind::Mirror | ToolKind::Split | ToolKind::Clearance => {
                PickKinds::FacesOnly
            }
            ToolKind::Blend => PickKinds::EdgesOnly,
            ToolKind::Boolean | ToolKind::Transform | ToolKind::Pattern | ToolKind::Part => {
                PickKinds::FacesAndEdges
            }
        };
        assert_eq!(tools.pick_kinds(), want, "{kind:?}");
    }
    tools.close();
    assert_eq!(tools.pick_kinds(), PickKinds::Any);
}

// ---------------------------------------------------------------
// AUTH-4: projecting one body out of several.
// ---------------------------------------------------------------

/// The volume of what the product slot reads, gathered off the landed
/// document as it is.
fn product_volume(session: &mut DocSession, tol: Tol) -> f64 {
    session.pump();
    let (landed, ev) = session.landed_pair().expect("a run landed");
    let body = pncad::document::product(landed, ev, tol).expect("the product gathers");
    pncad::topo::test_support::pinned_volume(
        "mass properties",
        &pncad::topo::finish(pncad::topo::Body::clone(&body), tol)
            .expect("the measured body passes tier 3"),
        tol,
    )
    .volume
}

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
        &pncad::topo::finish(pncad::topo::Body::clone(&body), tol)
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
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(A[0] * 2.0),
            },
        },
    );
    (session, body, pattern)
}

/// **A split's half is a body at a seat, read by its port**: the
/// Transform tool's placement of the split's `below` output, picked on
/// that half, reads that output's variable, and its body is the lower
/// half.
#[test]
fn a_split_half_is_read_by_its_port_at_a_body_seat() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let plane = common::cut_across(&session, body, A[2] / 4.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    let (above, below) = split_volumes(&mut session, split, tol);
    session.pump();
    let (doc, _) = session.landed_pair().expect("landed");
    let place =
        viewer::matetool::MateTool::moving(doc, split, Some(u32::from(SplitHalf::Below.port())))
            .expect("the half is a body")
            .take_place()
            .expect("a half picked as itself is Place");
    let moved = common::placed_by(&mut session, place);
    let doc = session.committed_doc();
    assert!(
        matches!(
            doc.node(moved),
            Some(Node::Place { source, .. })
                if Some(source.read) == doc.output(split, SplitHalf::Below.port())
        ),
        "the Transform tool's placement reads the split's below output"
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
    let plane = common::cut_across(&session, body, A[2] / 4.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
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
/// space and the body is not, so no member is drawn over it; picked on
/// a body's world copy, it reads the copy, so the world draws both
/// members there, apart, and the copy's source is untouched. A member
/// read is the node alone, placed nowhere.
#[test]
fn a_pattern_takes_its_body_over_and_a_member_read_places_nothing() {
    let tol = Tol::witness();
    let (mut session, body, pattern) = box_and_pattern(tol);
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

    // Placed into the world, a pattern picked on the copy reads the
    // copy: a family of it in the world, the copy's source untouched.
    let mut session = session_with_box_in_world(tol);
    let (body, copy) = session.1;
    let pattern = session_insert(
        &mut session.0,
        SessionOp::AddPattern {
            input: copy.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
        doc.output(body, 0),
        "the world copy still copies the body"
    );
    assert_eq!(
        doc.node_space(pattern),
        Some(pncad::document::VarId::WORLD),
        "the family of the copy stands in the world"
    );
    assert_eq!(
        viewer::world::drawn_nodes(doc),
        [pattern],
        "the family is drawn in the world, the copy it repeats is not"
    );
    // The product read the copy, and reads the family's `⊔` now: both
    // members, once each.
    let both = product_volume(session, tol);
    let one = A[0] * A[1] * A[2];
    assert!(near(both, one * 2.0), "both members in the product: {both}");
    assert_eq!(
        at_rest_findings(session),
        0,
        "and its members do not overlap"
    );
}

/// Box `A` added to the product, so placed into the world: the session,
/// the box and its world copy.
fn session_with_box_in_world(tol: Tol) -> (DocSession, (RecipeNodeId, RecipeNodeId)) {
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let (session, copy) = common::world_copy_in(&session, body, tol);
    (session, (body, copy))
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
    let plane = common::cut_across(&session, body, A[2] / 4.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    let pattern = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
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
/// placed half's copy seats that copy, and a split picked in the tree
/// waits for its half — the commit refuses typed until one is chosen,
/// and then reads that port.
#[test]
fn a_body_seat_reads_a_split_half_by_its_port() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, B);
    let plane = common::cut_across(&session, body, B[2] / 4.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    let below = pncad::document::Operand::output(split, SplitHalf::Below.port());
    let (doc, placed) = common::placed_read(session.committed_doc(), below.clone(), tol);

    // A pick on the placed half's copy seats the copy (Ev, PR 6353),
    // and the copy's original is the half it places.
    let mut tool = PatternTool::new();
    tool.pick_at(&doc, placed, Some(0));
    assert_eq!(tool.input(), Some(placed));
    assert_eq!(
        tool.seats().read(0),
        Some(pncad::document::Operand::Node(placed))
    );
    let source = viewer::world::copy_source(&doc, placed, 0).expect("a copy");
    assert_eq!(
        (source.operation, source.port),
        (split, SplitHalf::Below.port()),
        "of the half"
    );

    // A tree pick of the split names neither half: the seat refuses
    // naming itself, and reads the half once it is chosen.
    let mut tool = PatternTool::new();
    tool.pick(&doc, split);
    assert!(tool.seats().awaits_half(0));
    assert!(matches!(
        tool.read(),
        Err(SeatError::HalfUnchosen {
            seat: Seat::PatternBody
        })
    ));
    tool.choose_half(0, SplitHalf::Below);
    assert!(matches!(tool.read(), Ok(read) if read == below));
    assert!(
        seat_line(tool.seats(), Speaker::of(&doc)).ends_with("below half"),
        "{}",
        seat_line(tool.seats(), Speaker::of(&doc))
    );
}

/// **The projection tool closes on its own committed edit**, and on
/// nothing else — the rule that makes a tool modal.
#[test]
fn the_part_tool_closes_on_its_own_edit() {
    let node = RecipeNodeId::new(0, tagged(1));
    let part = SessionOp::AddPart {
        of: node,
        select: PartSelectSpec::Instance(0),
    };
    let mut tools = Tools::new();
    tools.open(ToolKind::Part);
    assert!(tools.commits_open_tool(&part));
    tools.open(ToolKind::Split);
    assert!(!tools.commits_open_tool(&part));
}

// ---------------------------------------------------------------
// What a VIEWPORT pick seats.
// ---------------------------------------------------------------

/// A face picked through the session's real ray path, looking straight
/// down at `(x, y)` — the viewport's own door, not a hand-built
/// selection.
fn picked_from_above(session: &DocSession, x: f64, y: f64) -> Selection {
    Selection::Face(common::asm::pick_face(session, &common::asm::down_at(x, y)))
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

/// Box `A` placed, and the copy moved two widths along +x and `lift`
/// along +y by the Transform tool, its editor — the workflow the gesture
/// exists for. Answers the session, the moved copy's node and where its
/// centre now is in plan.
fn a_moved_copy(tol: Tol, lift: f64) -> (DocSession, RecipeNodeId, [f64; 2]) {
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    session.pump();
    let outcome = session.perform(
        SessionOp::place_where_it_stands(session.committed_doc(), body.into())
            .expect("a body in a space"),
    );
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let copy = outcome.made.expect("the placement");
    let step = 2.0 * A[0];
    let op = common::transformed(
        &mut session,
        copy,
        &[
            (
                pncad::document::Coord::slide(pncad::document::TargetAxis::X),
                len(step),
            ),
            (
                pncad::document::Coord::slide(pncad::document::TargetAxis::Y),
                len(lift),
            ),
        ],
    );
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    (session, copy, [step, lift])
}

/// **A viewport click seats the body that was DRAWN, not the feature
/// that made the face** — for every tool whose seat takes a body.
///
/// The picked face on a moved copy was minted by the ORIGINAL extrude
/// (a placement mints no name), so `Selection::node` — the feature
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
            // The placement editor opened on the copy's placement.
            ToolKind::Transform => tools.mate().and_then(|t| t.door()).map(|d| d.placement),
            ToolKind::Pattern => tools.pattern().and_then(|t| t.input()),
            ToolKind::Mirror => tools.mirror().and_then(|t| t.input()),
            // Seats a split or a pattern, never a body: its own row
            // below picks one.
            ToolKind::Part => continue,
            // Faces and edges, taken whole rather than as a node; and
            // a profile and an axis, which a face pick never is.
            ToolKind::Mate | ToolKind::Blend => continue,
            // A face pick seats the face itself, not its body.
            ToolKind::Clearance => continue,
        };
        assert_eq!(
            held,
            Some(moved),
            "the {kind:?} tool seats the body the drawn copy places"
        );
    }
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

/// **A split half clicked in the viewport lands in a body seat by its
/// port**: the split drawn as itself, a pick on one of its halves is a
/// read of that half.
#[test]
fn a_body_seat_takes_a_split_half_clicked_in_the_viewport() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let plane = common::cut_across(&session, body, A[2] / 2.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    // The split reads the box, so its halves are drawn in its place.
    session.pump();
    let picked = picked_from_above(&session, 0.0, 0.0);
    let mut tools = Tools::new();
    tools.open(ToolKind::Split);
    let _ = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[SessionOp::Select(picked)],
    );
    let tool = tools.split().expect("open");
    assert_eq!(tool.target(), Some(split));
    assert_eq!(
        tool.seats().read(0),
        Some(pncad::document::Operand::output(
            split,
            SplitHalf::Above.port()
        )),
        "the half the ray met from above"
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
    let channel = common::moved_to(&mut session, raw, [0.0, 0.0, CHANNEL_DROP]);
    let home = session
        .committed_doc()
        .node_space(block)
        .expect("the block's space");
    common::into_space(&mut session, channel, home);
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
    let boss = common::moved_to(&mut session, raw_boss, [0.0, 0.0, common::BOSS_BLOCK[2]]);
    common::into_space(&mut session, boss, home);
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
    let plane = common::cut_across(&session, body, B[2] / 4.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    let doc = session.committed_doc().clone();
    let port = |h: SplitHalf| BodyRead::plain(Operand::output(split, h.port()));
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
/// Place of a body read by its variable lands, and a Place of a
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
    let placed = session.perform(
        SessionOp::place_where_it_stands(session.committed_doc(), Operand::Var(var))
            .expect("a body in a space"),
    );
    assert!(placed.refusal.is_none(), "{:?}", placed.refusal);
    let refused = session.perform(
        SessionOp::place_where_it_stands(
            session.committed_doc(),
            Operand::Select {
                body: Box::new(body.into()),
                names: Vec::new(),
            },
        )
        .expect("a body in a space"),
    );
    assert!(
        matches!(refused.refusal, Some(Refusal::NotABodyRead { .. })),
        "{:?}",
        refused.refusal
    );
}

/// **A pattern of one half of a split re-points neither half's copy**:
/// two placements draw the two halves of a split, and a pattern of the
/// lower half picked as itself repeats that half alone and writes no
/// placement's source (Ev, PR 6353).
#[test]
fn a_pattern_of_one_half_of_a_split_re_points_neither_copy() {
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
        SessionOp::AddPattern {
            input: below,
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::Y),
                spacing: len(A[1] * 2.0),
            },
        },
    );
    let doc = session.committed_doc();
    let read = |placement| match doc.node(placement) {
        Some(Node::Place { source, .. }) => Some(source.read),
        other => panic!("the placement stands, not {other:?}"),
    };
    assert_eq!(
        [read(upper), read(lower)],
        [above, doc.output(split, SplitHalf::Below.port())],
        "each copy keeps its half: the pattern re-points no placement"
    );
    assert_eq!(
        doc.copied_read(moved),
        doc.output(split, SplitHalf::Below.port()),
        "the family repeats the lower half alone"
    );
}

/// **A re-point that would strand a pick refuses, typed, and records
/// nothing.** A `⊔` of two placed boxes is placed in the world, and a
/// pick of its first piece is placed beside it. The union of the `⊔`
/// with the second box writes no placement; re-pointing the `⊔`'s world
/// copy onto the union, which holds no pieces, is refused: a pick is
/// never re-pointed, so the re-point refuses rather than leave the pick
/// naming a piece its copy no longer holds.
#[test]
fn a_combine_that_would_strand_a_pick_refuses_typed() {
    use pncad::document::VarId;
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
    let union = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![seat.into(), b.into()]),
        },
    );
    let doc = session.committed_doc();
    assert_eq!(
        doc.copied_read(held),
        doc.output(seat, 0),
        "the union writes no placement"
    );
    assert_eq!(
        viewer::world::repoint_offer(doc, union),
        None,
        "the union is built on a copy, so it offers no re-point"
    );
    let seat_body = doc.output(seat, 0).expect("the `⊔`'s body");
    let steps = session.history().len();
    let out = session.perform(SessionOp::RepointPlacements {
        body: Operand::Var(seat_body),
        onto: union,
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

/// A face picked through the session's real ray path, looking straight
/// up at `(x, y)` from below.
fn picked_from_below(session: &DocSession, x: f64, y: f64) -> Selection {
    Selection::Face(common::asm::pick_face(session, &common::asm::up_at(x, y)))
}

/// The boolean tool, opened under `op` and fed `picks` through the
/// tools' own selection door, as the viewport and the tree feed it.
fn boolean_fed(doc: &Doc<ProfileProgram>, op: BooleanOp, picks: Vec<Selection>) -> BooleanTool {
    let mut tools = Tools::new();
    tools.open(ToolKind::Boolean);
    if let Some(tool) = tools.boolean_mut() {
        tool.set_operation(op);
    }
    let picks: Vec<SessionOp> = picks.into_iter().map(SessionOp::Select).collect();
    let _ = tools.feed(doc, Err(viewer::pickcache::NotIndexed::Absent), &picks);
    tools.boolean().expect("the boolean tool is open")
}

/// Member `at` of the family `pattern`, read by its index.
fn member(pattern: RecipeNodeId, at: i64) -> BodyRead<Formula> {
    BodyRead::indexed(Operand::Node(pattern), vec![Formula::count(at)])
}

/// **The union of a split's two halves, each clicked in the viewport**:
/// the boolean tool reads one half by each pick's port, its line names
/// both halves, and the union it commits is the whole box again.
#[test]
fn the_boolean_tool_unions_a_splits_two_halves_picked_in_the_viewport() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = common::xy_box_in(&mut session, A);
    let plane = common::cut_across(&session, body, A[2] / 4.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            plane,
        },
    );
    session.pump();
    let picks = vec![
        picked_from_above(&session, 0.0, 0.0),
        picked_from_below(&session, 0.0, 0.0),
    ];
    let doc = session.committed_doc().clone();
    let tool = boolean_fed(&doc, BooleanOp::Union, picks);
    let port = |h: SplitHalf| BodyRead::plain(Operand::output(split, h.port()));
    assert_eq!(
        tool.spec().expect("both halves held"),
        BooleanSpec::Union(vec![port(SplitHalf::Above), port(SplitHalf::Below)]),
        "the ray from above met the upper half, the ray from below the lower"
    );
    let line = tool.line(Speaker::of(&doc));
    assert!(
        line.contains("above half") && line.contains("below half"),
        "the panel names both halves: {line}"
    );
    let union = session_insert(&mut session, tool.op().expect("committable"));
    let whole = A[0] * A[1] * A[2];
    assert_volume(&mut session, union, whole, tol);
}

/// Box `B` moved to stand inside member 1 of [`box_and_pattern`]'s row
/// (member 0 at the origin, member 1 `2·A[0]` along +x): the session,
/// the pattern and the moved box.
fn pattern_and_box_inside_member_one(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let (mut session, _, pattern) = box_and_pattern(tol);
    let raw = common::xy_box_in(&mut session, B);
    let inner = common::moved_to(&mut session, raw, [A[0] * 2.0, 0.0, 0.002]);
    let home = session
        .committed_doc()
        .node_space(pattern)
        .expect("the pattern's space");
    common::into_space(&mut session, inner, home);
    session.pump();
    (session, pattern, inner)
}

/// **A pattern's member clicked in the viewport is read by its index**
/// at every boolean seat: a union's member and a subtraction's kept
/// body read `pattern[k]` for the member the ray met, the panel line
/// spells that index, and the committed result's volume is that
/// member's arithmetic — `B` stands inside member 1 alone, so the union
/// of member 0 and `B` is the two volumes apart, and member 1 with `B`
/// cut away is the box less `B`.
#[test]
fn a_boolean_reads_the_pattern_member_picked_in_the_viewport() {
    let tol = Tol::witness();
    let (one, small) = (A[0] * A[1] * A[2], B[0] * B[1] * B[2]);

    let (mut session, pattern, inner) = pattern_and_box_inside_member_one(tol);
    let doc = session.committed_doc().clone();
    let picks = vec![
        picked_from_above(&session, 0.0, 0.0),
        Selection::Node(inner),
    ];
    let tool = boolean_fed(&doc, BooleanOp::Union, picks);
    assert_eq!(
        tool.spec().expect("both held"),
        BooleanSpec::Union(vec![member(pattern, 0), inner.into()]),
        "the member the ray met, by its index"
    );
    let line = tool.line(Speaker::of(&doc));
    assert!(line.contains("[0]"), "the panel spells the index: {line}");
    let union = session_insert(&mut session, tool.op().expect("committable"));
    assert_volume(&mut session, union, one + small, tol);

    let (mut session, pattern, inner) = pattern_and_box_inside_member_one(tol);
    let doc = session.committed_doc().clone();
    // Clear of `B`'s footprint, so the ray meets member 1's top.
    let picks = vec![
        picked_from_above(&session, A[0] * 2.0 - 0.015, 0.0),
        Selection::Node(inner),
    ];
    let tool = boolean_fed(&doc, BooleanOp::Subtract, picks);
    assert_eq!(
        tool.spec().expect("both seated"),
        BooleanSpec::Subtract {
            from: member(pattern, 1),
            tool: inner.into(),
        },
        "the body kept is the member the ray met"
    );
    let cut = session_insert(&mut session, tool.op().expect("committable"));
    assert_volume(&mut session, cut, one - small, tol);
}

/// **A family picked in the tree is read whole, and a member chosen
/// reads that member alone**: a tree pick names the node, so a boolean
/// seat reads every member (`D10 [repetition]`) and the line says so;
/// choosing a member, by the index values the family enumerates,
/// re-reads the seat as that member, in a union and in a subtraction
/// alike. A seat of another tool reads the family whole, as before.
#[test]
fn a_family_picked_in_the_tree_is_read_whole_until_a_member_is_chosen() {
    let tol = Tol::witness();
    let (session, _, pattern) = box_and_pattern(tol);
    let doc = session.committed_doc().clone();
    let mut tool = boolean_fed(&doc, BooleanOp::Union, vec![Selection::Node(pattern)]);
    assert_eq!(tool.family_picks(), vec![(0, pattern)]);
    assert_eq!(
        tool.spec().expect("held whole"),
        BooleanSpec::Union(vec![pattern.into()]),
        "the family whole"
    );
    let line = tool.line(Speaker::of(&doc));
    assert!(line.contains("every member"), "{line}");
    let members = viewer::world::members_of(&doc, pattern).expect("a family of two");
    assert_eq!(members, [vec![0], vec![1]], "one index, its two values");
    tool.choose_member(0, members[1].clone());
    assert_eq!(
        tool.spec().expect("chosen"),
        BooleanSpec::Union(vec![member(pattern, 1)])
    );
    assert!(tool.line(Speaker::of(&doc)).contains("[1]"));
    tool.choose_member(0, members[0].clone());
    assert_eq!(
        tool.spec().expect("re-chosen"),
        BooleanSpec::Union(vec![member(pattern, 0)]),
        "a member read re-chooses"
    );

    let mut pair = boolean_fed(
        &doc,
        BooleanOp::Subtract,
        vec![Selection::Node(pattern), Selection::Node(pattern)],
    );
    pair.choose_member(0, vec![0]);
    pair.choose_member(1, vec![1]);
    assert_eq!(
        pair.spec().expect("both seated"),
        BooleanSpec::Subtract {
            from: member(pattern, 0),
            tool: member(pattern, 1),
        }
    );

    let mut patterned = PatternTool::new();
    patterned.pick(&doc, pattern);
    assert_eq!(
        patterned.seats().read(0),
        Some(Operand::Node(pattern)),
        "a seat reading the family whole is unchanged"
    );
}

/// Box `A` placed into the world, then patterned twice along +x: the
/// world copy re-pointed to the family. Answers the session, the world
/// copy and the pattern.
fn world_copied_pattern(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let (mut session, (body, copy)) = session_with_box_in_world(tol);
    let pattern = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(A[0] * 2.0),
            },
        },
    );
    (session, copy, pattern)
}

/// **A member picked on a family drawn as itself is read as that
/// member, beside a world copy of its body**: the body has a world copy,
/// and a pattern of the body picked as itself writes no placement (Ev,
/// PR 6353), so the family is drawn in the body's own space as itself
/// and nothing reads it along a seen path. A pick on its member 1 reads
/// `pattern[1]`, and the union of it leaves the world copy placing the
/// body.
#[test]
fn a_member_picked_on_a_family_drawn_as_itself_reads_that_member() {
    let tol = Tol::witness();
    let (mut session, copy, pattern) = world_copied_pattern(tol);
    let body = viewer::world::original_of(session.committed_doc(), copy);
    let other = common::xy_box_in(&mut session, B);
    session.pump();
    let doc = session.committed_doc().clone();
    let picks = vec![
        picked_from_above(&session, A[0] * 2.0, 0.0),
        Selection::Node(other),
    ];
    let tool = boolean_fed(&doc, BooleanOp::Union, picks);
    assert_eq!(
        tool.spec().expect("both held"),
        BooleanSpec::Union(vec![
            viewer::combine::member_read(Operand::Node(pattern), &[1]),
            other.into(),
        ]),
        "member 1 of the family drawn as itself"
    );
    let union = session_insert(&mut session, tool.op().expect("committable"));
    let doc = session.committed_doc();
    assert_eq!(
        doc.copied_read(copy),
        doc.output(body, 0),
        "the world copy still places the body"
    );
    assert!(viewer::world::is_drawn(doc, union), "the union is drawn");
}

/// **A split of a family reads one half of one member**: picked in the
/// tree, the split waits for its half and reads every member; the half
/// and the member are chosen in either order, and the read is the
/// half's output at the member's index (`split.above[1]`). The door
/// takes it; evaluation refuses it, `WrongOperand` on the union, since
/// a split of a family evaluates to a family of splits whose halves no
/// read reaches —
/// `work/refs/a-split-of-a-family-has-no-halves-a-read-reaches.md`,
/// whose fix turns this row's last assertion into the member's upper
/// three quarters.
#[test]
fn a_split_of_a_family_reads_one_half_of_one_member_chosen_in_either_order() {
    let tol = Tol::witness();
    let (mut session, body, pattern) = box_and_pattern(tol);
    let plane = common::cut_across(&session, body, A[2] / 4.0);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: pattern.into(),
            plane,
        },
    );
    let doc = session.committed_doc().clone();
    let above = |at: i64| {
        viewer::combine::member_read(Operand::output(split, SplitHalf::Above.port()), &[at])
    };
    let mut half_first = boolean_fed(&doc, BooleanOp::Union, vec![Selection::Node(split)]);
    assert_eq!(half_first.awaiting_half(), vec![0], "the split waits");
    assert_eq!(half_first.family_picks(), vec![(0, split)], "a family");
    let line = half_first.line(Speaker::of(&doc));
    assert!(
        line.contains("every member, half not chosen"),
        "both said: {line}"
    );
    half_first.choose_half(0, SplitHalf::Above);
    half_first.choose_member(0, vec![1]);
    let mut member_first = boolean_fed(&doc, BooleanOp::Union, vec![Selection::Node(split)]);
    member_first.choose_member(0, vec![1]);
    assert!(matches!(
        member_first.spec(),
        Err(SeatError::HalfUnchosen { seat: Seat::Member })
    ));
    member_first.choose_half(0, SplitHalf::Above);
    for (order, tool) in [("half first", &half_first), ("member first", &member_first)] {
        assert_eq!(
            tool.spec().expect("chosen"),
            BooleanSpec::Union(vec![above(1)]),
            "{order}"
        );
    }
    assert!(
        half_first
            .line(Speaker::of(&doc))
            .contains("[1], above half"),
        "{}",
        half_first.line(Speaker::of(&doc))
    );
    let union = session_insert(&mut session, half_first.op().expect("committable"));
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    assert!(
        matches!(
            eval.result(union).and_then(NodeResult::error),
            Some(NodeError {
                kind: NodeErrorKind::WrongOperand { input, .. },
                ..
            }) if *input == split
        ),
        "{:?}",
        eval.result(union)
    );
}

/// **A grid's member is read at both its indices, in the order the
/// family enumerates them**: box `C` stands inside member `(i, j) =
/// (1, 2)` of a 2 × 3 grid alone, and the member a viewport pick there
/// reads, with `C` cut away, is the box less `C` — red if the drawn
/// body order, [`viewer::world::members_of`]'s order and the read's
/// indices disagree, since any other member holds none of `C`.
#[test]
fn a_grid_member_is_read_at_both_its_indices() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let block = common::xy_box_in(&mut session, A);
    let (i, j) = (
        pncad::document::VarName::new("i").expect("a name"),
        pncad::document::VarName::new("j").expect("a name"),
    );
    let mut doc = session.committed_doc().clone();
    for (name, count, within) in [(&i, 2, None), (&j, 3, Some(&i))] {
        doc = pncad::document::apply(
            &doc,
            &pncad::pattern::index_decl(name.clone(), Formula::count(count), within),
            tol,
        )
        .expect("an index declares")
        .doc;
    }
    let (step_i, step_j) = (A[0] * 2.0, A[1] * 2.0);
    let space = doc.node_space(block).expect("the block's space");
    let grid = pncad::pattern::grid(
        (
            &i,
            (
                pncad::pattern::RowDirection::Axis(pncad::document::TargetAxis::X),
                len(step_i),
            ),
        ),
        (
            &j,
            (
                pncad::pattern::RowDirection::Axis(pncad::document::TargetAxis::Y),
                len(step_j),
            ),
        ),
        BodyRead::plain(Operand::from(block)),
        Operand::Var(space),
        None,
    )
    .expect("length pitches");
    let (doc, grid) = common::inserted(&doc, grid, tol);
    let mut session = crate::common::inline_without_door(doc, tol);
    let raw = common::xy_box_in(&mut session, B);
    let inner = common::moved_to(&mut session, raw, [step_i, 2.0 * step_j, 0.002]);
    common::into_space(&mut session, inner, space);
    session.pump();
    let doc = session.committed_doc().clone();
    assert_eq!(
        viewer::world::members_of(&doc, grid).map(|members| members.len()),
        Some(6),
        "two by three"
    );
    let picks = vec![
        picked_from_above(&session, step_i - 0.015, 2.0 * step_j),
        Selection::Node(inner),
    ];
    let tool = boolean_fed(&doc, BooleanOp::Subtract, picks);
    assert_eq!(
        tool.spec().expect("both seated"),
        BooleanSpec::Subtract {
            from: viewer::combine::member_read(grid, &[1, 2]),
            tool: inner.into(),
        },
        "the member the ray met, at (i, j)"
    );
    let cut = session_insert(&mut session, tool.op().expect("committable"));
    let (one, small) = (A[0] * A[1] * A[2], B[0] * B[1] * B[2]);
    assert_volume(&mut session, cut, one - small, tol);
}

/// **A member read keeps its index across a change of operation**, as a
/// split's half keeps its port: a union's member re-seats as the body
/// kept, and back.
#[test]
fn the_boolean_tools_member_reads_carry_across_the_operation() {
    let tol = Tol::witness();
    let (session, pattern, inner) = pattern_and_box_inside_member_one(tol);
    let doc = session.committed_doc().clone();
    let picks = vec![
        picked_from_above(&session, A[0] * 2.0 - 0.015, 0.0),
        Selection::Node(inner),
    ];
    let mut tool = boolean_fed(&doc, BooleanOp::Union, picks);
    tool.set_operation(BooleanOp::Subtract);
    assert_eq!(
        tool.spec().expect("seated"),
        BooleanSpec::Subtract {
            from: member(pattern, 1),
            tool: inner.into(),
        },
        "members to seats keep the index"
    );
    tool.set_operation(BooleanOp::Intersect);
    assert_eq!(
        tool.spec().expect("members"),
        BooleanSpec::Intersect(vec![member(pattern, 1), inner.into()]),
        "seats to members keep it"
    );
}

/// **Every seat reads a member exactly where its door takes one**: a
/// viewport pick on member 1 of a family nothing copies reads `[1]` at
/// a seat [`Seat::reads_member`] says reads one, and the family whole at
/// every other; and the boolean door commits a member read at each of
/// its seats. The match is exhaustive, so a seat added to the
/// vocabulary is a seat this row has to place.
#[test]
fn every_seat_reads_a_member_exactly_where_its_door_takes_one() {
    let tol = Tol::witness();
    let (mut session, body, pattern) = box_and_pattern(tol);
    session.pump();
    let doc = session.committed_doc().clone();
    let Selection::Face(face) = picked_from_above(&session, A[0] * 2.0, 0.0) else {
        unreachable!("the helper answers a face")
    };
    for seat in Seat::ALL {
        let mut seats = viewer::seats::Seats::one(seat);
        seats.pick_at(&doc, face.node, Some(face.body));
        let read = seats.body_read(0).expect("seated");
        let want = if seat.reads_member() {
            member(pattern, 1)
        } else {
            pattern.into()
        };
        assert_eq!(read, want, "the {} seat", seat.name());
        let spec = match seat {
            Seat::Member => Some(BooleanSpec::Union(vec![read, body.into()])),
            Seat::SubtractFrom => Some(BooleanSpec::Subtract {
                from: read,
                tool: body.into(),
            }),
            Seat::SubtractTool => Some(BooleanSpec::Subtract {
                from: body.into(),
                tool: read,
            }),
            // Each of these ops holds an `Operand`, which has no member.
            Seat::SplitTarget
            | Seat::PatternBody
            | Seat::PartInstance
            | Seat::MirrorBody
            | Seat::ClearedFirst
            | Seat::ClearedSecond => None,
        };
        if let Some(spec) = spec {
            let mut session = crate::common::inline_without_door(doc.clone(), tol);
            let out = session.perform(SessionOp::AddBoolean { spec });
            assert!(
                out.refusal.is_none(),
                "the {} door takes the member: {:?}",
                seat.name(),
                out.refusal
            );
        }
    }
}

/// **A pattern of a world copy stands in the world, and a pick on its
/// member seats the pattern**
/// (`work/placing/a-pattern-of-a-copy-is-read-as-a-copy-of-its-source.md`,
/// built by PR 6581): the pattern tool picked on the copy seats the copy
/// (Ev, PR 6353), and a viewport pick on the pattern's member 1 seats
/// the pattern, never the copy it replicates. The product took the copy
/// over through a `⊔` of the family, so the family is read whole there
/// (`world::is_seen`).
#[test]
fn a_pick_on_a_pattern_of_a_world_copy_seats_the_pattern() {
    let tol = Tol::witness();
    let (mut session, (_, copy)) = session_with_box_in_world(tol);
    let doc = session.committed_doc().clone();
    let mut tools = Tools::new();
    tools.open(ToolKind::Pattern);
    let _ = tools.feed(
        &doc,
        Err(viewer::pickcache::NotIndexed::Absent),
        &[SessionOp::Select(picked_from_above(&session, 0.0, 0.0))],
    );
    assert_eq!(
        tools.pattern().and_then(|t| t.input()),
        Some(copy),
        "the pattern tool seats the copy"
    );
    let pattern = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: copy.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(A[0] * 2.0),
            },
        },
    );
    let other = common::xy_box_in(&mut session, B);
    session.pump();
    let doc = session.committed_doc().clone();
    assert_eq!(doc.node_space(pattern), Some(pncad::document::VarId::WORLD));
    let tool = boolean_fed(
        &doc,
        BooleanOp::Union,
        vec![
            picked_from_above(&session, A[0] * 2.0, 0.0),
            Selection::Node(other),
        ],
    );
    assert_eq!(
        tool.spec().expect("both held"),
        BooleanSpec::Union(vec![pattern.into(), other.into()]),
        "the member's pick seats the pattern, read whole since the product reads it"
    );
}
