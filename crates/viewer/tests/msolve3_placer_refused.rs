//! **The tree row names the cause, not a dangling head.**
//!
//! The finding's document — a mate onto a family member whose slide
//! overflows to infinity — once drew a row about a member that did not
//! resolve, about a family that resolves and is well-formed apart from
//! one value. A mate's target reads the family's member, so the family
//! is upstream of the placement: a family that refuses poisons the
//! placement through it, and the placement's row points at the
//! family's, which states the slide's refusal in its own words. A
//! target the family no longer defines is the mate's own refusal
//! (`SideUnevaluated`). The row is the chrome's whole channel for a
//! refusal, so what it says is the measurement.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use crate::fixture;

use fixture::resolver::{PartStore, in_part, with_resolver};
use pncad::document::{
    CancelToken, Coord, Doc, DocumentId, EvalOptions, Evaluation, Formula, MateFault, MateSide,
    Node, NodeErrorKind, NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, TargetAxis, VarId,
    evaluate,
};
use pncad::geom_core::Tol;
use pncad::select::CapEnd;
use viewer::frame::Tone;
use viewer::tree::{self, RowStatus};

/// A small block, as a whole part document, and its body.
fn block(label: &str, tol: Tol) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let (doc, profile) = common::square_profile(&doc, 0.02, tol);
    common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: common::len(0.02),
        },
        tol,
    )
}

/// **The finding's document, through the tree the chrome draws.**
/// The row of the placement the mate places must state the direction
/// refusal and link to the pattern, whose own row fails beside it.
#[test]
fn the_mate_row_names_the_direction_and_not_a_dangling_head() {
    let tol = Tol::witness();
    let mut store = PartStore::default();
    let (leg, leg_body) = store.insert_part(block("msolve3-view-leg", tol), tol);
    let (top, top_body) = store.insert_part(block("msolve3-view-top", tol), tol);

    let doc: Doc<ProfileProgram> = ProfileDoc::empty(DocumentId::derive("msolve3-view"), tol);
    let (doc, legs) = common::inserted(&doc, crate::fixture::mated_instance(leg), tol);
    // A row whose one slide overflows to infinity at every member.
    let (doc, k) = fixture::index(doc, "k", 4);
    let (doc, pattern) = fixture::family_of(
        doc,
        legs,
        vec![(
            Coord::Slide(TargetAxis::X),
            Formula::mul(common::len(1e200), common::scl(1e200))
                .expect("length times scalar is a length"),
        )],
    );
    let (doc, cap) = common::inserted(&doc, fixture::mated_instance(top), tol);
    let (doc, _) = viewer::test_support::mated(
        &doc,
        cap,
        fixture::rest(
            common::head(in_part(cap, top_body, CapEnd::Start)),
            common::head_at(
                pattern,
                fixture::in_copy(pattern, k, 1, in_part(legs, leg_body, CapEnd::End)),
            ),
        ),
        Vec::new(),
        tol,
    );
    // The mate's row is the row of the placement it places.
    let mate = cap;

    let opts = with_resolver(store);
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, tol);
    assert!(
        matches!(ev.result(mate), Some(NodeResult::Poisoned { through }) if *through == pattern),
        "the placement is poisoned through the pattern its mate reads: {:?}",
        ev.result(mate)
    );
    let rows = tree::rows(&doc, Some(&ev), &viewer::parts::PartFiles::default());
    let row = common::row_of(&rows, mate);
    assert!(
        matches!(row.status, RowStatus::Poisoned { through, .. } if through == pattern),
        "the placement's row points at the pattern: {:?}",
        row.status
    );
    // The pattern's row is the cause, in the kernel's words — the WHOLE
    // rendered cause: a wrong role word, a wrong refusal kind or a wrong
    // node all fail here.
    let Some(NodeResult::Failed(error)) = ev.result(pattern) else {
        panic!("the pattern is Failed in the evaluation");
    };
    assert_eq!(
        error.spoken(&doc, &ev),
        format!(
            "Place {} failed: the expression at slot slide value failed to evaluate: the \
             evaluated result is not finite — the arithmetic overflowed or hit a pole (1/0, \
             0/0). Recourse: fix the expression or the values feeding it",
            test_utils::refusal::tag(pattern.0.digest()),
        ),
        "the family's own words name the slide"
    );
    let placer = common::row_of(&rows, pattern);
    assert!(
        matches!(placer.status, RowStatus::Failed { .. }),
        "the pattern fails on its own: {:?}",
        placer.status
    );
    assert_no_row_links(&rows);
}

// ---- An edit that strands a mate's target ----
//
// The mate's target reads a copy of a pattern; an edit to the pattern
// after the mate was authored strands it. The tree blames the MATE's
// placement, whose row carries the evaluation's own words, and never
// the pattern beside it.

/// A pattern of three copies of one instance, a second instance, and
/// a mate resting the second on copy `copy` of the first.
struct Copies {
    doc: Doc<ProfileProgram>,
    opts: EvalOptions,
    pattern: RecipeNodeId,
    index: VarId,
    mate: RecipeNodeId,
}

fn copies(label: &str, copy: u32, tol: Tol) -> Copies {
    let mut store = PartStore::default();
    let (leg, leg_body) = store.insert_part(block(&format!("{label}-leg"), tol), tol);
    let (top, top_body) = store.insert_part(block(&format!("{label}-top"), tol), tol);

    let doc: Doc<ProfileProgram> = ProfileDoc::empty(DocumentId::derive(label), tol);
    let (doc, legs) = common::inserted(&doc, crate::fixture::mated_instance(leg), tol);
    let (doc, k) = fixture::index(doc, "k", 3);
    let (doc, pattern) =
        fixture::family_of(doc, legs, fixture::slides_along(k, [1.0, 0.0, 0.0], 0.05));
    let (doc, cap) = common::inserted(&doc, fixture::mated_instance(top), tol);
    let named = fixture::in_copy(pattern, k, copy, in_part(legs, leg_body, CapEnd::End));
    let (doc, _) = viewer::test_support::mated(
        &doc,
        cap,
        fixture::rest(
            common::head(in_part(cap, top_body, CapEnd::Start)),
            common::head_at(pattern, named),
        ),
        Vec::new(),
        tol,
    );
    // The mate's row is the row of the placement it places.
    let mate = cap;
    Copies {
        doc,
        opts: with_resolver(store),
        pattern,
        index: k,
        mate,
    }
}

/// The fault the evaluation recorded against `mate`, which must be a
/// mate refusal.
fn mate_fault(ev: &Evaluation<f64>, mate: RecipeNodeId) -> MateFault {
    let Some(NodeResult::Failed(error)) = ev.result(mate) else {
        panic!("the mate's own result must be Failed");
    };
    let NodeErrorKind::Mate(fault) = &error.kind else {
        panic!("the mate must fail as a mate refusal: {error}");
    };
    (**fault).clone()
}

/// **No row links to a node to repair** — a mate's refusal is its
/// placement's (`viewer::tree`'s module header), so the mate's words
/// and the lines they carry are the whole of what its row says.
fn assert_no_row_links(rows: &[tree::TreeRow]) {
    let linking: Vec<(RecipeNodeId, RecipeNodeId)> = rows
        .iter()
        .filter_map(|row| row.repair_at.as_ref().map(|at| (row.id, at.id())))
        .collect();
    assert_eq!(linking, Vec::new(), "no row links to a node to repair");
}

/// **The mate's row is the cause, and the named node's row is not
/// pointed at.**
///
/// The mate keeps its own `Failed`, in the kernel's words, and is the
/// actionable row; the named node evaluated on its own and reads `Ok`,
/// so a tree that blamed it would be sending a poisoned row's pointer
/// to a healthy row.
fn assert_the_mate_is_blamed(
    ev: &Evaluation<f64>,
    doc: &Doc<ProfileProgram>,
    mate: RecipeNodeId,
    named: RecipeNodeId,
) {
    let rows = tree::rows(doc, Some(ev), &viewer::parts::PartFiles::default());
    let mate_row = common::status_of(&rows, mate);
    let Some(NodeResult::Failed(error)) = ev.result(mate) else {
        panic!("the mate must be Failed in the evaluation");
    };
    let RowStatus::Failed { message, .. } = &mate_row else {
        panic!("the mate's row is the failure: {mate_row:?}");
    };
    assert_eq!(
        *message,
        error.spoken(doc, ev),
        "the mate's row is the cause and carries the fault's own words"
    );
    assert_eq!(mate_row.tone(), Tone::Actionable);
    assert_no_row_links(&rows);
    assert_eq!(
        common::status_of(&rows, named),
        RowStatus::Ok,
        "the node named beside the mate evaluated on its own"
    );
    let pointed_at_named: Vec<RecipeNodeId> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Poisoned { through, .. } if through == named))
        .map(|row| row.id)
        .collect();
    assert_eq!(pointed_at_named, Vec::<RecipeNodeId>::new());
}

/// **A shrunk pattern strands a mate's target: the mate's row is the
/// cause, and the pattern is not.**
///
/// The copy the mate rests on is now past the pattern's count, so the
/// mate's target does not evaluate (`SideUnevaluated`); the pattern
/// itself evaluates its two copies without complaint.
#[test]
fn a_stranded_copy_blames_the_mate_and_not_the_pattern_it_stopped_at() {
    let tol = Tol::witness();
    let s = copies("msolve3-view-dangling", 2, tol);
    let (doc, _) = common::edited(
        &s.doc,
        fixture::recount_edit(&s.doc, s.index, Formula::count(2)),
        tol,
    );
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &s.opts, tol);
    let fault = mate_fault(&ev, s.mate);
    assert!(
        matches!(
            fault,
            MateFault::SideUnevaluated {
                side: MateSide::To,
                ..
            }
        ),
        "the fixture strands the mate's target: {fault:?}"
    );
    assert_the_mate_is_blamed(&ev, &doc, s.mate, s.pattern);
}
