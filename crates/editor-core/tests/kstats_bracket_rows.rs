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
//! - the Profile node's log at `f64` IS the precompute's, and at a lane
//!   scalar the op's own second pass follows it;
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
    CancelToken, Dimension, DocEdit, DocRef, DocumentId, EvalOptions, EvalScalar, Evaluation,
    Formula, Frame, FreeValue, FreeVar, LoopProgram, Node, NodeResult, ProfileDoc, ProfileProgram,
    RecipeNodeId, VarName, evaluate,
};
use fixture::resolver::{PartStore, with_resolver};
use fixture::{insert, len, on_frame, run, square, step};
use geom_core::Band;
use geom_core::Tol;
use geom_core::k_stats::{Bracket, Verdict};

/// A one-solid part: a `side`-wide square extruded 1 tall, placed in
/// its world.
fn part(label: &str, cx: f64, side: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, side / 2.0)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    fixture::place(doc, body).0
}

/// An assembly instantiating `refs` in order, each a loose copy until
/// [`placed`] pins it.
fn assembly(label: &str, refs: &[DocRef]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for r in refs {
        let (next, id) = insert(doc, crate::fixture::mated_instance(*r));
        doc = next;
        ids.push(id);
    }
    (crate::fixture::assemble_world_copies(doc), ids)
}

/// The part's one Profile node.
fn profile_node(doc: &ProfileDoc) -> RecipeNodeId {
    doc.ids()
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
/// part: the program's four junctions and the f64 validation of the
/// assembled profile, drawn on its own xy.
const PRE_PASS: usize = 80;
/// The one-solid part's log sizes by node at `f64`: profile, extrude,
/// placement. At `f64` the Profile node's op takes the pre-pass's
/// validated form (`editor_core::ProfileLane`) and logs what the second
/// pass it skips decides, so its log is a lane scalar's. Its at-rest
/// gate re-certifies the box's edges, where limb 2's distance bound (`ssi_hull_sup`, C2) adds 24 = 12 box edges × 2 operands
/// to the 655 it held before limb 2 was owed by every carrier.
const EXTRUDE_LOG: usize = 679;
/// The one-solid part's second pass: the op's own guided replay and
/// validation, after the pre-pass in the Profile node's log.
const LANE_OP: usize = 75;
const PROFILE_LOG: usize = PRE_PASS + LANE_OP;
/// A placement standing where its body stands ([`fixture::place`]): the
/// identity bundle, which the fold places by no map, deciding nothing.
const PLACEMENT_LOG: usize = 0;
/// Each instance's own op: its pin, values on the world's axes composed
/// as a motion (reading no pose and deciding no margin). The copy shares
/// the part's product body under that motion, so it builds and decides
/// nothing.
const INSTANCE_LOG: usize = 0;

/// Two instances, both pinned to the world, so both ops do the same
/// work. `o` reaches the part they copy.
fn placed(doc: ProfileDoc, ids: &[RecipeNodeId], o: &EvalOptions) -> ProfileDoc {
    let doc = fixture::pin(doc, ids[0], &Frame::translation([0.0, 9.0, 0.0]), o);
    fixture::pin(doc, ids[1], &Frame::translation([9.0, 0.0, 0.0]), o)
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
    let mut store = PartStore::default();
    let doc_ref = store.insert(part(&format!("{label}-part"), 0.0, 1.0), Tol::witness());
    let (doc, ids) = assembly(&format!("{label}-asm"), &[doc_ref, doc_ref]);
    let opts = with_resolver(store);
    let doc = placed(doc, &ids, &opts);
    (doc, ids, opts)
}

/// **The parallel schedule.** Two instances of one part under rayon:
/// the instantiate node's log is the same — the same as the other
/// instance's and the same as the sequential schedule's — over eight
/// runs. The shield on the part's evaluation is what this row exists
/// for.
#[test]
fn the_instantiate_log_is_schedule_independent() {
    let (doc, ids, seq_opts) = two_instances("kstats-sched");
    let seq = logs(&run(&doc, &seq_opts), &ids);
    assert_eq!(seq, [INSTANCE_LOG, INSTANCE_LOG]);
    let par_opts = EvalOptions {
        parallel: true,
        ..seq_opts.clone()
    };
    for i in 0..8 {
        let par = logs(&run(&doc, &par_opts), &ids);
        assert_eq!(par, seq, "parallel run {i} differs from the sequential log");
    }
}

/// **A part inside a part**: two nested part evaluations, shielded. The
/// outer instantiate nodes' logs are their own op's — equal to each
/// other and to the same document under the parallel schedule — and,
/// like a leaf's, empty: a copy of the middle assembly's two solids
/// shares them and decides nothing.
#[test]
fn a_part_inside_a_part_keeps_the_outer_log_its_own_under_both_schedules() {
    let mut store = PartStore::default();
    let leaf = store.insert(part("kstats-nest-leaf", 0.0, 1.0), Tol::witness());
    let (mid, mid_ids) = assembly("kstats-nest-mid", &[leaf, leaf]);
    let mid = placed(mid, &mid_ids, &with_resolver(store.clone()));
    let mid = crate::fixture::assemble_world_copies(mid);
    let mid_ref = store.insert(mid, Tol::witness());
    let (top, top_ids) = assembly("kstats-nest-top", &[mid_ref, mid_ref]);
    let opts = with_resolver(store);
    let top = placed(top, &top_ids, &opts);
    let seq = logs(&run(&top, &opts), &top_ids);
    assert_eq!(seq[0], seq[1], "the two outer instances differ: {seq:?}");
    assert_eq!(
        seq[0], INSTANCE_LOG,
        "the outer op's own log over a two-solid part, and its pin's"
    );
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
    assert_eq!(logs(&again, &ids), [INSTANCE_LOG, INSTANCE_LOG]);
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
/// behalf and its log holds those decisions, then the second pass's,
/// which the `f64` op logs in the pass's place.
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
    let order = part_doc.ids();
    assert_eq!(
        counts,
        BTreeMap::from([
            (order[0], PROFILE_LOG),
            (order[1], EXTRUDE_LOG),
            (order[2], PLACEMENT_LOG),
        ]),
        "one log per node, the Profile node's carrying its precompute: {counts:?}"
    );
}

/// **The assembly decides nothing outside its instances' brackets**:
/// the part's decisions are shielded where the census evaluates it,
/// and each instance's log is its own op's.
#[test]
fn the_assembly_decides_nothing_outside_its_instances_brackets() {
    let (doc, ids, opts) = two_instances("kstats-outside");
    let outer = Bracket::open();
    let ev = run(&doc, &opts);
    let outside = outer.finish();
    assert_eq!(logs(&ev, &ids), [INSTANCE_LOG, INSTANCE_LOG]);
    assert!(
        outside.verdicts.is_empty() && outside.escalations.is_empty(),
        "{outside:?}"
    );
}

/// The one-solid part's Profile log at `T`.
fn profile_log<T: EvalScalar>() -> Vec<Verdict> {
    let part_doc = part("kstats-order-part", 0.0, 1.0);
    let ev = evaluate::<T>(
        &part_doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    ev.value(profile_node(&part_doc))
        .expect("the profile evaluates")
        .verdicts
        .to_vec()
}

/// **At `f64` the Profile node's log opens with the precompute's**,
/// checked for its shape. One frame, in the order made: the program's
/// replay first — the plane is READ off the frame node's value and
/// decides nothing here — then the f64 validation, then what the second
/// pass decides, logged in its place: the `f64` lane takes the
/// precompute's validated form instead of validating again. The
/// histogram moves legitimately only when `profile::validate`'s probes
/// change (a predicate added, a probe count per segment pair changed)
/// or the fixture does.
#[test]
fn the_profile_nodes_log_at_f64_opens_with_the_pre_pass() {
    let full = profile_log::<f64>();
    assert_eq!(
        full.len(),
        PROFILE_LOG,
        "the pre-pass, then the second pass's"
    );
    let log = &full[..PRE_PASS];
    let mut histogram: BTreeMap<&str, usize> = BTreeMap::new();
    for v in log {
        *histogram.entry(v.predicate).or_default() += 1;
    }
    assert!(
        !histogram.contains_key("pose_unit_norm"),
        "the plane's axes are the FRAME's decision, not the profile's: {histogram:?}"
    );
    // Seven from the replay's junctions (each read at its arriving
    // arm, and the three a departing leg follows read again at the
    // shorter arm once it is stored), four from validation's joint
    // pass, which reads the same turn.
    assert_eq!(histogram["path_junction_turn"], 11, "{histogram:?}");
    assert_eq!(histogram["chord_side"], 28, "{histogram:?}");
    assert_eq!(histogram["line_span"], 8, "{histogram:?}");
    assert_eq!(
        log[0].predicate, "path_junction_turn",
        "the replay decides first"
    );
    assert_eq!(
        log[7].predicate, "vertex_separation",
        "then the f64 validation"
    );
}

/// **At a lane scalar the op's own decisions follow the
/// precompute's.** The precompute is an `f64` computation whatever the
/// lane scalar, so the log opens with the `f64` precompute's, verdict
/// for verdict; the op then resolves the program at the lane scalar and
/// validates what IT elaborated — a guided second decision sequence,
/// not the precompute's again — and at the nominal it decides what the
/// `f64` log holds in its place, verdict for verdict.
fn the_lane_log_opens_with_the_pre_pass<T: EvalScalar>() {
    let at_f64 = profile_log::<f64>();
    let at_lane = profile_log::<T>();
    assert_eq!(
        at_lane.len(),
        PRE_PASS + LANE_OP,
        "{}: the pre-pass, then the op's own second pass",
        T::NAME
    );
    let (pre, op) = at_lane.split_at(PRE_PASS);
    assert_eq!(
        pre,
        &at_f64[..PRE_PASS],
        "{}: the precompute decides at f64 under every lane",
        T::NAME
    );
    assert_eq!(
        op,
        &at_f64[PRE_PASS..],
        "{}: the second pass decides what the f64 log holds in its place",
        T::NAME
    );
    assert_eq!(
        op[0].predicate,
        "path_junction_turn",
        "{}: then the op's own replay at the lane scalar",
        T::NAME
    );
    assert!(
        op.iter().any(|v| v.predicate == "vertex_separation"),
        "{}: and its own validation",
        T::NAME
    );
}

#[test]
fn the_profile_nodes_log_at_dual_opens_with_the_pre_pass() {
    the_lane_log_opens_with_the_pre_pass::<geom_core::Dual64>();
}

#[test]
fn the_profile_nodes_log_at_interval_opens_with_the_pre_pass() {
    the_lane_log_opens_with_the_pre_pass::<geom_core::Interval>();
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
fn a_memo_hit_drops_its_frame<T: EvalScalar>(log: usize) {
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
    assert_eq!(reused.verdicts.len(), log);
    assert!(
        outside.verdicts.is_empty() && outside.escalations.is_empty(),
        "the hit run's pre-pass decided inside the frame the node dropped, not here: {outside:?}"
    );
}

#[test]
fn a_memo_hit_drops_its_frame_on_f64() {
    a_memo_hit_drops_its_frame::<f64>(PROFILE_LOG);
}

#[test]
fn a_memo_hit_drops_its_frame_on_interval() {
    a_memo_hit_drops_its_frame::<geom_core::Interval>(PROFILE_LOG);
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
/// where a parameter's profile is decided. That pre-pass reads two
/// line pairs' gaps in band (`segment_gap`, a screen each pair then
/// reads past): the island's right edge projects to a point that
/// distance inside an outer edge's span, and the outer right edge to one
/// that distance past an island edge's. It then escalates on
/// `chord_side` between the outer edge and the island's, and the node
/// refuses before its op runs. All three in-band readings are on
/// the node's error, in the order made — the frame is the node's
/// whether the refusal came from the op or from the pass before the key
/// — and in no outer frame.
#[test]
fn a_pre_pass_that_escalates_before_failing_carries_the_escalation() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness tolerance bands");
    let in_band = (band.zero() * band.escalate()).sqrt();
    let edge = VarName::from_static("island_edge");
    let doc = ProfileDoc::empty(DocumentId::derive("kstats-pre-pass-fails"), tol);
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: edge.clone(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.25)),
        },
    );
    let at_edge = || Formula::named(edge.clone(), Dimension::Length);
    let island = LoopProgram::polygon_expr([
        [len(0.0), len(-0.25)],
        [at_edge(), len(-0.25)],
        [at_edge(), len(0.25)],
        [len(0.0), len(0.25)],
    ]);
    let program = ProfileProgram {
        loops: vec![
            LoopProgram::polygon(square(0.0, 0.0, 0.5)).expect("finite corners"),
            island,
        ],
        ids: Vec::new(),
        on: None,
    };
    let (doc, profile) = insert(doc, Node::Profile(program));
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetVarValue {
            var: edge.into(),
            value: FreeValue::Continuous(0.5 - in_band),
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
        ["segment_gap", "segment_gap", "chord_side"],
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
    let divisor = VarName::from_static("divisor");
    let doc = ProfileDoc::empty(DocumentId::derive("kstats-expr-refusal"), Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: divisor.clone(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Scalar, 1.0)),
        },
    );
    let over = || {
        Formula::div(len(1.0), Formula::named(divisor.clone(), Dimension::Scalar))
            .expect("a length over a scalar")
    };
    let program = ProfileProgram {
        loops: vec![LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [over(), len(0.0)],
            [over(), len(1.0)],
            [len(0.0), len(1.0)],
        ])],
        ids: Vec::new(),
        on: None,
    };
    let (doc, profile) = insert(doc, Node::Profile(program));
    let (doc, _) = step(
        doc,
        DocEdit::SetVarValue {
            var: divisor.into(),
            value: FreeValue::Continuous(0.0),
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
