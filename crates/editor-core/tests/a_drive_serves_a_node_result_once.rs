//! **A drive serves a node result once** (E6's leaf protocol: "a node
//! whose inputs a leaf leaves unchanged is served from an earlier
//! replay of the drive: result or refusal, and record, whole").
//!
//! A leaf served a node's result, value or refusal, reads and counts
//! what a leaf that ran it reads and counts, so a drive's verdict,
//! every leaf with its receipt, and its total are the same with the
//! drive's node store on or off, on one thread or many.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, ParamBoxVerdict, drive};
use editor_core::eval::testing::{Store, drive_result_key};
use editor_core::{
    CancelToken, EvalOptions, Evaluation, MemoDial, NodeResult, PriorRefusal, ProfileDoc,
    evaluate,
};
use geom_core::sym::{DriveMemo, with_session_memo_retry};
use geom_core::{Interval, Sym, SymBudget, SymCounts, SymRetry, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    }
}

fn table() -> Arc<DriveMemo> {
    Arc::new(DriveMemo::new(budget(), SymRules::shipped()))
}

/// A document whose leaves run the retry ladder: R2's filleted bracket
/// past the box it certifies whole, so a drive splits it.
fn retrying(tol: Tol) -> ProfileDoc {
    crate::m10_7_r2_probes_interval::bracket(1.0e3 * tol.eps(), tol).0
}

/// A document whose constructors register identities, with two
/// features that read different variables: the two-hole plate.
fn registering(tol: Tol) -> ProfileDoc {
    crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0
}

fn driven(doc: &ProfileDoc, config: DriveConfig, tol: Tol) -> ParamBoxVerdict {
    drive(
        doc,
        &analyzed_box(doc, &AnalysisPolicy::default()),
        &config,
        &editor_core::Stop::default(),
        tol,
    )
    .expect("the document drives")
}

fn eight(parallel: bool, memo: MemoDial) -> DriveConfig {
    DriveConfig {
        max_leaves: 8,
        parallel,
        memo,
        ..DriveConfig::default()
    }
}

/// Every leaf's receipt, summed: what the drive's total was before it
/// counted each node result once.
fn summed(v: &ParamBoxVerdict) -> SymCounts {
    let mut sum = SymCounts::default();
    for c in v.certified() {
        sum.absorb(c.decisions);
    }
    for r in v.refused() {
        sum.absorb(r.decisions);
    }
    sum
}

/// **The store on against off, one thread against many**, on a drive
/// whose ladder runs and on one whose constructors register: the
/// serialized drive, every leaf with its receipt, and the total are the
/// same in every state of the dial and under both schedules, and the
/// store held results.
///
/// Mutant: a served node scope that closes with what ran (the work
/// before its key) and not the held record leaves every served leaf's
/// receipt short (`geom_core::sym::serve` a no-op).
#[test]
fn a_drive_is_the_same_with_its_node_store_on_or_off_on_one_thread_or_many() {
    let tol = Tol::witness();
    for (label, doc) in [
        ("retrying", retrying(tol)),
        ("registering", registering(tol)),
    ] {
        let reference = driven(&doc, eight(false, MemoDial::Off), tol);
        assert!(
            reference.certified().len() + reference.refused().len() > 1,
            "{label}: the drive has more than one leaf"
        );
        assert!(
            label != "retrying" || reference.decisions().retried > 0,
            "the ladder closed a decision in the retrying drive"
        );
        for (parallel, memo) in [
            (false, MemoDial::Nodes),
            (true, MemoDial::Nodes),
            (false, MemoDial::Table),
            (true, MemoDial::Off),
        ] {
            let other = driven(&doc, eight(parallel, memo), tol);
            let what = format!("{label}: parallel {parallel}, {memo:?}");
            assert_eq!(reference.serialize(), other.serialize(), "{what}");
            assert_eq!(reference.certified(), other.certified(), "{what}");
            assert_eq!(reference.refused(), other.refused(), "{what}");
            assert_eq!(reference.decisions(), other.decisions(), "{what}: the total");
            assert_eq!(
                other.node_store().entries > 0,
                memo == MemoDial::Nodes,
                "{what}: the store holds results exactly when it is on"
            );
        }
    }
}

/// **The drive's total counts each distinct node result once**: on a
/// multi-leaf drive whose leaves share nodes, it is less than the sum
/// of the leaves' receipts, and it is the same with the store on and
/// off.
///
/// Red on main, where the total was that sum.
#[test]
fn a_drives_total_counts_each_node_result_once() {
    let tol = Tol::witness();
    let doc = registering(tol);
    let on = driven(&doc, eight(true, MemoDial::Nodes), tol);
    let off = driven(&doc, eight(false, MemoDial::Off), tol);
    assert_eq!(on.decisions(), off.decisions(), "the total, on against off");
    let sum = summed(&off);
    assert!(
        off.decisions().decisions() < sum.decisions(),
        "the total {:?} counts the plate's shared nodes once, under the leaves' sum {sum:?}",
        off.decisions()
    );
}

/// **The growth guard reads the same with the store on and off**: the
/// retrying drive with the guard at one form per attempt, which
/// withholds attempts the default guard offers, serializes the same and
/// holds every leaf's receipt with the store on and off. The guard is a
/// node's (`geom_core::SymRetry::max_forms`), so a node served without
/// its asks spends nothing of another node's.
#[test]
fn the_growth_guard_reads_the_same_with_the_node_store_on_and_off() {
    let tol = Tol::witness();
    let doc = retrying(tol);
    let guarded = |memo| {
        let mut config = eight(false, memo);
        config.symbolic.retry.max_forms = 1;
        driven(&doc, config, tol)
    };
    let off = guarded(MemoDial::Off);
    let on = guarded(MemoDial::Nodes);
    assert_eq!(off.serialize(), on.serialize(), "serialized");
    assert_eq!(off.certified(), on.certified(), "certified leaves");
    assert_eq!(off.refused(), on.refused(), "refused leaves");
    assert_ne!(
        off.serialize(),
        driven(&doc, eight(false, MemoDial::Off), tol).serialize(),
        "the guard at one form withholds what the default guard offers"
    );
}

/// The plate's root box and its half along the first axis that varies.
fn root_and_half(doc: &ProfileDoc) -> (ParamBox, ParamBox, editor_core::VarId) {
    let root = ParamBox::of(&analyzed_box(doc, &AnalysisPolicy::default()));
    let (var, lo, hi) = root.varying().next().expect("an axis varies");
    let mut axes = root.axes().clone();
    axes.insert(
        var,
        BoxAxis::Varying {
            lo,
            hi: 0.5 * (lo + hi),
        },
    );
    (root.clone(), ParamBox::from_axes(axes), var)
}

fn opts_over(box_: &ParamBox) -> EvalOptions {
    EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        ..EvalOptions::default()
    }
}

/// **A held entry is a leaf's own only where the leaf's box agrees on
/// what the node reads**: the key a drive holds a node's result by
/// differs between two boxes that differ on a variable the node reads,
/// and is one between two that differ only on a variable it does not.
///
/// Mutant: a key without the read set's brackets is one key across the
/// boxes a read variable splits.
#[test]
fn a_node_result_is_held_under_the_brackets_of_what_the_node_reads() {
    let tol = Tol::witness();
    let doc = registering(tol);
    let (root, half, var) = root_and_half(&doc);
    let reads = doc.free_reads();
    let ev = evaluate::<Interval>(&doc, None, &CancelToken::new(), &opts_over(&root), tol);
    let (mut reading, mut blind) = (0, 0);
    for id in &ev.order {
        let Some(v) = ev.value(*id) else { continue };
        let keys = (v.content_key, v.naming_key);
        let same = drive_result_key(&doc, *id, keys, &root) == drive_result_key(&doc, *id, keys, &half);
        let read = reads.get(id).is_some_and(|r| r.contains(&var));
        assert_eq!(same, !read, "{id:?}: one key across the split exactly when it does not read it");
        if read {
            reading += 1;
        } else {
            blind += 1;
        }
    }
    assert!(reading > 0 && blind > 0, "nodes on both sides: {reading} read it, {blind} do not");
}

/// **A store built under another table, or under none, is not read**:
/// a leaf in a session on one table takes nothing from a store of
/// another's, and puts nothing in it.
#[test]
fn a_store_of_another_table_or_of_none_is_not_read() {
    let tol = Tol::witness();
    let doc = registering(tol);
    let (root, _, _) = root_and_half(&doc);
    let opts = opts_over(&root);
    let other = table();
    for (label, token) in [("no table", None), ("another table", Some(other.token()))] {
        let store = Store::<Sym<Interval>>::new(&doc, &opts, token);
        let ev = with_session_memo_retry(
            budget(),
            SymRules::shipped(),
            SymRetry::kept_atom(),
            &table(),
            || store.serve(&doc, &opts, tol),
        )
        .0;
        assert_eq!(ev.prior_refused, Some(PriorRefusal::Table), "{label}");
        assert_eq!(store.entries(), 0, "{label}: nothing is put in it");
    }
}

/// **A deferring session takes no prior**: an evaluation inside one,
/// handed another of the same session (one table), reads nothing of it.
///
/// Mutant: `PriorRefusal::of` without the deferring arm reads it, every
/// node reused.
#[test]
fn a_deferring_session_takes_no_prior() {
    let tol = Tol::witness();
    let doc = registering(tol);
    let (root, _, _) = root_and_half(&doc);
    let opts = opts_over(&root);
    let (reused, refused) = geom_core::sym::deferred::with_deferred_session(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        || {
            let first: Evaluation<Sym<Interval>> = evaluate(&doc, None, &CancelToken::new(), &opts, tol);
            let second = evaluate(&doc, Some(&first), &CancelToken::new(), &opts, tol);
            (second.reused, second.prior_refused)
        },
    )
    .0;
    assert_eq!(refused, Some(PriorRefusal::Deferring));
    assert_eq!(reused, 0, "no node is taken from the prior");
}

/// **A family whose every member is served carries the family's
/// record**: the plate's row of three holes evaluated over the root box
/// with that box's own evaluation as its prior (every member served)
/// holds the record a fresh build holds.
///
/// Red on main, where a family whose members were all reused kept an
/// empty record.
#[test]
fn a_family_served_whole_carries_its_record() {
    let tol = Tol::witness();
    let p = crate::m10_7_plate::plate_with_holes(5.0e-5, 1.0e-5, tol);
    let (doc, _k, row) = crate::fixture::row(
        p.doc,
        p.holes[0],
        ("k", 3),
        editor_core::TargetAxis::Y,
        5.0e-3,
    );
    let (root, _, _) = root_and_half(&doc);
    let opts = opts_over(&root);
    let shared = table();
    let over = |prior: Option<&Evaluation<Sym<Interval>>>, table: &Arc<DriveMemo>| {
        with_session_memo_retry(
            budget(),
            SymRules::shipped(),
            SymRetry::kept_atom(),
            table,
            || evaluate::<Sym<Interval>>(&doc, prior, &CancelToken::new(), &opts, tol),
        )
        .0
    };
    let first = over(None, &shared);
    let served = over(Some(&first), &shared);
    let record = |ev: &Evaluation<Sym<Interval>>| {
        let v = ev.value(row).expect("the row built");
        (v.sym.counts(), v.sym.registrations(), v.sym.read())
    };
    assert!(served.reused > 0, "the second run took its nodes from the first");
    assert!(record(&first).0.decisions() > 0, "the family decided");
    assert_eq!(record(&served), record(&first), "the family's record, served or built");
}

/// The planted notch over its whole box, which straddles the branch
/// change: a node there refuses on an escalation.
fn refusing() -> (ProfileDoc, ParamBox) {
    let doc = crate::m10_3_driver_interval::notch(-0.25, 0.3);
    let root = ParamBox::of(&analyzed_box(&doc, &AnalysisPolicy::default()));
    (doc, root)
}

/// **A leaf served a refusal records what a leaf that ran it records**:
/// the node's refusal, its escalation log, its record and its keys,
/// and the leaf's receipt. E6's leaf comparison reads a failed node's
/// log, so a store that served a shorter one would move a verdict.
///
/// Mutant: a served refusal whose record is replaced by what ran before
/// its key (`recorded` writing a reused failure's record) reads an
/// empty record.
#[test]
fn a_served_refusal_is_the_refusal_a_run_records() {
    let tol = Tol::witness();
    let (doc, box_) = refusing();
    let opts = opts_over(&box_);
    let shared = table();
    let store = Store::<Sym<Interval>>::new(&doc, &opts, Some(shared.token()));
    let leaf = |served: bool, table: &Arc<DriveMemo>| {
        with_session_memo_retry(
            budget(),
            SymRules::shipped(),
            SymRetry::kept_atom(),
            table,
            || {
                if served {
                    store.serve(&doc, &opts, tol)
                } else {
                    evaluate::<Sym<Interval>>(&doc, None, &CancelToken::new(), &opts, tol)
                }
            },
        )
    };
    let (_built, _) = leaf(true, &shared);
    let (served, served_counts) = leaf(true, &shared);
    let (fresh, fresh_counts) = leaf(false, &table());
    assert!(served.reused > 0, "the second leaf was served");
    let failures = |ev: &Evaluation<Sym<Interval>>| -> Vec<String> {
        ev.order
            .iter()
            .filter_map(|id| match ev.result(*id) {
                Some(NodeResult::Failed(e)) => Some(format!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    e.node, e.kind, e.escalations, e.sym, e.keys
                )),
                _ => None,
            })
            .collect()
    };
    let failed = failures(&fresh);
    assert!(
        fresh.order.iter().any(|id| matches!(
            fresh.result(*id),
            Some(NodeResult::Failed(e)) if e.keys.is_some() && !e.escalations.is_empty()
        )),
        "a node refuses past its keys with escalations: {failed:?}"
    );
    assert_eq!(failures(&served), failed, "the refusals, served or run");
    assert_eq!(
        SymCounts {
            frozen: 0,
            ..served_counts
        },
        SymCounts {
            frozen: 0,
            ..fresh_counts
        },
        "the leaf's receipt"
    );
}
