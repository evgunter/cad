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
use profile::{ProfileLoop, ProfileVertex, circle, circle_split};

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

/// The §5-1 half-disc: two same-carrier quarter arcs and the diameter
/// back. Its arc run is exactly what `arc_continue` exists for.
fn half_disc() -> ProfileLoop<f64> {
    let b = quarter_bulge();
    chain(&[(1.0, 0.0, b), (0.0, 1.0, b), (-1.0, 0.0, 0.0)])
}

/// Boss's rim: one closed carrier declared into three equal arcs.
fn thirds() -> ProfileLoop<f64> {
    circle_split(Point2::new(1.2, 1.7), 0.35, 3, 0.0, Tol::witness())
        .expect("boss rim splits")
        .loop_
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

/// Two collinear sides in a row: a same-carrier LINE run, for which the
/// chain vocabulary has no `line_continue`.
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
        ("arc_chain", arc_chain(), Class::Bits),
        ("lens", lens(), Class::Bits),
        ("circle_h", circle_h(0.0, 0.0, 1.0), Class::Bits),
        ("circle_v", circle_v(0.0, 0.0, 1.0), Class::Value),
        (
            "circle_primitive",
            circle(Point2::new(0.5, -0.25), 1.5, Tol::witness())
                .expect("circle")
                .loop_,
            Class::Bits,
        ),
        ("circle_split_3", thirds(), Class::Bits),
        ("half_disc", half_disc(), Class::Bits),
        ("bracket", bracket(), Class::Value),
        ("rounded_rect", rounded_rect(4.0, 3.0, 0.5), Class::Refused),
        ("unequal_split", unequal_split(), Class::Refused),
        ("collinear_run", collinear_run(), Class::Wall),
    ]
}

// ------------------------------------------------------------------
// The census
// ------------------------------------------------------------------

#[test]
fn the_census() {
    let mut rows = Vec::new();
    let mut tally = [0usize; 5];
    for (name, loop_, expected) in corpus() {
        let outcome = lift_checked(&loop_, Tol::witness());
        let got = classify(&outcome);
        rows.push(format!("{name:18} {}", describe(&outcome)));
        // Tally what was OBSERVED, not what was expected, so the totals
        // below are an independent measurement rather than a restatement
        // of the fixture table.
        tally[got as usize] += 1;
        assert_eq!(
            got,
            expected,
            "census row `{name}` changed class: {}",
            describe(&outcome)
        );
    }
    println!(
        "--- LIFT CENSUS ---\n{}\n-------------------",
        rows.join("\n")
    );

    // The tally of record. A vocabulary change that moves a loop
    // between buckets must move these numbers deliberately.
    assert_eq!(tally[Class::Bits as usize], 8, "bit-identical lifts");
    assert_eq!(tally[Class::Value as usize], 2, "value-equal lifts");
    assert_eq!(tally[Class::Refused as usize], 2, "structural walls");
    assert_eq!(tally[Class::Wall as usize], 1, "geometric walls");
    assert_eq!(tally[Class::Mismatch as usize], 0, "defects");
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
    // The undeclared shapes are exact — nothing derived enters them.
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

fn vert(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

#[test]
fn structural_walls_are_named() {
    // Too few vertices.
    let one = ProfileLoop::new(vec![vert(0.0, 0.0, 0.0)]);
    assert_eq!(
        refusal(&one),
        Some(LiftRefusal::TooFewVertices { vertices: 1 })
    );

    // Non-finite authored data.
    let nan = ProfileLoop::new(vec![
        vert(0.0, 0.0, 0.0),
        vert(f64::NAN, 1.0, 0.0),
        vert(1.0, 1.0, 0.0),
    ]);
    assert_eq!(refusal(&nan), Some(LiftRefusal::NonFinite { vertex: 1 }));

    // A declared index that names no vertex.
    let mut stray = rect(0.0, 0.0, 1.0, 1.0);
    stray = stray.with_tangent_joints(vec![9]);
    assert_eq!(
        refusal(&stray),
        Some(LiftRefusal::JointIndexOutOfRange {
            joint: 9,
            vertices: 4
        })
    );

    // A fully filleted outline: no sharp joint to seam the chain at.
    assert_eq!(
        refusal(&rounded_rect(4.0, 3.0, 0.5)),
        Some(LiftRefusal::AllJointsDeclared { joints: 8 })
    );

    // A declared joint whose leaving segment closes the loop straight:
    // after `.tangent()` only `.line(len)` is available, and `.line`
    // cannot close.
    let mut closing_line = ProfileLoop::new(vec![
        vert(0.0, 0.0, 0.3),
        vert(1.0, 1.0, 0.0),
        vert(0.0, 1.0, 0.0),
    ]);
    closing_line = closing_line.with_tangent_joints(vec![2]);
    assert_eq!(
        refusal(&closing_line),
        Some(LiftRefusal::DeclaredJointBeforeClosingLine { joint: 2 })
    );

    // A same-carrier arc run that reaches the seam: `arc_continue` has
    // no closing form, so the §5-1 class survives here as a wall even
    // though its mid-chain twin now lifts.
    assert_eq!(
        refusal(&unequal_split()),
        Some(LiftRefusal::SameCarrierClose { joint: 2 })
    );
}

/// The geometric walls stay the DRIVER's: the lift re-implements no
/// predicate, so the wall of record is the binder's own typed error.
#[test]
fn geometric_walls_are_the_drivers_own() {
    match lift_checked(&collinear_run(), Tol::witness()) {
        LiftOutcome::ReplayRefused { error, .. } => {
            // The collinear continuation is carrier identity, and the
            // chain vocabulary has no `line_continue` to spell it.
            let rendered = error.to_string();
            assert!(
                rendered.contains("carrier") || rendered.contains("tangent"),
                "unexpected wall: {rendered}"
            );
        }
        other => panic!(
            "a collinear line run should hit a wall: {}",
            describe(&other)
        ),
    }
}

/// The §5-1 same-carrier class, both halves in one place: MID-CHAIN it
/// lifts (through the declared-subdivision verb the PR-B ruling added),
/// AT THE SEAM it still refuses.
#[test]
fn the_same_carrier_class_splits_in_two() {
    assert_eq!(
        classify(&lift_checked(&half_disc(), Tol::witness())),
        Class::Bits
    );
    assert_eq!(
        classify(&lift_checked(&unequal_split(), Tol::witness())),
        Class::Refused
    );
}
