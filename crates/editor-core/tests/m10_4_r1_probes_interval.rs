//! **R1's independent consumer probes for M10-4** (E4/E5/E9;
//! `docs/M10-4-SPEC.md`). Written against the PUBLIC doors only —
//! `analyzed_box`, `drive`, `sensitivities`, `stackup`, `seed_env` — and
//! deriving every expected number independently of the implementation's
//! own fixtures.
//!
//! The rows split into three kinds and each says which it is:
//!
//! - **PIN** — an independently derived law this unit must keep.
//! - **DATUM** — a state the reviewer measured and is recording as the
//!   shipped behaviour, red-capable if it changes.
//! - **EVIDENCE-ONLY** — a print, no assertion that can fail on a
//!   number (`memories/test-suite-cost.md`).
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::analysis::{AnalysisPolicy, analyzed_box, std_deviation};
use editor_core::drive::{DriveConfig, drive};
use editor_core::stackup::{
    Chamber, Rss, SensitivityOutcome, SensitivityRefusal, StackupRefusal, Unavailable,
    sensitivities, stackup,
};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DocParam, EvalOptions, Evaluation, Expr,
    LoopProgram, MeasureExpr, MeasurePrimitive, MeasureRef, Node, ParamName, ParamValue,
    ProfileDoc, ProfileLift, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId, UnitSym, ValuePayload, evaluate, seed_env,
};
use geom_core::{Dual64, Tol};
use profile::SketchPlane;

use fixture::{Recorder, len, scl};

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

fn param(n: &str, dim: Dimension) -> Expr {
    Expr::param(name(n), dim)
}

fn eps() -> f64 {
    Tol::witness().eps()
}

fn uniform(half: f64) -> Distribution {
    Distribution::Uniform {
        lo: -half,
        hi: half,
    }
}

fn continuous(dim: Dimension, value: f64, distribution: Option<Distribution>) -> DocParam {
    DocParam::Continuous {
        dim,
        value,
        display_unit: UnitSym::canonical_for(dim),
        distribution,
    }
}

fn config(max_leaves: usize) -> DriveConfig {
    DriveConfig {
        max_leaves,
        ..DriveConfig::default()
    }
}

fn eval_f64(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn opts(seed: Option<&str>, lift: ProfileLift) -> EvalOptions {
    EvalOptions {
        seed: seed.map(name),
        profile_lift: lift,
        ..EvalOptions::default()
    }
}

fn measured(ev: &Evaluation<Dual64>, id: RecipeNodeId) -> Dual64 {
    match ev.result(id) {
        Some(editor_core::NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => *value,
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        other => panic!("node {id:?} did not evaluate: {other:?}"),
    }
}

// ------------------------------------------------------------ fixtures

/// **My own measured, distributed document — a stepped shaft.** Two
/// stacked square prisms: a `base` of side 2 extruded by `h1`, and on
/// top of it (translated by `h1`) a `boss` of side 1 extruded by
/// `h2` — nothing but magnitude slots, so the geometry is different
/// from the plate's and the analytic answer is unambiguous. The measure
/// is the distance from the base's bottom cap to the boss's top cap,
/// i.e. `h1 + h2`: ∂m/∂h1 = ∂m/∂h2 = 1 exactly.
fn stepped_shaft(
    h1: f64,
    h2: f64,
    d1: Option<Distribution>,
    d2: Option<Distribution>,
) -> (ProfileDoc, RecipeNodeId) {
    stepped_shaft_sized(1.0, h1, h2, d1, d2)
}

fn stepped_shaft_sized(
    size: f64,
    h1: f64,
    h2: f64,
    d1: Option<Distribution>,
    d2: Option<Distribution>,
) -> (ProfileDoc, RecipeNodeId) {
    use editor_core::{CapEnd, RoleSeg};

    use fixture::fname;

    let (o, i) = (size, 0.5 * size);
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("h1"),
        value: continuous(Dimension::Length, h1, d1),
    });
    r.push(DocEdit::SetDocParam {
        name: name("h2"),
        value: continuous(Dimension::Length, h2, d2),
    });
    let base_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(-o, -o), (o, -o), (o, o), (-o, o)]).expect("finite corners"),
        ],
    }));
    let base = r.insert(Node::Extrude {
        profile: base_p,
        distance: param("h1", Dimension::Length),
    });
    let boss_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(-i, -i), (i, -i), (i, i), (-i, i)]).expect("finite corners"),
        ],
    }));
    let boss_raw = r.insert(Node::Extrude {
        profile: boss_p,
        distance: param("h2", Dimension::Length),
    });
    let boss = r.insert(Node::Transform {
        input: boss_raw,
        translation: [len(0.0), len(0.0), param("h1", Dimension::Length)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
    });
    let refs = vec![
        MeasureRef::new(base, fname(base, RoleSeg::Cap(CapEnd::Bottom))),
        MeasureRef::new(boss, fname(boss_raw, RoleSeg::Cap(CapEnd::Top))),
    ];
    let m = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            refs,
        )
        .expect("indices in range"),
    );
    (r.doc, m)
}

/// A pure-arithmetic measure `m = f(a)` over one Scalar parameter `a`,
/// with no geometry at all — the drive certifies the whole box in one
/// leaf, so a row over it is about the report's own arithmetic.
fn scalar_measure(
    nominal: f64,
    dist: Distribution,
    build: impl Fn(&dyn Fn() -> MeasureExpr) -> MeasureExpr,
) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("a"),
        value: continuous(Dimension::Scalar, nominal, Some(dist)),
    });
    let a = || MeasureExpr::value(param("a", Dimension::Scalar));
    let m = r.insert(Node::measure(build(&a), Vec::new()).expect("no references to address"));
    (r.doc, m)
}

/// **An ARC-carrying profile** the PR did not exercise: a chain whose
/// second leg is a sharp `arc_to` through a point, with the profile's
/// x-extent driven by a document parameter `w`. Extruded by a literal;
/// the measure is the distance between the two x-walls, which is `w`.
fn arc_slab(w: f64) -> (ProfileDoc, RecipeNodeId) {
    use fixture::{fname, wall};

    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("w"),
        value: continuous(Dimension::Length, w, None),
    });
    // A chain: (0,0) -> (w,0) [line, seg 0] -> (w,1) [line, seg 1] ->
    // arc through (w/2, 1.25) to (0,1) [seg 2] -> close [seg 3]. Both
    // x-walls stay planar so the measure has a closed form, and the
    // ARC's derived centre/radius are parameter-driven through both its
    // via point and its endpoints — a lane the polygon fixtures never
    // touch.
    let half_w = Expr::mul(param("w", Dimension::Length), scl(0.5)).expect("Length · Scalar");
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::LineTo(ProgramTarget::Point([
            param("w", Dimension::Length),
            len(0.0),
        ])),
        ProgramStep::LineTo(ProgramTarget::Point([
            param("w", Dimension::Length),
            len(1.0),
        ])),
        ProgramStep::ArcTo(ProgramArcData::Via {
            q: [half_w, len(1.25)],
            target: ProgramTarget::Point([len(0.0), len(1.0)]),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![chain],
    }));
    let slab = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    // Segment 3 is the x = 0 wall, segment 1 the x = w wall; their
    // distance is `w`, so ∂m/∂w = 1 exactly.
    let refs = vec![
        MeasureRef::new(slab, fname(slab, wall(3))),
        MeasureRef::new(slab, fname(slab, wall(1))),
    ];
    let m = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            refs,
        )
        .expect("indices in range"),
    );
    (r.doc, m)
}

// ------------------------------------------------- claim 1: unseeded

/// **PIN — claim 1, the merge-base differential this reviewer can
/// take.** `seed: None` leaves the evaluation identical at EVERY
/// scalar the seam touches, not only `f64`: same node order, same
/// per-node content keys, same result arms, and the same measured bits
/// at `f64`, `Dual64` and `Interval`. The unit's own row checks `f64`
/// keys over the corpus; this one checks the three scalars the seed
/// door discriminates between, which is where a dropped-seed bug would
/// hide.
#[test]
fn r1_seed_none_is_bit_identical_at_every_scalar() {
    use geom_core::interval::Interval;

    let (doc, m) = stepped_shaft(1.0, 0.5, Some(uniform(eps() / 16.0)), None);
    macro_rules! same {
        ($t:ty) => {{
            let d: Evaluation<$t> = evaluate(
                &doc,
                None,
                &CancelToken::new(),
                &EvalOptions::default(),
                Tol::witness(),
            );
            let e: Evaluation<$t> = evaluate(
                &doc,
                None,
                &CancelToken::new(),
                &opts(None, ProfileLift::Pinned),
                Tol::witness(),
            );
            assert_eq!(d.order, e.order, "{}", stringify!($t));
            for id in &d.order {
                assert_eq!(
                    d.value(*id).map(|v| v.content_key),
                    e.value(*id).map(|v| v.content_key),
                    "{}: node {}",
                    stringify!($t),
                    id.0
                );
            }
            assert_eq!(d.recomputed, e.recomputed);
        }};
    }
    same!(f64);
    same!(Dual64);
    same!(Interval);
    // And an unseeded dual carries an exactly zero tangent at the sink.
    let unseeded: Evaluation<Dual64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    // NOTE: `-0.0` on this fixture (`+0.0` on the plate the unit's own
    // row uses) — the "exactly zero" law is IEEE-zero, not bit-zero.
    assert_eq!(measured(&unseeded, m).deriv, 0.0);
}

// -------------------------------------------- claim 2: seed hygiene

/// **PIN — claim 2, on MY fixture.** The stepped shaft's measure is
/// `h1 + h2`, so both tangents are exactly 1 and the unseeded pass is
/// exactly 0; the value channel is the f64 build's bits in every pass;
/// and the schedule does not leak (rayon vs sequential, bit for bit).
#[test]
fn r1_seed_hygiene_and_schedule_independence_on_a_stepped_shaft() {
    let (doc, m) = stepped_shaft(1.0, 0.5, None, None);
    let f = eval_f64(&doc);
    let Some(editor_core::NodeResult::Ok(v)) = f.result(m) else {
        panic!("the shaft measures at f64")
    };
    let ValuePayload::Measure { value: nominal, .. } = v.payload else {
        panic!("a measure")
    };
    assert!((nominal - 1.5).abs() < 1e-12, "nominal {nominal}");
    for p in ["h1", "h2"] {
        for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
            let d = measured(
                &evaluate(
                    &doc,
                    None,
                    &CancelToken::new(),
                    &opts(Some(p), lift),
                    Tol::witness(),
                ),
                m,
            );
            assert_eq!(
                d.deriv.to_bits(),
                1.0f64.to_bits(),
                "∂m/∂{p} under {lift:?}"
            );
            assert_eq!(d.value.to_bits(), nominal.to_bits(), "{p} value channel");
        }
    }
    // Schedule independence at the driver, on this document.
    let seq = sensitivities(&doc, m, None, None, false, Tol::witness()).expect("ok");
    let par = sensitivities(&doc, m, None, None, true, Tol::witness()).expect("ok");
    assert_eq!(seq, par);
    assert_eq!(seq.len(), 2);
}

/// **PIN — DL2, exercised on a shared subgraph of MY choosing.** Two
/// passes over the shaft share the whole `boss` sub-build when seeded
/// on `h1`... except that `h1` also translates the boss, so the shared
/// subgraph is the boss's own profile+extrude. The law: (a) a pass
/// threaded from the OTHER pass reads its own tangent bit for bit; (b)
/// nodes outside the seeded cone are SERVED (reuse > 0); (c) nodes
/// inside either cone are re-keyed, so the threaded pass's key at the
/// measure differs from the prior's.
#[test]
fn r1_dl2_two_passes_share_a_subgraph_without_aliasing() {
    let (doc, m) = stepped_shaft(1.0, 0.5, None, None);
    let on_h1: Evaluation<Dual64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &opts(Some("h1"), ProfileLift::Guided),
        Tol::witness(),
    );
    let on_h2_fresh: Evaluation<Dual64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &opts(Some("h2"), ProfileLift::Guided),
        Tol::witness(),
    );
    let on_h2_threaded: Evaluation<Dual64> = evaluate(
        &doc,
        Some(&on_h1),
        &CancelToken::new(),
        &opts(Some("h2"), ProfileLift::Guided),
        Tol::witness(),
    );
    let (fresh, threaded) = (measured(&on_h2_fresh, m), measured(&on_h2_threaded, m));
    assert_eq!(
        threaded.deriv.to_bits(),
        fresh.deriv.to_bits(),
        "the threaded pass must read its OWN tangent"
    );
    assert_eq!(threaded.deriv.to_bits(), 1.0f64.to_bits());
    assert_eq!(threaded.value.to_bits(), fresh.value.to_bits());
    // Keys: the measure is downstream of both seeds, so the two passes
    // must key it differently — the memo cannot alias them.
    assert_ne!(
        on_h1.value(m).map(|v| v.content_key),
        on_h2_fresh.value(m).map(|v| v.content_key),
        "two seeds must not share the measure's key"
    );
    // And something WAS served: the boss's own profile is outside both
    // cones (it is driven by neither parameter).
    assert!(
        on_h2_threaded.reused > 0,
        "nothing was seed-independent: {} reused",
        on_h2_threaded.reused
    );
}

// ---------------------------------- claim 3/4: the verdict is ungated

/// **DATUM — the reviewer's counterexample to "no path yields a
/// sensitivity of an unvalidated build".** The pairing hook gates the
/// f64 anchor against the handed build. It does NOT gate the `chamber`
/// verdict against the document. So: drive the shaft, then edit a piece
/// of the document the analyzed box cannot see (a literal — the boss's
/// own extrude becomes a parameter-free 0.75 instead of `h2`), hand the
/// FRESH f64 build so the pairing hook is satisfied, and pass the STALE
/// verdict. The driver returns `ChamberCertified` carrying the old
/// drive's leaf and `verdict_vector_key` — a chamber certificate for a
/// build nobody drove.
///
/// Red-capable in the direction that matters: it goes red the day the
/// driver gates the verdict against the document.
#[test]
fn r1_a_stale_verdict_still_mints_a_chamber_certificate() {
    let half = eps() / 16.0;
    let (doc, m) = stepped_shaft(1.0, 0.5, Some(uniform(half)), Some(uniform(half)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty(), "{:?}", verdict.receipt());

    // Edit the document in a way the analyzed box cannot see: node 3 is
    // the boss extrude; retarget its distance off the parameter onto a
    // literal. The parameter SET, the nominals and the distributions
    // are untouched, so `ParamBox::of(analyzed)` is unchanged.
    let edited = editor_core::apply(
        &doc,
        &DocEdit::SetParam {
            node: RecipeNodeId(3),
            slot: editor_core::SlotId::Distance,
            expr: len(0.75),
        },
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the edit applies: {e}"))
    .doc;
    let analyzed_edited = analyzed_box(&edited, &AnalysisPolicy::default());
    assert_eq!(
        editor_core::ParamBox::of(&analyzed_edited),
        *verdict.root(),
        "the box the ForeignBox check compares is unchanged by this edit"
    );

    // The FRESH f64 build of the edited document: the pairing hook is
    // fully satisfied.
    let handed = eval_f64(&edited);
    // PIN (flipped from DATUM at the fix pass): the verdict's certified
    // leaf is content-tied to the build — the retargeted slot re-keys
    // node 3 in the leaf's replay — so the stale verdict is refused
    // typed, in the driver and in the report alike, before any mark is
    // written.
    let refused = sensitivities(
        &edited,
        m,
        Some(&handed),
        Some(&verdict),
        false,
        Tol::witness(),
    );
    assert!(
        matches!(
            refused,
            Err(SensitivityRefusal::VerdictNotOfThisBuild { .. })
        ),
        "a stale verdict must be refused by content: {refused:?}"
    );
    let report = stackup(
        &edited,
        m,
        &analyzed_edited,
        &verdict,
        Some(&handed),
        false,
        Tol::witness(),
    );
    assert!(
        matches!(
            report,
            Err(StackupRefusal::Sensitivity(
                SensitivityRefusal::VerdictNotOfThisBuild { .. }
            ))
        ),
        "the stackup must not price an edited document with the old leaves: {report:?}"
    );
    // The edited document's OWN sensitivity has changed (h2 no longer
    // drives anything), which is the proof that the certified chamber
    // is not this document's — read without a chamber, where it is
    // honestly `LocalOnly`.
    let entries = sensitivities(&edited, m, Some(&handed), None, false, Tol::witness())
        .expect("the pairing hook is satisfied by a fresh anchor");
    match entries
        .iter()
        .find(|s| s.param == name("h2"))
        .map(|s| &s.outcome)
    {
        Some(SensitivityOutcome::Derivative { value, .. }) => {
            // IEEE zero, not bit-zero: a zero tangent arrives signed.
            assert_eq!(*value, 0.0, "h2 drives nothing in the edited document");
        }
        other => panic!("{other:?}"),
    }
}

/// **DATUM — the same hole, through a wholly different document.** A
/// verdict driven over document A certifies document B's sensitivities
/// whenever B's continuous parameter NAMES and analyzed offsets match
/// A's. `ForeignVerdict` compares only the axis name set.
#[test]
fn r1_another_documents_verdict_certifies_this_one() {
    let half = eps() / 16.0;
    let (a_doc, _) = stepped_shaft(1.0, 0.5, Some(uniform(half)), Some(uniform(half)));
    let a_analyzed = analyzed_box(&a_doc, &AnalysisPolicy::default());
    let a_verdict = drive(&a_doc, &a_analyzed, &config(1024), Tol::witness()).expect("builds");
    assert!(!a_verdict.certified().is_empty());

    // Document B: the same two parameter names, nominals and
    // distributions, DIFFERENT geometry — a shaft whose base and boss
    // are two-thirds the size and whose nominals are the same. Its own
    // drive is a different drive; A's is the one handed in.
    let (b_doc, b_m) =
        stepped_shaft_sized(0.66, 1.0, 0.5, Some(uniform(half)), Some(uniform(half)));
    let b_analyzed = analyzed_box(&b_doc, &AnalysisPolicy::default());
    assert_eq!(
        editor_core::ParamBox::of(&b_analyzed),
        *a_verdict.root(),
        "the two documents share an analyzed box"
    );

    // PIN (flipped from DATUM at the fix pass): A's leaf replays over B
    // with different keys at the first node whose geometry differs, so
    // A's verdict is refused by content — in the report and in the
    // driver — however alike the two documents' parameter sets are.
    let report = stackup(
        &b_doc,
        b_m,
        &b_analyzed,
        &a_verdict,
        None,
        false,
        Tol::witness(),
    );
    assert!(
        matches!(
            report,
            Err(StackupRefusal::Sensitivity(
                SensitivityRefusal::VerdictNotOfThisBuild { .. }
            ))
        ),
        "A's verdict must not price B: {report:?}"
    );
    let entries = sensitivities(&b_doc, b_m, None, Some(&a_verdict), false, Tol::witness());
    assert!(
        matches!(
            entries,
            Err(SensitivityRefusal::VerdictNotOfThisBuild { .. })
        ),
        "A's verdict must not mark B: {entries:?}"
    );
}

// ------------------------------------- claim 5/deviation 4: the kinks

/// **PIN — deviation 4's argument, and the E9 arm the `abs` kink
/// actually exercises.** At `a = 0`
/// the tangent of `|a|` is finite (the ratified subgradient), so the
/// driver reports a `Derivative`, not `TangentDegraded` — the number is
/// a ONE-SIDED derivative reported with the same confidence a smooth
/// one gets. The `0/0` fixture the PR shipped exercises the loud arm;
/// this row records the quiet one, which is the state E4's honesty
/// clause is about.
#[test]
fn r1_the_abs_kink_reports_a_confident_one_sided_derivative() {
    let (doc, m) = scalar_measure(0.0, uniform(eps() / 16.0), |a: &dyn Fn() -> MeasureExpr| {
        MeasureExpr::max(a(), MeasureExpr::neg(a())).expect("Scalar lattice max")
    });
    let entries = sensitivities(&doc, m, None, None, false, Tol::witness()).expect("no refusal");
    match &entries[0].outcome {
        SensitivityOutcome::Derivative { value, .. } => {
            assert!(
                value.is_finite(),
                "DATUM: the abs kink is a finite, one-sided derivative: {value}"
            );
            assert_eq!(value.abs().to_bits(), 1.0f64.to_bits());
        }
        other => panic!("DATUM: the abs kink is not the degraded arm, it is {other:?}"),
    }
}

/// **DATUM — the `TangentDegraded` doc's unenforced premise.**
/// `SensitivityOutcome::TangentDegraded`'s own docs say "the measured
/// VALUE is finite (the value channel is fine)". Nothing checks it: a
/// measure whose VALUE is non-finite reaches the same arm, and the
/// report then carries a non-finite `nominal` beside a "the value
/// channel is fine" state.
#[test]
fn r1_tangent_degraded_does_not_check_that_the_value_is_finite() {
    // m = a / a at a = 0 → 0/0 in the VALUE channel as well.
    let (doc, m) = scalar_measure(0.0, uniform(eps() / 16.0), |a: &dyn Fn() -> MeasureExpr| {
        MeasureExpr::div(a(), a()).expect("Scalar / Scalar")
    });
    let entries = sensitivities(&doc, m, None, None, false, Tol::witness()).expect("no refusal");
    println!(
        "EVIDENCE-ONLY r1 0/0-value outcome: {:?}",
        entries[0].outcome
    );
    // The row records whichever arm it lands in; what it pins is that
    // the driver does not refuse (E9) — the honesty question is in the
    // printed arm.
    assert!(matches!(
        entries[0].outcome,
        SensitivityOutcome::Derivative { .. }
            | SensitivityOutcome::TangentDegraded { .. }
            | SensitivityOutcome::MeasureRefused { .. }
    ));
}

// ------------------------------------------ claim 6: worst_case honesty

/// **PIN — the hull beats the linearization by MY curvature case.**
/// `m = a³` over `a ∈ 2 ± 1`: the linearized band is `8 ± 12·1 =
/// [−4, 20]`; the true range is `[1, 27]`. The hull must contain
/// `[1, 27]` and must NOT be the linearized band — and, being an
/// enclosure, its top must exceed the linearized top.
#[test]
fn r1_worst_case_is_the_range_not_the_linearization_on_a_cubic() {
    let (doc, m) = scalar_measure(2.0, uniform(1.0), |a: &dyn Fn() -> MeasureExpr| {
        MeasureExpr::mul(MeasureExpr::mul(a(), a()).expect("scalar"), a()).expect("scalar")
    });
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(16), Tol::witness()).expect("builds");
    let report = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness())
        .expect("a certified arithmetic box");
    assert!((report.nominal - 8.0).abs() < 1e-12);
    let row = &report.per_param[0];
    match &row.sensitivity {
        SensitivityOutcome::Derivative { value, .. } => {
            assert!(
                (value - 12.0).abs() < 1e-12,
                "d(a³)/da at 2 = 12, got {value}"
            );
        }
        other => panic!("{other:?}"),
    }
    let contribution = row.contribution.clone().expect("available");
    assert!((contribution - 12.0).abs() < 1e-12);
    let wc = report.worst_case;
    // Encloses the true range, and is not the linearization.
    assert!(wc.lo <= 1.0 + 1e-9 && wc.hi >= 27.0 - 1e-9, "{wc:?}");
    assert!(
        wc.hi > report.nominal + contribution,
        "the hull top {} must exceed the linearized top {}",
        wc.hi,
        report.nominal + contribution
    );
    // The linearized band's BOTTOM is below the true minimum, which is
    // the direction that makes a linearized stackup unsafe: the hull is
    // the one that is right.
    assert!(
        report.nominal - contribution < wc.lo,
        "the linearized bottom {} understates the true minimum {}",
        report.nominal - contribution,
        wc.lo
    );
}

// -------------------------------------------- claim 7: the sigma doors

/// **PIN — every distribution form's σ, derived independently.**
/// Uniform: `w/√12`. Normal: `σ`. TruncatedNormal: checked against a
/// direct numerical second moment of the truncated density, computed
/// here by Simpson quadrature — an independent derivation, not the
/// implementation's formula restated. Symmetric and ASYMMETRIC windows
/// both, since the asymmetric case is where the shifted mean matters.
/// Band: refuses.
#[test]
fn r1_std_deviation_matches_an_independent_quadrature() {
    let p = name("p");
    // Uniform.
    let u = std_deviation(&p, &Distribution::Uniform { lo: -3.0, hi: 1.0 }).expect("uniform");
    assert!((u - 4.0 / f64::sqrt(12.0)).abs() < 1e-14, "uniform σ {u}");
    // Normal.
    let n = std_deviation(&p, &Distribution::Normal { sigma: 0.7 }).expect("normal");
    assert_eq!(n.to_bits(), 0.7f64.to_bits());
    // Band.
    assert!(std_deviation(&p, &Distribution::Band { lo: -1.0, hi: 1.0 }).is_err());
    // TruncatedNormal, against Simpson on the truncated density.
    for (sigma, lo, hi) in [
        (1.0_f64, -1.0_f64, 1.0_f64),
        (1.0, -3.0, 3.0),
        (0.5, -0.25, 1.5),
        (2.0, -1.0, 0.5),
        (1.0, -0.2, 0.2),
    ] {
        let got = std_deviation(&p, &Distribution::TruncatedNormal { sigma, lo, hi })
            .expect("truncated normal");
        // Simpson over [lo, hi] with 200_001 points on φ(x/σ)/σ.
        let n = 200_000_usize;
        let h = (hi - lo) / n as f64;
        let phi = |x: f64| {
            (-0.5 * (x / sigma).powi(2)).exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt())
        };
        let simpson = |f: &dyn Fn(f64) -> f64| {
            let mut s = f(lo) + f(hi);
            for i in 1..n {
                let x = lo + h * i as f64;
                s += f(x) * if i % 2 == 0 { 2.0 } else { 4.0 };
            }
            s * h / 3.0
        };
        let z = simpson(&phi);
        let mean = simpson(&|x| x * phi(x)) / z;
        let second = simpson(&|x| x * x * phi(x)) / z;
        let want = (second - mean * mean).max(0.0).sqrt();
        assert!(
            (got - want).abs() <= 1e-9 * want.max(1e-12),
            "TruncatedNormal(σ={sigma}, [{lo}, {hi}]): shipped {got}, quadrature {want}"
        );
    }
}

/// **PIN — RSS totality on MY document.** Both parameters uniform:
/// `σ_m = √((1·σ₁)² + (1·σ₂)²)`, derived from the shaft's own formula
/// `m = h1 + h2`. One band ⇒ the whole column refuses, naming BOTH the
/// band and nothing else; a fixed (undistributed) parameter contributes
/// a zero term rather than blocking — recorded as a DATUM, since the
/// spec's letter is "available only when every contributor carries a
/// measure".
#[test]
fn r1_rss_totality_and_the_fixed_parameter_door() {
    let half = eps() / 16.0;
    let (doc, m) = stepped_shaft(1.0, 0.5, Some(uniform(half)), Some(uniform(2.0 * half)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    let report = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness()).expect("ok");
    let s1 = (2.0 * half) / f64::sqrt(12.0);
    let s2 = (4.0 * half) / f64::sqrt(12.0);
    let want = (s1 * s1 + s2 * s2).sqrt();
    match report.rss {
        Rss::Advisory { sigma } => assert!(
            (sigma - want).abs() <= 1e-9 * want,
            "rss {sigma} vs independently derived {want}"
        ),
        other => panic!("{other:?}"),
    }

    // One band, one uniform: the column refuses whole and names exactly
    // the band.
    let (doc, m) = stepped_shaft(
        1.0,
        0.5,
        Some(uniform(half)),
        Some(Distribution::Band {
            lo: -half,
            hi: half,
        }),
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    let report = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness()).expect("ok");
    match &report.rss {
        Rss::UnavailableBecause { blockers } => {
            assert_eq!(blockers.len(), 1, "{blockers:?}");
            assert_eq!(
                blockers[0],
                Unavailable::BandHasNoMeasure { param: name("h2") }
            );
        }
        other => panic!("{other:?}"),
    }
    // And no partial RSS is representable: the enum has two arms.
    assert!(report.per_param.iter().all(|p| p.contribution.is_ok()));

    // DATUM: a parameter with NO distribution does not block the RSS;
    // it contributes a zero term.
    let (doc, m) = stepped_shaft(1.0, 0.5, Some(uniform(half)), None);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    let report = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness()).expect("ok");
    match report.rss {
        Rss::Advisory { sigma } => assert!(
            (sigma - s1).abs() <= 1e-9 * s1,
            "DATUM: an undistributed parameter is a point mass: {sigma} vs {s1}"
        ),
        other => panic!("DATUM changed: {other:?}"),
    }
}

// ------------------------------------- claim 8: the profile pin, again

/// **PIN — a profile shape the PR did not exercise: an ARC leg.** The
/// slab's x-walls are `w` apart and the profile carries a sharp `arc_to`
/// whose derived centre and radius are parameter-driven. Under the
/// guided lift the seed must reach the measure with tangent exactly 1;
/// under the pinned lift it is the silent zero.
#[test]
fn r1_an_arc_carrying_profile_propagates_the_seed() {
    let (doc, m) = arc_slab(2.0);
    let guided = measured(
        &evaluate(
            &doc,
            None,
            &CancelToken::new(),
            &opts(Some("w"), ProfileLift::Guided),
            Tol::witness(),
        ),
        m,
    );
    println!(
        "EVIDENCE-ONLY r1 arc slab: value {} deriv {}",
        guided.value, guided.deriv
    );
    assert!(guided.deriv.is_finite(), "an arc profile's seed is finite");
    let pinned = measured(
        &evaluate(
            &doc,
            None,
            &CancelToken::new(),
            &opts(Some("w"), ProfileLift::Pinned),
            Tol::witness(),
        ),
        m,
    );
    assert_eq!(
        pinned.deriv, 0.0,
        "the pinned lift is the silent zero, as documented"
    );
    assert!(
        guided.deriv != 0.0,
        "the guided lift must move an arc-carrying profile's measure"
    );
}

// -------------------------------------------- the e2e consumer read

/// **EVIDENCE-ONLY + DATUM — a real ±0.1 mm study on my own document.**
/// The stepped shaft with realistic tolerances (±0.1 on a 1.0 nominal)
/// is what a consumer actually wants. Records what they get, and
/// whether the refusal is legible.
#[test]
fn r1_a_real_tolerance_study_on_the_stepped_shaft() {
    let (doc, m) = stepped_shaft(1.0, 0.5, Some(uniform(0.1)), Some(uniform(0.1)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(512), Tol::witness()).expect("builds");
    println!(
        "EVIDENCE-ONLY r1 ±0.1 study: receipt {:?}; certified {}; refused {}",
        verdict.receipt(),
        verdict.certified().len(),
        verdict.refused().len()
    );
    let entries = sensitivities(&doc, m, None, Some(&verdict), false, Tol::witness()).expect("ok");
    println!("EVIDENCE-ONLY r1 ±0.1 sensitivities: {entries:?}");
    let got = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness());
    println!("EVIDENCE-ONLY r1 ±0.1 stackup: {got:?}");
    match got {
        Err(StackupRefusal::NothingCertified {
            sensitivities,
            coverage,
            ..
        }) => {
            // DATUM: today's answer to a real study — a refusal that
            // CARRIES the accounting it points at and the `LocalOnly`
            // sensitivities the driver computed (the fix pass's answer
            // to this row's original finding, which was that both were
            // thrown away).
            assert!(verdict.certified().is_empty());
            assert_eq!(&*coverage, verdict.accounting());
            assert_eq!(sensitivities.len(), 2);
        }
        other => panic!("DATUM changed — a ±0.1 study now yields {other:?}"),
    }
}

/// **PIN — the seed door refuses a name from another document's
/// parameter set, at the door.** `seed_env` is public; a caller pairing
/// one document's env with another document's name gets a typed refusal
/// rather than a wrong tangent.
#[test]
fn r1_seed_env_refuses_a_foreign_name() {
    let (a, _) = stepped_shaft(1.0, 0.5, None, None);
    assert!(
        seed_env::<Dual64, _>(&a, a.param_env::<Dual64>(), &name("nope")).is_err(),
        "an unknown name refuses"
    );
    // And the bindings it does produce carry exactly one unit tangent.
    let env = seed_env::<Dual64, _>(&a, a.param_env::<Dual64>(), &name("h1")).expect("h1");
    let mut ones = 0_usize;
    let mut zeros = 0_usize;
    for v in env.bindings.values() {
        if let ParamValue::Continuous { value, .. } = v {
            if value.deriv.to_bits() == 1.0f64.to_bits() {
                ones += 1;
            } else if value.deriv.to_bits() == 0.0f64.to_bits() {
                zeros += 1;
            } else {
                panic!("a lift carries neither 1 nor 0: {value:?}");
            }
        }
    }
    assert_eq!(ones, 1, "exactly one seeded lift");
    assert_eq!(zeros, 1, "every other lift is exactly zero");
}

/// **DATUM — the `contribution` column extrapolates past its own
/// chamber.** `Chamber::ChamberCertified` names ONE certified LEAF, and
/// the drive splits the analyzed box into many; `contribution` is
/// `|dm/dp| * (the ANALYZED box\'s half-width)`. So the report
/// multiplies a derivative marked valid over a leaf by a span many
/// times the leaf\'s — the extrapolation E4\'s marking clause exists to
/// make unwritable. This row measures the ratio on a drive that split.
#[test]
fn r1_the_contribution_extrapolates_past_its_certified_chamber() {
    let half = eps() / 8.0;
    let (doc, m) = stepped_shaft(1.0, 0.5, Some(uniform(half)), Some(uniform(half)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    assert!(
        verdict.certified().len() > 1,
        "this row needs a drive that SPLIT: {:?}",
        verdict.receipt()
    );
    let report = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness()).expect("ok");
    let row = &report.per_param[0];
    let Chamber::ChamberCertified { leaf, .. } = (match &row.sensitivity {
        SensitivityOutcome::Derivative { chamber, .. } => chamber.clone(),
        other => panic!("{other:?}"),
    }) else {
        panic!("the nominal\'s leaf certifies here")
    };
    let (lo, hi) = leaf.get(&row.param).expect("the axis").span();
    let leaf_half = 0.5 * (hi - lo);
    let box_half = 0.5 * analyzed.get(&row.param).expect("the axis").offsets.width();
    println!(
        "EVIDENCE-ONLY r1 chamber vs contribution: leaf half-width {leaf_half:e}, \
         analyzed half-width {box_half:e}, ratio {:.1}x; contribution {:?}",
        box_half / leaf_half,
        row.contribution
    );
    assert!(
        box_half > leaf_half,
        "DATUM: the contribution\'s span exceeds the certified chamber\'s"
    );
}
