//! Corpus document **revolves_on_one_axis** — two revolves made
//! coaxial by reading one pair of variables.
//!
//! Two rings, each a rectangle revolved a full turn about the profile's
//! `y` heading through the point `(ax, ay)`, both revolves' `axis_through`
//! slots reading the document variables `ax` and `ay`. The first ring is
//! `ρ ∈ [1/4, 1/2]`, `y ∈ [0, 1]`; the second, the same radii over
//! `y ∈ [1, 3/2]`, stands where it is in the first's space, on its top.
//! The two are unioned: the union glues the second's base to the first's
//! top, and reads the two bores and the two walls on one carrier each.
//!
//! No mass pin: both rings are revolved, so their volumes are
//! `π`-valued.

use editor_core::{
    Dimension, DocEdit, Formula, FreeVar, LoopProgram, Node, Operand, ProfileProgram, ProgramStep,
    ProgramTarget, RecipeNodeId, VarDecl, VarName,
};

use crate::fixture::{ang, len2};

use super::{CorpusDoc, PlacePart, Recorder, placed_in};

/// The axis's point in the profile plane, meters.
pub const AXIS_AT: (f64, f64) = (0.5, 0.0);
/// The rings' bore and outer radii, meters.
pub const RADII: (f64, f64) = (0.25, 0.5);
/// The first ring's height and the second's, meters.
pub const HEIGHTS: (f64, f64) = (1.0, 0.5);

/// A ring `ρ ∈ RADII`, `y ∈ [lo, hi]`, revolved about the axis the
/// variables `ax` and `ay` place.
fn ring(r: &mut Recorder, (lo, hi): (f64, f64)) -> RecipeNodeId {
    let (inner, outer) = RADII;
    let x = |rho: f64| AXIS_AT.0 + rho;
    let to = |rho, y| ProgramTarget::Point(len2([x(rho), y]));
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At(len2([x(inner), lo])),
            ProgramStep::LineTo(to(outer, lo)),
            ProgramStep::LineTo(to(outer, hi)),
            ProgramStep::LineTo(to(inner, hi)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
        ids: Vec::new(),
    }));
    let var = |name| Formula::named(VarName::from_static(name), Dimension::Length);
    r.insert(Node::Revolve {
        profile: profile.into(),
        axis_through: [var("ax"), var("ay")],
        axis_heading: crate::fixture::along_y(),
        angle: ang(std::f64::consts::TAU),
    })
}

/// The revolves-on-one-axis corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    for (name, value) in [("ax", AXIS_AT.0), ("ay", AXIS_AT.1)] {
        r.push(DocEdit::DeclareVar {
            name: VarName::from_static(name),
            def: VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
        });
    }
    let first = ring(&mut r, (0.0, HEIGHTS.0));
    let second = ring(&mut r, (HEIGHTS.0, HEIGHTS.0 + HEIGHTS.1));
    let stacked = placed_in(
        &mut r,
        &Operand::output(first, 0),
        Operand::output(second, 0),
        editor_core::stand(),
    );
    let whole = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![
            editor_core::BodyRead::plain(Operand::output(first, 0)),
            stacked.into(),
        ]),
    });
    r.place_part(whole);

    CorpusDoc {
        name: "revolves_on_one_axis",
        about: "two rings whose revolves read one pair of axis variables, stacked and unioned",
        edits: r.edits,
        doc: r.doc,
        result: Some(whole),
        pin: None, // revolved: π-valued
        // The second ring turns its full turn again: the first is
        // reused.
        bump: DocEdit::SetParam {
            node: second,
            slot: editor_core::SlotId::RevolveAngle,
            value: Formula::turn_over(1).expect("a full turn").into(),
            fresh: Vec::new(),
        },
        bump_root: second,
    }
}
