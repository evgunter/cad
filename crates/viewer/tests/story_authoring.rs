//! **One user, one sitting, one chess rook** — the single-part
//! authoring story, driven end to end through the session's own op
//! vocabulary (`SessionOp` → [`DocSession::perform`]) with no renderer
//! anywhere.
//!
//! The part is a rook: a chamfered square plinth, a base disc and a
//! cylindrical shaft, and a square crown block whose crenellations are
//! cut by two crossing slots — every dimension chosen so the evaluated
//! volume has a closed form the assertions derive beside the ops. (The
//! crown is square deliberately: a slab cut through a cylinder WALL is
//! the boolean's curved-sector frontier and refuses typed —
//! `CurvedSectorSideUnsupported`, issue 1455's frontier — while the
//! curved unions below are the supported boss class. The story stays
//! on what the kernel ships. Stacked discs stand in for the revolved
//! silhouette a rook naturally is: `ProfileShape` spells no revolvable
//! silhouette — issue 1457.) On the way
//! the user mis-picks a boolean (typed refusals), tries to crown the
//! rook with a circular pattern and learns instances are not a body,
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
//! profiles deliberately share NO plane with their neighbours: each
//! disc is inset into the one below it, so no boolean is asked about
//! coincident faces.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use core::f64::consts::{FRAC_PI_2, PI};

use common::{body_volume, insert, near};
use pncad::document::{BooleanOp, Doc, DocEdit, RecipeNodeId, SlotId};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::prelude::ValuePayload;
use pncad::profile::SketchPlane;
use viewer::props::SlotValue;
use viewer::session::{
    DatumSpec, DocSession, NodeKindWanted, PatternRuleSpec, ProfileShape, Refusal, SessionOp,
};
use viewer::tree::RowStatus;

// --- the rook's dimensions (metres) --------------------------------
//
// A ~37 mm piece. Each disc's sketch plane sits INSIDE the body below
// it, so every union overlap is a clean coaxial slab and no two
// operand faces are coplanar.

/// The square plinth: side and height.
const PLINTH_SIDE: f64 = 0.036;
const PLINTH_H: f64 = 0.004;
/// The equal-setback chamfer on all twelve plinth edges.
const CHAMFER: f64 = 0.001;
/// The base disc: radius, sketch height, extrusion.
const BASE_R: f64 = 0.016;
const BASE_Z: f64 = 0.003;
const BASE_H: f64 = 0.006;
/// The shaft: radius, sketch height, extrusion.
const SHAFT_R: f64 = 0.009;
const SHAFT_Z: f64 = 0.008;
const SHAFT_H: f64 = 0.021;
/// The square crown block: side, sketch height, extrusion.
const DRUM_S: f64 = 0.026;
const DRUM_Z: f64 = 0.028;
const DRUM_H: f64 = 0.008;
/// The crenellation cutter: a slab across the whole crown — length
/// past the crown's side, thickness, sketch height, extrusion past
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

/// History states from `NewDocument` to the carved rook: the root
/// plus seventeen committed edits.
const CARVED_STATES: usize = 18;
/// Edits on the FINAL saved path: the seventeen carving edits
/// (`CARVED_STATES` less the root) plus the taller-drum edit. Equal to
/// `CARVED_STATES` only because one root and one drum edit cancel —
/// the two constants count different quantities (states vs edits).
const SAVED_PATH_EDITS: usize = (CARVED_STATES - 1) + 1;

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

/// A circle profile of `radius` on the plane `z` up the rook's axis.
fn circle_at(session: &mut DocSession, radius: f64, z: f64) -> RecipeNodeId {
    insert(
        session,
        SessionOp::AddProfile {
            plane: SketchPlane::from_frame(
                Point3::new(0.0, 0.0, z),
                Vec3::unit_x(),
                Vec3::unit_y(),
            ),
            loops: vec![ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius,
            }],
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
    let plinth_profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Rectangle {
                width: PLINTH_SIDE,
                height: PLINTH_SIDE,
            }],
        },
    );
    let plinth = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: plinth_profile,
            distance: PLINTH_H,
        },
    );
    let v_pad = body_volume(&mut session, plinth, tol);
    assert!(
        near(v_pad, PLINTH_SIDE * PLINTH_SIDE * PLINTH_H),
        "the pad is the box the form described: {v_pad}"
    );

    // The blend's selection comes off the landed evaluation through
    // the shipped all-edges door — the whole-body set, because the
    // freeze semantics admit only fully-requested chain sets.
    let edges = {
        let eval = session.evaluation().expect("the pad landed");
        let edges = pncad::select::all_edges(eval, plinth);
        assert_eq!(edges.len(), 12, "a box has twelve edges");
        edges
    };
    let softened = insert(
        &mut session,
        SessionOp::AddChamfer {
            target: plinth,
            distance: CHAMFER,
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

    // ── The base disc, and a mis-pick at the boolean door ───────────
    let base_profile = circle_at(&mut session, BASE_R, BASE_Z);
    let base = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: base_profile,
            distance: BASE_H,
        },
    );
    // The user double-picks the plinth into both operand seats. The
    // door refuses TYPED, names the node, and records nothing.
    let states = session.history().len();
    let mispick = session.perform(SessionOp::AddBoolean {
        op: BooleanOp::Union,
        a: softened,
        b: softened,
    });
    assert!(
        matches!(mispick.refusal, Some(Refusal::SelfBoolean { node }) if node == softened),
        "{:?}",
        mispick.refusal
    );
    assert!(mispick.committed.is_empty(), "a refusal commits nothing");
    assert_eq!(session.history().len(), states, "and mints no state");

    let u1 = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: softened,
            b: base,
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
    let shaft_profile = circle_at(&mut session, SHAFT_R, SHAFT_Z);
    let shaft = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: shaft_profile,
            distance: SHAFT_H,
        },
    );
    let u2 = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: u1,
            b: shaft,
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
    // is the boolean's curved-sector frontier, issue 1455 — the module
    // docs carry the ruling), so its slots stay in the crossing-slots
    // class.
    let drum_profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::from_frame(
                Point3::new(0.0, 0.0, DRUM_Z),
                Vec3::unit_x(),
                Vec3::unit_y(),
            ),
            loops: vec![ProfileShape::Rectangle {
                width: DRUM_S,
                height: DRUM_S,
            }],
        },
    );
    let drum = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: drum_profile,
            distance: DRUM_H,
        },
    );
    let u3 = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: u2,
            b: drum,
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
    //    quarter-turned about the axis (one cutter body, consumed by
    //    both the subtract and the transform — the DAG's sharing) ────
    let cutter_profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::from_frame(
                Point3::new(0.0, 0.0, CUT_Z),
                Vec3::unit_x(),
                Vec3::unit_y(),
            ),
            loops: vec![ProfileShape::Rectangle {
                width: CUT_W,
                height: CUT_T,
            }],
        },
    );
    let cutter = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: cutter_profile,
            distance: CUT_H,
        },
    );
    let cut1 = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Subtract,
            a: u3,
            b: cutter,
        },
    );
    let v_cut1 = body_volume(&mut session, cut1, tol);
    assert!(
        near(v_cut1, v_u3 - slab_area() * (DRUM_Z + DRUM_H - CUT_Z)),
        "the first slot removes a chord slab of the drum: {v_cut1}"
    );
    let cutter2 = insert(
        &mut session,
        SessionOp::AddTransform {
            input: cutter,
            translation: [0.0, 0.0, CUT2_LIFT],
            rotation_axis: [0.0, 0.0, 1.0],
            rotation_angle: FRAC_PI_2,
        },
    );
    let carved = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Subtract,
            a: cut1,
            b: cutter2,
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
        session.committed_doc().roots(),
        &[carved],
        "one product root: the rook"
    );
    let carved_doc = session.committed_doc().clone();

    // ── The pattern experiment: a merlon block, patterned round the
    //    axis — and the discovery that instances are not a body ──────
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 0.0, 1.0],
            },
        },
    );
    let block_profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::from_frame(
                Point3::new(0.010, 0.0, DRUM_Z + DRUM_H),
                Vec3::unit_x(),
                Vec3::unit_y(),
            ),
            loops: vec![ProfileShape::Rectangle {
                width: 0.006,
                height: 0.006,
            }],
        },
    );
    let block = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile: block_profile,
            distance: 0.004,
        },
    );
    let pattern = insert(
        &mut session,
        SessionOp::AddPattern {
            input: block,
            count: 4,
            rule: PatternRuleSpec::Circular {
                axis,
                step: FRAC_PI_2,
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
    // Fusing them onto the rook is refused at the door, typed: a
    // pattern's value is several bodies, not the ONE a boolean seat
    // consumes (the F4 division the heatsink demo documents). No
    // session op wraps `Node::PlacedUnion`, so the vocabulary has no
    // door that fuses a pattern — issue 1456 — which is why this
    // experiment can only end in the delete below.
    let states = session.history().len();
    let refused = session.perform(SessionOp::AddBoolean {
        op: BooleanOp::Union,
        a: carved,
        b: pattern,
    });
    assert!(
        matches!(
            refused.refusal,
            Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body })
                if node == pattern
        ),
        "{:?}",
        refused.refusal
    );
    assert!(refused.committed.is_empty(), "a refusal commits nothing");
    assert_eq!(session.history().len(), states, "and mints no state");

    // ── Deleting the experiment: the affordance prices the cascade
    //    BEFORE the click, the delete takes the cone as one action ───
    let affordance = session.delete_affordance(block);
    assert_eq!(
        affordance.label,
        "Delete feature 'Extrude' and 1 dependent feature"
    );
    assert!(
        affordance
            .hover
            .as_deref()
            .is_some_and(|hover| hover.contains("1 × Pattern")),
        "the hover names the dependent by kind: {:?}",
        affordance.hover
    );
    assert_eq!(
        affordance.cascade,
        vec![pattern, block],
        "consumers first, the target last"
    );
    let deleted = session.perform(SessionOp::DeleteNode { node: block });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    assert_eq!(
        deleted.committed.len(),
        2,
        "one delete per doomed node, recorded as ONE action"
    );
    for (edit, want) in deleted.committed.iter().zip(&affordance.cascade) {
        assert!(
            matches!(edit, DocEdit::DeleteNode { id } if id == want),
            "the committed edits are the affordance's own list: {edit:?}"
        );
    }
    let doc = session.committed_doc();
    assert!(doc.node(block).is_none() && doc.node(pattern).is_none());
    assert!(
        doc.node(axis).is_some() && doc.node(block_profile).is_some(),
        "nodes that only FED the target survive as roots of their own"
    );
    assert_eq!(session.history().len(), CARVED_STATES + 5);

    // One undo brings the WHOLE cone back — the cascade was one
    // action, so it is one step.
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    let doc = session.committed_doc();
    assert!(
        doc.node(block).is_some() && doc.node(pattern).is_some(),
        "the block and its pattern are back together"
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
    for _ in 0..4 {
        assert!(session.perform(SessionOp::Undo).refusal.is_none());
    }
    assert!(
        session.committed_doc().bit_eq(&carved_doc),
        "four undos land exactly on the carved rook"
    );
    for _ in 0..2 {
        assert!(session.perform(SessionOp::Redo).refusal.is_none());
    }
    let doc = session.committed_doc();
    assert!(
        doc.node(axis).is_some() && doc.node(block_profile).is_some(),
        "redo walks forward along the branch it left"
    );
    assert!(doc.node(block).is_none(), "…but only as far as asked");
    for _ in 0..2 {
        assert!(session.perform(SessionOp::Undo).refusal.is_none());
    }
    assert!(session.committed_doc().bit_eq(&carved_doc));
    assert_eq!(
        session.history().len(),
        CARVED_STATES + 5,
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
        CARVED_STATES + 6,
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
        history.entry(abandoned).doc().node(axis).is_some(),
        "…and its document is intact, axis and all"
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
        assert_eq!(rows.len(), CARVED_STATES - 1, "one row per feature");
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
    // The file records the CURRENT PATH's linear log — the abandoned
    // branch is session state, not document.
    assert_eq!(session.history().path_edits().len(), SAVED_PATH_EDITS);
    let saved_doc = session.committed_doc().clone();
    assert!(
        session.perform(SessionOp::Open(path)).refusal.is_none(),
        "and reopens"
    );
    assert!(
        session.committed_doc().bit_eq(&saved_doc),
        "save → reopen is bit-identity on the document"
    );
    assert_eq!(
        session.history().path_edits().len(),
        SAVED_PATH_EDITS,
        "the reopened history replays exactly the saved log"
    );
    // Same ids (the log replays in order), same solid, bit for bit.
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
