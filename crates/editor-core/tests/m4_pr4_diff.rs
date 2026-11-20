//! M4 PR 4 spec D2: the verdict-vector diff engine — one engine, one
//! call signature, cause-agnostic. Identical runs diff empty; no-flip
//! parameter motion diffs empty (memo transfer included); an edit whose
//! runs part at their pruning names no flip read off different pairs;
//! structural edits name no fake flip and surface as status rows.
//!
//! SWEEP-STRATEGY NOTE (Ev's 2026-07-29 ruling): this file's pins
//! are about diff/resolve engine behavior GIVEN verdicts, so its
//! evaluator deliberately runs the idealized (verdict-rich) sweep;
//! the production-path degradation is pinned in `m4_pr4_banked`
//! (both strategies side by side) and in the re-pinned `m4_pr4_ci`
//! golden.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, DocEdit, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId, RunStatus,
    SlotId, diff_verdicts, evaluate,
};
use fixture::{insert, len, on_frame, step};
use geom_core::Tol;

/// Idealized (brute-force) boolean sweep since M5 PR 8: this file
/// pins the DIFF/RESOLVE engine's semantics, whose evidence substrate
/// is the verdict log — the idealized sweep keeps interaction-boundary
/// scenarios (overlapping ↔ disjoint) verdict-rich on both sides.
/// The realized sweep prunes the disjoint side's pair space empty
/// (its job); that production-path degradation is pinned in
/// `m4_pr4_banked` (both strategies) — see `fixture/pr4.rs`'s note.
fn run(doc: &editor_core::ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    let opts = EvalOptions {
        boolean_sweep: topo::SweepStrategy::Idealized,
        ..EvalOptions::default()
    };
    evaluate::<f64>(doc, prior, &CancelToken::new(), &opts, Tol::witness())
}

/// A rectangular block: profile on the plane z = `z0`, extruded `dz`.
fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    )
}

/// The sliding-union document: A fixed, B placed against A by a
/// placement whose slide along x is the edit knob, then A ∪ B.
struct Slide {
    doc: ProfileDoc,
    placement: RecipeNodeId,
    union: RecipeNodeId,
}

fn slide_union(tx: f64) -> Slide {
    let doc = ProfileDoc::empty_derived("m4_pr4_diff", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, placement) = fixture::slid(doc, a, b0, [tx, 0.0, 0.0]);
    // The B side is read at the PLACEMENT, the boolean's operand; the
    // copy wears `b0`'s names under its `Placed` layer.
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), placement.into()]),
        },
    );
    Slide {
        doc,
        placement,
        union,
    }
}

fn slide_to(s: &Slide, tx: f64) -> ProfileDoc {
    let slot = fixture::value_slot(
        &s.doc,
        s.placement,
        editor_core::Coord::slide(editor_core::TargetAxis::X),
    );
    let (doc, _) = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.placement,
            slot,
            value: len(tx).into(),
            fresh: Vec::new(),
        },
    );
    doc
}

#[test]
fn identical_runs_diff_empty() {
    let s = slide_union(0.5);
    let ev1 = run(&s.doc, None);
    let ev2 = run(&s.doc, Some(&ev1));
    let flips = diff_verdicts(&ev1, &ev2);
    assert!(
        flips.is_empty(),
        "identical runs must diff empty: {flips:?}"
    );
    // And without memo transfer (independent recompute) too — D9
    // determinism makes the logs bit-equal, not just Arc-equal.
    let ev3 = run(&s.doc, None);
    assert!(diff_verdicts(&ev1, &ev3).is_empty());
}

#[test]
fn no_flip_parameter_motion_diffs_empty() {
    let s = slide_union(0.5);
    let ev1 = run(&s.doc, None);
    let doc2 = slide_to(&s, 0.25); // still overlapping: same verdicts
    let ev2 = run(&doc2, Some(&ev1));
    let flips = diff_verdicts(&ev1, &ev2);
    assert!(
        flips.is_empty(),
        "no-flip motion must diff empty: {flips:?}"
    );
}

#[test]
fn a_disjoining_edit_names_no_flip_read_off_other_pairs() {
    let s = slide_union(0.5);
    let ev1 = run(&s.doc, None);
    let doc2 = slide_to(&s, 2.5); // overlapping -> disjoint
    let ev2 = run(&doc2, Some(&ev1));
    // Both runs' first choice is a parallelism reading of two face
    // planes, which no slide can turn: the overlapping run's is a pair
    // the union's box query handed on, the disjoint run's another pair,
    // after a query that handed on none. The candidate marks show the
    // two runs parting there, so the engine names no turn, rather than
    // a "flip" between decisions about two different pairs.
    let union = ev1.value(s.union).expect("the overlapping union builds");
    assert!(
        union
            .marks
            .iter()
            .any(|m| matches!(m.item, geom_core::k_stats::MarkItem::Candidate(_))),
        "the union's box queries hand on candidates: {:?}",
        union.marks.len()
    );
    let flips = diff_verdicts(&ev1, &ev2);
    assert!(flips.is_empty(), "no turn on the record: {flips:?}");
    // Deterministic: diffing again yields the identical FlipSet.
    assert_eq!(flips, diff_verdicts(&ev1, &ev2));
}

/// **An n-ary union whose hull test pairs other members names no flip
/// read off the other pair.** Members `[C1, B, C2]`, `C1` and `C2`
/// congruent and apart; `B` slides from across `C1` to across `C2`. The
/// union's hull test (`topo`'s `judged_pairs`) runs the pair boolean
/// `(C1, B)` in one run and `(B, C2)` in the other: congruent pairs,
/// whose decisions can read alike or differ by place, about two different
/// pairs either way. The test's outcomes are marked, so the two runs
/// part at the first pair the test judged differently, and no flip is
/// named. Read with no mark, the choices alone name a fake turn here
/// (`bool_vertex_face_side`, decided about the other pair), and the row
/// holds that the hull test's marks alone remove it.
#[test]
fn an_nary_union_whose_hull_test_pairs_other_members_names_no_flip() {
    let doc = ProfileDoc::empty_derived("m4_pr4_diff", Tol::witness());
    let (doc, c1) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c2_block) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    // Both are placed in `c1`'s space, as a union's members must be.
    let (doc, c2) = fixture::slid(doc, c1, c2_block, [3.0, 0.0, 0.0]);
    let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, placement) = fixture::slid(doc, c1, b0, [0.5, 0.25, 0.25]);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![c1.into(), placement.into(), c2.into()]),
        },
    );
    let s = Slide {
        doc,
        placement,
        union,
    };
    let ev1 = run(&s.doc, None);
    let ev2 = run(&slide_to(&s, 3.5), Some(&ev1));
    let marked = |ev: &Evaluation<f64>| {
        ev.value(union)
            .expect("the union builds")
            .marks
            .iter()
            .filter(|m| m.site == "nary_hulls_meet")
            .map(|m| m.item)
            .collect::<Vec<_>>()
    };
    let (m1, m2) = (marked(&ev1), marked(&ev2));
    assert!(!m1.is_empty(), "the hull test marks each pair it judges");
    assert_ne!(m1, m2, "the hull test pairs other members in the two runs");
    // The union's two paths part, at the hull test's marks: read with
    // those marks alone beside the verdicts, the paths still part.
    let (v1, v2) = (
        ev1.value(union).expect("the union builds"),
        ev2.value(union).expect("the union builds"),
    );
    let hull_only = |v: &editor_core::NodeValue<f64>| -> Vec<geom_core::k_stats::Mark> {
        v.marks
            .iter()
            .filter(|m| m.site == "nary_hulls_meet")
            .copied()
            .collect()
    };
    let (h1, h2) = (hull_only(v1), hull_only(v2));
    let turn = |m1: &[geom_core::k_stats::Mark], m2: &[geom_core::k_stats::Mark]| {
        editor_core::resolve::first_turn(
            editor_core::resolve::Path {
                verdicts: &v1.verdicts,
                marks: m1,
            },
            editor_core::resolve::Path {
                verdicts: &v2.verdicts,
                marks: m2,
            },
        )
    };
    assert_eq!(
        turn(&v1.marks, &v2.marks).mark().map(|(site, _)| site),
        Some("nary_hulls_meet"),
        "the union's runs part at the hull test: {:?}",
        turn(&v1.marks, &v2.marks)
    );
    assert_eq!(
        turn(&h1, &h2).mark().map(|(site, _)| site),
        Some("nary_hulls_meet"),
        "the hull test's marks alone part them: {:?}",
        turn(&h1, &h2)
    );
    // Read with no mark at all, the choices alone name a turn: the
    // first pair each run judged decides one predicate with the other
    // sign, about two different pairs. That fake turn is what the hull
    // test's marks remove.
    assert!(
        matches!(turn(&[], &[]), editor_core::resolve::FirstTurn::Turned(_)),
        "without marks the choices read a turn between two pairs: {:?}",
        turn(&[], &[])
    );
    let flips = diff_verdicts(&ev1, &ev2);
    assert!(
        flips.nodes.get(&union).is_none_or(|d| d.flip.is_none()),
        "no turn at the union: {flips:?}"
    );
}

#[test]
fn structural_count_edit_names_no_fake_flip() {
    // A union over a row of overlapping blocks whose index's count is
    // the knob: the union's decision SEQUENCE changes length (one
    // member fewer to judge). Its choices part where one run judges a
    // member the other does not, which is no turn of a sign, so no flip
    // is named at all.
    let doc = ProfileDoc::empty_derived("m4_pr4_diff", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, k, row) = crate::fixture::linear(doc, body, [1.0, 0.0, 0.0], 0.5, 3);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Family(row.into()),
        },
    );
    let ev1 = run(&doc, None);
    let edit = crate::fixture::recount_edit(&doc, k, editor_core::Formula::count(2));
    let (doc2, _) = step(doc, edit);
    let ev2 = run(&doc2, Some(&ev1));
    let flips = diff_verdicts(&ev1, &ev2);
    assert_eq!(flips.flip_at(union), None, "no fake flip: {flips:?}");
    assert!(flips.is_empty(), "no node's standing moved: {flips:?}");
}

#[test]
fn failure_transitions_surface_as_status_rows() {
    // An extrude whose depth is the knob: zeroing it fails the extrude
    // and poisons its consumer.
    let doc = ProfileDoc::empty_derived("m4_pr4_diff", Tol::witness());
    let (doc, body) = block(doc, (0.0, 2.0), (0.0, 2.0), 2.0);
    let tool = fixture::plane_on(&doc, body, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: body.into(),
            tool,
        },
    );
    let ev1 = run(&doc, None);
    let (doc2, _) = step(
        doc,
        DocEdit::SetParam {
            node: body,
            slot: SlotId::Distance,
            value: len(0.0).into(),
            fresh: Vec::new(),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    let flips = diff_verdicts(&ev1, &ev2);
    let d_plane = flips.nodes.get(&body).expect("extrude delta");
    assert_eq!(d_plane.old_status, RunStatus::Ok);
    assert_eq!(d_plane.new_status, RunStatus::Failed);
    let d_split = flips.nodes.get(&split).expect("split delta");
    assert_eq!(d_split.old_status, RunStatus::Ok);
    assert_eq!(d_split.new_status, RunStatus::Poisoned);
}

// ---- Review Finding 9: the parallel schedule preserves verdict ----
// ---- logs (adopted reviewer probe) ----

#[test]
fn parallel_schedule_preserves_verdict_logs() {
    // PR 2's D9 cross-check compares body fingerprints only; PR 4
    // added verdict logs to node values, and the verdict sink is
    // thread-local — pin that the rayon lane records IDENTICAL logs
    // per node (the diff engine's inputs are schedule-independent).
    let doc = ProfileDoc::empty_derived("m4_pr4_diff", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 1.0);
    let (doc, b) = fixture::slid(doc, a, b, [0.0; 3]);
    let (doc, _uni) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let seq = run(&doc, None);
    let par = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions {
            epoch: editor_core::Epoch::mint(),
            parallel: true,
            boolean_sweep: topo::SweepStrategy::Idealized,
            ..EvalOptions::default()
        },
        Tol::witness(),
    );
    for &id in &seq.order {
        match (seq.value(id), par.value(id)) {
            (Some(sv), Some(pv)) => {
                assert_eq!(
                    sv.verdicts, pv.verdicts,
                    "verdict log diverged under the parallel schedule at {id:?}"
                );
            }
            (None, None) => {}
            other => panic!("status diverged at {id:?}: {:?}", other.0.is_some()),
        }
    }
    // And the engine sees them as identical runs.
    assert!(diff_verdicts(&seq, &par).is_empty());
}
