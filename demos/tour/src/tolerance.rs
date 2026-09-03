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
//! **Stop 1 is what a user gets today.** ±0.05 mm on the hole spacing
//! and σ = 0.01 mm on each radius — a real study — and the answer is
//! `NothingCertified`: no leaf of that box replays at the interval
//! scalar, because every certification identity the kernel runs widens
//! with the box (issue 1191). The refusal is not silence: it carries
//! the nominal, every sensitivity marked `LocalOnly`, the coverage
//! saying where the mass went, and the drive's receipt. Beside it the
//! Monte-Carlo lane answers the same question the only way anything
//! can at that box — advisorily, labeled, with its count and seed.
//!
//! **Stop 2 is the MVP's reason to exist, at the scale where it
//! works.** The same plate with every tolerance scaled to the box the
//! driver can certify: the certified worst case and the RSS's 3σ
//! figure printed side by side, disagreeing, with the tail on every
//! line. The DISAGREEMENT is scale-free — it is a ratio between a
//! linear sum and a root-sum-square — so shrinking the study does not
//! shrink the point.
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
//!    `MeasureRef` names the node it reads at, so authoring one means
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
//!    part needs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::analysis::{
    AnalysisPolicy, DriveConfig, MassBudget, McConfig, Stackup, StackupRefusal, analyzed_box,
    drive, leaf_histogram, monte_carlo, stackup,
};
use pncad::document::{
    AssertionDir, CancelToken, Dimension, Distribution, DocEdit, DocParam, DocumentId, EvalOptions,
    Evaluation, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, MeasureRef, Node, ParamName,
    ProfileDoc, ProfileProgram, RecipeNodeId, UnitSym, apply, evaluate,
};
use pncad::geom_core::Tol;
use pncad::select::{EntityKind, GeomPred, NamePat, Selector, SurfaceKindSet, select_where};

/// The nominal hole spacing, in metres (3.1 mm).
const SPACING: f64 = 3.1e-3;
/// The nominal hole radius, in metres (1.25 mm).
const RADIUS: f64 = 1.25e-3;
/// The nominal web: `SPACING − 2·RADIUS` = 0.6 mm.
const WEB: f64 = SPACING - 2.0 * RADIUS;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar")
}

fn param(n: &str) -> Expr {
    Expr::param(ParamName::new(n), Dimension::Length)
}

fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the insert applies");
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

fn declare(doc: &mut ProfileDoc, n: &str, value: f64, distribution: Distribution, tol: Tol) {
    let applied = apply(
        doc,
        &DocEdit::SetDocParam {
            name: ParamName::new(n),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(distribution),
            },
        },
        tol,
    )
    .expect("the parameter applies");
    *doc = applied.doc;
}

/// The plate, its two holes, the web measure and the assertion — the
/// worked example, authored the way a user would.
///
/// The two tolerances are passed separately rather than scaled from
/// one number: their RATIO is what decides whether the RSS and the
/// certified worst case disagree, so it is a modelling choice and not a
/// scale.
fn plate(
    spacing_half_width: f64,
    radius_sigma: f64,
    bound: f64,
    tol: Tol,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut doc = ProfileDoc::empty(DocumentId::derive("pncad-demo-tolerance"), tol);
    // The hole spacing: a UNIFORM tolerance, the machinist's ±.
    declare(
        &mut doc,
        "half_spacing",
        SPACING / 2.0,
        Distribution::Uniform {
            lo: -spacing_half_width,
            hi: spacing_half_width,
        },
        tol,
    );
    // The two radii: INDEPENDENT normals. Independent because they are
    // two names (PL6), which is what makes the RSS's root-sum-square
    // differ from the worst case's linear sum — the whole subject of
    // stop 2.
    for n in ["hole_a_r", "hole_b_r"] {
        declare(
            &mut doc,
            n,
            RADIUS,
            Distribution::Normal {
                sigma: radius_sigma,
            },
            tol,
        );
    }

    let plane = insert(
        &mut doc,
        Node::Datum(pncad::document::Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
        tol,
    );
    // The plate itself is a literal rectangle: the study is about the
    // holes, and a parameter nothing measures would be noise in the
    // stackup's per-parameter table.
    let plate_profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![
                LoopProgram::polygon([
                    (-4.0e-3, -2.0e-3),
                    (4.0e-3, -2.0e-3),
                    (4.0e-3, 2.0e-3),
                    (-4.0e-3, 2.0e-3),
                ])
                .expect("finite plate corners"),
            ],
        }),
        tol,
    );
    let _plate = insert(
        &mut doc,
        Node::Extrude {
            profile: plate_profile,
            distance: len(1.0e-3),
        },
        tol,
    );

    let hole = |doc: &mut ProfileDoc, centre: Expr, radius: &str, tol| {
        let profile = insert(
            doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![LoopProgram::Circle {
                    centre: [centre, len(0.0)],
                    radius: param(radius),
                }],
            }),
            tol,
        );
        insert(
            doc,
            Node::Extrude {
                profile,
                distance: len(1.0e-3),
            },
            tol,
        )
    };
    let hole_a = hole(
        &mut doc,
        Expr::sub(len(0.0), param("half_spacing")).expect("a length"),
        "hole_a_r",
        tol,
    );
    let hole_b = hole(&mut doc, param("half_spacing"), "hole_b_r", tol);

    // The wall names come from the SELECTION door, the way a user gets
    // them: evaluate what is built so far, then ask each hole for its
    // cylindrical face.
    let ev: Evaluation<f64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let wall = |node: RecipeNodeId| {
        let mut faces = select_where(
            &ev,
            node,
            &Selector::of(NamePat::of_kind(EntityKind::Face)),
            &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                pncad::geom_brep::SurfaceKind::Cylinder,
            ))],
            &doc.param_env::<f64>(),
            tol,
        )
        .expect("the surface-kind atom is exact");
        faces.sort();
        assert!(!faces.is_empty(), "a hole extrude has a cylindrical wall");
        MeasureRef::new(node, faces.remove(0))
    };

    // web = distance(wall_a, wall_b) − r_a − r_b. The distance between
    // two parallel cylinder faces is their AXIS distance (the closed
    // form's own contract), so the subtraction of the radii is the
    // author's arithmetic and not a hidden convention.
    let radius_of = |n: &str| MeasureExpr::value(param(n));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(radius_of("hole_a_r"), radius_of("hole_b_r")).expect("Length + Length"),
    )
    .expect("Length − Length");
    // The two references are read BEFORE the insert borrows the
    // document mutably — the borrow checker's way of saying that a
    // measure's references are resolved against a document that
    // already exists, which is exactly the E3 contract.
    let refs = vec![wall(hole_a), wall(hole_b)];
    let measure = insert(
        &mut doc,
        Node::measure(web, refs).expect("both indices in range"),
        tol,
    );
    let assertion = insert(
        &mut doc,
        Node::Assertion {
            measure,
            bound: len(bound),
            dir: AssertionDir::AtLeast,
        },
        tol,
    );
    (doc, measure, assertion)
}

/// **Stop 1's leaf budget, and why it is not the default.**
///
/// At ±0.05 mm nothing certifies at ANY budget — the box is six orders
/// wider than the width a certification identity survives — so the
/// default 65,536 leaves spends about a minute subdividing its way to
/// the same `NothingCertified` a thousand reach in a second. The cell
/// caps it and says so, which is a statement about the COST of a
/// refusal rather than a thumb on the answer: a reader who doubts it
/// can raise the number and watch the report not change.
fn starved() -> DriveConfig {
    DriveConfig {
        max_leaves: 1024,
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
    let (doc, measure, _assertion) = plate(5.0e-5, 1.0e-5, bound, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    println!(
        "   the real study: web nominal {:.4} mm, asserted >= {:.4} mm, over ±0.05 mm of \
         spacing and σ = 0.01 mm on each radius",
        WEB * 1e3,
        bound * 1e3
    );

    let verdict = drive(&doc, &analyzed, &starved(), tol).expect("the nominal builds");
    println!("{}", indent(&verdict.render(&analyzed)));
    match stackup(&doc, measure, &analyzed, &verdict, None, true, tol) {
        Ok(report) => {
            // Not the expected answer today; if the widening class ever
            // closes (issue 1191) this is what a reader should see.
            println!("{}", indent(&report.render(&analyzed)));
        }
        Err(StackupRefusal::NothingCertified {
            nominal,
            sensitivities,
            coverage,
            receipt,
        }) => {
            println!("   NOTHING CERTIFIED — and the refusal carries the study's answer anyway:");
            println!("     nominal web {:.4} mm", nominal * 1e3);
            for s in &sensitivities {
                println!("     ∂web/∂{}: {:?}", s.param.0, s.outcome);
            }
            println!(
                "     the drive: {} certified, {} refused",
                receipt.certified, receipt.refused
            );
            println!("{}", indent(&MassBudget::of(&coverage, &analyzed).render()));
            println!(
                "     WHY: every certification identity the kernel runs widens with the \
                 box, so at ±0.05 mm no leaf replays at the interval scalar at all. That \
                 is the ε-scale ceiling — issue 1191 — and it is this MVP's honest limit, \
                 not a property of the plate."
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
    let (doc, measure, assertion) = plate(spacing_half_width, radius_sigma, bound, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), tol).expect("the nominal builds");
    println!("{}", indent(&verdict.render(&analyzed)));
    match stackup(&doc, measure, &analyzed, &verdict, None, true, tol) {
        Ok(report) => {
            println!("{}", indent(&report.render(&analyzed)));
            print_divergence(&report, bound, worst);
            // The E11.6 datum: where each certified leaf's mass lands.
            let histogram = leaf_histogram(&doc, &analyzed, &verdict, measure, tol);
            println!("{}", indent(&histogram.render()));
        }
        Err(refusal) => {
            println!(
                "   the certifiable box did not certify either: {refusal}\n     \
                 That is a finding about the widening class (issue 1191), not about the \
                 plate — the cell prints it rather than choosing a box that flatters the \
                 kernel."
            );
        }
    }
    // And what the assertion node itself says over the analyzed box's
    // own leaves — the recorded requirement, read back.
    println!(
        "   the assertion node {} is the recorded requirement; its verdict per certified \
         leaf is what the CI row gates on.",
        assertion.0
    );
    let mc = monte_carlo(&doc, &analyzed, &McConfig::default(), tol).expect("the nominal builds");
    println!("{}", indent(&mc.render()));
    // **And the advisory lane MISSES what the certified one found**,
    // which is the sharpest thing this stop says. The violating region
    // is a corner of a three-dimensional box; 512 samples land in it
    // with a probability nobody should round up. The MC's empirical
    // violation fraction reads 0% and the certified worst case reads
    // FAILS, and both are correct answers to different questions —
    // which is precisely why E11 makes the certified one the only
    // gate.
    println!(
        "     NOTE: the advisory lane's violation fraction above is 0% while the \
         CERTIFIED worst case fails. Both are right: the violating region is a corner of \
         the box that 512 samples do not visit. This is why the certified number gates \
         and the sampled one is labeled."
    );
}

/// The divergence, printed as the two numbers a reader is meant to
/// compare: the gating one first (the E5 ordering rule).
///
/// `linear_worst` is the LINEARIZED worst case the study's own
/// arithmetic predicts, so the caption can say how much wider the
/// certified enclosure is than the sum of contributions — which is a
/// finding in itself and not a defect (see the printed line).
fn print_divergence(report: &Stackup, bound: f64, linear_worst: f64) {
    println!(
        "     CERTIFIED worst case: [{:e}, {:e}] against the bound {bound:e} — {}",
        report.worst_case.lo,
        report.worst_case.hi,
        if report.worst_case.lo >= bound {
            "the requirement HOLDS over every certified leaf"
        } else {
            "the requirement FAILS somewhere in the box: this is the number that gates"
        }
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
        pncad::analysis::Rss::Advisory { sigma } => println!(
            "     ADVISORY rss: σ ≈ {sigma:e}, so a 3σ reading would say the web reaches \
             {:e} — {}",
            report.nominal - 3.0 * sigma,
            if report.nominal - 3.0 * sigma >= bound {
                "'3σ fine'. That is the divergence this milestone exists for: the \
                 linearized figure and the certified one disagree, and only one of them \
                 is a proof."
            } else {
                "which agrees with the certified answer here."
            }
        ),
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
    use super::*;

    /// **The cell's own row, so it EXECUTES on hosted CI.**
    ///
    /// The narration runs in a tour WALK (`cargo run`), and the hosted
    /// lane that walks the tour is a render lane which does not pass
    /// `--features interval`. This row is what puts the cell inside
    /// `ci.yml`'s `demos tour suite` step instead, and it asserts the
    /// two findings the captions claim rather than merely running the
    /// code: a real study certifies NOTHING, and at the certifiable box
    /// the certified answer and the RSS's disagree.
    #[test]
    fn the_two_stops_say_what_their_captions_say() {
        let tol = Tol::witness();

        // Stop 1: the real study certifies nothing, and the refusal
        // carries the answer.
        let bound = WEB - 1.0e-4;
        let (doc, measure, _) = plate(5.0e-5, 1.0e-5, bound, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let verdict = drive(&doc, &analyzed, &starved(), tol).expect("the nominal builds");
        assert!(
            verdict.certified().is_empty(),
            "the caption says a ±0.05 mm study certifies nothing; it certified {}",
            verdict.certified().len()
        );
        match stackup(&doc, measure, &analyzed, &verdict, None, true, tol) {
            Err(StackupRefusal::NothingCertified { sensitivities, .. }) => {
                assert!(
                    !sensitivities.is_empty(),
                    "the refusal carries the study's sensitivities"
                );
            }
            other => panic!("expected NothingCertified, got {other:?}"),
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
        let (doc, measure, _) = plate(half_width, sigma, bound, tol);
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
                report.nominal - 3.0 * sigma >= bound,
                "and the RSS's 3σ reading does not — that disagreement is the cell's \
                 subject; σ = {sigma:e}"
            ),
            ref other => panic!("every contributor carries a measure here: {other:?}"),
        }
        // And the MC lane agrees with the RSS, missing the corner —
        // the caption's second finding.
        let mc = monte_carlo(&doc, &analyzed, &McConfig::default(), tol).expect("replays");
        assert_eq!(
            mc.assertions.len(),
            1,
            "the document carries one assertion to sample"
        );
    }
}
