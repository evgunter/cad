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
    ArcMode, ArcSweep, ClosedLoop, Open, PathError, ProfileLoop, RadiusRole, ReplayError,
    ReplayErrorKind, Start, Step, Target, TipState, Verb, replay,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The recorded program of a chain, kept alongside its pinned loop.
fn program_of(closed: &ClosedLoop<f64>) -> Vec<Step<f64>> {
    closed.program.clone()
}

/// Each step as `Debug` renders it — the comparable form of a
/// recording, `Step` carrying no `PartialEq`.
fn rendered(program: &[Step<f64>]) -> Vec<String> {
    program.iter().map(|step| format!("{step:?}")).collect()
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

/// **A partial path reports the prefix of the program it publishes.**
///
/// `PartialPath::recorded` is what lets a caller writing a notation
/// beside a recording address the verb it has just called without
/// counting the ones before it, so the claim it has to carry is
/// exactly this: after every verb of a chain, the steps so far ARE the
/// finished program's prefix, one step per verb, binders included.
///
/// The chain mixes the shapes that could break the correspondence — a
/// leg, an arc, a `fillet` binder (a verb that records a step and
/// emits geometry only when the next one resolves it), a re-entry pair
/// and the closer. A `recorded` that omitted binder steps, or that
/// answered a step late, moves every prefix after the omission and
/// reds here.
///
/// Steps are compared through `Debug`, which renders every authored
/// field of a step and prints an `f64` as its shortest round-tripping
/// form — so two steps that render alike hold the same authored
/// numbers.
#[test]
fn a_partial_path_reports_the_prefix_of_the_program_it_publishes() {
    use profile::{ArcSide, Sweep};
    let t = Tol::witness();
    let mut prefixes: Vec<Vec<String>> = Vec::new();
    let path = Open.at(p2(0.0, 0.0));
    prefixes.push(rendered(path.recorded()));
    let path = path.angle(0.0, t).unwrap();
    prefixes.push(rendered(path.recorded()));
    let path = path
        .arc_to(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.6,
            },
            t,
        )
        .unwrap();
    prefixes.push(rendered(path.recorded()));
    let path = path.fillet(0.2, t).unwrap();
    prefixes.push(rendered(path.recorded()));
    let path = path.at(p2(4.0, 3.0), t).unwrap();
    prefixes.push(rendered(path.recorded()));
    let path = path.toward(0.0, 1.0, t).unwrap();
    prefixes.push(rendered(path.recorded()));
    let path = path.line(3.0, t).unwrap();
    prefixes.push(rendered(path.recorded()));
    let closed = path.line_to(Start, t).unwrap();

    let program = rendered(&program_of(&closed));
    assert_eq!(
        verbs(&program_of(&closed)),
        vec![
            Verb::At,
            Verb::Angle,
            Verb::ArcTo,
            Verb::Fillet,
            Verb::At,
            Verb::Toward,
            Verb::Line,
            Verb::LineTo
        ],
        "the chain this row walks, binders and all"
    );
    assert_eq!(
        program.len(),
        prefixes.len() + 1,
        "one step per verb, and the closing verb records the last one"
    );
    for (i, prefix) in prefixes.iter().enumerate() {
        assert_eq!(
            prefix.len(),
            i + 1,
            "the path had recorded {} verbs, so it holds {} steps",
            i + 1,
            i + 1
        );
        assert_eq!(
            prefix.as_slice(),
            &program[..=i],
            "the prefix after verb {} is the published program's own prefix",
            i + 1
        );
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

/// **The half-disc equator through the lattice's own spelling** (the
/// need `arc_continue` served, re-authored — BOOL-10, Ev's ruling of
/// 2026-09-13: the sixth round already admits adjacent same-carrier
/// arcs as declared tangent joints, so no second verb and no split form
/// is needed). Two quarter arcs on ONE carrier: the first authored
/// (`arc_to(Bulge { .. })`, bulge tan(π/8)), the second
/// `.tangent().tangent_arc_to(p)` — its joint DECLARED, its arc derived
/// from the inherited tangent and the authored target. The table is
/// the one the retired verb produced, bit for bit: the equator vertex
/// is the authored `(0.5, 0)` exactly, and the derived bulge is the
/// value `arc_continue` derived (measured before its removal:
/// `0x3fda827999fcef33`, one ulp above tan(π/8) — the tangent-chord
/// derivation's rounding, which both spellings share). The one
/// difference from the retired verb is the joint: DECLARED now.
#[test]
fn the_equator_through_tangent_arc_to_is_arc_continues_table_bit_for_bit() {
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
        .tangent()
        .tangent_arc_to(p2(0.0, 0.5), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    assert_eq!(
        verbs(&program_of(&closed)),
        vec![
            Verb::At,
            Verb::ArcTo,
            Verb::Tangent,
            Verb::TangentArcTo,
            Verb::LineTo
        ],
    );
    let lowered = pinned(closed);
    assert_eq!(lowered.vertices().len(), 3);
    let v1 = lowered.vertices()[1];
    assert_eq!(
        (v1.pos().x.to_bits(), v1.pos().y.to_bits()),
        (0.5f64.to_bits(), 0.0f64.to_bits()),
        "the equator vertex is the authored point exactly"
    );
    assert_eq!(lowered.vertices()[0].bulge().to_bits(), q.to_bits());
    assert_eq!(
        v1.bulge().to_bits(),
        0x3fda827999fcef33,
        "the derived bulge is the retired verb's, bit for bit (got {:#x})",
        v1.bulge().to_bits()
    );
    assert_eq!(lowered.tangent_joints(), &[1], "the joint is declared");
    validate_ok(&lowered);
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

/// **Every verb names itself in the authoring spelling, and no two
/// verbs name themselves the same.**
///
/// `Verb`'s `Display` is what a surface writing a sentence about the
/// step a person authored renders it through
/// (`crates/viewer/src/sketch.rs`'s preview refusal is the one in the
/// tree), so the two ways it can silently stop being that are the two
/// this row pins, both anchored on [`Verb::ALL`] so a verb the table
/// gains arrives pinned:
///
/// - a word that is the VARIANT IDENTIFIER — what a `stringify!` in
///   the macro, or a plain `Debug` forward, would give. The author
///   wrote `line_to`; a refusal naming `LineTo` is naming the table's
///   coordinate, which is [`ReplayError`]'s sentence and not this one;
/// - a word SHARED with another verb, which a copied literal on a new
///   row gives. Two rows rendering alike makes the refusal ambiguous
///   about which step it is refusing, and nothing else in the tree
///   would notice.
#[test]
fn every_verb_says_its_authoring_spelling_and_says_it_uniquely() {
    let mut said: Vec<(String, Verb)> = Vec::new();
    for verb in Verb::ALL {
        let word = verb.to_string();
        assert_ne!(
            word,
            format!("{verb:?}"),
            "`{verb:?}` renders as its own variant identifier — the authoring \
             spelling is the word a person wrote, and the identifier is what \
             `ReplayError` deliberately renders instead"
        );
        if let Some((_, other)) = said.iter().find(|(w, _)| *w == word) {
            panic!(
                "`{verb:?}` and `{other:?}` both say \"{word}\" — a refusal naming \
                 that word cannot say which step it refused"
            );
        }
        said.push((word, *verb));
    }
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
// The per-radius emission record
//
// A span says which segments a step produced; it cannot say which of
// that step's radii drew which of them, and for a BINDER it cannot say
// anything at all — the radius is on one step and the arc it opens is
// emitted by another. `ReplayStructure::radii` is that fact, and the
// rows below are what pin it: the address is the step whose RADIUS
// ARGUMENT drew the arc, whichever step emitted it.
// ------------------------------------------------------------------

/// The emission record of a chain, in emission order.
fn radii(closed: &ClosedLoop<f64>) -> Vec<(usize, RadiusRole, usize)> {
    closed
        .structure
        .radii
        .iter()
        .map(|e| (e.step, e.role, e.segment))
        .collect()
}

/// **A fillet arc is credited to the step that BINDS its radius**, not
/// to the step that emits it.
///
/// The composite §2c walk is the fixture because it holds three binder
/// shapes at once: a `fillet(r)` whose arc a later arrival emits, a
/// `fillet_arc(r, spec)` that emits its own, and an
/// `arc_fillet(spec, r)` whose arc a later straight arrival emits. Each
/// records at the step holding the radius a reader would edit, and the
/// row asserts the binders' SPANS are empty beside it — which is what
/// makes the credit unobtainable from the spans and the reason this
/// record exists.
///
/// The `Sweep` leg records its carrier radius on its own step; the
/// `arc_fillet(Radius { .. })` extension records none for its incoming
/// side, because an extension emits no segment — it moves the vertex
/// the previous leg already owns.
#[test]
fn a_fillet_arc_is_credited_to_the_step_that_binds_its_radius() {
    use profile::{ArcSide, Center, Radius, Sweep};
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
        radii(&walk),
        vec![
            (2, RadiusRole::Carrier, 0),
            (3, RadiusRole::Fillet, 2),
            (6, RadiusRole::Fillet, 4),
            (7, RadiusRole::Fillet, 6),
        ],
        "each radius names the segment its own arc drew"
    );
    for step in [3, 7] {
        assert!(
            walk.structure.steps[step].is_empty(),
            "step {step} is a BINDER — it emitted {}, so the segment credited to its \
             radius cannot have come from its span",
            walk.structure.steps[step]
        );
    }
    assert!(
        walk.structure.steps[5].start() <= 2 && 2 < walk.structure.steps[5].end(),
        "and segment 2 was emitted by the ARRIVAL step 5, which holds no radius: {}",
        walk.structure.steps[5]
    );
}

/// **A fused step records one emission per radius ROLE**: three radii,
/// three segments, three addresses on one step.
///
/// `arc_fillet_arc(Sweep, r, Radius)` is the whole shape — the incoming
/// carrier the verb authors, the fillet it opens, the arrival carrier
/// it lands on — and each is a different argument of one step. A
/// recording that swapped the incoming and arrival roles, or that
/// shifted a segment by one, would name a different wall for two of
/// the three and reds here.
#[test]
fn a_fused_step_records_one_emission_per_radius_role() {
    use profile::{ArcSide, Radius, Sweep};
    let three = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .line(4.0, Tol::witness())
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
            Tol::witness(),
        )
        .unwrap()
        .at(p2(2.0, 6.0))
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    assert_eq!(
        verbs(&program_of(&three))[4],
        Verb::ArcFilletArc,
        "step 4 is the fused verb the addresses below name"
    );
    assert_eq!(
        radii(&three),
        vec![
            (4, RadiusRole::Carrier, 1),
            (4, RadiusRole::Fillet, 2),
            (4, RadiusRole::Carrier2, 3),
        ],
        "one step, three radius arguments, three segments — each on its own"
    );
    assert!(
        three.structure.steps[4].is_empty(),
        "and the fused step emitted {} of them: a `Radius` arrival binds its anchor \
         and its director on later steps, so all three arcs are emitted by the \
         binder that completes it",
        three.structure.steps[4]
    );
    let emitter = three.structure.steps[6];
    assert_eq!(
        (emitter.start(), emitter.end()),
        (1, 4),
        "step 6 is the director binding that emitted them, and it holds no radius \
         at all — which is why the SPANS alone cannot say which radius drew which"
    );
}

/// **An arc no radius argument authored records nothing.**
///
/// A `Bulge`, `Via` or `Center` spec authors a carrier from a bulge, a
/// through-point or a centre, so its arc has no radius address to name
/// and the record says so by omission rather than by naming an
/// argument the step does not hold. The fused step's own fillet radius
/// is still recorded: the two halves of `arc_fillet(Bulge { .. }, r)`
/// are answered separately.
#[test]
fn an_arc_no_radius_drew_records_no_emission() {
    use profile::{Bulge, Center};
    let bulged = Open
        .at(p2(0.0, 0.0))
        .arc_fillet(
            Bulge {
                p: p2(3.0, 0.0),
                b: 0.2,
            },
            0.2,
            Tol::witness(),
        )
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .to(p2(3.3, 4.0), Tol::witness())
        .unwrap()
        .line_to(p2(0.0, 4.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    assert_eq!(
        radii(&bulged)
            .iter()
            .map(|(step, role, _)| (*step, *role))
            .collect::<Vec<_>>(),
        vec![(1, RadiusRole::Fillet)],
        "the bulge spec's arc names no radius; the fillet's own still does"
    );
    let centred = Open
        .at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), Tol::witness())
        .unwrap()
        .toward(2.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(
        radii(&centred)
            .iter()
            .map(|(step, role, _)| (*step, *role))
            .collect::<Vec<_>>(),
        vec![(3, RadiusRole::Fillet)],
        "nor does a Center arrival's closing carrier run"
    );
}

/// The carrier radius of the segment LEAVING vertex `i`, read back
/// off the stored chord-and-bulge the way a reader classifies it —
/// `r = c(1 + b²) / (4|b|)` for chord `c` and bulge `b = tan(θ/4)`.
/// `None` where the stored segment is straight.
///
/// The emission record names a segment; this is what says the segment
/// it names is the arc the authored radius drew, rather than a
/// neighbour that happens to be an arc too.
fn stored_radius(closed: &ClosedLoop<f64>, i: usize) -> Option<f64> {
    let vs = closed.loop_.vertices();
    let b = vs[i].bulge();
    if b == 0.0 {
        return None;
    }
    let a = vs[i].pos();
    let z = vs[(i + 1) % vs.len()].pos();
    let chord = (z - a).norm_squared().sqrt();
    Some(chord * (1.0 + b * b) / (4.0 * b.abs()))
}

/// **The EXACT-FIT close records its fillet on the closing segment,
/// and the segment it names is stored at the authored radius.**
///
/// `family::resolve_arc_close`'s `else` arm is the branch where the
/// fillet arc consumes the whole arrival side and IS the closing
/// segment. `r = 1.0` is the radius that consumes the line × arc
/// corner's outgoing side exactly
/// (`fillet_stored_tangency::an_exact_outgoing_fit_leaves_its_joint_undeclared_and_still_validates`
/// is the fixture); the same chain at `r = 0.5` leaves carrier run
/// behind and takes the OTHER arm, and the two arms must agree about
/// what a fillet emission looks like.
#[test]
fn the_exact_fit_close_records_its_fillet_on_the_closing_segment() {
    use profile::Center;
    let exact = Open
        .at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), Tol::witness())
        .unwrap()
        .toward(2.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            1.0,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap();
    let last = exact.loop_.vertices().len() - 1;
    assert_eq!(
        radii(&exact),
        vec![(3, RadiusRole::Fillet, last)],
        "the exact fit's one emission is the fillet's, on the CLOSING segment"
    );
    assert_eq!(
        (
            exact.structure.steps[3].start(),
            exact.structure.steps[3].end()
        ),
        (1, last + 1),
        "the arrival step emitted the trimmed incoming run and the closing arc"
    );
    let got = stored_radius(&exact, last).expect("the closing segment is an arc");
    assert!(
        (got - 1.0).abs() < 1e-9,
        "and the segment it names is stored at the authored radius, not {got}"
    );
    let inexact = Open
        .at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), Tol::witness())
        .unwrap()
        .toward(2.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap();
    let [(step, role, segment)] = radii(&inexact)[..] else {
        panic!(
            "the inexact fit records one emission too: {:?}",
            radii(&inexact)
        );
    };
    assert_eq!((step, role), (3, RadiusRole::Fillet));
    let got = stored_radius(&inexact, segment).expect("and it is an arc as well");
    assert!((got - 0.5).abs() < 1e-9, "at its own radius, not {got}");
}

/// **A VIA close credits its fillet to the step that BOUND the
/// radius, not to the step that closes the loop.**
///
/// `Via { q, p: Start }` leaves the seam director free, so the close
/// runs from `ViaArrivalStart::toward_kernel` — which has already
/// RECORDED the director's own step. There `current_step()` and the
/// binder's `bound_at` are two different steps, and the director's
/// holds no radius at all. Every other close in this crate authors
/// `Center { p: Start }`, where the two coincide and an address read
/// off `current_step()` passes.
#[test]
fn a_via_close_credits_its_fillet_to_the_binder_not_the_closing_step() {
    use profile::Via;
    let h = 2.0_f64.sqrt();
    let closed = Open
        .at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), Tol::witness())
        .unwrap()
        .toward(2.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Via {
                q: p2(h, h),
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap()
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap();
    assert_eq!(
        closed.structure.steps.len(),
        5,
        "the director binding is a step of its own"
    );
    let [(step, role, segment)] = radii(&closed)[..] else {
        panic!("one radius argument, one arc: {:?}", radii(&closed));
    };
    assert_eq!(
        (step, role),
        (3, RadiusRole::Fillet),
        "step 3 is the `fillet_arc` that authored the radius; step 4 is the \
         director binding that emitted the arcs and holds none"
    );
    assert!(
        closed.structure.steps[4].len() > 1,
        "step 4 emitted the fillet arc AND the Via arc — {} of them",
        closed.structure.steps[4]
    );
    let got = stored_radius(&closed, segment).expect("the named segment is an arc");
    assert!(
        (got - 0.5).abs() < 1e-9,
        "and it is the FILLET arc, not the Via carrier's: {got}"
    );
}

/// **A radius-bearing LEG records its carrier on its own step,
/// wherever the leg sits** — first emitting step, middle, or last
/// before the closer.
///
/// `family::arc_to_kernel` is the one emission site that addresses
/// `current_step()` rather than a binder's `bound_at`, and it is
/// correct exactly because the two coincide for a plain leg. A leg at
/// three positions of one chain is what measures that: each `Sweep`
/// and `ArcLen` leg's emission names the step that authored it, and
/// the straight legs between them contribute nothing.
#[test]
fn a_radius_bearing_leg_records_its_own_step_at_every_position() {
    use profile::{ArcLen, ArcSide, Sweep};
    let walk = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .arc_to(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.5,
            },
            Tol::witness(),
        )
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .tangent()
        .arc_to(
            ArcLen {
                r: 3.0,
                side: ArcSide::Left,
                len: 1.5,
            },
            Tol::witness(),
        )
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .tangent()
        .arc_to(
            Sweep {
                r: 1.5,
                side: ArcSide::Left,
                angle: 0.7,
            },
            Tol::witness(),
        )
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    assert_eq!(
        radii(&walk),
        vec![
            (2, RadiusRole::Carrier, 0),
            (5, RadiusRole::Carrier, 2),
            (8, RadiusRole::Carrier, 4),
        ],
        "each leg's carrier radius names its own step and its own segment"
    );
    for (want, (_, _, segment)) in [2.0, 3.0, 1.5].into_iter().zip(radii(&walk)) {
        let got = stored_radius(&walk, segment).expect("the named segment is an arc");
        assert!(
            (got - want).abs() < 1e-6,
            "segment {segment} is stored at {got}, not the {want} its address names"
        );
    }
    let program: Vec<_> = walk.program.clone();
    let (_, recorded) =
        profile::replay_recording(&program, Tol::witness()).expect("the walk replays recording");
    assert_eq!(recorded.radii, walk.structure.radii);
    profile::replay_guided(&program, &recorded, Tol::witness())
        .expect("and replays guided against its own record");
}

/// **No two emissions of one loop name the same segment.**
///
/// `editor-core`'s `eval::wire::edge_radii` reads the per-edge door's
/// answer with `.find(|(e, _)| *e == want)` — the FIRST pair naming a
/// canonical segment wins and any second is dropped without a word.
/// That is sound only while one segment carries at most one emission,
/// and nothing in the types says so, so it is measured here over
/// every chain the coverage corpus holds.
#[test]
fn no_two_emissions_of_one_loop_name_the_same_segment() {
    for (i, closed) in coverage_corpus().iter().enumerate() {
        let mut seen: Vec<usize> = closed.structure.radii.iter().map(|e| e.segment).collect();
        let n = seen.len();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(
            seen.len(),
            n,
            "chain {i} records two radii on one segment, which `edge_radii`'s \
             first-match read would silently drop one of: {:?}",
            closed.structure.radii
        );
        for e in &closed.structure.radii {
            assert!(
                e.segment < closed.loop_.vertices().len(),
                "chain {i}: {e} names a segment the loop does not have"
            );
        }
    }
}

/// **Every radius ROLE the record vocabulary declares is reached by
/// the corpus**, so the guided fence's positive half — every corpus
/// program replayed guided against its own record
/// (`guided_replay::guided_replay_at_f64_reproduces_plain_replay_bitwise`)
/// — reproduces each of them.
///
/// The role travels inside an emission, so the verb and arc-mode
/// censuses above cannot see it: `Carrier2` is recorded only where a
/// radius-bearing ARRIVAL spec resolves, and `ArrivalSpec` is
/// implemented for `Radius`, `Via` and `Center` alone — so exactly one
/// shape in the language reaches it, and a corpus without that shape
/// leaves the guided arm's `Carrier2` comparison unexercised while
/// every row still passes. Anchored on [`RadiusRole::ALL`], so a role
/// the vocabulary gains cannot fall behind it.
#[test]
fn every_radius_role_is_reached_by_the_corpus() {
    let seen: Vec<RadiusRole> = coverage_corpus()
        .iter()
        .flat_map(|c| c.structure.radii.iter().map(|e| e.role))
        .collect();
    let missing: Vec<&RadiusRole> = RadiusRole::ALL
        .iter()
        .filter(|r| !seen.contains(r))
        .collect();
    assert!(
        missing.is_empty(),
        "these radius roles are declared but never recorded by the corpus: \
         {missing:?} — the guided fence reproduces only the roles it reaches, \
         so add a chain to `coverage_corpus` that authors them"
    );
}

// ------------------------------------------------------------------
// Pieces: which step and role each segment is.
// ------------------------------------------------------------------

/// The pieces a closed chain's segments are, as `(step, role)`.
fn pieces_of(closed: &ClosedLoop<f64>) -> Vec<(usize, profile::PieceRole)> {
    closed
        .structure
        .pieces
        .iter()
        .map(|p| (p.step, p.role))
        .collect()
}

/// EDIT's zero-fit chain: `at`, `toward(+x)`, `fillet(r)`,
/// `toward(+y)`, `to (2, 2)`, `line_to(0, 2)`, `line_to(Start)`.
fn zero_fit_chain(r: f64) -> ClosedLoop<f64> {
    let t = Tol::witness();
    Open.at(p2(0.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .to(p2(2.0, 2.0), t)
        .unwrap()
        .line_to(p2(0.0, 2.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
}

/// **A fillet draws its run in, its arc and its run out, all credited
/// to the step its radius is authored on.** The far end's own leg is
/// the run out drawn as one segment with it; the fillet is authored
/// first, so the segment is the fillet's run out and the far-end step
/// draws nothing of its own.
#[test]
fn a_fillet_draws_its_run_in_its_arc_and_its_run_out() {
    use profile::PieceRole::{Arc, Leg, RunIn, RunOut};
    let closed = zero_fit_chain(0.3);
    assert_eq!(
        pieces_of(&closed),
        vec![(2, RunIn), (2, Arc), (2, RunOut), (5, Leg), (6, Leg)]
    );
    pinned(closed);
}

/// **A run a `Zero` fit suppresses is no piece, and nothing else moves**:
/// at `r = 2` both runs have no length, the arc spans the whole corner,
/// and every other segment keeps the piece it was.
#[test]
fn a_zero_fit_run_is_no_piece_and_every_other_keeps_its_own() {
    use profile::PieceRole::{Arc, Leg};
    let closed = zero_fit_chain(2.0);
    assert_eq!(pieces_of(&closed), vec![(2, Arc), (5, Leg), (6, Leg)]);
    pinned(closed);
}

/// **A leg a fillet's run continues keeps the segment**: the `line`
/// is authored before the fillet, so the extended segment is the
/// line's leg and the fillet's run in is not drawn.
#[test]
fn a_leg_a_run_continues_keeps_the_segment() {
    use profile::PieceRole::{Arc, Leg, RunOut};
    let t = Tol::witness();
    let closed = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .fillet(0.3, t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .to(p2(2.0, 2.0), t)
        .unwrap()
        .line_to(p2(0.0, 2.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    assert_eq!(
        pieces_of(&closed),
        vec![(2, Leg), (3, Arc), (3, RunOut), (6, Leg), (7, Leg)]
    );
    pinned(closed);
}

/// **A fused verb's authored arc carrier is its run**: `arc_fillet`'s
/// incoming arc is its run in, drawn by the fused step itself.
#[test]
fn a_fused_verbs_incoming_arc_is_its_run_in() {
    use profile::Bulge;
    use profile::PieceRole::{Arc, Leg, RunIn, RunOut};
    let t = Tol::witness();
    let closed = Open
        .at(p2(0.0, 0.0))
        .arc_fillet(
            Bulge {
                p: p2(3.0, 0.0),
                b: 0.2,
            },
            0.2,
            t,
        )
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .to(p2(3.3, 4.0), t)
        .unwrap()
        .line_to(p2(0.0, 4.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    assert_eq!(
        pieces_of(&closed),
        vec![(1, RunIn), (1, Arc), (1, RunOut), (4, Leg), (5, Leg)]
    );
    pinned(closed);
}

/// **A circle is two pieces, a `circle_split` is `n`**: the one step's
/// `Piece(k)` for its segment `k`.
#[test]
fn a_carrier_form_draws_piece_k_for_its_segment_k() {
    use profile::PieceRole::Piece;
    let t = Tol::witness();
    let circle = profile::circle(p2(0.0, 0.0), 1.0, t).unwrap();
    assert_eq!(pieces_of(&circle), vec![(0, Piece(0)), (0, Piece(1))]);
    let split = profile::circle_split(p2(0.0, 0.0), 1.0, 3, 0.0, t).unwrap();
    assert_eq!(
        pieces_of(&split),
        vec![(0, Piece(0)), (0, Piece(1)), (0, Piece(2))]
    );
    pinned(circle);
    pinned(split);
}
