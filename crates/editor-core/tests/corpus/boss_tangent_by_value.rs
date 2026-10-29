//! Corpus document **boss_tangent_by_value** — a boss whose wall is
//! tangent to a cylinder because two numbers were typed equal.
//!
//! A cylinder, a disc of radius `1/2` extruded `1`; a boss, the
//! rectangle `x ∈ [0, 1]`, `y ∈ [−1/4, 1/2]` extruded `1/2`, placed
//! against the cylinder where it stands. The boss's wall `y = 1/2` is
//! tangent to the cylinder along the line `x = 0`, `y = 1/2` only
//! because its offset was written equal to the radius: nothing
//! constructs it. The two are unioned, and the union glues that
//! tangency.
//!
//! The scene stands beside the corpus registry
//! ([`super::stage6_beside_the_registry`]): its union is green at `f64`
//! and escalates at `Interval`, which `coincidence_door.rs` pins beside
//! its rows.
//!
//! No mass pin: the cylinder's volume is `π`-valued.

use editor_core::{DocEdit, Frame, LoopProgram, Node, ProfileProgram, SlotId};

use crate::fixture::len;

use super::{CorpusDoc, PlacePart, Recorder};

/// The cylinder's radius, meters, and the boss wall's offset from its
/// axis, typed the same.
pub const RADIUS: f64 = 0.5;
/// The cylinder's height, meters.
pub const HEIGHT: f64 = 1.0;
/// The boss's reach along `x`, meters.
pub const REACH: f64 = 1.0;
/// The boss's other wall, meters: across the cylinder's axis side.
pub const BACK: f64 = -0.25;
/// The boss's height, meters.
pub const BOSS_H: f64 = 0.5;

/// The boss-tangent-by-value corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let disc = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::circle(0.0, 0.0, RADIUS).expect("a disc")],
        ids: Vec::new(),
    }));
    let cylinder = r.insert(Node::Extrude {
        profile: disc.into(),
        distance: len(HEIGHT),
    });
    let footprint = r.insert(Node::Profile(crate::fixture::desc(vec![vec![
        (0.0, BACK),
        (REACH, BACK),
        (REACH, RADIUS),
        (0.0, RADIUS),
    ]])));
    let boss = r.insert(Node::Extrude {
        profile: footprint.into(),
        distance: len(BOSS_H),
    });
    let placed = r.against(cylinder, boss, &Frame::IDENTITY);
    let whole = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![cylinder.into(), placed.into()]),
    });
    r.place_part(whole);

    CorpusDoc {
        name: "boss_tangent_by_value",
        about: "a boss whose wall's offset is typed equal to a cylinder's radius, unioned",
        edits: r.edits,
        doc: r.doc,
        result: Some(whole),
        pin: None, // the cylinder is π-valued
        // The boss grows taller: the cylinder is reused.
        bump: DocEdit::SetParam {
            node: boss,
            slot: SlotId::Distance,
            value: len(0.75).into(),
            fresh: Vec::new(),
        },
        bump_root: boss,
    }
}
