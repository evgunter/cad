//! **The two-hole plate** (ERROR-DESIGN's worked example, M10-6 §6):
//! the tolerance study a user actually has, authored through the
//! public doors and run twice — once at the tolerances a machinist
//! would write down, once at the box where the kernel can certify.
//!
//! Narration-only, like [`crate::checks`]: the subject is a REPORT and
//! its rendering is the picture.
//!
//! # What the two stops are for
//!
//! **Stop 1 is what a user gets today, and E12 turned it from a
//! refusal into an ANSWER.** ±0.05 mm on the hole spacing and σ =
//! 0.01 mm on each radius — a real study — and the driver now certifies
//! it. At the cell's 512-leaf budget (release, default ε): 193 leaves
//! certified, 319 refused at the budget and none for any other reason,
//! 83.4% of the study's mass certified; the certified worst case on the
//! web is `[0.419, 0.845]` mm against the asserted floor of 0.500 mm,
//! the nominal 0.600 mm sits in a certified chamber, and the
//! requirement — read off the ASSERTION NODE over each certified leaf,
//! the way stop 2 reads it — is MIXED, with its masses: it HOLDS on
//! 0.8337 of the study's mass and is VIOLATED on 0.0002, certified —
//! the corner where the spacing is short and both holes are large —
//! with 0.1661 in leaves the budget left unresolved. That is the
//! study's answer, and it is a gating one: "with probability 2·10⁻⁴
//! the web is under the floor" is now a sentence the kernel says.
//! The hull `[0.4188, 0.8450]` mm pads the exact affine range over the
//! certified leaves, `[0.4400, 0.7600]` mm, by 0.021 mm below and
//! 0.085 mm above — the interval lane's dependency widening, pinned
//! at both ends by the cell's row. Every sensitivity is
//! chamber-certified (`∂web/∂spacing = 2`, `∂web/∂r = −1` each).
//!
//! The symbolic identity tier (`geom_core::sym`, ERROR-DESIGN E12)
//! discharges a margin whose expression is identically zero in the
//! parameters, at any box width. On this plate it now discharges ALL
//! of them. The arc family took three units and an amendment: M10-9's
//! door lets the swept arc carrier's builder REGISTER the two
//! identities it guarantees (the rim `‖q − c‖ = r` and the span
//! `carrier.eval(param_end) = q_to`), and both endpoint pinnings go
//! (16 of 16 each at the nominal); M10-10's form-level algebra writes
//! the arc's trig in closed form — `sin`/`cos` of `q · atan(bulge)` as
//! rational functions of the bulge and `sqrt(1 + bulge²)`, a theorem
//! of the reals with no value read — and rules A/B per node close the
//! ring, so the carrier against its scaffold pushforward
//! (`carrier_matches_mapped_source`, 72 decisions), the cylinder
//! residual at the carrier's samples (`carrier_on_surface_2`, 72) and
//! at the strut witness (`witness_on_surface_2`, 8) go; and M10-10's
//! amendment A1 folds the chart's own phase — `atan2(0, r²/sqrt(r²))`
//! from the cylinder chart derivation is `atan2` of the zero form over
//! a form non-negative BY SYNTAX, so it is the zero form, and on the
//! negative frame `cos π = −1` — which takes the last one
//! (`pcurve_map_residual`, 36). MEASURED: the widest box of this plate
//! that certifies whole is 0.2368, 0.2631 and 0.2631 of THIS study at
//! ε = 1e-6, 1e-9 and 1e-12 — a fraction of the study, not a multiple
//! of ε any more (it was `1.25e3 · ε` before A1, and `7.81e2 · ε` under
//! M10-7's, M10-8's and M10-9's tiers alike). What bounds that CEILING
//! is `assert_bound`'s ENCLOSURE straddling zero — dependency widening
//! of a real margin, not a flip (`work/m10/real-margin-dependency-widening`):
//! the margin `web − floor = 1e-4 + 2·Δhs − Δr_a − Δr_b` is affine, its
//! true range at the ceiling is `[5.8e-5, 1.4e-4]` m, positive
//! everywhere, and the real flip first enters the box at 0.625 of the
//! study. Beyond the ceiling the driver splits and the LEAVES certify
//! up to that flip: every refusal at the budget sits along the surface
//! where the web crosses the floor, and refining a refused leaf leaves
//! `assert_bound` alone over the band. The cell prints the VIOLATED
//! mass beside `Mixed` (the sentence E12 quotes as the one to become
//! sayable), and the hull's padding beside the true range over the
//! certified leaves, so "straddles the floor" is read against a
//! number rather than a hull that pads its way across.
//!
//! **And the family is wider than this plate's rim.** M10-7's two
//! reviews took the tier to a filleted L-bracket with bores — the
//! ordinary shape of a machined part, where an arc stands between the
//! profile and its walls — and it certified whole only below `3.7e1 ·
//! ε` of its study; M10-8's constant fold lifted it to `3.9e2 · ε`
//! (10.4×) and a real ±0.1 study on M10-4's stepped shaft certifies
//! whole. Under M10-10 R1's eccentric annulus moves with the plate —
//! 0.70–0.84 of ITS real study, bounded by the enclosures of its own
//! dihedral and arc-diameter margins — while the bracket and R2's link
//! do not move: the term/coefficient BUDGET freezes their carrier
//! frames' squared components at any affordable width (the per-node
//! cap is a cost wall, not a reach: raising it leaves the link
//! byte-identical), so the scaffold residual stands there and their
//! answers are still ε-scale, waiting on that residual's retirement
//! (PCURVE/D3). And the mechanism's reach is the UNIT bulge — this
//! plate's circles: a parameter bulge is entirely outside it and a
//! literal bulge other than 1 leaves residue
//! (`work/m10/rule-d-reaches-the-unit-bulge-only`). So the honest
//! general statement is: **a real study on circle-authored geometry
//! certifies up to its real flips, and a study whose arcs carry a
//! frame the budget freezes, or a bulge that is not 1, still gets an
//! ε-scale answer.** `work/m10/symbolic-tier-census` carries what
//! bounds each document, with numbers.
//!
//! Where a leaf refuses it is not silence: the receipt says how many
//! and why (here, all at the leaf budget), and the coverage says where
//! the mass went. Beside it the Monte-Carlo lane answers the same
//! question advisorily, labeled, with its count and seed.
//!
//! **Stop 2 is the MVP's reason to exist, at the scale where it
//! works.** The same plate with every tolerance scaled to the box the
//! driver can certify: the certified worst case and the RSS's 3σ
//! figure printed side by side, disagreeing, with the tail on every
//! line. The DISAGREEMENT is scale-free — it is a ratio between a
//! linear sum and a root-sum-square — so shrinking the study does not
//! shrink the point.
//!
//! **What stop 2 reports, and what it does not** (M10-6's review, and
//! the sharpest thing in this file). The verdict it prints comes from
//! the ASSERTION NODE, over each certified leaf, through
//! `analysis::assertion_at`. It used to come from `worst_case.lo <
//! bound`, an `f64` comparison this cell made for itself — and the two
//! disagreed: the enclosure reaches under the bound by ~4e-11 while
//! the run's coincidence threshold is 1e-9, so the kernel classifies
//! that margin as coincident and the requirement HOLDS. The caption
//! said "FAILS somewhere in the box: this is the number that gates"
//! while the row that actually gates said the opposite.
//!
//! The divergence is still real and still the subject: the certified
//! worst case reaches further under the bound than the RSS's 3σ figure
//! does. What is now said out loud is its SIZE. Measured on this
//! fixture, the whole window between the two answers is ~6e-10 wide
//! against an escalation threshold of 1e-8 — so every bound that
//! separates them is one the funnel calls coincident, and no such
//! bound can be gated at this ε. That is not fixable by choosing a
//! wider box: the driver certifies up to a spread of about ε (3968
//! leaves) and refuses everything at 4ε, and the certified half-width
//! grows only ~1.5× the spread, so the margin never reaches the band.
//! Stop 2 therefore prints a VERDICT plus the window's size, and adds
//! one bound far enough out to be decided so a reader sees the gate
//! gate.
//!
//! # What was awkward to write, stated rather than smoothed over
//!
//! Per `memories/demo-purpose.md`, the awkwardness is the finding:
//!
//! 1. **The analysis lane had to be assembled by hand.** A consumer
//!    writes `analyzed_box` → `drive` → `stackup` → `monte_carlo`
//!    themselves, holding a box, a verdict and a policy in the right
//!    order across four calls. There is no "analyse this document"
//!    door, and every one of these four takes the run's `Tol` again.
//! 2. **The measure's references are POSITIONAL node ids.** A
//!    `SitedRef` names the node it reads at, so authoring one means
//!    keeping the extrude's id in a local — and the primitive then
//!    indexes the reference LIST by number (`Distance { a: 0, b: 1 }`),
//!    so a reader checks the vector's order to know what is being
//!    measured.
//! 3. **The goldening form is hex bits.** `serialize()` is exact and
//!    unreadable by design; the reason a `render()` exists beside it
//!    is that before M10-6 the only rendering of a `Stackup` was
//!    `Debug`, which prints masses as `Ok(0.9973002039367398)` and a
//!    verdict as a struct dump.
//! 4. **The assertion's bound is placed by the demo, not by a
//!    designer.** Stop 2 puts it between the RSS's 3σ figure and the
//!    certified worst case on purpose — that interval is exactly where
//!    the two disagree, and the disagreement is the subject. Said out
//!    loud because a bound chosen to make a point is not a bound a
//!    part needs. And, since the review: that interval is entirely
//!    inside the coincidence band, so it is also not a bound a run can
//!    decide.
//! 5. **Reading a requirement back needed a door that did not exist.**
//!    E10 says the assertion's verdict per certified leaf is what a CI
//!    row gates on, and a consumer holding a `ParamBoxVerdict` had no
//!    way to ask for it — the shortest path was to rebuild
//!    `EvalOptions`, pick the interval scalar and match on
//!    `ValuePayload`, and the shorter WRONG path was to compare two
//!    floats. `analysis::assertion_at` exists because this cell took
//!    the wrong one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::analysis::{
    AnalysisPolicy, AnalyzedBox, BoxAxis, DriveConfig, MassBudget, McConfig, ParamBoxVerdict,
    Stackup, StackupRefusal, analyzed_box, assertion_at, box_mass, drive, leaf_histogram,
    monte_carlo, render_sensitivity, stackup,
};
use pncad::document::{ProfileDoc, RecipeNodeId};
use pncad::geom_core::Tol;

use crate::plate::{Plate, WEB, plate};

/// **Stop 1's leaf budget, and why it is not the default.**
///
/// At ±0.05 mm every leaf REPLAYS now (0.2–0.4 s each, release), and
/// the driver splits toward the surface where the web crosses the
/// asserted floor, so the default 65,536 leaves would spend hours
/// refining a boundary the answer does not depend on. 512 leaves
/// certify 83% of the study's mass in about three and a half minutes
/// and already reach the violating corner (256 leaves certify 79% and
/// do not; 1024 certify 89%, measured in
/// `editor-core/tests/m10_10_evidence_interval`). The cell caps it and
/// says so, which is a statement about the COST rather than a thumb
/// on the answer: a reader who doubts it can raise the number and
/// watch the certified mass grow and the verdict not change.
fn starved() -> DriveConfig {
    DriveConfig {
        max_leaves: 512,
        ..DriveConfig::default()
    }
}

/// The tour's tolerance cell.
pub fn narration(tol: Tol) {
    real_study(tol);
    certified_study(tol);
}

/// **Stop 1 — the study a user actually has.** ±0.05 mm on the
/// spacing, σ = 0.01 mm on each radius.
fn real_study(tol: Tol) {
    let bound = WEB - 1.0e-4;
    let Plate {
        doc,
        measure,
        assertion,
        ..
    } = plate(5.0e-5, 1.0e-5, bound, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    println!(
        "   the real study: web nominal {:.4} mm, asserted >= {:.4} mm, over ±0.05 mm of \
         spacing and σ = 0.01 mm on each radius",
        WEB * 1e3,
        bound * 1e3
    );

    let verdict = drive(&doc, &analyzed, &starved(), tol).expect("the nominal builds");
    println!("{}", indent(&verdict.render(&analyzed)));
    // The verdict on the requirement, read off the ASSERTION NODE over
    // each certified leaf — stop 2's discipline, applied to the study
    // a user actually has.
    let decided = assertion_over_leaves(&doc, &verdict, assertion, tol);
    let masses = requirement_masses(&doc, &analyzed, &verdict, assertion, tol);
    match stackup(&doc, measure, &analyzed, &verdict, None, true, tol) {
        Ok(report) => {
            println!("{}", indent(&report.render(&analyzed)));
            println!(
                "   the assertion node {} is the recorded requirement, and THIS is the \
                 study's answer: {} — over the certified leaves the floor HOLDS on \
                 {:.4} of the study's mass, is VIOLATED on {:.4} (the web is under \
                 {:.4} mm there, certified), and is undecided on {:.4}; {:.4} of the mass \
                 is in leaves the budget left unresolved",
                assertion.0,
                describe(&decided),
                masses.holds,
                masses.violated,
                bound * 1e3,
                masses.unevaluated,
                1.0 - masses.holds - masses.violated - masses.unevaluated
            );
            let slack = hull_slack(&verdict, (report.worst_case.lo, report.worst_case.hi));
            println!(
                "   the certified hull [{:.4e}, {:.4e}] m against the TRUE range over the \
                 certified leaves [{:.4e}, {:.4e}] m (the web is affine in the parameters, \
                 so that range is exact): padding {:.2e} m below and {:.2e} m above — the \
                 interval lane's dependency widening, proportional to the leaf's width \
                 (work/m10/certified-hull-padding-is-the-leaf-width-not-the-lane)",
                report.worst_case.lo,
                report.worst_case.hi,
                slack.true_lo,
                slack.true_hi,
                slack.below,
                slack.above
            );
            println!(
                "     WHY it certifies now: the symbolic identity tier (E12) discharges \
                 every one of this plate's certification identities, so a leaf's width \
                 is bounded by the numeric channel's enclosure of the study's own margins \
                 and nothing else. MEASURED: the widest whole-certifying box is 0.2368 / \
                 0.2631 / 0.2631 of THIS study at ε = 1e-6 / 1e-9 / 1e-12 — a fraction of \
                 the study, not a multiple of ε (1.25e3·ε before M10-10's amendment A1; \
                 7.81e2·ε under M10-7's, M10-8's and M10-9's tiers alike). What refuses \
                 just past it is assert_bound, the web assertion's enclosure straddling \
                 zero — and that straddle is DEPENDENCY WIDENING, not a flip: the margin \
                 web − floor = 1e-4 + 2·Δhs − Δr_a − Δr_b is affine, its true range at the \
                 ceiling is [5.8e-5, 1.4e-4] m > 0 everywhere while the enclosure is \
                 [−2.1e-9, 2.0e-4], and the real flip first enters the box at 0.625 of \
                 the study (work/m10/real-margin-dependency-widening). The LEAVES certify \
                 up to that flip: every refusal above is the leaf budget, sitting along \
                 the surface where the web crosses the floor, and refining a refused leaf \
                 leaves assert_bound alone over the band. What moved it: M10-9's door \
                 registers the rim ‖q − c‖ = r and the span carrier.eval(4·atan|b|) = \
                 q_to; M10-10's rule D writes sin/cos of q·atan(bulge) in closed form and \
                 rules A/B per node close the ring (carrier_matches_mapped_source 72, \
                 carrier_on_surface_2 72, witness_on_surface_2 8 decisions); A1 folds the \
                 chart's phase atan2(0, r²/sqrt(r²)) to the zero form and cos π to −1 \
                 (pcurve_map_residual 36). No value was read by any of them. The reach: \
                 the unit bulge (this plate's circles) — a parameter bulge is outside the \
                 mechanism and a literal bulge other than 1 leaves residue \
                 (work/m10/rule-d-reaches-the-unit-bulge-only); what still bounds R2's \
                 bracket and link is the term budget freezing their carrier frames' \
                 squared components (work/m10/symbolic-tier-census)."
            );
        }
        Err(StackupRefusal::NothingCertified {
            nominal,
            sensitivities,
            coverage,
            receipt,
        }) => {
            println!("   NOTHING CERTIFIED — and the refusal carries the study's answer anyway:");
            println!(
                "     nominal web {:.4} mm",
                nominal.expect("the web is a closed form, so it has an f64 nominal") * 1e3
            );
            for s in &sensitivities {
                // Through the library's own spelling, not `Debug`:
                // the E4 chamber mark is the load-bearing half of a
                // sensitivity reading and `Derivative { .. }` buries
                // it. `render_sensitivity` was made public for this.
                println!(
                    "     ∂web/∂{}: {}",
                    s.param.0,
                    render_sensitivity(&s.outcome)
                );
            }
            println!(
                "     the drive: {} certified, {} refused",
                receipt.certified, receipt.refused
            );
            println!("{}", indent(&MassBudget::of(&coverage, &analyzed).render()));
            println!(
                "     This is NOT the expected answer any more: under M10-10's tier this \
                 study certifies (the module header carries the numbers). A refusal \
                 here means the arc family's identity residuals are bounding the plate \
                 again — a regression in geom_core::sym, to be read off the over-band \
                 set at ceiling + δ (editor-core/tests/m10_8_harness), not off this line."
            );
        }
        Err(other) => panic!("unexpected stackup refusal: {other}"),
    }

    // The advisory lane, which CAN answer at this box — and says what
    // it is on every line.
    let mc = monte_carlo(&doc, &analyzed, &McConfig::default(), tol).expect("the nominal builds");
    println!("{}", indent(&mc.render()));
}

/// **Stop 2 — the same plate at the box the driver can certify**: the
/// certified worst case and the RSS's optimism, side by side.
fn certified_study(tol: Tol) {
    // The tolerances scaled to the box that certifies. The RATIO
    // between the worst case and the RSS figure is scale-free, so the
    // point survives the shrink even though the numbers stop being a
    // machinist's.
    let spread = tol.eps() / 64.0;
    // **The tolerances are NORMAL-dominated on purpose**, and that is
    // the modelling choice the divergence rests on. `∂web/∂p · Δp`
    // summed linearly is the worst case; `√Σ(∂·σ)²` is the RSS. For a
    // UNIFORM contributor those two nearly agree — 3σ of a uniform
    // exceeds its own half-width — so a study dominated by uniforms has
    // no divergence to show. Two independent NORMALS do: their box is
    // ±3σ each, so the linear sum is `3σ₁ + 3σ₂` where the RSS is
    // `3√(σ₁² + σ₂²)`, a factor of √2 apart. Two holes machined
    // independently is also the honest model.
    let spacing_half_width = 0.05 * spread;
    let radius_sigma = 0.2 * spread;
    // `∂web/∂half_spacing` is 2 (the measure reads the SPACING, which
    // is twice the parameter) and `∂web/∂rᵢ` is −1.
    let worst = 2.0 * spacing_half_width + 2.0 * (3.0 * radius_sigma);
    let rss3 = 3.0
        * ((2.0 * spacing_half_width / 3.0_f64.sqrt()).powi(2) + 2.0 * radius_sigma.powi(2)).sqrt();
    // The bound sits between the two answers on purpose (see the
    // module header's finding 4).
    let bound = WEB - 0.5 * (worst + rss3);
    println!(
        "   the same plate at the certifiable box: the tolerances scaled to ε/64 = {:e} m",
        spread
    );
    println!(
        "     the linear worst case swings {worst:e} m; the RSS's 3σ figure swings \
         {rss3:e} m — a factor of {:.2}. The bound is placed between them.",
        worst / rss3
    );
    let Plate {
        doc,
        measure,
        assertion,
        ..
    } = plate(spacing_half_width, radius_sigma, bound, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), tol).expect("the nominal builds");
    println!("{}", indent(&verdict.render(&analyzed)));
    // **The verdict the CI row gates on, read off the ASSERTION NODE**
    // — not off a comparison this cell makes for itself. That is the
    // whole correction R1 forced (see the module header's finding on
    // the caption): `worst_case.lo < bound` is an f64 `<` over two
    // numbers that differ by less than the run's own coincidence
    // threshold, and a demo that decides on it is claiming a certainty
    // the kernel refuses to claim one line away.
    let decided = assertion_over_leaves(&doc, &verdict, assertion, tol);
    match stackup(&doc, measure, &analyzed, &verdict, None, true, tol) {
        Ok(report) => {
            println!("{}", indent(&report.render(&analyzed)));
            print_divergence(&report, bound, worst, &decided, tol);
            // The E11.6 datum: where each certified leaf's mass lands.
            let histogram = leaf_histogram(&doc, &analyzed, &verdict, measure, tol);
            println!("{}", indent(&histogram.render()));
        }
        Err(refusal) => {
            println!(
                "   the certifiable box did not certify either: {refusal}\n     \
                 That is a finding about the arc-rim endpoint family the module header \
                 names, not about the plate — the cell prints it rather than choosing a \
                 box that flatters the kernel."
            );
        }
    }
    println!(
        "   the assertion node {} is the recorded requirement, and THIS is what the CI \
         row gates on: {}",
        assertion.0,
        describe(&decided)
    );
    let mc = monte_carlo(&doc, &analyzed, &McConfig::default(), tol).expect("the nominal builds");
    println!("{}", indent(&mc.render()));
    // The advisory lane's own number, READ OFF THE REPORT rather than
    // asserted in prose. The first pass printed "0%" as a literal, so
    // a study whose sampling did find the corner would have been
    // narrated wrongly by a sentence nobody re-ran.
    if let Some(fraction) = mc
        .assertions
        .iter()
        .find(|a| a.node == assertion)
        .and_then(|a| a.violation_fraction())
    {
        println!(
            "     NOTE: the advisory lane's violation fraction is {:.4}% over {} samples, \
             while the CERTIFIED worst case reaches {:e} below the bound. The two answer \
             different questions — the sampled one asks where 512 draws landed, the \
             certified one asks what the whole box admits — which is why E11 makes the \
             certified one the only gate.",
            100.0 * fraction,
            mc.samples,
            bound
                - stackup(&doc, measure, &analyzed, &verdict, None, true, tol)
                    .map(|r| r.worst_case.lo)
                    .unwrap_or(bound)
        );
    }
    // **And a bound the run CAN decide**, so the reader sees the gate
    // actually gate rather than only refuse. See `print_divergence`
    // for why the interesting bound is not one of these.
    definite_arm(&doc, &analyzed, measure, tol);
}

/// The assertion node's verdict over every certified leaf, collapsed to
/// the one thing a caption may say: three states, and `Mixed` when the
/// leaves disagree (which is itself a verdict a reader must see rather
/// than have averaged away).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Decided {
    /// No leaf certified, so no verdict was taken.
    Nothing,
    Holds,
    Violated,
    /// Every leaf `Unevaluated`.
    Unevaluated,
    Mixed,
}

fn describe(d: &Decided) -> &'static str {
    match d {
        Decided::Nothing => "nothing certified, so the requirement was never put to a leaf",
        Decided::Holds => "HOLDS over every certified leaf",
        Decided::Violated => "VIOLATED over a certified leaf — the requirement fails",
        Decided::Unevaluated => {
            "UNEVALUATED over every certified leaf: the margin is inside the run's \
             coincidence band, so the kernel refuses to call it either way"
        }
        Decided::Mixed => "MIXED across the certified leaves — read them individually",
    }
}

fn assertion_over_leaves(
    doc: &ProfileDoc,
    verdict: &ParamBoxVerdict,
    assertion: RecipeNodeId,
    tol: Tol,
) -> Decided {
    let mut seen: Option<Decided> = None;
    for leaf in verdict.certified() {
        // On the lane the drive certified the leaf on — the verdict
        // carries it, so a consumer never has to know which.
        let one = match assertion_at(doc, assertion, &leaf.box_, verdict.symbolic(), tol) {
            Some(v) => match v.holds() {
                Some(true) => Decided::Holds,
                Some(false) => Decided::Violated,
                None => Decided::Unevaluated,
            },
            None => Decided::Nothing,
        };
        seen = Some(match seen {
            None => one,
            Some(prev) if prev == one => prev,
            Some(_) => Decided::Mixed,
        });
    }
    seen.unwrap_or(Decided::Nothing)
}

/// **The requirement's answer WITH ITS MASSES**: over the certified
/// leaves, how much of the study's probability mass sits where the
/// assertion HOLDS, where it is VIOLATED, and where the kernel refuses
/// to call it (`Unevaluated`) — each leaf's mass under the study's own
/// distributions (`box_mass`, the same integral the driver's accounting
/// takes). `Mixed` alone says the leaves disagree; this says by how
/// much, which is the sentence E12 quotes as the one to become sayable
/// ("with probability p the web is under the floor").
struct RequirementMasses {
    holds: f64,
    violated: f64,
    unevaluated: f64,
}

fn requirement_masses(
    doc: &ProfileDoc,
    analyzed: &AnalyzedBox,
    verdict: &ParamBoxVerdict,
    assertion: RecipeNodeId,
    tol: Tol,
) -> RequirementMasses {
    let mut out = RequirementMasses {
        holds: 0.0,
        violated: 0.0,
        unevaluated: 0.0,
    };
    for leaf in verdict.certified() {
        let mass = leaf_mass(analyzed, &leaf.box_);
        match assertion_at(doc, assertion, &leaf.box_, verdict.symbolic(), tol)
            .and_then(|v| v.holds())
        {
            Some(true) => out.holds += mass,
            Some(false) => out.violated += mass,
            None => out.unevaluated += mass,
        }
    }
    out
}

/// One leaf's mass under the study's distributions: the product over
/// its varying axes.
fn leaf_mass(analyzed: &AnalyzedBox, box_: &pncad::analysis::ParamBox) -> f64 {
    let mut m = 1.0;
    for (name, axis) in box_.axes() {
        let Some(dist) = analyzed.get(name).and_then(|p| p.distribution.as_ref()) else {
            continue;
        };
        if let BoxAxis::Varying { lo, hi } = axis {
            m *= box_mass(name, dist, (*lo, *hi)).unwrap_or(f64::NAN);
        }
    }
    m
}

/// **The hull's slack against the TRUE range over the certified
/// leaves.** The web is AFFINE in the study's parameters — `web = WEB +
/// 2·Δhalf_spacing − Δr_a − Δr_b` — so each certified leaf's true range
/// is exact arithmetic on its box, and the union over the leaves is
/// what a hull with no dependency padding would report. The slack
/// below and above that union is the interval lane's padding, and a
/// caption that says the hull "straddles the floor" has to say it
/// beside this number: a padded hull straddles more easily, so the
/// straddle alone gets EASIER as the padding grows (R2's Q3).
#[derive(Debug)]
struct HullSlack {
    true_lo: f64,
    true_hi: f64,
    below: f64,
    above: f64,
}

fn hull_slack(verdict: &ParamBoxVerdict, hull: (f64, f64)) -> HullSlack {
    let (mut true_lo, mut true_hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for leaf in verdict.certified() {
        let span = |n: &str| match leaf.box_.axes().get(&pncad::document::ParamName::new(n)) {
            Some(BoxAxis::Varying { lo, hi }) => (*lo, *hi),
            _ => (0.0, 0.0),
        };
        let (hs_lo, hs_hi) = span("half_spacing");
        let (a_lo, a_hi) = span("hole_a_r");
        let (b_lo, b_hi) = span("hole_b_r");
        true_lo = true_lo.min(WEB + 2.0 * hs_lo - a_hi - b_hi);
        true_hi = true_hi.max(WEB + 2.0 * hs_hi - a_lo - b_lo);
    }
    HullSlack {
        true_lo,
        true_hi,
        below: true_lo - hull.0,
        above: hull.1 - true_hi,
    }
}

/// **A bound the run can decide, so the gate is seen gating.**
///
/// The interesting bound — the one between the certified worst case and
/// the RSS's 3σ figure — is undecidable at this ε, and
/// `print_divergence` says why. This one is not: it sits a full
/// escalation threshold above the enclosure, so the margin is definite
/// and the assertion reads a plain `Violated`. Printed so the stop does
/// not leave a reader thinking the requirement machinery only ever
/// refuses.
fn definite_arm(doc: &ProfileDoc, analyzed: &AnalyzedBox, measure: RecipeNodeId, tol: Tol) {
    let verdict = drive(doc, analyzed, &DriveConfig::default(), tol).expect("the nominal builds");
    let Ok(report) = stackup(doc, measure, analyzed, &verdict, None, true, tol) else {
        return;
    };
    // A decade above the escalation threshold: definitely outside the
    // band, with no arithmetic near a boundary.
    let far = report.worst_case.hi + 100.0 * tol.eps();
    println!(
        "     and a bound the run CAN decide: at ≥ {far:e} m (a decade past the escalation \
         threshold above the whole enclosure) the same assertion reads a definite \
         VIOLATED — the gate gates. It is not the interesting bound, and the line above \
         says why."
    );
}

/// The divergence, printed as the two numbers a reader is meant to
/// compare: the gating one first (the E5 ordering rule).
///
/// `linear_worst` is the LINEARIZED worst case the study's own
/// arithmetic predicts, so the caption can say how much wider the
/// certified enclosure is than the sum of contributions — which is a
/// finding in itself and not a defect (see the printed line).
fn print_divergence(report: &Stackup, bound: f64, linear_worst: f64, decided: &Decided, tol: Tol) {
    // The two numbers, and then the VERDICT — which comes from the
    // assertion node, not from comparing them here.
    println!(
        "     CERTIFIED worst case: [{:e}, {:e}] against the bound {bound:e}. \
         The recorded requirement over these leaves: {}",
        report.worst_case.lo,
        report.worst_case.hi,
        describe(decided)
    );
    // **Why the enclosure reaching under the bound is not a failure**,
    // said here because the arithmetic invites the opposite reading and
    // the first version of this cell took it. `worst_case.lo` is below
    // `bound`, and by an amount SMALLER THAN ε: the funnel classifies
    // that margin as coincident and the assertion holds. Both readings
    // are correct about different questions and the kernel's is the one
    // that gates.
    let margin = report.worst_case.lo - bound;
    println!(
        "     the enclosure reaches {:e} m under the bound — but |margin| = {:e} is inside \
         the run's coincidence threshold ε = {:e}, so that reach is not a decidable \
         failure. A raw `lo < bound` here would read FAILS off a difference the kernel \
         refuses to call a difference.",
        margin.abs(),
        margin.abs(),
        tol.eps()
    );
    // The certified enclosure is WIDER than the linearized worst case,
    // and a reader who has just been told "the linear sum is 2.0e-11"
    // will notice. It is not curvature here — the measure is affine in
    // every parameter — it is INTERVAL WIDENING: the leaf replay
    // evaluates the whole geometry over the box, and an interval
    // evaluation of an affine function through a chain of non-affine
    // intermediates (a norm, a projection) overestimates. The
    // conservative direction, and the price of a certificate.
    let certified_half = 0.5 * (report.worst_case.hi - report.worst_case.lo);
    println!(
        "     the certified half-width is {certified_half:e} m against the linearized \
         {:e} m — a factor of {:.2}, and NOT curvature (the measure is affine in every \
         parameter): it is interval widening through the geometry chain, the \
         conservative direction and the price of a certificate.",
        0.5 * linear_worst,
        certified_half / (0.5 * linear_worst)
    );
    match &report.rss {
        pncad::analysis::Rss::Advisory { sigma } => {
            let Ok(nominal) = report.nominal else {
                // A measure with no f64 nominal has no linearized
                // figure to disagree with; the certified column above
                // still printed and still gates.
                println!(
                    "     ADVISORY rss: no linearized figure — this measure has no f64 \
                     nominal to expand around"
                );
                return;
            };
            let three_sigma = nominal - 3.0 * sigma;
            println!(
                "     ADVISORY rss: σ ≈ {sigma:e}, so a 3σ reading says the web reaches \
                 {three_sigma:e} — {} the bound.",
                if three_sigma >= bound {
                    "ABOVE"
                } else {
                    "below"
                }
            );
            // **The divergence, and the honest size of it.** The
            // certified enclosure reaches further under the bound than
            // the 3σ figure does: the two answers disagree, which is
            // what this milestone is about. What the first version of
            // this cell did not say is that the whole disagreement is
            // SMALLER than the run's escalation threshold, so no bound
            // placed inside it can be decided either way.
            let gap = three_sigma - report.worst_case.lo;
            // The run's own escalation threshold, K·ε — the caption
            // below reports it as such, so it follows `CAD_AMBIGUITY_K`
            // rather than the default K a literal 10 would pin it to.
            let escalate = tol.k() * tol.eps();
            println!(
                "     the certified answer and the RSS's disagree over a window {gap:e} m \
                 wide (the certified worst case reaches that much further under). THE \
                 WINDOW IS INSIDE THE BAND: the escalation threshold is {escalate:e} m, \
                 {:.0}× wider, so every bound that separates the two answers is one the \
                 funnel classifies as coincident. The divergence is real and it is \
                 sub-band at this ε — which is a finding about what a certificate can \
                 GATE here, not a defect in either number, and it is the reason this stop \
                 reports a verdict rather than a failure.",
                escalate / gap
            );
        }
        other => println!("     ADVISORY rss unavailable: {other:?}"),
    }
}

/// Indents a rendered report under the tour's narration.
fn indent(text: &str) -> String {
    text.lines()
        .map(|l| format!("     {l}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use pncad::analysis::RefusalReason;

    use super::*;

    /// The hull's padding below and above the true range over the
    /// certified leaves at stop 1's budget (512 leaves, 193 certified),
    /// MEASURED at the default ε in metres — `2.125e-5` below and
    /// `8.500e-5` above the exact affine range `[4.400e-4, 7.600e-4]`
    /// — and pinned at BOTH ends within 2% at the CI row (a hull that
    /// padded more would fail, and so would one whose leaves narrowed:
    /// the widening is proportional to the leaf's width), as a ceiling
    /// at the other ε rows.
    const HULL_SLACK_BELOW: f64 = 2.125e-5;
    const HULL_SLACK_ABOVE: f64 = 8.500e-5;

    /// **The cell's own row, so it EXECUTES on hosted CI.**
    ///
    /// The narration runs in a tour WALK (`cargo run`), and the hosted
    /// lane that walks the tour is a render lane which does not pass
    /// `--features interval`. This row is what puts the cell inside
    /// `ci.yml`'s `demos tour suite` step instead, and it asserts the
    /// findings the captions claim rather than merely running the
    /// code: a real study CERTIFIES — every refusal the leaf budget,
    /// the requirement `Mixed` over the certified leaves with a stated
    /// violated mass, the hull straddling the floor, and the leaves
    /// and padding the caption names (a padded hull straddles more
    /// easily, so the straddle alone would get EASIER as the hull
    /// pads) — and at the certifiable box the certified answer and the
    /// RSS's disagree.
    #[test]
    fn the_two_stops_say_what_their_captions_say() {
        let tol = Tol::witness();

        // Stop 1: the real study CERTIFIES, every refusal is the leaf
        // budget, and the requirement read off the assertion node is
        // MIXED — held where the mass is, violated in the corner.
        let bound = WEB - 1.0e-4;
        let Plate {
            doc,
            measure,
            assertion,
            ..
        } = plate(5.0e-5, 1.0e-5, bound, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let verdict = drive(&doc, &analyzed, &starved(), tol).expect("the nominal builds");
        assert!(
            !verdict.certified().is_empty(),
            "the caption says a ±0.05 mm study certifies; it certified nothing: {:?}",
            verdict.receipt()
        );
        assert!(
            verdict
                .refused()
                .iter()
                .all(|l| matches!(l.reason, RefusalReason::Budget(_))),
            "the caption says every refusal is the leaf budget: {:?}",
            verdict.receipt()
        );
        let decided = assertion_over_leaves(&doc, &verdict, assertion, tol);
        assert_eq!(
            decided,
            Decided::Mixed,
            "the caption says the requirement holds where the mass is and is violated \
             in the corner"
        );
        // The VIOLATED mass is a number, not only a `Mixed`: the floor
        // fails on a certified part of the study, and holds on most of
        // it.
        let masses = requirement_masses(&doc, &analyzed, &verdict, assertion, tol);
        println!(
            "stop 1 at {} leaves: {:?}; holds {:.4}, violated {:.4}, unevaluated {:.4}",
            starved().max_leaves,
            verdict.receipt(),
            masses.holds,
            masses.violated,
            masses.unevaluated
        );
        assert!(
            masses.violated > 0.0 && masses.holds > masses.violated,
            "the caption says the floor is violated on certified mass and holds on most: \
             holds {:.4}, violated {:.4}, unevaluated {:.4}",
            masses.holds,
            masses.violated,
            masses.unevaluated
        );
        // At the CI row (the default ε) the leaf counts the header
        // quotes, exactly: a tier whose reach moved would move them.
        if (tol.eps() / 1.0e-9 - 1.0).abs() < 1.0e-3 {
            assert_eq!(
                (verdict.certified().len(), verdict.refused().len()),
                (193, 319),
                "the header's leaf counts at 512 leaves: {:?}",
                verdict.receipt()
            );
        }
        match stackup(&doc, measure, &analyzed, &verdict, None, true, tol) {
            Ok(report) => {
                assert!(
                    report.worst_case.lo < bound && bound < report.worst_case.hi,
                    "the certified worst case straddles the floor: {:?} against {bound:e}",
                    report.worst_case
                );
                // The straddle beside its padding (R2's Q3): the hull
                // ENCLOSES the true range over the certified leaves
                // and exceeds it by a padding proportional to the
                // leaf's width — bounded here at both ends, so a hull
                // that padded its way across the floor would fail.
                let slack = hull_slack(&verdict, (report.worst_case.lo, report.worst_case.hi));
                println!(
                    "stop 1 hull [{:.6e}, {:.6e}] over {} leaves; {slack:?}",
                    report.worst_case.lo, report.worst_case.hi, report.worst_case.leaves
                );
                assert!(
                    slack.below >= 0.0 && slack.above >= 0.0,
                    "the hull encloses the true range: {slack:?}"
                );
                assert_eq!(
                    report.worst_case.leaves,
                    verdict.certified().len(),
                    "the hull is over every certified leaf"
                );
                assert!(
                    slack.true_lo < bound,
                    "the TRUE range over the certified leaves reaches under the floor — the \
                     straddle is the study's, not the padding's: {slack:?} against {bound:e}"
                );
                let at_the_ci_row = (tol.eps() / 1.0e-9 - 1.0).abs() < 1.0e-3;
                let within = |got: f64, want: f64| {
                    if at_the_ci_row {
                        (got - want).abs() <= 0.02 * want
                    } else {
                        got <= 1.05 * want
                    }
                };
                assert!(
                    within(slack.below, HULL_SLACK_BELOW) && within(slack.above, HULL_SLACK_ABOVE),
                    "the padding is the measured one ({slack:?} against {HULL_SLACK_BELOW:e} / \
                     {HULL_SLACK_ABOVE:e}); if it moved, the leaves moved"
                );
            }
            Err(other) => panic!("the real study's stackup refused: {other}"),
        }
        // The advisory lane still answers, and its label rides it.
        let mc = monte_carlo(&doc, &analyzed, &McConfig::default(), tol).expect("replays");
        assert!(mc.render().contains("ADVISORY"));
        assert_eq!(mc.samples, pncad::analysis::DEFAULT_SAMPLES);

        // Stop 2: the certifiable box certifies, and the two answers
        // disagree in the direction the caption claims.
        let spread = tol.eps() / 64.0;
        let (half_width, sigma) = (0.05 * spread, 0.2 * spread);
        let worst = 2.0 * half_width + 2.0 * (3.0 * sigma);
        let rss3 = 3.0 * ((2.0 * half_width / 3.0_f64.sqrt()).powi(2) + 2.0 * sigma.powi(2)).sqrt();
        let bound = WEB - 0.5 * (worst + rss3);
        let Plate {
            doc,
            measure,
            assertion,
            ..
        } = plate(half_width, sigma, bound, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let verdict =
            drive(&doc, &analyzed, &DriveConfig::default(), tol).expect("the nominal builds");
        assert!(
            !verdict.certified().is_empty(),
            "the ε-scaled box is the one that certifies"
        );
        let report =
            stackup(&doc, measure, &analyzed, &verdict, None, true, tol).expect("a stackup");
        assert!(
            report.worst_case.lo < bound,
            "the caption's punchline: the CERTIFIED worst case reaches under the bound"
        );
        match report.rss {
            pncad::analysis::Rss::Advisory { sigma } => assert!(
                report.nominal.expect("the web has an f64 nominal") - 3.0 * sigma >= bound,
                "and the RSS's 3σ reading does not — that disagreement is the cell's \
                 subject; σ = {sigma:e}"
            ),
            ref other => panic!("every contributor carries a measure here: {other:?}"),
        }
        // **The verdict the caption reports, read off the node** — the
        // correction R1 forced. `worst_case.lo < bound` is true above
        // by less than eps, and the recorded requirement says HOLDS
        // over exactly those leaves. A caption that printed "FAILS"
        // off the float contradicted the row that gates.
        let decided = assertion_over_leaves(&doc, &verdict, assertion, tol);
        assert_eq!(
            decided,
            Decided::Holds,
            "the straddle is inside the coincidence band, so the assertion node HOLDS — \
             and the caption must say what the node says"
        );
        // The margin the caption calls sub-band really is sub-band.
        let margin = report.worst_case.lo - bound;
        assert!(
            margin < 0.0 && margin.abs() < tol.eps(),
            "the caption says the enclosure reaches under the bound by less than eps: \
             margin {margin:e}, eps {:e}",
            tol.eps()
        );
        // And the DIVERGENCE window the caption sizes: the certified
        // answer reaches further under than the RSS's, and the whole
        // disagreement is narrower than the escalation threshold. That
        // second half is the honest limit this stop reports, so it is
        // asserted rather than narrated.
        let sigma = match report.rss {
            pncad::analysis::Rss::Advisory { sigma } => sigma,
            ref other => panic!("every contributor carries a measure here: {other:?}"),
        };
        let gap = (report.nominal.expect("the web has an f64 nominal") - 3.0 * sigma)
            - report.worst_case.lo;
        assert!(
            gap > 0.0,
            "the certified worst case must reach further under than 3σ"
        );
        assert!(
            gap < tol.k() * tol.eps(),
            "the caption says the whole divergence is inside the escalation threshold: \
             window {gap:e} against {:e}",
            tol.k() * tol.eps()
        );
        // The MC lane's number, which the caption now READS rather than
        // hardcodes: it must exist and be a fraction.
        let mc = monte_carlo(&doc, &analyzed, &McConfig::default(), tol).expect("replays");
        let fraction = mc
            .assertions
            .iter()
            .find(|a| a.node == assertion)
            .and_then(|a| a.violation_fraction())
            .expect("the sampled assertion has a violation fraction to report");
        assert!(
            (0.0..=1.0).contains(&fraction),
            "a violation fraction is a fraction: {fraction}"
        );
        assert_eq!(mc.samples, pncad::analysis::DEFAULT_SAMPLES);
    }
}
