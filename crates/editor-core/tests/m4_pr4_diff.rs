//! M4 PR 4 spec D2: the verdict-vector diff engine — one engine, one
//! call signature, cause-agnostic. Identical runs diff empty; no-flip
//! parameter motion diffs empty (memo transfer included); a
//! flip-inducing edit reports exactly the flipped predicates,
//! localized to the deciding node; structural edits surface as
//! divergence/status rows; and the report is the `SetTolerance`
//! audit surface (H4 — PR 6 reuses it untouched).
//!
//! SWEEP-STRATEGY NOTE (Ev's 2026-07-29 ruling): this file's pins
//! are about diff/resolve engine behavior GIVEN verdicts, so its
//! evaluator deliberately runs the idealized (verdict-rich) sweep;
//! the production-path degradation is pinned in `m4_pr4_banked`
//! (both strategies side by side) and in the re-pinned `m4_pr4_ci`
//! golden.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, CancelToken, DocEdit, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId,
    RunStatus, SlotId, diff_verdicts, evaluate,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::Tol;

/// Idealized (brute-force) boolean sweep since M5 PR 8: this file
/// pins the DIFF/RESOLVE engine's semantics, whose evidence substrate
/// is the verdict log — the idealized sweep keeps interaction-boundary
/// scenarios (overlapping ↔ disjoint) verdict-rich on both sides.
/// The realized sweep prunes the disjoint side's pair space empty
/// (its job); that production-path degradation is pinned in
/// `m4_pr4_banked` (both strategies) — see `fixture/pr4.rs`'s note.
fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
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
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

/// The sliding-union document: A fixed, B placed by a Transform whose
/// x-translation is the edit knob, then A ∪ B.
struct Slide {
    doc: ProfileDoc,
    transform: RecipeNodeId,
    union: RecipeNodeId,
}

fn slide_union(tx: f64) -> Slide {
    let doc = ProfileDoc::empty_derived("m4_pr4_diff", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, transform) = insert(
        doc,
        Node::Transform {
            input: b0,
            translation: [len(tx), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    // M4 PR 5: the sliding overlap's flush planes are declared.
    let (doc, decl) = fixture::declare_x_offset_flush(doc, a, b0);
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b: transform,
            declare: Some(decl),
        },
    );
    Slide {
        doc,
        transform,
        union,
    }
}

fn slide_to(s: &Slide, tx: f64) -> ProfileDoc {
    let (doc, _) = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.transform,
            slot: SlotId::Translation(editor_core::Axis3::X),
            expr: len(tx),
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
fn flip_edit_reports_flips_localized_to_the_deciding_node() {
    let s = slide_union(0.5);
    let ev1 = run(&s.doc, None);
    let doc2 = slide_to(&s, 2.5); // overlapping -> disjoint
    let ev2 = run(&doc2, Some(&ev1));
    let flips = diff_verdicts(&ev1, &ev2);
    assert!(!flips.is_empty(), "the flip edit must report");
    // Every recorded delta lives at the boolean (the deciding node):
    // the operands' derivations are untouched (N7 localization), and
    // the transform's own decisions (axis normalization) do not flip.
    for (&node, delta) in &flips.nodes {
        assert_eq!(node, s.union, "delta outside the boolean: {delta:?}");
    }
    // The report surface (the SetTolerance audit shape): exactly the
    // flipped predicates, deterministically ordered, every entry a
    // real k_stats name with from != to.
    let report = flips.report();
    assert!(!report.is_empty());
    for (node, f) in &report {
        assert_eq!(*node, s.union);
        assert!(!f.predicate.is_empty());
        assert_ne!(f.from, f.to, "a flip must change sign: {f:?}");
    }
    // Deterministic: diffing again yields the identical FlipSet.
    assert_eq!(flips, diff_verdicts(&ev1, &ev2));
}

#[test]
fn structural_count_edit_surfaces_as_divergence_not_fake_flips() {
    // A linear pattern whose count is the knob: the pattern node's
    // decision SEQUENCE changes length (one direction normalization
    // per placed instance), which must surface as divergence, never
    // as sign flips of unrelated positions.
    let doc = ProfileDoc::empty_derived("m4_pr4_diff", Tol::witness());
    let (doc, body) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: body,
            count: editor_core::Expr::count(3),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let ev1 = run(&doc, None);
    let (doc2, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: pattern,
            slot: SlotId::Count,
            expr: editor_core::Expr::count(2),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    let flips = diff_verdicts(&ev1, &ev2);
    let delta = flips.nodes.get(&pattern).expect("pattern delta");
    assert!(delta.flips.is_empty(), "no fake flips: {delta:?}");
    assert!(!delta.diverged.is_empty(), "count change is divergence");
    assert_eq!(flips.nodes.len(), 1, "localized to the pattern node");
}

#[test]
fn failure_transitions_surface_as_status_rows() {
    // A datum plane whose normal is the knob: zeroing it fails the
    // datum and poisons its consumer.
    let doc = ProfileDoc::empty_derived("m4_pr4_diff", Tol::witness());
    let (doc, body) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 2.0);
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(1.0)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: body,
            tool: plane,
        },
    );
    let ev1 = run(&doc, None);
    let (doc2, _) = step(
        doc,
        DocEdit::SetParam {
            node: plane,
            slot: SlotId::Normal(editor_core::Axis3::Z),
            expr: scl(0.0),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    let flips = diff_verdicts(&ev1, &ev2);
    let d_plane = flips.nodes.get(&plane).expect("datum delta");
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
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, _uni) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
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
