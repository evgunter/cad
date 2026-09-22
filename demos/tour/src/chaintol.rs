//! **The four-link chain on the CERTIFIED lane** — what an enclosure
//! can say about the picture [`crate::mcchain`] draws, measured rather
//! than assumed.
//!
//! Same document as the density sheet (both take it from
//! [`crate::chain`]), same study. That cell draws 512 built chains and
//! summarizes them; this one asks the other question — what holds over
//! a BOX of joint angles, with no sampling at all — and reports what
//! it finds, including where it stops.
//!
//! Behind the `interval` feature, because the certified scalar is its
//! entire subject.
//!
//! # Why this is a measurement and not a demonstration
//!
//! A widened ROTATION ANGLE has never been carried on this lane. The
//! one widened `Node::Transform` in the tree
//! (`editor-core/tests/m10_derived_frame_interval.rs`) widens the
//! TRANSLATION and pins `rotation_angle` at exactly zero, so interval
//! `sin`/`cos` of a parameter has never reached a rigid map. This cell
//! is the first document that does it, and the table below is the
//! answer either way: the walls it finds are filed, not fixed here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;
use std::time::Instant;

use pncad::analysis::{AnalysisPolicy, DriveConfig, ParamBox, analyzed_box, assertion_at, drive};
use pncad::document::{
    CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, ProfileLift, RecipeNodeId,
    evaluate,
};
use pncad::geom_core::{Interval, Sym, SymBudget, SymCounts, SymRules, Tol};

use crate::chain::{Chain, JOINT_SIGMA, LINKS, POSITION_BOUND, chain};

/// Metres to millimetres, for every printed number.
const MM: f64 = 1e3;

/// **The driver's own symbolic dials, read off its default config.**
///
/// `DriveConfig::symbolic` is a public field of a public type whose
/// own type — `editor_core::drive::SymbolicDials` — is not on the
/// façade, so a consumer can READ the driver's budget and rules but
/// cannot name them, declare one, or build a modified copy. Reading
/// the three fields off `DriveConfig::default()` is the whole of what
/// is reachable from out here, and it is what this cell does rather
/// than restating `4096` and `128` as literals.
/// (`work/sym/the-drivers-symbolic-dials-have-no-name-on-the-facade`.)
fn drive_dials() -> (SymBudget, SymRules) {
    let dials = DriveConfig::default().symbolic;
    (
        SymBudget {
            max_terms: dials.max_terms,
            max_degree: dials.max_degree,
        },
        dials.rules,
    )
}

/// One leaf's evaluation options over the WHOLE declared box, in the
/// driver's own lane (`ProfileLift::Guided`, as `drive` sets it).
fn whole_box(doc: &ProfileDoc) -> EvalOptions {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    }
}

/// Every node that failed or was poisoned, in evaluation order —
/// the first entry is the wall, and its text names the predicate.
fn failures<T: pncad::geom_core::Decide>(ev: &Evaluation<T>) -> Vec<String> {
    ev.order
        .iter()
        .filter_map(|id| match ev.result(*id) {
            Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
            Some(NodeResult::Poisoned { through }) => {
                Some(format!("node {} poisoned through {}", id.0, through.0))
            }
            _ => None,
        })
        .collect()
}

/// One row of the table: a link count on a scalar lane.
struct Row {
    links: usize,
    lane: &'static str,
    /// Nothing failed and nothing was poisoned over the whole box.
    certifies: bool,
    /// How many nodes failed or were poisoned.
    refused: usize,
    /// The first refusal, verbatim — it names the predicate.
    first: Option<String>,
    /// Wall-clock of the one leaf.
    cost: std::time::Duration,
    /// The session's decision counts; all zero on the plain `Interval`
    /// lane, which runs no session at all.
    counts: SymCounts,
}

/// One leaf over the whole box at plain `Interval`.
fn interval_leaf(links: usize, doc: &ProfileDoc) -> Row {
    let opts = whole_box(doc);
    let t0 = Instant::now();
    let ev: Evaluation<Interval> = evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
    let cost = t0.elapsed();
    let bad = failures(&ev);
    Row {
        links,
        lane: "Interval",
        certifies: bad.is_empty(),
        refused: bad.len(),
        first: bad.first().cloned(),
        cost,
        counts: SymCounts::default(),
    }
}

/// One leaf over the whole box at `Sym<Interval>`, under the driver's
/// own budget and rules.
fn sym_leaf(links: usize, doc: &ProfileDoc) -> Row {
    let opts = whole_box(doc);
    let (budget, rules) = drive_dials();
    let t0 = Instant::now();
    let (bad, counts) = pncad::geom_core::sym::with_session_rules(budget, rules, || {
        let ev: Evaluation<Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
        failures(&ev)
    });
    let cost = t0.elapsed();
    Row {
        links,
        lane: "Sym<Interval>",
        certifies: bad.is_empty(),
        refused: bad.len(),
        first: bad.first().cloned(),
        cost,
        counts,
    }
}

fn print_row(r: &Row) {
    println!(
        "   {:>5} link{}  {:<14}  {:<9}  {:>8.2} s  {:>4} refused  {}",
        r.links,
        if r.links == 1 { " " } else { "s" },
        r.lane,
        if r.certifies { "CERTIFIES" } else { "refuses" },
        r.cost.as_secs_f64(),
        r.refused,
        match &r.first {
            Some(f) => f.as_str(),
            None => "—",
        }
    );
    if r.counts != SymCounts::default() {
        println!("          {:?}", r.counts);
    }
}

/// The tour's certified chain cell.
pub fn narration(tol: Tol) {
    println!(
        "   the study: {LINKS} links, each joint an independent normal at σ = {JOINT_SIGMA} rad, \
         tip asserted within {:.2} mm of the target — the SAME document the density sheet \
         replays, at the analyzed box instead of at 512 draws",
        POSITION_BOUND * MM
    );
    println!(
        "   ONE leaf over the WHOLE box per row (no splitting), in the driver's own lane \
         (ProfileLift::Guided, the driver's symbolic budget and rules):"
    );

    let mut rows = Vec::new();
    for links in 1..=LINKS {
        let built: Chain = chain(links, JOINT_SIGMA, POSITION_BOUND, tol);
        for row in [
            interval_leaf(links, &built.doc),
            sym_leaf(links, &built.doc),
        ] {
            print_row(&row);
            rows.push((links, row));
        }
    }

    // Where a leaf certifies, the drive is affordable and its verdict
    // is the thing a CI row would gate on.
    for (links, row) in rows
        .iter()
        .filter(|(_, r)| r.certifies && r.lane != "Interval")
    {
        let built = chain(*links, JOINT_SIGMA, POSITION_BOUND, tol);
        let analyzed = analyzed_box(&built.doc, &AnalysisPolicy::default());
        let config = DriveConfig {
            max_leaves: 64,
            ..DriveConfig::default()
        };
        let t0 = Instant::now();
        match drive(&built.doc, &analyzed, &config, tol) {
            Ok(verdict) => {
                let mut holds = 0usize;
                let mut violated = 0usize;
                let mut undecided = 0usize;
                for leaf in verdict.certified() {
                    // On the lane the drive certified the leaf on — the
                    // verdict carries it, so a consumer never has to
                    // know which.
                    match assertion_at(
                        &built.doc,
                        built.assertion,
                        &leaf.box_,
                        verdict.symbolic(),
                        tol,
                    )
                    .and_then(|v| v.holds())
                    {
                        Some(true) => holds += 1,
                        Some(false) => violated += 1,
                        None => undecided += 1,
                    }
                }
                println!(
                    "   the drive at {links} link{}, leaf budget {} ({:.1} s): {} certified \
                     leaves — the tip assertion HOLDS on {holds}, is VIOLATED on {violated}, \
                     and is undecided on {undecided}",
                    if *links == 1 { "" } else { "s" },
                    config.max_leaves,
                    t0.elapsed().as_secs_f64(),
                    verdict.certified().len(),
                );
            }
            Err(refusal) => println!("   the drive at {links} links refused: {refusal}"),
        }
        let _ = row;
    }
}

/// Silence the unused import in a build where nothing below reads it.
#[allow(dead_code)]
fn _unused(_: RecipeNodeId) {}
