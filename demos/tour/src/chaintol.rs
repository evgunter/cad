//! **The four-link chain on the CERTIFIED lane** — what an enclosure
//! can say about the picture [`crate::mcchain`] draws, measured rather
//! than assumed.
//!
//! Same document as the density sheet (both take it from
//! [`crate::chain`]), same study. That cell draws 512 built chains and
//! summarizes them; this one asks the other question — what holds over
//! a BOX of joint angles, with no sampling at all — and reports what
//! it finds.
//!
//! The certified scalar is its entire subject.
//!
//! # The table, measured (σ = 0.01 rad at every joint, release)
//!
//! One leaf over the whole declared box, `±3σ` at every joint, no
//! splitting, in the driver's own lane. The costs are one box's wall
//! clock and move with the load, and the cell prints its own on every
//! run.
//!
//! | links | `Interval` | `Sym<Interval>` | cost |
//! |---|---|---|---|
//! | 1–4 | **CERTIFIES** | **CERTIFIES** | ~0.1 s / ~2 s |
//!
//! Both lanes certify the whole study at every link count. Each
//! joint's rotation is a rigid motion by construction (`RigidMotion`),
//! and a moved copy carries its certificates
//! (`topo::FinishedBody::moved`), so no joint's transform re-certifies
//! an edge whose images it enclosed apart. Those re-certifications were
//! this table's walls before ONEHOME stage 2 (PR 6820): plain
//! `Interval` refused at the first transform and `Sym<Interval>` at two
//! links (`work/shell/transform-rigid-recertifies-images-enclosed-apart`).
//!
//! # The drive over the whole study
//!
//! Since the whole study certifies, the drive's subject is the tip
//! assertion, the pin within 1 mm of the target: it splits the box
//! where the assertion's verdict changes, and reads the assertion off
//! each certified leaf. At the default ε, leaf budget 64:
//!
//! | links | certified leaves | HOLDS | VIOLATED | undecided |
//! |---|---|---|---|---|
//! | 1 | 1 (the whole box) | 1 | 0 | 0 |
//! | 2 | 36 | 24 | 12 | 0 |
//! | 3 | 21 | 18 | 3 | 0 |
//! | 4 | 2 | 0 | 2 | 0 |
//!
//! The split moves with ε — at `1e-6`, measured, two links are 35
//! leaves (31 hold, 4 violated), three are 18 (15, 3), four alike — so
//! [`DRIVE_AT_THE_STUDY`] is pinned at the default ε only, and another ε
//! row prints its own.
//!
//! **The VIOLATED leaves are the study's answer, not a defect.** Over
//! `±3σ` per joint the tip's certified enclosure across the chain grows
//! as the worst-case lever sum, `L · 3σ · n(n+1)/2`, which is past the
//! 1 mm band from two links on ([`crate::chain::CERTIFIED_PIN_BOX`]):
//! the certified lane proves WHERE in the box the requirement fails,
//! leaf by leaf, and that it holds on the rest. The leaves the drive
//! does not certify are the budget's, not a refusal of the geometry
//! (the cell prints them).
//!
//! [`crate::chain::CERTIFIED_PIN_BOX`] is each pin's certified
//! enclosure over that whole box, and [`crate::mcchain`] draws it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;
use std::time::Instant;

use pncad::analysis::{AnalysisPolicy, DriveConfig, ParamBox, analyzed_box, assertion_at, drive};
use pncad::document::{CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, evaluate};
use pncad::geom_core::{Bounds, Interval, Sym, SymBudget, SymCounts, SymRules, Tol};

use crate::chain::{CERTIFIED_PIN_BOX, Chain, LINKS, POSITION_BOUND, study};

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
/// (`work/pydoor/the-drivers-symbolic-dials-have-no-name-on-the-facade`.)
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
/// driver's own lane.
fn whole_box(doc: &ProfileDoc) -> EvalOptions {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    }
}

/// Every node that failed or was poisoned, in evaluation order —
/// the first entry is the wall, and its text names the predicate.
fn failures<T: pncad::geom_core::Decide + pncad::topo::AtRestPolicy>(
    ev: &Evaluation<T>,
) -> Vec<String> {
    ev.order
        .iter()
        .filter_map(|id| match ev.result(*id) {
            // The payload's `Debug` carries the predicate key; the
            // user-facing sentence no longer names it.
            Some(NodeResult::Failed(e)) => Some(format!("node {} — {} [{:?}]", id, e.kind, e.kind)),
            Some(NodeResult::Poisoned { through }) => {
                Some(format!("node {} poisoned through node {}", id, through))
            }
            _ => None,
        })
        .collect()
}

/// Which scalar lane a row ran on. An enum rather than the label
/// itself, because the drive below asks "was this the symbolic one?"
/// and a string comparison would be a spelling deciding a lane.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Lane {
    /// The plain enclosure.
    Interval,
    /// The enclosure carrying parameter expressions (E12).
    Symbolic,
}

impl Lane {
    fn label(self) -> &'static str {
        match self {
            Self::Interval => "Interval",
            Self::Symbolic => "Sym<Interval>",
        }
    }
}

/// One row of the table: a link count on a scalar lane.
pub struct Row {
    links: usize,
    lane: Lane,
    /// Nothing failed and nothing was poisoned over the whole box.
    pub certifies: bool,
    /// How many nodes failed or were poisoned.
    pub refused: usize,
    /// The first refusal: its sentence, then its payload, which names
    /// the predicate.
    pub first: Option<String>,
    /// Wall-clock of the one leaf.
    cost: std::time::Duration,
    /// The session's decision counts; all zero on the plain `Interval`
    /// lane, which runs no session at all.
    counts: SymCounts,
}

/// One leaf over the whole box at plain `Interval`.
pub fn interval_leaf(links: usize, doc: &ProfileDoc) -> Row {
    let opts = whole_box(doc);
    let t0 = Instant::now();
    let ev: Evaluation<Interval> = evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness());
    let cost = t0.elapsed();
    let bad = failures(&ev);
    Row {
        links,
        lane: Lane::Interval,
        certifies: bad.is_empty(),
        refused: bad.len(),
        first: bad.first().cloned(),
        cost,
        counts: SymCounts::default(),
    }
}

/// One leaf over the whole box at `Sym<Interval>`, under the driver's
/// own budget and rules.
pub fn sym_leaf(links: usize, doc: &ProfileDoc) -> Row {
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
        lane: Lane::Symbolic,
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
        r.lane.label(),
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

/// **What the drive answers over the whole study, per link count**, at
/// the default ε: `(certified leaves, HOLDS, VIOLATED, undecided)`, leaf
/// budget 64. MEASURED by this cell and pinned here; the module header
/// carries the table and what it says.
pub const DRIVE_AT_THE_STUDY: [(usize, usize, usize, usize); LINKS] =
    [(1, 1, 0, 0), (36, 24, 12, 0), (21, 18, 3, 0), (2, 0, 2, 0)];

/// The tour's certified chain cell.
///
/// **It asserts what its table and header claim, not only prints
/// them** ([`assert_row`], the drive's verdicts, and at the default ε
/// the published pin enclosures), so `demo-tour certified`'s exit 0 in
/// `tests/eps_regression.rs` carries those findings.
pub fn narration(tol: Tol) {
    println!(
        "   the study: {LINKS} links, each joint an independent normal at σ = {} rad, \
         tip asserted within {:.2} mm of the target — the SAME document the density sheet \
         replays, at the analyzed box instead of at 512 draws",
        crate::chain::JOINT_SIGMA,
        POSITION_BOUND * MM
    );
    println!(
        "   ONE leaf over the WHOLE box per row (no splitting), in the driver's own lane \
         (the driver's symbolic budget and rules):"
    );
    for links in 1..=LINKS {
        let built: Chain = study(links, tol);
        for row in [
            interval_leaf(links, &built.doc),
            sym_leaf(links, &built.doc),
        ] {
            print_row(&row);
            assert_row(&row);
        }
    }

    // The whole study certifies, so the drive is over the study itself,
    // and its subject is the tip assertion.
    for links in 1..=LINKS {
        drive_and_report(links, tol);
    }

    println!(
        "   the certified enclosure of each joint pin's centre over the WHOLE study, about \
         the nominal (half-width along the chain × across it):"
    );
    let boxes = certified_pin_boxes(LINKS, tol);
    for (k, (dx, dy)) in boxes.iter().enumerate() {
        println!("     pin {}: {:.5} mm × {:.5} mm", k + 1, dx * MM, dy * MM);
    }
    // **The published per-pin enclosures are these, at the default ε.**
    // [`CERTIFIED_PIN_BOX`] is drawn to scale by the density sheet as
    // the certified half of the picture, so a number that drifted from
    // what the tier encloses would be a box on the sheet that no leaf
    // ever certified.
    if crate::tolerance::at_the_ci_row(tol) {
        assert_pin_boxes_are_published(&boxes);
    }
}

/// **What the table claims, asserted per row** — the cell panics when
/// the kernel stops doing what it narrates: both lanes certify the
/// whole study at every link count.
fn assert_row(row: &Row) {
    assert!(
        row.certifies,
        "the header says the {} lane certifies the whole study at {} link(s); it refused at: \
         {:?}",
        row.lane.label(),
        row.links,
        row.first
    );
    if row.lane == Lane::Symbolic {
        assert!(
            row.counts.symbolic > 0,
            "the tier is what carries it, so the leaf discharged identities: {:?}",
            row.counts
        );
    }
}

/// [`CERTIFIED_PIN_BOX`] against the measured enclosures, within 2%,
/// with the whole array in the message in the literal's own shape so a
/// re-baseline is a paste rather than five readings.
fn assert_pin_boxes_are_published(measured: &[(f64, f64)]) {
    assert_eq!(measured.len(), CERTIFIED_PIN_BOX.len());
    let literal: String = measured
        .iter()
        .map(|(x, y)| format!("    ({x:e}, {y:e}),\n"))
        .collect();
    let drifted = measured
        .iter()
        .zip(CERTIFIED_PIN_BOX)
        .any(|((mx, my), (px, py))| {
            (mx - px).abs() > 0.02 * px.max(1e-9) || (my - py).abs() > 0.02 * py.max(1e-9)
        });
    assert!(
        !drifted,
        "chain::CERTIFIED_PIN_BOX is {CERTIFIED_PIN_BOX:?}; the certified leaf \
         encloses\n[\n{literal}];\nre-baseline the constant and say in the PR what moved."
    );
}

/// The drive over a chain's whole study, with the tip assertion read
/// off each certified leaf — the verdict a CI row gates on, printed, and
/// at the default ε asserted against [`DRIVE_AT_THE_STUDY`].
fn drive_and_report(links: usize, tol: Tol) {
    let built = study(links, tol);
    let analyzed = analyzed_box(&built.doc, &AnalysisPolicy::default());
    let config = DriveConfig {
        max_leaves: 64,
        ..DriveConfig::default()
    };
    let t0 = Instant::now();
    let verdict = drive(
        &built.doc,
        &analyzed,
        &config,
        &pncad::document::Stop::default(),
        tol,
    )
    .unwrap_or_else(|refusal| {
        panic!("the drive at {links} link(s) over the whole study refused: {refusal}")
    });
    let (mut holds, mut violated, mut undecided) = (0usize, 0usize, 0usize);
    for leaf in verdict.certified() {
        // On the lane the drive certified the leaf on — the verdict
        // carries it, so a consumer never has to know which.
        match assertion_at(
            &built.doc,
            built.assertion,
            &leaf.box_,
            verdict.symbolic(),
            &pncad::document::Stop::default(),
            tol,
        )
        .expect("the replay runs to completion at the default member budget")
        .and_then(|v| v.holds())
        {
            Some(true) => holds += 1,
            Some(false) => violated += 1,
            None => undecided += 1,
        }
    }
    let leaves = verdict.certified().len();
    println!(
        "   the drive at {links} link{} over the whole study, leaf budget {} ({:.1} s): \
         {leaves} certified leaf/leaves{}, {} left to the budget — the tip assertion HOLDS on \
         {holds}, is VIOLATED on {violated}, and is undecided on {undecided}",
        if links == 1 { "" } else { "s" },
        config.max_leaves,
        t0.elapsed().as_secs_f64(),
        // The budget is not what is being exercised when the count is
        // one: nothing split, so the leaf IS the whole box.
        if leaves == 1 && verdict.refused().is_empty() {
            " (the WHOLE box, unsplit)"
        } else {
            ""
        },
        verdict.refused().len(),
    );
    if crate::tolerance::at_the_ci_row(tol) {
        assert_eq!(
            (leaves, holds, violated, undecided),
            DRIVE_AT_THE_STUDY[links - 1],
            "the header pins the drive at {links} link(s) over the whole study as \
             (certified, HOLDS, VIOLATED, undecided); re-measure and say in the PR what moved"
        );
    }
}

/// **The certified enclosure of every joint pin's centre, over the
/// whole study** — the picture Ev asked for in full, as numbers the
/// sheet can draw.
///
/// Returned as `(half-width along the chain, half-width across it)`
/// per pin, in metres: the pin's stored `Surface::Cylinder` origin is
/// a `Sym<Interval>` here, and its numeric channel IS the enclosure
/// the certified leaf establishes. Read out through `Bounds` because
/// this is a REPORTING surface — the bracket is being drawn, not
/// decided on.
pub fn certified_pin_boxes(links: usize, tol: Tol) -> Vec<(f64, f64)> {
    let built = study(links, tol);
    let opts = whole_box(&built.doc);
    let (budget, rules) = drive_dials();
    let (boxes, _) = pncad::geom_core::sym::with_session_rules(budget, rules, || {
        let ev: Evaluation<Sym<Interval>> =
            evaluate(&built.doc, None, &CancelToken::new(), &opts, Tol::witness());
        // **The premise, checked here and not assumed by the caller.**
        // An enclosure read off a leaf that refused is not an
        // enclosure; the pins below would panic on a node that never
        // evaluated, and the reason would be lost. Checked in this
        // function so every caller gets it.
        let bad = failures(&ev);
        assert!(
            bad.is_empty(),
            "certified_pin_boxes asks for the enclosure over {links} link(s) over the \
             whole study, and that leaf does NOT certify — there is no certified box to \
             read. First refusal: {}",
            bad.first().map_or("none", String::as_str)
        );
        built
            .pins
            .iter()
            .enumerate()
            .map(|(k, id)| {
                let payload = &ev.value(*id).expect("the pin evaluated").payload;
                let Some(Some(body)) = payload.one_body() else {
                    panic!("a placed pin evaluates to a body, got {payload:?}");
                };
                // Where the pin stands in the world: its frame composed
                // through the one door a picture composes by.
                let placed = pncad::topo::Body::clone(
                    &pncad::export::body_in_space(body, Tol::witness())
                        .expect("a pin composes into the world"),
                );
                let (x, y) = crate::chain::pin_axis(&placed);
                // The nominal is the enclosure's own centre only to
                // within the widening, so the half-width is reported
                // about the NOMINAL: that is what a reader compares the
                // advisory cloud against.
                let nominal = k as f64 * crate::chain::LINK_LENGTH;
                (
                    (x.hi() - nominal).abs().max((nominal - x.lo()).abs()),
                    y.hi().abs().max(y.lo().abs()),
                )
            })
            .collect::<Vec<_>>()
    });
    boxes
}
