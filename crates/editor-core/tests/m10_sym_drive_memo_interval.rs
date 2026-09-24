//! **The drive-scoped plain memo** (`geom_core::sym::DriveMemo`): the
//! measurement that sizes it, the premise it rests on, and the pins
//! that keep it honest.
//!
//! A node's plain form is a function of its id and the session's budget
//! alone — the id is a content hash of `(op, payload, kids)` and the
//! plain walk reads no value — so a form computed on one leaf of a
//! drive is the form every other leaf of that drive would build. The
//! rows here are the three things that claim rests on:
//!
//! 1. **The ceiling** (evidence-only): over one drive, the plain forms
//!    computed against the DISTINCT ids they were computed for. The
//!    ratio is the most a memo keyed by the id can remove.
//! 2. **The premise** ([`the_opaque_sequence_is_identical_across_the_leaves_of_a_drive`]):
//!    an `Opaque` id is a per-leaf SEQUENCE number, not a content hash
//!    of anything (`OPAQUE_SEQ`'s D9 argument), so the memo is sound
//!    only while every leaf of a drive mints the same set of them.
//!    Measured by execution rather than read off the code.
//!
//! 3. **The differential** ([`the_plain_memo_moves_no_decision`]): the
//!    same drive with the memo on and off — every verdict, every leaf
//!    and every decision column identical, and the serialization a byte
//!    comparison rather than a filtered one.
//!
//! The basename carries `interval` so `ci-filter.py` pins the lane.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the code it measures: the tier itself, the memo, the
// instrument that counts it, and the driver that installs it.
test_utils::gated_to![
    "crates/geom-core/src/sym.rs",
    "crates/geom-core/src/sym/",
    "crates/editor-core/src/drive.rs",
    "crates/editor-core/tests/fixture/",
    "crates/editor-core/tests/m10_3_r1_probes_interval.rs",
    "crates/editor-core/tests/m10_7_plate.rs",
    "crates/editor-core/tests/m10_derived_frame_tilted_interval.rs",
];

use std::collections::BTreeSet;
use std::time::Instant;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, ParamBoxVerdict, drive};
use geom_core::sym::profile::{start_profile, take_profile};
use geom_core::{SymBudget, SymRules, Tol};

use crate::fixture::on_pool;
use crate::m10_3_r1_probes_interval::{CHAMBER_LEAVES, bounded_chamber};
use crate::m10_7_plate::plate;
use crate::m10_derived_frame_tilted_interval::boss_on_tilted;

// The preamble below — `eps`, `slab`, `the_plate` — is copied from
// `m10_sym_profile_interval` verbatim, so that the two suites' numbers
// are about one document. It is one more instance of the class
// `work/sym/interval-test-preamble-is-copied-across-the-m10-files`
// counts, and that row records this copy and why it was taken over an
// import (an import makes this suite's gated marker name the exporting
// one, and couples the unit's rows to a file they do not depend on).
fn eps() -> f64 {
    Tol::witness().eps()
}

fn slab() -> ProfileDoc {
    bounded_chamber(60.0 * eps(), 30.0 * eps(), 100.0 * eps())
}

/// The two-hole plate at the tolerances `m10_sym_profile_interval`
/// profiles it at, so the two suites' numbers are about one document.
fn the_plate(tol: Tol) -> ProfileDoc {
    plate(5.0e-5, 1.0e-5, tol).0
}

/// **Every leaf's `frozen` column, in receipt order** — the certified
/// leaves then the refused ones, which is the order
/// `ParamBoxVerdict::serialize` writes them in. The one spelling the
/// rows here read the column through (`geom_core::SymCounts::frozen`
/// says what it means).
fn leaf_column(v: &ParamBoxVerdict) -> Vec<u64> {
    v.certified()
        .iter()
        .map(|l| l.decisions.frozen)
        .chain(v.refused().iter().map(|l| l.decisions.frozen))
        .collect()
}

/// The leaf budget the pins drive at — small enough that the row costs
/// seconds rather than the minutes the ceiling rows pay, and large
/// enough that the frontier spreads over several levels, so a premise
/// that only holds for the first leaf reds here.
const PIN_LEAVES: usize = 48;

fn config(max_leaves: usize) -> DriveConfig {
    DriveConfig {
        max_leaves,
        ..DriveConfig::default()
    }
}

/// Prints the hit-rate ceiling of a drive: the plain forms the walk
/// computed, the distinct ids it computed them for, and the ratio.
fn ceiling(label: &str, doc: &ProfileDoc, max_leaves: usize) {
    let tol = Tol::witness();
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    // Sequentially, with the memo OFF: the ceiling is what the tier
    // recomputes without one, so the run that measures it must be the
    // run that does the recomputing. The profile is thread-local, so
    // the sequential schedule is also the only one it sees whole.
    let cfg = DriveConfig {
        plain_memo: false,
        ..config(max_leaves)
    };
    start_profile();
    let t0 = Instant::now();
    let v = drive(doc, &analyzed, &cfg, tol).unwrap();
    let wall = t0.elapsed();
    let p = take_profile();
    let forms = p.walk(geom_core::sym::profile::Walk::Plain).forms;
    let distinct = p.plain_ids.len() as u64;
    let leaves = p.sessions;
    println!(
        "{label}: leaves {leaves} wall {wall:?} | plain forms {forms} \
         ({:.0} per leaf) | distinct ids {distinct} | ratio {:.1}x | \
         frozen {} | receipt {:?}",
        forms as f64 / leaves.max(1) as f64,
        forms as f64 / distinct.max(1) as f64,
        v.decisions().frozen,
        v.receipt(),
    );
}

/// **The ceiling on the slab** — the M10-3 chamber drive at the row's
/// own leaf budget.
#[test]
#[ignore = "evidence-only: the plain memo's hit-rate ceiling over the slab drive"]
fn plain_memo_ceiling_slab_drive() {
    ceiling("slab", &slab(), CHAMBER_LEAVES);
}

/// **The ceiling on the plate** — the two-hole plate at
/// [`PLATE_WALL_LEAVES`], not at the driver's default: a plate replay
/// is seven times a slab replay, and with the memo off a drive of this
/// document at `DEFAULT_MAX_LEAVES` ran past twenty minutes without
/// finishing on the measuring box.
#[test]
#[ignore = "evidence-only: the plain memo's hit-rate ceiling over the plate drive"]
fn plain_memo_ceiling_plate_drive() {
    let tol = Tol::witness();
    ceiling("plate", &the_plate(tol), PLATE_WALL_LEAVES);
}

/// Every leaf's set of `Opaque` ids over one drive of `doc`, in leaf
/// order.
fn opaque_sets(doc: &ProfileDoc, max_leaves: usize) -> Vec<std::collections::BTreeSet<u128>> {
    let tol = Tol::witness();
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    start_profile();
    drive(doc, &analyzed, &config(max_leaves), tol).unwrap();
    take_profile().opaque_ids
}

/// **The `Opaque` sets every leaf of a drive mints — reported, per
/// leaf, with their SIZES.**
///
/// An `Opaque` id is the SEQUENCE NUMBER the leaf minted it at
/// (`Sym::opaque`, `OPAQUE_SEQ`) — the one part of a node id that is
/// not a hash of what the expression says. Two leaves that mint in
/// different orders build different ids for the same subexpression and
/// the drive memo MISSES on them; it does not answer them wrongly
/// (`geom_core::sym::memo`'s header carries that argument once, and
/// `geom-core`'s `sym_drive_memo` plants a leaf-varying mint and shows
/// every decision unmoved). So this row is a HIT-RATE guard, not a
/// soundness one.
///
/// **On every document in this tree the sets are EMPTY, and the row
/// says so rather than claiming a measurement it did not make.**
/// `Sym::opaque`'s only caller is the unnamed `AxisScalar::axis`
/// (`analysis.rs`), and a drive never reaches it: `drive` binds its
/// axes through `axis_named` → `Sym::param_over`. The comparison below
/// is therefore vacuous today — empty sets agree — and the row exists
/// for the day a lane mints an opaque under a drive. Both reviewers
/// planted a value-dependent mint in `axis_named` and measured what
/// happens: THIS row reds first (leaf 0's set stops matching), while
/// the differential and the schedule rows stay green — the memo keeps
/// answering the same decisions, more slowly.
///
/// No `assert!(!empty)` here: it would red today on a tree that is
/// working exactly as designed.
#[test]
fn the_opaque_sets_a_drive_mints_are_reported_per_leaf() {
    for (label, doc) in [("slab", slab()), ("plate", the_plate(Tol::witness()))] {
        let sets = opaque_sets(&doc, PIN_LEAVES);
        assert!(
            sets.len() > 1,
            "{label}: the comparison is ACROSS leaves, so the drive must produce more than \
             one: {} leaves",
            sets.len()
        );
        let sizes: Vec<usize> = sets.iter().map(std::collections::BTreeSet::len).collect();
        let total: usize = sizes.iter().sum();
        println!(
            "{label}: {} leaves, Opaque ids per leaf {:?}… total {total}",
            sets.len(),
            &sizes[..sizes.len().min(8)]
        );
        let first = &sets[0];
        for (i, s) in sets.iter().enumerate().skip(1) {
            assert_eq!(
                s.len(),
                first.len(),
                "{label}: leaf {i} minted {} Opaque ids and leaf 0 minted {} — the drive \
                 memo will MISS on this document's later leaves (it cannot answer them \
                 wrongly: `sym::memo`'s header)",
                s.len(),
                first.len()
            );
            assert_eq!(
                s, first,
                "{label}: leaf {i}'s Opaque ids differ from leaf 0's at equal size — same \
                 count, different sequence, so the memo misses on every node above them"
            );
        }
        if total == 0 {
            println!(
                "{label}: every set is empty — no drive of this document mints an opaque, so \
                 the comparison above is vacuous and this row is a guard for the day one does"
            );
        }
    }
}

/// The two drives of `doc` the differential compares: the memo on and
/// the memo off, sequentially, at `max_leaves`.
fn on_and_off(doc: &ProfileDoc, max_leaves: usize) -> (ParamBoxVerdict, ParamBoxVerdict) {
    let tol = Tol::witness();
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let run = |plain_memo| {
        drive(
            doc,
            &analyzed,
            &DriveConfig {
                max_leaves,
                plain_memo,
                ..DriveConfig::default()
            },
            tol,
        )
        .unwrap()
    };
    (run(true), run(false))
}

/// **The differential: the memo moves no decision.**
///
/// A plain form is a function of the node's content-hashed id, the
/// budget and the tier's dials — so a form the memo hands back is the
/// form the leaf would have built, and every verdict, every leaf and
/// every decision column is what it was. The drive's `frozen` column is
/// the same too, because it is the DISTINCT nodes frozen over the drive
/// in both lanes and a set does not care which leaf reached a node
/// first. That makes the whole serialization — and the content key over
/// it — a byte comparison rather than a filtered one.
///
/// **A LEAF's `frozen` is compared too, unmasked.** The column is that
/// leaf's NEED of the drive's frozen set and not the work it happened
/// to do (`geom_core::SymCounts::frozen`), so a leaf that inherits
/// every form still reports what its own reasoning rested on: the
/// saving the unit is for is in the WALL TIME and in
/// `MemoSize::forms`, not in a receipt column. The mask this row used
/// to compare through is gone with the meaning that needed it.
#[test]
fn the_plain_memo_moves_no_decision() {
    for (label, doc) in [("slab", slab()), ("plate", the_plate(Tol::witness()))] {
        let (on, off) = on_and_off(&doc, PIN_LEAVES);
        assert_eq!(
            on.serialize(),
            off.serialize(),
            "{label}: the drive-scoped plain memo changed the verdict's serialization"
        );
        assert_eq!(on.content_key(), off.content_key(), "{label}: content key");
        assert_eq!(on.receipt(), off.receipt(), "{label}: receipt");
        assert_eq!(
            on.decisions(),
            off.decisions(),
            "{label}: the drive receipt, `frozen` included — the distinct nodes frozen over a \
             drive do not depend on which leaf computed them"
        );
        assert_eq!(
            on.certified().len(),
            off.certified().len(),
            "{label}: certified leaves"
        );
        for (i, (a, b)) in on.certified().iter().zip(off.certified()).enumerate() {
            assert_eq!(a.box_, b.box_, "{label}: certified leaf {i}'s box");
            assert_eq!(
                a.verdict_vector_key, b.verdict_vector_key,
                "{label}: certified leaf {i}'s verdict vector"
            );
            assert_eq!(
                a.results, b.results,
                "{label}: certified leaf {i}'s results"
            );
            assert_eq!(
                a.decisions, b.decisions,
                "{label}: certified leaf {i}'s decision counts, `frozen` included"
            );
        }
        for (i, (a, b)) in on.refused().iter().zip(off.refused()).enumerate() {
            assert_eq!(a.box_, b.box_, "{label}: refused leaf {i}'s box");
            assert_eq!(a.reason, b.reason, "{label}: refused leaf {i}'s reason");
            assert_eq!(
                a.decisions, b.decisions,
                "{label}: refused leaf {i}'s decision counts, `frozen` included"
            );
        }
        // Non-vacuity, both ways: the memo really held the drive's forms,
        // and the leaves really took them — the work the leaves did
        // themselves fell, which is the whole point of the unit and the
        // one thing a memo that silently served nothing would still pass
        // every assertion above with.
        let memo = on.plain_memo();
        assert!(
            memo.forms > 0,
            "{label}: the memo held no form, so nothing above was a differential"
        );
        assert_eq!(
            off.plain_memo().forms,
            0,
            "{label}: the dial off must serve no form"
        );
        let own = |v: &ParamBoxVerdict| -> u64 { leaf_column(v).iter().sum() };
        println!(
            "{label}: memo {memo:?} | leaves' own frozen on {} off {} | drive frozen {}",
            own(&on),
            own(&off),
            on.decisions().frozen
        );
        // The leaf column is the SAME column in both lanes, which is
        // the whole of what made it a receipt column: with the memo off
        // a leaf froze every node its reasoning needed itself, so the
        // off lane's reading is that leaf's NEED already, and the on
        // lane — where the root leaf paid for nearly every freeze —
        // must reproduce it. Before SYM-13 the on lane read 0 here on
        // both documents against 50,112 off on the plate.
        assert_eq!(
            (own(&on) > 0, own(&on)),
            (memo.frozen > 0, own(&off)),
            "{label}: the leaves' NEED must be the same with the memo on and off, and \
             non-zero exactly where the drive froze ({} distinct nodes)",
            memo.frozen
        );
        assert_eq!(
            on.decisions().frozen,
            off.decisions().frozen,
            "{label}: both lanes report the same distinct drive column"
        );
    }
}

/// **The memo is schedule-independent.**
///
/// It is shared across the drive's workers rather than held per worker,
/// so what any leaf can inherit is the same set of forms whatever the
/// schedule, and the one receipt column the unit moved — `frozen`, now
/// the distinct nodes frozen over the drive — is a SET and not a sum.
/// The M10-3 suite repeats this claim at the chamber row's own leaf
/// budget; this row is the cheap one that sits beside the memo.
#[test]
fn the_memo_keeps_the_receipt_identical_across_schedules() {
    let tol = Tol::witness();
    let doc = slab();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |parallel| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: PIN_LEAVES,
                parallel,
                ..DriveConfig::default()
            },
            tol,
        )
        .unwrap()
    };
    let seq = run(false);
    let par = run(true);
    let par2 = run(true);
    assert_eq!(
        seq.serialize(),
        par.serialize(),
        "the parallel schedule must replay the sequential one byte for byte, `frozen` included"
    );
    assert_eq!(
        par.serialize(),
        par2.serialize(),
        "two parallel runs must agree byte for byte"
    );
    assert_eq!(seq.content_key(), par.content_key());
    assert_eq!(seq.decisions(), par.decisions());
}

/// Everything two drives of one document must agree on.
fn assert_same(label: &str, a: &ParamBoxVerdict, b: &ParamBoxVerdict) {
    assert_eq!(a.serialize(), b.serialize(), "{label}: serialization");
    assert_eq!(a.content_key(), b.content_key(), "{label}: content key");
    assert_eq!(a.receipt(), b.receipt(), "{label}: receipt");
    assert_eq!(
        a.decisions(),
        b.decisions(),
        "{label}: the drive receipt, `frozen` included"
    );
    assert_eq!(
        a.certified().len(),
        b.certified().len(),
        "{label}: certified count"
    );
    assert_eq!(
        a.refused().len(),
        b.refused().len(),
        "{label}: refused count"
    );
    for (i, (x, y)) in a.certified().iter().zip(b.certified()).enumerate() {
        assert_eq!(x.box_, y.box_, "{label}: certified {i} box");
        assert_eq!(
            x.verdict_vector_key, y.verdict_vector_key,
            "{label}: certified {i} verdict vector"
        );
        assert_eq!(x.results, y.results, "{label}: certified {i} results");
        assert_eq!(
            x.decisions, y.decisions,
            "{label}: certified {i} decisions, `frozen` included"
        );
    }
    for (i, (x, y)) in a.refused().iter().zip(b.refused()).enumerate() {
        assert_eq!(x.box_, y.box_, "{label}: refused {i} box");
        assert_eq!(x.reason, y.reason, "{label}: refused {i} reason");
        assert_eq!(
            x.decisions, y.decisions,
            "{label}: refused {i} decisions, `frozen` included"
        );
    }
}

/// The leaf budget the FREEZING schedule row drives at. The plate
/// replay is seven times the slab's, and this row pays for three
/// drives, so it is read at the smallest budget that still bisects —
/// the root leaf alone freezes the document's whole frozen set (1,044
/// distinct nodes), so `frozen` is non-zero from the first box and the
/// row cannot quietly degrade into the slab's all-zero case.
const FREEZING_LEAVES: usize = 8;

/// **A FREEZING document, across schedules and the dial** — the gap
/// both reviewers found in the unit's own coverage.
///
/// [`the_memo_keeps_the_receipt_identical_across_schedules`] and the
/// M10-3 receipt-identity row both drive the SLAB, which freezes
/// nothing at all, so the one receipt column this unit moved was pinned
/// across schedules only at zero. This row drives the plate, whose
/// `frozen` is 1,044, and asks that the sequential drive, the parallel
/// drive and the memo-off drive be byte-identical with that column
/// non-zero.
#[test]
fn a_freezing_drive_is_identical_across_schedules_and_the_dial() {
    let tol = Tol::witness();
    let doc = the_plate(tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |parallel, plain_memo| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: FREEZING_LEAVES,
                parallel,
                plain_memo,
                ..DriveConfig::default()
            },
            tol,
        )
        .unwrap()
    };
    let seq_on = run(false, true);
    assert!(
        seq_on.decisions().frozen > 0,
        "the row is about the moved column and it must be non-zero here: {:?}",
        seq_on.decisions()
    );
    assert!(seq_on.plain_memo().forms > 0, "the memo held forms");
    println!(
        "plate at {FREEZING_LEAVES} leaves: {:?} decisions {:?} memo {:?}",
        seq_on.receipt(),
        seq_on.decisions(),
        seq_on.plain_memo()
    );
    assert_same("plate seq on vs par on", &run(true, true), &seq_on);
    assert_same("plate seq on vs seq off", &run(false, false), &seq_on);
}

/// **The adversary drives**, and the one place their numbers are
/// written: the gating row below drives all three, and the
/// unrecorded-freeze census drives the first.
///
/// A drive's level 0 is always ONE box (`drive`'s frontier starts as
/// one and each level's boxes split in two), so a later leaf can only
/// reach a node the earlier levels never published when the leaf DAGs
/// are not subsets of the root's. Cutting the symbolic budget produces
/// exactly that on the slab: the root box's predicates go indeterminate
/// where a narrower box's do not, so the narrower boxes' replays get
/// further and build what the root's never did — and at these budgets
/// what they build FREEZES.
struct Racing {
    label: &'static str,
    leaves: usize,
    terms: usize,
    degree: u32,
    /// Whether this drive certifies any leaf at all. A drive that
    /// refuses every box compares an EMPTY `certified()` list, and the
    /// row says which of its arms is carrying that comparison rather
    /// than leaving it to be assumed.
    certifies: bool,
}

/// Three of them, because one document's leaf partition is one shape:
/// the unit's own race, the second reviewer's (which hands twelve
/// leaves nine distinct NEEDs), and a wider drive that CERTIFIES — the
/// only arm in which the `certified()` comparison below is about
/// anything.
const RACING: [Racing; 3] = [
    Racing {
        label: "race",
        leaves: 8,
        terms: 8,
        degree: 4,
        certifies: false,
    },
    Racing {
        label: "race (the second reviewer's)",
        leaves: 12,
        terms: 16,
        degree: 6,
        certifies: false,
    },
    Racing {
        label: "certifying",
        leaves: 64,
        terms: 512,
        degree: 32,
        certifies: true,
    },
];

fn racing_config(r: &Racing, parallel: bool, plain_memo: bool) -> DriveConfig {
    DriveConfig {
        max_leaves: r.leaves,
        parallel,
        plain_memo,
        symbolic: editor_core::drive::SymbolicDials {
            max_terms: r.terms,
            max_degree: r.degree,
            ..editor_core::drive::SymbolicDials::default()
        },
        ..DriveConfig::default()
    }
}

/// **The gating row: every leaf reports ONE column, whatever the
/// schedule and whichever way the memo dial is set.** The column is
/// each leaf's NEED of the drive's frozen set
/// (`geom_core::SymCounts::frozen`), and the drives here are the ones
/// where a leaf's own freezes are not the root leaf's — where the old
/// work-count column could not have passed.
///
/// **What this row gates is AGREEMENT ACROSS LANES, not the column's
/// meaning.** A column that was uniformly wrong — every leaf reading
/// the same wrong number under every schedule — would pass everything
/// here; what pins the meaning is `geom-core`'s `sym_drive_memo` rows
/// at the scalar door (reached twice counted once, built-but-never-
/// asked counted zero, inherited counted all the same, and the
/// unrecorded branch in both orders). The non-vacuity below is against
/// the other failure: a document that quietly stopped freezing, or a
/// column that collapsed to one value for every leaf.
///
/// **What the old column did here.** With `frozen` as the work a leaf
/// happened to do, the `seq off` arm reds DETERMINISTICALLY on every
/// one of these drives — the off lane re-freezes per leaf while the on
/// lane leaves nearly everything to the root leaf. The parallel arms
/// red too, but that is a DRAW and not a reading: which leaf of a level
/// gets to a node first is the schedule's business, and the vector
/// differs from run to run (measured on the first drive here: leaf 1
/// read 369 in one run of four workers and 284 in another, against 0
/// sequentially).
#[test]
fn every_leaf_reports_one_column_under_every_schedule_and_both_dials() {
    let tol = Tol::witness();
    let doc = slab();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    for r in &RACING {
        let run = |parallel, plain_memo| {
            drive(
                &doc,
                &analyzed,
                &racing_config(r, parallel, plain_memo),
                tol,
            )
            .unwrap()
        };
        let seq_on = run(false, true);
        let base = leaf_column(&seq_on);
        println!(
            "{} slab at {} leaves, budget {}/{}: receipt {:?} | drive frozen {} | \
             leaves' NEED {base:?} | memo {:?}",
            r.label,
            r.leaves,
            r.terms,
            r.degree,
            seq_on.receipt(),
            seq_on.decisions().frozen,
            seq_on.plain_memo(),
        );
        // Non-vacuity: the document really freezes, the memo really
        // held forms, the leaves need visibly different amounts of the
        // set, and the `certified()` comparison below is about
        // something in the arm that says it is.
        assert!(
            seq_on.decisions().frozen > 0 && seq_on.plain_memo().forms > 0,
            "{}: the drive must freeze and the memo must hold forms: {:?} {:?}",
            r.label,
            seq_on.decisions(),
            seq_on.plain_memo()
        );
        assert!(
            base.iter().collect::<BTreeSet<_>>().len() >= 3,
            "{}: the leaves must need visibly different amounts: {base:?}",
            r.label
        );
        assert_eq!(
            !seq_on.certified().is_empty(),
            r.certifies,
            "{}: the row's certified arm is vacuous unless this drive certifies: {:?}",
            r.label,
            seq_on.receipt()
        );
        for (label, v) in [
            ("par@2 on", on_pool(2, || run(true, true))),
            ("par@4 on", on_pool(4, || run(true, true))),
            ("seq off", run(false, false)),
            ("par@4 off", on_pool(4, || run(true, false))),
        ] {
            let label = format!("{} {label}", r.label);
            // The column first, because it is what this row is about
            // and a whole-list comparison names the leaf rather than
            // the number.
            assert_eq!(leaf_column(&v), base, "{label}: the leaves' NEED moved");
            // Then the comparison `my_own_drive_is_bit_identical…`
            // makes, on the drives that used to break it.
            assert_eq!(
                v.certified(),
                seq_on.certified(),
                "{label}: certified leaves"
            );
            assert_eq!(v.refused(), seq_on.refused(), "{label}: refused leaves");
            assert_eq!(v.serialize(), seq_on.serialize(), "{label}: serialization");
            assert_eq!(
                v.content_key(),
                seq_on.content_key(),
                "{label}: content key"
            );
        }
    }
}

/// **R2's own end-to-end row, kept whole** — two freezing documents
/// (a boss on a derived frame over a tilted datum, and the plate) at
/// the unit's pin budget, driven sequentially, on one, two and four
/// rayon workers, and with the dial off, everything byte-identical.
///
/// `#[ignore]`d for its cost, not its value: at 48 leaves the tilted
/// document alone is ~275 s per drive in the test profile. The numbers
/// R2 measured, and this lane re-took: `frozen` 632 on the tilted
/// document and 1,044 on the plate, identical on every lane above.
#[test]
#[ignore = "evidence-only: two freezing documents across four thread counts and the dial"]
fn a_freezing_drive_is_identical_across_thread_counts_too() {
    let tol = Tol::witness();
    for (label, doc) in [
        ("tilted-derived", boss_on_tilted(1.0e-3, true)),
        ("plate", the_plate(tol)),
    ] {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let run = |parallel, plain_memo| {
            drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: PIN_LEAVES,
                    parallel,
                    plain_memo,
                    ..DriveConfig::default()
                },
                tol,
            )
            .unwrap()
        };
        let t0 = Instant::now();
        let seq_on = run(false, true);
        let on_wall = t0.elapsed();
        let t0 = Instant::now();
        let seq_off = run(false, false);
        println!(
            "{label} at {PIN_LEAVES} leaves: seq on {on_wall:?} off {:?} | decisions {:?} | \
             memo {:?}",
            t0.elapsed(),
            seq_on.decisions(),
            seq_on.plain_memo()
        );
        assert!(
            seq_on.decisions().frozen > 0,
            "{label}: frozen is the column"
        );
        assert_same(&format!("{label} seq off"), &seq_off, &seq_on);
        for threads in [1, 2, 4] {
            let par = on_pool(threads, || run(true, true));
            assert_same(&format!("{label} par@{threads}"), &par, &seq_on);
        }
        let par_off = on_pool(2, || run(true, false));
        assert_same(&format!("{label} par@2 off"), &par_off, &seq_on);
    }
}

/// **The memo is valid for one budget and one set of dials**, dial by
/// dial.
///
/// The plain walk reads exactly two of the eight — `const_fold` and
/// `early` (`combine`'s `a0`) — and `within` reads the budget, so those
/// three are what a plain form's identity actually depends on. The
/// earlier shape of this row flipped the budget and then ALL eight
/// dials at once, which cannot tell a guard that tracks the two from
/// one that tracks the whole struct (R2 MINOR-4, R1 S2). Each is
/// flipped alone here, including one the plain walk does not read:
/// `accepts` refuses that one too, because it compares the whole
/// `SymRules` CONSERVATIVELY — refusing over a dial the walk ignores
/// costs hits and nothing else, and `DriveMemo::accepts`'s own doc
/// says so.
#[test]
fn a_drive_memo_refuses_a_leaf_it_was_not_made_for() {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let rules = SymRules::shipped();
    let memo = geom_core::sym::DriveMemo::new(budget, rules);
    assert!(memo.accepts(budget, rules));

    for (what, other) in [
        (
            "max_terms",
            SymBudget {
                max_terms: 64,
                ..budget
            },
        ),
        (
            "max_degree",
            SymBudget {
                max_degree: 32,
                ..budget
            },
        ),
    ] {
        assert!(
            !memo.accepts(other, rules),
            "a different {what} freezes different nodes, so the forms are different forms"
        );
    }

    // The two the plain walk reads, each alone.
    assert!(
        !memo.accepts(
            budget,
            SymRules {
                const_fold: !rules.const_fold,
                ..rules
            }
        ),
        "`const_fold` decides whether rule A0 folds in the plain walk (`combine`'s `a0`)"
    );
    assert!(
        !memo.accepts(
            budget,
            SymRules {
                early: !rules.early,
                ..rules
            }
        ),
        "`early` decides whether A0 applies in the plain walk or only alongside it"
    );
    // And one it does not read: refused anyway, conservatively.
    assert!(
        !memo.accepts(
            budget,
            SymRules {
                sqrt_square: !rules.sqrt_square,
                ..rules
            }
        ),
        "`sqrt_square` is a rule the PLAIN walk never applies, and the guard still refuses it: \
         it compares the whole `SymRules`, which costs a drive its hits and never a decision"
    );
}

/// **The memo is dropped with the drive**, detected rather than
/// assumed.
///
/// The earlier shape of this row drove ONE document twice and compared
/// the two memo sizes — which a memo that SURVIVED would also pass,
/// because it would hold the same DAG and report the same size (R2
/// MINOR-3). So the row drives two DIFFERENT documents back to back and
/// asks that the second drive's memo be the size that document builds
/// ALONE: a memo carrying the slab's 18 k forms into the plate's drive
/// reads larger, and reds here.
///
/// Claim 7 also holds by construction — `drive` builds its memo into a
/// local `Arc` and never hands it anywhere that outlives the call — and
/// this row is the guard on that construction, not its proof.
#[test]
fn a_memo_from_one_drive_never_serves_the_next() {
    let tol = Tol::witness();
    let plate = the_plate(tol);
    let plate_box = analyzed_box(&plate, &AnalysisPolicy::default());
    let run = |doc: &ProfileDoc, analyzed| {
        drive(doc, analyzed, &config(PIN_LEAVES), tol)
            .unwrap()
            .plain_memo()
    };
    let alone = run(&plate, &plate_box);
    assert!(alone.forms > 0, "the plate drive built forms: {alone:?}");

    let slab = slab();
    let slab_box = analyzed_box(&slab, &AnalysisPolicy::default());
    let first = run(&slab, &slab_box);
    assert!(first.forms > 0, "the slab drive built forms: {first:?}");
    let after = run(&plate, &plate_box);
    assert_eq!(
        after, alone,
        "the plate's memo after a slab drive must be the plate's own ({alone:?}); a memo that \
         outlived the slab drive would carry its {} forms into this one",
        first.forms
    );
}

/// **No node of a drive reaches the UNRECORDED branch.**
///
/// `form_in` freezes a node absent from the leaf's own hash-consing
/// table and — since the fix this unit's reviews forced — keeps that
/// freeze to itself rather than publishing it to the drive memo. The
/// asymmetry the reviewers demonstrated at the tier's own door
/// (`geom-core`'s `sym_drive_memo`) needs a node unrecorded in one leaf
/// and recorded in another, so the question this row answers is whether
/// a DRIVE ever produces one at all. Measured: it does not, on either
/// measured document nor on any of the three racing drives, and the row
/// pins that — a count that moves off zero means some lane started
/// minting nodes outside the session, and the guard in `form_in` is
/// then load-bearing rather than belt-and-braces.
#[test]
fn no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded() {
    let tol = Tol::witness();
    // The RACING drives are censused too, and they are what this row
    // owes most: they are the drives whose leaf DAGs are NOT subsets of
    // the root's, which is the state an unrecorded freeze would have to
    // come out of, and the one thing left unsettled about a leaf's NEED
    // (`geom_core::sym::memo`'s header, last paragraph) is what a leaf
    // does when it INHERITS a form across that branch.
    let racing = |i: usize| racing_config(&RACING[i], false, true);
    for (label, doc, cfg) in [
        ("slab", slab(), config(UNRECORDED_LEAVES)),
        ("plate", the_plate(tol), config(UNRECORDED_LEAVES)),
        ("racing slab", slab(), racing(0)),
        ("racing slab (the second reviewer's)", slab(), racing(1)),
        ("certifying slab", slab(), racing(2)),
    ] {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        start_profile();
        drive(&doc, &analyzed, &cfg, tol).unwrap();
        let p = take_profile();
        let unrecorded = p
            .freezes
            .iter()
            .filter(|f| f.cause == geom_core::sym::profile::FreezeCause::Unrecorded)
            .count();
        println!(
            "{label}: {} sessions, {} freezes, {unrecorded} of them unrecorded",
            p.sessions,
            p.freezes.len()
        );
        assert_eq!(
            unrecorded, 0,
            "{label}: a leaf froze a node its session never recorded"
        );
    }
}

/// The leaf budget the unrecorded-freeze census drives at — the profile
/// records every freeze individually, and the plate freezes ~1,000 per
/// leaf, so the census is read over a few leaves rather than many.
const UNRECORDED_LEAVES: usize = 8;

/// **The growth guard**: the memo is a second scope for the tier's
/// state, so what it comes to over a drive is a number the tree keeps
/// rather than a number a lane measured once.
///
/// CEILINGS on an ESTIMATE, not equalities like `SLAB_MAX_TERMS`:
/// `MemoSize::bytes_estimate` counts term by term and admits what it
/// does not count, so the byte ceiling bounds that estimate rather than
/// the heap. The population is
/// the drive's distinct nodes, which moves with the ε row the matrix
/// draws and with any change to how many boxes the drive visits, while
/// what the guard is for — a memo that starts holding a multiple of the
/// DAG — is an order of magnitude away from either. The measured
/// numbers are printed beside them, so a run says how much headroom is
/// left rather than only that there is some.
#[test]
fn the_drive_memo_stays_the_size_of_the_dag() {
    // The whole slab DAG is ~12 k nodes and the plate's ~17 k; a memo
    // keyed by the node id can hold one form per node it was asked for
    // and no more, so a reading near twice the DAG is the shape of the
    // thing and a reading near ten times it is a leak of leaf state.
    for (label, doc, max_forms, max_atoms, max_bytes) in [
        ("slab", slab(), 30_000, 256, 16 << 20),
        ("plate", the_plate(Tol::witness()), 30_000, 2_048, 24 << 20),
    ] {
        let tol = Tol::witness();
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let v = drive(&doc, &analyzed, &config(PIN_LEAVES), tol).unwrap();
        let m = v.plain_memo();
        println!(
            "{label} memo at {PIN_LEAVES} leaves: {m:?} (ceilings {max_forms} forms, \
             {max_atoms} atoms, {max_bytes} estimated bytes)"
        );
        assert!(
            m.forms > 0 && m.forms <= max_forms,
            "{label}: the memo holds {} plain forms, guarded at {max_forms}",
            m.forms
        );
        assert!(
            m.atoms <= max_atoms,
            "{label}: the memo holds {} atoms, guarded at {max_atoms}",
            m.atoms
        );
        assert!(
            m.bytes_estimate <= max_bytes,
            "{label}: the memo's heap ESTIMATE is {} bytes, guarded at {max_bytes}",
            m.bytes_estimate
        );
    }
}

/// **The number** — the slab and the plate driven with the memo on and
/// off, sequentially and in parallel, with the memo's size beside each.
///
/// Evidence-only and `#[ignore]`d, in `m10_sym_profile_interval`'s
/// mould: a wall time is a reading of one box on one day and not a
/// contract the tier makes, so it is re-taken by re-running the row and
/// nothing reds when it moves.
#[test]
#[ignore = "evidence-only: the drive's wall time with the plain memo on and off"]
fn plain_memo_wall_times() {
    let tol = Tol::witness();
    for (label, doc, max_leaves) in [
        ("slab", slab(), CHAMBER_LEAVES),
        ("plate", the_plate(tol), PLATE_WALL_LEAVES),
    ] {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for plain_memo in [false, true] {
            for parallel in [false, true] {
                let cfg = DriveConfig {
                    max_leaves,
                    parallel,
                    plain_memo,
                    ..DriveConfig::default()
                };
                let t0 = Instant::now();
                let v = drive(&doc, &analyzed, &cfg, tol).unwrap();
                println!(
                    "{label} {max_leaves} leaves memo={plain_memo} parallel={parallel}: \
                     wall {:?} | receipt {:?} | decisions {:?} | memo {:?}",
                    t0.elapsed(),
                    v.receipt(),
                    v.decisions(),
                    v.plain_memo(),
                );
            }
        }
    }
}

/// The plate's leaf budget for the wall-time row — the plate replay is
/// seven times the slab's, so the row is read at a budget that costs
/// minutes rather than the driver's default, which on this document is
/// a number no lane has ever driven to.
const PLATE_WALL_LEAVES: usize = 256;

/// **The callgrind target for the memo** — `CAD_SYM_MEMO_DOC`
/// (`slab` | `plate`, default `slab`) driven sequentially to
/// `CAD_SYM_MEMO_LEAVES` leaves (default 8) with the plain memo on
/// unless `CAD_SYM_MEMO=0`, and nothing else in the process.
///
/// `m10_sym_profile_interval::sym_profile_callgrind_replay` measures ONE
/// leaf, where a drive memo is empty and can only cost; this row is the
/// same instrument at the scale the memo exists for, so the number it
/// prints is an instruction count rather than a wall time on a shared
/// box. The command that takes it is in the unit's PR body.
#[test]
#[ignore = "evidence-only: the callgrind target — drives one document, nothing else"]
fn sym_memo_callgrind_drive() {
    let tol = Tol::witness();
    let doc = match std::env::var("CAD_SYM_MEMO_DOC").as_deref() {
        Ok("plate") => the_plate(tol),
        _ => slab(),
    };
    let max_leaves: usize = std::env::var("CAD_SYM_MEMO_LEAVES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);
    let plain_memo = std::env::var("CAD_SYM_MEMO").as_deref() != Ok("0");
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let cfg = DriveConfig {
        max_leaves,
        plain_memo,
        ..DriveConfig::default()
    };
    let t0 = Instant::now();
    let v = drive(&doc, &analyzed, &cfg, tol).unwrap();
    println!(
        "drive {max_leaves} leaves memo={plain_memo}: wall {:?} receipt {:?} decisions {:?} \
         memo {:?}",
        t0.elapsed(),
        v.receipt(),
        v.decisions(),
        v.plain_memo()
    );
}

/// **The probe that looks for a RACE**: a drive where two leaves of one
/// level reach a freezing node no earlier level published, so that the
/// leaves' own `frozen` column records which of them got there first.
///
/// Env-driven so that a candidate is a run and not a build:
/// `CAD_NEED_DOC` (`slab` | `plate` | `tilted`), `CAD_NEED_LEAVES`,
/// `CAD_NEED_TERMS` and `CAD_NEED_DEGREE` (the symbolic budget, the
/// dials a tight setting freezes the whole document through),
/// `CAD_NEED_THREADS` (a comma list of rayon widths). Prints each
/// leaf's own column where it is non-zero, the drive's column beside
/// it, and whether the leaf LISTS — what
/// `m10_3_r2_probes_interval::my_own_drive_is_bit_identical_across_repeats_and_schedules`
/// compares — agree across the schedules.
#[test]
#[ignore = "evidence-only: hunts a drive whose leaves race for a freezing node"]
fn the_leaf_column_across_schedules_probe() {
    let tol = Tol::witness();
    let doc = match std::env::var("CAD_NEED_DOC").as_deref() {
        Ok("plate") => the_plate(tol),
        Ok("tilted") => boss_on_tilted(1.0e-3, true),
        _ => slab(),
    };
    let env = |k: &str, d: usize| -> usize {
        std::env::var(k)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(d)
    };
    // Defaulted to the first adversary the gating row drives, so the
    // probe and the row are one spelling of it rather than two.
    let max_leaves = env("CAD_NEED_LEAVES", RACING[0].leaves);
    let dials = editor_core::drive::SymbolicDials {
        max_terms: env("CAD_NEED_TERMS", RACING[0].terms),
        max_degree: env("CAD_NEED_DEGREE", RACING[0].degree as usize) as u32,
        ..editor_core::drive::SymbolicDials::default()
    };
    let threads: Vec<usize> = std::env::var("CAD_NEED_THREADS")
        .unwrap_or_else(|_| "2,4".to_owned())
        .split(',')
        .filter_map(|t| t.parse().ok())
        .collect();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |parallel, plain_memo| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves,
                parallel,
                plain_memo,
                symbolic: dials,
                ..DriveConfig::default()
            },
            tol,
        )
        .unwrap()
    };
    let own = leaf_column;
    let report = |label: &str, v: &ParamBoxVerdict, base: Option<&ParamBoxVerdict>| {
        let o = own(v);
        let nz: Vec<(usize, u64)> = o
            .iter()
            .enumerate()
            .filter(|&(_, &f)| f != 0)
            .map(|(i, &f)| (i, f))
            .collect();
        println!(
            "{label}: receipt {:?} | drive frozen {} | leaves' own sum {} over {} leaves, \
             {} non-zero {:?} | memo {:?}",
            v.receipt(),
            v.decisions().frozen,
            o.iter().sum::<u64>(),
            o.len(),
            nz.len(),
            &nz[..nz.len().min(12)],
            v.plain_memo(),
        );
        if let Some(b) = base {
            println!(
                "  vs base: certified lists equal {} | refused lists equal {} | \
                 serialize equal {} | drive decisions equal {}",
                v.certified() == b.certified(),
                v.refused() == b.refused(),
                v.serialize() == b.serialize(),
                v.decisions() == b.decisions(),
            );
        }
    };
    println!(
        "doc={:?} leaves={max_leaves} dials={dials:?}",
        std::env::var("CAD_NEED_DOC")
    );
    let seq = run(false, true);
    report("seq on", &seq, None);
    for t in &threads {
        let par = on_pool(*t, || run(true, true));
        report(&format!("par@{t} on"), &par, Some(&seq));
    }
    let off = run(false, false);
    report("seq off", &off, Some(&seq));
}
