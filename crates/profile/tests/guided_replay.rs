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

use crate::common;

use common::{annulus, coverage_corpus, p2, profile, rect, rounded_rect, tol};
use geom_core::{Sign, Tol};
use profile::{
    ArcSweep, Center, Decision, DecisionValue, Open, PathError, ProfileLoop, ReplayErrorKind,
    ReplayStructure, StructureRefusalKind, replay, replay_guided, replay_recording,
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
        assert_eq!(u.x.to_bits(), v.x.to_bits(), "{what} vertex {i} x");
        assert_eq!(u.y.to_bits(), v.y.to_bits(), "{what} vertex {i} y");
        assert_eq!(
            a.bulges()[i].to_bits(),
            b.bulges()[i].to_bits(),
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
                        u.x.to_bits(),
                        got.x.to_bits(),
                        "{name} loop {li} vertex {k}: {which} x"
                    );
                    assert_eq!(
                        u.y.to_bits(),
                        got.y.to_bits(),
                        "{name} loop {li} vertex {k}: {which} y"
                    );
                }
            }
            assert_eq!(a.tangent_joints(), c.tangent_joints(), "{name} loop {li}");
        }
    }
}

/// **Every entry verb installs the guide** — the census that keeps the
/// per-arm convention honest.
///
/// The guide reaches a chain through its core, and only an ENTRY row
/// mints a core, so exactly the entry rows install it. That is five
/// hand-written `adopt(guide())` calls today, and a sixth row added
/// later that mints a core and forgets one would not fail loudly: it
/// would elaborate under a fresh RECORDING guide, selecting structure
/// freely while its caller believed it was guided. The driver checks
/// that invariant directly — a guide no row took is still `Guided` in
/// its hand after step 0 — and this row is what runs the check across
/// the whole entry vocabulary instead of on whichever verb a fixture
/// happened to start with.
///
/// Every row here guides against ITS OWN recorded structure, so the
/// expected outcome is success; a forgotten install turns that into a
/// `GuideNotInstalled` refusal, which is the failure this census is
/// for. `Angle` and `Toward` get authored chains because no corpus
/// program starts with either.
#[test]
fn every_entry_verb_installs_the_guide() {
    use profile::{Step, Verb};
    use std::f64::consts::FRAC_PI_2;

    // The angle slot is pinned so `line` resolves: the verb has a second
    // row (the straight continuation, off a directed point with NO bound
    // angle), and an unannotated tip leaves both rows applicable.
    let tail_from_directed = |chain: profile::PartialPath<f64, _, profile::path::HasAng>| {
        chain
            .line(3.0, Tol::witness())
            .expect("line")
            .turn(FRAC_PI_2, Tol::witness())
            .expect("turn")
            .line(3.0, Tol::witness())
            .expect("line")
            .turn(FRAC_PI_2, Tol::witness())
            .expect("turn")
            .line(3.0, Tol::witness())
            .expect("line")
            .line_to(profile::Start, Tol::witness())
            .expect("close")
            .program
    };
    let angle_first = tail_from_directed(
        Open.angle(0.0)
            .at(p2(0.0, 0.0), Tol::witness())
            .expect("Angle then At binds"),
    );
    let toward_first = tail_from_directed(
        Open.toward(1.0, 0.0, Tol::witness())
            .expect("Toward binds at entry")
            .at(p2(0.0, 0.0), Tol::witness())
            .expect("then At"),
    );

    let mut seen: Vec<Verb> = Vec::new();
    let mut rows: Vec<Vec<Step<f64>>> = vec![angle_first, toward_first];
    rows.extend(coverage_corpus().into_iter().map(|c| c.program));
    for (i, program) in rows.iter().enumerate() {
        let entry = program
            .first()
            .map(Step::verb)
            .expect("a program has steps");
        seen.push(entry);
        let (_, record) = replay_recording(program, tol()).expect("row records at f64");
        replay_guided(program, &record, tol()).unwrap_or_else(|e| {
            panic!(
                "row {i} (entry verb {entry:?}) failed its own record: {e} — a \
                 `GuideNotInstalled` here means that entry row minted a core without \
                 installing the guide"
            )
        });
    }
    // The census half: every verb an entry row can bind is represented.
    for want in [
        Verb::At,
        Verb::Angle,
        Verb::Toward,
        Verb::ArcFillet,
        Verb::ArcFilletArc,
        Verb::Circle,
        Verb::CircleSplit,
    ] {
        assert!(
            seen.contains(&want),
            "no row starts with {want:?}, so its entry arm's install is unexercised — \
             add a chain that begins with it rather than letting the arm go uncovered"
        );
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
        ..structure.clone()
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
        .any(|(a, b)| a.y.to_bits() != b.y.to_bits());
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

/// **The planted probe.** The hairline lens at `Interval`, guided: the
/// record's index is USED, not re-derived — the `f64` row above's claim,
/// made on the enclosure lane where re-deriving would be the tempting
/// thing to do.
///
/// The ulp of asymmetry puts the two survivors' setback gap inside the
/// interval channel's enclosure width — the configuration whose two
/// lanes `fillet_select` says may legally disagree, so the ladder here
/// genuinely has no answer of its own to fall back on. Told the other
/// index, the lane answers differently; a pass that re-ran the ladder
/// could not, since the ladder's answer does not depend on what it is
/// told.
///
/// **What "differently" is allowed to be.** Usually the other pocket,
/// built and separated from this one. But the other pocket is a fillet
/// like any other, and the path door now reads every fillet it emits
/// the way `Profile::validate` will: at a tight epsilon on the
/// enclosure lane the other pocket's joint clearance is an enclosure
/// straddling zero and wider than the band, so the door escalates it —
/// and so does validation, on the very loop this row used to build
/// (`carrier_circles_internal`, enclosure ±2.4e-12 against a band of
/// (1e-12, 1e-11)). A refusal there is therefore not a lost row: it is
/// the consumption, stated more sharply than geometry can state it.
/// The ladder's own answer is the recorded one, which builds; a pass
/// that ignored the record would have built it whatever it was told,
/// so an outcome that differs from the recorded one AT ALL is the
/// claim.
///
/// This is the row a two-survivor ranking at `Interval` was waiting on.
/// It could not be written while the advance gate's zero swept angle
/// straddled a composed period fold: a lens' two carriers cross at both
/// tips, so the entry anchor is itself a derived corner, and the gate
/// that had to classify it saw a whole-period enclosure and escalated —
/// for the symmetric lens (`dx = 0`) as much as for this one, which is
/// why the abort was never evidence about the asymmetry. With the
/// signed sweep folding its raw difference once
/// ([`geom_core::Real::reduce_periodic_centred`]) the gate classifies,
/// the ladder is reached, and consumption is observable here.
#[test]
fn the_hairline_lens_at_interval_consumes_the_recorded_pick() {
    use geom_core::{Bounds, Interval, Real};

    let program = vesica_lens(f64::EPSILON);
    let (_, structure) = replay_recording(&program, tol()).expect("the lens replays at f64");
    let lifted: Vec<profile::Step<Interval>> = program
        .iter()
        .map(|s| s.map_scalar(Interval::from_f64))
        .collect();
    let d = &structure.fillets[0];
    assert_eq!(
        d.survivors, 2,
        "the lens is the two-survivor configuration this row is about"
    );
    let nominal = replay_guided(&lifted, &structure, tol())
        .expect("the interval lane confirms the recorded structure");
    let other = ReplayStructure {
        fillets: vec![profile::FilletDecision {
            candidate: 1 - d.candidate,
            ..d.clone()
        }],
        ..structure.clone()
    };
    match replay_guided(&lifted, &other, tol()) {
        // The other pocket built: same arity, and the two are
        // SEPARATED — not merely different bits, which an enclosure
        // lane cannot honestly claim: some vertex's y enclosures are
        // disjoint, so no single geometry lies in both answers and the
        // pick provably moved with the record.
        Ok(flipped) => {
            assert_eq!(nominal.vertices().len(), flipped.vertices().len());
            let moved = nominal
                .vertices()
                .iter()
                .zip(flipped.vertices())
                .any(|(a, b)| a.y.hi() < b.y.lo() || b.y.hi() < a.y.lo());
            assert!(
                moved,
                "the guided pass produced an overlapping pocket after being told the other \
                 one — it is ranking rather than consuming, which is the whole hazard this \
                 machinery exists to foreclose"
            );
        }
        // The other pocket is one this run's tolerance cannot certify.
        // The pass still CONSUMED the record: told the recorded index it
        // built, told the other it refused, and the refusal is about
        // that other pocket's own geometry — a typed authoring refusal,
        // not a lattice violation.
        Err(refused) => {
            let ReplayErrorKind::Path(profile::PathError::Escalated { source }) = &refused.kind
            else {
                panic!(
                    "told the other index the lane refused, which is consumption — but the \
                     refusal must be the door relaying a stored-form classification, not \
                     any other refusal and not a lattice violation: {refused:?}"
                );
            };
            assert_eq!(
                source.predicate,
                Some("carrier_circles_internal"),
                "the other pocket's joint is an arc/arc clearance the enclosure lane cannot \
                 certify at this eps; another predicate here is a different finding: \
                 {refused:?}"
            );
        }
    }
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
        ..structure.clone()
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

/// **A record whose per-step segment span this pass did not reproduce
/// refuses TYPED, naming the step** — the span is consumed the way a
/// fillet decision is, not carried along unread.
///
/// The span a lane could actually move is the one a fit gate decides
/// (an exact fit emits the arc alone where an overrun emits a straight
/// leg before it), and under guidance the fit signs come from the
/// record, so the elaboration cannot drift there on its own. What this
/// row pins is that the comparison HAPPENS: a record claiming a step
/// reached one more segment than it did is refused rather than
/// accepted, which is the state in which a future arm could emit a
/// different chain under a record that says otherwise.
#[test]
fn a_lying_step_span_refuses_typed_naming_the_step() {
    let program = vesica_lens(0.0);
    let (_, structure) = replay_recording(&program, tol()).expect("the lens replays");
    let step = structure
        .steps
        .iter()
        .position(|s| !s.is_empty())
        .expect("some step of the lens produced a segment");
    let mut lie = structure.clone();
    lie.steps[step] = profile::StepSpan::new(
        structure.steps[step].start(),
        structure.steps[step].end() + 1,
    );
    let err = replay_guided(&program, &lie, tol()).expect_err("the span is contradicted");
    let ReplayErrorKind::Path(PathError::Structure(refusal)) = err.kind else {
        panic!("expected a structure refusal, got {:?}", err.kind);
    };
    assert_eq!(refusal.decision, Decision::StepSpan { step });
    assert!(matches!(
        refusal.kind,
        StructureRefusalKind::Flipped {
            recorded: DecisionValue::Span(_),
            found: DecisionValue::Span(_),
        }
    ));
    // Both sides reach the sentence as words, through the span's own
    // `Display` — the vocabulary rule the refusal payloads all follow.
    let rendered = refusal.to_string();
    for want in [
        format!("the segments step {step} produced"),
        structure.steps[step].to_string(),
        lie.steps[step].to_string(),
    ] {
        assert!(rendered.contains(&want), "{rendered:?} is missing {want:?}");
    }
    assert!(
        !rendered.contains("StepSpan"),
        "the Debug spelling leaked into the sentence: {rendered}"
    );
}

/// **A lying span on a `circle_split` record refuses typed, naming the
/// step** — the CARRIER case, which is why the comparison sits at
/// `replay_guided` rather than inside the guide.
///
/// The row above is authored on a chain, and a chain reaches the guide.
/// A complete-loop carrier form never takes a guide at all: it mints
/// its `ClosedLoop` and its `ReplayStructure::carrier(n)` directly, so
/// a record and an elaboration that disagreed about the subdivision
/// count would pass unread if the check lived one level in.
#[test]
fn a_lying_step_span_on_a_carrier_form_refuses_typed() {
    let program = vec![profile::Step::CircleSplit {
        centre: p2(0.0, 0.0),
        radius: 1.0,
        n: 4,
        phase: 0.0,
    }];
    let (loop_, structure) = replay_recording(&program, tol()).expect("the carrier replays");
    assert_eq!(structure.steps.len(), 1, "one authored step");
    assert_eq!(
        structure.steps[0],
        profile::StepSpan::new(0, loop_.vertices().len()),
        "the carrier's one step reaches every segment"
    );
    let mut lie = structure.clone();
    lie.steps[0] = profile::StepSpan::new(0, loop_.vertices().len() - 1);
    let err = replay_guided(&program, &lie, tol()).expect_err("the span is contradicted");
    let ReplayErrorKind::Path(PathError::Structure(refusal)) = err.kind else {
        panic!("expected a structure refusal, got {:?}", err.kind);
    };
    assert_eq!(refusal.decision, Decision::StepSpan { step: 0 });
    assert!(matches!(
        refusal.kind,
        StructureRefusalKind::Flipped {
            recorded: DecisionValue::Span(_),
            found: DecisionValue::Span(_),
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
    use geom_core::k_stats::Bracket;
    const PINNED: [&str; 3] = ["canonical_order_x", "canonical_order_y", "loop_orientation"];
    // A rectangle, so that the control genuinely reaches all three: two
    // of its vertices share an x, which is the only way the y rung of
    // the lexicographic order is ever asked.
    let p = profile(vec![rect(0.0, 0.0, 3.0, 2.0)]);
    let (_, canonical) = p.validate_recording(tol()).expect("records");

    let bracket = Bracket::open();
    let _ = p.validate(tol()).expect("validates");
    let plain = bracket.finish().verdicts;
    let ran: Vec<&str> = PINNED
        .into_iter()
        .filter(|n| plain.iter().any(|v| v.predicate == *n))
        .collect();
    assert_eq!(
        ran.len(),
        PINNED.len(),
        "the unguided validation is the control and must run all three; it ran {ran:?}"
    );

    let bracket = Bracket::open();
    let _ = p.validate_guided(tol(), &canonical).expect("is guided");
    let recorded = bracket.finish();
    let guided = recorded.verdicts;
    // Both channels: a pinned predicate that ESCALATED rather than
    // decided would still have been asked, and asking is the leak.
    let leaked: Vec<&'static str> = guided
        .iter()
        .map(|v| v.predicate)
        .chain(recorded.escalations.iter().map(|e| e.predicate()))
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
#[test]
fn guided_validation_at_interval_certifies_without_the_pinned_decides() {
    // Used ONLY by this row, so imported here rather than at module
    // scope.
    use common::lift;
    use geom_core::Interval;
    use geom_core::k_stats::Bracket;
    use profile::Profile;
    let p = annulus();
    let (_, canonical) = p.validate_recording(tol()).expect("records at f64");
    let lifted: Profile<Interval> = lift(&p);
    let bracket = Bracket::open();
    let vp = lifted
        .validate_guided(tol(), &canonical)
        .expect("the interval lane certifies the pinned canonical form");
    let recorded = bracket.finish();
    let log = recorded.verdicts;
    for name in ["canonical_order_x", "canonical_order_y", "loop_orientation"] {
        assert!(
            !log.iter().any(|v| v.predicate == name)
                && !recorded.escalations.iter().any(|e| e.predicate() == name),
            "the interval lane reached {name}, which it is not supposed to be asked"
        );
    }
    assert_eq!(vp.loops().len(), 2);
    assert_eq!(vp.loops()[0].role(), profile::LoopRole::Outer);
    assert_eq!(vp.loops()[1].role(), profile::LoopRole::Hole);
}

/// A structure refusal reports both sides of a disagreement in PROSE.
///
/// Every payload that has a vocabulary reaches the sentence through
/// that vocabulary's `Display`; none reaches it through `Debug`. The
/// assertions are on exact renderings, so reverting any arm to `{:?}`
/// fails this test rather than passing unnoticed — a fieldless
/// variant's `Debug` spelling is the type's identifier, and putting an
/// identifier where a word belongs is the defect being guarded.
#[test]
fn structure_refusal_renders_its_payloads_as_words_not_debug() {
    use profile::{CornerGate, LoopRole, RadiusRole, SegmentShape, StructureRefusal};

    // Each converted vocabulary, rendered on its own. The paired
    // `Debug` spelling is asserted absent: it is what a reverted arm
    // would emit, and it differs from the prose in case alone for
    // `LoopRole`, which a substring check would miss.
    let rows: [(String, &str, &str); 9] = [
        (CornerGate::Admitted.to_string(), "admitted", "Admitted"),
        (RadiusRole::Fillet.to_string(), "fillet", "Fillet"),
        (
            RadiusRole::Carrier.to_string(),
            "incoming carrier",
            "Carrier",
        ),
        (
            RadiusRole::Carrier2.to_string(),
            "arrival carrier",
            "Carrier2",
        ),
        (
            CornerGate::RefusedAdvance.to_string(),
            "refused (corner not ahead of the incoming anchor)",
            "RefusedAdvance",
        ),
        (
            CornerGate::RefusedReach.to_string(),
            "refused (corner not behind the arrival anchor)",
            "RefusedReach",
        ),
        (LoopRole::Outer.to_string(), "outer", "Outer"),
        (LoopRole::Hole.to_string(), "hole", "Hole"),
        (SegmentShape::Line.to_string(), "a line", "Line"),
    ];
    for (rendered, prose, debug_spelling) in rows {
        assert_eq!(rendered, prose);
        assert!(
            !rendered.contains(debug_spelling),
            "{rendered:?} carries the Debug spelling {debug_spelling:?}"
        );
    }

    // The arc carries its turn through `Sign`'s own words.
    assert_eq!(
        SegmentShape::Arc {
            turn: Sign::Positive
        }
        .to_string(),
        "an arc turning positive"
    );

    // The composed sentence: a `Flipped` refusal names both sides, and
    // each side is a `DecisionValue` arm rendering through the above.
    let refusal = StructureRefusal {
        decision: Decision::Role { loop_: 1 },
        kind: StructureRefusalKind::Flipped {
            recorded: DecisionValue::Role(LoopRole::Outer),
            found: DecisionValue::Role(LoopRole::Hole),
        },
    };
    let rendered = refusal.to_string();
    assert!(
        rendered.contains("selected outer, this binding gives hole"),
        "the roles did not reach the sentence as words: {rendered}"
    );

    // The index-set arm keeps a `Debug` list — its members are
    // identifiers-as-location — but a noun introduces it, so the list
    // reads as a value rather than as a dump that leaked into prose.
    let sets = StructureRefusal {
        decision: Decision::TangentJoints { loop_: 0 },
        kind: StructureRefusalKind::Flipped {
            recorded: DecisionValue::Set(vec![1, 2]),
            found: DecisionValue::Set(vec![1, 3]),
        },
    };
    assert!(
        sets.to_string()
            .contains("selected indices [1, 2], this binding gives indices [1, 3]"),
        "the index sets lost their noun: {sets}"
    );

    // The gate and shape arms, composed the same way.
    let gate = StructureRefusal {
        decision: Decision::CornerGate {
            fillet: 0,
            corner: 0,
        },
        kind: StructureRefusalKind::Flipped {
            recorded: DecisionValue::Gate(CornerGate::Admitted),
            found: DecisionValue::Gate(CornerGate::RefusedReach),
        },
    };
    let g = gate.to_string();
    assert!(
        g.contains("selected admitted, this binding gives refused"),
        "{g}"
    );
    assert!(!g.contains("Admitted"), "the gate reverted to Debug: {g}");
}

/// **A record whose radius emissions this pass did not reproduce
/// refuses TYPED, naming the emission** — the emission record is
/// consumed the way the spans are, not carried along unread.
///
/// What this pins is that the guided pass reports ITS OWN emissions
/// and that the comparison happens: an arm that handed the caller back
/// the record it was given would agree with any lie, including one
/// crediting a fillet arc to a step that never held a radius, and the
/// consumer two layers up would then stamp a wall with an expression
/// nothing drew it from.
///
/// The fence row above buys the positive half over the whole coverage
/// corpus: every recorded program is replayed guided against its own
/// record, and a reproduced emission list that differed anywhere would
/// refuse there. That half reaches all three roles —
/// `path_program::every_radius_role_is_reached_by_the_corpus` is what
/// holds it to that — and the row below authors the sharpest of them
/// on its own so the refusal side is pinned there too.
#[test]
fn a_lying_radius_emission_refuses_typed_naming_the_emission() {
    let program = vesica_lens(0.0);
    let (_, structure) = replay_recording(&program, tol()).expect("the lens replays");
    assert!(
        !structure.radii.is_empty(),
        "the lens draws a fillet arc, so its record names one"
    );
    let mut lie = structure.clone();
    lie.radii[0].step += 1;
    let err = replay_guided(&program, &lie, tol()).expect_err("the emission is contradicted");
    let ReplayErrorKind::Path(PathError::Structure(refusal)) = err.kind else {
        panic!("expected a structure refusal, got {:?}", err.kind);
    };
    assert_eq!(refusal.decision, Decision::RadiusEmission { at: 0 });
    assert!(matches!(
        refusal.kind,
        StructureRefusalKind::Flipped {
            recorded: DecisionValue::Emission(_),
            found: DecisionValue::Emission(_),
        }
    ));
    // Both sides reach the sentence as words, through the emission's
    // own `Display` — the vocabulary rule the refusal payloads follow.
    let rendered = refusal.to_string();
    for want in [lie.radii[0].to_string(), structure.radii[0].to_string()] {
        assert!(rendered.contains(&want), "{rendered:?} is missing {want:?}");
    }
    assert!(
        !rendered.contains("RadiusEmission"),
        "the Debug spelling leaked into the sentence: {rendered}"
    );
}

/// **A record naming MORE radius emissions than the pass made is
/// refused at the record's own shape** — the same arm a short fillet
/// list draws, because no single emission moved.
#[test]
fn a_record_with_an_extra_radius_emission_refuses_at_its_shape() {
    let program = vesica_lens(0.0);
    let (_, structure) = replay_recording(&program, tol()).expect("the lens replays");
    let mut lie = structure.clone();
    let extra = *lie.radii.first().expect("the lens records one");
    lie.radii.push(extra);
    let err = replay_guided(&program, &lie, tol()).expect_err("the record is longer");
    let ReplayErrorKind::Path(PathError::Structure(refusal)) = err.kind else {
        panic!("expected a structure refusal, got {:?}", err.kind);
    };
    assert_eq!(refusal.decision, Decision::RecordShape);
    assert_eq!(
        refusal.kind,
        StructureRefusalKind::Flipped {
            recorded: DecisionValue::Count(structure.radii.len() + 1),
            found: DecisionValue::Count(structure.radii.len()),
        }
    );
}

/// **A guided pass reproduces an ARRIVAL carrier emission, and every
/// way of moving it is refused.**
///
/// `Carrier2` is the role only a radius-bearing arrival spec reaches,
/// and `ArrivalSpec` admits a radius in second position for
/// `Radius` alone — so `arc_fillet_arc(Sweep, r, Radius)` is the one
/// replayable shape in the language that records all three roles at
/// once. The three lies are the three independent ways a reproduced
/// emission can differ from a recorded one: its ROLE, its SEGMENT,
/// and its ORDER in the list.
#[test]
fn a_guided_pass_reproduces_and_checks_an_arrival_carrier_emission() {
    use profile::{ArcSide, Radius, RadiusRole, Start, Sweep};
    let three = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, tol())
        .unwrap()
        .line(4.0, tol())
        .unwrap()
        .tangent()
        .arc_fillet_arc(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.6,
            },
            0.25,
            Radius {
                r: 3.0,
                side: ArcSide::Left,
            },
            tol(),
        )
        .unwrap()
        .at(p2(2.0, 6.0))
        .toward(-1.0, 0.0, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    let program = three.program.clone();
    let (_, recorded) = replay_recording(&program, tol()).expect("the fused chain replays");
    assert!(
        recorded
            .radii
            .iter()
            .any(|e| e.role == RadiusRole::Carrier2),
        "the fixture records an arrival carrier: {:?}",
        recorded.radii
    );
    replay_guided(&program, &recorded, tol()).expect("guided against its own record");
    let at = recorded
        .radii
        .iter()
        .position(|e| e.role == RadiusRole::Carrier2)
        .expect("the arrival carrier is recorded");
    let mut lie = recorded.clone();
    lie.radii[at].role = RadiusRole::Carrier;
    replay_guided(&program, &lie, tol()).expect_err("a swapped role is refused");
    let mut moved = recorded.clone();
    moved.radii[at].segment += 1;
    replay_guided(&program, &moved, tol()).expect_err("a moved segment is refused");
    let mut reordered = recorded.clone();
    reordered.radii.swap(0, at);
    replay_guided(&program, &reordered, tol())
        .expect_err("the same emissions in a different ORDER are refused");
}
