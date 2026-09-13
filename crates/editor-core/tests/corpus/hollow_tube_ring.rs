//! Corpus document **hollow_tube_ring** — a CLOSED hollow ring torus
//! authored through `Node::HollowTube`, and the one corner of this
//! vocabulary's two-by-two that produces a topology no other corner
//! can.
//!
//! `tube_arc`'s mirror. That document holds the KIND fixed and moves
//! the window; this one holds the WINDOW fixed and moves the kind, so
//! between them the registry now varies each axis with the other held
//! — which is what `tube_ring` and `hollow_tube_elbow`, opposite
//! corners of the same square, could never do alone.
//!
//! # The cavity, which is why this corner is not a formality
//!
//! An arc's hollow tube is capped at both ends by ANNULI, so its
//! inner wall is reachable from outside and the body is ONE shell.
//! Close the window and the annuli go away: the inner wall closes on
//! itself into a second torus, and the body becomes TWO shells in one
//! solid — an outer boundary and a sealed VOID. That is the same
//! topology `hollowring`'s revolve produces and the same one `shell`'s
//! sealed arm produces, arrived at from a third door, and no other
//! corner of this square reaches it: `tube_ring` is one shell,
//! `tube_arc` is one shell, `hollow_tube_elbow` is one shell.
//!
//! The bump edits the WALL, as `hollow_tube_elbow`'s does — the slot
//! only a hollow tube has. It moves the inner torus's stored minor
//! radius and nothing else, so the cavity stays a cavity and the
//! covariance row measures a rebuild rather than a re-authoring.
//!
//! # No mass pin
//!
//! Pappus on the annulus gives `V = 2π²R(r_o² − r_i²)`, which carries
//! π², and the corpus `MassPin` is asserted with `==` — so a pin here
//! would fix `f64` rounding of an irrational rather than the geometry.
//! The `die_chamfer` disposition, and every sibling's in this family.

use editor_core::{Datum, DocEdit, Node, SlotId, TubeWindow};

use super::super::fixture::len;
use super::{CorpusDoc, Recorder};

/// The spine circle's radius, meters (dyadic).
pub const R: f64 = 2.0;
/// The OUTER cross-sectional radius, meters (dyadic).
pub const OUTER: f64 = 0.5;
/// The wall thickness, meters (dyadic, so `OUTER - WALL` is exact).
pub const WALL: f64 = 0.125;
/// The bumped wall (dyadic; still clears the bore, so the cavity
/// survives the edit — a wall that ate the bore would change the
/// topology and the covariance row would be measuring a different
/// body).
pub const WALL_BUMPED: f64 = 0.1875;

/// The inner torus's stored minor radius: the caller's own
/// subtraction, which is what the body holds and what a caller
/// recovers by writing the same line.
pub fn inner(wall: f64) -> f64 {
    OUTER - wall
}

/// The closed-hollow-ring corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();

    let spine = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scalar(0.0), scalar(0.0), scalar(1.0)],
    }));
    let ring = r.insert(Node::HollowTube {
        spine,
        u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
        major_radius: len(R),
        window: TubeWindow::Full,
        minor_radius: len(OUTER),
        wall: len(WALL),
    });

    CorpusDoc {
        name: "hollow_tube_ring",
        about: "a sealed hollow ring torus — two shells, one solid — R = 2, outer 0.5, wall 0.125",
        edits: r.edits,
        doc: r.doc,
        result: Some(ring),
        // Pappus carries π² — module docs.
        pin: None,
        bump: DocEdit::SetParam {
            node: ring,
            slot: SlotId::TubeWall,
            expr: len(WALL_BUMPED),
        },
        bump_root: ring,
    }
}

/// A dimensionless component, the spelling a direction takes.
fn scalar(v: f64) -> editor_core::Expr {
    editor_core::Expr::literal(v, editor_core::Dimension::Scalar).expect("finite")
}
