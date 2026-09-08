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

use crate::common;

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
            Step::ArcTo { spec, .. }
            | Step::FilletArc { spec, .. }
            | Step::ArcFillet { spec, .. } => {
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
            | Step::ContinueTo(_)
            | Step::TangentArcTo(_)
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
            Step::ArcTo {
                spec: profile::ArcData::Sweep {
                    r: 1.0,
                    side: profile::ArcSide::Left,
                    angle: 0.5,
                },
                splits: 1,
            },
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

/// A lattice refusal renders the (tip state, verb) pair as the
/// transition table's own spellings, each introduced by the noun it
/// is. The pair is a COORDINATE — the row a reader looks up next — so
/// a prose paraphrase of either half would name nothing findable; this
/// pin is what makes the `Debug` spelling a decision rather than an
/// oversight, and it fails if either half is worded away.
#[test]
fn a_lattice_refusal_names_the_table_coordinate_it_could_not_walk() {
    let unwalkable = replay(&[Step::Fillet { radius: 0.5 }], Tol::witness())
        .expect_err("a leading fillet binds nothing, so it is off the lattice");
    assert_eq!(
        unwalkable.to_string(),
        "step 0: the Fillet verb is not a legal continuation of tip state Entry \
         (lattice violation — no authoring surface can produce this program)"
    );

    let unclosed = replay(
        &[
            Step::At(p2(0.0, 0.0)),
            Step::LineTo(Target::Point(p2(1.0, 0.0))),
        ],
        Tol::witness(),
    )
    .expect_err("a chain that never returns to Start does not close");
    assert_eq!(
        unclosed.to_string(),
        "step 2: the program ends at tip state DirectedPoint without closing the loop \
         (lattice violation — a chain must end at Start)"
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

// ------------------------------------------------------------------
// The declared-split arc leg — `arc_to(spec.split(n))`
// ------------------------------------------------------------------

fn bits(v: f64) -> u64 {
    v.to_bits()
}

/// **The half-disc's equator as ONE leg that declares its split.** The
/// semicircle from the south pole to the north pole, split in two: the
/// station is the axis point `(R, 0)` bit for bit, both pieces carry
/// tan(π/8) exactly, and the joint at the station is a DECLARED
/// TANGENT joint on the one carrier (Ev, in-chat, 2026-09-02: every
/// zero-turn joint is a declared tangent joint). The program records
/// the plain verb with its count; it replays bit-identically.
#[test]
fn split_arc_mints_declared_tangent_joints_on_one_carrier() {
    use profile::Bulge;
    let q = std::f64::consts::FRAC_PI_8.tan();
    let closed = Open
        .at(p2(0.0, -0.5))
        .arc_to(
            Bulge {
                p: p2(0.0, 0.5),
                b: 1.0,
            }
            .split(2),
            Tol::witness(),
        )
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let program = program_of(&closed);
    assert_eq!(verbs(&program), vec![Verb::At, Verb::ArcTo, Verb::LineTo]);
    assert!(
        matches!(program[1], Step::ArcTo { splits: 2, .. }),
        "the split rides the leg's own step: {:?}",
        program[1]
    );
    let lowered = pinned(closed);
    assert_eq!(lowered.vertices().len(), 3);
    let v1 = lowered.vertices()[1];
    assert_eq!((bits(v1.pos().x), bits(v1.pos().y)), (bits(0.5), bits(0.0)));
    assert_eq!(bits(lowered.vertices()[0].bulge()), bits(q));
    assert_eq!(bits(v1.bulge()), bits(q));
    assert_eq!(lowered.tangent_joints(), &[1]);
    validate_ok(&lowered);
}

/// **The axis station is exact.** The same semicircle authored about
/// its centre from `(1, 0)` to `(−1, 0)`: the middle station of an
/// even split is `centre + R·m̂` (the placement contract), so the pole
/// lands on `(0, 1)` bit for bit where a station computed by rotation
/// would sit cos(π/2) ≈ 6e-17 off the axis — a vertex a revolve may
/// read as off-axis.
#[test]
fn split_arc_places_the_axis_station_exactly() {
    use profile::Center;
    let q = std::f64::consts::FRAC_PI_8.tan();
    let closed = Open
        .at(p2(1.0, 0.0))
        .arc_to(
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(-1.0, 0.0),
            }
            .split(2),
            Tol::witness(),
        )
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let lowered = pinned(closed);
    let v1 = lowered.vertices()[1];
    assert_eq!((bits(v1.pos().x), bits(v1.pos().y)), (bits(0.0), bits(1.0)));
    assert_eq!(bits(lowered.vertices()[0].bulge()), bits(q));
    assert_eq!(bits(v1.bulge()), bits(q));
    assert_eq!(lowered.tangent_joints(), &[1]);
    validate_ok(&lowered);
}

/// **The reversal identity.** Stations are computed from their NEARER
/// endpoint, so the leg authored backwards (end→start, bulge negated)
/// places the same stations bit for bit, for an odd count and for an
/// even one (whose middle station is the closed form).
#[test]
fn split_arc_reverses_bit_identically() {
    use profile::Bulge;
    let (a, b, bulge) = (p2(0.3, 0.1), p2(1.7, 0.9), 0.37);
    for n in [4usize, 5] {
        let forward = pinned(
            Open.at(a)
                .arc_to(Bulge { p: b, b: bulge }.split(n), Tol::witness())
                .unwrap()
                .line_to(Start, Tol::witness())
                .unwrap(),
        );
        let backward = pinned(
            Open.at(b)
                .arc_to(Bulge { p: a, b: -bulge }.split(n), Tol::witness())
                .unwrap()
                .line_to(Start, Tol::witness())
                .unwrap(),
        );
        assert_eq!(forward.vertices().len(), n + 1);
        for k in 1..n {
            let f = forward.vertices()[k];
            let r = backward.vertices()[n - k];
            assert_eq!(
                (bits(f.pos().x), bits(f.pos().y)),
                (bits(r.pos().x), bits(r.pos().y)),
                "n = {n}: station {k} vs reversed station {}",
                n - k
            );
            assert_eq!(
                bits(f.bulge()),
                bits(-r.bulge()),
                "n = {n}: piece bulge at {k}"
            );
        }
        let joints: Vec<usize> = (1..n).collect();
        assert_eq!(forward.tangent_joints(), &joints[..]);
        assert_eq!(backward.tangent_joints(), &joints[..]);
        validate_ok(&forward);
    }
}

/// **D2 — a count below 2 refuses typed**, on the typed surface and at
/// replay alike: one piece declares nothing the plain leg does not, and
/// zero is no leg. At replay `splits: 1` IS the plain leg (the count is
/// what every recorded arc leg carries).
#[test]
fn split_count_below_two_refuses_typed() {
    use profile::{ArcData, Bulge};
    for n in [0usize, 1] {
        match Open.at(p2(0.0, -0.5)).arc_to(
            Bulge {
                p: p2(0.0, 0.5),
                b: 1.0,
            }
            .split(n),
            Tol::witness(),
        ) {
            Err(PathError::ArcSplitCount { n: got }) => assert_eq!(got, n),
            other => panic!("split({n}) must refuse ArcSplitCount, got {other:?}"),
        }
    }
    let leg = |splits: usize| {
        [
            Step::At(p2(0.0, -0.5)),
            Step::ArcTo {
                spec: ArcData::Bulge {
                    target: Target::Point(p2(0.0, 0.5)),
                    b: 1.0,
                },
                splits,
            },
            Step::LineTo(Target::Start),
        ]
    };
    let plain = replay(&leg(1), Tol::witness()).expect("splits: 1 is the plain leg");
    assert_eq!(plain.vertices().len(), 2);
    assert!(plain.tangent_joints().is_empty());
    match replay(&leg(0), Tol::witness()) {
        Err(ReplayError {
            step: 1,
            kind: ReplayErrorKind::Path(PathError::ArcSplitCount { n: 0 }),
        }) => {}
        other => panic!("splits: 0 must refuse ArcSplitCount at step 1, got {other:?}"),
    }
}

/// **The endpoint-free legs split too** (admissibility is the wrapped
/// mode's): a `Sweep` split in three lands on the SAME end vertex the
/// unsplit leg does, its stations sit on the carrier, every piece
/// carries tan(θ/12), and both interior joints are declared.
#[test]
fn split_arc_on_a_directed_leg_lands_where_the_unsplit_leg_does() {
    use profile::{ArcSide, Sweep};
    use std::f64::consts::FRAC_PI_2;
    let sweep = Sweep {
        r: 0.5,
        side: ArcSide::Left,
        angle: FRAC_PI_2,
    };
    let unsplit = pinned(
        Open.at(p2(0.0, 0.0))
            .angle(0.0, Tol::witness())
            .unwrap()
            .arc_to(sweep, Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .unwrap(),
    );
    let split = pinned(
        Open.at(p2(0.0, 0.0))
            .angle(0.0, Tol::witness())
            .unwrap()
            .arc_to(sweep.split(3), Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .unwrap(),
    );
    assert_eq!(unsplit.vertices().len(), 2);
    assert_eq!(split.vertices().len(), 4);
    let (e, s) = (unsplit.vertices()[1], split.vertices()[3]);
    assert_eq!(
        (bits(e.pos().x), bits(e.pos().y)),
        (bits(s.pos().x), bits(s.pos().y))
    );
    let piece = (FRAC_PI_2 / 12.0).tan();
    for k in 0..3 {
        let v = split.vertices()[k];
        assert_eq!(bits(v.bulge()), bits(piece), "piece {k}");
        let radial = ((v.pos().x - 0.0).powi(2) + (v.pos().y - 0.5).powi(2)).sqrt();
        assert!(
            (radial - 0.5).abs() < 1e-15,
            "station {k} off the carrier by {radial}"
        );
    }
    assert_eq!(split.tangent_joints(), &[1, 2]);
    validate_ok(&split);
}

/// **A split closer**: the sharp arc seam with its interior stations
/// declared; the seam's own junction is classified exactly as the
/// unsplit closer's is.
#[test]
fn split_arc_closes_with_its_stations_declared() {
    use profile::Bulge;
    let closed = pinned(
        Open.at(p2(0.0, 0.0))
            .line_to(p2(1.0, 0.0), Tol::witness())
            .unwrap()
            .arc_to(Bulge { p: Start, b: 0.5 }.split(3), Tol::witness())
            .unwrap(),
    );
    assert_eq!(closed.vertices().len(), 4);
    assert_eq!(closed.tangent_joints(), &[2, 3]);
    let piece = (4.0 * 0.5f64.atan() / 12.0).tan();
    for k in 1..4 {
        assert_eq!(bits(closed.vertices()[k].bulge()), bits(piece), "piece {k}");
    }
    validate_ok(&closed);
}
