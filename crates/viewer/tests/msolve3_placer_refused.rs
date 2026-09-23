//! **The tree row names the cause, not a dangling head.**
//!
//! The finding's document — a mate onto a pattern copy whose
//! direction slot is `1e200` — used to draw a row reading "resolves
//! through node 1, which does not resolve to a live member", about a
//! pattern that resolves and is well-formed apart from one slot. The
//! row is the chrome's whole channel for a refusal, so what it says
//! is the measurement.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use crate::fixture;

use fixture::resolver::{PartStore, in_part, with_resolver};
use pncad::document::{
    Alignment, AxisSense, CancelToken, Doc, DocEdit, DocumentId, EvalOptions, Evaluation, Expr,
    MateFault, MateFrame, MatePrimitive, Node, NodeErrorKind, NodeResult, PartSelect, PatternKind,
    ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, evaluate,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{CapEnd, ContactClass, EntityKind, RoleSeg};
use viewer::frame::Tone;
use viewer::tree::{self, RowStatus};

/// A small block, as a whole part document: frame, profile, extrude,
/// so the extrude is `fixture::resolver::PART_BODY`.
fn block(label: &str, tol: Tol) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let (doc, profile) = common::framed_square(&doc, 0.02, tol);
    let (doc, _) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.02),
        },
        tol,
    );
    doc
}

/// **The finding's document, through the tree the chrome draws.**
/// The mate's row must state the direction refusal; the pattern's row
/// is `Poisoned` through the mate, which is exactly why the row that
/// names the cause has to be this one.
#[test]
fn the_mate_row_names_the_direction_and_not_a_dangling_head() {
    let tol = Tol::witness();
    let mut store = PartStore::default();
    let leg = store.insert(block("msolve3-view-leg", tol), tol);
    let top = store.insert(block("msolve3-view-top", tol), tol);

    let doc: Doc<ProfileProgram> = ProfileDoc::empty(DocumentId::derive("msolve3-view"), tol);
    let (doc, legs) = common::inserted(&doc, Node::instantiate_part(leg), tol);
    let (doc, pattern) = common::inserted(
        &doc,
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
    let (doc, cap) = common::inserted(&doc, Node::instantiate_part(top), tol);
    let frame = |origin: [f64; 3], axis: [f64; 3]| MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    };
    let (doc, mate) = common::edited(
        &doc,
        DocEdit::InsertNode {
            node: Node::Mate {
                a: common::head(StableName {
                    kind: EntityKind::Face,
                    node: pattern,
                    path: vec![RoleSeg::Instance {
                        i: 1,
                        of: in_part(legs, CapEnd::End).into(),
                    }],
                }),
                b: common::head(in_part(cap, CapEnd::Start)),
                class: ContactClass::Rest,
                alignment: Alignment {
                    a: frame([0.0, 0.0, 0.02], [0.0, 0.0, 1.0]),
                    b: frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
                    primitive: MatePrimitive::FrameCoincidence,
                    sense: AxisSense::Opposed,
                    clocking: None,
                },
            },
        },
        tol,
    );
    let mate = mate.expect("the mate mints");

    let opts = with_resolver(store);
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, tol);
    let rows = tree::rows(&doc, Some(&ev));
    let row = rows
        .iter()
        .find(|r| r.id == mate)
        .expect("the mate has a row");
    let message = row
        .status
        .message()
        .expect("a failed row carries the refusal's prose");
    assert!(
        matches!(row.status, RowStatus::Failed { .. }),
        "the mate's row is the failure: {:?}",
        row.status
    );
    // The WHOLE rendered cause, not a substring of it: a wrong role
    // word, a wrong refusal kind (a zero length instead of an
    // unmeasurable one), or a wrong node all fail here.
    assert_eq!(
        message,
        format!(
            "node {} failed: the mate solve refused: mate {}'s a reference has no derived pose: \
             node {} refuses — the pattern direction has no finite length — its components \
             overflow the norm, or one of them is not a number; scale the geometry into the \
             session's range",
            mate.0, mate.0, pattern.0
        ),
        "the row states the cause the evaluation typed"
    );

    // The compensation the old design rested on, measured: the
    // pattern's own row cannot state the cause, because the mate
    // fault poisoned it.
    let placer = rows
        .iter()
        .find(|r| r.id == pattern)
        .expect("the pattern has a row");
    assert!(
        matches!(placer.status, RowStatus::Poisoned { through, .. } if through == mate),
        "the pattern is poisoned through the mate: {:?}",
        placer.status
    );
}

// ---- A fault that names a node beside its mate ----
//
// Two more arms carry a second node id: `DanglingHead`'s `head` and
// `PartSelectsAnotherCopy`'s `part`. The tree still blames the MATE for
// both (`viewer::tree`'s module header gives the kernel's reason per
// arm), and these rows hold that where a user meets it: an edit to the
// named node, after the mate was authored, is what strands the mate.

/// A pattern of three copies of one instance, a second instance, and
/// a mate from a copy of the first onto the second — the reference
/// read at `at` when one is given (a `Part` above the pattern), at its
/// own mint otherwise.
struct Copies {
    doc: Doc<ProfileProgram>,
    opts: EvalOptions,
    pattern: RecipeNodeId,
    part: Option<RecipeNodeId>,
    mate: RecipeNodeId,
}

fn copies(label: &str, copy: u32, part_selects: Option<i64>, tol: Tol) -> Copies {
    let mut store = PartStore::default();
    let leg = store.insert(block(&format!("{label}-leg"), tol), tol);
    let top = store.insert(block(&format!("{label}-top"), tol), tol);

    let doc: Doc<ProfileProgram> = ProfileDoc::empty(DocumentId::derive(label), tol);
    let (doc, legs) = common::inserted(&doc, Node::instantiate_part(leg), tol);
    let (doc, pattern) = common::inserted(
        &doc,
        Node::Pattern {
            input: legs,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [common::scl(1.0), common::scl(0.0), common::scl(0.0)],
                spacing: common::len(0.05),
            },
        },
        tol,
    );
    let (doc, part) = match part_selects {
        Some(i) => {
            let (doc, part) = common::inserted(
                &doc,
                Node::Part {
                    of: pattern,
                    select: PartSelect::Instance(Expr::count(i)),
                },
                tol,
            );
            (doc, Some(part))
        }
        None => (doc, None),
    };
    let (doc, cap) = common::inserted(&doc, Node::instantiate_part(top), tol);
    let named = StableName {
        kind: EntityKind::Face,
        node: pattern,
        path: vec![RoleSeg::Instance {
            i: copy,
            of: in_part(legs, CapEnd::End).into(),
        }],
    };
    let a = match part {
        Some(part) => common::head_at(part, named),
        None => common::head(named),
    };
    let frame = |origin: [f64; 3], axis: [f64; 3]| MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    };
    let (doc, mate) = common::edited(
        &doc,
        DocEdit::InsertNode {
            node: Node::Mate {
                a,
                b: common::head(in_part(cap, CapEnd::Start)),
                class: ContactClass::Rest,
                alignment: Alignment {
                    a: frame([0.0, 0.0, 0.02], [0.0, 0.0, 1.0]),
                    b: frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
                    primitive: MatePrimitive::FrameCoincidence,
                    sense: AxisSense::Opposed,
                    clocking: None,
                },
            },
        },
        tol,
    );
    Copies {
        doc,
        opts: with_resolver(store),
        pattern,
        part,
        mate: mate.expect("the mate mints while its copy is there"),
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

/// **The mate's row is the cause, and the named node's row is not
/// pointed at** — the three assertions both rows below make.
///
/// The mate keeps its own `Failed`, carrying the payload's words (which
/// name the other node by number), and is the actionable row; the named
/// node evaluated on its own and reads `Ok`, so a tree that blamed it
/// would be sending a poisoned row's pointer to a healthy row.
fn assert_the_mate_is_blamed(
    ev: &Evaluation<f64>,
    doc: &Doc<ProfileProgram>,
    mate: RecipeNodeId,
    named: RecipeNodeId,
) {
    let rows = tree::rows(doc, Some(ev));
    let mate_row = common::status_of(&rows, mate);
    let Some(NodeResult::Failed(error)) = ev.result(mate) else {
        panic!("the mate must be Failed in the evaluation");
    };
    assert_eq!(
        mate_row,
        RowStatus::Failed {
            message: error.to_string()
        },
        "the mate's row is the cause and carries the payload's own words"
    );
    assert_eq!(mate_row.tone(), Tone::Actionable);
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

/// **A shrunk pattern strands a mate's copy: the mate's row is the
/// cause, and the pattern the walk stopped at is not.**
///
/// `DanglingHead`'s `head` is the pattern whose count the named copy is
/// now past. The kernel's own message names the recourse — *"rebind
/// it"*, the mate's reference — and the pattern itself evaluates its
/// two copies without complaint.
#[test]
fn a_stranded_copy_blames_the_mate_and_not_the_pattern_it_stopped_at() {
    let tol = Tol::witness();
    let s = copies("msolve3-view-dangling", 2, None, tol);
    let (doc, _) = common::edited(
        &s.doc,
        DocEdit::SetStructuralParam {
            node: s.pattern,
            slot: SlotId::Count,
            expr: Expr::count(2),
        },
        tol,
    );
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &s.opts, tol);
    let fault = mate_fault(&ev, s.mate);
    assert!(
        matches!(fault, MateFault::DanglingHead { head, .. } if head == s.pattern),
        "the fixture reaches the arm, naming the pattern: {fault:?}"
    );
    assert_the_mate_is_blamed(&ev, &doc, s.mate, s.pattern);
}

/// **A `Part` re-pointed at another copy: the mate's row is the cause,
/// and the `Part` is not.**
///
/// `PartSelectsAnotherCopy` is a disagreement between the mate's name
/// and the `Part`'s index, which the kernel refuses rather than
/// choosing between; the `Part` still gathers the copy it selects.
#[test]
fn a_part_selecting_another_copy_blames_the_mate_and_not_the_part() {
    let tol = Tol::witness();
    let s = copies("msolve3-view-part", 1, Some(1), tol);
    let part = s.part.expect("this fixture has a Part");
    let (doc, _) = common::edited(
        &s.doc,
        DocEdit::SetStructuralParam {
            node: part,
            slot: SlotId::Instance,
            expr: Expr::count(2),
        },
        tol,
    );
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &s.opts, tol);
    let fault = mate_fault(&ev, s.mate);
    assert!(
        matches!(fault, MateFault::PartSelectsAnotherCopy { part: p, .. } if p == part),
        "the fixture reaches the arm, naming the Part: {fault:?}"
    );
    assert_the_mate_is_blamed(&ev, &doc, s.mate, part);
}

/// **When the named node fails in its own right, both rows are loud and
/// neither points at the other** — the counter-case to the two rows
/// above, whose named node happens to evaluate.
///
/// The fault reaches the mate alone (it is raised where the solve reads
/// the mate's references), so blame decides only the mate's own row;
/// the named node's row is the evaluation's own verdict about it.
fn assert_both_loud(
    ev: &Evaluation<f64>,
    doc: &Doc<ProfileProgram>,
    mate: RecipeNodeId,
    named: RecipeNodeId,
) {
    let rows = tree::rows(doc, Some(ev));
    for id in [mate, named] {
        let Some(NodeResult::Failed(error)) = ev.result(id) else {
            panic!("{id:?} must be Failed in the evaluation");
        };
        assert_eq!(
            common::status_of(&rows, id),
            RowStatus::Failed {
                message: error.to_string()
            },
            "{id:?} carries its own words"
        );
    }
    let pointed: Vec<RecipeNodeId> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Poisoned { through, .. } if through == mate || through == named))
        .map(|row| row.id)
        .collect();
    assert_eq!(
        pointed,
        Vec::<RecipeNodeId>::new(),
        "no row is sent anywhere"
    );
}

/// A `Part` re-pointed PAST its pattern's count: the mate refuses with
/// `PartSelectsAnotherCopy`, and the `Part` fails on its own.
#[test]
fn a_part_past_its_patterns_count_fails_beside_the_mate() {
    let tol = Tol::witness();
    let s = copies("msolve3-view-part-past", 1, Some(1), tol);
    let part = s.part.expect("this fixture has a Part");
    let (doc, _) = common::edited(
        &s.doc,
        DocEdit::SetStructuralParam {
            node: part,
            slot: SlotId::Instance,
            expr: Expr::count(5),
        },
        tol,
    );
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &s.opts, tol);
    let fault = mate_fault(&ev, s.mate);
    assert!(
        matches!(fault, MateFault::PartSelectsAnotherCopy { part: p, .. } if p == part),
        "the fixture reaches the arm, naming the Part: {fault:?}"
    );
    assert_both_loud(&ev, &doc, s.mate, part);
}

/// A pattern shrunk to ZERO copies: the mate refuses with
/// `DanglingHead` at the pattern, and the pattern fails on its own.
#[test]
fn a_pattern_of_no_copies_fails_beside_the_mate() {
    let tol = Tol::witness();
    let s = copies("msolve3-view-dangling-zero", 1, None, tol);
    let (doc, _) = common::edited(
        &s.doc,
        DocEdit::SetStructuralParam {
            node: s.pattern,
            slot: SlotId::Count,
            expr: Expr::count(0),
        },
        tol,
    );
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &s.opts, tol);
    let fault = mate_fault(&ev, s.mate);
    assert!(
        matches!(fault, MateFault::DanglingHead { head, .. } if head == s.pattern),
        "the fixture reaches the arm, naming the pattern: {fault:?}"
    );
    assert_both_loud(&ev, &doc, s.mate, s.pattern);
}
