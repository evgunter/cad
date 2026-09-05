//! The verdict bracket under the evaluator's own hard cases: the
//! parallel schedule, a part inside a part, a node-memo hit, a
//! cancelled prefix, the decisions no node's bracket holds, and a
//! failure before the op runs. Every row is about WHAT FRAME RECEIVES
//! WHAT when the bracket stack meets the evaluation service; the
//! bracket's own rules are pinned in `geom_core::k_stats`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    CancelToken, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation, Frame, Node, NodeResult,
    PartResolver, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, content_pin, evaluate,
};
use fixture::{insert, len, on_frame, square, step};
use geom_core::Tol;
use geom_core::k_stats::Bracket;

#[derive(Debug, Default)]
struct Store {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl Store {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for Store {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let doc = self.docs.get(&doc_ref.id).ok_or_else(|| ResolveFailure {
            fault: ResolveFault::Unresolved,
            message: "no such document".to_string(),
        })?;
        Ok(doc.clone())
    }
}

/// A one-solid part: a `side`-wide square extruded 1 tall.
fn part(label: &str, cx: f64, side: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, side / 2.0)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

fn assembly(label: &str, refs: &[DocRef]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for r in refs {
        let (next, id) = insert(doc, Node::instantiate_part(*r));
        doc = next;
        ids.push(id);
    }
    (doc, ids)
}

fn run(doc: &ProfileDoc, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, Tol::witness())
}

/// Two instances, both placed, so both ops do the same work.
fn placed(doc: ProfileDoc, ids: &[RecipeNodeId]) -> ProfileDoc {
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([0.0, 9.0, 0.0]),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame: Frame::translation([9.0, 0.0, 0.0]),
        },
    );
    doc
}

fn logs(ev: &Evaluation<f64>, ids: &[RecipeNodeId]) -> Vec<usize> {
    ids.iter()
        .map(|&id| {
            ev.result(id)
                .and_then(NodeResult::value)
                .expect("the instance evaluates")
                .verdicts
                .len()
        })
        .collect()
}

fn two_instances(label: &str) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions) {
    let mut store = Store::default();
    let doc_ref = store.insert(part(&format!("{label}-part"), 0.0, 1.0), Tol::witness());
    let (doc, ids) = assembly(&format!("{label}-asm"), &[doc_ref, doc_ref]);
    let doc = placed(doc, &ids);
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    (doc, ids, opts)
}

/// **The parallel schedule.** Two instances racing for the part cache
/// under rayon: whichever wins the miss, the instantiate node's log is
/// the same — the same as the other instance's and the same as the
/// sequential schedule's — over eight runs. The shield on the cache's
/// miss path is what this row exists for.
#[test]
fn the_instantiate_log_is_schedule_independent() {
    let (doc, ids, seq_opts) = two_instances("kstats-sched");
    let seq = logs(&run(&doc, &seq_opts), &ids);
    assert_eq!(seq, [466, 466]);
    let par_opts = EvalOptions {
        parallel: true,
        ..seq_opts.clone()
    };
    for i in 0..8 {
        let par = logs(&run(&doc, &par_opts), &ids);
        assert_eq!(par, seq, "parallel run {i} differs from the sequential log");
    }
}

/// **A part inside a part**: two nested miss paths, each shielded. The
/// outer instantiate nodes' logs are their own op's — equal to each
/// other and to the same document under the parallel schedule — and
/// larger than a leaf's instantiate log, because the placed body is
/// the middle assembly's two solids.
#[test]
fn a_part_inside_a_part_keeps_the_outer_log_its_own_under_both_schedules() {
    let mut store = Store::default();
    let leaf = store.insert(part("kstats-nest-leaf", 0.0, 1.0), Tol::witness());
    let (mid, mid_ids) = assembly("kstats-nest-mid", &[leaf, leaf]);
    let mid = placed(mid, &mid_ids);
    let mid_ref = store.insert(mid, Tol::witness());
    let (top, top_ids) = assembly("kstats-nest-top", &[mid_ref, mid_ref]);
    let top = placed(top, &top_ids);
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let seq = logs(&run(&top, &opts), &top_ids);
    assert_eq!(seq[0], seq[1], "the two outer instances differ: {seq:?}");
    assert_eq!(seq[0], 922, "the outer op's own log over a two-solid part");
    let par = logs(
        &run(
            &top,
            &EvalOptions {
                parallel: true,
                ..opts.clone()
            },
        ),
        &top_ids,
    );
    assert_eq!(par, seq);
}

/// **A node-memo hit never runs the op and opens no bracket**: the
/// reused value carries the PRIOR run's log, not an empty one.
#[test]
fn a_node_memo_hit_carries_the_priors_log() {
    let (doc, ids, opts) = two_instances("kstats-memo");
    let first = run(&doc, &opts);
    let again = evaluate::<f64>(
        &doc,
        Some(&first),
        &CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    assert_eq!(again.recomputed, 0, "every node was a memo hit");
    assert_eq!(logs(&again, &ids), logs(&first, &ids));
    assert_eq!(logs(&again, &ids), [466, 466]);
}

/// **A cancelled prefix leaves no frame behind**: the run after a
/// pre-cancelled one — and after one cancelled between levels under
/// the parallel schedule — records exactly what a clean run records.
/// A frame left open by the cancelled run would sit under every later
/// bracket on that thread and receive the later run's out-of-bracket
/// decisions; the later run's node logs would still be its own, so the
/// row also reads the frame directly.
#[test]
fn a_cancelled_run_leaves_the_next_runs_logs_and_the_frame_stack_intact() {
    let (doc, ids, opts) = two_instances("kstats-cancel");
    let clean = logs(&run(&doc, &opts), &ids);
    for parallel in [false, true] {
        let opts = EvalOptions {
            parallel,
            ..opts.clone()
        };
        let token = CancelToken::new();
        token.cancel();
        let canceled = evaluate::<f64>(&doc, None, &token, &opts, Tol::witness());
        assert_eq!(canceled.outcome, editor_core::EvalOutcome::Canceled);
        // What the next run's out-of-bracket decisions land in is a
        // fresh outer frame: a leaked frame would sit beneath it and
        // change nothing here, so the direct read is the node logs.
        let outer = Bracket::open();
        let after = logs(&run(&doc, &opts), &ids);
        let outside = outer.finish();
        assert_eq!(
            after, clean,
            "parallel={parallel}: a run after a cancelled one differs"
        );
        assert!(
            outside.verdicts.is_empty() && outside.escalations.is_empty(),
            "the assembly decides nothing outside its nodes' brackets: {outside:?}"
        );
    }
}

/// **What no node's bracket holds.** Evaluated inside an outer bracket,
/// a document's decisions made before any node's bracket opens — the
/// profile pre-pass, the whole-document mate solve — land in that
/// outer frame: 75 for the part (its profile node's pre-pass), none
/// for the assembly (the part's are shielded on the cache's miss
/// path). The counts are the finding
/// `work/issues/bracket-scope-is-run-op-not-the-node.md` measures.
#[test]
fn the_decisions_outside_every_node_bracket_are_the_pre_pass_ones() {
    let part_doc = part("kstats-outside-part", 0.0, 1.0);
    let outer = Bracket::open();
    let ev = run(&part_doc, &EvalOptions::default());
    let outside = outer.finish();
    let on_nodes: usize = ev
        .nodes
        .values()
        .filter_map(NodeResult::value)
        .map(|v| v.verdicts.len())
        .sum();
    assert_eq!(on_nodes, 724);
    assert_eq!(outside.verdicts.len(), 75, "{:?}", outside.verdicts);
    let mut histogram: BTreeMap<&str, usize> = BTreeMap::new();
    for v in &outside.verdicts {
        *histogram.entry(v.predicate).or_default() += 1;
    }
    assert_eq!(histogram["chord_side"], 28);
    assert_eq!(histogram["line_span"], 8);

    let (doc, ids, opts) = two_instances("kstats-outside");
    let outer = Bracket::open();
    let ev = run(&doc, &opts);
    let outside = outer.finish();
    assert_eq!(logs(&ev, &ids), [466, 466]);
    assert!(
        outside.verdicts.is_empty() && outside.escalations.is_empty(),
        "{outside:?}"
    );
}

/// **A failure before the op runs carries no escalations**: a document
/// evaluated at a process ε other than the one it recorded refuses on
/// every node before any op runs (the D4 door), no bracket was open,
/// and `NodeError::escalations` is empty rather than whatever an outer
/// frame held.
#[test]
fn a_pre_op_failure_has_empty_escalations() {
    let part_doc = part("kstats-pre-op", 0.0, 1.0);
    let (part_doc, _) = step(
        part_doc,
        DocEdit::SetTolerance {
            eps: Tol::witness().eps() * 2.0,
        },
    );
    let outer = Bracket::open();
    let ev = run(&part_doc, &EvalOptions::default());
    drop(outer.finish());
    let failed: Vec<_> = ev.nodes.values().filter_map(NodeResult::error).collect();
    assert_eq!(
        failed.len(),
        ev.nodes.len(),
        "every node refuses at the ε door"
    );
    for err in failed {
        assert!(
            !matches!(err.kind, editor_core::NodeErrorKind::Escalated { .. }),
            "{}",
            err.kind
        );
        assert!(err.escalations.is_empty());
    }
}
