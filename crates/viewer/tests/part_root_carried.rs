//! **A part's root failure draws one level per document, each line the
//! node's own and each within the budget.**
//!
//! An instance whose part does not evaluate names the part's failed
//! node and points at it; the part's own refusal is drawn under the
//! row as a level of its own, and a part inside a part adds one level
//! per document (`tree::carried_lines`). Each level's line is its node's
//! refusal exactly as that node's own tree draws it, and the document
//! it is in is a label beside it, never words inside it. The rows here
//! build that from a real directory store and open it through the
//! session's `Open` door, which is what wires the resolver and the file
//! names.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::path::Path;
use std::sync::Arc;

use crate::common;
use crate::fixture;

use fixture::resolver::in_part;
use pncad::document::{
    Alignment, AxisSense, CancelToken, Doc, DocRef, EvalOptions, Expr, MateFrame, MatePrimitive,
    Node, NodeResult, PatternKind, ProfileDoc, RecipeNodeId, content_pin, evaluate,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{CapEnd, ContactClass, EntityKind, RoleSeg};
use pncad::workspace::Workspace;
use viewer::parts::PartFiles;
use viewer::session::{DocSession, SessionOp};
use viewer::tree::{CarriedLine, RowStatus, TreeRow};

fn reference(doc: &ProfileDoc, tol: Tol) -> DocRef {
    DocRef {
        id: doc.id(),
        pin: content_pin(doc, tol).expect("the pin computes"),
    }
}

/// `node`'s failure as `doc`'s own evaluation, over the store in `dir`,
/// renders it: the line `doc`'s own tree draws for it.
fn own_line(doc: &ProfileDoc, node: RecipeNodeId, dir: &Path, tol: Tol) -> String {
    let opts = EvalOptions {
        resolver: Some(Arc::new(Workspace::open(dir).expect("the store opens"))),
        ..EvalOptions::default()
    };
    match evaluate::<f64>(doc, None, &CancelToken::new(), &opts, tol).result(node) {
        Some(NodeResult::Failed(error)) => error.to_string(),
        other => panic!("{node:?} fails in its own document: {other:?}"),
    }
}

/// Opens `path` in a fresh session, reads the tree before and after the
/// run lands, and answers `instance`'s row from each.
fn opened(path: std::path::PathBuf, instance: RecipeNodeId, tol: Tol) -> (TreeRow, TreeRow) {
    let mut session = DocSession::inline(Doc::empty_derived("partroot-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let row = |session: &DocSession| common::row_of(&session.tree_rows(), instance).clone();
    let before = row(&session);
    session.pump();
    (before, row(&session))
}

/// Every drawn line of a failed row, held to the refusal standard, and
/// every level's label to it too.
///
/// Two checks are not this row's: a label is a document's name, not a
/// refusal, so it states no recourse; and a carried line is the carried
/// node's own refusal, whose recourse the refusal roster holds
/// (`editor-core/tests/refusal_concision_chains.rs`, with its filed
/// admissions), so it is not held to one here a second time.
fn hold_to_the_standard(message: &str, carried: &[CarriedLine]) {
    let not_a_recourse_check = |name: &str| {
        let lost = format!("{name} states no recourse");
        move |p: &String| !p.starts_with(&lost)
    };
    let mut problems = test_utils::refusal::problems("level 0", message, &[], false);
    for (level, carried) in carried.iter().enumerate() {
        let name = format!("level {}", level + 1);
        problems.extend(
            test_utils::refusal::problems(&name, &carried.line, &[], false)
                .into_iter()
                .filter(not_a_recourse_check(&name)),
        );
        let label = format!("{name}'s label");
        problems.extend(
            test_utils::refusal::problems(&label, &carried.document, &[], false)
                .into_iter()
                .filter(not_a_recourse_check(&label)),
        );
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// **Depth 2**: an assembly instantiates `bracket.pncad`, whose one
/// root instantiates `boss.pncad`, whose one root, an extrude, refuses.
///
/// Before the run lands the instance row says its part's file is not
/// read yet, never that there is none. Once it lands, the row names its
/// part by file name, its own line names the bracket's failed node and
/// points, and under it are two levels: the bracket's node, labelled
/// `bracket.pncad`, and the boss's extrude, labelled `boss.pncad`. Each
/// level's line is byte for byte what that part's own evaluation
/// renders for its node, no line quotes the line under it, and every
/// line is within the budget.
#[test]
fn a_nested_part_failure_draws_one_line_per_document_within_the_budget() {
    let tol = Tol::witness();
    let dir = common::tempdir("partroot-carried");
    let mut store = Workspace::open(&dir).expect("the empty workspace opens");

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
    let bracket_root = common::insert_into(
        &mut bracket,
        Node::instantiate_part(reference(&boss, tol)),
        tol,
    );
    store
        .save_at(&bracket, "bracket.pncad", tol)
        .expect("the bracket stores");

    let mut assembly = Doc::empty_derived("partroot-assembly", tol);
    let instance = common::insert_into(
        &mut assembly,
        Node::instantiate_part(reference(&bracket, tol)),
        tol,
    );
    let path = store
        .save_at(&assembly, "assembly.pncad", tol)
        .expect("the assembly stores");

    let (before, row) = opened(path, instance, tol);
    assert_eq!(
        before.pose.as_deref(),
        Some(PartFiles::UNSCANNED),
        "before the first scan the row says the file is unread, not absent"
    );
    assert_eq!(
        row.pose.as_deref(),
        Some("bracket.pncad"),
        "the instance row names its part by file name"
    );
    let RowStatus::Failed { message, carried } = &row.status else {
        panic!("the instance's own operation failed: {:?}", row.status);
    };
    eprintln!("DRAWN\n{message}");
    for (level, carried) in carried.iter().enumerate() {
        let indent = "  ".repeat(level + 1);
        eprintln!("{indent}[{}]\n{indent}{}", carried.document, carried.line);
    }

    assert_eq!(
        carried,
        &vec![
            CarriedLine {
                document: "bracket.pncad".to_owned(),
                line: own_line(&bracket, bracket_root, &dir, tol),
            },
            CarriedLine {
                document: "boss.pncad".to_owned(),
                line: own_line(&boss, boss_root, &dir, tol),
            },
        ],
        "one level per document below the instance, each labelled with its file and drawn \
         byte for byte as its part's own tree draws it"
    );
    assert!(
        message.contains(&format!(
            "repair node {}",
            test_utils::refusal::tag(bracket_root.0)
        )) && carried[0].line.contains(&format!(
            "repair node {}",
            test_utils::refusal::tag(boss_root.0)
        )),
        "each carrying line points at the node the level under it names: {message} / {}",
        carried[0].line
    );
    let refusal = carried[1]
        .line
        .strip_prefix(&format!(
            "node {} failed: ",
            test_utils::refusal::tag(boss_root.0)
        ))
        .expect("the boss's line opens with its node");
    for line in [message, &carried[0].line] {
        assert!(
            !line.contains(refusal),
            "a carrying line never quotes the refusal it carries: {line}"
        );
    }
    hold_to_the_standard(message, carried);

    std::fs::remove_dir_all(&dir).expect("remove the fixture directory");
}

/// **A poisoned root draws the failure that poisoned it, at depth 2.**
///
/// `broken.pncad` is [`common::broken_document`]: its extrude refuses
/// and its one root, a transform over the extrude, never runs.
/// `bracket.pncad`'s one root is a transform over an instance of it, so
/// that root never runs either. An assembly instantiating the bracket
/// draws two levels under its row, each at the node that FAILED rather
/// than the root it cost: the bracket's instance, labelled
/// `bracket.pncad`, and the broken part's extrude, labelled
/// `broken.pncad`. The traceback ends at the failing node, drawn byte
/// for byte as its own document draws it.
#[test]
fn a_poisoned_part_root_draws_the_failure_that_poisoned_it() {
    let tol = Tol::witness();
    let dir = common::tempdir("partroot-poisoned");
    let mut store = Workspace::open(&dir).expect("the empty workspace opens");

    let (broken, extrude, broken_root) = common::broken_document(tol);
    store
        .save_at(&broken, "broken.pncad", tol)
        .expect("the broken part stores");

    let mut bracket = Doc::empty_derived("partroot-poisoned-bracket", tol);
    let inner = common::insert_into(
        &mut bracket,
        Node::instantiate_part(reference(&broken, tol)),
        tol,
    );
    let bracket_root = common::insert_into(
        &mut bracket,
        Node::transform(
            inner,
            pncad::document::Step::Rigid {
                translation: [common::len(0.01), common::len(0.0), common::len(0.0)],
                axis: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
                angle: common::ang(0.0),
            },
        ),
        tol,
    );
    store
        .save_at(&bracket, "bracket.pncad", tol)
        .expect("the bracket stores");

    let mut assembly = Doc::empty_derived("partroot-poisoned-assembly", tol);
    let instance = common::insert_into(
        &mut assembly,
        Node::instantiate_part(reference(&bracket, tol)),
        tol,
    );
    let path = store
        .save_at(&assembly, "assembly.pncad", tol)
        .expect("the assembly stores");

    let (_, row) = opened(path, instance, tol);
    let RowStatus::Failed { message, carried } = &row.status else {
        panic!("the instance's own operation failed: {:?}", row.status);
    };
    eprintln!("DRAWN\n{message}");
    for (level, carried) in carried.iter().enumerate() {
        let indent = "  ".repeat(level + 1);
        eprintln!("{indent}[{}]\n{indent}{}", carried.document, carried.line);
    }

    assert_eq!(
        carried,
        &vec![
            CarriedLine {
                document: "bracket.pncad".to_owned(),
                line: own_line(&bracket, inner, &dir, tol),
            },
            CarriedLine {
                document: "broken.pncad".to_owned(),
                line: own_line(&broken, extrude, &dir, tol),
            },
        ],
        "one level per document, each at the node that failed, labelled with its file and \
         drawn as its part's own tree draws it"
    );
    for (line, root, failed) in [
        (message, bracket_root, inner),
        (&carried[0].line, broken_root, extrude),
    ] {
        assert!(
            line.contains(&format!(
                "its root, node {}",
                test_utils::refusal::tag(root.0)
            )) && line.contains(&format!(
                "repair node {}",
                test_utils::refusal::tag(failed.0)
            )),
            "each carrying line names the root it cost and points at the node that failed: \
             {line}"
        );
    }
    hold_to_the_standard(message, carried);

    std::fs::remove_dir_all(&dir).expect("remove the fixture directory");
}

/// A small block, as a whole part document, and its body.
fn block(label: &str, tol: Tol) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = common::framed_square(&Doc::empty_derived(label, tol), 0.02, tol);
    common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.02),
        },
        tol,
    )
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame::authored(origin, axis, [1.0, 0.0, 0.0])
}

/// **A mate's carried level inside a part is labelled with that part.**
///
/// `sub.pncad` mates onto copy 1 of a pattern whose direction is
/// `1e200`, so its fold refuses `PlacerRefused` and poisons the pattern;
/// its first root, the cap instance the mate places, fails with that
/// fault. An assembly instantiating `sub.pncad` draws two levels under
/// its row: the cap's refusal and, under it, the pattern's, which the
/// mate carries. Both nodes are numbered in `sub.pncad`, and both
/// levels say so.
#[test]
fn a_mates_carried_level_inside_a_part_is_labelled_with_the_part() {
    let tol = Tol::witness();
    let dir = common::tempdir("partroot-carried-mate");
    let mut store = Workspace::open(&dir).expect("the empty workspace opens");
    let (leg, leg_body) = block("partroot-carried-leg", tol);
    let (top, top_body) = block("partroot-carried-top", tol);
    store.save_at(&leg, "leg.pncad", tol).expect("stores");
    store.save_at(&top, "top.pncad", tol).expect("stores");

    let sub = Doc::empty_derived("partroot-carried-sub", tol);
    // The cap first, so it is the first root the product door reads.
    let (sub, cap) = common::inserted(&sub, Node::instantiate_part(reference(&top, tol)), tol);
    let (sub, legs) = common::inserted(&sub, Node::instantiate_part(reference(&leg, tol)), tol);
    let (sub, pattern) = common::inserted(
        &sub,
        Node::Pattern {
            input: legs,
            count: Expr::count(4),
            kind: PatternKind::Linear {
                direction: [common::scl(1e200), common::scl(0.0), common::scl(0.0)],
                spacing: common::len(0.05),
            },
        },
        tol,
    );
    let (sub, _mate) = common::inserted(
        &sub,
        Node::Mate {
            a: common::head(StableName {
                kind: EntityKind::Face,
                node: pattern,
                path: vec![RoleSeg::Instance {
                    i: 1,
                    of: in_part(legs, leg_body, CapEnd::End).into(),
                }],
            }),
            b: common::head(in_part(cap, top_body, CapEnd::Start)),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: frame([0.0, 0.0, 0.02], [0.0, 0.0, 1.0]),
                b: frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Opposed,
                clocking: None,
            },
        },
        tol,
    );
    store.save_at(&sub, "sub.pncad", tol).expect("stores");
    let mut assembly = Doc::empty_derived("partroot-carried-mate-asm", tol);
    let instance = common::insert_into(
        &mut assembly,
        Node::instantiate_part(reference(&sub, tol)),
        tol,
    );
    let path = store
        .save_at(&assembly, "assembly.pncad", tol)
        .expect("stores");

    let (_, row) = opened(path, instance, tol);
    let RowStatus::Failed { message, carried } = &row.status else {
        panic!("the instance fails: {:?}", row.status);
    };
    assert_eq!(
        carried
            .iter()
            .map(|c| c.document.as_str())
            .collect::<Vec<_>>(),
        vec!["sub.pncad", "sub.pncad"],
        "the cap's level and the placer's under it are both sub.pncad's: {carried:#?}"
    );
    assert_eq!(
        carried[0].line,
        own_line(&sub, cap, &dir, tol),
        "the cap's level is sub.pncad's own line for it"
    );
    assert!(
        carried[1].line.starts_with(&format!(
            "node {} failed: ",
            test_utils::refusal::tag(pattern.0)
        )),
        "the carried level is the pattern's refusal: {}",
        carried[1].line
    );
    hold_to_the_standard(message, carried);
    std::fs::remove_dir_all(&dir).expect("remove the fixture directory");
}
