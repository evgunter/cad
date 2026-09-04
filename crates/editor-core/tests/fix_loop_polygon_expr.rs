//! FIX — the polygon builder at expression corners
//! (`no-parametric-loop-constructor`).
//!
//! `LoopProgram::polygon_expr` is the polygon expansion — `At(p0)`,
//! `LineTo(p1)`, …, `LineTo(Start)` — and `LoopProgram::polygon` is
//! that door at literal corners, so a document authored from literals
//! and one authored from the same numbers as `Expr`s are the same
//! program.
//!
//! **What the agreement rows can and cannot show.** They pin that the
//! two doors AGREE, which catches DRIFT between two expansions. They
//! cannot catch the EXISTENCE of a second expansion: any correct
//! duplicate satisfies `polygon(c) == polygon_expr(lift(c))` by
//! definition, so a copy inlined back into `polygon` passes them. Nor
//! does agreement at one arity say anything about another — a
//! duplicate that closed the loop only at two corners or more would
//! agree on every four-corner row here and diverge at zero and one.
//! Both gaps were measured by planting exactly those duplicates, and
//! both plants went green against agreement rows alone.
//!
//! So the rows below are three claims, not one:
//!
//! - the two doors agree, at four corners AND at each degenerate
//!   arity, which is the drift pin at the widths it actually covers;
//! - the expansion keeps its shape at corners no literal door can
//!   take, which is the authoring gap this unit closed;
//! - the polygon close is written ONCE in shipped `src`, which is the
//!   existence claim, and the only row here that can detect a second
//!   expansion arriving.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

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

/// The arities a four-corner row cannot speak for.
///
/// The builder is TOTAL: too few points is not refused here, because
/// the edit door's replay probe refuses a degenerate loop typed, at
/// `insert` — one rule for authored and hand-written programs alike.
/// These rows pin that both doors are total in the same way, so a
/// duplicate that closed the loop only above some arity would red.
#[test]
fn both_doors_agree_at_the_degenerate_arities() {
    // No corners at all: the close, and nothing to close.
    let empty: [(f64, f64); 0] = [];
    let literal = LoopProgram::polygon(empty).expect("no corners is not a refusal");
    assert_eq!(
        literal,
        LoopProgram::Chain(vec![ProgramStep::LineTo(ProgramTarget::Start)])
    );
    assert_eq!(literal, LoopProgram::polygon_expr([]));

    // One corner: the anchor, then the close.
    let literal = LoopProgram::polygon([(1.0, 2.0)]).expect("one corner is not a refusal");
    assert_eq!(
        literal,
        LoopProgram::Chain(vec![
            ProgramStep::At([len(1.0), len(2.0)]),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])
    );
    assert_eq!(literal, LoopProgram::polygon_expr([[len(1.0), len(2.0)]]));
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

/// The repository root: this crate's directory, two levels up.
fn repo_root() -> PathBuf {
    let root = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repository root resolves");
    assert!(
        root.join("Cargo.toml").is_file(),
        "{} is not the repository root",
        root.display()
    );
    root
}

/// **The polygon close is written once in shipped `src`.**
///
/// The agreement rows above cannot see a second expansion; this one
/// can. A hand-rolled polygon ends by pushing `LineTo(Start)` onto a
/// step vector, and after this unit the only site in `crates/*/src`
/// that does so is `polygon_expr` itself — the viewer's template
/// rectangle and the Python `Node.polygon` binding both route through
/// it rather than respelling it.
///
/// **What this cannot match.** It reads a whitespace-stripped
/// `code_only` view for a literal `push` of the close, under
/// `crates/*/src` only. An expansion that appends by `extend`, by
/// collecting an iterator, or by building the `Vec` literally rather
/// than pushing is invisible to it, as is one written in `tests/`, in
/// `demos/`, or behind a macro. It is a guard against the shape that
/// actually recurred three times in this tree, not a proof of
/// uniqueness.
#[test]
fn the_polygon_close_is_written_once_in_shipped_src() {
    const NEEDLES: [&str; 2] = [
        "push(ProgramStep::LineTo(ProgramTarget::Start))",
        "push(d::ProgramStep::LineTo(d::ProgramTarget::Start))",
    ];
    let root = repo_root();
    let mut found: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(root.join("crates")).expect("crates dir listing") {
        let src = entry.expect("dir entry").path().join("src");
        if !src.is_dir() {
            continue;
        }
        for path in test_utils::source::rust_sources(&src) {
            let text = std::fs::read_to_string(&path).expect("readable source file");
            let code: String = test_utils::source::code_only(&text)
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let hits: usize = NEEDLES.iter().map(|n| code.matches(n).count()).sum();
            let rel = path
                .strip_prefix(&root)
                .expect("a walked file lies under the root")
                .to_string_lossy()
                .replace('\\', "/");
            for _ in 0..hits {
                found.push(rel.clone());
            }
        }
    }
    assert_eq!(
        found,
        vec!["crates/editor-core/src/program.rs".to_string()],
        "the polygon close is expanded somewhere other than \
         `LoopProgram::polygon_expr`; route the site through the builder, \
         or give this roster its line with a reason"
    );
}
