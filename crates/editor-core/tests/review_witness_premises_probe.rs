//! Reviewer probes for the unlogged lifted closed-form pass (ledger row
//! F48): what an `f64` witness and an `Interval` replay of one document
//! record, predicate by predicate, and what the drive certifies on the
//! plate at members of a certified leaf.
//!
//! The corpus probe prints and asserts parity only for the documents it
//! names as expected-equal; run it with `--nocapture` to read the table.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus;
use crate::fixture::tol;

use corpus::plate_param::{HOLE_R, HOLE_R_VALUE, PLATE_DEPTH};
use corpus::{body_of, eval};
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, RefusalReason, certifying_vector};
use editor_core::range::{RangeField, RangeSeed, derive};
use editor_core::VerdictVector;
use editor_core::{
    Dimension, DocEdit, EvalOutcome, Evaluation, FreeVar, ProfileDoc, VarName, apply,
};
use geom_core::{Interval, Tol};

/// Per predicate, how many definite verdicts a vector holds.
fn counts(v: &VerdictVector) -> BTreeMap<&'static str, usize> {
    let mut out = BTreeMap::new();
    for row in &v.rows {
        for verdict in &row.verdicts {
            *out.entry(verdict.predicate).or_insert(0) += 1;
        }
    }
    out
}

/// The rows of a vector with their node ids dropped: two documents that
/// differ in one variable's value may mint other ids.
fn shape(v: &VerdictVector) -> Vec<(String, Vec<String>)> {
    v.rows
        .iter()
        .map(|r| {
            (
                format!("{:?}", r.outcome),
                r.verdicts.iter().map(|d| format!("{d:?}")).collect(),
            )
        })
        .collect()
}

/// The first index at which two vectors' rows part, as text.
fn first_parting(w: &VerdictVector, l: &VerdictVector) -> String {
    for (i, (a, b)) in w.rows.iter().zip(&l.rows).enumerate() {
        if a.outcome != b.outcome {
            return format!("row {i}: outcome {:?} vs {:?}", a.outcome, b.outcome);
        }
        if a.verdicts != b.verdicts {
            let at = a
                .verdicts
                .iter()
                .zip(&b.verdicts)
                .position(|(x, y)| x != y)
                .unwrap_or(a.verdicts.len().min(b.verdicts.len()));
            return format!(
                "row {i}: {} vs {} verdicts, first parting #{at}: {:?} vs {:?}",
                a.verdicts.len(),
                b.verdicts.len(),
                a.verdicts.get(at),
                b.verdicts.get(at)
            );
        }
    }
    format!("{} vs {} rows", w.rows.len(), l.rows.len())
}

/// The curved corpus documents the probe reads: name, and whether the
/// `Interval` point replay is expected to state the witness's vector.
const CURVED: &[&str] = &[
    "plate_param",
    "cone_on_sphere",
    "cut_cylinder",
    "cup",
    "pin_in_bore",
    "boss",
];

/// **An `f64` witness and an `Interval` replay at the same point state
/// one verdict vector on the curved corpus documents**, and every
/// `props_*` kind the witness records the replay records as often.
#[test]
fn a_point_replay_states_the_witnesss_vector_on_curved_documents() {
    let only = std::env::var("REVIEW_WITNESS_DOCS").ok();
    let mut parted = Vec::new();
    for doc in corpus::documents() {
        let wanted = match &only {
            Some(list) if list == "all" => true,
            Some(list) => list.split(',').any(|n| n == doc.name),
            None => CURVED.contains(&doc.name),
        };
        if !wanted {
            continue;
        }
        let t = std::time::Instant::now();
        let witness: Evaluation<f64> = eval(&doc.doc);
        let leaf: Evaluation<Interval> = eval(&doc.doc);
        let (wv, lv) = (
            certifying_vector(&doc.doc, &witness),
            certifying_vector(&doc.doc, &leaf),
        );
        let (wc, lc) = (counts(&wv), counts(&lv));
        let props: Vec<String> = wc
            .iter()
            .filter(|(name, _)| name.starts_with("props_"))
            .map(|(name, n)| format!("{name} {n}:{}", lc.get(name).copied().unwrap_or(0)))
            .collect();
        let excess: Vec<String> = wc
            .iter()
            .filter(|(name, n)| lc.get(*name).copied().unwrap_or(0) != **n)
            .map(|(name, n)| format!("{name} {n}:{}", lc.get(name).copied().unwrap_or(0)))
            .chain(
                lc.iter()
                    .filter(|(name, _)| !wc.contains_key(*name))
                    .map(|(name, n)| format!("{name} 0:{n}")),
            )
            .collect();
        let equal = wv == lv;
        println!(
            "REVIEW {}: witness {:?} leaf {:?} equal={equal} ({:.1}s)\n  props witness:leaf {}\n  \
             counts that differ: {}\n  {}",
            doc.name,
            witness.outcome,
            leaf.outcome,
            t.elapsed().as_secs_f64(),
            props.join(", "),
            excess.join(", "),
            if equal {
                String::new()
            } else {
                first_parting(&wv, &lv)
            }
        );
        if !equal {
            parted.push(doc.name);
        }
    }
    assert!(parted.is_empty(), "vectors part on {parted:?}");
}

fn set_hole_r(doc: &ProfileDoc, value: f64) -> ProfileDoc {
    apply(
        doc,
        &DocEdit::DefineVar {
            var: VarName::from_static(HOLE_R).into(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("DefineVar does not refuse")
    .doc
}

fn volume(doc: &ProfileDoc, solid: editor_core::RecipeNodeId) -> f64 {
    let ev: Evaluation<f64> = eval(doc);
    assert_eq!(ev.outcome, EvalOutcome::Completed);
    topo::test_support::pinned_volume(
        "member",
        &topo::test_support::finished("member", topo::Body::clone(&body_of(&ev, solid)), tol()),
        tol(),
    )
    .volume
}

/// **What certifies over the plate's `[0, ε/4]` leaf is the witness's
/// build at members of the leaf**: an `f64` build at each sampled
/// member states the witness's decisions row for row, and its volume
/// is the witness's less the two holes' growth.
#[test]
fn members_of_the_plates_certified_leaf_build_as_the_witness_did() {
    let corpus_doc = corpus::plate_param::document();
    let doc = corpus_doc.doc;
    let solid = corpus_doc.result.unwrap();
    let field = RangeField::Param(doc.var_named(HOLE_R).unwrap());
    let width = tol().eps() / 4.0;
    let derived = derive(&doc, &field, RangeSeed { lo: 0.0, hi: width }, tol()).expect("admitted");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let level = DriveConfig {
        max_depth: 0,
        ..DriveConfig::default()
    };
    let stop = editor_core::Stop::default();
    let verdict = editor_core::drive::drive(&derived.doc, &analyzed, &level, &stop, tol())
        .expect("the plate builds");
    assert_eq!(verdict.certified().len(), 1, "{:?}", verdict.refused());

    let witness: Evaluation<f64> = eval(&doc);
    let wshape = shape(&certifying_vector(&doc, &witness));
    let v0 = volume(&doc, solid);
    for k in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let s = k * width;
        let member = set_hole_r(&doc, HOLE_R_VALUE + s);
        let ev: Evaluation<f64> = eval(&member);
        assert_eq!(ev.outcome, EvalOutcome::Completed, "member {k} builds");
        assert!(
            shape(&certifying_vector(&member, &ev)) == wshape,
            "the member at {k} of the leaf decides other than the witness"
        );
        let r = HOLE_R_VALUE + s;
        let grown = 2.0
            * std::f64::consts::PI
            * (r * r - HOLE_R_VALUE * HOLE_R_VALUE)
            * PLATE_DEPTH;
        let v = volume(&member, solid);
        println!("REVIEW member {k}: volume {v:.17e}, witness less growth {:.17e}", v0 - grown);
        assert!(
            (v - (v0 - grown)).abs() < 1e-13,
            "member {k}: volume {v} against {}",
            v0 - grown
        );
    }
}

/// What a depth-0 drive says of the plate's one leaf at each width, and
/// of a leaf across the two holes' tangency: printed.
#[test]
fn what_the_drive_says_of_wider_leaves_and_of_a_leaf_across_a_tangency() {
    let eps = tol().eps();
    let report = |label: &str, doc: &ProfileDoc, seed: RangeSeed, depth: u32| {
        let field = RangeField::Param(doc.var_named(HOLE_R).unwrap());
        let derived = match derive(doc, &field, seed, tol()) {
            Ok(d) => d,
            Err(e) => {
                println!("REVIEW {label}: not admitted: {e:?}");
                return (0, Vec::new());
            }
        };
        let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
        let level = DriveConfig {
            max_depth: depth,
            ..DriveConfig::default()
        };
        let stop = editor_core::Stop::default();
        match editor_core::drive::drive(&derived.doc, &analyzed, &level, &stop, tol()) {
            Ok(verdict) => {
                let reasons: Vec<String> = verdict
                    .refused()
                    .iter()
                    .map(|leaf| match &leaf.reason {
                        RefusalReason::FlipCrossing { flipped: evidence } => format!(
                            "flip_crossing(empty={}, {:?})",
                            evidence.is_empty(),
                            evidence.verdicts
                        ),
                        other => format!("{}: {other:?}", other.class().name()),
                    })
                    .collect();
                println!(
                    "REVIEW {label}: certified {} refused {}: {:#?}",
                    verdict.certified().len(),
                    reasons.len(),
                    reasons
                );
                (verdict.certified().len(), reasons)
            }
            Err(e) => {
                println!("REVIEW {label}: drive refused: {e:?}");
                (0, Vec::new())
            }
        }
    };
    let plate = corpus::plate_param::document().doc;
    for (label, hi) in [("eps/2", eps / 2.0), ("eps", eps), ("32 eps", 32.0 * eps)] {
        let _ = report(label, &plate, RangeSeed { lo: 0.0, hi }, 0);
    }
    // The holes' centres are 1.2 apart: they touch at r = 0.6. A witness
    // 20 ε short of it, and leaves reaching to, into and past the band.
    let near = set_hole_r(&plate, 0.6 - 20.0 * eps);
    for (label, hi, depth) in [
        ("tangency: [0, eps/4] clear of the band", eps / 4.0, 0),
        ("tangency: [0, 40 eps] across it, depth 0", 40.0 * eps, 0),
        ("tangency: [0, 40 eps] across it, depth 3", 40.0 * eps, 3),
    ] {
        let (certified, _) = report(label, &near, RangeSeed { lo: 0.0, hi }, depth);
        if hi > eps {
            assert_eq!(certified, 0, "{label}: a leaf across the tangency certified");
        }
    }
}
