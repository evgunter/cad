//! Corpus document **face_sketch** — DOCM-1's register payoff: a
//! sketch drawn ON A FACE, spelled the way a user spells it.
//!
//! A box; a boss drawn in its own coordinates and placed ON the box's
//! top face by a plane mate that NAMES that face through the naming
//! vocabulary, rather than transcribing nine numbers; the two unioned.
//! The boss therefore sits on the box's top face BECAUSE it names that
//! face: raise the box and the boss rides up with it, which is the
//! document's own incremental-recompute probe (`bump`, below) and
//! acceptance row A1.
//!
//! Joining the corpus registry gives the document the standard rows
//! for free — evaluation at every CI ε row and under `interval` (the
//! placement solves at the lane scalar), persistence round-trip (D6.1,
//! acceptance row A8), and the latency table.
//!
//! Dyadic mass pin: the 2 × 2 × 1 box plus the 1 × 1 × 0.5 boss on top.

use super::PlacePart;
use editor_core::{
    CapEnd, Coord, Dimension, DocEdit, Formula, Mate, Node, Operand, RoleSeg, SlotId, TargetAxis,
};

use crate::fixture::{ang, desc, fname, len, plane};

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
    let box_p = r.insert(Node::Profile(desc(vec![vec![
        (-BOX_HALF, -BOX_HALF),
        (BOX_HALF, -BOX_HALF),
        (BOX_HALF, BOX_HALF),
        (-BOX_HALF, BOX_HALF),
    ]])));
    let cube = r.insert(Node::Extrude {
        profile: box_p.into(),
        distance: len(BOX_H),
    });

    // ---- "sketch on this face" ----
    // The boss profile, centred on its own origin: a square about its
    // centre, so the turn below moves its corners and not its mass.
    let boss_p = r.insert(Node::Profile(desc(vec![vec![
        (-BOSS_HALF, -BOSS_HALF),
        (BOSS_HALF, -BOSS_HALF),
        (BOSS_HALF, BOSS_HALF),
        (-BOSS_HALF, BOSS_HALF),
    ]])));
    let boss = r.insert(Node::Extrude {
        profile: boss_p.into(),
        distance: len(BOSS_H),
    });
    // The face is NAMED, not measured: the extrude's top cap, in the
    // vocabulary every selection uses. The boss's start cap, flipped to
    // look up its own `z`, lies on it, centred on the box's axis and
    // TURNED about the face's outward normal (a non-zero spin), so the
    // corpus rows walk the rotation rather than the identity.
    let cap = |node, end| Operand::select(node, vec![fname(node, RoleSeg::Cap(end))]);
    let on = Operand::Pose(Box::new(editor_core::pose::PoseDef::Flip {
        pose: editor_core::pose::RoleRead::plain(plane(cap(boss, CapEnd::Start))),
    }));
    let placed = r.insert(crate::fixture::stood(editor_core::BodyRead::plain(
        crate::fixture::body_read(&r.doc, boss.into()),
    )));
    r.constrain(
        placed,
        Mate {
            on,
            to: plane(cap(cube, CapEnd::End)),
            orient: editor_core::Orientation::IDENTITY,
        },
        vec![
            (
                Coord::Slide(TargetAxis::X),
                Formula::zero(Dimension::Length),
            ),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, ang(SPIN)),
        ],
    );
    let whole = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![cube.into(), placed.into()]),
    });

    r.place_part(whole);

    CorpusDoc {
        name: "face_sketch",
        about: "DOCM-1: a boss sketched ON a box's top face, placed by a mate that names the face",
        edits: r.edits,
        doc: r.doc,
        // The box with its boss is the headline solid: the boss's pin
        // rides on the box's top face.
        result: Some(whole),
        pin: Some(MassPin {
            volume: (2.0 * BOX_HALF) * (2.0 * BOX_HALF) * BOX_H
                + (2.0 * BOSS_HALF) * (2.0 * BOSS_HALF) * BOSS_H,
            // The box's surface, less the boss's footprint on its top,
            // plus the boss's top and four walls.
            area: Some(
                2.0 * (2.0 * BOX_HALF) * (2.0 * BOX_HALF)
                    + 4.0 * (2.0 * BOX_HALF) * BOX_H
                    + 4.0 * (2.0 * BOSS_HALF) * BOSS_H,
            ),
        }),
        // D2's incremental probe, and acceptance row A1: raise the box.
        // The box's profile and the boss's construction are reused; the
        // box, the placement, the union and its world copy recompute —
        // the placement is IN the cone because the face it names moved.
        bump: DocEdit::SetParam {
            node: cube,
            slot: SlotId::Distance,
            value: len(1.5).into(),
            fresh: Vec::new(),
        },
        bump_root: cube,
    }
}
