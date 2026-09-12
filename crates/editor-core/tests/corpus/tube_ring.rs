//! Corpus document **tube_ring** — a solid ring torus authored through
//! `Node::Tube`, the smallest document that exercises the solid door.
//!
//! Two nodes: a datum axis and the tube. That IS the recipe — a tube
//! takes no profile, because its cross-section is the door's own
//! intent parameters rather than a sketch, which is the whole reason
//! the door exists.
//!
//! The bump edits the MAJOR radius, which moves the body without
//! changing its topology (still one closed torus shell, still four
//! band faces), so the covariance row measures a rebuild rather than a
//! re-authoring.
//!
//! # No mass pin
//!
//! `V = 2π²Rr²` carries π², and the corpus `MassPin` is asserted with
//! `==`, so a pin here would fix `f64` rounding of an irrational
//! rather than the geometry — the `die_fillet` / `die_chamfer`
//! disposition, and the sibling `hollow_tube_elbow`'s. This document
//! therefore carries `pin: None`, and the closed forms are metered at
//! a stated relative tolerance in `lib_tube_node.rs`.
//!
//! What IS exact about this document is not a mass at all: the STORED
//! MINOR RADIUS, the number this door exists to keep. `MINOR` is
//! dyadic and comes back out of the body bit for bit — metered where
//! it can be, by `lib_tube_node.rs`'s stored-bits rows reading
//! `Surface::Torus`, not by anything in this file.

use editor_core::{Datum, DocEdit, Node, SlotId, TubeWindow};

use super::super::fixture::len;
use super::{CorpusDoc, Recorder};

/// The spine circle's radius, meters (dyadic).
pub const R: f64 = 2.0;
/// The tube's own cross-sectional radius, meters (dyadic, and well
/// under `R` — the ring-torus convention is `R > r > 0`).
pub const MINOR: f64 = 0.5;
/// The bumped major radius (dyadic; still a ring torus).
pub const R_BUMPED: f64 = 2.5;

/// The solid-ring-torus corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    let spine = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scalar(0.0), scalar(0.0), scalar(1.0)],
    }));
    let ring = r.insert(Node::Tube {
        spine,
        u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
        major_radius: len(R),
        window: TubeWindow::Full,
        minor_radius: len(MINOR),
    });

    CorpusDoc {
        name: "tube_ring",
        about: "a solid ring torus from its intent parameters, R = 2, r = 0.5",
        edits: r.edits,
        doc: r.doc,
        result: Some(ring),
        // 2π²Rr² is not dyadic — module docs.
        pin: None,
        bump: DocEdit::SetParam {
            node: ring,
            slot: SlotId::TubeMajorRadius,
            expr: len(R_BUMPED),
        },
        bump_root: ring,
    }
}

/// A dimensionless component, the spelling a direction takes.
fn scalar(v: f64) -> editor_core::Expr {
    editor_core::Expr::literal(v, editor_core::Dimension::Scalar).expect("finite")
}
