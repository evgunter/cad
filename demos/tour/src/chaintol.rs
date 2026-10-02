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
//! The certified scalar is its entire subject.
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
//!
//! # The table, measured (σ = 0.01 rad at every joint, release)
//!
//! One leaf over the whole declared box, no splitting, in the driver's
//! own lane. The costs are one box's wall clock and move with the
//! load; what they are here for is the SHAPE — sub-second at four
//! links, against the 219 s per replay the derived-frame family cost
//! at two stacked frames (measured elsewhere, and quoted from
//! `a-widened-rotation-angle-is-unmeasured-on-the-certified-lane`'s
//! survey rather than re-taken here) — and the cell prints its own on
//! every run.
//!
//! | links | lane | | first refusal OVER THE WHOLE STUDY | cost |
//! |---|---|---|---|---|
//! | 1–4 | `Interval` | refuses | `transform_rigid_col0_unit` | <0.01 s |
//! | 1 | `Sym<Interval>` | **CERTIFIES** | — | 0.16 s |
//! | 2 | `Sym<Interval>` | refuses | `dihedral_wedge`: no tangent plane | 0.31 s |
//! | 3 | `Sym<Interval>` | refuses | `dihedral_arm`, `[0, 7.34e-3]` | 0.47 s |
//! | 4 | `Sym<Interval>` | refuses | `dihedral_arm`, `[0, 7.34e-3]` | 0.73 s |
//!
//! **That column is the first refusal at the WHOLE study, which is a
//! different question from what bounds the certifiable box.**
//! Evaluation order decides which refusal is reported first, and at
//! the whole study the `dihedral_arm` straddle happens to come first
//! at three and four links. Just above the WALL the first refusal is
//! the wedge on `EdgeKey(1v1)` at sample 4 at two, three and four links
//! (nodes 14, 21, 30): **the wedge is what bounds the box**, as the
//! next section says. The arm's straddle is a second, ε-independent
//! refusal that is first only over the whole study
//! (`work/sym/a-chain-of-three-joints-straddles-dihedral-arm`).
//!
//! **The plain interval lane does not carry a widened rotation angle
//! at all.** `Mat3::rotation_about` builds its columns out of
//! `cos(angle)` and `sin(angle)`; on an interval angle those are two
//! independent brackets, `cos² + sin²` is a bracket AROUND 1, and the
//! rigid map's own column-unit check is what notices. The symbolic
//! tier discharges exactly that identity, which is the whole
//! difference between the two lanes here.
//!
//! # The widest box that certifies whole, and what sets it
//!
//! Bisected: `1.000` of the study at one link, then `0.370`, `0.185`,
//! `0.111` ([`crate::chain::CERTIFIABLE_FRACTION_BY_LINKS`]). Those
//! are not four numbers — they are ONE. The tip's certified lateral
//! half-width, `L · 3σ · f · n(n+1)/2` at `n` links, is `3.998e-4` m
//! at two, three and four links alike.
//!
//! **What that one number is, is half the PIN RADIUS**, and it is a
//! property of this document's geometry rather than of the tier. The
//! ratio to [`crate::chain::PIN_RADIUS`] is `0.500 / 0.500 / 0.499` at
//! two, three and four links; MEASURED with the radius doubled to
//! `1.6e-3` m, the fractions move to `1.0000 / 0.73841 / 0.36921 /
//! 0.22192` and the half-width to `7.975e-4` m — still `0.498` of the
//! radius. So the earlier reading of this table, "the certified lane
//! carries about 1.9° of accumulated swing however many joints it is
//! spread over", was an artefact of the shipped radius: the swing
//! doubles with it, to `3.81°`. The invariant across LINK COUNTS is
//! real; the angle was not the invariant.
//!
//! **What sets the wall: this paragraph is the mechanism's one home.**
//! The wall's edge is the tip pin's cap circle, a plane × cylinder
//! `Intersection` the outer joint's transform re-certifies. Its point
//! and its cylinder's axis are mapped and enclosed APART, each carrying
//! the tip's lateral half-width `δ` across the chain, so the radial
//! vector `p − origin` the cylinder's gradient is built from is enclosed
//! `r ± 2δ` wide. Two things follow, from two different causes:
//!
//! - **The poison, at `2δ = r`.** The gradient's enclosure reaches zero,
//!   no tangent plane is defined over the box, and the wedge refuses
//!   with "the surfaces' tangent planes at sample 4 are undefined". The
//!   box where that happens does not move with ε. It is set by the
//!   transform enclosing the two images apart
//!   (`work/shell/transform-rigid-recertifies-images-enclosed-apart`),
//!   and at the default ε it IS the wall, so the tip's certified box is
//!   half the pin radius.
//! - **The straddle, before it.** `sin θ = ‖n1 × n2‖/(‖n1‖·‖n2‖)`
//!   carries the gradient's magnitude in both its numerator and its
//!   denominator, so its enclosure's lower end falls toward zero as
//!   `2δ` approaches `r`, although the true value over these boxes is
//!   one. The box stops certifying where that lower end drops under
//!   `K·ε`, which is why the fraction moves with ε. That width is the
//!   formula's
//!   (`work/props/interval-sin-theta-as-cross-over-norms-loses-the-shared-magnitude`).
//!
//! The enclosures, the model, and the fractions and first refusals at
//! three ε are on
//! `work/sym/a-chain-of-two-or-more-joints-poisons-its-transversality-margin`
//! ("What Phase 1 found"). `geom_brep`'s
//! `a_cylinder_gradient_reaching_zero_leaves_no_tangent_plane` is the
//! poison on one plane and one cylinder. The one-link row is outside
//! the pattern and says why: it is capped by the study itself (`f = 1`
//! at `0.450` of the radius), the wall lying beyond it.
//!
//! At that box the drive certifies the whole box in ONE leaf — it
//! splits nothing, because nothing refuses — and **the four-link tip's
//! assertion HOLDS on that leaf**, so the enclosure per joint is real
//! geometry rather than a caption:
//! [`crate::chain::CERTIFIED_PIN_BOX`] carries it and
//! [`crate::mcchain`] draws it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;
use std::time::Instant;

use pncad::analysis::{AnalysisPolicy, DriveConfig, ParamBox, analyzed_box, assertion_at, drive};
use pncad::document::{
    CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, ProfileLift, ValuePayload,
    evaluate,
};
use pncad::geom_core::{Bounds, Interval, Sym, SymBudget, SymCounts, SymRules, Tol};

use crate::chain::{
    CERTIFIABLE_FRACTION_BY_LINKS, CERTIFIED_PIN_BOX, Chain, JOINT_SIGMA, LINK_LENGTH, LINKS,
    POSITION_BOUND, chain,
};

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
/// (`work/lib/the-drivers-symbolic-dials-have-no-name-on-the-facade`.)
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
struct Row {
    links: usize,
    lane: Lane,
    /// Nothing failed and nothing was poisoned over the whole box.
    certifies: bool,
    /// How many nodes failed or were poisoned.
    refused: usize,
    /// The first refusal: its sentence, then its payload, which names
    /// the predicate.
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

/// The tour's certified chain cell.
///
/// **It asserts what its table and header claim, not only prints
/// them** ([`assert_row`], the drive's verdicts, and at the default ε
/// the published box and pin enclosures), so `demo-tour certified`'s
/// exit 0 in `tests/eps_regression.rs` carries those findings and no
/// unit row re-computes them. The rows below are the MEASUREMENTS the
/// narration only reads back — the bisection behind
/// [`CERTIFIABLE_FRACTION_BY_LINKS`], the wall just past it, and the
/// tip ratio at every link count.
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
            assert_row(&row);
            rows.push((links, row));
        }
    }

    // Where a leaf certifies, the drive is affordable and its verdict
    // is the thing a CI row would gate on.
    for (links, _) in rows
        .iter()
        .filter(|(_, r)| r.certifies && r.lane == Lane::Symbolic)
    {
        drive_and_report(*links, 1.0, tol);
    }

    println!(
        "   the widest box that CERTIFIES WHOLE, as a fraction of the study — MEASURED at \
         the default ε (`chain::CERTIFIABLE_FRACTION_BY_LINKS`, bisected and pinned by this \
         cell's own CI row), beside the TIP DISPLACEMENT each one allows:"
    );
    for (i, f) in CERTIFIABLE_FRACTION_BY_LINKS.iter().enumerate() {
        let links = i + 1;
        // `3σ` is the analyzed box's half-width per joint and
        // `n(n+1)/2` is the tip's lever sum over the joints above it,
        // so `L · 3σ · f · n(n+1)/2` is how far the tip may move over
        // the certified box. THAT is the quantity that is the same at
        // every link count — half the pin radius — and the ANGLE below
        // is it divided by the fixed link length, which is why the
        // angle looked like the invariant and is not one.
        let lever: f64 = (links * (links + 1) / 2) as f64;
        let swing = 3.0 * JOINT_SIGMA * f * lever;
        println!(
            "     {links} link{}: {f:.3e} of the study — tip within {:.4e} m ({:.4} of the \
             pin radius), a swing of {swing:.4} rad ({:.2}°)",
            if links == 1 { " " } else { "s" },
            LINK_LENGTH * swing,
            LINK_LENGTH * swing / crate::chain::PIN_RADIUS,
            swing.to_degrees()
        );
    }

    // **Whether that box still certifies HERE.** The fraction moves
    // with ε — MEASURED, `1.083e-1` at ε = 1e-6 against `1.110e-1` at
    // the default — for the reason the module header's "What sets the
    // wall" gives.
    // So the cell asks rather than reasoning: it declares the
    // frontier instead of assuming its own published number
    // (`demos/tour/tests/eps_regression.rs` on a declared frontier),
    // and one leaf is what it costs to know.
    let narrow = chain(
        LINKS,
        JOINT_SIGMA * crate::chain::CERTIFIABLE_FRACTION,
        POSITION_BOUND,
        tol,
    );
    if !sym_leaf(LINKS, &narrow.doc).certifies {
        // At the default ε the published box IS this run's box, so a
        // refusal there is the finding moving, not the frontier.
        assert!(
            !crate::tolerance::at_the_ci_row(tol),
            "the published {LINKS}-link box ({} of the study) is the default ε's \
             measurement, and at the default ε it no longer certifies",
            crate::chain::CERTIFIABLE_FRACTION
        );
        println!(
            "   the published box does NOT certify at this run's ε — it is the default ε's \
             number, and the box moves with ε (1.083e-1 at 1e-6, measured). No enclosure is \
             reported here; the cell's CI row measures the fraction at the default ε."
        );
        return;
    }

    // …and at that box the certified lane reaches the TIP, so the
    // picture has a certified half: an enclosure per joint, drawn
    // beside the advisory cloud.
    println!(
        "   the certified enclosure of each joint pin's centre over that box, about the \
         nominal (half-width along the chain × across it):"
    );
    let boxes = certified_pin_boxes(LINKS, crate::chain::CERTIFIABLE_FRACTION, tol);
    for (k, (dx, dy)) in boxes.iter().enumerate() {
        println!("     pin {}: {:.5} mm × {:.5} mm", k + 1, dx * MM, dy * MM);
    }
    // **The published per-pin enclosures are these, at the default ε.**
    // [`CERTIFIED_PIN_BOX`] is drawn to scale by the density sheet as
    // the certified half of the picture, so a number that drifted from
    // what the tier encloses would be a box on the sheet that no leaf
    // ever certified. Pinned at the default ε only: the box itself
    // moves with ε.
    if crate::tolerance::at_the_ci_row(tol) {
        assert_pin_boxes_are_published(&boxes);
    }
    // …and WHAT that tip half-width is: half the pin radius, which is
    // the invariant the four fractions are four spellings of. Printed
    // beside the ratio's published value so the sentence and the
    // number are read together rather than the sentence alone.
    let tip = boxes.last().expect("a chain has a tip pin").1;
    println!(
        "   the tip's is {:.4} of the pin radius ({:.2} mm) — the invariant across link \
         counts, and a property of THIS document's geometry rather than of the tier: \
         `chain::CERTIFIED_TIP_OVER_PIN_RADIUS` = {:.4}, and doubling the radius doubles \
         the certified swing",
        tip / crate::chain::PIN_RADIUS,
        crate::chain::PIN_RADIUS * MM,
        crate::chain::CERTIFIED_TIP_OVER_PIN_RADIUS
    );

    // The row this unit answers asks one question: does the certified
    // lane reach the FOUR-link tip's assertion at any box. It does, at
    // that one — so the drive is run there and the verdict printed.
    drive_and_report(LINKS, crate::chain::CERTIFIABLE_FRACTION, tol);
}

/// **What the table claims, asserted per row** — the cell panics when
/// the kernel stops doing what it narrates.
///
/// The plain interval lane refuses at EVERY link count, at the FIRST
/// transform, on the rigid map's own isometry check: an interval
/// `cos`/`sin` makes `cos² + sin²` a bracket around 1 rather than 1,
/// and the column-unit predicate is what notices. The symbolic tier
/// discharges that identity, so the ONE-link chain certifies whole over
/// the study a user actually has — and the wall MOVES rather than going
/// away: from two links on it is a transversality margin during the
/// mapped edge's re-certification, not the isometry. The table names
/// WHICH predicate per link count, so that is what is asserted, not
/// merely that some dihedral refused: the two are different predicates
/// with different answers, and the header once said the wrong one.
fn assert_row(row: &Row) {
    let links = row.links;
    let first = || {
        row.first
            .as_deref()
            .expect("a refusing row names its first refusal")
    };
    match (row.lane, links) {
        (Lane::Interval, _) => {
            assert!(
                !row.certifies,
                "the header says a widened rotation angle does not survive the plain \
                 interval lane at any link count; {links} link(s) certified"
            );
            assert!(
                first().contains("transform_rigid_col0_unit"),
                "the header names `transform_rigid_col0_unit` as the plain lane's wall; at \
                 {links} link(s) the first refusal was: {}",
                first()
            );
        }
        (Lane::Symbolic, 1) => {
            assert!(
                row.certifies,
                "the header says the symbolic tier carries the one-link chain over the \
                 whole study; it refused at: {:?}",
                row.first
            );
            assert!(
                row.counts.symbolic_zero > 0,
                "the tier is what carries it, so the leaf discharged identities: {:?}",
                row.counts
            );
        }
        (Lane::Symbolic, _) => {
            let predicate = if links == 2 {
                "dihedral_wedge"
            } else {
                "dihedral_arm"
            };
            assert!(
                !row.certifies,
                "the header says the symbolic tier stops at two links over the whole \
                 study; {links} link(s) certified"
            );
            assert!(
                !first().contains("transform_rigid_col0_unit"),
                "the header says the isometry wall is GONE on the symbolic lane; at \
                 {links} link(s) it was still the first refusal: {}",
                first()
            );
            assert!(
                first().contains(predicate),
                "the table says the first refusal at {links} link(s) over the whole study \
                 is `{predicate}`; it was: {}",
                first()
            );
        }
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

/// The drive over a chain's analyzed box at a stated fraction of the
/// study, with the tip assertion read off each certified leaf — the
/// verdict a CI row would gate on, printed.
fn drive_and_report(links: usize, fraction: f64, tol: Tol) {
    let built = chain(links, JOINT_SIGMA * fraction, POSITION_BOUND, tol);
    let analyzed = analyzed_box(&built.doc, &AnalysisPolicy::default());
    let config = DriveConfig {
        max_leaves: 64,
        ..DriveConfig::default()
    };
    let t0 = Instant::now();
    match drive(&built.doc, &analyzed, &config, tol) {
        Ok(verdict) => {
            let (mut holds, mut violated, mut undecided) = (0usize, 0usize, 0usize);
            for leaf in verdict.certified() {
                // On the lane the drive certified the leaf on — the
                // verdict carries it, so a consumer never has to know
                // which.
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
            let leaves = verdict.certified().len();
            // The tip assertion is what a CI row would gate on, and
            // the cell says it HOLDS, certified, wherever the drive
            // reaches it.
            assert!(
                leaves > 0 && holds == leaves,
                "the header says the drive at {links} link(s) over {fraction} of the study \
                 certifies and the tip assertion HOLDS on every certified leaf: {leaves} \
                 certified, {holds} hold, {violated} violated, {undecided} undecided"
            );
            println!(
                "   the drive at {links} link{} over {} of the study, leaf budget {} \
                 ({:.1} s): {leaves} certified leaf/leaves{} — the tip assertion HOLDS on \
                 {holds}, is VIOLATED on {violated}, and is undecided on {undecided}",
                if links == 1 { "" } else { "s" },
                fraction,
                config.max_leaves,
                t0.elapsed().as_secs_f64(),
                // The budget is not what is being exercised when the
                // count is one: nothing refused, so the driver never
                // split and the leaf IS the whole box. Worth saying,
                // because "the drive at leaf budget 64 certifies"
                // reads as if splitting had done some work.
                if leaves == 1 {
                    " (the WHOLE box, unsplit: nothing refused, so the driver never bisected)"
                } else {
                    ""
                },
            );
        }
        Err(refusal) => {
            panic!("the drive at {links} link(s) over {fraction} of the study refused: {refusal}")
        }
    }
}

/// **The widest box of a chain that certifies whole, as a fraction of
/// the real study** — the plate's `CERTIFIABLE_FRACTION`, measured the
/// same way: ONE leaf over the whole box, at the study's σ scaled by
/// `f`, with nothing failed and nothing poisoned.
///
/// Searched in the log: `f` is halved from 1 until a leaf certifies
/// (or the floor is reached), then the bracket is bisected in the
/// exponent. A ratio is the right variable here for the reason it is
/// on the plate — what the number says is how much of the study the
/// certified answer covers, and "how much" is scale-free.
///
/// `#[cfg(test)]` because it is the MEASUREMENT, not the narration:
/// the numbers it produces are published as
/// [`CERTIFIABLE_FRACTION_BY_LINKS`] and the cell prints those, so a
/// tour walk pays one leaf to say whether the published box still
/// certifies at its ε rather than paying a whole bisection per link
/// count on every run of `eps_regression`.
#[cfg(test)]
fn certifiable_fraction(links: usize, tol: Tol) -> f64 {
    let certifies = |f: f64| {
        let built = chain(links, JOINT_SIGMA * f, POSITION_BOUND, tol);
        sym_leaf(links, &built.doc).certifies
    };
    if certifies(1.0) {
        return 1.0;
    }
    // The floor: 2^-40 of a 0.01 rad study is 9e-15 rad, narrower than
    // any box a reader would call a study. A chain that has not
    // certified by there has not certified.
    const FLOOR: f64 = -40.0;
    let mut hi = 0.0f64; // log2 of a fraction that REFUSES
    let lo = loop {
        let next = hi - 1.0;
        if next < FLOOR {
            return 0.0;
        }
        if certifies(next.exp2()) {
            break next;
        }
        hi = next;
    };
    // Eight bisections of the EXPONENT — the answer to about a third of
    // a percent, which is more than the two digits it is reported to.
    //
    // **What the answer establishes, and what it assumes.** Both ends
    // of it are executed: the width returned CERTIFIES (the search's
    // last accepted probe, re-measured by
    // `the_published_certifiable_fractions_are_the_measured_ones`),
    // and 1.02× that width REFUSES
    // (`the_wall_is_the_wedge_not_the_arm`). What is ASSUMED is
    // monotonicity BELOW the answer — that no narrower box refuses.
    //
    // For the wall there is an argument, not a guarantee: on the same
    // sequence of operations interval arithmetic encloses a sub-box
    // inside the wider box's enclosure, so the wedge's lower end (the
    // module header's "What sets the wall") should not fall narrower.
    // Nothing enforces that sequence across boxes, and it says nothing
    // of the leaf's other refusals.
    //
    // So the assumption is carried as an assumption, and the guard is
    // the half and quarter rows: the fractions row re-certifies each
    // answer at 1/2 and 1/4 of its width, so a refusal returning
    // anywhere in that range reds. Outside it the
    // claim the constants actually carry is the executed one —
    // certifies at this width, refuses 2% above.
    let mut lo = lo;
    for _ in 0..8 {
        let mid = f64::midpoint(lo, hi);
        if certifies(mid.exp2()) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo.exp2()
}

/// **The certified enclosure of every joint pin's centre, over the
/// widest box that certifies whole** — the picture Ev asked for in
/// full, as numbers the sheet can draw.
///
/// Returned as `(half-width along the chain, half-width across it)`
/// per pin, in metres: the pin's stored `Surface::Cylinder` origin is
/// a `Sym<Interval>` here, and its numeric channel IS the enclosure
/// the certified leaf establishes. Read out through `Bounds` because
/// this is a REPORTING surface — the bracket is being drawn, not
/// decided on.
fn certified_pin_boxes(links: usize, fraction: f64, tol: Tol) -> Vec<(f64, f64)> {
    let built = chain(links, JOINT_SIGMA * fraction, POSITION_BOUND, tol);
    let opts = whole_box(&built.doc);
    let (budget, rules) = drive_dials();
    let (boxes, _) = pncad::geom_core::sym::with_session_rules(budget, rules, || {
        let ev: Evaluation<Sym<Interval>> =
            evaluate(&built.doc, None, &CancelToken::new(), &opts, Tol::witness());
        // **The premise, checked here and not assumed by the caller.**
        // An enclosure read off a leaf that refused is not an
        // enclosure; the pins below would panic on a node that never
        // evaluated, and the reason would be lost. Checked in this
        // function so every caller gets it — the narration, which
        // probes the ε frontier before it arrives, and the two rows,
        // which would otherwise be leaning on the fractions row
        // having run first.
        let bad = failures(&ev);
        assert!(
            bad.is_empty(),
            "certified_pin_boxes asks for the enclosure over {links} link(s) at {fraction} \
             of the study, and that leaf does NOT certify — there is no certified box to \
             read. First refusal: {}",
            bad.first().map_or("none", String::as_str)
        );
        built
            .pins
            .iter()
            .enumerate()
            .map(|(k, id)| {
                let payload = &ev.value(*id).expect("the pin evaluated").payload;
                let ValuePayload::Body(body) = payload else {
                    panic!("a placed pin evaluates to a body, got {payload:?}");
                };
                let (x, y) = crate::chain::pin_axis(&**body);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// **What BOUNDS the certifiable box is `dihedral_wedge`.**
    ///
    /// Not `dihedral_arm`, which is the first refusal over the WHOLE
    /// study at three and four links. The question the box answers is
    /// asked just above the wall: at `1.02×` and `1.10×` of each link
    /// count's published fraction — the DEFAULT ε's
    /// ([`CERTIFIABLE_FRACTION_BY_LINKS`]) — the first refusal is the
    /// wedge with no tangent plane, at two, three and four links alike,
    /// and it says so in words rather than as an invalid margin.
    ///
    /// Those boxes are past the one where the tip pin's gradient
    /// enclosure reaches zero, which does not move with ε (the module
    /// header's "What sets the wall"), so the row asks the same
    /// question at every ε it runs at. Just above a LARGER ε's own wall
    /// the same margin straddles `K·ε` instead; this row does not ask
    /// that.
    #[test]
    fn the_wall_is_the_wedge_not_the_arm() {
        let tol = Tol::witness();
        for (i, f) in CERTIFIABLE_FRACTION_BY_LINKS.iter().enumerate().skip(1) {
            let links = i + 1;
            for over in [1.02, 1.10] {
                let built = chain(links, JOINT_SIGMA * f * over, POSITION_BOUND, tol);
                let row = sym_leaf(links, &built.doc);
                assert!(
                    !row.certifies,
                    "{over}× the {links}-link wall must be past it; it certified"
                );
                let first = row.first.expect("a refusing row names its first refusal");
                assert!(
                    first.contains("dihedral_wedge"),
                    "the header says the WALL is `dihedral_wedge`; at {links} links, \
                     {over}× the fraction, the first refusal was: {first}"
                );
                assert!(
                    first.contains("the surfaces' tangent planes at sample 4 are undefined"),
                    "the header says past the default ε's wall the wedge has no tangent \
                     plane, and names that as its cause; at {links} links, {over}× the \
                     fraction: {first}"
                );
            }
        }
    }

    /// **The tip's certified box is half the PIN RADIUS**, at every
    /// link count whose box the wall sets.
    ///
    /// This is the statement the four fractions are four spellings of,
    /// and it is the one that was got wrong: the header read the
    /// invariance as a property of the tier (an angle) when it is a
    /// property of this document's geometry. Pinned because a reader
    /// who changes `PIN_RADIUS` should be told by a test, not by a
    /// caption that quietly stops being true.
    ///
    /// The one-link chain is out of scope by construction: its widest
    /// box is the STUDY, not the wall, and it sits at `0.450`.
    #[test]
    fn the_certified_tip_box_is_half_the_pin_radius() {
        let tol = Tol::witness();
        let mut ratios = Vec::new();
        for (i, f) in CERTIFIABLE_FRACTION_BY_LINKS.iter().enumerate().skip(1) {
            let links = i + 1;
            let boxes = certified_pin_boxes(links, *f, tol);
            let tip = boxes.last().expect("a chain has a tip pin").1;
            ratios.push(tip / crate::chain::PIN_RADIUS);
        }
        let published = crate::chain::CERTIFIED_TIP_OVER_PIN_RADIUS;
        let drifted = ratios
            .iter()
            .any(|r| (r - published).abs() > 0.02 * published);
        assert!(
            !drifted,
            "chain::CERTIFIED_TIP_OVER_PIN_RADIUS is {published:e}; the certified tip \
             half-width over the pin radius measures {ratios:?} at 2..{LINKS} links (ε = \
             {}). Re-baseline the constant and say in the PR what moved.",
            tol.eps()
        );
    }

    /// **The published fractions are the measured ones**, at every
    /// link count.
    ///
    /// The last of them is drawn to scale by the density sheet, so a
    /// number that drifted from what the tier actually reaches would
    /// be a caption about a box that does not exist; and the four
    /// together are what say the wall is ONE threshold, so a drift in
    /// any of them is a change in that claim.
    ///
    /// The whole table is at the AMBIENT ε — `ci.yml` runs this row at
    /// the default — because the fractions move with ε: `1.110e-1` at
    /// the default against `1.083e-1` at `1e-6`, measured, for the
    /// reason the module header's "What sets the wall" gives.
    ///
    /// It also runs the check the bisection's monotonicity assumption
    /// owes (see `certifiable_fraction`): each answer is re-certified
    /// at 1/2 and 1/4 of its width, so a refusal returning at a
    /// NARROWER box reds here. The wedge's own enclosure argument
    /// covers the wall; this covers the rest of the leaf.
    #[test]
    fn the_published_certifiable_fractions_are_the_measured_ones() {
        let tol = Tol::witness();
        let measured: Vec<f64> = (1..=LINKS).map(|n| certifiable_fraction(n, tol)).collect();
        // The check the bisection's monotonicity assumption owes: a
        // narrower box must still certify. A refusal other than the
        // wall's would be seen here if one came back narrower.
        for (i, f) in measured.iter().enumerate() {
            let links = i + 1;
            for half in [0.5, 0.25] {
                let built = chain(links, JOINT_SIGMA * f * half, POSITION_BOUND, tol);
                let row = sym_leaf(links, &built.doc);
                assert!(
                    row.certifies,
                    "the fraction is reported as the WIDEST box that certifies, which \
                     assumes narrower ones do; at {links} link(s), {half}× of {f:e} of the \
                     study REFUSES: {:?}",
                    row.first
                );
            }
        }
        let drifted = measured
            .iter()
            .zip(CERTIFIABLE_FRACTION_BY_LINKS)
            .any(|(m, p)| (m - p).abs() > 0.02 * p);
        assert!(
            !drifted,
            "chain::CERTIFIABLE_FRACTION_BY_LINKS is {CERTIFIABLE_FRACTION_BY_LINKS:?}; the \
             widest whole-certifying boxes measure {measured:?} at ε = {}. Re-baseline the \
             constant and say in the PR what moved.",
            tol.eps()
        );
        assert_eq!(
            crate::chain::CERTIFIABLE_FRACTION,
            CERTIFIABLE_FRACTION_BY_LINKS[LINKS - 1],
            "the sheet's fraction is the table's last row"
        );
    }
}
