//! **`plate_param` — the parametric acceptance scene** (LIB-SWITCH §7,
//! PROFILES-V2 §V8).
//!
//! The switch's whole point, in one document: a profile whose geometry
//! is an EXPRESSION, so a document parameter drives it. Every other
//! corpus profile is literal-authored; this one is the first where
//! editing a `DocParam` changes a profile's shape.
//!
//! Shape: a rectangular plate with two round holes, unioned with a
//! small tab (the tab is there so the incremental probe has a sibling
//! branch to reuse — it carries no parametric weight). The holes'
//! radius is `Expr::param("hole_r")`, SHARED between both loops, so one
//! parameter edit moves two loops at once — the sharing V2's
//! expression layer exists for.
//!
//! The rows this document exists to serve live in
//! `tests/switch_plate_param.rs`:
//!
//! - edit `hole_r` → re-evaluate → NEW geometry (the volume moves, and
//!   moves by the right amount);
//! - `hole_r` → 0 → the driver refuses `NonpositiveCircleRadius`, and
//!   the node's typed error NAMES the loop and the step;
//! - `hole_r` → large enough that the holes overlap each other and the
//!   plate wall → the VALIDATE ladder refuses (a different door from
//!   the replay one);
//! - the §4d authoring door: an edit that writes a refusing radius
//!   INTO the program is refused at the door, while `SetDocParam` — by
//!   design — is not.

use editor_core::{
    BooleanOp, Dimension, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName, ProfileProgram,
    ProgramStep, ProgramTarget, SlotId,
};
use geom_core::{Affine3, Vec3};
use profile::SketchPlane;

use super::{CorpusDoc, Recorder};
use crate::fixture::len;

/// The document parameter that drives both holes.
pub const HOLE_R: &str = "hole_r";

/// The radius the document is authored at.
pub const HOLE_R_VALUE: f64 = 0.25;

/// The plate's extent (x0, x1, y0, y1) and thickness.
pub const PLATE: (f64, f64, f64, f64) = (0.0, 4.0, 0.0, 2.0);
/// The plate's extrusion depth.
pub const PLATE_DEPTH: f64 = 0.5;
/// Where the two holes sit. The 1.2 m separation is chosen so that a
/// radius exists (0.8) at which the two holes overlap EACH OTHER while
/// both still clear the plate walls — the acceptance suite's
/// validate-refusal row wants exactly one reason to refuse.
pub const HOLE_CENTRES: [(f64, f64); 2] = [(1.0, 1.0), (2.2, 1.0)];

/// A literal length expression.
fn lit(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length literal")
}

/// The hole radius, as the shared parameter reference.
pub fn hole_radius() -> Expr {
    Expr::param(ParamName::new(HOLE_R), Dimension::Length)
}

/// One hole loop: a circle whose radius is the shared parameter.
pub fn hole_loop(centre: (f64, f64)) -> LoopProgram {
    LoopProgram::Circle {
        centre: [lit(centre.0), lit(centre.1)],
        radius: hole_radius(),
    }
}

/// The plate outline, authored as an explicit chain rather than through
/// `LoopProgram::polygon`, so the document also exercises a hand-built
/// `Chain` beside the carrier forms.
pub fn outline() -> LoopProgram {
    let (x0, x1, y0, y1) = PLATE;
    LoopProgram::Chain(vec![
        ProgramStep::At([lit(x0), lit(y0)]),
        ProgramStep::LineTo(ProgramTarget::Point([lit(x1), lit(y0)])),
        ProgramStep::LineTo(ProgramTarget::Point([lit(x1), lit(y1)])),
        ProgramStep::LineTo(ProgramTarget::Point([lit(x0), lit(y1)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The parametric plate profile: outline first, then the two holes.
pub fn plate_profile() -> ProfileProgram {
    ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            outline(),
            hole_loop(HOLE_CENTRES[0]),
            hole_loop(HOLE_CENTRES[1]),
        ],
    }
}

pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new(HOLE_R),
        value: DocParam::continuous(Dimension::Length, HOLE_R_VALUE),
    });

    let plate_p = r.insert(Node::Profile(plate_profile()));
    let plate = r.insert(Node::Extrude {
        profile: plate_p,
        distance: len(PLATE_DEPTH),
    });

    // A plain tab on its own branch: the incremental probe bumps this
    // side, so the parametric side is what gets REUSED. Its sketch
    // plane sits strictly INSIDE the plate's slab and its footprint
    // crosses the plate's +x/+y walls transversally, so the union has
    // no coincident faces to refuse.
    let tab_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.125))),
        loops: vec![
            LoopProgram::polygon([(3.5, 1.75), (4.5, 1.75), (4.5, 2.5), (3.5, 2.5)])
                .expect("finite tab corners"),
        ],
    }));
    let tab = r.insert(Node::Extrude {
        profile: tab_p,
        distance: len(0.25),
    });

    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: plate,
        b: tab,
        declare: None,
    });

    CorpusDoc {
        name: "plate_param",
        about: "the parametric scene: a plate whose two hole radii are one DocParam",
        edits: r.edits,
        doc: r.doc,
        result: Some(union),
        pin: None,
        bump: DocEdit::SetParam {
            node: tab,
            slot: SlotId::Distance,
            expr: len(0.625),
        },
        bump_root: tab,
    }
}
