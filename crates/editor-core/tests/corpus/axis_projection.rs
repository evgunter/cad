//! Corpus document **axis_projection** — two faces of one revolve met
//! by a later boolean, each through a copy of it.
//!
//! One meridian, a bushing with a spigot below and a socket above,
//! revolved a full turn about its own `y`: a through bore `ρ = 1/4`, a
//! spigot `ρ ≤ 1/2`, `y ∈ [0, 1/2]` under a body `ρ ≤ 1`,
//! `y ∈ [1/2, 3/2]`, whose top is sunk by a socket `ρ ≤ 1/2` half a
//! unit deep. A copy of the revolve is placed in its space by one mate,
//! its own `axis` output on the revolve's, slid one unit up it and spun
//! off the seam, so its spigot fills the revolve's socket; the two are
//! unioned. The union glues the revolve's socket wall to the copy's
//! spigot wall: two faces of the one revolve, compared through the
//! revolve's axis read twice.
//!
//! No mass pin: the body is revolved, so its volume is `π`-valued.

use editor_core::{Coord, DocEdit, Formula, Node, ProgramStep, ProgramTarget, SlotId, TargetAxis};

use crate::fixture::{ang, len, len2};

use super::{CorpusDoc, PlacePart, Recorder, placed_in, revolved};

/// The through bore's radius, meters.
pub const BORE_R: f64 = 0.25;
/// The spigot's and the socket's radius, meters.
pub const SPIGOT_R: f64 = 0.5;
/// The body's radius, meters.
pub const BODY_R: f64 = 1.0;
/// The spigot's length and the socket's depth, meters.
pub const SPIGOT_L: f64 = 0.5;
/// The bushing's height from the spigot's foot to the body's top,
/// meters.
pub const HEIGHT: f64 = 1.5;
/// The copy's slide up the revolve's axis, meters: the body's height
/// less the spigot, so the copy's spigot fills the socket.
pub const SLIDE: f64 = 1.0;
/// The copy's spin about the revolve's axis, radians: off the seam the
/// revolve starts at.
pub const SPIN: f64 = 0.3;

/// The axis-projection corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let to = |x, y| ProgramTarget::Point(len2([x, y]));
    let bushing = revolved(
        &mut r,
        vec![
            ProgramStep::At(len2([BORE_R, 0.0])),
            ProgramStep::LineTo(to(SPIGOT_R, 0.0)),
            ProgramStep::LineTo(to(SPIGOT_R, SPIGOT_L)),
            ProgramStep::LineTo(to(BODY_R, SPIGOT_L)),
            ProgramStep::LineTo(to(BODY_R, HEIGHT)),
            ProgramStep::LineTo(to(SPIGOT_R, HEIGHT)),
            ProgramStep::LineTo(to(SPIGOT_R, HEIGHT - SPIGOT_L)),
            ProgramStep::LineTo(to(BORE_R, HEIGHT - SPIGOT_L)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ],
    );
    let placed = editor_core::Placed {
        mates: vec![crate::fixture::mate(bushing.axis(), bushing.axis())],
        values: vec![
            (Coord::Slide(TargetAxis::Y), len(SLIDE)),
            (Coord::SPIN, ang(SPIN)),
        ],
    };
    let copy = placed_in(&mut r, &bushing.body(), bushing.body(), placed);
    let whole = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![
            editor_core::BodyRead::plain(bushing.body()),
            copy.into(),
        ]),
    });
    r.place_part(whole);

    CorpusDoc {
        name: "axis_projection",
        about: "two copies of one bushing stacked spigot in socket along its own axis, unioned",
        edits: r.edits,
        doc: r.doc,
        result: Some(whole),
        pin: None, // revolved: π-valued
        // The revolve turns its full turn again: the profile is reused.
        bump: DocEdit::SetParam {
            node: bushing.revolve,
            slot: SlotId::RevolveAngle,
            value: Formula::turn_over(1).expect("a full turn").into(),
            fresh: Vec::new(),
        },
        bump_root: bushing.revolve,
    }
}
