//! Corpus document **vessel** — a full revolve of a potter's meridian
//! (a base disc, a foot, a spherical belly, a mouth disc) hollowed
//! through `Node::Shell` with the MOUTH opened into a rim: the teapot's
//! class, chosen because the mouth of a solid of revolution is where
//! `shell_open` was wrong twice before it shipped, and the class the
//! by-description plane scan in the tour's teapot was written for.
//!
//! The recipe is five nodes — frame → in-plane axis → profile →
//! revolve → shell. The meridian is the teapot's, station for station:
//! every coordinate is a dyadic rational and the belly is a sphere of
//! radius `5/64` about `(0, 4/64)`, meeting the foot at the 3-4-5 point
//! `(4/64, 1/64)` and the mouth at `(3/64, 8/64)`, so both junction
//! residuals are exactly `0.0` in `f64`.
//!
//! # The mouth is TWO faces, and the document names both
//!
//! A FULL revolve emits every profile segment as two faces — the
//! `[0, π)` band and the `[π, 2π)` band — on ONE chart, so the mouth
//! disc is two half-discs on one plane. The kernel's rim surgery lifts
//! a chart as a whole and refuses a partial designation
//! (`ShellError::OpenFaceChartPartial`), and the document layer does
//! not complete charts on the author's behalf (that is a kernel rule,
//! and the seat's), so `open` names both halves. The order is the
//! rim's identity: the first named carries it, so the rim here is
//! `Rim(Band(mouth))`, and a document naming the halves the other way
//! round mints `Rim(BandPi(mouth))` and keys apart from this one.
//!
//! # No mass pin
//!
//! The belly is a spherical zone and every closed form carries `π`;
//! the corpus `MassPin` is asserted with `==`, so a pin here would fix
//! `f64` rounding of an irrational rather than the geometry — the
//! `die_fillet` / `tube_ring` disposition. Validity, closure, the face
//! census and the rim's name are what `lib_g17_shell_node.rs` pins.
//!
//! # Not in the registry
//!
//! Held beside [`super::documents`] for `cup`'s reason (its module
//! docs): a dual has no shell door, and registry membership requires
//! every document green at `Dual64`.

use editor_core::{
    Dimension, DocEdit, Expr, LoopProgram, Node, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, RecipeNodeId, SlotId, StableName, band, band_pi,
};

use crate::fixture::{ang, axis_in_plane, frame, len};

use super::{CorpusDoc, Recorder};

/// The foot's radius — the cylinder the vessel stands on.
pub const R_FOOT: f64 = 4.0 / 64.0;
/// The belly's sphere radius — the widest wall, at `Y_BELLY_C`.
pub const R_BELLY: f64 = 5.0 / 64.0;
/// The mouth's radius.
pub const R_NECK: f64 = 3.0 / 64.0;
/// Where the foot ends and the belly's arc begins.
pub const Y_FOOT: f64 = 1.0 / 64.0;
/// The belly sphere's centre, on the axis.
pub const Y_BELLY_C: f64 = 4.0 / 64.0;
/// The mouth's plane.
pub const Y_MOUTH: f64 = 8.0 / 64.0;
/// The wall thickness: a tenth of the belly's radius and an eighth of
/// the narrowest wall (the neck's), so every per-face reach margin is
/// definite by a wide margin.
pub const WALL: f64 = 1.0 / 128.0;
/// The bumped wall (dyadic; still an eighth under the neck's radius).
pub const WALL_BUMPED: f64 = 1.0 / 64.0;

/// The meridian's segment indices, in program order: base disc, foot,
/// belly, mouth disc, axis chord.
pub const SEG_BASE: u32 = 0;
/// The foot cylinder's segment.
pub const SEG_FOOT: u32 = 1;
/// The belly sphere's segment.
pub const SEG_BELLY: u32 = 2;
/// The mouth disc's segment.
pub const SEG_MOUTH: u32 = 3;

/// The meridian as a program (module docs).
pub fn meridian() -> LoopProgram {
    let lpt = |x: f64, y: f64| {
        [
            Expr::literal(x, Dimension::Length).unwrap(),
            Expr::literal(y, Dimension::Length).unwrap(),
        ]
    };
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(R_FOOT, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(R_FOOT, Y_FOOT))),
        ProgramStep::ArcTo(ProgramArcData::Center {
            c: lpt(0.0, Y_BELLY_C),
            winding: profile::ArcSweep::Ccw,
            target: ProgramTarget::Point(lpt(R_NECK, Y_MOUTH)),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(lpt(0.0, Y_MOUTH))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The vessel with its `open` list AUTHORED by the caller — two names,
/// whatever faces they are: the mouth's two halves in either order
/// (which decides which half carries the rim), or a designation the
/// kernel refuses. [`document`] names the mouth's `Band` half first.
pub fn document_with_open(open: fn(RecipeNodeId) -> [StableName; 2]) -> CorpusDoc {
    let mut r = Recorder::new();

    // u = +X (the radius), v = +Z (the axis): the meridian's own axis
    // is its +v through the origin, written in the frame it turns.
    let plane = r.insert(frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]));
    let axis = r.insert(axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![meridian()],
    }));
    let pot = r.insert(Node::Revolve {
        profile,
        axis,
        angle: ang(std::f64::consts::TAU),
    });
    let open = open(pot);
    let vessel = r.insert(Node::shell(pot, len(WALL), open.to_vec()));

    CorpusDoc {
        name: "vessel",
        about: "a revolved pot (foot, spherical belly, mouth) hollowed to a wall of 1/128 \
                with its mouth opened",
        edits: r.edits,
        doc: r.doc,
        result: Some(vessel),
        // π-valued closed forms are not dyadic — module docs.
        pin: None,
        bump: DocEdit::SetParam {
            node: vessel,
            slot: SlotId::ShellThickness,
            expr: len(WALL_BUMPED),
        },
        bump_root: vessel,
    }
}

/// The vessel's corpus document: the mouth's `Band` half named first.
pub fn document() -> CorpusDoc {
    document_with_open(|pot| [band(pot, SEG_MOUTH), band_pi(pot, SEG_MOUTH)])
}
