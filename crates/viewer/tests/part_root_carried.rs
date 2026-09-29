//! **A part's root failure draws one line per document, each within
//! the budget.**
//!
//! An instance whose part does not evaluate names the part's failed
//! node and points at it; the part's own refusal is drawn under the
//! row as a line of its own, and a part inside a part adds one line per
//! document (`tree::carried_lines`). The rows here build that from a
//! real directory store, open it through the session's `Open` door
//! (which is what wires the resolver and the file names), and hold
//! every drawn line to the refusal standard
//! (`test_utils::refusal::problems`) — a hex document id included,
//! which is how a part used to be named on these lines.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::document::{Doc, DocRef, Expr, Node, ProfileDoc, content_pin};
use pncad::geom_core::Tol;
use pncad::workspace::Workspace;
use viewer::session::{DocSession, SessionOp};
use viewer::tree::RowStatus;

/// **Depth 2**: an assembly instantiates `bracket.pncad`, whose one
/// root instantiates `boss.pncad`, whose one root, an extrude, refuses.
///
/// The instance row names its part by file name, its own line names
/// the bracket's failed node and points, and under it are two lines:
/// the bracket's node in `bracket.pncad`, and the boss's extrude in
/// `boss.pncad`, the last drawn exactly as the boss's own tree draws
/// it. No line quotes the line under it, and every line is within the
/// budget with no hex id.
#[test]
fn a_nested_part_failure_draws_one_line_per_document_within_the_budget() {
    let tol = Tol::witness();
    let dir = std::env::temp_dir().join(format!("partroot-carried-{}", std::process::id()));
    if dir.exists() {
        std::fs::remove_dir_all(&dir).expect("clear the fixture directory");
    }
    std::fs::create_dir_all(&dir).expect("create the fixture directory");
    let mut store = Workspace::open(&dir).expect("the empty workspace opens");
    let reference = |doc: &ProfileDoc| DocRef {
        id: doc.id(),
        pin: content_pin(doc, tol).expect("the pin computes"),
    };

    // The boss: its one root is an extrude whose distance does not
    // evaluate, so the boss has no product.
    let (boss, profile) =
        common::framed_square(&Doc::empty_derived("partroot-boss", tol), 0.04, tol);
    let (boss, boss_root) = common::inserted(
        &boss,
        Node::Extrude {
            profile,
            distance: Expr::div(common::len(0.008), common::scl(0.0))
                .expect("length / scalar is a length"),
        },
        tol,
    );
    store
        .save_at(&boss, "boss.pncad", tol)
        .expect("the boss stores");

    // The bracket: its one root instantiates the boss.
    let mut bracket = Doc::empty_derived("partroot-bracket", tol);
    let bracket_root =
        common::insert_into(&mut bracket, Node::instantiate_part(reference(&boss)), tol);
    store
        .save_at(&bracket, "bracket.pncad", tol)
        .expect("the bracket stores");

    let mut assembly = Doc::empty_derived("partroot-assembly", tol);
    let instance = common::insert_into(
        &mut assembly,
        Node::instantiate_part(reference(&bracket)),
        tol,
    );
    let path = store
        .save_at(&assembly, "assembly.pncad", tol)
        .expect("the assembly stores");

    let mut session = DocSession::inline(Doc::empty_derived("partroot-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let rows = session.tree_rows();
    let row = rows
        .iter()
        .find(|row| row.id == instance)
        .expect("the instance has a row");

    assert_eq!(
        row.pose.as_deref(),
        Some("bracket.pncad"),
        "the instance row names its part by file name"
    );
    let RowStatus::Failed { message, carried } = &row.status else {
        panic!("the instance's own operation failed: {:?}", row.status);
    };
    let drawn: Vec<&String> = core::iter::once(message).chain(carried).collect();
    eprintln!(
        "DRAWN\n{}",
        drawn
            .iter()
            .map(|l| l.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );

    assert_eq!(
        carried.len(),
        2,
        "one carried line per document below the instance: {drawn:#?}"
    );
    assert!(
        message.contains(&format!("repair node {}", bracket_root.0)),
        "the row's own line points at the bracket's failed node: {message}"
    );
    assert!(
        carried[0].starts_with(&format!(
            "in bracket.pncad, node {} failed: ",
            bracket_root.0
        )) && carried[0].contains(&format!("repair node {}", boss_root.0)),
        "the bracket's line names its file and points at the boss's node: {}",
        carried[0]
    );
    let boss_line = format!("in boss.pncad, node {} failed: ", boss_root.0);
    assert!(
        carried[1].starts_with(&boss_line),
        "the last line names the boss's file and its failed node: {}",
        carried[1]
    );
    let refusal = carried[1].strip_prefix(&boss_line).expect("checked above");
    for line in &drawn[..2] {
        assert!(
            !line.contains(refusal),
            "a carrying line never quotes the refusal it carries: {line}"
        );
    }
    let problems: Vec<String> = drawn
        .iter()
        .enumerate()
        .flat_map(|(level, line)| {
            test_utils::refusal::problems(&format!("level {level}"), line, &[], false)
        })
        .collect();
    assert!(problems.is_empty(), "{}", problems.join("\n"));

    std::fs::remove_dir_all(&dir).expect("remove the fixture directory");
}
