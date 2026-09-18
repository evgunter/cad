//! Corpus document **tube_arc** — a WINDOWED solid tube: an open bend
//! of circular section, authored through `Node::Tube`.
//!
//! The third corner of the two-by-two `tube_ring` and
//! `hollow_tube_elbow` opened on the DIAGONAL. Those two differ in
//! both axes at once — solid/full against hollow/windowed — which
//! covers each vocabulary term but never separates them: nothing in
//! the corpus could tell "the solid door works" from "the full window
//! works". This document holds the KIND fixed and moves the WINDOW,
//! and `hollow_tube_ring` beside it does the mirror, so each axis is
//! now varied with the other held.
//!
//! What the window buys that `tube_ring` cannot have: two END CAPS.
//! A full ring torus is one closed shell of band faces and no cap at
//! all; an arc's tube is capped at both ends by discs standing in the
//! window's own two half-planes. So this is also the first solid tube
//! in the corpus whose face census is not all bands.
//!
//! The bump edits the window's END ANGLE — the slot `tube_ring` cannot
//! have, exactly as `hollow_tube_elbow`'s bump edits the wall slot
//! `tube_ring` cannot have. It sweeps the same section further around
//! the same spine, so the topology holds and the covariance row
//! measures a rebuild.
//!
//! # No mass pin
//!
//! Pappus on the disc gives `V = θ·R·πr²`, which carries π, and the
//! corpus `MassPin` is asserted with `==` — so a pin here would fix
//! `f64` rounding of an irrational rather than the geometry. The
//! `die_chamfer` disposition, and both siblings'.

use editor_core::{Datum, DocEdit, Node, SlotId, TubeWindow};

use super::super::fixture::len;
use super::{CorpusDoc, Recorder};

/// The spine circle's radius, meters (dyadic).
pub const R: f64 = 2.0;
/// The tube's own cross-sectional radius, meters (dyadic, and well
/// under `R`).
pub const MINOR: f64 = 0.5;
/// The window's start angle, radians.
pub const T0: f64 = 0.0;
/// The window's end angle, radians (dyadic, and well under one period
/// — an exactly full tube must say `TubeWindow::Full`).
pub const T1: f64 = 1.0;
/// The bumped end angle (dyadic; still short of a period, so the body
/// stays a capped bend rather than closing on itself).
pub const T1_BUMPED: f64 = 2.0;

/// The windowed-solid-tube corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    // A different spine axis from `tube_ring`'s deliberately: that
    // document turns about +z and this one about +y, so a lowering
    // that assumed one world axis has two documents to disagree with.
    let spine = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scalar(0.0), scalar(1.0), scalar(0.0)],
    }));
    let bend = r.insert(Node::Tube {
        spine,
        u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
        major_radius: len(R),
        window: TubeWindow::Arc {
            t0: angle(T0),
            t1: angle(T1),
        },
        minor_radius: len(MINOR),
    });

    CorpusDoc {
        name: "tube_arc",
        about: "a capped solid bend from its intent parameters, R = 2, r = 0.5, 1 rad",
        edits: r.edits,
        doc: r.doc,
        result: Some(bend),
        // Pappus carries π — module docs.
        pin: None,
        bump: DocEdit::SetParam {
            node: bend,
            slot: SlotId::TubeWindowEnd,
            expr: angle(T1_BUMPED),
        },
        bump_root: bend,
    }
}

/// A dimensionless component, the spelling a direction takes.
fn scalar(v: f64) -> editor_core::Expr {
    editor_core::Expr::literal(v, editor_core::Dimension::Scalar).expect("finite")
}

/// An angle in radians, the spelling a window bound takes.
fn angle(v: f64) -> editor_core::Expr {
    editor_core::Expr::literal(v, editor_core::Dimension::Angle).expect("finite")
}
