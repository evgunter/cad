//! Corpus document **hollow_tube_elbow** — a windowed hollow tube: an
//! open elbow of annular section, authored through `Node::HollowTube`.
//!
//! `tube_ring`'s sibling, and deliberately different in BOTH the ways
//! this unit's vocabulary can differ: the node kind (hollow, so the
//! wall slot exists) and the window (an arc, so the two window-angle
//! slots exist). A corpus that carried only the full solid ring would
//! cover one corner of a two-by-two and read as covering "tube".
//!
//! The bump edits the WALL, which is the slot `tube_ring` cannot have.
//! It moves the inner half-walls' stored radius and nothing else — the
//! outer radius, the spine and the window all hold — so the covariance
//! row measures exactly the parameter the split vocabulary exists for.
//!
//! # No mass pin
//!
//! The Pappus forms below carry π (`lib_tube_node.rs` meters them at a
//! stated relative tolerance). The corpus `MassPin` is asserted with
//! `==`, so it would pin `f64` rounding of an irrational rather than
//! the geometry — the `die_chamfer` disposition.

use editor_core::{Datum, DocEdit, Node, SlotId, TubeWindow};

use super::super::fixture::len;
use super::{CorpusDoc, Recorder};

/// The spine circle's radius, meters (dyadic).
pub const R: f64 = 2.0;
/// The OUTER cross-sectional radius, meters (dyadic).
pub const OUTER: f64 = 0.5;
/// The wall thickness, meters (dyadic, so `OUTER - WALL` is exact).
pub const WALL: f64 = 0.125;
/// The bumped wall (dyadic; still clears the bore).
pub const WALL_BUMPED: f64 = 0.25;
/// The window's start angle, radians.
pub const T0: f64 = 0.0;
/// The window's end angle, radians (dyadic, and well under one
/// period — an exactly full tube must say `TubeWindow::Full`).
pub const T1: f64 = 1.5;

/// The inner half-walls' stored radius: the caller's own subtraction,
/// which is what the body holds and what a caller recovers by writing
/// the same line.
pub fn inner(wall: f64) -> f64 {
    OUTER - wall
}

/// The hollow-elbow corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    let spine = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scalar(0.0), scalar(1.0), scalar(0.0)],
    }));
    let elbow = r.insert(Node::HollowTube {
        spine,
        u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
        major_radius: len(R),
        window: TubeWindow::Arc {
            t0: angle(T0),
            t1: angle(T1),
        },
        minor_radius: len(OUTER),
        wall: len(WALL),
    });

    CorpusDoc {
        name: "hollow_tube_elbow",
        about: "an open elbow of annular section, R = 2, outer 0.5, wall 0.125",
        edits: r.edits,
        doc: r.doc,
        result: Some(elbow),
        // The Pappus forms carry π — module docs.
        pin: None,
        bump: DocEdit::SetParam {
            node: elbow,
            slot: SlotId::TubeWall,
            expr: len(WALL_BUMPED),
        },
        bump_root: elbow,
    }
}

fn scalar(v: f64) -> editor_core::Expr {
    editor_core::Expr::literal(v, editor_core::Dimension::Scalar).expect("finite")
}

fn angle(v: f64) -> editor_core::Expr {
    editor_core::Expr::literal(v, editor_core::Dimension::Angle).expect("finite")
}
