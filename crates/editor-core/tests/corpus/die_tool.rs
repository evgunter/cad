//! Corpus document **die_tool** — the die tour's cutting tool as ONE
//! node.
//!
//! The tour's `diefillet` stop states the gap in a comment: assembling
//! a multi-shell cutting tool from N placed balls costs N−1 pairwise
//! `Boolean(Union)` nodes, and the FIRST ball's cavity faces come out
//! N−1 `FromA`/`FromB` segments deep. The group discipline itself is
//! load-bearing there — cutting the pips one at a time would present a
//! body already carrying a TRIMMED sphere face as the next operand,
//! which S13 refuses — so the tool is not optional, only expensive.
//!
//! `PlacedUnion(ball, Explicit(frames))` is the same tool said once:
//! one prototype ball, one listed frame per pip, one body out. Every
//! cavity face is then `Instance(i)` of a ball-local name — ONE
//! segment, whatever the pip count.
//!
//! Six pips here, one per face, rather than the tour's twenty-one: the
//! subject is the tool's SHAPE, not the pip layout, and the corpus pays
//! evaluation cost at every ε row, under Interval, and through both
//! sweep strategies. Six is the smallest set that exercises all six
//! face rotations — including the `±π/2` and `π` ones the tour
//! authors — and leaves the pips 0.764 m apart centre to centre against
//! a 0.09 m radius, comfortably certifiable.
//!
//! Vocabulary: Profile, Datum (Axis), Revolve, PlacedUnion (Explicit),
//! Boolean (Subtract), `InsertNode`, `SetParam`.
//!
//! # It crosses to Python, byte for byte
//!
//! `work/lib/log.md` carried "die_tool's Python re-authoring (banked
//! behind its Revolve/datum half)" from LIB-PYPU on: that unit bound
//! the placement vocabulary and authored the LINEAR twin
//! (`heat_sink_fins`, extrude-only), while this document's prototype is
//! a Revolve about a `Datum::Axis` whose meridian runs pole to pole —
//! the chart `die_pips`' retired deviation (b) existed to dodge.
//!
//! LIB-DIETOOL measured that half CLEARED, by construction. The recipe
//! below is unchanged — it has carried the natural meridian since the
//! day it was authored, `7581fb65d` having deleted the workaround one
//! commit earlier — and
//! `crates/pncad-py/tests/test_placed_union.py::TestTheDieTool` now
//! says the same seven nodes through the bound Python doors. The claim
//! is not eyeballed: `crates/editor-core/tests/lib_dietool_crossing.rs`
//! pins THIS document's saved text as `corpus/die_tool.pncad`, and the
//! Python row asserts its own `Doc.save()` against those bytes line for
//! line (bar the swept `epsilon`), so a recipe change on either side is
//! a red run rather than a silent divergence.
//!
//! # No mass pin
//!
//! `die_pips`' reason verbatim: the oracle is `L³ − 6 · cap(R, H)` with
//! `cap(r, h) = π h²(3r − h)/3`, π-valued and so not dyadic, while
//! [`MassPin`](super::MassPin) asserts with `==`. What this document
//! pins is validity and the census; `lib_placedunion.rs` pins the tool
//! against the pairwise Transform + Union chain it replaces.

use editor_core::{BooleanOp, DocEdit, Frame, LoopProgram, Node, ProfileProgram, SlotId};

use crate::fixture::{ang, axis_in_plane, frame, len, xy_frame};

use super::die_pips::{DIE_L, PIP_H, PIP_R, half_disc_program};
use super::{CorpusDoc, Recorder};

/// The pip ball's centre coordinate along its face normal, in the
/// cube's `[0, L]³` frame (`die_pips`' derivation verbatim: the face
/// plane, plus the `R − H` the centre stands outside it so the cavity
/// is a cap of height exactly `H`).
const PIP_C: f64 = DIE_L + (PIP_R - PIP_H);

/// The six face-centre placements, in the tour's own order and with
/// the tour's own authored rotations: each carries the master ball's
/// `+Z` pole onto the face normal it is cut by, so every chart stays
/// polar to the plane that cuts it.
///
/// Every angle is `0`, `±π/2` or `π` about a coordinate axis — the
/// placement is DATA, not a runtime `cross`/`acos` branch, which is
/// exactly what an explicit rule is for.
fn placements() -> Vec<Frame> {
    use std::f64::consts::{FRAC_PI_2, PI};
    let h = DIE_L / 2.0;
    let lo = DIE_L - PIP_C; // the −normal faces' centre coordinate
    let x = [1.0, 0.0, 0.0];
    let y = [0.0, 1.0, 0.0];
    let z = [0.0, 0.0, 1.0];
    [
        (z, 0.0, [h, h, PIP_C]),
        (x, PI, [h, h, lo]),
        (y, FRAC_PI_2, [PIP_C, h, h]),
        (y, -FRAC_PI_2, [lo, h, h]),
        (x, -FRAC_PI_2, [h, PIP_C, h]),
        (x, FRAC_PI_2, [h, lo, h]),
    ]
    .into_iter()
    .map(|(axis, angle, t)| Frame::rotate_then_translate(axis, angle, t))
    .collect()
}

/// The one-node-tool corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    // ---- the sharp cube, [0, L]³ ----
    let square = LoopProgram::polygon([(0.0, 0.0), (DIE_L, 0.0), (DIE_L, DIE_L), (0.0, DIE_L)])
        .expect("the die's square");
    let cube_plane = r.insert(xy_frame());
    let cube_p = r.insert(Node::Profile(ProfileProgram {
        plane: cube_plane,
        loops: vec![square],
    }));
    let cube = r.insert(Node::Extrude {
        profile: cube_p,
        distance: len(DIE_L),
    });

    // ---- the master ball, poled along +Z (`die_pips`' construction) ----
    let ball_plane = r.insert(frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]));
    // The axis, written in the meridian frame it turns: that frame's
    // v is world +Z, so the pole axis is its own +y through (0, 0).
    // It is minted AFTER the frame because it names it.
    let axis = r.insert(axis_in_plane(ball_plane, (0.0, 0.0), (0.0, 1.0)));
    let ball_p = r.insert(Node::Profile(ProfileProgram {
        plane: ball_plane,
        loops: vec![half_disc_program()],
    }));
    let ball = r.insert(Node::Revolve {
        profile: ball_p,
        axis,
        angle: ang(std::f64::consts::TAU),
    });

    // ---- the whole cutting tool, in ONE node ----
    let tool = r.insert(Node::placed_union_at(ball, placements()));
    let pipped = r.insert(Node::Boolean {
        op: BooleanOp::Subtract,
        a: cube,
        b: tool,
        declare: None,
    });

    CorpusDoc {
        name: "die_tool",
        about: "the die's multi-shell cutting tool as one PlacedUnion(Explicit) node",
        edits: r.edits,
        doc: r.doc,
        result: Some(pipped),
        // π-valued spherical caps are not dyadic — see the module docs.
        pin: None,
        // D2's incremental probe: the cube's extrude distance, which
        // moves ONLY the +Z face (every pip frame is absolute). The
        // ball and the whole tool are reused; the cube and the cut
        // recompute — the memoization claim the group must not break.
        // At 1.03125 the +Z pip still cuts a proper cap (the plane
        // sits 0.00875 below the ball's centre, leaving cap height
        // 0.08125 < 2R); every other pip is untouched.
        bump: DocEdit::SetParam {
            node: cube,
            slot: SlotId::Distance,
            expr: len(1.03125),
        },
        bump_root: cube,
    }
}
