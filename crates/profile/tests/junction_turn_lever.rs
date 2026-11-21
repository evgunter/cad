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
use std::panic::{AssertUnwindSafe, catch_unwind};

use profile::{
    ArcSweep, Center, EscalatedAt, Gate, LeveredTurn, Open, PathError, Start, TurnLever,
};

/// Runs every row's check, so one red names every row that fails rather
/// than the first.
fn each_row(rows: Vec<Row>, check: impl Fn(Row)) {
    let failed: Vec<&str> = rows
        .into_iter()
        .filter_map(|row| {
            let spelling = row.spelling;
            catch_unwind(AssertUnwindSafe(|| check(row)))
                .is_err()
                .then_some(spelling)
        })
        .collect();
    assert!(failed.is_empty(), "rows failed: {failed:?}");
}

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
/// - the DEPARTING leg short (a line);
/// - the seam with the FIRST leg short (the run continues straight on
///   from it, so that leg's own end is no junction) and a metre-long
///   closing leg.
///
/// The seam with a short CLOSING leg is its own row
/// ([`the_seam_reads_a_short_closing_arc_at_its_arm`]): an arc's chord
/// under ε is refused as a degenerate chord first.
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
            short.line_to(Point2::new(lever + c, s), tol).map(|_| ()),
            bend,
        ),
        junction(
            "a short line_to after a long leg",
            long.line_to(Point2::new(1.0 + lever * c, lever * s), tol)
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
        each_row(
            rows(k, tol),
            |Row {
                 spelling,
                 seam,
                 outcome,
                 want,
             }| {
                let row = format!("{spelling} of {k}ε");
                let err = outcome.expect_err(&row);
                let shown = err.to_string();
                match (seam, &err) {
                    (true, PathError::SeamTurnWithinTolerance { turn }) => {
                        assert_names_its_factors(&row, turn, &shown, want, 1e-3);
                        assert!(
                            shown.contains("DECLARE it with Start.arrives_tangent() on the target"),
                            "{row}: the seam's declaration is offered: {shown}"
                        );
                    }
                    (false, PathError::JunctionTurnWithinTolerance { turn }) => {
                        assert_names_its_factors(&row, turn, &shown, want, 1e-3);
                    }
                    _ => panic!("{row}: expected the turn-within-tolerance refusal, got {err:?}"),
                }
            },
        );
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
    each_row(
        rows(3.0, tol),
        |Row {
             spelling,
             seam,
             outcome,
             ..
         }| {
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
        },
    );
    each_row(
        rows(50.0, tol),
        |Row {
             spelling, outcome, ..
         }| {
            outcome.unwrap_or_else(|e| panic!("{spelling} on a 50ε leg builds: {e}"));
        },
    );
}

/// **An arc's arm can be its radius.** A half-turn arc of radius 20ε
/// departing 2.5° off a metre-long leg: its chord is 40ε, so its arm is
/// the radius, and `sin 2.5° · 20ε` is zero at tolerance. The payload's
/// lever is the radius, the sentence calls it the shorter leg's arm (not
/// its length), and the recourse's "grow an arc's radius" is the lever
/// that moves it: the same turn on a radius of 2000ε builds, though the
/// arc is still a fraction of a micrometre long.
#[test]
fn an_arcs_arm_is_its_radius_when_the_radius_is_the_shorter() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let phi = 2.5_f64.to_radians();
    let (c, s) = (phi.cos(), phi.sin());
    let half_turn = |r: f64| {
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(1.0, 0.0), tol)
            .expect("a metre east")
            .arc_to(
                Center {
                    c: Point2::new(1.0 - r * s, r * c),
                    winding: ArcSweep::Ccw,
                    p: Point2::new(1.0 - 2.0 * r * s, 2.0 * r * c),
                },
                tol,
            )
            .map(|_| ())
    };
    let r = 20.0 * eps;
    let err = half_turn(r).expect_err("sin 2.5° over a 20ε radius is zero at tolerance");
    let PathError::JunctionTurnWithinTolerance { turn } = &err else {
        panic!("expected the turn-within-tolerance refusal, got {err:?}")
    };
    let shown = err.to_string();
    assert_names_its_factors("half-turn arc", turn, &shown, [s, c, r], 1e-3);
    half_turn(2000.0 * eps).expect("the grown radius builds");
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

/// **The seam reads the shorter of its two legs, the closing one here.**
/// A tangent arc of chord 40ε (leaving its chord 20°, so its radius is
/// about 58ε and its arm is the chord) declared off a metre-long leg
/// closes into a metre-long first leg, arriving 1° off it: `sin 1° · 40ε`
/// is zero at tolerance at the closing arc's arm, though it is decided at
/// the first leg's.
#[test]
fn the_seam_reads_a_short_closing_arc_at_its_arm() {
    let tol = Tol::witness();
    let lever = 40.0 * tol.eps();
    // The closing chord runs at −19°; the tangent arc leaves it 20° below
    // (along −39°) and so arrives 20° above, at 1°.
    let [chord, depart, arrive] = [-19.0_f64, -39.0, 1.0].map(f64::to_radians);
    let before = Point2::new(-lever * chord.cos(), -lever * chord.sin());
    let err = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(1.0, 0.0), tol)
        .expect("a metre east")
        .line_to(Point2::new(1.0, -2.0), tol)
        .expect("a real corner")
        .line_to(
            Point2::new(before.x - depart.cos(), before.y - depart.sin()),
            tol,
        )
        .expect("a real corner")
        .line_to(before, tol)
        .expect("a metre-long leg along −39°")
        .tangent()
        .tangent_arc_to(Start, tol)
        .expect_err("the seam's turn is zero at the closing arc's arm");
    let PathError::SeamTurnWithinTolerance { turn } = &err else {
        panic!("expected the seam's turn-within-tolerance refusal, got {err:?}")
    };
    let shown = err.to_string();
    // Arriving at 1°, leaving along +x: a clockwise turn.
    assert_names_its_factors(
        "closing arc",
        turn,
        &shown,
        [-arrive.sin(), arrive.cos(), lever],
        1e-3,
    );
}
