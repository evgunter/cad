//! Corpus document **boss_union** — M5 PR 9's acceptance shape (ii):
//! the first transverse CURVED BOOLEAN in the corpus. A three-arc
//! cylinder boss pierces a plate's top face; the union's seam is the
//! rim circle, minted as exact `Circle` arcs on both operands and
//! described intrinsically. Joining the corpus registry gives the
//! document the standard rows for free — evaluation at every CI ε row
//! and under `interval`, persistence round-trip (D6.1), and the
//! latency table (the deviation-9 rows, landed at the fix pass).
//!
//! No dyadic mass pin (the boss volume carries π); the closed-form
//! volume and the interval bracket are pinned by
//! `review_m5_pr9_doc_probe.rs`, and validity + the seam-arc counts
//! are pinned by the boolean's own acceptance suites.

use editor_core::{BooleanOp, DocEdit, LoopProgram, Node, ProfileProgram, SlotId};

use super::super::fixture::{frame, len, xy_frame};
use super::{CorpusDoc, Recorder};

/// The boss-union corpus document: 3×3×0.8 plate ∪ r = 0.35 three-arc
/// boss at (1.2, 1.7), sketched at z = 0.3, extruded 1.0 (pokes 0.5
/// out of the plate).
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    let plate_loop =
        LoopProgram::polygon([(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)]).unwrap();
    let plate_plane = r.insert(xy_frame());
    let plate_p = r.insert(Node::Profile(ProfileProgram {
        plane: plate_plane,
        loops: vec![plate_loop],
    }));
    let plate = r.insert(Node::Extrude {
        profile: plate_p,
        distance: len(0.8),
    });

    // v4 (LIB-SWITCH corpus ruling (a)): the three-arc boss authors as
    // the declared-subdivision closed carrier — same carrier, same 3
    // structural seams (the rim-seam-count assertions downstream keep
    // their meaning), vertex positions now the primitive's own libm
    // lowering rather than hand-transcribed cos/sin (a NUMBERED
    // deviation: the boss's export bits shift; geometry is the same
    // circle).
    let boss_loop = LoopProgram::circle_split(1.2, 1.7, 0.35, 3, 0.0).unwrap();
    let boss_plane = r.insert(frame(
        [0.0, 0.0, 0.3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    ));
    let boss_p = r.insert(Node::Profile(ProfileProgram {
        plane: boss_plane,
        loops: vec![boss_loop],
    }));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(1.0),
    });

    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: plate,
        b: boss,
        declare: None,
    });

    CorpusDoc {
        name: "boss_union",
        about: "M5 shape (ii): cylinder boss ∪ plate — the first transverse curved boolean",
        edits: r.edits,
        doc: r.doc,
        result: Some(union),
        pin: None, // π volume is not dyadic; the doc probe pins the closed form
        // D2's incremental probe: grow the boss — the plate chain is
        // reused, the boss extrude + union recompute.
        bump: DocEdit::SetParam {
            node: boss,
            slot: SlotId::Distance,
            expr: len(1.125),
        },
        bump_root: boss,
    }
}
