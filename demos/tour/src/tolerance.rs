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
    AnalysisPolicy, AnalyzedBox, DriveConfig, MassBudget, McConfig, ParamBoxVerdict, Stackup,
    StackupRefusal, analyzed_box, assertion_at, drive, leaf_histogram, monte_carlo,
    render_sensitivity, stackup,
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
                 That is a finding about the widening class (issue 1191), not about the \
                 plate — the cell prints it rather than choosing a box that flatters the \
                 kernel."
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
            bound - stackup(&doc, measure, &analyzed, &verdict, None, true, tol)
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
        let one = match assertion_at(doc, assertion, &leaf.box_, tol) {
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

/// **A bound the run can decide, so the gate is seen gating.**
///
/// The interesting bound — the one between the certified worst case and
/// the RSS's 3σ figure — is undecidable at this ε, and
/// `print_divergence` says why. This one is not: it sits a full
/// escalation threshold above the enclosure, so the margin is definite
/// and the assertion reads a plain `Violated`. Printed so the stop does
/// not leave a reader thinking the requirement machinery only ever
/// refuses.
fn definite_arm(
    doc: &ProfileDoc,
    analyzed: &AnalyzedBox,
    measure: RecipeNodeId,
    tol: Tol,
) {
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
fn print_divergence(
    report: &Stackup,
    bound: f64,
    linear_worst: f64,
    decided: &Decided,
    tol: Tol,
) {
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
            let three_sigma = report.nominal - 3.0 * sigma;
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
            let escalate = 10.0 * tol.eps();
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
        let (doc, measure, assertion) = plate(half_width, sigma, bound, tol);
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
        // **The verdict the caption reports, read off the node** — the
        // correction R1 forced. `worst_case.lo < bound` is true above
        // by less than eps, and the recorded requirement says HOLDS
        // over exactly those leaves. A caption that printed "FAILS"
        // off the float contradicted the row that gates.
        let decided = assertion_over_leaves(&doc, &verdict, assertion, tol);
        assert_eq!(
            decided,
            Decided::Holds,
            "the straddle is inside the coincidence band, so the assertion node HOLDS —              and the caption must say what the node says"
        );
        // The margin the caption calls sub-band really is sub-band.
        let margin = report.worst_case.lo - bound;
        assert!(
            margin < 0.0 && margin.abs() < tol.eps(),
            "the caption says the enclosure reaches under the bound by less than eps:              margin {margin:e}, eps {:e}",
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
        let gap = (report.nominal - 3.0 * sigma) - report.worst_case.lo;
        assert!(gap > 0.0, "the certified worst case must reach further under than 3σ");
        assert!(
            gap < 10.0 * tol.eps(),
            "the caption says the whole divergence is inside the escalation threshold:              window {gap:e} against {:e}",
            10.0 * tol.eps()
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
