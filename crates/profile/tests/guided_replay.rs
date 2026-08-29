//! **Guided elaboration**: consuming a structure record instead of
//! remaking the decisions in it.
//!
//! Four things are pinned here, and the third is the reason the
//! machinery exists at all.
//!
//! 1. **The fence.** Guided elaboration at `f64` reproduces plain
//!    elaboration bit for bit, over the whole verb-coverage corpus.
//!    Consuming a record is not a different computation.
//! 2. **Consumption is real.** Hand a guided pass a record naming the
//!    OTHER fillet pocket and it builds the other pocket. A pass that
//!    quietly re-ranked would produce the first pocket again and this
//!    row would fail — which is what makes the claim testable rather
//!    than a reading of the source.
//! 3. **No lane re-picks.** The hairline-asymmetric lens is the planted
//!    probe: `fillet_select`'s own docs say two lanes may legally rank
//!    its two survivors differently, so it is precisely the shape where
//!    a lane that re-decided would silently build a different solid.
//!    Guided at `Interval`, it verifies or it aborts TYPED, naming the
//!    decision — it never arrives at a pick of its own.
//! 4. **Canonicalization is absent, not lucky.** A guided validation at
//!    `Interval` runs no `canonical_order_*` and no `loop_orientation`
//!    decide at all, and the verdict log is the receipt.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{annulus, coverage_corpus, lift, p2, profile, rect, rounded_rect, tol};
use geom_core::{Sign, Tol};
use profile::{
    ArcSweep, Center, Decision, DecisionValue, Open, PathError, Profile, ProfileLoop,
    ReplayErrorKind, ReplayStructure, StructureRefusalKind, replay, replay_guided,
    replay_recording,
};

/// √3 — the vesica tip's height.
fn s3() -> f64 {
    3.0_f64.sqrt()
}

/// The vesica of the two radius-2 circles about (±1, 0), authored as ONE
/// fused act, with the incoming lobe's centre displaced by `dx`.
///
/// At `dx = 0` the configuration is mirror-symmetric; an ulp of `dx`
/// moves the DERIVED corner off the mirror axis by the same order,
/// which is the hairline-asymmetric lens: a strict but tiny setback gap
/// between two valid fillets.
fn vesica_lens(dx: f64) -> Vec<profile::Step<f64>> {
    Open.arc_fillet_arc(
        Center {
            c: p2(-1.0 + dx, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -s3()),
        },
        0.5,
        Center {
            c: p2(1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: profile::Start,
        },
        Tol::witness(),
    )
    .expect("the lens constructs")
    .program
}

fn same_bits(a: &ProfileLoop<f64>, b: &ProfileLoop<f64>, what: &str) {
    assert_eq!(a.vertices().len(), b.vertices().len(), "{what}: arity");
    for (i, (u, v)) in a.vertices().iter().zip(b.vertices()).enumerate() {
        assert_eq!(
            u.pos().x.to_bits(),
            v.pos().x.to_bits(),
            "{what} vertex {i} x"
        );
        assert_eq!(
            u.pos().y.to_bits(),
            v.pos().y.to_bits(),
            "{what} vertex {i} y"
        );
        assert_eq!(
            u.bulge().to_bits(),
            v.bulge().to_bits(),
            "{what} vertex {i} b"
        );
    }
    assert_eq!(a.tangent_joints(), b.tangent_joints(), "{what}: joints");
}

// ------------------------------------------------------------------
// 1. The fence
// ------------------------------------------------------------------

/// Guided replay at `f64` IS plain replay at `f64`, over every chain
/// the verb-coverage corpus authors.
#[test]
fn guided_replay_at_f64_reproduces_plain_replay_bitwise() {
    for (i, closed) in coverage_corpus().into_iter().enumerate() {
        let plain = replay(&closed.program, tol()).expect("the corpus replays");
        let (recorded, structure) =
            replay_recording(&closed.program, tol()).expect("and records while it does");
        same_bits(&plain, &recorded, &format!("row {i}: recording"));
        // The chain's OWN record — written as it lowered — must be the
        // one a replay of its program rebuilds.
        assert_eq!(
            structure, closed.structure,
            "row {i}: the lowering's record and the replay's must agree"
        );
        let guided = replay_guided(&closed.program, &structure, tol())
            .expect("and the record it just wrote guides it");
        same_bits(&plain, &guided, &format!("row {i}: guided"));
    }
}

/// Guided validation at `f64` IS plain validation at `f64`: same
/// canonical form, same roles, same joints.
#[test]
fn guided_validation_at_f64_reproduces_plain_validation() {
    for (name, p) in [
        ("annulus", annulus()),
        ("rect", profile(vec![rect(0.0, 0.0, 3.0, 2.0)])),
        ("rounded", profile(vec![rounded_rect(4.0, 3.0, 0.5)])),
    ] {
        let plain = p.validate(tol()).expect("validates");
        let (recorded, canonical) = p.validate_recording(tol()).expect("and records");
        let guided = p.validate_guided(tol(), &canonical).expect("and is guided");
        for (li, ((a, b), c)) in plain
            .loops()
            .iter()
            .zip(recorded.loops())
            .zip(guided.loops())
            .enumerate()
        {
            assert_eq!(a.role(), b.role(), "{name} loop {li}: recorded role");
            assert_eq!(a.role(), c.role(), "{name} loop {li}: guided role");
            for (k, ((u, v), w)) in a
                .vertices()
                .iter()
                .zip(b.vertices())
                .zip(c.vertices())
                .enumerate()
            {
                for (which, got) in [("recorded", v), ("guided", w)] {
                    assert_eq!(
                        u.pos().x.to_bits(),
                        got.pos().x.to_bits(),
                        "{name} loop {li} vertex {k}: {which} x"
                    );
                    assert_eq!(
                        u.pos().y.to_bits(),
                        got.pos().y.to_bits(),
                        "{name} loop {li} vertex {k}: {which} y"
                    );
                }
            }
            assert_eq!(a.tangent_joints(), c.tangent_joints(), "{name} loop {li}");
        }
    }
}

// ------------------------------------------------------------------
// 2. Consumption is real
// ------------------------------------------------------------------

/// The record's index is USED, not re-derived.
///
/// The lens's surviving corner offers two valid fillets. Recording it
/// names one; editing the record to name the other and replaying
/// guided builds the OTHER — which a pass that re-ran the selection
/// ladder could not do, since the ladder's answer does not depend on
/// what it is told.
#[test]
fn guided_replay_consumes_the_recorded_pick_rather_than_ranking() {
    let program = vesica_lens(0.0);
    let (nominal, structure) = replay_recording(&program, tol()).expect("the lens replays");
    assert_eq!(structure.fillets.len(), 1, "one fused resolution");
    let d = &structure.fillets[0];
    assert_eq!(
        d.survivors, 2,
        "the lens is the two-survivor configuration this row is about"
    );
    let other = ReplayStructure {
        fillets: vec![profile::FilletDecision {
            candidate: 1 - d.candidate,
            ..d.clone()
        }],
    };
    let flipped = replay_guided(&program, &other, tol())
        .expect("the other pocket is a valid fillet of the same legs");
    // Same arity, different geometry: the pick moved because the record
    // moved.
    assert_eq!(nominal.vertices().len(), flipped.vertices().len());
    let moved = nominal
        .vertices()
        .iter()
        .zip(flipped.vertices())
        .any(|(a, b)| a.pos().y.to_bits() != b.pos().y.to_bits());
    assert!(
        moved,
        "the guided pass produced the SAME pocket after being told the other one — \
         it is ranking rather than consuming, which is the whole hazard this \
         machinery exists to foreclose"
    );
}

// ------------------------------------------------------------------
// 3. No lane re-picks
// ------------------------------------------------------------------

/// **The planted probe.** The hairline lens at `Interval`, guided:
/// verify-or-abort, never a pick of its own.
///
/// The ulp of asymmetry puts the two survivors' setback gap inside the
/// interval channel's enclosure width — the configuration whose two
/// lanes `fillet_select` says may legally disagree. Guided, the lane
/// gets no chance to disagree: it re-runs the corner gates, cannot
/// classify one of them, and refuses TYPED naming that gate. No index
/// is chosen at `Interval` anywhere in this path.
#[cfg(feature = "interval")]
#[test]
fn the_hairline_lens_aborts_typed_at_interval_instead_of_re_picking() {
    use geom_core::{Interval, Real};
    /// Lifts one `f64` step to another scalar (the suite-local embedding;
    /// `generic_replay.rs` carries the exhaustive one and the census
    /// argument for it).
    fn embed<T: geom_core::Real>(step: &profile::Step<f64>) -> profile::Step<T> {
        use geom_core::Point2;
        use profile::{ArcData, Step, Target};
        let pt = |p: Point2<f64>| Point2::new(T::from_f64(p.x), T::from_f64(p.y));
        let tgt = |t: Target<f64>| match t {
            Target::Start => Target::Start,
            Target::Point(p) => Target::Point(pt(p)),
        };
        let spec = |s: ArcData<f64>| match s {
            ArcData::Center { c, winding, target } => ArcData::Center {
                c: pt(c),
                winding,
                target: tgt(target),
            },
            _ => panic!("this suite's fixtures author Center-mode arcs only"),
        };
        match *step {
            Step::ArcFilletArc {
                spec: s,
                radius,
                spec2,
            } => Step::ArcFilletArc {
                spec: spec(s),
                radius: T::from_f64(radius),
                spec2: spec(spec2),
            },
            ref other => panic!("this suite's fixtures are one fused step, got {other:?}"),
        }
    }

    let program = vesica_lens(f64::EPSILON);
    let (_, structure) = replay_recording(&program, tol()).expect("the lens replays at f64");
    let lifted: Vec<profile::Step<Interval>> = program
        .iter()
        .map(embed)
        .collect::<Vec<profile::Step<Interval>>>();
    let err = replay_guided(&lifted, &structure, tol())
        .expect_err("the interval lane cannot confirm this structure");
    let ReplayErrorKind::Path(PathError::Structure(refusal)) = err.kind else {
        panic!("expected a named structure refusal, got {:?}", err.kind);
    };
    assert!(
        matches!(refusal.decision, Decision::CornerGate { .. }),
        "the refusal must name the decision that went unconfirmed, got {:?}",
        refusal.decision
    );
    match refusal.kind {
        StructureRefusalKind::Indeterminate(source) => assert_eq!(
            source.predicate,
            Some("path_corner_advance_arc"),
            "and the predicate it could not classify"
        ),
        other => panic!("expected the indeterminate arm, got {other:?}"),
    }
    // Nothing was built: no pocket, no partial loop, no nominal
    // structure quietly kept.
    let _ = Interval::from_f64(0.0);
}

/// A record whose fit sign disagrees with what this scalar classifies
/// refuses TYPED, naming the fit that moved — it does not adopt either
/// answer.
#[test]
fn a_flipped_fit_sign_refuses_typed_naming_it() {
    let program = vesica_lens(0.0);
    let (_, structure) = replay_recording(&program, tol()).expect("the lens replays");
    let d = &structure.fillets[0];
    let lie = ReplayStructure {
        fillets: vec![profile::FilletDecision {
            fit_in: match d.fit_in {
                Sign::Positive => Sign::Zero,
                _ => Sign::Positive,
            },
            ..d.clone()
        }],
    };
    let err = replay_guided(&program, &lie, tol()).expect_err("the fit sign is contradicted");
    let ReplayErrorKind::Path(PathError::Structure(refusal)) = err.kind else {
        panic!("expected a structure refusal, got {:?}", err.kind);
    };
    assert!(
        matches!(refusal.decision, Decision::FitIn { fillet: 0 }),
        "got {:?}",
        refusal.decision
    );
    assert!(matches!(
        refusal.kind,
        StructureRefusalKind::Flipped {
            recorded: DecisionValue::Sign(_),
            found: DecisionValue::Sign(_),
        }
    ));
}

/// A record from a DIFFERENT program is refused at its own shape: no
/// per-decision disagreement is invented for a mismatch that is not
/// one.
#[test]
fn a_record_from_another_program_refuses_at_its_shape() {
    let program = vesica_lens(0.0);
    let err = replay_guided(&program, &ReplayStructure::default(), tol())
        .expect_err("an empty record does not describe a program with a fillet in it");
    let ReplayErrorKind::Path(PathError::Structure(refusal)) = err.kind else {
        panic!("expected a structure refusal, got {:?}", err.kind);
    };
    assert_eq!(refusal.decision, Decision::RecordShape);
}

// ------------------------------------------------------------------
// 4. Canonicalization is absent, not lucky
// ------------------------------------------------------------------

/// The receipt for the pinned permutation: a guided validation runs
/// ZERO `canonical_order_x`, `canonical_order_y` and `loop_orientation`
/// decides, at `f64` and at `Interval` alike, while the unguided one it
/// is compared against runs them. Structurally absent — not agreeing by
/// luck on inputs that happen to be easy.
#[test]
fn guided_validation_runs_no_canonicalization_decide() {
    use geom_core::k_stats::{start_verdict_log, take_verdict_log};
    const PINNED: [&str; 3] = ["canonical_order_x", "canonical_order_y", "loop_orientation"];
    // A rectangle, so that the control genuinely reaches all three: two
    // of its vertices share an x, which is the only way the y rung of
    // the lexicographic order is ever asked.
    let p = profile(vec![rect(0.0, 0.0, 3.0, 2.0)]);
    let (_, canonical) = p.validate_recording(tol()).expect("records");

    start_verdict_log();
    let _ = p.validate(tol()).expect("validates");
    let plain = take_verdict_log();
    let ran: Vec<&str> = PINNED
        .into_iter()
        .filter(|n| plain.iter().any(|v| v.predicate == *n))
        .collect();
    assert_eq!(
        ran.len(),
        PINNED.len(),
        "the unguided validation is the control and must run all three; it ran {ran:?}"
    );

    start_verdict_log();
    let _ = p.validate_guided(tol(), &canonical).expect("is guided");
    let guided = take_verdict_log();
    let leaked: Vec<&'static str> = guided
        .iter()
        .map(|v| v.predicate)
        .filter(|n| PINNED.contains(n))
        .collect();
    assert!(
        leaked.is_empty(),
        "a guided validation reached the canonicalization predicates {leaked:?} — \
         the permutation is supposed to be consumed, and a lane scalar cannot \
         answer these questions"
    );
    // It is still validating: the value-channel predicates all ran.
    assert!(
        guided.len() > 4,
        "a guided validation that decided almost nothing is not verifying anything"
    );
}

/// The same receipt at `Interval`, where it is load-bearing: this
/// profile's guided validation SUCCEEDS at a scalar whose `lex_min`
/// comparisons would have to be asked of overlapping enclosures.
#[cfg(feature = "interval")]
#[test]
fn guided_validation_at_interval_certifies_without_the_pinned_decides() {
    use geom_core::Interval;
    use geom_core::k_stats::{start_verdict_log, take_verdict_log};
    let p = annulus();
    let (_, canonical) = p.validate_recording(tol()).expect("records at f64");
    let lifted: Profile<Interval> = lift(&p);
    start_verdict_log();
    let vp = lifted
        .validate_guided(tol(), &canonical)
        .expect("the interval lane certifies the pinned canonical form");
    let log = take_verdict_log();
    for name in ["canonical_order_x", "canonical_order_y", "loop_orientation"] {
        assert!(
            !log.iter().any(|v| v.predicate == name),
            "the interval lane reached {name}, which it is not supposed to be asked"
        );
    }
    assert_eq!(vp.loops().len(), 2);
    assert_eq!(vp.loops()[0].role(), profile::LoopRole::Outer);
    assert_eq!(vp.loops()[1].role(), profile::LoopRole::Hole);
}
