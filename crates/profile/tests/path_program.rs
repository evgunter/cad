//! **LIB-SWITCH-P: profiles-as-programs v2, the profile half.**
//!
//! The mandated differential pin (spec §3d) rides on `common::pinned`,
//! which every closing verb in the corpus already funnels through — so
//! `path_differential.rs`, `path_property.rs` and this file together pin
//! record→replay bit-identity over the WHOLE typed-surface corpus, not a
//! sample. This suite adds what that blanket cannot express:
//!
//! - the ONE census smoke row (LIB-RESPELL item 2): the §2c re-spell
//!   made the typed surface and the replay driver call the SAME kernel
//!   binders, so the V2 drift-proofing census — the dedicated semantic
//!   rows, the tour shapes, the random-chain generator — became a
//!   tautology and RETIRED onto `the_fused_family_records_and_replays`
//!   below (the mapping: every retired row's (state, verb) pairs are a
//!   subset of the arms that composite chain plus the differential and
//!   property suites' blanket `pinned` exercise);
//! - the fused-family recording shapes: every §2c chain — entry fused
//!   verb, mid-chain `Radius` arc extension, the arc-arrival close —
//!   records the ONE fused vocabulary, and its steps keep authored data
//!   only;
//! - the driver's own refusal surface: the Transition class (corrupt
//!   file — no authoring surface can produce it) against the Path class
//!   (legal at rest, geometry refuses under this binding).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{coverage_corpus, pinned};
use geom_core::Point2;
use geom_core::Tol;
use profile::{
    ArcMode, ArcSweep, ClosedLoop, Open, PathError, ProfileLoop, ReplayError, ReplayErrorKind,
    Start, Step, Target, TipState, Verb, replay,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The recorded program of a chain, kept alongside its pinned loop.
fn program_of(closed: &ClosedLoop<f64>) -> Vec<Step<f64>> {
    closed.program.clone()
}

/// Every verb a program names, in order — the readable half of a step
/// inventory assertion.
fn verbs(program: &[Step<f64>]) -> Vec<Verb> {
    program.iter().map(Step::verb).collect()
}

/// Every arc mode a program names, in step order — the two fused
/// verbs' incoming and arrival specs counted separately.
///
/// Exhaustive on [`Step`], with the spec-free verbs named rather than
/// swept into a trailing arm: which verbs carry an arc spec is the one
/// thing this helper assumes, and a verb that GAINS one has to be
/// adjudicated here rather than silently falling out of reach.
fn arc_modes(program: &[Step<f64>]) -> Vec<ArcMode> {
    let mut out = Vec::new();
    for step in program {
        match step {
            Step::ArcTo(spec) | Step::FilletArc { spec, .. } | Step::ArcFillet { spec, .. } => {
                out.push(spec.mode());
            }
            Step::ArcFilletArc { spec, spec2, .. } => {
                out.push(spec.mode());
                out.push(spec2.mode());
            }
            Step::At(_)
            | Step::Angle(_)
            | Step::Toward { .. }
            | Step::Tangent
            | Step::Cusp
            | Step::Turn(_)
            | Step::Line(_)
            | Step::LineTo(_)
            | Step::TangentArcTo(_)
            | Step::ArcContinue(_)
            | Step::Fillet { .. }
            | Step::FarEndTo(_)
            | Step::CloseTo
            | Step::Circle { .. }
            | Step::CircleSplit { .. } => {}
        }
    }
    out
}

// ------------------------------------------------------------------
// §3d — the four dedicated semantic rows
// ------------------------------------------------------------------

fn validate_ok(lp: &ProfileLoop<f64>) {
    use profile::{Profile, SketchPlane};
    Profile::new(SketchPlane::xy(), vec![lp.clone()])
        .validate(Tol::witness())
        .expect("the replayed loop validates");
}

/// **The census smoke row** — the one survivor of the V2 differential
/// census (LIB-RESPELL item 2). One composite chain exercises the §2c
/// family end to end: the fused ENTRY verb, an interior arc arrival
/// (a directed point at the hard anchor), the `Radius` arc extension
/// off it, straight arrivals via the uniform binders, a sharp `Sweep`
/// leg, ray extension off a leg end, and a straight close. Its recorded program must replay to the
/// SAME bits — which, now that both surfaces call one kernel, is the
/// tautology the census retired into; this row smokes the plumbing
/// (recording, driver arms, the state walk) rather than proving two
/// implementations equivalent.
#[test]
fn the_fused_family_records_and_replays_bit_identically() {
    use profile::{ArcSide, Center, Radius, Sweep};
    let closed = Open
        .arc_fillet(
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(5.0, 0.0),
            },
            0.5,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    assert_eq!(
        verbs(&program_of(&closed)),
        vec![
            Verb::ArcFillet,
            Verb::At,
            Verb::Toward,
            Verb::Line,
            Verb::LineTo
        ]
    );
    // The steps kept authored data only: the fused step stores the
    // carrier spec verbatim, the binders their own arguments.
    match closed.program[0] {
        Step::ArcFillet {
            spec: profile::ArcData::Center { c, winding, target },
            radius,
        } => {
            assert_eq!(c.x.to_bits(), 0.0_f64.to_bits());
            assert_eq!(winding, ArcSweep::Ccw);
            assert!(matches!(target, Target::Point(p) if p.x.to_bits() == 5.0_f64.to_bits()));
            assert_eq!(radius.to_bits(), 0.5_f64.to_bits());
        }
        ref other => panic!("expected the fused ArcFillet step, got {other:?}"),
    }
    validate_ok(&pinned(closed));

    // The wider walk: a Sweep leg, ray extension off its end, an
    // interior Center arrival, and the Radius arc extension off its
    // directed point — recorded and replayed to the bit (`pinned` is
    // the assertion).
    let walk = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .arc_to(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.6,
            },
            Tol::witness(),
        )
        .unwrap()
        .fillet(0.2, Tol::witness())
        .unwrap()
        .at(p2(4.0, 3.0), Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.25,
            Center {
                c: p2(2.0, 6.0),
                winding: ArcSweep::Ccw,
                p: p2(2.0, 9.0),
            },
            Tol::witness(),
        )
        .unwrap()
        .arc_fillet(
            Radius {
                r: 3.0,
                side: ArcSide::Left,
            },
            0.25,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(1.0, 4.0), Tol::witness())
        .unwrap()
        .toward(0.0, -1.0, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    assert_eq!(
        verbs(&program_of(&walk)),
        vec![
            Verb::At,
            Verb::Angle,
            Verb::ArcTo,
            Verb::Fillet,
            Verb::At,
            Verb::Toward,
            Verb::FilletArc,
            Verb::ArcFillet,
            Verb::At,
            Verb::Toward,
            Verb::Line,
            Verb::LineTo
        ]
    );
    validate_ok(&pinned(walk));
}

/// **The MID-CHAIN Radius arc-extension row** (see
/// `family::LegEndIncoming`). An entry fused verb with an interior
/// `Center` arrival, continued off the resulting directed point by
/// `arc_fillet(Radius { .. })` and its binders, comes out as
/// [`ArcFilletArc` (entry `Center`), `ArcFillet` (`Radius`), binders,
/// legs]; the `Radius` names the carrier the tip already runs on
/// (`r = |anchor − centre| = 3`, centre right of Cw travel), so the
/// recorded program and the typed elaboration are the same geometry and
/// replay to the same bits (`pinned` asserts it).
#[test]
fn the_mid_chain_radius_row_records_and_replays() {
    use profile::{ArcSide, Center, Radius};
    let closed = Open
        .arc_fillet_arc(
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(5.0, 0.0),
            },
            0.5,
            Center {
                c: p2(0.0, 7.0),
                winding: ArcSweep::Cw,
                p: p2(0.0, 4.0),
            },
            Tol::witness(),
        )
        .unwrap()
        .arc_fillet(
            Radius {
                r: 3.0,
                side: ArcSide::Right,
            },
            0.3,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(-2.0, 2.0), Tol::witness())
        .unwrap()
        .toward(0.0, -1.0, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    assert_eq!(
        verbs(&program_of(&closed)),
        vec![
            Verb::ArcFilletArc,
            Verb::ArcFillet,
            Verb::At,
            Verb::Toward,
            Verb::Line,
            Verb::LineTo
        ]
    );
    match closed.program[1] {
        Step::ArcFillet {
            spec: profile::ArcData::Radius { r, side },
            radius,
        } => {
            assert_eq!(r.to_bits(), 3.0_f64.to_bits(), "|anchor − centre|");
            assert_eq!(side, profile::ArcSide::Right, "Cw travel = centre right");
            assert_eq!(radius.to_bits(), 0.3_f64.to_bits());
        }
        ref other => panic!("expected the Radius arc-extension fused step, got {other:?}"),
    }
    validate_ok(&pinned(closed));
}

/// **A whole loop in ONE fused step**: the eye — arc-carrier entry,
/// fillet, arc-arrival close — is authored by a single
/// `arc_fillet_arc(Center { .. }, r, Center { .., p: Start })` and so
/// records as exactly one `ArcFilletArc` step, whose payload is the two
/// authored carrier specs and the radius; that program replays to the
/// same bits.
#[test]
fn the_eye_is_one_fused_step() {
    use profile::Center;
    let tip = 0.75f64.sqrt();
    let closed = Open
        .arc_fillet_arc(
            Center {
                c: p2(-0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -tip),
            },
            0.35,
            Center {
                c: p2(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(verbs(&program_of(&closed)), vec![Verb::ArcFilletArc]);
    match closed.program[0] {
        Step::ArcFilletArc {
            spec,
            radius,
            spec2,
        } => {
            assert!(matches!(
                spec,
                profile::ArcData::Center { target: Target::Point(p), .. }
                    if p.y.to_bits() == (-tip).to_bits()
            ));
            assert_eq!(radius.to_bits(), 0.35_f64.to_bits());
            assert!(matches!(
                spec2,
                profile::ArcData::Center {
                    target: Target::Start,
                    ..
                }
            ));
        }
        ref other => panic!("expected the fused ArcFilletArc step, got {other:?}"),
    }
    validate_ok(&pinned(closed));
}

/// **`circle`'s two-pole lowering.** The primitive is a ONE-STEP
/// complete-loop program; the ±x poles and the unit bulges are its
/// private lowering, and the replay reproduces them from `(centre, r)`
/// alone.
#[test]
fn circle_is_a_one_step_program_that_replays_to_its_two_poles() {
    let closed = profile::circle(p2(1.5, -2.25), 0.75, Tol::witness()).unwrap();
    assert_eq!(verbs(&program_of(&closed)), vec![Verb::Circle]);
    assert_eq!(
        closed.program.len(),
        1,
        "a circle program is exactly one step"
    );
    let lowered = pinned(closed);
    assert_eq!(lowered.vertices().len(), 2);
    assert_eq!(lowered.vertices()[0].pos().x.to_bits(), 2.25_f64.to_bits());
    assert_eq!(lowered.vertices()[1].pos().x.to_bits(), 0.75_f64.to_bits());
    assert_eq!(lowered.vertices()[0].bulge().to_bits(), 1.0_f64.to_bits());
    assert!(
        lowered.tangent_joints().is_empty(),
        "same-carrier joints declare nothing — there is no tangency to claim"
    );
    validate_ok(&lowered);
}

/// **`circle_split`'s declared subdivision (LIB-SWITCH corpus ruling).**
/// One carrier, `n` structural vertices: the program is one step, the
/// replay reproduces the lowering from `(centre, r, n, phase)` alone,
/// every bulge is `tan(π/(2n))`, and nothing is declared tangent
/// (same-carrier identities, exactly `circle`'s posture).
#[test]
fn circle_split_is_a_one_step_program_with_structural_seams() {
    let closed = profile::circle_split(p2(1.0, 0.5), 0.4, 3, 0.25, Tol::witness()).unwrap();
    assert_eq!(verbs(&program_of(&closed)), vec![Verb::CircleSplit]);
    let lowered = pinned(closed);
    assert_eq!(lowered.vertices().len(), 3, "n vertices, n arcs");
    // Expected values through the SAME libm-pure trig the lowering uses
    // (geom-core `Real`; std's tan/sin_cos may differ by an ulp).
    let expected_bulge = geom_core::Real::tan(std::f64::consts::PI / 6.0);
    for v in lowered.vertices() {
        assert_eq!(v.bulge().to_bits(), expected_bulge.to_bits());
    }
    // Vertex k at centre + r·(cos θ_k, sin θ_k), θ_k = phase + k·2π/n.
    for (k, v) in lowered.vertices().iter().enumerate() {
        let theta = 0.25 + (k as f64) * std::f64::consts::TAU / 3.0;
        let (s, c) = geom_core::Real::sin_cos(theta);
        assert_eq!(v.pos().x.to_bits(), (1.0 + 0.4 * c).to_bits());
        assert_eq!(v.pos().y.to_bits(), (0.5 + 0.4 * s).to_bits());
    }
    assert!(
        lowered.tangent_joints().is_empty(),
        "structural subdivisions declare nothing — one carrier, no tangency claim"
    );
    validate_ok(&lowered);
}

/// `circle_split` refusals: the radius gate is `circle`'s own (funnel
/// row `path_circle_radius`), and n < 2 is the structural
/// [`PathError::CircleSplitCount`] class.
#[test]
fn circle_split_refuses_nonpositive_radius_and_tiny_counts() {
    let _tol = Tol::witness().get();
    match profile::circle_split(p2(0.0, 0.0), 0.0, 4, 0.0, Tol::witness()) {
        Err(PathError::NonpositiveCircleRadius { .. }) => {}
        other => panic!("r = 0 must refuse as NonpositiveCircleRadius, got {other:?}"),
    }
    match profile::circle_split(p2(0.0, 0.0), 1.0, 1, 0.0, Tol::witness()) {
        Err(PathError::CircleSplitCount { n: 1 }) => {}
        other => panic!("n = 1 must refuse as CircleSplitCount, got {other:?}"),
    }
    // n = 2 is legal — the smallest subdivision, circle's own count.
    let two = profile::circle_split(p2(0.0, 0.0), 1.0, 2, 0.0, Tol::witness()).unwrap();
    assert_eq!(pinned(two).vertices().len(), 2);
}

/// **`arc_continue`'s declared subdivision (LIB-SWITCH §5-1 fallback,
/// the half-disc's equator vertex).** Two quarter arcs on ONE carrier:
/// the first authored (`arc_to(Bulge { .. })` with bulge tan(π/8)), the
/// second a structural subdivision — same carrier, derived bulge, no
/// junction claim, nothing declared tangent. Replays bit-identically.
#[test]
fn arc_continue_subdivides_the_carrier_structurally() {
    use profile::Bulge;
    let q = std::f64::consts::FRAC_PI_8.tan();
    let closed = Open
        .at(p2(0.0, -0.5))
        .arc_to(
            Bulge {
                p: p2(0.5, 0.0),
                b: q,
            },
            Tol::witness(),
        )
        .unwrap()
        .arc_continue(p2(0.0, 0.5), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    assert_eq!(
        verbs(&program_of(&closed)),
        vec![Verb::At, Verb::ArcTo, Verb::ArcContinue, Verb::LineTo],
        "the subdivision records as its own verb, storing only the authored target"
    );
    let lowered = pinned(closed);
    assert_eq!(lowered.vertices().len(), 3);
    // The derived bulge continues the SAME carrier: a quarter of the
    // r = 0.5 circle about the origin, tan(π/8) up to the tangent-chord
    // derivation's rounding.
    let b = lowered.vertices()[1].bulge();
    assert!(
        (b - q).abs() < 1e-15,
        "continuation bulge ≈ tan(π/8), got {b}"
    );
    assert!(
        lowered.tangent_joints().is_empty(),
        "a subdivision vertex claims nothing — same-carrier identity, not tangency"
    );
    validate_ok(&lowered);
}

/// `arc_continue` refusals: a straight incoming leg has nothing to
/// subdivide; an off-carrier target is contradictory authored data.
#[test]
fn arc_continue_refuses_lines_and_off_carrier_targets() {
    use profile::Bulge;
    let after_line = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(1.0, 0.0), Tol::witness())
        .unwrap();
    match after_line.arc_continue(p2(2.0, 0.0), Tol::witness()) {
        Err(PathError::ArcContinueNeedsArcCarrier) => {}
        other => panic!("a straight leg must refuse arc_continue, got {other:?}"),
    }
    let q = std::f64::consts::FRAC_PI_8.tan();
    let after_arc = Open
        .at(p2(0.0, -0.5))
        .arc_to(
            Bulge {
                p: p2(0.5, 0.0),
                b: q,
            },
            Tol::witness(),
        )
        .unwrap();
    match after_arc.arc_continue(p2(0.3, 0.5), Tol::witness()) {
        Err(PathError::ArcContinueOffCarrier { .. }) => {}
        other => panic!("an off-carrier target must refuse, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// The replay-coverage census (LIB-RTABLE)
// ------------------------------------------------------------------

/// **Every verb the transition table declares is exercised by a
/// record→replay round-trip.** The failure class this pins is a row
/// whose DRIVER projection silently stops working: the four-projection
/// invariant makes a deleted ROW a compile error everywhere, but a row
/// that keeps its typed method while its arm goes missing or
/// over-strict still compiles, and the only thing standing behind that
/// direction is a test that actually replays the verb.
///
/// The census is anchored on [`Verb::ALL`], projected from the same
/// declaration as the rows, so it cannot fall behind a verb the table
/// gains — a new verb with no corpus chain fails HERE, by name, rather
/// than quietly acquiring an unpinned arm. (`.turn(δ)` is the standing
/// example: its round-trip rows were retired with the V2 census in
/// 68d80104 and nothing replaced them, leaving the (DirectedPoint,
/// Turn) arm pinned by nothing until this row.)
///
/// Granularity is honest: this is verb coverage, not per-arm coverage.
/// The arms of a multi-row verb are covered here only where the corpus
/// happens to reach both states; what is NOT possible is a verb with
/// no replayed arm at all.
#[test]
fn every_table_verb_is_replayed_by_the_corpus() {
    let mut seen: Vec<Verb> = Vec::new();
    for closed in coverage_corpus() {
        seen.extend(verbs(&closed.program));
        // The round-trip itself: replay the recording, bit-identical.
        pinned(closed);
    }
    let missing: Vec<&Verb> = Verb::ALL.iter().filter(|v| !seen.contains(v)).collect();
    assert!(
        missing.is_empty(),
        "these table verbs are declared but never replayed by the corpus: {missing:?} \
         — every row's driver arm must be exercised by a record->replay chain, \
         so add one to `coverage_corpus` (see this test's rustdoc)"
    );
}

/// **Every arc mode the vocabulary declares is exercised by a
/// record→replay round-trip.**
///
/// The mode travels INSIDE a verb, so the verb census above cannot see
/// it: `ArcTo` is replayed by one chain whatever the other five modes
/// do, and a mode whose dispatcher arm goes missing or over-strict
/// takes nothing red with it. This row is that arm's only standing
/// pressure, anchored on [`ArcMode::ALL`] and therefore unable to fall
/// behind a mode the vocabulary gains.
///
/// Granularity is honest for the same reason the verb census's is:
/// this is mode coverage, not (state, mode) coverage — the pairs the
/// §2c matrix admits at more than one state are covered here only
/// where the corpus reaches them.
#[test]
fn every_arc_mode_is_replayed_by_the_corpus() {
    let mut seen: Vec<ArcMode> = Vec::new();
    for closed in coverage_corpus() {
        seen.extend(arc_modes(&closed.program));
        // The round-trip itself: replay the recording, bit-identical.
        pinned(closed);
    }
    let missing: Vec<&ArcMode> = ArcMode::ALL.iter().filter(|m| !seen.contains(m)).collect();
    assert!(
        missing.is_empty(),
        "these arc modes are declared but never replayed by the corpus: {missing:?} \
         — every mode's dispatcher arm must be exercised by a record->replay chain, \
         so add one to `coverage_corpus` (see this test's rustdoc)"
    );
}

fn assert_transition(program: &[Step<f64>], step: usize, state: TipState, verb: Option<Verb>) {
    match replay(program, Tol::witness()) {
        Err(ReplayError {
            step: s,
            kind:
                ReplayErrorKind::Transition {
                    state: st,
                    verb: vb,
                },
        }) => {
            assert_eq!((s, st, vb), (step, state, verb), "transition refusal");
        }
        other => panic!("expected a lattice violation, got {other:?}"),
    }
}

/// **The Transition class**: programs no authoring surface can produce.
/// Each row is one of the lattice violations PROFILES-V2 §V1 names —
/// reachable only from a hand-edited or corrupt wire form.
#[test]
fn lattice_violations_refuse_as_the_transition_class() {
    let a = p2(0.0, 0.0);
    // A leading fillet: nothing is bound yet.
    assert_transition(
        &[Step::Fillet { radius: 0.5 }],
        0,
        TipState::Entry,
        Some(Verb::Fillet),
    );
    // A double director on a Directed tip.
    assert_transition(
        &[Step::At(a), Step::Angle(0.0), Step::Angle(1.0)],
        2,
        TipState::DirectedPlain,
        Some(Verb::Angle),
    );
    // A leg from a half-bound tip (position without direction).
    assert_transition(
        &[Step::At(a), Step::Line(1.0)],
        1,
        TipState::PlainPoint,
        Some(Verb::Line),
    );
    // `.tangent()` on a plain point: no incoming tangent to inherit.
    assert_transition(
        &[Step::At(a), Step::Tangent],
        1,
        TipState::PlainPoint,
        Some(Verb::Tangent),
    );
    // The seam close mid-chain, with no fillet open.
    assert_transition(
        &[Step::At(a), Step::Angle(0.0), Step::CloseTo],
        2,
        TipState::DirectedPlain,
        Some(Verb::CloseTo),
    );
    // The one-step complete-loop form is not a chain verb.
    assert_transition(
        &[
            Step::At(a),
            Step::Circle {
                centre: a,
                radius: 1.0,
            },
        ],
        1,
        TipState::PlainPoint,
        Some(Verb::Circle),
    );
    // §2c: an INADMISSIBLE (state, mode) pair is unrepresentable at the
    // typed surface (a missing trait impl), so at the wire it is this
    // same class. Sweep needs a tangent, which a bare Point lacks …
    assert_transition(
        &[
            Step::At(a),
            Step::ArcTo(profile::ArcData::Sweep {
                r: 1.0,
                side: profile::ArcSide::Left,
                angle: 0.5,
            }),
        ],
        1,
        TipState::PlainPoint,
        Some(Verb::ArcTo),
    );
    // … the entry fused verb admits Center alone (nothing else seeds) …
    assert_transition(
        &[Step::ArcFillet {
            spec: profile::ArcData::Radius {
                r: 1.0,
                side: profile::ArcSide::Left,
            },
            radius: 0.25,
        }],
        0,
        TipState::Entry,
        Some(Verb::ArcFillet),
    );
    // … and Bulge is never an arrival (no chord exists there).
    assert_transition(
        &[
            Step::At(a),
            Step::Angle(0.0),
            Step::FilletArc {
                radius: 0.25,
                spec: profile::ArcData::Bulge {
                    target: Target::Point(p2(2.0, 2.0)),
                    b: 0.5,
                },
            },
        ],
        2,
        TipState::DirectedPlain,
        Some(Verb::FilletArc),
    );
}

/// A chain that never reaches `Start`, an empty program, and a step
/// AFTER the close are the same class, one step past where they stop.
#[test]
fn unclosed_trailing_and_empty_programs_are_the_transition_class() {
    let (a, b, c) = (p2(0.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0));
    assert_transition(&[], 0, TipState::Entry, None);
    assert_transition(
        &[Step::At(a), Step::LineTo(Target::Point(b))],
        2,
        TipState::DirectedPoint,
        None,
    );
    assert_transition(
        &[
            Step::At(a),
            Step::LineTo(Target::Point(b)),
            Step::LineTo(Target::Point(c)),
            Step::LineTo(Target::Start),
            Step::Tangent,
        ],
        4,
        TipState::Closed,
        Some(Verb::Tangent),
    );
    assert_transition(
        &[
            Step::Circle {
                centre: a,
                radius: 1.0,
            },
            Step::At(b),
        ],
        1,
        TipState::Closed,
        Some(Verb::At),
    );
}

/// **The Path class**: well-typed programs the GEOMETRY refuses. These
/// can exist at rest — under another parameter binding the same program
/// elaborates cleanly (PROFILES-V2 §V1 class 2). The pair below is that
/// argument executed: one program shape, two radii, two outcomes.
#[test]
fn geometry_refusals_are_the_path_class_and_are_binding_dependent() {
    let square = |r: f64| {
        vec![
            Step::At(p2(0.0, -1.0)),
            Step::Angle(0.0),
            Step::Fillet { radius: r },
            Step::At(p2(1.0, 0.0)),
            Step::Angle(std::f64::consts::FRAC_PI_2),
            Step::Fillet { radius: r },
            Step::At(p2(0.0, 1.0)),
            Step::Angle(std::f64::consts::PI),
            Step::Fillet { radius: r },
            Step::At(p2(-1.0, 0.0)),
            Step::Angle(-std::f64::consts::FRAC_PI_2),
            Step::Fillet { radius: r },
            Step::CloseTo,
        ]
    };
    replay(&square(0.25), Tol::witness()).expect("r = 0.25 elaborates");
    let refused = replay(&square(5.0), Tol::witness());
    match refused {
        Err(ReplayError {
            kind: ReplayErrorKind::Path(_),
            ..
        }) => {}
        other => panic!("r = 5.0 must refuse in the PATH class, got {other:?}"),
    }

    // The sign gates are the same class, carried straight through.
    match replay(
        &[Step::Circle {
            centre: p2(0.0, 0.0),
            radius: 0.0,
        }],
        Tol::witness(),
    ) {
        Err(ReplayError {
            step: 0,
            kind: ReplayErrorKind::Path(PathError::NonpositiveCircleRadius { .. }),
        }) => {}
        other => panic!("a zero radius must refuse NonpositiveCircleRadius, got {other:?}"),
    }
    match replay(
        &[
            Step::At(p2(0.0, 0.0)),
            Step::Angle(0.0),
            Step::Fillet { radius: -1.0 },
        ],
        Tol::witness(),
    ) {
        Err(ReplayError {
            step: 2,
            kind: ReplayErrorKind::Path(PathError::NonpositiveFilletRadius { .. }),
        }) => {}
        other => panic!("a negative fillet radius must refuse typed, got {other:?}"),
    }
}
