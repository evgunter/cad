//! **The feature tree's indentation is a branch count, not a chain
//! length.**
//!
//! The tree's shape is a pure function of the recipe DAG, so it is
//! asserted here as values rather than looked at. The one rule under
//! test is `tree`'s: a node continues the line of its PRIMARY input
//! (the first entry of `Node::inputs()` — a boolean's `a`, a fillet's
//! `target`), and every other input is a branch that indents one
//! level.
//!
//! The case that motivates it is the one a real part has: a base
//! solid with twenty features cut out of it in sequence. Those are
//! twenty chained booleans, and under a longest-chain depth each one
//! indents past the last until the tree walks off the panel. The
//! chain is not a hierarchy — it is one accumulating body — and the
//! assertions below say so by holding the depth FLAT as the chain
//! grows.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use std::collections::BTreeMap;

use pncad::document::{BooleanOp, Doc, Node, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use viewer::tree;

/// Depth by node id, for a document read without an evaluation.
fn depths(doc: &Doc<ProfileProgram>) -> BTreeMap<RecipeNodeId, usize> {
    tree::rows(doc, None, &viewer::parts::PartFiles::default())
        .into_iter()
        .map(|row| (row.id, row.depth))
        .collect()
}

/// A square plate at `side`, answering the document, the profile and
/// the extrude.
fn plate(
    doc: &Doc<ProfileProgram>,
    side: f64,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let (doc, profile) = common::framed_square(doc, side, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.01),
        },
        tol,
    );
    (doc, profile, extrude)
}

#[test]
fn a_single_input_node_continues_its_inputs_line() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("tree-shape-chain", tol);
    let (doc, profile, extrude) = plate(&doc, 0.04, tol);
    let depths = depths(&doc);
    assert_eq!(depths.get(&profile), Some(&0));
    assert_eq!(
        depths.get(&extrude),
        Some(&0),
        "an extrude's only input is its primary, so it continues that line"
    );
}

#[test]
fn a_chain_of_booleans_stays_at_one_level_however_long_it_gets() {
    let tol = Tol::witness();
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("tree-shape-booleans", tol);
    let (next, _, base) = plate(&doc, 0.1, tol);
    doc = next;

    // Twenty cuts into one accumulating body — the die's shape.
    let mut accumulated = base;
    let mut booleans = Vec::new();
    let mut tools = Vec::new();
    for i in 0..20 {
        let (next, _, tool) = plate(&doc, 0.005 + f64::from(i) * 0.001, tol);
        doc = next;
        let (next, cut) = common::inserted(
            &doc,
            Node::Boolean {
                op: BooleanOp::Subtract,
                a: accumulated,
                b: tool,
                declare: None,
            },
            tol,
        );
        doc = next;
        tools.push(tool);
        booleans.push(cut);
        accumulated = cut;
    }

    let depths = depths(&doc);
    assert_eq!(
        depths.get(&base),
        Some(&0),
        "the body being cut is the line"
    );
    for (i, cut) in booleans.iter().enumerate() {
        assert_eq!(
            depths.get(cut),
            Some(&1),
            "boolean {i} indents once for its tool branch and no further"
        );
    }
    for tool in &tools {
        assert_eq!(
            depths.get(tool),
            Some(&0),
            "a tool solid is its own line's root"
        );
    }
}

#[test]
fn a_tool_that_is_itself_a_branch_indents_one_level_further() {
    let tol = Tol::witness();
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("tree-shape-nested", tol);
    let (next, _, base) = plate(&doc, 0.1, tol);
    doc = next;
    let (next, _, tool_a) = plate(&doc, 0.02, tol);
    doc = next;
    let (next, _, tool_b) = plate(&doc, 0.01, tol);
    doc = next;

    // The tool is itself a boolean, so it sits at depth 1 in its own
    // right; used as a branch it draws one deeper still.
    let (next, compound_tool) = common::inserted(
        &doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: tool_a,
            b: tool_b,
            declare: None,
        },
        tol,
    );
    doc = next;
    let (doc, cut) = common::inserted(
        &doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: base,
            b: compound_tool,
            declare: None,
        },
        tol,
    );

    let depths = depths(&doc);
    assert_eq!(depths.get(&compound_tool), Some(&1));
    assert_eq!(
        depths.get(&cut),
        Some(&2),
        "one past the branch it consumes, not one past its own line"
    );
}

/// **A measure's row reads the landed run, as every row's badge
/// does**: while the document has moved past the landed evaluation,
/// the row shows what that run measured on the document it ran over,
/// and the new value arrives with the run that measures it.
///
/// Red if the tree blanks a value while a run is outstanding, or keeps
/// the old one once the new run has landed.
#[test]
fn a_measure_row_shows_the_landed_value_until_the_next_run_lands() {
    use pncad::document::{Dimension, DocParam, Expr, MeasureExpr, ParamName};
    use viewer::props::{Computed, SlotValue};
    use viewer::session::{DocSession, SessionOp};
    use viewer::tree::Readout;

    let tol = Tol::witness();
    let gap = ParamName::from_static("gap");
    let mut doc = common::declared(
        "measure-landed",
        &gap,
        DocParam::continuous(Dimension::Length, 0.01),
        tol,
    );
    let measure = common::insert_into(
        &mut doc,
        Node::measure(
            MeasureExpr::value(Expr::param(gap.clone(), Dimension::Length)),
            Vec::new(),
        )
        .expect("a value measure references nothing"),
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let measured = |session: &DocSession| {
        common::row_of(&session.tree_rows(), measure)
            .readout
            .clone()
    };
    let reading = |metres| {
        Some(Readout::Value(Computed {
            canonical: metres,
            dimension: Dimension::Length,
        }))
    };
    let landed = reading(0.01);
    assert_eq!(measured(&session), landed);

    session.perform(SessionOp::SetParam {
        name: gap,
        value: SlotValue::Continuous(0.012),
    });
    assert!(session.busy(), "the premise: the document has moved on");
    assert_eq!(
        measured(&session),
        landed,
        "the row is the landed picture's until the next one lands"
    );
    session.pump();
    assert_eq!(measured(&session), reading(0.012));
}
