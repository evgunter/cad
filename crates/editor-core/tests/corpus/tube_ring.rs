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
//! # The mass pin is EXACT, and that is not luck
//!
//! `V = 2π²Rr²` carries π², so the pin cannot be the volume. It is the
//! STORED MINOR RADIUS instead — the number this door exists to keep —
//! and that one is dyadic and reproduces bit for bit. The corpus's
//! `MassPin` is asserted with `==`, so anything irrational belongs in
//! the suite's relative-tolerance rows, not here (the `die_fillet` /
//! `die_chamfer` disposition, one artifact over).

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
