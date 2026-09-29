//! Review probes (lane `partroot-rev`) over PR 3459's carried refusal.
//! Each row states the claim it measures and asserts what the tree
//! does; a red row is a finding.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::print_stderr)]

use crate::common;
use crate::fixture;

use fixture::resolver::{PartStore, in_part, with_resolver};
use pncad::document::{
    Alignment, AxisSense, CancelToken, Doc, DocEdit, DocRef, DocumentId, Expr, MateFrame,
    MatePrimitive, Node, NodeErrorKind, NodeResult, PatternKind, ProfileDoc, ProfileProgram,
    RecipeNodeId, SlotId, content_pin, evaluate,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{CapEnd, ContactClass, EntityKind, RoleSeg};
use pncad::workspace::Workspace;
use viewer::parts::PartFiles;
use viewer::session::{DocSession, SessionOp};
use viewer::tree::{self, RowStatus};

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

fn reference(doc: &ProfileDoc, tol: Tol) -> DocRef {
    DocRef {
        id: doc.id(),
        pin: content_pin(doc, tol).expect("the pin computes"),
    }
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    }
}

/// msolve3's fold-path document (a mate onto copy 1 of a pattern whose
/// direction is 1e200), over the given leg and top references.
fn fold_doc(label: &str, leg: DocRef, top: DocRef, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = ProfileDoc::empty(DocumentId::derive(label), tol);
    // The cap first, so it is the first root the product door reads.
    let (doc, cap) = common::inserted(&doc, Node::instantiate_part(top), tol);
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
    let (doc, mate) = common::inserted(
        &doc,
        Node::Mate {
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
        tol,
    );
    (doc, pattern, mate)
}

/// **Probe A — a mate's carried line inside a part names no file.**
/// A part (`sub.pncad`) whose mate refuses `PlacerRefused`, instantiated
/// by an assembly. Every carried line whose node number is `sub.pncad`'s
/// must say so; `carried_lines` prefixes a file only at a `Part` level.
#[test]
fn probe_a_placer_line_inside_a_part_names_the_parts_file() {
    let tol = Tol::witness();
    let dir = common::tempdir("partroot-rev-a");
    let mut store = Workspace::open(&dir).expect("the empty workspace opens");
    let leg = block("partroot-rev-a-leg", tol);
    let top = block("partroot-rev-a-top", tol);
    store.save_at(&leg, "leg.pncad", tol).expect("stores");
    store.save_at(&top, "top.pncad", tol).expect("stores");
    let (sub, pattern, mate) = fold_doc(
        "partroot-rev-a-sub",
        reference(&leg, tol),
        reference(&top, tol),
        tol,
    );
    eprintln!("sub roots: {:?} pattern {pattern:?} mate {mate:?}", sub.roots());
    {
        let mut ps = PartStore::default();
        ps.insert(leg.clone(), tol);
        ps.insert(top.clone(), tol);
        let ev = evaluate::<f64>(&sub, None, &CancelToken::new(), &with_resolver(ps), tol);
        for id in sub.order() {
            eprintln!("  sub {id:?}: {:?}", ev.result(*id).map(|r| match r {
                NodeResult::Ok(_) => "ok".to_string(),
                NodeResult::Failed(e) => e.to_string(),
                NodeResult::Poisoned { through } => format!("poisoned via {through:?}"),
            }));
        }
    }
    store.save_at(&sub, "sub.pncad", tol).expect("stores");
    let mut assembly = Doc::empty_derived("partroot-rev-a-asm", tol);
    let instance = common::insert_into(
        &mut assembly,
        Node::instantiate_part(reference(&sub, tol)),
        tol,
    );
    let path = store
        .save_at(&assembly, "assembly.pncad", tol)
        .expect("stores");
    let mut session = DocSession::inline(Doc::empty_derived("partroot-rev-a-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let rows = session.tree_rows();
    let row = rows.iter().find(|r| r.id == instance).expect("row");
    let RowStatus::Failed { message, carried } = &row.status else {
        panic!("the instance fails: {:?}", row.status);
    };
    eprintln!("PROBE-A\n{message}\n{}", carried.join("\n"));
    assert!(
        carried.len() >= 2,
        "the part's root is the mate, carrying the placer: {carried:#?}"
    );
    for (level, line) in carried.iter().enumerate() {
        assert!(
            line.starts_with("in sub.pncad, "),
            "level {level}'s node number is sub.pncad's, and the line does not say so: {line}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// **Probe B — the non-fold path draws the placer's refusal twice.**
/// msolve3's count-overflow fixture: the fault reaches the mate alone
/// and the pattern fails in its own right. Deviation 1 extends
/// `carried()` to `PlacerRefused` on every path, so the mate row now
/// draws the pattern's own refusal as a carried line while the pattern
/// row draws the same refusal.
#[test]
fn probe_b_non_fold_placer_refusal_is_drawn_once() {
    let tol = Tol::witness();
    let mut store = PartStore::default();
    let leg = store.insert(block("partroot-rev-b-leg", tol), tol);
    let top = store.insert(block("partroot-rev-b-top", tol), tol);
    let doc: Doc<ProfileProgram> = ProfileDoc::empty(DocumentId::derive("partroot-rev-b"), tol);
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
    let (doc, cap) = common::inserted(&doc, Node::instantiate_part(top), tol);
    let (doc, mate) = common::inserted(
        &doc,
        Node::Mate {
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
        tol,
    );
    let overflowing =
        Expr::mul(Expr::count(i64::MAX), Expr::count(2)).expect("a count times a count");
    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetStructuralParam {
            node: pattern,
            slot: SlotId::Count,
            expr: overflowing,
        },
        tol,
    );
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &with_resolver(store), tol);
    let rows = tree::rows(&doc, Some(&ev), &PartFiles::default());
    let status = |id| common::status_of(&rows, id);
    let RowStatus::Failed { carried, message } = status(mate) else {
        panic!("the mate fails");
    };
    let RowStatus::Failed {
        message: pattern_message,
        ..
    } = status(pattern)
    else {
        panic!("the pattern fails in its own right on this path");
    };
    eprintln!("PROBE-B mate: {message}\n  carried: {carried:?}\n  pattern: {pattern_message}");
    assert!(
        !carried.contains(&pattern_message),
        "the pattern's refusal is drawn on the pattern's row AND as the mate row's carried line"
    );
}

/// **Probe C — a part's root poisoned, not failed.** The part's root is
/// a transform over an extrude that refuses. What does the instance row
/// carry? (Filed as a-parts-poisoned-root-drops-the-failure-that-poisoned-it.)
#[test]
fn probe_c_poisoned_part_root_carries_nothing() {
    let tol = Tol::witness();
    let dir = common::tempdir("partroot-rev-c");
    let mut store = Workspace::open(&dir).expect("opens");
    let (broken, _, _) = common::broken_document(tol);
    store.save_at(&broken, "broken.pncad", tol).expect("stores");
    let mut assembly = Doc::empty_derived("partroot-rev-c-asm", tol);
    let instance = common::insert_into(
        &mut assembly,
        Node::instantiate_part(reference(&broken, tol)),
        tol,
    );
    let path = store.save_at(&assembly, "asm.pncad", tol).expect("stores");
    let mut session = DocSession::inline(Doc::empty_derived("partroot-rev-c-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let rows = session.tree_rows();
    let row = rows.iter().find(|r| r.id == instance).expect("row");
    let RowStatus::Failed { message, carried } = &row.status else {
        panic!("fails: {:?}", row.status);
    };
    eprintln!("PROBE-C {message}\n  carried: {carried:?}");
    assert!(carried.is_empty(), "measured: the poisoned root carries nothing");
    let _ = std::fs::remove_dir_all(&dir);
}

/// **Probe D — the file-name prefix is outside every budget row.** The
/// kernel chain row holds `line_at` (no prefix); the viewer row holds a
/// short extrude refusal. Here: the tree's carried line for a part whose
/// root refusal is long, with the file unknown (the NO_FILE wording).
#[test]
fn probe_d_prefix_words_on_a_carried_line() {
    let words = |s: &str| s.split_whitespace().count();
    let inner = || NodeErrorKind::Part {
        doc_ref: DocRef {
            id: DocumentId::derive("partroot-rev-d-inner"),
            pin: pncad::document::ContentPin([7u8; 32]),
        },
        fault: pncad::document::PartFault::NoResolver,
    };
    let kind = NodeErrorKind::Part {
        doc_ref: DocRef {
            id: DocumentId::derive("partroot-rev-d"),
            pin: pncad::document::ContentPin([9u8; 32]),
        },
        fault: pncad::document::PartFault::PartRootFailed {
            node: RecipeNodeId(2),
            refusal: inner().into(),
        },
    };
    let lines = tree::carried_lines(&kind, &PartFiles::default());
    let own = pncad::document::NodeRefusal::from(inner()).line_at(RecipeNodeId(2));
    eprintln!("PROBE-D line: {} words vs own {} words\n{}", words(&lines[0]), words(&own), lines[0]);
    assert!(
        words(&lines[0]) <= words(&own),
        "the drawn line is the node's own line plus {} words of prefix",
        words(&lines[0]) - words(&own)
    );
}

/// **Probe E — a part renamed on disk while the assembly is open.**
#[test]
fn probe_e_rename_while_open() {
    let tol = Tol::witness();
    let dir = common::tempdir("partroot-rev-e");
    let mut store = Workspace::open(&dir).expect("opens");
    let boss = block("partroot-rev-e-boss", tol);
    let boss_ref = reference(&boss, tol);
    store.save_at(&boss, "boss.pncad", tol).expect("stores");
    let mut assembly = Doc::empty_derived("partroot-rev-e-asm", tol);
    let instance = common::insert_into(&mut assembly, Node::instantiate_part(boss_ref), tol);
    let path = store.save_at(&assembly, "asm.pncad", tol).expect("stores");
    let mut session = DocSession::inline(Doc::empty_derived("partroot-rev-e-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let pose = |s: &DocSession| {
        s.tree_rows()
            .into_iter()
            .find(|r| r.id == instance)
            .and_then(|r| r.pose)
    };
    let before = pose(&session);
    std::fs::rename(dir.join("boss.pncad"), dir.join("renamed.pncad")).expect("rename");
    let after_rename = pose(&session);
    std::fs::remove_file(dir.join("renamed.pncad")).expect("delete");
    let after_delete = pose(&session);
    eprintln!("PROBE-E before {before:?} after-rename {after_rename:?} after-delete {after_delete:?}");
    assert_eq!(
        after_delete.as_deref(),
        Some(PartFiles::NO_FILE),
        "a deleted part file is still named until the next landing"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// **Probe F — the instance row before anything lands.**
#[test]
fn probe_f_pose_before_landing() {
    let tol = Tol::witness();
    let mut assembly = Doc::empty_derived("partroot-rev-f", tol);
    let (boss, _, _) = common::broken_document(tol);
    let instance = common::insert_into(
        &mut assembly,
        Node::instantiate_part(reference(&boss, tol)),
        tol,
    );
    let rows = tree::rows(&assembly, None, &PartFiles::default());
    let pose = rows.iter().find(|r| r.id == instance).and_then(|r| r.pose.clone());
    eprintln!("PROBE-F unevaluated pose: {pose:?}");
    let _ = NodeResult::<f64>::Poisoned {
        through: RecipeNodeId(0),
    };
}
