//! **A part's product failure draws one level per document, each line the
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

use pncad::document::{
    CancelToken, Doc, DocEdit, DocRef, EvalOptions, Formula, Label, Node, NodeResult, ProfileDoc,
    RecipeNodeId, content_pin, evaluate,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{CapEnd, EntityKind, RoleSeg};
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
/// says it from `doc`: the line `doc`'s own tree draws for it.
fn own_line(doc: &ProfileDoc, node: RecipeNodeId, dir: &Path, tol: Tol) -> String {
    let opts = EvalOptions {
        resolver: Some(Arc::new(Workspace::open(dir).expect("the store opens"))),
        ..EvalOptions::default()
    };
    let evaluation = evaluate::<f64>(doc, None, &CancelToken::new(), &opts, tol);
    match evaluation.result(node) {
        Some(NodeResult::Failed(error)) => error.spoken(doc, &evaluation),
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
/// piece instantiates `boss.pncad`, whose one piece, an extrude, refuses.
///
/// Before the run lands the instance row says its part's file is not
/// read yet, never that there is none. Once it lands, the row names its
/// part by file name, its own line names the bracket's failed node and
/// points, and under it are two levels: the bracket's node, labelled
/// `bracket.pncad`, and the boss's extrude, labelled `boss.pncad`. Each
/// level's line is byte for byte what that part's own tree draws for
/// its node, labels included, no line quotes the line under it, and
/// every line is within the budget.
///
/// The assembly's first node is the bracket's piece again, an instance
/// of the boss, so it has the same id (both documents mint from the
/// zero chain) under another label: every line that says the bracket's
/// node says the bracket's label, and none says the assembly's.
#[test]
fn a_nested_part_failure_draws_one_line_per_document_within_the_budget() {
    let tol = Tol::witness();
    let dir = common::tempdir("partroot-carried");
    let mut store = Workspace::open(&dir).expect("the empty workspace opens");

    // The boss: its one placed body is an extrude whose distance does
    // not evaluate, so the boss has no product.
    let (boss, profile) =
        common::square_profile(&Doc::empty_derived("partroot-boss", tol), 0.04, tol);
    let (boss, boss_piece) = common::inserted(
        &boss,
        Node::Extrude {
            profile: profile.into(),
            distance: Formula::div(common::len(0.008), common::scl(0.0))
                .expect("length / scalar is a length"),
        },
        tol,
    );
    let boss = labelled(&boss, boss_piece, "boss plate", tol);
    let (boss, _) = common::placed(&boss, boss_piece, tol);
    store
        .save_at(&boss, "boss.pncad", tol)
        .expect("the boss stores");

    // The bracket: its one placed body instantiates the boss.
    let mut bracket = Doc::empty_derived("partroot-bracket", tol);
    let bracket_piece = common::insert_into(
        &mut bracket,
        crate::fixture::mated_instance(reference(&boss, tol)),
        tol,
    );
    let bracket = labelled(&bracket, bracket_piece, "bracket seat", tol);
    let (bracket, _) = common::placed(&bracket, bracket_piece, tol);
    store
        .save_at(&bracket, "bracket.pncad", tol)
        .expect("the bracket stores");

    let mut assembly = Doc::empty_derived("partroot-assembly", tol);
    let twin = common::insert_into(
        &mut assembly,
        crate::fixture::mated_instance(reference(&boss, tol)),
        tol,
    );
    assert_eq!(
        twin, bracket_piece,
        "both documents mint from the zero chain"
    );
    let mut assembly = labelled(&assembly, twin, "host twin", tol);
    let instance = common::insert_into(
        &mut assembly,
        crate::fixture::mated_instance(reference(&bracket, tol)),
        tol,
    );
    let path = store
        .save_at(&assembly, "assembly.pncad", tol)
        .expect("the assembly stores");

    let (before, row) = opened(path, instance, tol);
    assert_eq!(
        before.which.as_deref(),
        Some(PartFiles::UNSCANNED),
        "before the first scan the row says the file is unread, not absent"
    );
    assert_eq!(
        row.which.as_deref(),
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
                line: own_line(&bracket, bracket_piece, &dir, tol),
            },
            CarriedLine {
                document: "boss.pncad".to_owned(),
                line: own_line(&boss, boss_piece, &dir, tol),
            },
        ],
        "one level per document below the instance, each labelled with its file and drawn \
         byte for byte as its part's own tree draws it"
    );
    let (bracket_node, boss_node) = (bracket.spoken(bracket_piece), boss.spoken(boss_piece));
    assert!(
        message.contains(&format!("repair {bracket_node}"))
            && carried[0].line.contains(&format!("repair {boss_node}")),
        "each carrying line points at the node the level under it names, by the label its \
         part holds: {message} / {}",
        carried[0].line
    );
    assert!(
        message.contains(&format!("the part's {bracket_node} failed"))
            && !message.contains("host twin")
            && carried.iter().all(|c| !c.line.contains("host twin")),
        "the bracket's node is said with the bracket's label, never the assembly's for the \
         same id: {message} / {carried:?}"
    );
    let refusal = carried[1]
        .line
        .strip_prefix(&format!("{boss_node} failed: "))
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

/// **A poisoned part product draws the failure that poisoned it, at depth 2.**
///
/// `broken.pncad` is [`common::broken_document`]: its extrude refuses
/// and its one piece, a placement of the extrude, never runs.
/// `bracket.pncad`'s one placed body is an instance of it, placed
/// moved, so that placement never runs either; the instance is a placement
/// of its own, loose, and the bracket's gather names it as the node
/// that failed. An assembly instantiating the bracket draws two levels
/// under its row, each at the node that FAILED rather than the piece it
/// cost: the bracket's instance, labelled `bracket.pncad`, and the
/// broken part's extrude, labelled `broken.pncad`. The traceback ends
/// at the failing node, drawn byte for byte as its own document draws
/// it.
#[test]
fn a_poisoned_part_product_draws_the_failure_that_poisoned_it() {
    let tol = Tol::witness();
    let dir = common::tempdir("partroot-poisoned");
    let mut store = Workspace::open(&dir).expect("the empty workspace opens");

    let (broken, extrude, broken_piece) = common::broken_document(tol);
    store
        .save_at(&broken, "broken.pncad", tol)
        .expect("the broken part stores");

    let mut bracket = Doc::empty_derived("partroot-poisoned-bracket", tol);
    // The instance's bundle empty, so it is no piece of the bracket's
    // product of its own: the moved placement of it is.
    let inner = common::insert_into(
        &mut bracket,
        crate::fixture::mated_instance(reference(&broken, tol)),
        tol,
    );
    let (bracket, _) = common::placed_at(&bracket, inner, [0.01, 0.0, 0.0], tol);
    store
        .save_at(&bracket, "bracket.pncad", tol)
        .expect("the bracket stores");

    let mut assembly = Doc::empty_derived("partroot-poisoned-assembly", tol);
    let instance = common::insert_into(
        &mut assembly,
        crate::fixture::mated_instance(reference(&bracket, tol)),
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
    // Each carrying line points at the node that failed; where that
    // node poisoned a piece of its part's product — the body's world
    // placement (A10) — it names the piece it cost too.
    let inner_said = bracket.spoken(inner);
    assert!(
        message.contains(&format!("the part's {inner_said} failed"))
            && message.contains(&format!("repair {inner_said}")),
        "the top line points at the bracket's failed instance: {message}"
    );
    let (piece, failed) = (broken.spoken(broken_piece), broken.spoken(extrude));
    assert!(
        carried[0]
            .line
            .contains(&format!("poisoned {piece}, which its product reads"))
            && carried[0].line.contains(&format!("repair {failed}")),
        "the broken part's line names the piece it cost and points at the node that failed: \
         {}",
        carried[0].line
    );
    hold_to_the_standard(message, carried);

    std::fs::remove_dir_all(&dir).expect("remove the fixture directory");
}

/// `doc` with `node` labelled `text`.
fn labelled(doc: &ProfileDoc, node: RecipeNodeId, text: &str, tol: Tol) -> ProfileDoc {
    common::edited(
        doc,
        DocEdit::SetLabel {
            node,
            label: Some(Label::new(text).expect("a valid label")),
        },
        tol,
    )
    .0
}

/// A small block, as a whole part document, and its body.
/// A placed block: its document, its extrude and the extrude's world
/// placement (A10).
fn block(label: &str, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, profile) = common::square_profile(&Doc::empty_derived(label, tol), 0.02, tol);
    let (doc, body) = common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: common::len(0.02),
        },
        tol,
    );
    let (doc, placement) = common::placed(&doc, body, tol);
    (doc, body, placement)
}

/// **A cap of `block`'s product, read at `instance`**: the extrude's
/// cap, which the part's copy and the instance's carry as it is.
fn in_part(
    instance: RecipeNodeId,
    (body, _): (RecipeNodeId, RecipeNodeId),
    cap: CapEnd,
) -> crate::fixture::resolver::PartName {
    crate::fixture::resolver::PartName {
        at: instance,
        name: StableName {
            kind: EntityKind::Face,
            node: body,
            path: vec![RoleSeg::Cap(cap)],
        },
    }
}

/// **A mate's carried level inside a part is labelled with that part.**
///
/// `sub.pncad` rests its cap instance on copy 1 of a pattern whose
/// direction is `1e200`, so the pattern refuses and the cap, whose
/// mate's target reads the pattern's copy, is poisoned through it. An
/// assembly instantiating `sub.pncad` draws one level under its row:
/// the pattern's refusal, the node that failed. It is numbered in
/// `sub.pncad`, and the level says so.
#[test]
fn a_mates_carried_level_inside_a_part_is_labelled_with_the_part() {
    let tol = Tol::witness();
    let dir = common::tempdir("partroot-carried-mate");
    let mut store = Workspace::open(&dir).expect("the empty workspace opens");
    let (leg, leg_body, leg_copy) = block("partroot-carried-leg", tol);
    let (top, top_body, top_copy) = block("partroot-carried-top", tol);
    store.save_at(&leg, "leg.pncad", tol).expect("stores");
    store.save_at(&top, "top.pncad", tol).expect("stores");

    let sub = Doc::empty_derived("partroot-carried-sub", tol);
    let (sub, cap) = common::inserted(
        &sub,
        crate::fixture::mated_instance(reference(&top, tol)),
        tol,
    );
    let (sub, legs) = common::inserted(
        &sub,
        crate::fixture::mated_instance(reference(&leg, tol)),
        tol,
    );
    // A row whose one slide overflows to infinity at every member.
    let (sub, k) = crate::fixture::index(sub, "k", 4);
    let (sub, pattern) = crate::fixture::family_of(
        sub,
        legs,
        vec![(
            pncad::document::Coord::Slide(pncad::document::TargetAxis::X),
            Formula::mul(common::len(1e200), common::scl(1e200))
                .expect("length times scalar is a length"),
        )],
    );
    let (sub, _) = viewer::test_support::mated(
        &sub,
        cap,
        crate::fixture::rest(
            common::head(in_part(cap, (top_body, top_copy), CapEnd::Start)),
            common::head_at(
                pattern,
                crate::fixture::in_copy(
                    pattern,
                    k,
                    1,
                    in_part(legs, (leg_body, leg_copy), CapEnd::End),
                ),
            ),
        ),
        Vec::new(),
        tol,
    );
    // The world: the cap is the first placement the product door
    // reads, then the legs, then the family's members, which stand in
    // the legs' space.
    store.save_at(&sub, "sub.pncad", tol).expect("stores");
    let mut assembly = Doc::empty_derived("partroot-carried-mate-asm", tol);
    let instance = common::insert_into(
        &mut assembly,
        crate::fixture::mated_instance(reference(&sub, tol)),
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
        vec!["sub.pncad"],
        "the pattern's level is sub.pncad's: {carried:#?}"
    );
    assert_eq!(
        carried[0].line,
        own_line(&sub, pattern, &dir, tol),
        "the level is sub.pncad's own line for the pattern"
    );
    assert!(
        message.contains(&format!("repair {}", sub.spoken(pattern))),
        "the instance's line points at the pattern: {message}"
    );
    hold_to_the_standard(message, carried);
    std::fs::remove_dir_all(&dir).expect("remove the fixture directory");
}
