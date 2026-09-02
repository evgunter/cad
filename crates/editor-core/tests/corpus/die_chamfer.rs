//! Corpus document **die_chamfer** — `die_fillet`'s twin, one verb
//! over: a unit cube with every one of its twelve edges CHAMFERED at
//! equal setback, so the corpus carries a body whose boundary is
//! planes and nothing else, produced by the same composition surgery
//! that produces the fillet's cylinders and spheres.
//!
//! The recipe is three nodes — profile → extrude → chamfer — and it is
//! `die_fillet`'s recipe with one node kind changed, deliberately: the
//! two documents differ in the blend and in nothing else, so a
//! difference in their behaviour is a difference the blend accounts
//! for.
//!
//! The chamfer's selection is every edge of the cube, AUTHORED. The
//! bump edits the EXTRUDE's distance, which mints and retires no edge,
//! so the frozen selection still resolves — the same covariance row
//! `die_fillet` runs, over a node whose emitter is new.
//!
//! # No mass pin
//!
//! The closed forms below (`lib_g16_chamfer_node.rs` meters them)
//! carry √2 and √3, and the corpus's [`MassPin`](super::MassPin) is
//! asserted with `==` against an EXACT oracle. A pin here would pin
//! `f64` rounding of an irrational, not the geometry — the
//! `die_fillet` disposition, for the same reason.

use editor_core::{DocEdit, LoopProgram, Node, ProfileProgram, SlotId};

use super::super::fixture::{len, prism_edges, xy_frame};
use super::{CorpusDoc, Recorder};

/// The blank's side, meters (dyadic).
pub const L: f64 = 1.0;
/// The chamfer setback along each support, meters (dyadic, and
/// comfortably under `L/2`).
pub const D: f64 = 0.125;
/// The bumped extrude distance, meters (dyadic; still a box every edge
/// of which admits a `D` setback).
pub const L_BUMPED: f64 = 1.25;

/// The chamfered-blank corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    let square = LoopProgram::polygon([(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]).unwrap();
    let plane = r.insert(xy_frame());
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![square],
    }));
    let cube = r.insert(Node::Extrude {
        profile,
        distance: len(L),
    });
    let blank = r.insert(Node::chamfer(cube, len(D), prism_edges(cube, 4)));

    CorpusDoc {
        name: "die_chamfer",
        about: "every edge of a unit cube chamfered at a setback of 0.125",
        edits: r.edits,
        doc: r.doc,
        result: Some(blank),
        // √2- and √3-valued closed forms are not dyadic — module docs.
        pin: None,
        bump: DocEdit::SetParam {
            node: cube,
            slot: SlotId::Distance,
            expr: len(L_BUMPED),
        },
        bump_root: cube,
    }
}
