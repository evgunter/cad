//! Corpus document **loft_prism** — R5 shape (iii)'s loft body
//! (M6-3): three POLYLINE quad sections (squares at z = 0 and z = 2,
//! a trapezoid at z = 1), skinned at v-degree 2 and assembled by
//! `sweep::loft_body`. The middle section is NOT an affine image of
//! the squares (affine maps preserve parallelism; the trapezoid has a
//! non-parallel pair), so the walls are genuinely curved — the R5
//! text's "≥3 sections, ≥1 non-affine pair" without arcs (arc-bearing
//! profiles skin to RATIONAL walls, whose flux lane is banked — the
//! honest partial, M6-3 spec §3).
//!
//! # The volume, derived (not measured)
//!
//! The degree-2 skin through sections at parameters (0, ½, 1) is the
//! quadratic Lagrange interpolant, so with the end sections EQUAL the
//! corner paths are `P(v) = S + λ(v)·D`, `λ = 4v(1−v)`, `D = T − S`.
//! The z-channel interpolates (0, 1, 2) to `z = 2v` exactly (the
//! quadratic terms cancel), and each fixed-v slice is a straight-edged
//! quad (the sections are polylines, and the skin is linear in u per
//! segment). With `D = [(−d, 0), (+d, 0), 0, 0]`, `d = 0.375`, the
//! slice is a trapezoid of area `4 + 2dλ`, so
//!
//! ```text
//! V = ∫₀² A dz = 2·∫₀¹ (4 + 2d·4v(1−v)) dv
//!   = 2·(4 + 8d·∫₀¹ v(1−v) dv) = 2·(4 + 8d/6) = 8 + 8d/3
//!   = 8 + 1 = 9 m³ exactly (d = 0.375).
//! ```
//!
//! (Check: ∫₀¹ λ dv = 4(1/2 − 1/3) = 2/3, so V = 2(4 + 2d·2/3)
//! = 8 + 8d/3 — with d = 0.375: 8·0.375/3 = 1. V = 9.)
//!
//! The corpus `pin` stays `None` — pins assert EXACT f64 equality and
//! a quadrature midpoint is an enclosure, not a closed form. The
//! bracket pin `9 ∈ volume ± pad` lives with the builder's acceptance
//! suite (`sweep/tests/m6_loft_body.rs`), where the derivation above
//! is asserted against the certified enclosure.

use editor_core::{DocEdit, Expr, LoopProgram, Node, ProfileProgram, RecipeNodeId, SlotId};

use super::super::fixture::frame;
use super::{CorpusDoc, Recorder};

/// One quad section at height `z` from its four corners: the frame
/// the section is drawn on, then the profile that names it.
///
/// The three sections sit at three DIFFERENT heights, so each gets its
/// own frame — a shared one would be a shared plane, which is exactly
/// what a loft's sections are not.
fn section(r: &mut Recorder, z: f64, pts: [(f64, f64); 4]) -> RecipeNodeId {
    let plane = r.insert(frame([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::polygon(pts).unwrap()],
    }))
}

/// The loft-prism corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let bottom = section(&mut r, 0.0, square);
    let middle = section(&mut r, 1.0, trapezoid);
    let top = section(&mut r, 2.0, square);
    let loft = r.insert(Node::Loft {
        profiles: vec![bottom, middle, top],
        v_degree: Expr::count(2),
    });

    CorpusDoc {
        name: "loft_prism",
        about: "R5 shape (iii): three-section polyline loft, non-affine middle (M6-3)",
        edits: r.edits,
        doc: r.doc,
        result: Some(loft),
        // Pins assert exact f64 equality; a quadrature enclosure's
        // midpoint is not a closed form. The derived V = 9 m³ bracket
        // pin lives in sweep/tests/m6_loft_body.rs (module docs).
        pin: None,
        // The incremental probe: re-skin at v-degree 1 — the three
        // profile nodes are reused, the loft cone recomputes.
        bump: DocEdit::SetStructuralParam {
            node: loft,
            slot: SlotId::VDegree,
            expr: Expr::count(1),
        },
        bump_root: loft,
    }
}
