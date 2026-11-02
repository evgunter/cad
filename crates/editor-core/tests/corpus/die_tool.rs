//! Corpus document **die_tool** — the die tour's cutting tool, its
//! pips written out.
//!
//! The tour's `diefillet` stop states why the tool is one body: cutting
//! the pips one at a time would present a body already carrying a
//! TRIMMED sphere face as the next operand, which S13 refuses. An
//! irregular family is its members written out, each saying where it is
//! (D10 `[repetition]`): one placement of the master ball per pip, each
//! bundle its turns and lengths as written, and one `Union` of the six.
//!
//! Six pips here, one per face, rather than the tour's twenty-one: the
//! subject is the tool's SHAPE, not the pip layout, and the corpus pays
//! evaluation cost at every ε row, under Interval, and through both
//! sweep strategies. Six is the smallest set that exercises all six
//! face rotations — including the `±turn/4` and `turn/2` ones the tour
//! authors — and leaves the pips 0.764 m apart centre to centre against
//! a 0.09 m radius.
//!
//! Vocabulary: Profile, Revolve, Place, Union, Subtract, `InsertNode`,
//! `SetParam`.
//!
//! # It crosses to Python, byte for byte
//!
//! `work/lib/log.md` carried "die_tool's Python re-authoring (banked
//! behind its Revolve/datum half)" from LIB-PYPU on: that unit bound
//! the placement vocabulary and authored the LINEAR twin
//! (`heat_sink_fins`, extrude-only), while this document's prototype is
//! a Revolve about its own axis whose meridian runs pole to pole —
//! the chart `die_pips`' retired deviation (b) existed to dodge.
//!
//! LIB-DIETOOL measured that half CLEARED, by construction. The recipe
//! below is unchanged — it has carried the natural meridian since the
//! day it was authored, `7581fb65d` having deleted the workaround one
//! commit earlier — and
//! `crates/pncad-py/tests/test_placed_union.py::TestTheDieTool` now
//! says the same nodes through the bound Python doors. The claim
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
//! pins is validity and the census.

use super::PlacePart;
use editor_core::{Coord, DocEdit, Formula, LoopProgram, Node, ProfileProgram, SlotId, TargetAxis};

use crate::fixture::{ang, len, revolve_about};

use super::die_pips::{DIE_L, PIP_H, PIP_R, half_disc_program};
use super::{CorpusDoc, Recorder};

/// The pip ball's centre coordinate along its face normal, in the
/// cube's `[0, L]³` frame (`die_pips`' derivation verbatim: the face
/// plane, plus the `R − H` the centre stands outside it so the cavity
/// is a cap of height exactly `H`).
const PIP_C: f64 = DIE_L + (PIP_R - PIP_H);

/// The six face-centre placements, in the tour's own order and with
/// the tour's own authored rotations, each as its bundle's values: one
/// turn about a coordinate axis of the cube's space, written as the
/// fraction of a turn it is, then the slides to the pip's centre. Each
/// carries the master ball's `+Z` pole onto the face normal it is cut
/// by, so every chart stays polar to the plane that cuts it.
fn placements() -> Vec<Vec<(Coord, Formula)>> {
    let h = DIE_L / 2.0;
    let lo = DIE_L - PIP_C; // the −normal faces' centre coordinate
    [
        (TargetAxis::Z, (0, 1), [h, h, PIP_C]),
        (TargetAxis::X, (1, 2), [h, h, lo]),
        (TargetAxis::Y, (1, 4), [PIP_C, h, h]),
        (TargetAxis::Y, (-1, 4), [lo, h, h]),
        (TargetAxis::X, (-1, 4), [h, PIP_C, h]),
        (TargetAxis::X, (1, 4), [h, lo, h]),
    ]
    .into_iter()
    .map(|(axis, (num, den), t)| {
        let turn = Formula::mul(
            Formula::ratio(num, den).expect("a fraction"),
            Formula::turn(),
        )
        .expect("an angle");
        let slides = [TargetAxis::X, TargetAxis::Y, TargetAxis::Z]
            .into_iter()
            .zip(t)
            .map(|(along, by)| (Coord::Slide(along), len(by)));
        slides
            .chain((num != 0).then_some((Coord::Spin(Some(axis)), turn)))
            .collect()
    })
    .collect()
}

/// The written-out-tool corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    // ---- the sharp cube, [0, L]³ ----
    let square = LoopProgram::polygon([(0.0, 0.0), (DIE_L, 0.0), (DIE_L, DIE_L), (0.0, DIE_L)])
        .expect("the die's square");
    let cube_p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![square],
        ids: Vec::new(),
        on: None,
    }));
    let cube = r.insert(Node::Extrude {
        profile: cube_p.into(),
        distance: len(DIE_L),
    });

    // ---- the master ball, poled along +Z (`die_pips`' construction) ----
    let ball_p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![half_disc_program()],
        ids: Vec::new(),
        on: None,
    }));
    // The pole axis is the meridian's own +y through (0, 0).
    let ball = r.insert(revolve_about(
        ball_p,
        (0.0, 0.0),
        crate::fixture::along_y(),
        ang(std::f64::consts::TAU),
    ));

    // ---- the master in the cube's space, its pole along +Z ----
    let own = r.corner_frame(cube, [0.0; 3], [0.0, 0.0, DIE_L], [DIE_L, 0.0, 0.0]);
    let master = r.on_axis(
        ball,
        own,
        editor_core::Formula::zero(editor_core::Dimension::Length),
        crate::fixture::meridian_up(),
    );

    // ---- the cutting tool: one placement of the master per pip ----
    let pips: Vec<editor_core::BodyRead<Formula>> = placements()
        .into_iter()
        .map(|values| editor_core::BodyRead::plain(r.copies(master, values)))
        .collect();
    let tool = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(pips),
    });
    let pipped = r.insert(Node::Subtract {
        from: cube.into(),
        tool: tool.into(),
    });

    r.place_part(pipped);

    CorpusDoc {
        name: "die_tool",
        about: "the die's multi-shell cutting tool: six pip placements, one union",
        edits: r.edits,
        doc: r.doc,
        result: Some(pipped),
        // π-valued spherical caps are not dyadic — see the module docs.
        pin: None,
        // D2's incremental probe: the cube's extrude distance, which
        // moves ONLY the +Z face (every pip's values are absolute). The
        // ball is reused; the cube, the master's placement (it reads
        // the cube), the pips, the tool and the cut recompute.
        // At 1.03125 the +Z pip still cuts a proper cap (the plane
        // sits 0.00875 below the ball's centre, leaving cap height
        // 0.08125 < 2R); every other pip is untouched.
        bump: DocEdit::SetParam {
            node: cube,
            slot: SlotId::Distance,
            value: len(1.03125).into(),
            fresh: Vec::new(),
        },
        bump_root: cube,
    }
}
