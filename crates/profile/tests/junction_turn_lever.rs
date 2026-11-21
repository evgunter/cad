//! The junction turn's definite arm names its two factors
//! (`seg::junction_turn`, `path_junction_turn`): the margin is
//! `sin φ · arm`, the shorter leg's arm, so a zero turn on a long leg and
//! a real angle on a leg about ε long both read Zero. The refusal says
//! neither is a tangency; it carries the sine, the cosine and the lever,
//! and its recourse offers the lever (a longer leg) beside the
//! declarations a smooth joint takes. The seam, read by the same gate,
//! refuses the same way under its own name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Gate, LeveredTurn, Open, PathError, Start, TurnLever};

use crate::common::told;

/// The number the sentence prints after `label`.
fn rendered_after(shown: &str, label: &str) -> f64 {
    let rest = &shown[shown
        .find(label)
        .unwrap_or_else(|| panic!("no {label:?} in: {shown}"))
        + label.len()..];
    let end = rest
        .find(|ch: char| !(ch.is_ascii_digit() || matches!(ch, '.' | '-' | 'e' | '+')))
        .unwrap_or(rest.len());
    rest[..end]
        .parse()
        .unwrap_or_else(|_| panic!("{label:?} renders no number in: {shown}"))
}

/// The payload holds the turn's sine and cosine and the lever it was
/// read over, the sentence tells all three as the payload holds them,
/// and what it says before its recourse names no tangency.
fn assert_names_its_factors(row: &str, turn: &LeveredTurn, shown: &str, want: [f64; 3]) {
    let got = [&turn.sin_turn, &turn.align, &turn.lever].map(told);
    for ((name, got), want) in ["sin φ", "cos φ", "lever"].iter().zip(got).zip(want) {
        assert!(
            (got - want).abs() <= 1e-9 * want.abs(),
            "{row}: {name} holds {got:e}, the geometry has {want:e}"
        );
    }
    assert_eq!(turn.lever_of, TurnLever::ShorterLeg, "{row}");
    for (label, want) in [
        ("sin φ = ", got[0]),
        ("(cos φ = ", got[1]),
        ("over the shorter leg of ", got[2]),
    ] {
        let said = rendered_after(shown, label);
        assert!(
            (said - want).abs() <= 5e-4 * want.abs(),
            "{row}: {label:?} renders {said:e}, the payload holds {want:e}: {shown}"
        );
    }
    let (fact, recourse) = shown
        .split_once("Recourse: ")
        .unwrap_or_else(|| panic!("{row}: no recourse in: {shown}"));
    assert!(
        !fact.contains("tangent"),
        "{row}: the fact asserts a tangency: {shown}"
    );
    assert!(
        recourse.starts_with("if the angle is real, lengthen the shorter leg"),
        "{row}: the recourse names the lever first: {shown}"
    );
}

/// The ways a 64° turn after a straight leg of `k·ε` is authored: the
/// junction, by direction and by point, and the seam, a metre-long
/// closing leg arriving at 64° into a start whose first leg is `k·ε`
/// along +x (the run continues straight on from it, so that leg's own
/// end is no junction).
fn rows(k: f64, tol: Tol) -> [(&'static str, Result<(), PathError<f64>>); 3] {
    let eps = tol.eps();
    let phi = 64.0_f64.to_radians();
    let (c, s) = (phi.cos(), phi.sin());
    let leg = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(k * eps, 0.0), tol)
        .expect("a leg of k·ε authors");
    let toward = leg.clone().toward(c, s, tol).map(|_| ());
    let line = leg.line_to(Point2::new(k * eps + c, s), tol).map(|_| ());
    let seam = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(k * eps, 0.0), tol)
        .expect("a first leg of k·ε authors")
        .line(1.0, tol)
        .expect("the first side runs on along +x")
        .line_to(Point2::new(1.0 + k * eps, -2.0), tol)
        .expect("a real corner")
        .line_to(Point2::new(-c, -s), tol)
        .expect("a real corner")
        .line_to(Start, tol)
        .map(|_| ());
    [("toward", toward), ("line_to", line), ("seam", seam)]
}

/// A real 64° turn after a leg of 0.3ε to 1.1ε is zero at tolerance on
/// its levered margin. Both the junction and the seam refuse naming the
/// turn and the leg, and neither says the joint is tangent.
#[test]
fn a_real_turn_on_a_short_leg_names_its_factors_at_junction_and_seam() {
    let tol = Tol::witness();
    let phi = 64.0_f64.to_radians();
    for k in [0.3, 0.9, 1.05, 1.1] {
        let lever = k * tol.eps();
        for (spelling, outcome) in rows(k, tol) {
            let row = format!("{spelling} after a {k}ε leg");
            let err = outcome.expect_err(&row);
            let shown = err.to_string();
            match (spelling, &err) {
                ("seam", PathError::SeamTurnWithinTolerance { turn }) => {
                    // Arriving at 64°, leaving along +x: a clockwise turn.
                    assert_names_its_factors(&row, turn, &shown, [-phi.sin(), phi.cos(), lever]);
                    assert!(
                        shown.contains("Start.arrives_tangent()"),
                        "{row}: the seam's declaration is still offered: {shown}"
                    );
                }
                ("toward" | "line_to", PathError::JunctionTurnWithinTolerance { turn }) => {
                    assert_names_its_factors(&row, turn, &shown, [phi.sin(), phi.cos(), lever]);
                }
                _ => panic!("{row}: expected the turn-within-tolerance refusal, got {err:?}"),
            }
        }
    }
}

/// The same turn on a leg of 3ε is in band and escalates under the
/// junction turn's gate, ending in the lever the definite arm also
/// names; on a leg of 50ε the turn is decided and every spelling builds.
#[test]
fn the_same_turn_escalates_at_three_eps_and_builds_at_fifty() {
    let tol = Tol::witness();
    for (spelling, outcome) in rows(3.0, tol) {
        let err = outcome.expect_err(spelling);
        let PathError::Escalated { gate, .. } = &err else {
            panic!("{spelling} on a 3ε leg: expected an escalation, got {err:?}")
        };
        assert_eq!(*gate, Gate::PathJunctionTurn, "{spelling}");
        let shown = err.to_string();
        assert!(
            shown.contains("if the angle is real, lengthen the shorter leg"),
            "{spelling}: the in-band ending names the lever: {shown}"
        );
    }
    for (spelling, outcome) in rows(50.0, tol) {
        outcome.unwrap_or_else(|e| panic!("{spelling} on a 50ε leg builds: {e}"));
    }
}

/// A genuinely smooth joint on metre-long legs is refused the same way,
/// and its recourse fits it: the declaration it offers is the one the
/// joint takes, and followed, it builds.
#[test]
fn a_near_zero_turn_on_a_long_leg_is_refused_with_a_recourse_that_fits() {
    let tol = Tol::witness();
    let leg = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(1.0, 0.0), tol)
        .expect("a metre east");
    let err = leg
        .clone()
        .toward(1.0, 1e-10, tol)
        .expect_err("a turn of 1e-10 over a metre is zero at tolerance");
    let PathError::JunctionTurnWithinTolerance { turn } = &err else {
        panic!("expected the turn-within-tolerance refusal, got {err:?}")
    };
    let shown = err.to_string();
    assert_names_its_factors("junction", turn, &shown, [1e-10, 1.0, 1.0]);
    assert!(
        shown.contains("if smooth, declare it: .tangent()")
            && shown.contains("or line(len) for a straight continuation"),
        "the smooth joint's declarations are offered: {shown}"
    );
    leg.line(1.0, tol)
        .expect("the straight continuation it offers builds");

    // The seam: a closing metre arriving along the entry's first side.
    let tip = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(1.0, 0.0), tol)
        .unwrap()
        .line_to(Point2::new(1.0, 1.0), tol)
        .unwrap()
        .line_to(Point2::new(-1.0, 1.0), tol)
        .unwrap()
        .line_to(Point2::new(-1.0, 0.0), tol)
        .unwrap();
    let err = tip
        .clone()
        .line_to(Start, tol)
        .expect_err("an undeclared smooth seam refuses");
    let PathError::SeamTurnWithinTolerance { turn } = &err else {
        panic!("expected the seam's turn-within-tolerance refusal, got {err:?}")
    };
    let shown = err.to_string();
    assert_names_its_factors("seam", turn, &shown, [0.0, 1.0, 1.0]);
    assert!(
        shown.contains("DECLARE it with Start.arrives_tangent() on the target"),
        "the seam's declaration is offered: {shown}"
    );
    tip.line_to(Start.arrives_tangent(), tol)
        .expect("the declared seam closes");
}
