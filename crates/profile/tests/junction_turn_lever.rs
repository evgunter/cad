//! The junction turn's definite arm names its two factors
//! (`seg::junction_turn`, `path_junction_turn`): the margin is
//! `sin φ · arm`, the shorter leg's arm, so a zero turn on a long leg and
//! a real angle on a leg about ε long both read Zero. The refusal says
//! neither is a tangency; it carries the sine, the cosine and the lever,
//! and its recourse offers the lever (a longer leg, or an arc's wider
//! radius) beside the declarations a smooth joint takes. The seam, read
//! by the same gate, refuses the same way under its own name and ends in
//! its own declaration, undecided or definite.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{
    ArcSweep, Center, EscalatedAt, Gate, LeveredTurn, Open, PathError, Start, TurnLever,
};

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
/// read over (to `rel`), the sentence tells all three as the payload
/// holds them, and what it says before its recourse names no tangency.
fn assert_names_its_factors(row: &str, turn: &LeveredTurn, shown: &str, want: [f64; 3], rel: f64) {
    let got = [&turn.sin_turn, &turn.align, &turn.lever].map(told);
    for ((name, got), want) in ["sin φ", "cos φ", "lever"].iter().zip(got).zip(want) {
        assert!(
            (got - want).abs() <= rel * want.abs(),
            "{row}: {name} holds {got:e}, the geometry has {want:e}"
        );
    }
    assert_eq!(turn.lever_of, TurnLever::ShorterLeg, "{row}");
    for (label, want) in [
        ("sin φ = ", got[0]),
        ("(cos φ = ", got[1]),
        ("over the shorter leg's arm of ", got[2]),
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
        recourse.starts_with(
            "if the angle is real, lengthen the shorter leg (or grow an arc's radius)"
        ),
        "{row}: the recourse names the lever first: {shown}"
    );
}

/// One authored 64° turn, the joint it lands on, and the factors the
/// refusal must carry.
struct Row {
    spelling: &'static str,
    seam: bool,
    outcome: Result<(), PathError<f64>>,
    /// `[sin φ, cos φ, lever]`.
    want: [f64; 3],
}

/// The ways a 64° turn on a short leg of `k·ε` is authored, each with
/// the leg that is short named in its spelling:
/// - the ARRIVING leg short, departing by `.toward`, `.angle`, `.turn`
///   and `.line_to`;
/// - the DEPARTING leg short: a line, and a half-turn arc of radius
///   `k·ε`, whose arm is its radius (the radius is shorter than its
///   chord);
/// - the seam with the FIRST leg short (the run continues straight on
///   from it, so that leg's own end is no junction) and a metre-long
///   closing leg;
/// - the seam with the CLOSING leg short: a tangent arc of chord `k·ε`
///   (its arm is the chord) declared off a metre-long leg, into a long
///   first leg.
fn rows(k: f64, tol: Tol) -> Vec<Row> {
    let eps = tol.eps();
    let phi = 64.0_f64.to_radians();
    let (c, s) = (phi.cos(), phi.sin());
    let lever = k * eps;
    let short = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(lever, 0.0), tol)
        .expect("a leg of k·ε authors");
    let long = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(1.0, 0.0), tol)
        .expect("a metre east");
    let bend = [s, c, lever];
    let junction = |spelling, outcome: Result<(), PathError<f64>>, want| Row {
        spelling,
        seam: false,
        outcome,
        want,
    };
    let mut rows = vec![
        junction(
            "toward after a short leg",
            short.clone().toward(c, s, tol).map(|_| ()),
            bend,
        ),
        junction(
            "angle after a short leg",
            short.clone().angle(phi, tol).map(|_| ()),
            bend,
        ),
        junction(
            "turn after a short leg",
            short.clone().turn(phi, tol).map(|_| ()),
            bend,
        ),
        junction(
            "line_to after a short leg",
            short
                .clone()
                .line_to(Point2::new(lever + c, s), tol)
                .map(|_| ()),
            bend,
        ),
        junction(
            "a short line_to after a long leg",
            long.clone()
                .line_to(Point2::new(1.0 + lever * c, lever * s), tol)
                .map(|_| ()),
            bend,
        ),
        junction(
            "a short half-turn arc after a long leg",
            long.clone()
                .arc_to(
                    Center {
                        c: Point2::new(1.0 - lever * s, lever * c),
                        winding: ArcSweep::Ccw,
                        p: Point2::new(1.0 - 2.0 * lever * s, 2.0 * lever * c),
                    },
                    tol,
                )
                .map(|_| ()),
            bend,
        ),
    ];
    rows.push(Row {
        spelling: "the seam after a short first leg",
        seam: true,
        outcome: Open
            .at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(lever, 0.0), tol)
            .expect("a first leg of k·ε authors")
            .line(1.0, tol)
            .expect("the first side runs on along +x")
            .line_to(Point2::new(1.0 + lever, -2.0), tol)
            .expect("a real corner")
            .line_to(Point2::new(-c, -s), tol)
            .expect("a real corner")
            .line_to(Start, tol)
            .map(|_| ()),
        // Arriving at 64°, leaving along +x: a clockwise turn.
        want: [-s, c, lever],
    });
    // The closing chord runs at 54°, the tangent arc leaves it 10° below
    // (along 44°) and so arrives 10° above, at 64°.
    let (chord, depart) = (54.0_f64.to_radians(), 44.0_f64.to_radians());
    let before = Point2::new(-lever * chord.cos(), -lever * chord.sin());
    rows.push(Row {
        spelling: "the seam after a short closing arc",
        seam: true,
        outcome: long
            .line_to(Point2::new(1.0, -2.0), tol)
            .expect("a real corner")
            .line_to(
                Point2::new(before.x - depart.cos(), before.y - depart.sin()),
                tol,
            )
            .expect("a real corner")
            .line_to(before, tol)
            .expect("a metre-long leg along 44°")
            .tangent()
            .tangent_arc_to(Start, tol)
            .map(|_| ()),
        want: [-s, c, lever],
    });
    rows
}

/// A real 64° turn after a leg of 0.3ε to 1.1ε is zero at tolerance on
/// its levered margin, whichever leg is short. The junction and the seam
/// refuse naming the turn and the shorter arm, and neither says the joint
/// is tangent.
#[test]
fn a_real_turn_on_a_short_leg_names_its_factors_at_junction_and_seam() {
    let tol = Tol::witness();
    for k in [0.3, 0.9, 1.05, 1.1] {
        for Row {
            spelling,
            seam,
            outcome,
            want,
        } in rows(k, tol)
        {
            let row = format!("{spelling} of {k}ε");
            let err = outcome.expect_err(&row);
            let shown = err.to_string();
            match (seam, &err) {
                (true, PathError::SeamTurnWithinTolerance { turn }) => {
                    assert_names_its_factors(&row, turn, &shown, want, 1e-6);
                    assert!(
                        shown.contains("DECLARE it with Start.arrives_tangent() on the target"),
                        "{row}: the seam's declaration is offered: {shown}"
                    );
                }
                (false, PathError::JunctionTurnWithinTolerance { turn }) => {
                    assert_names_its_factors(&row, turn, &shown, want, 1e-6);
                }
                _ => panic!("{row}: expected the turn-within-tolerance refusal, got {err:?}"),
            }
        }
    }
}

/// The same turn on a leg of 3ε is in band and escalates under the
/// junction turn's gate. The junction's ending names the leg lever and
/// the junction's declarations; the seam's names the leg lever and the
/// seam's own declaration, which rides the target. On a leg of 50ε the
/// turn is decided and every spelling builds.
#[test]
fn the_same_turn_escalates_at_three_eps_and_builds_at_fifty() {
    let tol = Tol::witness();
    for Row {
        spelling,
        seam,
        outcome,
        ..
    } in rows(3.0, tol)
    {
        let err = outcome.expect_err(spelling);
        let PathError::Escalated { gate, site, .. } = &err else {
            panic!("{spelling} on a 3ε leg: expected an escalation, got {err:?}")
        };
        assert_eq!(*gate, Gate::PathJunctionTurn, "{spelling}");
        let shown = err.to_string();
        assert!(
            shown.contains(
                "if the angle is real, lengthen the shorter leg (or grow an arc's radius)"
            ),
            "{spelling}: the in-band ending names the lever: {shown}"
        );
        if seam {
            assert_eq!(*site, EscalatedAt::Seam, "{spelling}");
            assert!(
                shown.contains("DECLARE it with Start.arrives_tangent() on the target")
                    && !shown.contains(".tangent()"),
                "{spelling}: the seam's ending offers the seam's declaration: {shown}"
            );
        } else {
            assert_eq!(*site, EscalatedAt::Door, "{spelling}");
            assert!(
                shown.contains("if smooth, declare it: .tangent()")
                    && !shown.contains("arrives_tangent"),
                "{spelling}: the junction's ending offers the junction's declarations: {shown}"
            );
        }
    }
    for Row {
        spelling, outcome, ..
    } in rows(50.0, tol)
    {
        outcome.unwrap_or_else(|e| panic!("{spelling} on a 50ε leg builds: {e}"));
    }
}

/// A genuinely smooth joint on metre-long legs is refused the same way,
/// and its recourse fits it: the declaration it offers is the one the
/// joint takes, and followed, it builds.
#[test]
fn a_near_zero_turn_on_a_long_leg_is_refused_with_a_recourse_that_fits() {
    let tol = Tol::witness();
    let sin_turn = 0.1 * tol.eps();
    let leg = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(1.0, 0.0), tol)
        .expect("a metre east");
    let err = leg
        .clone()
        .toward(1.0, sin_turn, tol)
        .expect_err("a turn of ε/10 over a metre is zero at tolerance");
    let PathError::JunctionTurnWithinTolerance { turn } = &err else {
        panic!("expected the turn-within-tolerance refusal, got {err:?}")
    };
    let shown = err.to_string();
    assert_names_its_factors("junction", turn, &shown, [sin_turn, 1.0, 1.0], 1e-9);
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
    assert_names_its_factors("seam", turn, &shown, [0.0, 1.0, 1.0], 1e-9);
    assert!(
        shown.contains("DECLARE it with Start.arrives_tangent() on the target"),
        "the seam's declaration is offered: {shown}"
    );
    tip.line_to(Start.arrives_tangent(), tol)
        .expect("the declared seam closes");
}
