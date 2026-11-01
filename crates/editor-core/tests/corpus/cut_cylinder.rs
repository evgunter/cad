//! Corpus document **cut_cylinder** — M5 PR 5's acceptance shape (i):
//! an extruded disc split by a TILTED plane, the first curved
//! cut in the corpus. The split's section edges carry the exact
//! `Ellipse` carrier (rung 2 of the C1 ladder), pinned by
//! `m5_pr5_corpus.rs`; joining the corpus registry gives the document
//! the standard rows for free — evaluation at every CI ε row and under
//! `interval`, persistence round-trip (D6.1), and the latency table.
//!
//! No mass pin: the tilted section is not dyadic, and the cut wall
//! pieces sit behind props' curved-quadrature frontier (M5 PR 11) —
//! validity + the ellipse pins are the oracles.

use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    Dimension, DocEdit, Formula, FreeVar, LoopProgram, Node, Operand, ProfileProgram, VarName,
};
use geom_core::Tol;

use crate::fixture::{ang, len};

use super::{CorpusDoc, Recorder};

/// The tilt angle (radians) of the cutting plane's normal off the
/// cylinder axis — comfortably transverse, far from both the rim and
/// the ruling degeneracies.
pub const TILT: f64 = 0.3;

/// The document parameter the tool plane's height reads.
pub const CUT: &str = "cut";

/// The cut-cylinder corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(CUT),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.5)),
    });

    // The disc profile: two half-circle arcs (bulge 1), radius 0.5 —
    // extrudes to a cylinder (two wall faces sharing one cylinder
    // surface, the cosurface run).
    // v4: the disc is the `circle` program form; its private two-pole
    // lowering IS the same two half-circle arcs (canonical form
    // bit-identical — the lowering starts the loop at the same pole
    // with the same two arcs).
    let disc = LoopProgram::circle(0.0, 0.0, 0.5).unwrap();
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![disc],
        ids: Vec::new(),
        on: None,
    }));
    let cylinder = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(1.0),
    });

    // The tilted tool plane through mid-height: the cylinder's own
    // frame read off its geometry, slid `cut` up its z and spun TILT
    // about its y, read through its xy — normal at TILT off the axis,
    // crossing both wall seams strictly between the caps.
    let own = editor_core::test_support::frame_on(
        &r.doc,
        &Operand::from(cylinder),
        &editor_core::Frame::IDENTITY,
        Tol::witness(),
    );
    let pose = |def| Operand::Pose(Box::new(def));
    let slid = pose(PoseDef::Slide {
        pose: RoleRead::plain(own.clone()),
        along: RoleRead::through(own, PoseRole::Z),
        by: Formula::named(VarName::from_static(CUT), Dimension::Length),
    });
    let tilted = pose(PoseDef::Spin {
        pose: RoleRead::plain(slid.clone()),
        about: RoleRead::through(slid, PoseRole::Y),
        by: ang(TILT),
    });
    let split = r.insert(Node::Split {
        target: cylinder.into(),
        tool: RoleRead::through(tilted, PoseRole::Xy),
    });
    super::place_halves(&mut r, split);

    CorpusDoc {
        name: "cut_cylinder",
        about: "M5 shape (i): tilted plane×cylinder cut — exact Ellipse section edges",
        edits: r.edits,
        doc: r.doc,
        result: None, // the head is the Split's two sides, not one body
        pin: None,    // not dyadic; props' curved lane is PR 11
        bump: DocEdit::SetVarValue {
            var: VarName::from_static(CUT).into(),
            value: editor_core::FreeValue::Continuous(0.4375),
        },
        bump_root: split,
    }
}
