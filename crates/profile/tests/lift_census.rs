//! **The lift's differential harness and refusal census** (LIB-SWITCH
//! §7, PROFILES-V2 §V5).
//!
//! Two jobs in one suite, exactly as §V5 specifies:
//!
//! 1. **The differential harness** — lift → replay → bit-compare
//!    against the source loop. Every row states the fidelity it
//!    achieved (bit-identical vs value-equal), which is the F10 report
//!    the design asks the tool to make.
//! 2. **The coverage meter** — a census over the fixture corpus,
//!    tallying lifts against named walls. The tally is ASSERTED, so a
//!    vocabulary change that turns a refusal into a lift (or the
//!    reverse) shows up as a failing row rather than as silence. That
//!    is what makes it an acceptance instrument for chain-vocabulary
//!    growth rather than a smoke test.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{
    bracket, chain, circle_h, circle_v, l_profile, lens, quarter_bulge, rect, rounded_rect,
};
use geom_core::Point2;
use geom_core::Tol;
use profile::RawLoop;
use profile::lift::{Fidelity, LiftOutcome, LiftRefusal, lift, lift_checked};
use profile::{
    Bulge, Open, ProfileLoop, Segment, Start, Step, Target, Verb, circle, circle_split, replay,
    test_support::bulge_loop,
};

/// The coarse bucket a census row falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    /// Lifted; replay reproduces the source loop bit for bit.
    Bits,
    /// Lifted; the shape agrees, derived bits shifted (F10 / W1).
    Value,
    /// A structural wall — the lift's own refusal.
    Refused,
    /// A geometric wall — the driver's refusal, verbatim.
    Wall,
    /// The lifted program replayed to a different loop: a defect.
    Mismatch,
}

fn classify(outcome: &LiftOutcome) -> Class {
    match outcome {
        LiftOutcome::Lifted {
            fidelity: Fidelity::BitIdentical,
            ..
        } => Class::Bits,
        LiftOutcome::Lifted {
            fidelity: Fidelity::ValueEqual,
            ..
        } => Class::Value,
        LiftOutcome::Refused(_) => Class::Refused,
        LiftOutcome::ReplayRefused { .. } => Class::Wall,
        LiftOutcome::Mismatch { .. } => Class::Mismatch,
    }
}

fn describe(outcome: &LiftOutcome) -> String {
    match outcome {
        LiftOutcome::Lifted {
            program,
            rotation,
            fidelity,
            worst_ulps,
            worst_abs,
        } => format!(
            "{fidelity:?} ({} steps, seam +{rotation}, worst {worst_ulps} ulp / {worst_abs:.3e} m)",
            program.len()
        ),
        LiftOutcome::Refused(r) => format!("REFUSED {r:?}"),
        LiftOutcome::ReplayRefused { error, .. } => format!("WALL {error}"),
        LiftOutcome::Mismatch {
            worst_ulps,
            worst_abs,
            ..
        } => format!("MISMATCH (worst {worst_ulps} ulp / {worst_abs:.3e} m)"),
    }
}

// ------------------------------------------------------------------
// Fixtures beyond `common`'s
// ------------------------------------------------------------------

/// The §5-1 half-disc: two quarter arcs on one carrier and the diameter
/// back. Its joint between the arcs is one carrier continuing, so it is
/// tangent; the driver refuses the run's zero-turn junction as a sharp
/// spelling, and the lift spells it `.tangent().tangent_arc_to(p)`.
fn half_disc() -> ProfileLoop<f64> {
    let b = quarter_bulge();
    chain(&[(1.0, 0.0, b), (0.0, 1.0, b), (-1.0, 0.0, 0.0)])
}

/// Boss's rim: one closed carrier declared into three equal arcs.
fn thirds() -> ProfileLoop<f64> {
    circle_split(Point2::new(1.2, 1.7), 0.35, 3, 0.0, Tol::witness())
        .expect("boss rim splits")
        .loop_
        .into_loop()
}

/// A plain arc chain: no declared joints, no same-carrier run.
fn arc_chain() -> ProfileLoop<f64> {
    chain(&[(0.0, 0.0, 0.3), (2.0, 0.0, -0.2), (2.0, 2.0, 0.1)])
}

/// A closed carrier split UNEQUALLY (90°, 90°, 180°). `circle_split`
/// spells equal splits only, so the carrier form does not fit and the
/// chain form's arc run runs into the seam.
fn unequal_split() -> ProfileLoop<f64> {
    chain(&[
        (1.0, 0.0, quarter_bulge()),
        (0.0, 1.0, quarter_bulge()),
        (-1.0, 0.0, 1.0),
    ])
}

/// Two collinear sides in a row: a same-carrier LINE run.
fn collinear_run() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(1.0, 1.0),
    ])
}

fn corpus() -> Vec<(&'static str, ProfileLoop<f64>, Class)> {
    vec![
        ("rect", rect(0.0, 0.0, 2.0, 1.0), Class::Bits),
        ("l_profile", l_profile(), Class::Bits),
        ("arc_chain", arc_chain(), Class::Value),
        ("lens", lens(), Class::Value),
        ("circle_h", circle_h(0.0, 0.0, 1.0), Class::Bits),
        ("circle_v", circle_v(0.0, 0.0, 1.0), Class::Value),
        (
            "circle_primitive",
            circle(Point2::new(0.5, -0.25), 1.5, Tol::witness())
                .expect("circle")
                .loop_
                .into_loop(),
            Class::Bits,
        ),
        ("circle_split_3", thirds(), Class::Bits),
        ("half_disc", half_disc(), Class::Value),
        ("bracket", bracket(), Class::Value),
        ("rounded_rect", rounded_rect(4.0, 3.0, 0.5), Class::Value),
        ("unequal_split", unequal_split(), Class::Value),
        ("collinear_run", collinear_run(), Class::Bits),
    ]
}

// ------------------------------------------------------------------
// The census
// ------------------------------------------------------------------

#[test]
fn the_census() {
    let mut rows = Vec::new();
    let mut moved = Vec::new();
    let mut tally = [0usize; 5];
    for (name, loop_, expected) in corpus() {
        let outcome = lift_checked(&loop_, Tol::witness());
        let got = classify(&outcome);
        rows.push(format!("{name:18} {}", describe(&outcome)));
        // Tally what was OBSERVED, not what was expected, so the totals
        // below are an independent measurement rather than a restatement
        // of the fixture table.
        tally[got as usize] += 1;
        if got != expected {
            moved.push(format!("`{name}`: {got:?}, want {expected:?}"));
        }
    }
    println!(
        "--- LIFT CENSUS ---\n{}\n-------------------",
        rows.join("\n")
    );
    assert!(moved.is_empty(), "census rows changed class: {moved:?}");

    // The tally of record. A vocabulary change that moves a loop
    // between buckets must move these numbers deliberately.
    assert_eq!(tally[Class::Bits as usize], 6, "bit-identical lifts");
    assert_eq!(tally[Class::Value as usize], 7, "value-equal lifts");
    assert_eq!(tally[Class::Refused as usize], 0, "structural walls");
    assert_eq!(tally[Class::Wall as usize], 0, "geometric walls");
    assert_eq!(tally[Class::Mismatch as usize], 0, "mismatches");
}

/// The F10 report the design demands: the tool must SAY which fidelity
/// it achieved, and must not claim bit-identity for an anchor-derived
/// spelling.
#[test]
fn the_fidelity_report_is_honest() {
    // A declared junction is spelled `.tangent()`, and the arc that
    // follows re-derives its bulge — so the bracket's fillet arc is
    // value-equal, never bit-identical, and the tool says exactly that.
    match lift_checked(&bracket(), Tol::witness()) {
        LiftOutcome::Lifted {
            fidelity,
            worst_ulps,
            worst_abs,
            ..
        } => {
            assert_eq!(fidelity, Fidelity::ValueEqual);
            assert!(worst_ulps > 0, "value-equal means some bit moved");
            assert!(
                worst_abs < 1e-12,
                "and only in the last bits: {worst_abs:e}"
            );
        }
        other => panic!("bracket should lift: {}", describe(&other)),
    }
    // `rounded_rect` joined the value-equal class when the lift widened
    // (BOOL-9: a declared joint before a closing straight is the
    // continuation verb, so this loop lifts instead of refusing), and it
    // arrived with no ceiling of its own — the coarse `LiftOutcome`
    // bucketing was holding it, and there the RELATIVE criterion is
    // inoperative on this row (4.39e18 ulp, a straddle of zero), so only
    // the 1e-12 absolute floor applied: 300x the residue actually
    // measured. Its residue is the same class as the bracket's — four
    // fillet arcs re-deriving their bulge — so it gets the same kind of
    // ceiling, sized to what it does.
    match lift_checked(&rounded_rect(4.0, 3.0, 0.5), Tol::witness()) {
        LiftOutcome::Lifted {
            fidelity,
            worst_ulps,
            worst_abs,
            ..
        } => {
            assert_eq!(fidelity, Fidelity::ValueEqual);
            assert!(worst_ulps > 0, "value-equal means some bit moved");
            assert!(
                worst_abs < 1e-14,
                "the fillet arcs' bulge re-derivation, and nothing more: {worst_abs:e}"
            );
        }
        other => panic!("rounded_rect should lift: {}", describe(&other)),
    }
    // An undeclared arc is written about its stored centre
    // (`arc_to(Center)`), and the replay keeps that centre and reads the
    // radius as the rim at the arc's start and the sweep off the chord
    // about it, so the free arc chains are value-equal too, and their
    // residue is this, measured.
    for (name, loop_, ceiling) in [("arc_chain", arc_chain(), 1e-14), ("lens", lens(), 1e-15)] {
        match lift_checked(&loop_, Tol::witness()) {
            LiftOutcome::Lifted {
                fidelity,
                worst_abs,
                ..
            } => {
                assert_eq!(fidelity, Fidelity::ValueEqual, "{name}");
                assert!(worst_abs < ceiling, "{name}: {worst_abs:e}");
            }
            other => panic!("{name} should lift: {}", describe(&other)),
        }
    }
    // The undeclared straight shapes are exact — nothing derived enters them.
    for (name, loop_) in [
        ("rect", rect(0.0, 0.0, 2.0, 1.0)),
        ("l_profile", l_profile()),
    ] {
        match lift_checked(&loop_, Tol::witness()) {
            LiftOutcome::Lifted {
                fidelity,
                worst_ulps,
                ..
            } => {
                assert_eq!(fidelity, Fidelity::BitIdentical, "{name}");
                assert_eq!(worst_ulps, 0, "{name}");
            }
            other => panic!("{name} should lift: {}", describe(&other)),
        }
    }
}

/// VQ4/W1, measured at the one place it survives: `circle_split`'s
/// `phase` is an ANGLE, so a carrier loop whose seam is not on the +x
/// axis round-trips through `sin_cos` and lands ulps off. The exact
/// director closed this class for chain DIRECTIONS; the carrier form's
/// phase is the residue, and this row is its receipt.
#[test]
fn the_carrier_phase_is_the_surviving_angle_residue() {
    match lift_checked(&circle_v(0.0, 0.0, 1.0), Tol::witness()) {
        LiftOutcome::Lifted {
            fidelity,
            worst_abs,
            ..
        } => {
            assert_eq!(fidelity, Fidelity::ValueEqual);
            assert!(worst_abs < 1e-15, "quantization scale only: {worst_abs:e}");
        }
        other => panic!(
            "a vertically split circle should lift: {}",
            describe(&other)
        ),
    }
    // The +x-seamed twin is exact, which is what makes the diagnosis
    // "the angle", not "the carrier form".
    assert_eq!(
        classify(&lift_checked(&circle_h(0.0, 0.0, 1.0), Tol::witness())),
        Class::Bits
    );
}

// ------------------------------------------------------------------
// The named walls, one row each
// ------------------------------------------------------------------

/// The lift's refusal, if it refused (`Step` carries no equality by
/// design, so the Ok side is not comparable).
fn refusal(loop_: &ProfileLoop<f64>) -> Option<LiftRefusal> {
    lift(loop_, Tol::witness()).err()
}

fn vert(x: f64, y: f64, bulge: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), bulge)
}

#[test]
fn structural_walls_are_named() {
    // Too few vertices.
    let one = bulge_loop(vec![vert(0.0, 0.0, 0.0)]);
    assert_eq!(
        refusal(&one),
        Some(LiftRefusal::TooFewVertices { vertices: 1 })
    );

    // Non-finite authored data.
    let nan = bulge_loop(vec![
        vert(0.0, 0.0, 0.0),
        vert(f64::NAN, 1.0, 0.0),
        vert(1.0, 1.0, 0.0),
    ]);
    assert_eq!(refusal(&nan), Some(LiftRefusal::NonFinite { vertex: 1 }));

    // A joint validation cannot decide: the subdivision vertex of the
    // bottom side stands inside the band off it, so there is no
    // tangent set to spell.
    let eps = Tol::witness().eps();
    let undecided = bulge_loop(vec![
        vert(0.0, 0.0, 0.0),
        vert(1.0, 2.0 * eps, 0.0),
        vert(2.0, 0.0, 0.0),
        vert(2.0, 1.0, 0.0),
        vert(0.0, 1.0, 0.0),
    ]);
    assert!(
        matches!(
            refusal(&undecided),
            Some(LiftRefusal::Unclassified(
                profile::ProfileError::Escalated { .. }
            ))
        ),
        "{:?}",
        refusal(&undecided)
    );

    // The two walls this list no longer names are demonstrated in
    // `bool9_probes.rs` instead.
}

/// **A same-carrier run lifts wherever it sits.** Its joints are one
/// carrier continuing, so validation derives them tangent and the lift
/// spells each with a continuation: a collinear LINE run as
/// `.tangent().line(len)`, bit for bit, and a cocircular ARC run mid-chain
/// (`.tangent().tangent_arc_to(p)`) and at the seam (the arrival
/// carrying the seam's tangency) alike, value-equal, the leading arc
/// being written about its stored centre.
///
/// Red if the lift stops reading the derived set: each run's
/// zero-turn junction is the one the driver refuses as a sharp
/// spelling.
#[test]
fn a_same_carrier_run_lifts_wherever_it_sits() {
    match lift_checked(&collinear_run(), Tol::witness()) {
        LiftOutcome::Lifted {
            program, fidelity, ..
        } => {
            assert_eq!(fidelity, Fidelity::BitIdentical);
            assert!(
                program.iter().any(|s| matches!(s.verb(), Verb::Tangent)),
                "{program:?}"
            );
        }
        other => panic!("a collinear line run lifts: {}", describe(&other)),
    }
    for (name, loop_) in [
        ("half_disc", half_disc()),
        ("unequal_split", unequal_split()),
    ] {
        assert_eq!(
            classify(&lift_checked(&loop_, Tol::witness())),
            Class::Value,
            "{name}"
        );
    }
}

/// **A cocircular run lifts with its joint constructed.** The driver
/// refuses the raw run's zero-turn junction as a sharp spelling
/// (`JunctionTangent`), and the lift spells the tangent joint the
/// lattice's own way — `.tangent()` then `tangent_arc_to(p)`, the
/// tangent-chord derivation the raw carrier satisfies — which mints the
/// raw run's vertex BITS: the vertex table replays bit for bit, the
/// arcs agree in the last bits (the leading arc is written about its
/// stored centre, and its replayed radius is the rim at its start), and
/// the replay constructs exactly the joint validation derives.
///
/// Red if the comparator reads the constructed joint as a different
/// loop, or the replay constructs a joint other than source joint 1.
#[test]
fn a_cocircular_run_lifts_with_its_joint_constructed() {
    let raw = half_disc();
    let outcome = lift_checked(&raw, Tol::witness());
    let LiftOutcome::Lifted {
        program,
        rotation,
        fidelity,
        ..
    } = outcome
    else {
        panic!(
            "the cocircular run lifts with its joint constructed: {}",
            describe(&outcome)
        );
    };
    assert_eq!(
        fidelity,
        Fidelity::ValueEqual,
        "the table replays to the last bits: the leading arc's radius is the rim at its start"
    );
    let verbs: Vec<Verb> = program.iter().map(Step::verb).collect();
    assert_eq!(
        verbs,
        vec![
            Verb::At,
            Verb::ArcTo,
            Verb::Tangent,
            Verb::TangentArcTo,
            Verb::LineTo
        ],
        "the joint is constructed, the arc derived from the inherited tangent"
    );
    let replayed = replay(&program, Tol::witness()).expect("the tangent spelling replays");
    let n = raw.vertices().len();
    assert_eq!(
        replayed.constructed_joints(),
        &[(1 + n - rotation) % n],
        "the replay constructs source joint 1"
    );
}

/// The 2 × 2 square whose first side is written as an ARC of bulge
/// `b`: `arc_to(Bulge { b })` with `b` a signed zero is the straight
/// segment the bulge form says it is.
fn zero_bulge_square(b: f64) -> ProfileLoop<f64> {
    let t = Tol::witness();
    Open.at(Point2::new(0.0, 0.0))
        .arc_to(
            Bulge {
                p: Point2::new(2.0, 0.0),
                b,
            },
            t,
        )
        .unwrap()
        .line_to(Point2::new(2.0, 2.0), t)
        .unwrap()
        .line_to(Point2::new(0.0, 2.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
        .loop_
        .into_loop()
}

/// The same square with its first side a `tangent_arc_to` whose target
/// lies straight ahead of the incoming leg: the tangent arc through a
/// collinear point is the straight continuation, bulge zero.
fn collinear_tangent_arc_square() -> ProfileLoop<f64> {
    let t = Tol::witness();
    Open.at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Point2::new(2.0, 0.0), t)
        .unwrap()
        .line_to(Point2::new(2.0, 2.0), t)
        .unwrap()
        .line_to(Point2::new(0.0, 2.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
        .loop_
        .into_loop()
}

/// **A zero bulge is stored as a line, whichever verb wrote it**, so it
/// lifts as the straight step: an arc lowered from b = 0 would lift as
/// `ArcTo(Bulge { b: 0 })`, a spelling the loop never had.
#[test]
fn zero_bulge_arc_to_lifts_as_a_line() {
    for b in [0.0, -0.0] {
        let lp = zero_bulge_square(b);
        assert!(
            matches!(lp.segments()[0], Segment::Line),
            "b = {b:e}: {:?}",
            lp.segments()[0]
        );
        let program = lift(&lp, Tol::witness()).expect("the square lifts");
        assert!(
            matches!(program[1], Step::LineTo(Target::Point(p)) if (p.x, p.y) == (2.0, 0.0)),
            "b = {b:e}: {program:?}"
        );
    }
}

/// The collinear `tangent_arc_to` sibling: its segment is stored as a
/// line, and the lift spells it straight.
#[test]
fn collinear_tangent_arc_to_lifts_as_a_line() {
    let lp = collinear_tangent_arc_square();
    assert!(
        matches!(lp.segments()[1], Segment::Line),
        "{:?}",
        lp.segments()[1]
    );
    let program = lift(&lp, Tol::witness()).expect("the square lifts");
    assert!(
        program
            .iter()
            .all(|s| !matches!(s, Step::ArcTo(_) | Step::TangentArcTo(_))),
        "{program:?}"
    );
}
