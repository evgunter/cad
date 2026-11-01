//! **One user, one sitting, one chess rook** — the single-part
//! authoring story, driven end to end through the session's own op
//! vocabulary (`SessionOp` → [`DocSession::perform`]) with no renderer
//! anywhere.
//!
//! The part is a rook: a chamfered square plinth, a base disc and a
//! cylindrical shaft, and a square crown block whose crenellations are
//! cut by two crossing slots — every dimension chosen so the evaluated
//! volume has a closed form the assertions derive beside the ops. (The
//! crown is square: the scene predates the join arm that lets a slab
//! cut through a cylinder WALL, whose round-crown cut is `editor-core`'s
//! `reach_slab_cut_sector_side` row. Stacked discs stand in for the revolved
//! silhouette a rook naturally is: this scene predates
//! `ProfileShape`'s `Path` arm, which can now spell one.) On the way
//! the user tries to crown the rook with a circular pattern and learns
//! instances are not a body,
//! fuses the same rule into ONE body and unions THAT onto the crown,
//! deletes that experiment (the cascade, priced by the affordance
//! first), undoes it back, walks the history both ways, edits AFTER an
//! undo (the sibling branch, nothing destroyed), and saves — reopening
//! through the same typed doors to the same document, bit for bit.
//!
//! # Why the volumes can be exact
//!
//! Every union overlap is a coaxial slab and both cuts are box slabs
//! through the square crown, so inclusion–exclusion closes over `π`;
//! the one measured term is the chamfered plinth (blends have
//! no closed form here — its own row bounds it), and it propagates
//! through every later assertion as a measured constant. The stacked
//! bodies deliberately share NO plane with their neighbours: each disc
//! is inset into the one below it, so no boolean is asked about
//! coincident faces.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::{Dimension, Formula};
use viewer::session::BooleanSpec;

use core::f64::consts::{FRAC_PI_2, PI};

use common::{ang, body_volume, len, len3, near, scl3, session_insert, shape, translation};
use pncad::document::{Doc, DocEdit, RecipeNodeId, SlotId};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use viewer::props::SlotValue;
use viewer::session::{DocSession, PatternRuleSpec, ProfileShape, Refusal, SessionOp};
use viewer::tree::RowStatus;

// --- the rook's dimensions (metres) --------------------------------
//
// A ~37 mm piece. Each disc is drawn on its own plane and lifted up
// the rook's axis to a height INSIDE the body below it, so every union
// overlap is a clean coaxial slab and no two operand faces are
// coplanar.

/// The square plinth: side and height.
const PLINTH_SIDE: f64 = 0.036;
const PLINTH_H: f64 = 0.004;
/// The equal-setback chamfer on all twelve plinth edges.
const CHAMFER: f64 = 0.001;
/// The base disc: radius, lift, extrusion.
const BASE_R: f64 = 0.016;
const BASE_Z: f64 = 0.003;
const BASE_H: f64 = 0.006;
/// The shaft: radius, lift, extrusion.
const SHAFT_R: f64 = 0.009;
const SHAFT_Z: f64 = 0.008;
const SHAFT_H: f64 = 0.021;
/// The square crown block: side, lift, extrusion.
const DRUM_S: f64 = 0.026;
const DRUM_Z: f64 = 0.028;
const DRUM_H: f64 = 0.008;
/// The crenellation cutter: a slab across the whole crown — length
/// past the crown's side, thickness, lift, extrusion past
/// the crown's top.
const CUT_W: f64 = 0.040;
const CUT_T: f64 = 0.006;
const CUT_Z: f64 = 0.031;
const CUT_H: f64 = 0.008;
/// The second cutter is the FIRST one quarter-turned and lifted a
/// little, so its floor is not coplanar with the first slot's.
const CUT2_LIFT: f64 = 0.0004;
/// The taller drum the user edits in after the undo walk.
const DRUM_H2: f64 = 0.009;
/// The merlon block the user patterns round the crown: side, and how
/// far it is sunk into the crown's top so the union has no coplanar
/// operand faces. Its four copies sit in the crown's CORNER quadrants,
/// clear of both slots.
const MERLON_S: f64 = 0.006;
const MERLON_H: f64 = 0.004;
const MERLON_SINK: f64 = 0.001;
/// How far each merlon's centre is from the axis on each of x and y.
const MERLON_OFF: f64 = 0.009;

/// History states from `NewDocument` to the carved rook: the root
/// plus twenty-one committed actions. Four of those are LIFTS: every
/// profile is drawn on its own plane, so each body the rook stacks
/// above the plinth is moved up its axis before it is combined.
const CARVED_STATES: usize = 22;
/// The placing edits among the carving actions (D10), each riding in
/// the action that caused it: five booleans each re-targeting its moved
/// second operand into the plinth's space, its values as written (5).
/// Every body lands unplaced, so no action writes or re-points a world
/// placement.
const PLACING_EDITS: usize = 5;
/// Edits on the session's FINAL path: the twenty-two carving actions'
/// own edits (`CARVED_STATES` less the root), the world's edits among
/// them, and the taller-drum edit — edits, where `CARVED_STATES`
/// counts states.
const PATH_EDITS: usize = (CARVED_STATES - 1) + PLACING_EDITS + 1;

// --- closed forms ---------------------------------------------------

/// A disc's volume.
fn disc(r: f64, h: f64) -> f64 {
    PI * r * r * h
}

/// The cross-sectional area one slot removes per unit depth: the
/// cutter's thickness across the whole square crown.
fn slab_area() -> f64 {
    DRUM_S * CUT_T
}

/// The carved rook's volume at crown height `drum_h`, from the
/// measured chamfered-plinth volume `v_plinth` and inclusion–exclusion
/// over the stacked bodies and the two crossing slots.
fn carved_volume(v_plinth: f64, drum_h: f64) -> f64 {
    let blank = v_plinth + disc(BASE_R, BASE_H) - disc(BASE_R, PLINTH_H - BASE_Z)
        + disc(SHAFT_R, SHAFT_H)
        - disc(SHAFT_R, BASE_Z + BASE_H - SHAFT_Z)
        + DRUM_S * DRUM_S * drum_h
        - disc(SHAFT_R, SHAFT_Z + SHAFT_H - DRUM_Z);
    let top = DRUM_Z + drum_h;
    let (d1, d2) = (top - CUT_Z, top - (CUT_Z + CUT2_LIFT));
    // Slot 2 re-removes the centre square slot 1 already emptied.
    blank - slab_area() * d1 - d2 * (slab_area() - CUT_T * CUT_T)
}

// --- session helpers ------------------------------------------------

/// A circle profile of `radius` on its own plane.
fn circle(session: &mut DocSession, radius: f64) -> RecipeNodeId {
    session_insert(
        session,
        SessionOp::AddProfile {
            loops: vec![shape(&ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius,
            })],
        },
    )
}

/// `body` moved by `translation` — the transform gesture, which places
/// the body against its own geometry.
fn lifted(session: &mut DocSession, body: RecipeNodeId, translation: [f64; 3]) -> RecipeNodeId {
    session_insert(
        session,
        SessionOp::AddTransform {
            input: body.into(),
            translation: len3(translation),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Formula::zero(Dimension::Angle),
        },
    )
}

/// The whole sitting, top to bottom. One test because it is one
/// session: the history the undo walk asserts on is the history the
/// build minted, and splitting the chapters would fake that.
#[test]
fn a_chess_rook_is_authored_probed_branched_and_reopened() {
    let tol = Tol::witness();
    let mut session = DocSession::inline(Doc::empty_derived("whatever-was-open", tol), tol);

    // ── A fresh document ────────────────────────────────────────────
    let out = session.perform(SessionOp::NewDocument {
        name: "chess-rook".to_owned(),
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert_eq!(session.history().len(), 1, "a fresh root, no edits yet");

    // ── The plinth: a square pad, then all twelve edges chamfered ───
    let plinth = common::xy_box_in(&mut session, [PLINTH_SIDE, PLINTH_SIDE, PLINTH_H]);
    let v_pad = body_volume(&mut session, plinth, tol);
    assert!(
        near(v_pad, PLINTH_SIDE * PLINTH_SIDE * PLINTH_H),
        "the pad is the box the form described: {v_pad}"
    );

    // The blend's selection comes off the landed evaluation through
    // the shipped all-edges door — the whole-body set.
    let edges = {
        let eval = session.evaluation().expect("the pad landed");
        let edges = pncad::select::all_edges(eval, plinth);
        assert_eq!(edges.len(), 12, "a box has twelve edges");
        edges
    };
    let softened = session_insert(
        &mut session,
        SessionOp::AddChamfer {
            target: plinth,
            distance: len(CHAMFER),
            selection: edges,
        },
    );
    // A blend has no closed form here, so its row is a bound: the
    // twelve edge prisms overcount the loss (they overlap at corners)
    // and half of them undercount it.
    let v_plinth = body_volume(&mut session, softened, tol);
    let prisms = (8.0 * PLINTH_SIDE + 4.0 * PLINTH_H) * CHAMFER * CHAMFER / 2.0;
    assert!(
        v_plinth < v_pad - 0.5 * prisms && v_plinth > v_pad - 1.1 * prisms,
        "the chamfer takes about the edge prisms off: {v_plinth} vs {v_pad}"
    );

    // ── The base disc ────────────────────────────────────────────────
    let base_profile = circle(&mut session, BASE_R);
    let base = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: base_profile,
            distance: len(BASE_H),
        },
    );
    let base = lifted(&mut session, base, [0.0, 0.0, BASE_Z]);
    let u1 = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![softened.into(), base.into()]),
        },
    );
    let v_u1 = body_volume(&mut session, u1, tol);
    assert!(
        near(
            v_u1,
            v_plinth + disc(BASE_R, BASE_H) - disc(BASE_R, PLINTH_H - BASE_Z)
        ),
        "plinth ∪ base is the two less their slab: {v_u1}"
    );

    // ── The shaft and the crown drum, stacked the same way ──────────
    let shaft_profile = circle(&mut session, SHAFT_R);
    let shaft = session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: shaft_profile,
            distance: len(SHAFT_H),
        },
    );
    let shaft = lifted(&mut session, shaft, [0.0, 0.0, SHAFT_Z]);
    let u2 = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![u1.into(), shaft.into()]),
        },
    );
    let v_u2 = body_volume(&mut session, u2, tol);
    assert!(
        near(
            v_u2,
            v_u1 + disc(SHAFT_R, SHAFT_H) - disc(SHAFT_R, BASE_Z + BASE_H - SHAFT_Z)
        ),
        "… ∪ shaft: {v_u2}"
    );
    // The crown is a square block (a slab cut through a cylinder WALL
    // stops at the pierce ring's join door — the module docs carry the
    // reason), so its slots stay in the crossing-slots class.
    let (_, drum) = common::box_in(&mut session, [DRUM_S, DRUM_S, DRUM_H]);
    let crown = lifted(&mut session, drum, [0.0, 0.0, DRUM_Z]);
    let u3 = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![u2.into(), crown.into()]),
        },
    );
    let v_u3 = body_volume(&mut session, u3, tol);
    assert!(
        near(
            v_u3,
            v_u2 + DRUM_S * DRUM_S * DRUM_H - disc(SHAFT_R, SHAFT_Z + SHAFT_H - DRUM_Z)
        ),
        "… ∪ crown: {v_u3}"
    );

    // ── The crown: two crossing slots, the second the first cutter
    //    quarter-turned about the axis (one cutter body, moved by
    //    both transforms — the DAG's sharing) ─────────────────────────
    let (_, cutter) = common::box_in(&mut session, [CUT_W, CUT_T, CUT_H]);
    let cutter1 = lifted(&mut session, cutter, [0.0, 0.0, CUT_Z]);
    let cut1 = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: u3.into(),
                tool: cutter1.into(),
            },
        },
    );
    let v_cut1 = body_volume(&mut session, cut1, tol);
    assert!(
        near(v_cut1, v_u3 - slab_area() * (DRUM_Z + DRUM_H - CUT_Z)),
        "the first slot removes a chord slab of the drum: {v_cut1}"
    );
    let cutter2 = session_insert(
        &mut session,
        SessionOp::AddTransform {
            input: cutter.into(),
            translation: translation([0.0, 0.0, CUT_Z + CUT2_LIFT]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: ang(FRAC_PI_2),
        },
    );
    let carved = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Subtract {
                from: cut1.into(),
                tool: cutter2.into(),
            },
        },
    );
    let v_carved = body_volume(&mut session, carved, tol);
    assert!(
        near(v_carved, carved_volume(v_plinth, DRUM_H)),
        "four merlons stand: {v_carved} vs {}",
        carved_volume(v_plinth, DRUM_H)
    );
    assert_eq!(session.history().len(), CARVED_STATES);
    assert_eq!(
        viewer::world::drawn_nodes(session.committed_doc()),
        [carved],
        "the picture is one body: the rook, unplaced: {:?}",
        viewer::world::drawn_nodes(session.committed_doc())
            .into_iter()
            .map(|node| session.committed_doc().spoken(node).to_string())
            .collect::<Vec<_>>()
    );
    let carved_doc = session.committed_doc().clone();

    // ── The crenellation experiment: a merlon block, patterned round
    //    the axis — instances are not a body, and the fused door is
    //    what the refusal points at ─────────────────────────────────
    // The block is drawn about its own origin and patterned about the
    // rook's axis as it will stand once moved into a CORNER quadrant of
    // the crown: `MERLON_OFF` back along x and y, in the block's own
    // coordinates.
    let (block_profile, block) = common::box_in(&mut session, [MERLON_S, MERLON_S, MERLON_H]);
    let pattern = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: block.into(),
            count: 4,
            rule: PatternRuleSpec::Circular {
                through: len3([-MERLON_OFF, -MERLON_OFF, 0.0]),
                direction: scl3([0.0, 0.0, 1.0]),
                step: ang(FRAC_PI_2),
            },
        },
    );
    session.pump();
    {
        let eval = session.evaluation().expect("the pattern landed");
        let ValuePayload::Instances(instances) =
            &eval.value(pattern).expect("the pattern evaluated").payload
        else {
            panic!("a pattern evaluates to its instances");
        };
        assert_eq!(instances.len(), 4, "four blocks round the crown");
    }
    // Fusing THESE onto the rook is refused at the door, typed: a union's
    // member seat reads the family whole (D10 `[repetition]`), its
    // members joining the list, but the family stands in the block's own
    // space and the carved rook in the rook's, and a union reads its
    // members in one space.
    let states = session.history().len();
    let refused = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![carved.into(), pattern.into()]),
    });
    assert!(
        matches!(
            &refused.refusal,
            Some(Refusal::Edit(edit))
                if matches!(edit.as_ref(), pncad::document::EditError::SpaceMismatch { .. })
        ),
        "{:?}",
        refused.refusal
    );
    assert!(refused.committed.is_empty(), "a refusal commits nothing");
    assert_eq!(session.history().len(), states, "and mints no state");

    // The door that refusal points at: the same prototype and the
    // same rule, said FUSED — one body out instead of four, which the
    // boolean seat takes without complaint.
    let merlons = session_insert(
        &mut session,
        SessionOp::AddFusedPattern {
            input: block.into(),
            count: 4,
            rule: PatternRuleSpec::Circular {
                through: len3([-MERLON_OFF, -MERLON_OFF, 0.0]),
                direction: scl3([0.0, 0.0, 1.0]),
                step: ang(FRAC_PI_2),
            },
        },
    );
    session.pump();
    {
        let eval = session.evaluation().expect("the fused group landed");
        let value = eval.value(merlons).expect("the group evaluated");
        assert!(
            matches!(
                value.payload,
                ValuePayload::Boolean(pncad::document::BooleanValue::Body { .. })
            ),
            "the fused group is ONE body where the pattern was four"
        );
    }
    let v_merlons = body_volume(&mut session, merlons, tol);
    assert!(
        near(v_merlons, 4.0 * MERLON_S * MERLON_S * MERLON_H),
        "four disjoint merlons as one body: {v_merlons}"
    );
    // Moved into the crown's corners, clear of both slots, and sunk a
    // little into its top: the story's no-coplanar-operands rule holds
    // for this union too.
    let crowning = lifted(
        &mut session,
        merlons,
        [MERLON_OFF, MERLON_OFF, DRUM_Z + DRUM_H - MERLON_SINK],
    );
    let crenellated = session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![carved.into(), crowning.into()]),
        },
    );
    // Each merlon meets the crown in its sunk slab and nowhere else,
    // so the crenellated rook closes in the same arithmetic the rest
    // of the story does.
    let v_crenellated = body_volume(&mut session, crenellated, tol);
    assert!(
        near(
            v_crenellated,
            carved_volume(v_plinth, DRUM_H) + 4.0 * MERLON_S * MERLON_S * (MERLON_H - MERLON_SINK)
        ),
        "the rook with four merlons fused onto its crown: {v_crenellated}"
    );

    // ── Deleting the experiment: the affordance prices the cascade
    //    BEFORE the click, the delete takes the cone as one action ───
    // The crenellated rook lands unplaced: no copy of it is in the cone.
    // The fused door's family: the placement its union reads whole.
    let Some(pncad::document::Node::Union {
        members: pncad::document::Bodies::Family(family),
    }) = session.committed_doc().node(merlons)
    else {
        panic!("the fused door mints a union over the family");
    };
    let fused_family = session
        .committed_doc()
        .operation_of(family.read)
        .expect("the family is a node");
    let affordance = session.delete_affordance(block);
    assert_eq!(
        affordance.label,
        format!(
            "Delete Extrude {} and 5 dependent features",
            test_utils::refusal::tag(block.0.digest())
        )
    );
    assert!(
        affordance.hover.as_deref().is_some_and(|hover| {
            ["2 × Union", "3 × Place"]
                .iter()
                .all(|kind| hover.contains(kind))
        }),
        "the hover names each dependent by kind: {:?}",
        affordance.hover
    );
    assert_eq!(
        affordance.cascade,
        vec![crenellated, crowning, merlons, fused_family, pattern, block],
        "consumers first, the target last"
    );
    let deleted = session.perform(SessionOp::DeleteNode { node: block });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    assert_eq!(
        deleted.committed.len(),
        6,
        "one delete per doomed node, recorded as ONE action"
    );
    for (edit, want) in deleted.committed.iter().zip(&affordance.cascade) {
        assert!(
            matches!(edit, DocEdit::DeleteNode { id } if id == want),
            "the committed edits are the affordance's own list: {edit:?}"
        );
    }
    let doc = session.committed_doc();
    assert!(
        doc.node(block).is_none()
            && doc.node(pattern).is_none()
            && doc.node(merlons).is_none()
            && doc.node(crowning).is_none()
            && doc.node(crenellated).is_none()
    );
    assert!(
        doc.node(block_profile).is_some(),
        "nodes that only FED the target survive"
    );
    assert_eq!(session.history().len(), CARVED_STATES + 7);

    // One undo brings the WHOLE cone back — the cascade was one
    // action, so it is one step.
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    let doc = session.committed_doc();
    assert!(
        doc.node(block).is_some()
            && doc.node(pattern).is_some()
            && doc.node(merlons).is_some()
            && doc.node(crowning).is_some()
            && doc.node(crenellated).is_some(),
        "the block and everything downstream of it are back together"
    );
    session.pump();
    {
        let eval = session.evaluation().expect("the undo re-evaluated");
        let ValuePayload::Instances(instances) = &eval
            .value(pattern)
            .expect("the pattern is whole again")
            .payload
        else {
            panic!("a pattern evaluates to its instances");
        };
        assert_eq!(instances.len(), 4, "all four instances back");
    }

    // ── The undo/redo walk: back to the carved rook, part-way
    //    forward, back again — the tree keeps every state ────────────
    // Six: the branch is profile, extrude, pattern, fused group, the
    // move into the corners, union.
    for _ in 0..6 {
        assert!(session.perform(SessionOp::Undo).refusal.is_none());
    }
    assert!(
        session.committed_doc().bit_eq(&carved_doc),
        "six undos land exactly on the carved rook"
    );
    for _ in 0..3 {
        assert!(session.perform(SessionOp::Redo).refusal.is_none());
    }
    let doc = session.committed_doc();
    assert!(
        doc.node(block_profile).is_some() && doc.node(pattern).is_some(),
        "redo walks forward along the branch it left"
    );
    assert!(doc.node(merlons).is_none(), "…but only as far as asked");
    for _ in 0..3 {
        assert!(session.perform(SessionOp::Undo).refusal.is_none());
    }
    assert!(session.committed_doc().bit_eq(&carved_doc));
    assert_eq!(
        session.history().len(),
        CARVED_STATES + 7,
        "the walk destroyed nothing"
    );

    // ── An edit AFTER the undo: a taller drum. It mints a SIBLING of
    //    the abandoned branch; the experiment stays in the tree ───────
    let branch_point = session.history().current();
    let abandoned = session
        .history()
        .entry(branch_point)
        .active_child()
        .expect("undo remembers the branch it left");
    let edited = session.perform(SessionOp::SetSlot {
        node: drum,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(DRUM_H2),
    });
    assert!(edited.refusal.is_none(), "{:?}", edited.refusal);
    assert_eq!(edited.committed.len(), 1, "exactly one committed edit");
    let history = session.history();
    assert_eq!(
        history.len(),
        CARVED_STATES + 8,
        "a sibling, not a truncation"
    );
    assert_eq!(
        history.entry(history.current()).parent(),
        Some(branch_point),
        "the new state hangs off the carved rook"
    );
    let children = history.entry(branch_point).children();
    assert_eq!(children.len(), 2, "the branch point has both children");
    assert!(
        children.contains(&abandoned),
        "the abandoned experiment is still a child"
    );
    assert!(
        history.entry(abandoned).doc().node(block_profile).is_some(),
        "…and its document is intact, the block's profile and all"
    );

    // The edited rook is a taller crown, cut deeper by the same slots.
    let v_tall = body_volume(&mut session, carved, tol);
    assert!(
        near(v_tall, carved_volume(v_plinth, DRUM_H2)),
        "the whole cone re-evaluated through the taller drum: {v_tall} vs {}",
        carved_volume(v_plinth, DRUM_H2)
    );
    {
        let rows = session.tree_rows();
        assert_eq!(
            rows.len(),
            CARVED_STATES - 1,
            "one row per feature: the rook lands unplaced"
        );
        assert!(
            rows.iter().all(|row| row.status == RowStatus::Ok),
            "every feature is green: {rows:?}"
        );
    }

    // ── Save, reopen through the same doors, and the gallery ────────
    let dir = common::tempdir("story-rook");
    let path = dir.join("chess-rook.pncad");
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none(),
        "the rook saves"
    );
    let history = session.history();
    let path_edits: usize = history
        .path()
        .into_iter()
        .map(|id| history.entry(id).edits().len())
        .sum();
    assert_eq!(path_edits, PATH_EDITS, "the edits on the current path");
    let saved_doc = session.committed_doc().clone();
    assert!(
        session.perform(SessionOp::Open(path)).refusal.is_none(),
        "and reopens"
    );
    assert!(
        session.committed_doc().bit_eq(&saved_doc),
        "save → reopen is bit-identity on the document"
    );
    assert!(
        !session.history().can_undo(),
        "the file is the document's state, so undo ends at the open"
    );
    // Same ids, same solid, bit for bit.
    let v_reopened = body_volume(&mut session, carved, tol);
    assert_eq!(
        v_tall.to_bits(),
        v_reopened.to_bits(),
        "the reopened rook is the same solid"
    );

    // The story-gallery door (`common::story_gallery_dir` states the
    // contract): the finished piece, saved through the session's own
    // save door. Nothing above depends on it.
    if let Some(gallery) = common::story_gallery_dir() {
        let shot = gallery.join("story-authoring-chess-rook.pncad");
        let out = session.perform(SessionOp::Save(shot));
        assert!(
            out.refusal.is_none(),
            "the gallery save lands: {:?}",
            out.refusal
        );
    }
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}
