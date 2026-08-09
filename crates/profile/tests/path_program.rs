//! **LIB-SWITCH-P: profiles-as-programs v2, the profile half.**
//!
//! The mandated differential pin (spec §3d) rides on `common::pinned`,
//! which every closing verb in the corpus already funnels through — so
//! `path_differential.rs`, `path_property.rs` and this file together pin
//! record→replay bit-identity over the WHOLE typed-surface corpus, not a
//! sample. This suite adds what that blanket cannot express:
//!
//! - the four DEDICATED semantic rows §3d names by anchor — far-end
//!   zero-fit declaration suppression, the seam fillet's retrim of
//!   vertex 0, the G2 arrival family, and `circle`'s two-pole lowering;
//! - the tour's authoring-site SHAPES, replicated here so the pin covers
//!   them without touching `demos/` (the §3e fence);
//! - the driver's own refusal surface: the Transition class (corrupt
//!   file — no authoring surface can produce it) against the Path class
//!   (legal at rest, geometry refuses under this binding);
//! - a generator: random legal chains, recorded and replayed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{assert_bit_identical, pinned};
use geom_core::{Point2, Tolerance};
use profile::{
    ArcSweep, ClosedLoop, Open, PathError, ProfileLoop, ReplayError, ReplayErrorKind, Start, Step,
    Target, TipState, Verb, replay,
};
use proptest::prelude::*;

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

// ------------------------------------------------------------------
// §3d — the four dedicated semantic rows
// ------------------------------------------------------------------

/// **Far-end zero-fit declaration suppression.** The exact-fit `.to(p)`
/// branch emits no straight run and leaves the fillet arc's outgoing
/// joint UNDECLARED. Nothing in the step vocabulary records that — it is
/// a consequence of the binder — so the replay reproduces it only
/// because the driver calls the binder. Pinned both ways: the joint set
/// is identical, and it does NOT contain the arc's outgoing joint.
#[test]
fn far_end_zero_fit_suppression_survives_replay() {
    let closed = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .unwrap()
        .line_to(p2(3.0, 1.0))
        .unwrap()
        .toward(-1.0, 0.0)
        .unwrap()
        .fillet(0.5)
        .unwrap()
        .toward(0.0, 1.0)
        .unwrap()
        .to(p2(1.0, 1.5))
        .unwrap()
        .line_to(p2(0.0, 3.0))
        .unwrap()
        .line_to(Start)
        .unwrap();
    assert_eq!(
        verbs(&program_of(&closed)),
        vec![
            Verb::At,
            Verb::LineTo,
            Verb::LineTo,
            Verb::Toward,
            Verb::Fillet,
            Verb::Toward,
            Verb::FarEndTo,
            Verb::LineTo,
            Verb::LineTo,
        ],
        "the far-end anchor is its own verb, not an `.at` plus a leg"
    );
    let lowered = pinned(closed);
    // The suppression itself: the exact fit absorbed the anchor, so the
    // arc's outgoing joint carries no declaration.
    assert!(
        !lowered.tangent_joints.is_empty(),
        "the fillet's INCOMING joint is still declared"
    );
    assert_eq!(
        lowered.tangent_joints.len(),
        1,
        "exactly one declaration: the outgoing side was suppressed, and \
         replaying the program must not resurrect it"
    );
    validate_ok(&lowered);
}

/// **The seam fillet retrims vertex 0.** `.to(Start)` makes the fillet
/// arc the CLOSING segment and retrims the provisional entry vertex to
/// that arc's end — vertex 0 is not where the author started writing.
/// The program records only `CloseTo`; the retrim is the binder's, and
/// the replay inherits it bit-for-bit.
#[test]
fn seam_fillet_retrim_of_vertex_zero_survives_replay() {
    let (east, north) = (0.0_f64, std::f64::consts::FRAC_PI_2);
    let (west, south) = (std::f64::consts::PI, -north);
    let entry = p2(0.0, -1.0);
    let closed = Open
        .at(entry)
        .angle(east)
        .unwrap()
        .fillet(0.25)
        .unwrap()
        .at(p2(1.0, 0.0))
        .unwrap()
        .angle(north)
        .unwrap()
        .fillet(0.25)
        .unwrap()
        .at(p2(0.0, 1.0))
        .unwrap()
        .angle(west)
        .unwrap()
        .fillet(0.25)
        .unwrap()
        .at(p2(-1.0, 0.0))
        .unwrap()
        .angle(south)
        .unwrap()
        .fillet(0.25)
        .unwrap()
        .to(Start)
        .unwrap();
    assert_eq!(
        verbs(&program_of(&closed)).last().copied(),
        Some(Verb::CloseTo)
    );
    let lowered = pinned(closed);
    let v0 = lowered.vertices[0].pos;
    assert!(
        (v0 - entry).norm_squared().sqrt() > Tolerance::get().eps,
        "vertex 0 must have been RETRIMMED off the authored entry point \
         (got {v0:?}, entry {entry:?})"
    );
    assert_eq!(lowered.vertices.len(), 8);
    assert_eq!(lowered.tangent_joints.len(), 8);
    validate_ok(&lowered);
}

fn validate_ok(lp: &ProfileLoop<f64>) {
    use profile::{Profile, SketchPlane};
    Profile::new(SketchPlane::xy(), vec![lp.clone()])
        .validate(Tolerance::get())
        .expect("the replayed loop validates");
}

/// **The G2 arrival family**: the entry bound on a carrier
/// (`Open.at_on`), the fillet arrival bound on a carrier
/// (`PartialPath::at_on`), and the carrier close (`to_on`). All three
/// derive a direction from the carrier rather than storing one, so the
/// steps keep only `(p, centre, winding)` and the replay re-derives.
#[test]
fn g2_arrival_family_records_only_its_authored_carrier_data() {
    // The rocker eye's shape: entry on the right lobe, one fillet, close
    // on the left lobe (a genuine two-carrier junction at the tip).
    let tip = 0.75f64.sqrt();
    let closed = Open
        .at_on(p2(0.0, -tip), p2(-0.5, 0.0), ArcSweep::Ccw)
        .unwrap()
        .fillet(0.35)
        .unwrap()
        .to_on(Start, p2(0.5, 0.0), ArcSweep::Ccw)
        .unwrap();
    assert_eq!(
        verbs(&program_of(&closed)),
        vec![Verb::AtOn, Verb::Fillet, Verb::CloseToOn]
    );
    match closed.program[0] {
        Step::AtOn { p, centre, winding } => {
            assert_eq!(p.x.to_bits(), 0.0_f64.to_bits());
            assert_eq!(p.y.to_bits(), (-tip).to_bits());
            assert_eq!(centre.x.to_bits(), (-0.5_f64).to_bits());
            assert_eq!(winding, ArcSweep::Ccw);
        }
        ref other => panic!("expected AtOn, got {other:?}"),
    }
    validate_ok(&pinned(closed));

    // The fillet-arrival `at_on` — the third member of the family. It is
    // a MID-chain binder, so it has no closing verb of its own and no
    // corpus chain ends on it; the driver arm is pinned instead by
    // replaying the program that reaches it and reading the state the
    // arm left the tip in (a Directed tip over a plain position).
    let centre = p2(2.0, -2.0);
    let radius = 8.0f64.sqrt();
    let theta = std::f64::consts::FRAC_PI_4 + 0.9;
    let anchor = p2(
        centre.x + radius * theta.cos(),
        centre.y + radius * theta.sin(),
    );
    Open.at(p2(0.0, 0.0))
        .toward(1.0, 0.0)
        .unwrap()
        .fillet(0.3)
        .unwrap()
        .at_on(anchor, centre, ArcSweep::Ccw)
        .expect("the typed surface accepts this arrival");
    let program = vec![
        Step::At(p2(0.0, 0.0)),
        Step::Toward { dx: 1.0, dy: 0.0 },
        Step::Fillet { radius: 0.3 },
        Step::AtOn {
            p: anchor,
            centre,
            winding: ArcSweep::Ccw,
        },
    ];
    match replay(&program) {
        Err(ReplayError {
            step: 4,
            kind:
                ReplayErrorKind::Transition {
                    state: TipState::DirectedPlain,
                    verb: None,
                },
        }) => {}
        other => panic!(
            "the arrival `at_on` arm must run and leave a DirectedPlain tip              that the program then fails to close; got {other:?}"
        ),
    }
}

/// **`circle`'s two-pole lowering.** The primitive is a ONE-STEP
/// complete-loop program; the ±x poles and the unit bulges are its
/// private lowering, and the replay reproduces them from `(centre, r)`
/// alone.
#[test]
fn circle_is_a_one_step_program_that_replays_to_its_two_poles() {
    let closed = profile::circle(p2(1.5, -2.25), 0.75).unwrap();
    assert_eq!(verbs(&program_of(&closed)), vec![Verb::Circle]);
    assert_eq!(
        closed.program.len(),
        1,
        "a circle program is exactly one step"
    );
    let lowered = pinned(closed);
    assert_eq!(lowered.vertices.len(), 2);
    assert_eq!(lowered.vertices[0].pos.x.to_bits(), 2.25_f64.to_bits());
    assert_eq!(lowered.vertices[1].pos.x.to_bits(), 0.75_f64.to_bits());
    assert_eq!(lowered.vertices[0].bulge.to_bits(), 1.0_f64.to_bits());
    assert!(
        lowered.tangent_joints.is_empty(),
        "same-carrier joints declare nothing — there is no tangency to claim"
    );
    validate_ok(&lowered);
}

// ------------------------------------------------------------------
// The tour's authoring-site SHAPES
//
// §3d names "the tour authoring sites" as pin corpus; §3e fences PR-A
// to crates/profile. The scenes' chain SHAPES are replicated here (the
// tour files themselves only gained the mechanical `.into()` the
// ClosedLoop pair forces), so every verb combination the tour authors
// is recorded and replayed under this suite.
// ------------------------------------------------------------------

/// `paths::path_polygon` — the shape every polygonal scene funnels
/// through: `at`, N × `line_to(point)`, `line_to(Start)`.
#[test]
fn tour_polygon_shape_replays() {
    let pts = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.5), (0.7, 2.2), (0.0, 1.0)];
    let (first, rest) = pts.split_first().unwrap();
    let (second, rest) = rest.split_first().unwrap();
    let mut tip = Open
        .at(p2(first.0, first.1))
        .line_to(p2(second.0, second.1))
        .unwrap();
    for q in rest {
        tip = tip.line_to(p2(q.0, q.1)).unwrap();
    }
    let closed = tip.line_to(Start).unwrap();
    assert_eq!(closed.program.len(), pts.len() + 1);
    validate_ok(&pinned(closed));
}

/// `bodies::bracket` — the far-end anchor in a production chain
/// (`toward` + `fillet` + `toward` + `.to(p)`), positive-fit branch.
#[test]
fn tour_bracket_shape_replays() {
    let closed = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .unwrap()
        .line_to(p2(3.0, 1.0))
        .unwrap()
        .toward(-1.0, 0.0)
        .unwrap()
        .fillet(0.5)
        .unwrap()
        .toward(0.0, 1.0)
        .unwrap()
        .to(p2(1.0, 3.0))
        .unwrap()
        .line_to(p2(0.0, 3.0))
        .unwrap()
        .line_to(Start)
        .unwrap();
    validate_ok(&pinned(closed));
}

/// `bodies::vase` — `arc_via`'s through-point binding in a revolve
/// profile. The via-point is stored VERBATIM; the bulge is re-derived.
#[test]
fn tour_vase_shape_replays_with_the_bulge_rederived() {
    let closed = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(1.2, 0.0))
        .unwrap()
        .line_to(p2(1.2, 0.3))
        .unwrap()
        .arc_via(p2(1.3, 0.8), p2(0.5, 2.0))
        .unwrap()
        .line_to(p2(0.9, 2.5))
        .unwrap()
        .line_to(p2(0.0, 2.5))
        .unwrap()
        .line_to(Start)
        .unwrap();
    match closed.program[3] {
        Step::ArcVia {
            via,
            target: Target::Point(t),
        } => {
            assert_eq!(via.x.to_bits(), 1.3_f64.to_bits());
            assert_eq!(via.y.to_bits(), 0.8_f64.to_bits());
            assert_eq!(t.x.to_bits(), 0.5_f64.to_bits());
        }
        ref other => panic!("expected ArcVia with an authored via, got {other:?}"),
    }
    validate_ok(&pinned(closed));
}

/// `lily::lantern` — `arc_center`'s centre+winding binding.
#[test]
fn tour_lantern_shape_replays_with_the_bulge_rederived() {
    // The belly rides a globe whose centre is on the axis: the tip and
    // the target are equidistant from it, which is what `arc_center`
    // requires and what the scene authors.
    let centre = p2(0.0, 1.1);
    let closed = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(1.1, 1.1))
        .unwrap()
        .arc_center(centre, p2(0.0, 2.2), ArcSweep::Ccw)
        .unwrap()
        .line_to(Start)
        .unwrap();
    match closed.program[2] {
        Step::ArcCenter {
            centre,
            winding: ArcSweep::Ccw,
            ..
        } => assert_eq!(centre.y.to_bits(), 1.1_f64.to_bits()),
        ref other => panic!("expected ArcCenter with an authored centre, got {other:?}"),
    }
    validate_ok(&pinned(closed));
}

/// `diefillet::pip` — the semicircular pip: one authored-bulge arc leg
/// and a straight seam.
#[test]
fn tour_pip_shape_replays() {
    let closed = Open
        .at(p2(0.0, -0.35))
        .arc_to(p2(0.0, 0.35), 1.0)
        .unwrap()
        .line_to(Start)
        .unwrap();
    match closed.program[1] {
        Step::ArcTo { bulge, .. } => assert_eq!(bulge.to_bits(), 1.0_f64.to_bits()),
        ref other => panic!("expected ArcTo with its authored bulge, got {other:?}"),
    }
    validate_ok(&pinned(closed));
}

// ------------------------------------------------------------------
// The driver's refusal surface: two classes, deliberately different
// ------------------------------------------------------------------

/// Asserts a program refuses as a LATTICE violation at `step`, from
/// `state`, on `verb` (`None` = the program simply ended there).
#[track_caller]
fn assert_transition(program: &[Step<f64>], step: usize, state: TipState, verb: Option<Verb>) {
    match replay(program) {
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
    replay(&square(0.25)).expect("r = 0.25 elaborates");
    let refused = replay(&square(5.0));
    match refused {
        Err(ReplayError {
            kind: ReplayErrorKind::Path(_),
            ..
        }) => {}
        other => panic!("r = 5.0 must refuse in the PATH class, got {other:?}"),
    }

    // The sign gates are the same class, carried straight through.
    match replay(&[Step::Circle {
        centre: p2(0.0, 0.0),
        radius: 0.0,
    }]) {
        Err(ReplayError {
            step: 0,
            kind: ReplayErrorKind::Path(PathError::NonpositiveCircleRadius { .. }),
        }) => {}
        other => panic!("a zero radius must refuse NonpositiveCircleRadius, got {other:?}"),
    }
    match replay(&[
        Step::At(p2(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Fillet { radius: -1.0 },
    ]) {
        Err(ReplayError {
            step: 2,
            kind: ReplayErrorKind::Path(PathError::NonpositiveFilletRadius { .. }),
        }) => {}
        other => panic!("a negative fillet radius must refuse typed, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// The generator (§3d): random legal chains, recorded and replayed
// ------------------------------------------------------------------

/// A convex polygon on a circle of radius `r`, with per-vertex angular
/// jitter — every vertex authored, every leg a `line_to`.
fn convex_ring(n: usize, r: f64, jitter: &[f64]) -> Vec<Point2<f64>> {
    (0..n)
        .map(|i| {
            let base = std::f64::consts::TAU * (i as f64) / (n as f64);
            let th = base + jitter.get(i).copied().unwrap_or(0.0);
            p2(r * th.cos(), r * th.sin())
        })
        .collect()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(96))]

    /// Random sharp polygons: the recorded program replays bit-identical.
    #[test]
    fn random_polygon_chains_replay_bit_identically(
        n in 3usize..9,
        r in 0.5f64..4.0,
        jitter in prop::collection::vec(-0.3f64..0.3, 3..9),
    ) {
        let pts = convex_ring(n, r, &jitter);
        let mut tip = Open.at(pts[0]).line_to(pts[1]).unwrap();
        for q in &pts[2..] {
            tip = tip.line_to(*q).unwrap();
        }
        let closed = tip.line_to(Start).unwrap();
        prop_assert_eq!(closed.program.len(), n + 1);
        let replayed = replay(&closed.program).unwrap();
        assert_bit_identical(&closed.loop_, &replayed);
    }

    /// Random ARC chains: every leg an authored-bulge arc, so the bulge
    /// path through `arc_to` is exercised alongside the seam.
    #[test]
    fn random_arc_chains_replay_bit_identically(
        n in 3usize..7,
        r in 0.6f64..3.0,
        bulge in -0.4f64..0.4,
    ) {
        let pts = convex_ring(n, r, &[]);
        let mut tip = Open.at(pts[0]).arc_to(pts[1], bulge).unwrap();
        for q in &pts[2..] {
            tip = tip.arc_to(*q, bulge).unwrap();
        }
        let closed = tip.arc_to(Start, bulge).unwrap();
        let replayed = replay(&closed.program).unwrap();
        assert_bit_identical(&closed.loop_, &replayed);
    }

    /// Random FILLETED chains: the seam fillet, the interior fillets and
    /// their derived corners all ride the driver.
    #[test]
    fn random_filleted_squares_replay_bit_identically(
        half in 0.8f64..3.0,
        frac in 0.05f64..0.4,
    ) {
        let r = half * frac;
        let quarter = std::f64::consts::FRAC_PI_2;
        let closed = Open
            .at(p2(0.0, -half))
            .angle(0.0)
            .unwrap()
            .fillet(r)
            .unwrap()
            .at(p2(half, 0.0))
            .unwrap()
            .angle(quarter)
            .unwrap()
            .fillet(r)
            .unwrap()
            .at(p2(0.0, half))
            .unwrap()
            .angle(2.0 * quarter)
            .unwrap()
            .fillet(r)
            .unwrap()
            .at(p2(-half, 0.0))
            .unwrap()
            .angle(-quarter)
            .unwrap()
            .fillet(r)
            .unwrap()
            .to(Start)
            .unwrap();
        let replayed = replay(&closed.program).unwrap();
        assert_bit_identical(&closed.loop_, &replayed);
    }

    /// Random circles: the one-step form, over the whole radius range.
    #[test]
    fn random_circles_replay_bit_identically(
        cx in -5.0f64..5.0,
        cy in -5.0f64..5.0,
        r in 0.01f64..12.0,
    ) {
        let closed = profile::circle(p2(cx, cy), r).unwrap();
        prop_assert_eq!(closed.program.len(), 1);
        let replayed = replay(&closed.program).unwrap();
        assert_bit_identical(&closed.loop_, &replayed);
    }
}
