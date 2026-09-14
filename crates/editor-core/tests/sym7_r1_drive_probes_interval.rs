//! **SYM-7 review probes (R1)** — a document the unit did not measure,
//! driven with the plain memo on and off, sequentially and in parallel.
//!
//! The slab freezes nothing at all (`m10_sym_drive_memo_interval`'s
//! growth-guard row reads `frozen: 0` on it), so the only document in
//! the unit's own rows that exercises the memo's FROZEN set is the
//! plate. This file drives a third: a boss on a DERIVED frame over a
//! TILTED datum, whose re-normalised axis forms freeze on degree
//! (`m10_derived_frame_tilted_interval`'s header). The document is
//! built here rather than borrowed so the reading is independent of
//! the fixtures the unit tuned its ceilings against.
//!
//! Thread count is set by `RAYON_NUM_THREADS` on the process, since the
//! driver takes the global pool; the row prints
//! `rayon::current_num_threads` equivalent as the worker count the
//! drive actually saw is not exposed.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, Recorder, ang, len, scl};

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, ParamBoxVerdict, drive};
use editor_core::{
    CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, Expr, Node, ParamName, ProfileDoc,
    RoleSeg, UnitSym,
};
use geom_core::Tol;

/// R1's own document: a boss on a face frame DERIVED from a datum whose
/// `v` axis carries the parameter, so the stored unit vector's `√S` is
/// a sqrt of a non-constant form and the renormalised forms freeze.
/// Deliberately not any fixture the unit measured — its own tilt, its
/// own two extrusion depths and its own box half-width.
fn r1_tilted_boss() -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("tilt"),
        value: DocParam::Continuous {
            dim: Dimension::Scalar,
            value: 0.375,
            display_unit: UnitSym::canonical_for(Dimension::Scalar),
            distribution: Some(Distribution::Uniform {
                lo: -2.0e-2,
                hi: 2.0e-2,
            }),
        },
    });
    let t = Expr::param(ParamName::new("tilt"), Dimension::Scalar);
    let base = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), t],
    }));
    let plate = r.insert(Node::Profile(fixture::desc(
        base,
        vec![fixture::square(0.0, 0.0, 0.75)],
    )));
    let block = r.insert(Node::Extrude {
        profile: plate,
        distance: len(0.625),
    });
    let derived = r.insert(Node::Datum(Datum::FaceFrame {
        at: block,
        face: fixture::fname(block, RoleSeg::Cap(CapEnd::End)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        derived,
        vec![fixture::square(0.1, -0.05, 0.3)],
    )));
    r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.125),
    });
    r.doc
}

const R1_LEAVES: usize = 48;

fn run(doc: &ProfileDoc, plain_memo: bool, parallel: bool, max_leaves: usize) -> ParamBoxVerdict {
    let tol = Tol::witness();
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    drive(
        doc,
        &analyzed,
        &DriveConfig {
            max_leaves,
            parallel,
            plain_memo,
            ..DriveConfig::default()
        },
        tol,
    )
    .unwrap()
}

/// Every leaf verdict and every per-leaf decision count, with the
/// leaf's own `frozen` kept OUT of the comparison of the memo-on lane
/// against the memo-off one (it is a work measure — the unit's own
/// filed row) but kept IN the comparison of two schedules on the same
/// dial, where nothing should move it at all.
fn leaf_rows(v: &ParamBoxVerdict, with_frozen: bool) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for l in v.certified() {
        out.push(format!(
            "C {:?} {:?} {:?} {:?}",
            l.box_,
            l.verdict_vector_key,
            l.results,
            scrub(l.decisions, with_frozen)
        ));
    }
    for l in v.refused() {
        out.push(format!(
            "R {:?} {:?} {:?}",
            l.box_,
            l.reason,
            scrub(l.decisions, with_frozen)
        ));
    }
    out
}

fn scrub(c: geom_core::SymCounts, with_frozen: bool) -> geom_core::SymCounts {
    if with_frozen {
        c
    } else {
        geom_core::SymCounts { frozen: 0, ..c }
    }
}

/// **The end-to-end exercise.** One document the unit never measured,
/// driven four ways: the memo on and off, sequential and parallel.
/// Every leaf verdict, every per-leaf decision count and the drive's
/// whole serialization must agree, `frozen` included on the drive's
/// own receipt.
#[test]
fn r1_own_document_is_identical_with_the_memo_on_and_off_and_across_schedules() {
    let doc = r1_tilted_boss();
    let seq_on = run(&doc, true, false, R1_LEAVES);
    let seq_off = run(&doc, false, false, R1_LEAVES);
    let par_on = run(&doc, true, true, R1_LEAVES);
    let par_off = run(&doc, false, true, R1_LEAVES);
    println!(
        "threads {} | leaves c{} r{} | memo {:?}\n  seq_on  {:?} {:?}\n  seq_off {:?} {:?}\n  \
         par_on  {:?} {:?}\n  par_off {:?} {:?}",
        std::thread::available_parallelism().map_or(0, std::num::NonZeroUsize::get),
        seq_on.certified().len(),
        seq_on.refused().len(),
        seq_on.plain_memo(),
        seq_on.receipt(),
        seq_on.decisions(),
        seq_off.receipt(),
        seq_off.decisions(),
        par_on.receipt(),
        par_on.decisions(),
        par_off.receipt(),
        par_off.decisions(),
    );
    // Non-vacuity: the document must actually freeze, or the row says
    // nothing about the column the unit moved.
    assert!(
        seq_on.plain_memo().frozen > 0,
        "the document froze nothing, so it does not exercise the drive's frozen set: {:?}",
        seq_on.plain_memo()
    );
    assert!(
        seq_on.certified().len() + seq_on.refused().len() > 1,
        "one leaf is not a drive"
    );
    for (label, other) in [
        ("seq memo off", &seq_off),
        ("par memo on", &par_on),
        ("par memo off", &par_off),
    ] {
        assert_eq!(
            seq_on.serialize(),
            other.serialize(),
            "{label}: the serialization moved"
        );
        assert_eq!(
            seq_on.content_key(),
            other.content_key(),
            "{label}: the content key moved"
        );
        assert_eq!(
            seq_on.decisions(),
            other.decisions(),
            "{label}: the drive's decision columns moved, `frozen` included"
        );
        assert_eq!(
            leaf_rows(&seq_on, false),
            leaf_rows(other, false),
            "{label}: a leaf verdict or a per-leaf decision count moved"
        );
    }
    // Same dial, two schedules: nothing at all may move, the leaves'
    // own `frozen` included with the memo OFF (where no leaf inherits).
    assert_eq!(
        leaf_rows(&seq_off, true),
        leaf_rows(&par_off, true),
        "memo off: two schedules disagreed on a leaf's own frozen count"
    );
    // And with the memo ON, this is the filed residue: the per-leaf
    // column can move between schedules. Printed, not asserted.
    println!(
        "memo on, leaves' own frozen: seq {} par {} | memo off: seq {} par {}",
        own_frozen(&seq_on),
        own_frozen(&par_on),
        own_frozen(&seq_off),
        own_frozen(&par_off),
    );
}

fn own_frozen(v: &ParamBoxVerdict) -> u64 {
    v.certified()
        .iter()
        .map(|l| l.decisions.frozen)
        .chain(v.refused().iter().map(|l| l.decisions.frozen))
        .sum()
}

/// **The memo is dropped with the drive.** Two drives of DIFFERENT
/// documents back to back: the second's memo must be the size the
/// second document builds alone, so no form of the first can have
/// served it.
#[test]
fn a_memo_from_one_drive_never_serves_the_next() {
    let a = r1_tilted_boss();
    let b = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, Tol::witness()).0;
    let solo_b = run(&b, true, false, 8).plain_memo();
    let _first = run(&a, true, false, R1_LEAVES);
    let after_a = run(&b, true, false, 8).plain_memo();
    println!("solo B {solo_b:?}\nB after A {after_a:?}");
    assert_eq!(
        solo_b, after_a,
        "the second drive's memo differs depending on what ran before it"
    );
}

/// **Is the unrecorded-node path reachable from a drive at all?**
///
/// `form_in` freezes a node absent from `Session::nodes` and PUBLISHES
/// that frozen form to the drive memo under the node's content id,
/// while refusing to READ one for such a node. The asymmetry only
/// bites if some node is unrecorded in one leaf and recorded in
/// another, so this row counts the unrecorded freezes a drive of each
/// document takes. Evidence-only: it prints the census.
#[test]
#[ignore = "evidence-only: the unrecorded-freeze census over a drive"]
fn r1_unrecorded_freezes_over_a_drive() {
    use geom_core::sym::profile::{FreezeCause, start_profile, take_profile};
    for (label, doc, leaves) in [
        ("r1_tilted", r1_tilted_boss(), R1_LEAVES),
        (
            "plate",
            crate::m10_7_plate::plate(5.0e-5, 1.0e-5, Tol::witness()).0,
            16,
        ),
    ] {
        start_profile();
        let v = run(&doc, true, false, leaves);
        let p = take_profile();
        let unrecorded = p
            .freezes
            .iter()
            .filter(|f| f.cause == FreezeCause::Unrecorded)
            .count();
        println!(
            "{label}: sessions {} | freezes {} | UNRECORDED {unrecorded} | memo {:?}",
            p.sessions,
            p.freezes.len(),
            v.plain_memo()
        );
    }
}

/// **Can the opaque-sequence row go red?** (Q3.) The unit's premise
/// pin asserts that every leaf of a drive mints the same SET of
/// `Opaque` ids. A set comparison over empty sets passes for free, so
/// this row prints how big those sets actually are on each document —
/// the only thing that decides whether the pin is an alarm or a
/// tautology.
#[test]
#[ignore = "evidence-only: the size of the per-leaf Opaque id sets the premise pin compares"]
fn r1_opaque_set_sizes_the_premise_pin_compares() {
    use geom_core::sym::profile::{start_profile, take_profile};
    for (label, doc, leaves) in [
        ("r1_tilted", r1_tilted_boss(), 8),
        (
            "slab",
            crate::m10_3_r1_probes_interval::bounded_chamber(
                60.0 * Tol::witness().eps(),
                30.0 * Tol::witness().eps(),
                100.0 * Tol::witness().eps(),
            ),
            8,
        ),
        (
            "plate",
            crate::m10_7_plate::plate(5.0e-5, 1.0e-5, Tol::witness()).0,
            8,
        ),
    ] {
        start_profile();
        let _ = run(&doc, true, false, leaves);
        let p = take_profile();
        let sizes: Vec<usize> = p
            .opaque_ids
            .iter()
            .map(std::collections::BTreeSet::len)
            .collect();
        println!(
            "{label}: {} leaf sets, sizes {:?}",
            sizes.len(),
            &sizes[..sizes.len().min(12)]
        );
    }
}
