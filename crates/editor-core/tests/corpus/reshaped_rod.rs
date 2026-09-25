//! Corpus document **reshaped_rod** — the one registered document
//! whose log holds a `DocEdit::SetProgram`: a rod's section standing on
//! a block's top edge (`edit_ruled_carve`'s sunk rod), one of its two
//! cylinder-meets-plane creases filleted, and THEN the block's program
//! reshaped under the fillet — a bump inserted into the right wall,
//! two steps drawing one segment — keeping every step it had, so the
//! crease's name, spelled by the piece its step draws, still denotes
//! the crease.
//!
//! The reshaping is the document's reason to exist: every other corpus
//! program is the one its profile was inserted with, so before this
//! document no persisted log carried a program replaced whole, and no
//! name digest was minted over a selection a reshaping kept. Here the
//! registry battery — the persistence round trip, the replay identity,
//! the interval lane, the latency table's structural row, the
//! name-digest goldens — runs over exactly that, so the kept step is a
//! persisted and replayed fact rather than a suite's.
//!
//! Vocabulary: Profile, Extrude, Fillet, `InsertNode`, `SetProgram`,
//! `SetParam` (the bump).
//!
//! # No mass pin
//!
//! The fillet's band is a quarter of a torus-free cylinder blend — a
//! π-valued form — so the pin is `None`, `die_fillet`'s reason. What
//! is metered instead lives in `edit_set_program`: the reshaped rod's
//! volume is the plain rod's plus exactly the bump's prism, and the
//! held strut stands where it did.
//!
//! D2 bump: the extrude's `Distance` (mid-DAG — its cone is the extrude
//! and the fillet; the frame and the profile are reused).

use editor_core::{
    DocEdit, LoopProgram, Node, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, RecipeNodeId, RoleSeg, SlotId, StableName, StepId,
};
use sweep::test_support::{ROD_FILLET, ROD_FLAT, ROD_L, rod_chord_at};

use crate::fixture::{ename, len, scl, xy_frame};

use super::{CorpusDoc, Recorder};

/// Where the inserted leg lands: a bump on the block's right wall.
pub const BUMP: (f64, f64) = (1.25, -0.5);

/// The rod's extrusion depth after the bump edit (dyadic).
pub const L_BUMPED: f64 = 1.5;

/// The crease the fillet is on, as a canonical vertex of the PLAIN
/// program: the arc's end, where the rod's section meets the block's
/// top on the left — the start of the leg to `(-1, 0)`.
pub const CREASE: usize = 4;

/// The same crease after the reshaping: one canonical vertex along, the
/// bump's one segment having been inserted before it. Its NAME does
/// not move: it spells the leg's step, which the reshaping keeps.
pub const CREASE_RESHAPED: usize = 5;

/// The sunk rod's loop, with or without the bump. The bump is
/// authored as a direction and a length — two steps, one segment — so
/// the step indices after it move by two while the segment indices
/// move by one; `edit_set_program` says why that asymmetry is kept.
pub fn rod_loop(bump: bool) -> LoopProgram {
    let c = rod_chord_at(ROD_FLAT);
    let xv = c.half;
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let mut steps = vec![
        ProgramStep::At(pt(-1.0, -1.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, -1.0))),
    ];
    if bump {
        let (dx, dy) = (BUMP.0 - 1.0, BUMP.1 + 1.0);
        steps.push(ProgramStep::Toward {
            dx: scl(dx),
            dy: scl(dy),
        });
        steps.push(ProgramStep::Line(len(dx.hypot(dy))));
    }
    steps.extend([
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(xv, 0.0))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(pt(-xv, 0.0)),
            b: scl(c.section_bulge),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(pt(-1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    LoopProgram::Chain(steps)
}

/// The step ids of the bumped loop over the plain one's (`old`, the
/// seven the plain program was minted): every step keeps its own id
/// except the two the bump inserted, which the door mints.
pub fn bump_ids(old: &[StepId]) -> Vec<Vec<Option<StepId>>> {
    let mut ids: Vec<Option<StepId>> = old.iter().copied().map(Some).collect();
    ids.splice(2..2, [None, None]);
    vec![ids]
}

/// The plain rod's step ids, as `doc` holds them for `profile`.
pub fn rod_ids(doc: &ProfileDoc, profile: RecipeNodeId) -> Vec<StepId> {
    match doc.node(profile) {
        Some(Node::Profile(p)) => p.ids[0].clone(),
        other => panic!("node {} is the rod's profile: {other:?}", profile.0),
    }
}

/// The strut edge at canonical vertex `vertex` of the rod, spelled by
/// the piece starting there.
pub fn lateral_edge(doc: &ProfileDoc, rod: RecipeNodeId, vertex: usize) -> StableName {
    ename(
        rod,
        RoleSeg::LateralEdge(crate::fixture::vpiece(doc, rod, 0, vertex)),
    )
}

/// The reshaped-rod corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let plane = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![rod_loop(false)],
        ids: Vec::new(),
    }));
    let rod = r.insert(Node::Extrude {
        profile,
        distance: len(ROD_L),
    });
    // The fillet is authored against the PLAIN program's crease, and
    // the reshaping below keeps the step whose piece the name spells,
    // so the name the recipe records is the name it keeps.
    let crease = lateral_edge(&r.doc, rod, CREASE);
    let fillet = r.insert(Node::fillet(rod, len(ROD_FILLET), vec![crease]));
    let ids = bump_ids(&rod_ids(&r.doc, profile));
    r.push(DocEdit::SetProgram {
        node: profile,
        loops: vec![rod_loop(true)],
        ids,
    });

    CorpusDoc {
        name: "reshaped_rod",
        about: "a rod's section on a block, one crease filleted, the block reshaped under it",
        edits: r.edits,
        doc: r.doc,
        result: Some(fillet),
        // π-valued (a cylinder blend) — see the module docs.
        pin: None,
        bump: DocEdit::SetParam {
            node: rod,
            slot: SlotId::Distance,
            expr: len(L_BUMPED),
        },
        bump_root: rod,
    }
}
