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
];

use std::time::Instant;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, ParamBoxVerdict, drive};
use geom_core::{SymBudget, SymRules, Tol};
use geom_core::sym::profile::{start_profile, take_profile};

use crate::m10_3_r1_probes_interval::{CHAMBER_LEAVES, bounded_chamber};
use crate::m10_7_plate::plate;

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
    // Sequentially: the profile is thread-local, so the sequential
    // schedule is the only one it can see whole.
    let cfg = config(max_leaves);
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

/// **The ceiling on the plate** — the two-hole plate at the driver's
/// default leaf budget.
#[test]
#[ignore = "evidence-only: the plain memo's hit-rate ceiling over the plate drive"]
fn plain_memo_ceiling_plate_drive() {
    let tol = Tol::witness();
    ceiling("plate", &the_plate(tol), DriveConfig::default().max_leaves);
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

/// **The premise the drive memo rests on, by execution.**
///
/// A `Param`'s indeterminate is its symbol, a `Lit`'s its bits, an
/// atom's the digests of its arguments' forms — all leaf-invariant. An
/// `Opaque`'s is the SEQUENCE NUMBER the leaf minted it at
/// (`Sym::opaque`, `OPAQUE_SEQ`), and that is leaf-invariant only
/// because the evaluation service walks each leaf's recipe in the same
/// fixed order. If a lane ever mints an opaque under a value-dependent
/// branch, two leaves give one id to two different unknowns, and a
/// plain form memoized across them is a form for an expression the
/// second leaf never built — an unsound `Zero`, not a slow one. This
/// row is the alarm for that, and it is the FIRST thing that should red
/// if it happens.
#[test]
fn the_opaque_sequence_is_identical_across_the_leaves_of_a_drive() {
    for (label, doc) in [("slab", slab()), ("plate", the_plate(Tol::witness()))] {
        let sets = opaque_sets(&doc, PIN_LEAVES);
        assert!(
            sets.len() > 1,
            "{label}: the premise is about AGREEMENT ACROSS leaves, so the drive must produce \
             more than one: {} leaves",
            sets.len()
        );
        let first = &sets[0];
        for (i, s) in sets.iter().enumerate().skip(1) {
            assert_eq!(
                s,
                first,
                "{label}: leaf {i} minted a different set of Opaque ids than leaf 0 \
                 ({} against {}) — the drive-scoped plain memo is UNSOUND on this document \
                 until it holds, because an Opaque id is a per-leaf sequence number and two \
                 leaves would be giving one id to two different unknowns",
                s.len(),
                first.len()
            );
        }
    }
}

/// Everything a leaf's receipt says that is a CLAIM about that leaf's
/// predicates — the decision columns, without `frozen`, which is a
/// measure of the work that leaf happened to do and is exactly what
/// the memo moves (`SymCounts::frozen`).
fn decisions_of(c: geom_core::SymCounts) -> geom_core::SymCounts {
    geom_core::SymCounts { frozen: 0, ..c }
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
/// What the row must NOT compare is a LEAF's own `frozen`: with the
/// memo on, a leaf inherits forms another leaf froze, so that column
/// falls, which is the saving the unit is for.
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
            assert_eq!(a.results, b.results, "{label}: certified leaf {i}'s results");
            assert_eq!(
                decisions_of(a.decisions),
                decisions_of(b.decisions),
                "{label}: certified leaf {i}'s decision counts"
            );
        }
        for (i, (a, b)) in on.refused().iter().zip(off.refused()).enumerate() {
            assert_eq!(a.box_, b.box_, "{label}: refused leaf {i}'s box");
            assert_eq!(a.reason, b.reason, "{label}: refused leaf {i}'s reason");
            assert_eq!(
                decisions_of(a.decisions),
                decisions_of(b.decisions),
                "{label}: refused leaf {i}'s decision counts"
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
        let own = |v: &ParamBoxVerdict| -> u64 {
            v.certified()
                .iter()
                .map(|l| l.decisions.frozen)
                .chain(v.refused().iter().map(|l| l.decisions.frozen))
                .sum()
        };
        println!(
            "{label}: memo {memo:?} | leaves' own frozen on {} off {} | drive frozen {}",
            own(&on),
            own(&off),
            on.decisions().frozen
        );
        assert!(
            own(&on) <= own(&off),
            "{label}: a leaf under the memo cannot do MORE freezing than one without it"
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

/// **The memo is valid for one budget and one set of dials**, and says
/// so rather than serving a form the leaf would not have built: the
/// plain walk consults `SymRules::const_fold` and `SymRules::early`,
/// and `within` consults the budget, so both are part of the form's
/// identity and neither is part of the key it is stored under.
#[test]
fn a_drive_memo_refuses_a_leaf_it_was_not_made_for() {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let other = SymBudget {
        max_terms: 64,
        max_degree: 128,
    };
    let memo = geom_core::sym::DriveMemo::new(budget, SymRules::shipped());
    assert!(memo.accepts(budget, SymRules::shipped()));
    assert!(
        !memo.accepts(other, SymRules::shipped()),
        "a different budget freezes different nodes, so the forms are different forms"
    );
    assert!(
        !memo.accepts(budget, SymRules::none()),
        "the plain walk reads `const_fold` and `early`, so the dials are part of a form's identity"
    );
}

/// **The memo is dropped with the drive** and holds nothing of the next
/// one: two drives of the same document report the same size, which a
/// memo that survived would not — the second would start full and end
/// having built nothing.
#[test]
fn the_memo_holds_nothing_of_the_previous_drive() {
    let doc = slab();
    let tol = Tol::witness();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = || {
        drive(&doc, &analyzed, &config(PIN_LEAVES), tol)
            .unwrap()
            .plain_memo()
    };
    let first = run();
    let second = run();
    assert!(first.forms > 0, "the drive built forms: {first:?}");
    assert_eq!(
        first, second,
        "a second drive of one document builds its memo again from nothing"
    );
}
