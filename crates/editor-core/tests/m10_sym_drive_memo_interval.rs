//! **The drive-scoped plain memo** (`geom_core::sym::DriveMemo`): the
//! measurement that sizes it, the premise it rests on, and the pins
//! that keep it honest.
//!
//! A node's plain form is a function of its id and the session's budget
//! alone — the id is a content hash of `(op, payload, kids)` and the
//! plain walk reads no value — so a form computed on one leaf of a
//! drive is the form every other leaf of that drive would build. The
//! rows here are the two things that claim rests on:
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
//! The differential rows that compare the memo on against the memo off
//! join them with the memo itself.
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
use editor_core::drive::{DriveConfig, drive};
use geom_core::Tol;
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
