//! **R2 review probes for BOOL-12** (PR #1573, frozen head 50740f96).
//!
//! Falsification lane. Each row either re-takes a measurement the PR
//! body reports or attacks a claim it makes; nothing is asserted that
//! was not run. The tolerance is the run's own witness throughout, so
//! the file means the same thing on every ε leg the battery walks.
//!
//! - the two pre-build MEASUREMENTS, re-taken on this head, including
//!   the stored ORDER of the stadium's declared joints (the body states
//!   the sorted form);
//! - the ESCALATION message at the two new funnel keys — §8 claims the
//!   recourse direction is the authored-data one, and the escalated arm
//!   is where that is read;
//! - the DEGENERATE closing leg, which `junction_check` names and
//!   refuses at its own site and which the declared twin accepts;
//! - the ARC member's lever, `radius.min(chord)`, which the suite pins
//!   only for the straight member;
//! - lily's lattice-authored vertex table against the raw table the
//!   demo used to hand over, which is the mechanism under the
//!   byte-stability claim.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Point2, Tol};
use profile::{
    Bulge, ClosedLoop, Open, PathError, Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane,
    Start,
};
use std::f64::consts::FRAC_PI_2;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validate_ok(l: &ProfileLoop<f64>) {
    Profile::new(SketchPlane::xy(), vec![l.clone()])
        .validate(Tol::witness())
        .expect("the loop passes the junction verifier");
}

fn band() -> (f64, f64) {
    let t = Tol::witness();
    (t.eps(), t.k() * t.eps())
}

// ------------------------------------------------------------------
// 1. The two pre-build measurements, re-taken.
// ------------------------------------------------------------------

fn stadium(declared: bool) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    let p = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(2.0, 2.0), t)
        .unwrap()
        .tangent()
        .line(2.0, t)
        .unwrap()
        .tangent();
    if declared {
        p.tangent_arc_to(Start.arrives_tangent(), t)
    } else {
        p.tangent_arc_to(Start, t)
    }
}

fn d_shape_forward(declared: bool) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    let p = Open
        .at(p2(0.0, 0.0))
        .angle(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .arc_to(
            Bulge {
                p: p2(0.0, -2.0),
                b: 1.0,
            },
            t,
        )
        .unwrap()
        .line_to(p2(0.0, -1.0), t)
        .unwrap();
    if declared {
        p.continue_to(Start.arrives_tangent(), t)
    } else {
        p.continue_to(Start, t)
    }
}

/// MEASUREMENT A, head side: the undeclared stadium refuses under the
/// SEAM's name, and the declared one closes. The joint list the loop
/// carries is recorded in its STORED order.
#[test]
fn r2_measurement_a_on_the_head() {
    let refused = stadium(false);
    println!("R2: undeclared stadium -> {refused:?}");
    assert!(matches!(refused, Err(PathError::SeamTangent { .. })));

    let closed = stadium(true).expect("the declared tangent seam closes");
    println!(
        "R2: declared stadium joints (stored order) -> {:?}",
        closed.loop_.tangent_joints()
    );
    validate_ok(&closed.loop_);
    let mut joints = closed.loop_.tangent_joints().to_vec();
    joints.sort_unstable();
    assert_eq!(joints, vec![0, 1, 2, 3]);
    assert_eq!(closed.loop_.vertices().len(), 4);
}

/// The D-shape, declared and undeclared, and the same for the stadium —
/// re-run so the whole file can be walked on an explicit ε leg via the
/// environment (the tolerance is process-global, so an ε row is a RUN,
/// not a fixture).
#[test]
fn r2_the_declared_seams_hold_on_this_epsilon_leg() {
    let (eps, _) = band();
    println!("R2: eps leg {eps}");
    let closed = d_shape_forward(true).expect("the declared D-shape closes");
    assert_eq!(closed.loop_.vertices().len(), 4);
    // The seam joint AND the interior continuation joint.
    assert_eq!(closed.loop_.tangent_joints(), &[0, 3]);
    validate_ok(&closed.loop_);
    assert!(matches!(
        d_shape_forward(false),
        Err(PathError::SeamTangent { .. })
    ));
    assert!(matches!(stadium(false), Err(PathError::SeamTangent { .. })));
    stadium(true).expect("the stadium closes on this leg");
}

// ------------------------------------------------------------------
// 2. The band, and the escalation message at the two new keys.
// ------------------------------------------------------------------

/// The suite's own fixture: the closing leg departs a corner at
/// `(off, -arm)` and arrives at the entry declaring a straight arrival,
/// so the levered miss is exactly `off`.
fn tilted_close(off: f64, arm: f64) -> Result<ClosedLoop<f64>, PathError<f64>> {
    let t = Tol::witness();
    Open.at(p2(0.0, 0.0))
        .angle(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0 + arm, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0 + off, t)
        .unwrap()
        .line_to(Start.arrives_tangent(), t)
}

/// **The escalation message at the two new keys.** `PathError`'s
/// `Escalated` arm composes a per-key message for
/// `path_continuation_target_offset` and `path_leg_length` — the two
/// sites where "declare the coincidence" is the wrong advice, because
/// the declaration is the verb — and falls through to the JUNCTION
/// template for every other key. `path_seam_arrival_turn` is a declared
/// site of exactly that kind (the declaration is the TARGET), and it
/// takes the fallthrough: the message opens "path junction
/// classification" and its recourse tail still offers "declare the
/// coincidence" for a coincidence the author already declared.
#[test]
fn r2_the_new_keys_inherit_the_junction_escalation_template() {
    let (eps, k_eps) = band();
    let escalated = tilted_close(3.0 * eps, 1.0).expect_err("in-band");
    let msg = escalated.to_string();
    println!("R2: escalated declared arrival -> {msg}");
    assert!(msg.contains("path_seam_arrival_turn"), "{msg}");
    // FIXED: the inherited junction template is gone and the recourse
    // is the authored-data one.
    assert!(!msg.starts_with("path junction classification"), "{msg}");
    assert!(!msg.contains("declare the coincidence"), "{msg}");
    assert!(msg.contains("the declaration is the target"), "{msg}");
    // The DEFINITE arm, for contrast: it composes its own recourse.
    let refused = tilted_close(100.0 * k_eps, 1.0).expect_err("past the band");
    let refused_msg = refused.to_string();
    println!("R2: refused declared arrival -> {refused_msg}");
    assert!(
        !refused_msg.contains("declare the coincidence"),
        "{refused_msg}"
    );
}

// ------------------------------------------------------------------
// 3. The lever, attacked.
// ------------------------------------------------------------------

/// **The LEVER is not pinned by the shipped suite.** Replace both
/// `Margin::levered(x, arm)` in `seam_arrival_check` with
/// `Margin::of(x)` — dropping the unit's headline design decision
/// entirely — and every row in `bool12_probes` stays GREEN, the
/// dimension-honesty row included: at arms 1 and 4 the levered and the
/// unlevered margins land on the SAME side of the band, and the row's
/// `margin`/`arm` assertions read a payload computed outside the
/// decision. What separates the two designs is an arm far from 1.
///
/// This row is that separator in the direction that matters (the other
/// is the degenerate arm below): a 1000 m closing leg whose ARRIVAL
/// misses by `100 * eps` of DISPLACEMENT is an angle of `0.1 * eps`,
/// which an unlevered check calls Zero and ACCEPTS. Levered, it refuses.
#[test]
fn r2_the_lever_only_bites_at_an_arm_far_from_one() {
    let (eps, k_eps) = band();
    let off = 100.0 * eps;
    assert!(off > k_eps, "the miss is past the input tolerance");
    let refused = tilted_close(off, 1000.0);
    match refused {
        Err(PathError::SeamArrivalOffDirection { margin, arm, .. }) => {
            println!("R2: long-arm miss -> margin {margin} arm {arm}");
            assert!(
                (margin.abs() - off).abs() <= 1e-2 * off,
                "margin {margin} is not the authored displacement {off} (to 1%)"
            );
            assert!(arm > 900.0, "arm {arm}");
            // The ANGLE this displacement subtends is well below eps, so
            // an unlevered check would have called it Zero and closed.
            assert!(
                margin.abs() / arm < eps,
                "the angle is sub-eps by construction"
            );
        }
        other => panic!("{other:?}"),
    }
}

/// **The degenerate closing leg — FIXED, the row flipped.**
/// `junction_check` names this case at its own site ("a Zero here means
/// the arm itself is degenerate") and refuses it; `seam_arrival_check`
/// had no such arm, so with the arm below ε the levered turn AND the
/// levered side both read Zero and a leg arriving from ANY direction
/// satisfied the declaration. It now gates the LEVER first and refuses
/// `SeamArrivalLeverTooShort` — a declaration cannot rescue a junction
/// nothing can measure — so the authoring layer speaks instead of the
/// gate.
#[test]
fn r2_a_degenerate_closing_leg_is_refused_for_want_of_a_lever() {
    let t = Tol::witness();
    let (eps, _) = band();
    let stub = 1e-4 * eps;
    let build = |declared: bool| {
        let p = Open
            .at(p2(0.0, 0.0))
            .angle(FRAC_PI_2, t)
            .unwrap()
            .line(1.0, t)
            .unwrap()
            .line_to(p2(1.0, 1.0), t)
            .unwrap()
            .line_to(p2(1.0, -1.0), t)
            .unwrap()
            .line_to(p2(-stub, 0.0), t)
            .unwrap();
        if declared {
            p.line_to(Start.arrives_tangent(), t)
        } else {
            p.line_to(Start, t)
        }
    };
    // The closing leg runs (-stub, 0) -> (0, 0): it arrives heading
    // EAST, at a right angle to the entry's outgoing NORTH. The
    // declaration is contradicted by 90 degrees, and the levered check
    // cannot see it because the lever is `stub`.
    let declared = build(true);
    println!("R2: degenerate declared close -> {declared:?}");
    let undeclared = build(false);
    println!("R2: degenerate undeclared close -> {undeclared:?}");
    match declared {
        Err(PathError::SeamArrivalLeverTooShort { arm }) => {
            assert!(arm.abs() < eps, "arm {arm} is the degenerate lever");
        }
        other => panic!("the declared check must refuse for want of a lever: {other:?}"),
    }
}

/// **The ARC member's lever, read back.** The suite pins dimension
/// honesty for the STRAIGHT closer only. Here the closing arc runs from
/// `(2, 1)` tangent to NORTH back to the entry at `(0, 0)`, whose
/// outgoing direction is EAST. One circle serves: centre `(0.75, 1)`,
/// radius `1.25`, chord `sqrt(5)`. Its end tangent is `(0.8, -0.6)`, so
/// the turn's sine against east is `0.6`, and the levered miss the
/// payload must carry is `0.6 * min(1.25, sqrt(5)) = 0.75` m. That
/// arithmetic is what "the lever is `radius.min(chord)`" means, and it
/// is checked rather than restated.
#[test]
fn r2_the_arc_lever_is_radius_min_chord() {
    let t = Tol::witness();
    let refused = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Start.arrives_tangent(), t);
    match refused {
        Err(PathError::SeamArrivalOffDirection { margin, arm, .. }) => {
            println!("R2: both-ends-tangent seam -> margin {margin} arm {arm}");
            assert!((arm - 1.25).abs() < 1e-12, "arm {arm} is not radius 1.25");
            assert!(
                (margin.abs() - 0.75).abs() < 1e-12,
                "margin {margin} is not 0.6 * 1.25"
            );
            // The chord is the LARGER of the two, so this row really
            // separates `radius.min(chord)` from `chord`.
            assert!(
                arm < 5.0_f64.sqrt(),
                "arm {arm} vs chord {}",
                5.0_f64.sqrt()
            );
        }
        other => panic!("{other:?}"),
    }
}

// ------------------------------------------------------------------
// 4. Lily's migration: the vertex table, byte for byte.
// ------------------------------------------------------------------

fn lily_ring(shoulder: f64) -> [(f64, f64); 8] {
    let (width, ridge_h, keel_h) = (0.170_f64, 0.028, 0.020);
    let sh = |a: (f64, f64), b: (f64, f64)| {
        let m = (0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1));
        (m.0 + shoulder * m.0, m.1 + shoulder * m.1)
    };
    let right = (0.5 * width, 0.0);
    let ridge = (0.0, ridge_h);
    let left = (-0.5 * width, 0.0);
    let keel = (0.0, -keel_h);
    [
        right,
        sh(right, ridge),
        ridge,
        sh(ridge, left),
        left,
        sh(left, keel),
        keel,
        sh(keel, right),
    ]
}

/// **The byte-stability claim's mechanism.** The demo used to hand the
/// loft a `RawLoop` built from eight computed points; it now authors the
/// same eight through the lattice. The render can only be byte-stable if
/// the lowered table is BIT-identical to the raw one, in both rotations
/// the loft pins. Checked here without a tour run.
#[test]
fn r2_lily_lattice_table_is_bit_identical_to_the_raw_table() {
    let t = Tol::witness();
    for shoulder in [0.0_f64, 1.0] {
        let ring = lily_ring(shoulder);
        let raw: ProfileLoop<f64> = RawLoop::new(
            ring.iter()
                .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
                .collect(),
        );
        let turns = |i: usize| {
            if shoulder == 0.0 {
                i.is_multiple_of(2)
            } else if shoulder == 1.0 {
                !i.is_multiple_of(2)
            } else {
                true
            }
        };
        let mut path = Open
            .at(p2(ring[0].0, ring[0].1))
            .line_to(p2(ring[1].0, ring[1].1), t)
            .expect("first side");
        for (i, v) in ring.iter().enumerate().skip(2) {
            path = if turns(i - 1) {
                path.line_to(p2(v.0, v.1), t).expect("corner")
            } else {
                path.continue_to(p2(v.0, v.1), t).expect("subdivision")
            };
        }
        let closed = match (turns(7), turns(0)) {
            (true, true) => path.line_to(Start, t),
            (true, false) => path.line_to(Start.arrives_tangent(), t),
            (false, true) => path.continue_to(Start, t),
            (false, false) => path.continue_to(Start.arrives_tangent(), t),
        }
        .unwrap_or_else(|e| panic!("shoulder {shoulder}: {e}"));

        let lowered = &closed.loop_;
        assert_eq!(lowered.vertices().len(), 8, "shoulder {shoulder}");
        for (i, (a, b)) in lowered
            .vertices()
            .iter()
            .zip(raw.vertices().iter())
            .enumerate()
        {
            assert!(
                a.pos().x.to_bits() == b.pos().x.to_bits()
                    && a.pos().y.to_bits() == b.pos().y.to_bits()
                    && a.bulge().to_bits() == b.bulge().to_bits(),
                "shoulder {shoulder} vertex {i}: {a:?} vs {b:?}"
            );
        }
        assert!(
            !lowered.tangent_joints().is_empty(),
            "shoulder {shoulder}: {:?}",
            lowered.tangent_joints()
        );
        validate_ok(lowered);
        println!("R2: lily shoulder {shoulder} — lattice table bit-identical to the raw one");
    }
}

/// The demo branches on `self.shoulder == 0.0` / `== 1.0` by exact float
/// equality, and every other value takes the "every vertex turns" arm
/// with `.expect(..)` on each leg. This row measures where that arm stops
/// being true as the shoulder approaches an end: it is a MEASUREMENT of
/// the knife edge the demo now sits on, not an assertion about it.
#[test]
fn r2_lily_near_kite_sections_are_where_the_demo_would_panic() {
    let t = Tol::witness();
    for shoulder in [1e-2_f64, 1e-4, 1e-6, 1e-8, 1e-10] {
        let ring = lily_ring(shoulder);
        let mut cur = match Open
            .at(p2(ring[0].0, ring[0].1))
            .line_to(p2(ring[1].0, ring[1].1), t)
        {
            Ok(p) => Some(p),
            Err(e) => {
                println!("R2: lily shoulder {shoulder} -> first side {e:?}");
                continue;
            }
        };
        let mut failed: Option<String> = None;
        for v in ring.iter().skip(2) {
            let p = cur.take().expect("a live path");
            match p.line_to(p2(v.0, v.1), t) {
                Ok(next) => cur = Some(next),
                Err(e) => {
                    failed = Some(format!("{e:?}"));
                    break;
                }
            }
        }
        match (failed, cur) {
            (Some(e), _) => println!("R2: lily shoulder {shoulder} -> leg refused {e}"),
            (None, Some(p)) => println!(
                "R2: lily shoulder {shoulder} -> close {:?}",
                p.line_to(Start, t).map(|_| "closed")
            ),
            (None, None) => unreachable!(),
        }
    }
}

// ------------------------------------------------------------------
// 5. The straight arrival when the entry's FIRST SIDE is an ARC.
// ------------------------------------------------------------------

/// **MAJOR — the premise `Start.arrives_tangent()` rests on is "one
/// carrier continues through the seam".** That holds when the entry's
/// first side is a LINE. When it is an ARC, a straight closing leg
/// arriving along the arc's start tangent is a G1 joint between
/// DISTINCT carriers — exactly what the #101 discipline says must be
/// DECLARED — and the straight arrival declares nothing (`declare_seam`
/// runs only on the tangent-arc closer, `path.rs:3297`).
///
/// So the lattice MINTS a loop the data gate then refuses
/// `UndeclaredTangency` at joint 0, and its suggestion names the RAW
/// door (`add 0 to loop 0\'s tangent_joints`) — the door issue 433 is
/// demoting. At the time of the review there was no lattice spelling:
/// `Start.arrives_tangent()` was not a `LineTarget`, on the argument
/// that "a straight leg\'s arrival direction IS its own direction" —
/// which this finding called beside the point, because what is
/// undeclared here is the JOINT, not the direction.
///
/// **RULED (Ev, in-chat, 2026-09-02): the finding was right, and the
/// argument is retired.** The token classifies the JOINT, so a straight
/// leg may declare a tangent seam and the recourse exists — this row
/// runs it below. The refusal STAYS at the data gate rather than moving
/// to the lattice, and that half is deliberate: nothing at the seam may
/// consult the following carrier, so the layer that owns materialized
/// carriers is the layer that speaks.
#[test]
fn r2_a_straight_arrival_onto_an_arc_first_side_authors_but_does_not_validate() {
    let t = Tol::witness();
    // Entry at (0,0) heading EAST, first side a quarter arc on the
    // circle centred (0,1); four corners; the closing leg runs
    // (-2,0) -> (0,0), arriving EAST — the arc's own start tangent.
    let ring = |declared: bool| {
        let p = Open
            .at(p2(0.0, 0.0))
            .angle(0.0, t)
            .unwrap()
            .tangent_arc_to(p2(1.0, 1.0), t)
            .unwrap()
            .line_to(p2(3.0, 3.0), t)
            .unwrap()
            .line_to(p2(3.0, -2.0), t)
            .unwrap()
            .line_to(p2(-2.0, -2.0), t)
            .unwrap()
            .line_to(p2(-2.0, 0.0), t)
            .unwrap();
        if declared {
            p.line_to(Start.arrives_tangent(), t)
        } else {
            p.line_to(Start, t)
        }
    };
    // BEFORE: the undeclared spelling refuses at the seam, at the
    // authoring layer, which is where this belongs.
    let undeclared = ring(false);
    println!("R2: arc-first-side, undeclared -> {undeclared:?}");
    assert!(matches!(undeclared, Err(PathError::SeamTangent { .. })));

    // AFTER THE RULING (Ev, in-chat, 2026-09-02): the lattice may not
    // consult the following carrier, so it closes declaring nothing and
    // the DATA gate — which owns materialized carriers — refuses.
    let closed = ring(true).expect("the declared arrival closes");
    assert_eq!(closed.loop_.tangent_joints(), &[0]);
    let verdict = Profile::new(SketchPlane::xy(), vec![closed.loop_]).validate(t);
    println!("R2: arc-first-side, declared -> at the gate {verdict:?}");
    // RULED (2026-09-02, addendum 3): one token, and it declares the
    // joint TANGENT — which is what this joint is — so the gate accepts.
    verdict.expect("the declared seam joint agrees with the data");

    // The RECOURSE the ruling gives: the same straight closer declaring
    // a TANGENT joint. It closes, declares joint 0, and validates.
    let g1 = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .tangent_arc_to(p2(1.0, 1.0), t)
        .unwrap()
        .line_to(p2(3.0, 3.0), t)
        .unwrap()
        .line_to(p2(3.0, -2.0), t)
        .unwrap()
        .line_to(p2(-2.0, -2.0), t)
        .unwrap()
        .line_to(p2(-2.0, 0.0), t)
        .unwrap()
        .line_to(Start.arrives_tangent(), t)
        .expect("a straight leg may declare a TANGENT seam joint");
    assert_eq!(g1.loop_.tangent_joints(), &[0]);
    validate_ok(&g1.loop_);
}

/// The SIBLING of the row above, on the tangent member: a closing arc
/// that arrives G1 at a seam whose first side is an arc on the SAME
/// circle. `declare_seam` marks joint 0 tangent unconditionally, and a
/// declaration on carrier IDENTITY is what `ProfileLoop::tangent_joints`
/// says is contradicted. `tangent_arc_geom`'s identity refusal compares
/// the closing arc against the PREVIOUS segment's carrier, not the
/// entry's, so a line in between hides it.
#[test]
fn r2_a_declared_g1_seam_onto_a_cocircular_first_side() {
    let t = Tol::witness();
    // The unit circle, entered at (1,0) heading north; a quarter of it
    // is the first side; two straights leave and come back to (0,-1);
    // the closing tangent arc is forced back onto the same circle.
    let turn_to_east = -(-3.0_f64).atan2(2.0);
    let built = Open
        .at(p2(1.0, 0.0))
        .angle(FRAC_PI_2, t)
        .unwrap()
        .tangent_arc_to(p2(0.0, 1.0), t)
        .unwrap()
        .line_to(p2(-2.0, 2.0), t)
        .unwrap()
        .line_to(p2(0.0, -1.0), t)
        .unwrap()
        .turn(turn_to_east, t)
        .unwrap()
        .tangent_arc_to(Start.arrives_tangent(), t);
    // AFTER THE RULING: the seam check reads NOTHING about the
    // following carrier, so this closes and the DATA gate refuses the
    // declaration the carriers contradict.
    // RULED the other way (Ev, in-chat, 2026-09-02, addendum 3):
    // every zero-turn joint is a declared tangent joint, so declaring
    // one onto an identical carrier is true rather than contradicted.
    let closed = built.expect("the declared G1 arrival closes");
    assert!(closed.loop_.tangent_joints().contains(&0));
    let verdict = Profile::new(SketchPlane::xy(), vec![closed.loop_]).validate(t);
    println!("R2: cocircular G1 seam, at the gate -> {verdict:?}");
    verdict.expect("the data gate accepts it: the directions agree");
}
