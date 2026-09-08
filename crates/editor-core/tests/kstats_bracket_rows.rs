//! The verdict bracket under the evaluator's own hard cases. Every row
//! is about WHAT FRAME RECEIVES WHAT when the bracket stack meets the
//! evaluation service; the bracket's own rules are pinned in
//! `geom_core::k_stats`. The rows:
//!
//! - the parallel schedule (an instantiate log is schedule-independent);
//! - a part inside a part (the outer log is its own under both schedules);
//! - a node-memo hit carries the prior's log;
//! - a cancelled prefix leaves the frame stack intact;
//! - every decision the part makes lands on one of its nodes' brackets,
//!   and the assembly decides nothing outside its instances';
//! - the Profile node's log opens with the precompute and the op's follow;
//! - a memo hit runs the precompute in a frame it drops (f64, Interval),
//!   and a hit whose prefix disagrees with the reused log refuses loud;
//! - three pre-op refusals told apart: the D4 ε door (no frame is ever
//!   opened), an `Expr` refusal (frame open, nothing decided, empty
//!   escalations), and a precompute that escalates before failing (its
//!   escalation rides the refusal).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, DocParamValue, DocRef, DocumentId, EvalOptions,
    EvalScalar, Evaluation, Expr, Frame, LoopProgram, Node, NodeResult, ParamName, PartResolver,
    ProfileDoc, ProfileProgram, RecipeNodeId, ResolveFailure, ResolveFault, content_pin, evaluate,
};
use fixture::{frame, insert, len, on_frame, square, step};
use geom_core::Band;
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

/// The part's one Profile node.
fn profile_node(doc: &ProfileDoc) -> RecipeNodeId {
    doc.order()
        .iter()
        .copied()
        .find(|&id| matches!(doc.node(id), Some(Node::Profile(_))))
        .expect("the part has a profile")
}

/// Every node's verdict count, keyed by node.
fn per_node(ev: &Evaluation<f64>) -> BTreeMap<RecipeNodeId, usize> {
    ev.nodes
        .iter()
        .filter_map(|(&id, r)| r.value().map(|v| (id, v.verdicts.len())))
        .collect()
}

/// The decisions the part's profile pre-pass makes on the one-solid
/// part: its plane's two axes, the program's four junctions, and the
/// f64 validation of the assembled profile.
const PRE_PASS: usize = 75;
/// The one-solid part's log sizes by node: frame, profile, extrude.
const FRAME_LOG: usize = 2;
const PROFILE_LOG: usize = 144;
const EXTRUDE_LOG: usize = 653;

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

/// **A node-memo hit never runs the op**: the reused value carries the
/// PRIOR run's log, not an empty one and not the frame the hit run
/// opened for its pre-pass (that frame's fate is the row below).
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

/// **Every decision the part makes lands on one of its nodes'
/// brackets.** Evaluated inside an outer bracket, the part leaves that
/// frame empty: the profile's f64 precompute — the plane's axes, the
/// program's replay, the validation — decides on the Profile node's
/// behalf and its log holds those decisions ahead of its op's, so the
/// Profile node's count is the precompute's plus the op's.
#[test]
fn every_decision_the_part_makes_lands_on_one_of_its_nodes_brackets() {
    let part_doc = part("kstats-outside-part", 0.0, 1.0);
    let outer = Bracket::open();
    let ev = run(&part_doc, &EvalOptions::default());
    let outside = outer.finish();
    assert!(
        outside.verdicts.is_empty() && outside.escalations.is_empty(),
        "the part decides nothing outside its nodes' brackets: {outside:?}"
    );
    let counts = per_node(&ev);
    let order = part_doc.order();
    assert_eq!(
        counts,
        BTreeMap::from([
            (order[0], FRAME_LOG),
            (order[1], PROFILE_LOG),
            (order[2], EXTRUDE_LOG),
        ]),
        "one log per node, the Profile node's carrying its precompute: {counts:?}"
    );
}

/// **The assembly decides nothing outside its instances' brackets**:
/// the part's decisions are shielded on the cache's miss path, and
/// each instance's log is its own op's.
#[test]
fn the_assembly_decides_nothing_outside_its_instances_brackets() {
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

/// **The Profile node's log opens with the precompute and the op's
/// decisions follow.** One frame, in the order made: the plane's two
/// axis decisions first, then the program's replay, then the f64
/// validation, then the op's. The histogram moves legitimately only
/// when `profile::validate`'s probes change (a predicate added, a
/// probe count per segment pair changed) or the fixture does. The last
/// assertion pins TODAY's double run — under the pinned lift at f64
/// the op validates the embedded f64 form again, decision for decision
/// (`work/eval/profile-node-log-holds-the-f64-validation-twice-under-the-pinned-lift.md`)
/// — and is the line to drop when that unit lands, not a property of
/// the log.
#[test]
fn the_profile_nodes_log_opens_with_the_pre_pass_and_the_ops_decisions_follow() {
    let part_doc = part("kstats-order-part", 0.0, 1.0);
    let ev = run(&part_doc, &EvalOptions::default());
    let log = &ev
        .value(profile_node(&part_doc))
        .expect("the profile evaluates")
        .verdicts;
    assert_eq!(log.len(), PROFILE_LOG);
    let (pre, op) = log.split_at(PRE_PASS);
    let mut histogram: BTreeMap<&str, usize> = BTreeMap::new();
    for v in pre {
        *histogram.entry(v.predicate).or_default() += 1;
    }
    assert_eq!(histogram["datum_unit_norm"], 2, "{histogram:?}");
    assert_eq!(histogram["path_junction_turn"], 4, "{histogram:?}");
    assert_eq!(histogram["chord_side"], 28, "{histogram:?}");
    assert_eq!(histogram["line_span"], 8, "{histogram:?}");
    assert_eq!(
        pre[0].predicate, "datum_unit_norm",
        "the plane's axes decide first"
    );
    assert_eq!(pre[2].predicate, "path_junction_turn", "then the replay");
    assert_eq!(
        op[0].predicate, "vertex_separation",
        "then the op's validation"
    );
    assert_eq!(
        &pre[6..],
        op,
        "the f64 validation, recorded by the precompute and again by the op"
    );
}

/// **A memo hit runs the precompute in a frame it drops, and the
/// reused value is the record.** The content key needs the resolved
/// program, so the precompute runs before the lookup, inside the
/// node's fresh frame; on a hit that frame is finished and discarded
/// and the prior value — whose log already opens with the same
/// decisions at f64, the precompute deciding from inputs the key fixes
/// (D9) — is returned as is, Arc and all. What this row can see from
/// outside: nothing recomputed, the same log object, and an outer
/// frame that received none of the hit run's decisions. The prefix
/// identity itself (dropped frame == reused log's opening) is asserted
/// at the hit site, which this row drives at both scalars and the row
/// after it reddens.
fn a_memo_hit_drops_its_frame<T: EvalScalar>() {
    let part_doc = part("kstats-hit-part", 0.0, 1.0);
    let profile = profile_node(&part_doc);
    let opts = EvalOptions::default();
    let first = evaluate::<T>(&part_doc, None, &CancelToken::new(), &opts, Tol::witness());
    let outer = Bracket::open();
    let again = evaluate::<T>(
        &part_doc,
        Some(&first),
        &CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    let outside = outer.finish();
    assert_eq!(again.recomputed, 0, "every node was a memo hit");
    let (prior, reused) = (
        first.value(profile).expect("the profile evaluates"),
        again.value(profile).expect("the profile is reused"),
    );
    assert!(
        Arc::ptr_eq(&prior.verdicts, &reused.verdicts)
            && Arc::ptr_eq(&prior.escalations, &reused.escalations),
        "the reused value carries the prior's log itself"
    );
    assert_eq!(reused.verdicts.len(), PROFILE_LOG);
    assert!(
        outside.verdicts.is_empty() && outside.escalations.is_empty(),
        "the hit run's pre-pass decided inside the frame the node dropped, not here: {outside:?}"
    );
}

#[test]
fn a_memo_hit_drops_its_frame_at_f64() {
    a_memo_hit_drops_its_frame::<f64>();
}

#[cfg(feature = "interval")]
#[test]
fn a_memo_hit_drops_its_frame_at_interval() {
    a_memo_hit_drops_its_frame::<geom_core::Interval>();
}

/// **A hit whose precompute disagrees with the reused log refuses
/// loud.** A prior whose Profile value carries an empty log, keys
/// intact, is served to the hit path: the fresh frame's decisions are
/// not a prefix of the reused log, and the hit site's assertion — the
/// one the row above rests on — fires rather than returning a value
/// whose record is not what the run decided.
#[test]
#[should_panic(expected = "a memo hit's pre-key decisions differ from the reused log's prefix")]
fn a_memo_hit_whose_prefix_disagrees_with_the_reused_log_refuses_loud() {
    let part_doc = part("kstats-hit-disagrees", 0.0, 1.0);
    let profile = profile_node(&part_doc);
    let mut first = run(&part_doc, &EvalOptions::default());
    match first.nodes.get_mut(&profile) {
        Some(NodeResult::Ok(v)) => v.verdicts = Arc::new(Vec::new()),
        other => panic!("the profile evaluates: {other:?}"),
    }
    let _ = evaluate::<f64>(
        &part_doc,
        Some(&first),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
}

/// **A refusal at the D4 ε door carries no escalations**: a document
/// evaluated at a process ε other than the one it recorded is refused
/// on every node before the schedule runs (`refuse_tolerance_conflict`
/// mints every `NodeError` with empty escalations), so `eval_node` is
/// never entered and no frame is ever opened — empty by construction
/// of that door, and never whatever an outer frame held. The row
/// after this one is the frame-open-nothing-decided case.
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

/// **A pre-pass that decides and then fails carries its escalations.**
/// A square with an island whose right edge is a document parameter:
/// authored at a good value (the insert door validates the program at
/// f64 and would refuse an in-band one), then the parameter's VALUE is
/// moved to an in-band distance inside the outer edge — a door that
/// validates nothing, by design, because the evaluation's pre-pass is
/// where a parameter's profile is decided. That pre-pass escalates on
/// `chord_side` between the outer edge and the island's, and the node
/// refuses before its op runs; the
/// escalation is on the node's error — the frame is the node's whether
/// the refusal came from the op or from the pass before the key — and
/// in no outer frame.
#[test]
fn a_pre_pass_that_escalates_before_failing_carries_the_escalation() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness tolerance bands");
    let in_band = (band.zero() * band.escalate()).sqrt();
    let edge = ParamName::new("island_edge");
    let doc = ProfileDoc::empty(DocumentId::derive("kstats-pre-pass-fails"), tol);
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: edge.clone(),
            value: DocParam::continuous(Dimension::Length, 0.25),
        },
    );
    let (doc, plane) = insert(
        doc,
        frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let at_edge = || Expr::param(edge.clone(), Dimension::Length);
    let island = LoopProgram::polygon_expr([
        [len(0.0), len(-0.25)],
        [at_edge(), len(-0.25)],
        [at_edge(), len(0.25)],
        [len(0.0), len(0.25)],
    ]);
    let program = ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon(square(0.0, 0.0, 0.5)).expect("finite corners"),
            island,
        ],
    };
    let (doc, profile) = insert(doc, Node::Profile(program));
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParamValue {
            name: edge,
            value: DocParamValue::Continuous(0.5 - in_band),
        },
    );
    let outer = Bracket::open();
    let ev = run(&doc, &EvalOptions::default());
    let outside = outer.finish();
    let err = ev
        .result(profile)
        .and_then(NodeResult::error)
        .expect("the profile refuses");
    let named: Vec<&str> = err.escalations.iter().map(|e| e.predicate()).collect();
    assert_eq!(
        named,
        ["chord_side"],
        "the pre-pass's escalation rides the refusal: {}",
        err.kind
    );
    assert!(
        matches!(err.kind, editor_core::NodeErrorKind::Profile(_)),
        "{}",
        err.kind
    );
    assert!(
        matches!(ev.result(extrude), Some(NodeResult::Poisoned { through }) if *through == profile)
    );
    assert!(
        outside.verdicts.is_empty() && outside.escalations.is_empty(),
        "{outside:?}"
    );
}

/// **A pre-key `Expr` refusal carries no escalations, with the frame
/// open**: a profile vertex written as `1 / divisor`, the parameter's
/// value moved to zero, refuses in the program's resolution — inside
/// the node's frame, after zero decisions — so the error's escalations
/// are empty because nothing decided, and the outer frame is empty
/// because the frame was the node's.
#[test]
fn a_pre_key_expr_refusal_carries_no_escalations() {
    let divisor = ParamName::new("divisor");
    let doc = ProfileDoc::empty(DocumentId::derive("kstats-expr-refusal"), Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: divisor.clone(),
            value: DocParam::continuous(Dimension::Scalar, 1.0),
        },
    );
    let (doc, plane) = insert(
        doc,
        frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let over = || {
        Expr::div(len(1.0), Expr::param(divisor.clone(), Dimension::Scalar))
            .expect("a length over a scalar")
    };
    let program = ProfileProgram {
        plane,
        loops: vec![LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [over(), len(0.0)],
            [over(), len(1.0)],
            [len(0.0), len(1.0)],
        ])],
    };
    let (doc, profile) = insert(doc, Node::Profile(program));
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParamValue {
            name: divisor,
            value: DocParamValue::Continuous(0.0),
        },
    );
    let outer = Bracket::open();
    let ev = run(&doc, &EvalOptions::default());
    let outside = outer.finish();
    let err = ev
        .result(profile)
        .and_then(NodeResult::error)
        .expect("the profile refuses");
    assert!(
        matches!(err.kind, editor_core::NodeErrorKind::Expr { .. }),
        "{}",
        err.kind
    );
    assert!(err.escalations.is_empty());
    assert!(
        outside.verdicts.is_empty() && outside.escalations.is_empty(),
        "{outside:?}"
    );
}
