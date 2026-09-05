//! Corpus document **face_sketch** — DOCM-1's register payoff: a
//! sketch drawn ON A FACE, spelled the way a user spells it.
//!
//! A box; a frame DERIVED from the box's top face
//! (`Datum::FaceFrame`, DOCM-REFERENCES-DESIGN DM1) named through the
//! naming vocabulary rather than transcribed as nine numbers; a
//! profile on that frame; an extrude of it. The boss therefore sits
//! on the box's top face BECAUSE it names that face: raise the box
//! and the boss rides up with it, which is the document's own
//! incremental-recompute probe (`bump`, below) and acceptance row A1.
//!
//! Joining the corpus registry gives the document the standard rows
//! for free — evaluation at every CI ε row and under `interval`
//! (DM1c: the profile on the derived frame is placed at the lane
//! scalar), persistence round-trip (D6.1, acceptance row A8), and the
//! latency table.
//!
//! Dyadic mass pin: 2 × 2 × 1 box plus a 1 × 1 × 0.5 boss on top.

use editor_core::{CapEnd, DocEdit, Node, RoleSeg, SlotId};

use crate::fixture::{ang, desc, fname, len, xy_frame};

use super::{CorpusDoc, MassPin, Recorder};

/// The box's footprint half-extent, meters.
const BOX_HALF: f64 = 1.0;
/// The box's height, meters — the parameter the probe raises.
const BOX_H: f64 = 1.0;
/// The boss's footprint half-extent, meters.
const BOSS_HALF: f64 = 0.5;
/// The boss's height, meters.
const BOSS_H: f64 = 0.5;
/// The sketch's turn about the face's outward normal, radians.
const SPIN: f64 = 0.3;

/// The face-sketch corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    // ---- the box, [-1, 1]² × [0, 1] ----
    let box_plane = r.insert(xy_frame());
    let box_p = r.insert(Node::Profile(desc(
        box_plane,
        vec![vec![
            (-BOX_HALF, -BOX_HALF),
            (BOX_HALF, -BOX_HALF),
            (BOX_HALF, BOX_HALF),
            (-BOX_HALF, BOX_HALF),
        ]],
    )));
    let cube = r.insert(Node::Extrude {
        profile: box_p,
        distance: len(BOX_H),
    });

    // ---- "sketch on this face": the frame derived from the top cap ----
    // The face is NAMED, not measured: the extrude's top cap, in the
    // vocabulary every selection uses. The sketch is TURNED on the
    // face (a non-zero spin), so the corpus rows walk the rotation
    // about the outward normal rather than the identity.
    let top = r.insert(Node::Datum(editor_core::Datum::FaceFrame {
        at: cube,
        face: fname(cube, RoleSeg::Cap(CapEnd::End)),
        spin: ang(SPIN),
    }));
    // The boss profile, centred on the frame's origin (the carrier's
    // own distinguished point, which for an extrude cap is the
    // sketch origin lifted to the cap). A square about its centre, so
    // the turn moves its corners and not its mass.
    let boss_p = r.insert(Node::Profile(desc(
        top,
        vec![vec![
            (-BOSS_HALF, -BOSS_HALF),
            (BOSS_HALF, -BOSS_HALF),
            (BOSS_HALF, BOSS_HALF),
            (-BOSS_HALF, BOSS_HALF),
        ]],
    )));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(BOSS_H),
    });

    CorpusDoc {
        name: "face_sketch",
        about: "DOCM-1: a boss sketched ON a box's top face through a derived frame (Datum::FaceFrame)",
        edits: r.edits,
        doc: r.doc,
        // The boss alone is the headline solid: it is what the derived
        // frame places, and its pin is what moves when the box does.
        result: Some(boss),
        pin: Some(MassPin {
            volume: (2.0 * BOSS_HALF) * (2.0 * BOSS_HALF) * BOSS_H,
            area: Some(
                2.0 * (2.0 * BOSS_HALF) * (2.0 * BOSS_HALF) + 4.0 * (2.0 * BOSS_HALF) * BOSS_H,
            ),
        }),
        // D2's incremental probe, and acceptance row A1: raise the box.
        // The box's own frame and profile are reused; the box, the
        // derived frame, the boss profile and the boss recompute — the
        // frame is IN the cone because the face it names moved.
        bump: DocEdit::SetParam {
            node: cube,
            slot: SlotId::Distance,
            expr: len(1.5),
        },
        bump_root: cube,
    }
}
