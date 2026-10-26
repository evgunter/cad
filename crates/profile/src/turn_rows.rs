//! **`profile`'s period sites agree with the spellings they replaced**
//! (the PRED charter's agreement test, `geom_core::period`'s turn rule):
//! an endpoint-free leg's sweep gates (`path_arc_sweep`,
//! `path_arc_sweep_full`) and validation's `arc_sweep_range`. Each old
//! spelling is an oracle here, bit for bit and generic over the scalar
//! as the shipped code was, run against the shipped site on a full turn,
//! `τ ± k·e/r` for `k ∈ {0.5, 3, 20}` at both band edges `e`, the same
//! small sweeps near zero in both senses, and a half turn.
#![allow(clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::k_stats::decide;
use geom_core::{Arc2, Band, Decide, Margin, Point2, Sign, Tol};

use crate::path::PathError;
use crate::path::verbs::sweep_gates;
use crate::seg::{Range, SegIssue, ShapeIssue, sweep_range};
use crate::validate::ArcCheck;

/// What a site reached.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Admitted,
    /// The sweep is degenerate (the path's), or inconsistent with its
    /// range (validation's).
    Refused,
    /// Not short of a full turn (the path's).
    FullTurn,
    /// Escalated, under a name.
    Escalated(Option<&'static str>),
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness band")
}

/// The multiples `k·e/ε` a sweep is read at, both senses: `k ∈ {0.5, 3,
/// 20}` at both band edges `e`.
fn multiples(band: Band) -> Vec<f64> {
    let mut out = Vec::new();
    for edge in [band.zero(), band.escalate()] {
        for k in [0.5, 3.0, 20.0] {
            out.extend([
                k * (edge / band.zero()).round(),
                -k * (edge / band.zero()).round(),
            ]);
        }
    }
    out
}

/// **The oracle**: the leg's two gates as they were spelled — the angle
/// bare against the band, then the arc length short of a turn, each
/// through `Margin::of`.
fn old_gates<T: Decide>(r: T, angle: T, band: Band) -> Outcome {
    match decide("path_arc_sweep", Margin::of(angle), band) {
        Err(cause) => return Outcome::Escalated(cause.predicate),
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => return Outcome::Refused,
    }
    match decide(
        "path_arc_sweep_full",
        Margin::of(r * (T::tau() - angle)),
        band,
    ) {
        Err(cause) => Outcome::Escalated(cause.predicate),
        Ok(Sign::Positive) => Outcome::Admitted,
        Ok(Sign::Zero | Sign::Negative) => Outcome::FullTurn,
    }
}

fn new_gates(r: f64, angle: f64, band: Band) -> Outcome {
    match sweep_gates(r, angle, band) {
        Ok(()) => Outcome::Admitted,
        Err(PathError::DegenerateArcSpec { .. }) => Outcome::Refused,
        Err(PathError::ArcSweepNotShortOfFullTurn { .. }) => Outcome::FullTurn,
        Err(PathError::Escalated { source }) => Outcome::Escalated(source.predicate),
        Err(other) => panic!("the gates refuse only as gates: {other:?}"),
    }
}

/// **A leg's sweep gates agree with the old pair, but for the listed
/// deltas**, at a radius of 4.
///
/// **The expected deltas**: `path_arc_sweep` read the bare angle (radians)
/// against the metres band; it now reads the arc length `r·θ`. An angle
/// of `k·e/r` now reads `k·e` where it read `k·e/4`, so a small angle can
/// land in a band one step further from zero than it did: `±3` (old
/// `Zero`, now in the band), `20` and `30` (old in the band, now
/// admitted), and `−20` and `−30` (old in the band, now refused). Near a
/// full turn nothing moves: the arc length short of it is the same
/// product.
#[test]
fn the_sweep_gates_agree_with_the_old_pair() {
    let band = band();
    let r = 4.0;
    let eps = band.zero();
    let deltas = [3.0, -3.0, 20.0, -20.0, 30.0, -30.0];
    let mut seen = 0;
    for k in multiples(band) {
        let small = k * eps / r;
        let (old, new) = (old_gates(r, small, band), new_gates(r, small, band));
        if deltas.contains(&k) {
            seen += 1;
            assert_ne!(new, old, "k = {k}: the listed delta");
        } else {
            assert_eq!(new, old, "k = {k}: near zero");
        }
        let full = TAU - small;
        assert_eq!(
            new_gates(r, full, band),
            old_gates(r, full, band),
            "k = {k}: near a turn"
        );
    }
    assert_eq!(seen, deltas.len(), "every listed delta is a case");
    for angle in [0.0, PI, TAU, 2.0 * TAU] {
        assert_eq!(
            new_gates(r, angle, band),
            old_gates(r, angle, band),
            "{angle}"
        );
    }
}

/// **The oracle**: `arc_sweep_range` as it was spelled — one product
/// margin, `r·|Δθ|·(τ − |Δθ|)/τ` for a chord arc (`Positive` accepted),
/// `r·(τ − |Δθ|)` for a full turn (`Zero` accepted).
fn old_range<T: Decide>(arc: Arc2<T>, range: Range, band: Band) -> Outcome {
    let tau = T::tau();
    let span = arc.sweep.abs();
    let (ratio, accept) = match range {
        Range::Chord => (span * (tau - span) / tau, Sign::Positive),
        Range::FullTurn => (tau - span, Sign::Zero),
    };
    match decide("arc_sweep_range", Margin::levered(ratio, arc.radius), band) {
        Err(cause) => Outcome::Escalated(cause.predicate),
        Ok(sign) if sign == accept => Outcome::Admitted,
        Ok(_) => Outcome::Refused,
    }
}

fn new_range(arc: Arc2<f64>, range: Range, band: Band) -> Outcome {
    match sweep_range(arc, range, band) {
        Ok(()) => Outcome::Admitted,
        Err(SegIssue::Inconsistent {
            check: ArcCheck::SweepRange,
        }) => Outcome::Refused,
        Err(SegIssue::Shape(ShapeIssue::Escalated(source))) => Outcome::Escalated(source.predicate),
        Err(_) => panic!("the range check refuses only as itself"),
    }
}

/// **`arc_sweep_range` agrees with its old product**, for both ranges,
/// in both senses, near zero and near a full turn. The product and the
/// two margins it is split into (the arc length, then the gap to a turn)
/// agree to a factor `1 − |Δθ|/τ` near zero and `|Δθ|/τ` near a turn,
/// so no verdict here moves; an in-band escalation's margin is the arc
/// length or the gap, where it was their product's.
///
/// **The expected delta**: a sweep in the band near zero escalates under
/// the arc length's own row, `arc_sweep_length`, where it escalated under
/// `arc_sweep_range`.
#[test]
fn the_arc_sweep_range_agrees_with_the_old_product() {
    let band = band();
    let radius = 2.5;
    let eps = band.zero();
    let arc = |sweep| Arc2 {
        centre: Point2::new(0.0, 0.0),
        radius,
        sweep,
    };
    let mut near_turn = vec![PI, TAU, -PI, -TAU];
    let mut near_zero = vec![0.0];
    for k in multiples(band) {
        let d = k * eps / radius;
        near_turn.extend([TAU + d, -(TAU + d)]);
        near_zero.push(d);
    }
    let mut renamed = 0;
    for &sweep in near_turn.iter().chain(&near_zero) {
        let (new, old) = (
            new_range(arc(sweep), Range::Chord, band),
            old_range(arc(sweep), Range::Chord, band),
        );
        if near_zero.contains(&sweep) && old == Outcome::Escalated(Some("arc_sweep_range")) {
            renamed += 1;
            assert_eq!(
                new,
                Outcome::Escalated(Some("arc_sweep_length")),
                "Δθ = {sweep}: the listed delta"
            );
        } else {
            assert_eq!(new, old, "a chord arc, Δθ = {sweep}");
        }
    }
    assert_eq!(renamed, 4, "the in-band sweeps near zero: ±3·ε and ±5·ε");
    // A full turn's range is asked only past `full_turn`'s reach gate,
    // so never of a sweep near zero.
    for sweep in near_turn {
        assert_eq!(
            new_range(arc(sweep), Range::FullTurn, band),
            old_range(arc(sweep), Range::FullTurn, band),
            "a full turn, Δθ = {sweep}"
        );
    }
}

/// **Each reading of `arc_sweep_range`'s check has its own row**, so a
/// recorded sign says which reading it is: the arc length's sign is the
/// turn sense (`arc_sweep_length`, a clockwise arc `Negative` and
/// accepted, a zero sweep `Zero` and refused), and the gap's says where
/// the sweep stands against a turn (`arc_sweep_range`, `Positive` on a
/// chord arc accepted, `Negative` past a turn refused, `Zero` on a full
/// turn accepted).
#[test]
fn each_reading_of_the_range_check_has_its_own_row() {
    use geom_core::k_stats::Bracket;
    let band = band();
    let arc = |sweep| Arc2 {
        centre: Point2::new(0.0, 0.0),
        radius: 2.5,
        sweep,
    };
    let signs = |sweep: f64, range: Range| {
        let bracket = Bracket::open();
        let out = new_range(arc(sweep), range, band);
        let rec = bracket.finish();
        let on = |name: &str| -> Vec<Sign> {
            rec.verdicts
                .iter()
                .filter(|v| v.predicate == name)
                .map(|v| v.sign)
                .collect()
        };
        (out, on("arc_sweep_length"), on("arc_sweep_range"))
    };
    let want = |sign: Sign| vec![sign];
    assert_eq!(
        signs(-PI / 2.0, Range::Chord),
        (
            Outcome::Admitted,
            want(Sign::Negative),
            want(Sign::Positive)
        ),
        "a clockwise chord arc"
    );
    assert_eq!(
        signs(1.5 * TAU, Range::Chord),
        (Outcome::Refused, want(Sign::Positive), want(Sign::Negative)),
        "a chord arc past a turn"
    );
    assert_eq!(
        signs(0.0, Range::Chord),
        (Outcome::Refused, want(Sign::Zero), vec![]),
        "a zero sweep"
    );
    assert_eq!(
        signs(TAU, Range::FullTurn),
        (Outcome::Admitted, want(Sign::Positive), want(Sign::Zero)),
        "a full turn"
    );
}
