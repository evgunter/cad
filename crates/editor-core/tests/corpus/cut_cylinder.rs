//! Corpus document **cut_cylinder** — M5 PR 5's acceptance shape (i):
//! an extruded disc split by a TILTED datum plane, the first curved
//! cut in the corpus. The split's section edges carry the exact
//! `Ellipse` carrier (rung 2 of the C1 ladder), pinned by
//! `m5_pr5_corpus.rs`; joining the corpus registry gives the document
//! the standard rows for free — evaluation at every CI ε row and under
//! `interval`, persistence round-trip (D6.1), and the latency table.
//!
//! No mass pin: the tilted section is not dyadic, and the cut wall
//! pieces sit behind props' curved-quadrature frontier (M5 PR 11) —
//! validity + the ellipse pins are the oracles.

use editor_core::{Datum, DocEdit, LoopProgram, Node, ProfileProgram, SlotId};

use crate::fixture::{len, scl, xy_frame};

use super::{CorpusDoc, Recorder};

/// The tilt angle (radians) of the cutting plane's normal off the
/// cylinder axis — comfortably transverse, far from both the rim and
/// the ruling degeneracies.
pub const TILT: f64 = 0.3;

/// The cut-cylinder corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    // The disc profile: two half-circle arcs (bulge 1), radius 0.5 —
    // extrudes to a cylinder (two wall faces sharing one cylinder
    // surface, the cosurface run).
    // v4: the disc is the `circle` program form; its private two-pole
    // lowering IS the same two half-circle arcs (canonical form
    // bit-identical — lex-min starts the loop at the −x pole either
    // way; only the program-order rotation differs, which is naming
    // substrate, not geometry).
    let disc = LoopProgram::circle(0.0, 0.0, 0.5).unwrap();
    let plane = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![disc],
    }));
    let cylinder = r.insert(Node::Extrude {
        profile,
        distance: len(1.0),
    });

    // The tilted tool plane through mid-height: normal at TILT off the
    // axis, crossing both wall seams strictly between the caps.
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(0.5)],
        normal: [scl(TILT.sin()), scl(0.0), scl(TILT.cos())],
    }));
    let split = r.insert(Node::Split {
        target: cylinder,
        tool,
    });
    let _ = split;

    CorpusDoc {
        name: "cut_cylinder",
        about: "M5 shape (i): tilted plane×cylinder cut — exact Ellipse section edges",
        edits: r.edits,
        doc: r.doc,
        result: None, // the head is the Split's two sides, not one body
        pin: None,    // not dyadic; props' curved lane is PR 11
        bump: DocEdit::SetParam {
            node: tool,
            slot: SlotId::Origin(editor_core::Axis3::Z),
            expr: len(0.4375),
        },
        bump_root: tool,
    }
}
