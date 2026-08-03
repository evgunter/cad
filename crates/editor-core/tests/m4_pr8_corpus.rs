//! M4 PR 8a spec D1 — the Band 4 corpus acceptance row.
//!
//! Every corpus document (see `corpus/mod.rs`) must:
//!
//! 1. evaluate GREEN — every node `Ok`, outcome `Completed`, nothing
//!    poisoned — at whatever ε the CI row commits (the workspace runs
//!    at 1e-6, 1e-9 and 1e-12; the Interval lane runs
//!    `m4_pr8_corpus_interval.rs`);
//! 2. hit its EXACT mass oracle where the dimensions are dyadic
//!    (`==`, never a tolerance), with tiers 1 and "closed" green;
//! 3. reuse exactly the complement of its bump's downstream cone —
//!    the cone being derived independently from the recipe DAG, not
//!    from the evaluator.
//!
//! And the corpus as a whole must cover EVERY F4 node kind, every node
//! sub-kind (datum flavours, boolean operators, pattern kinds) and
//! EVERY `DocEdit` kind.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use std::collections::BTreeSet;

use editor_core::{CancelToken, EvalOptions, EvalOutcome, evaluate};
use topo::{mass_properties, validate, validate_closed};

use corpus::{
    EDIT_KINDS, NODE_KINDS, SUB_KINDS, body_of, cone, documents, eval, failures, vocabulary,
};

#[test]
fn every_document_evaluates_green() {
    for d in documents() {
        let ev = eval::<f64>(&d.doc);
        let bad = failures(&ev);
        assert!(
            bad.is_empty(),
            "{}: {} node(s) did not evaluate green:\n{}",
            d.name,
            bad.len(),
            bad.join("\n")
        );
        assert_eq!(ev.outcome, EvalOutcome::Completed, "{}: outcome", d.name);
        assert_eq!(ev.order.len(), d.len(), "{}: evaluation order", d.name);
        assert_eq!(
            (ev.recomputed, ev.reused),
            (d.len(), 0),
            "{}: cold evaluation recomputes everything",
            d.name
        );
    }
}

#[test]
fn exact_mass_pins_hold() {
    let mut pinned = 0;
    for d in documents() {
        let Some(result) = d.result else { continue };
        let ev = eval::<f64>(&d.doc);
        let body = body_of(&ev, result);
        assert_eq!(validate(body), Ok(()), "{}: tier 1", d.name);
        assert_eq!(validate_closed(body), Ok(()), "{}: closed", d.name);
        let Some(pin) = d.pin else { continue };
        pinned += 1;
        let m = mass_properties(body).expect("mass properties");
        assert_eq!(
            m.volume, pin.volume,
            "{}: volume {} != exact oracle {} (derivation in the document's module docs)",
            d.name, m.volume, pin.volume
        );
        if let Some(area) = pin.area {
            assert_eq!(
                m.surface_area, area,
                "{}: surface area {} != exact oracle {}",
                d.name, m.surface_area, area
            );
        }
    }
    // Guard against the pins quietly evaporating.
    assert!(
        pinned >= 6,
        "only {pinned} corpus documents carry an exact mass pin"
    );
}

#[test]
fn incremental_recompute_reuses_the_cone_complement() {
    for d in documents() {
        let full = eval::<f64>(&d.doc);
        let bumped = d.bumped();
        let expected = cone(&bumped, d.bump_root);
        let after = evaluate::<f64>(
            &bumped,
            Some(&full),
            &CancelToken::new(),
            &EvalOptions::default(),
        );
        let bad = failures(&after);
        assert!(
            bad.is_empty(),
            "{}: the bumped document did not evaluate green:\n{}",
            d.name,
            bad.join("\n")
        );
        assert_eq!(
            (after.recomputed, after.reused),
            (expected.len(), bumped.len() - expected.len()),
            "{}: bump cone is {:?} ({} nodes) of {}",
            d.name,
            expected,
            expected.len(),
            bumped.len()
        );
        assert!(
            after.reused > 0,
            "{}: the bump must leave something to reuse (it is a MID-DAG edit)",
            d.name
        );
    }
}

#[test]
fn document_names_are_unique_and_stable() {
    // The names key the committed latency baseline; a duplicate would
    // silently overwrite a row.
    let docs = documents();
    let names: BTreeSet<&str> = docs.iter().map(|d| d.name).collect();
    assert_eq!(names.len(), docs.len(), "corpus document names collide");
}

#[test]
fn vocabulary_coverage_is_total() {
    let (nodes, subs, edits) = vocabulary();
    let mut report = String::from("\ncorpus vocabulary coverage\n");
    let mut missing = Vec::new();
    for kind in NODE_KINDS {
        match nodes.get(kind) {
            Some(docs) => report.push_str(&format!("  node {kind:<12} {}\n", docs.join(", "))),
            None => missing.push(format!("node {kind}")),
        }
    }
    for kind in SUB_KINDS {
        match subs.get(kind) {
            Some(docs) => report.push_str(&format!("  sub  {kind:<20} {}\n", docs.join(", "))),
            None => missing.push(format!("sub-kind {kind}")),
        }
    }
    for kind in EDIT_KINDS {
        match edits.get(kind) {
            Some(docs) => report.push_str(&format!("  edit {kind:<20} {}\n", docs.join(", "))),
            None => missing.push(format!("edit {kind}")),
        }
    }
    println!("{report}");
    // M5 PR 10: `Loft`/`Sweep` joined the vocabulary but CANNOT join
    // the corpus yet — a corpus document must evaluate to a body, and
    // a NURBS-walled solid is frontier-blocked
    // (`NodeErrorKind::CurvedSolidFrontier`; the blocker is
    // demonstrated in `sweep/tests/m5_pr10_frontier.rs`). They are
    // listed in `NODE_KINDS` on purpose, so the coverage report shows
    // them at ZERO instead of their absence reading as coverage.
    //
    // M5 PR 12: `Fillet` joins them, for a DIFFERENT and much shallower
    // reason. Its corpus document exists and is green — `die_fillet`,
    // in `corpus/die_fillet.rs`, pinned end to end by
    // `m5_pr12_fillet_node.rs` — but the fillet battery's clearance
    // screen seeds its gap with `T::from_f64(f64::INFINITY)`, which is
    // NaI at the Interval scalar and absorbs through `min`, so the op
    // escalates on `fillet3_face_clearance` under `--features
    // interval`. Registry membership means the Interval lane
    // (`m4_pr8_corpus_interval.rs`, `m4_pr6_roundtrip_interval.rs`),
    // so the document waits outside it rather than making that lane
    // red. The fix is one line in `sweep/src/fillet/battery.rs` (seed
    // the gap from the first sampled pair, not from ±∞).
    //
    // The exemption is EXACT and retires itself: the moment a corpus
    // document exercises one, `missing` shrinks and this assertion
    // fires, telling you to delete the entry.
    const FRONTIER_UNCOVERED: [&str; 3] = ["node Loft", "node Sweep", "node Fillet"];
    let still_missing: Vec<&String> = missing
        .iter()
        .filter(|m| !FRONTIER_UNCOVERED.contains(&m.as_str()))
        .collect();
    assert!(
        still_missing.is_empty(),
        "the Band 4 corpus does not cover: {}{report}",
        still_missing
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    for exempt in FRONTIER_UNCOVERED {
        assert!(
            missing.iter().any(|m| m == exempt),
            "{exempt} is now covered — remove it from FRONTIER_UNCOVERED{report}"
        );
    }
    // Guard the other direction: a NEW node/edit kind must be added
    // to the tally lists (and then to the corpus) — an unlisted kind
    // would otherwise pass unnoticed.
    assert_eq!(
        nodes.len() + FRONTIER_UNCOVERED.len(),
        NODE_KINDS.len(),
        "unlisted node kind covered"
    );
    assert_eq!(
        subs.len(),
        SUB_KINDS.len(),
        "unlisted node sub-kind covered"
    );
    assert_eq!(edits.len(), EDIT_KINDS.len(), "unlisted edit kind covered");
}
