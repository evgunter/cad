//! FIX — the polygon builder at expression corners
//! (`no-parametric-loop-constructor`).
//!
//! `LoopProgram::polygon_expr` is the ONE polygon expansion:
//! `At(p0)`, `LineTo(p1)`, …, `LineTo(Start)`. `LoopProgram::polygon`
//! is that door at literal corners, so a document authored from
//! literals and a document authored from the same numbers as `Expr`s
//! are the same program — which is what lets a parametric author
//! reach the builder instead of respelling the expansion, and what
//! keeps a second expansion from existing to drift.
//!
//! The rows pin both halves of that: the agreement of the two
//! spellings, and the expansion's shape at corners no literal door
//! can take (a document parameter).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{Dimension, Expr, LoopProgram, ParamName, ProgramStep, ProgramTarget};

/// A Length literal.
fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a finite length literal")
}

const CORNERS: [(f64, f64); 4] = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (0.0, 2.0)];

/// The literal door and the expression door agree, program for
/// program: `polygon` is `polygon_expr` at literal corners.
#[test]
fn literal_polygon_is_the_expr_polygon_at_literal_corners() {
    let literal = LoopProgram::polygon(CORNERS).expect("finite corners");
    let lifted = LoopProgram::polygon_expr(CORNERS.map(|(x, y)| [len(x), len(y)]));
    assert_eq!(literal, lifted);
}

/// The expansion at corners the literal door cannot express: a
/// document parameter rides through untouched, in the same
/// `At` … `LineTo(Start)` shape.
#[test]
fn parametric_corners_expand_to_the_same_shape() {
    let w = Expr::param(ParamName::new("w"), Dimension::Length);
    let zero = len(0.0);
    let program = LoopProgram::polygon_expr([
        [zero.clone(), zero.clone()],
        [w.clone(), zero.clone()],
        [w.clone(), len(1.0)],
        [zero.clone(), len(1.0)],
    ]);
    assert_eq!(
        program,
        LoopProgram::Chain(vec![
            ProgramStep::At([zero.clone(), zero.clone()]),
            ProgramStep::LineTo(ProgramTarget::Point([w.clone(), zero.clone()])),
            ProgramStep::LineTo(ProgramTarget::Point([w, len(1.0)])),
            ProgramStep::LineTo(ProgramTarget::Point([zero, len(1.0)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])
    );
}
