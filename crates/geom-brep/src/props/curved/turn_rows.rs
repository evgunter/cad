//! **Props' period sites agree with the spellings they replaced** (the
//! PRED charter's agreement test, `geom_core::period`'s turn rule): each
//! old spelling is kept here as an oracle, bit for bit and generic over
//! the scalar as the shipped code was, and run against the shipped site
//! on the cases it was written for — a full turn; `τ ± k·e/rate` for
//! `k ∈ {0.5, 3, 20}` at both band edges `e` (zero and escalate); a short
//! arc; a zero-width span; a reversed span where an `Extent` is built;
//! and the eccentric ellipse where a site is `Bounded`. Every case the
//! two disagree on is listed, and every listed case occurs.

use core::f64::consts::{FRAC_PI_2, TAU};

use geom_core::k_stats::decide;
use geom_core::period::Extent;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Tol, Vec3};

use super::{
    apex_loop_sweep, pole_arc_full, require_circle_span, require_cone_conic_spans,
    require_rim_only_closed,
};
use crate::props::{LoopEdge, PropsError};
use geom::Curve3;

/// What a site reached: admitted, refused under a name, or escalated
/// (with the escalation's point margin, where it has one).
#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Admitted,
    Refused(&'static str),
    Escalated(Option<f64>),
}

fn escalated(cause: Indeterminate) -> Outcome {
    Outcome::Escalated(cause.margin.diagnostic_f64_for_error_text().value())
}

fn outcome<V>(got: Result<V, PropsError>) -> Outcome {
    match got {
        Ok(_) => Outcome::Admitted,
        Err(PropsError::Escalated { cause, .. }) => escalated(cause),
        Err(
            PropsError::NotIsoRectangle { what }
            | PropsError::SphereLoop { what }
            | PropsError::ConeLoop { what },
        ) => Outcome::Refused(what),
        Err(other) => panic!("a span door refuses only by name: {other:?}"),
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness band")
}

/// The defects a turn site is read at, in radians at `rate`: none (a full
/// turn), and `±k·e/rate` for `k ∈ {0.5, 3, 20}` at both band edges.
/// Positive is past a turn.
fn defects(band: Band, rate: f64) -> Vec<(f64, f64)> {
    let mut out = vec![(0.0, 0.0)];
    for edge in [band.zero(), band.escalate()] {
        for k in [0.5, 3.0, 20.0] {
            for sign in [1.0, -1.0] {
                out.push((
                    sign * k * (edge / band.zero()).round(),
                    sign * k * edge / rate,
                ));
            }
        }
    }
    out
}

/// **The oracle**: `require_span_within_period` as it was spelled — the
/// forward half `Δt·r` definitely positive, then the winding half
/// `(τ − Δt)·r` not definitely negative, each through `Margin::levered`.
fn old_span<T: Decide>(
    (t0, t1): (T, T),
    radius: T,
    band: Band,
    [forward, winding]: [&'static str; 2],
) -> Outcome {
    let dt = t1 - t0;
    match decide(forward, Margin::levered(dt, radius), band) {
        Err(cause) => return escalated(cause),
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => return Outcome::Refused(forward),
    }
    match decide(winding, Margin::levered(T::tau() - dt, radius), band) {
        Err(cause) => escalated(cause),
        Ok(Sign::Negative) => Outcome::Refused(winding),
        Ok(Sign::Positive | Sign::Zero) => Outcome::Admitted,
    }
}

/// **Every circle span door agrees with its old spelling, bit for bit**:
/// the rim, meridian, sphere-arc and torus-pieces pairs (one door,
/// [`require_circle_span`]) at a radius that is not one, on every
/// defect, a quarter turn, a zero-width span and a reversed one. No
/// case moves: the circle's rate is exact.
#[test]
fn the_circle_span_doors_agree_with_the_old_pair() {
    let band = band();
    let names = [
        ["props_rim_span_forward", "props_rim_span_winding"],
        ["props_meridian_span_forward", "props_meridian_span_winding"],
        [
            "props_sphere_arc_span_forward",
            "props_sphere_arc_span_winding",
        ],
        [
            "props_meridian_pieces_forward",
            "props_meridian_pieces_winding",
        ],
    ];
    let radius = 2.5;
    let t0 = 0.3;
    let mut spans: Vec<(&str, (f64, f64))> = defects(band, radius)
        .into_iter()
        .map(|(_, d)| ("defect", (t0, t0 + TAU + d)))
        .collect();
    spans.push(("a quarter turn", (t0, t0 + FRAC_PI_2)));
    spans.push(("zero width", (t0, t0)));
    spans.push(("reversed", (t0, t0 - 0.5)));
    let refuse = |what| PropsError::NotIsoRectangle { what };
    for pair in names {
        for &(case, span) in &spans {
            let old = old_span(span, radius, band, pair);
            let new = outcome(require_circle_span(span, radius, band, pair, refuse));
            assert_eq!(new, old, "{pair:?}, {case} {span:?}");
        }
    }
    // Both refusals of the forward half occur, and the turn's too.
    let refused = |span| outcome(require_circle_span(span, radius, band, names[0], refuse));
    assert_eq!(refused((t0, t0)), Outcome::Refused(names[0][0]));
    assert_eq!(refused((t0, t0 - 0.5)), Outcome::Refused(names[0][0]));
    assert_eq!(refused((t0, t0 + 2.0 * TAU)), Outcome::Refused(names[0][1]));
}

/// The cone's ellipse door, at `semi_u : semi_v = 16 : 1` (where no case
/// lands on a band edge): the door the cone asks, on one hand-built arc.
fn cone_ellipse(span: (f64, f64), band: Band) -> Outcome {
    let ellipse = Curve3::Ellipse {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        semi_u: 16.0,
        semi_v: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let edge = LoopEdge::hand_built(ellipse, span.0, span.1, true, 0, 1);
    outcome(require_cone_conic_spans(&[edge], band))
}

/// **The cone's ellipse door agrees with its old pair, but for the
/// listed deltas**: the old pair read both halves at the smaller
/// semi-axis `1`; the door now reads the turn between the inf and the
/// sup over the turn's defect, as certification does.
///
/// **The expected deltas**, each listed, all with the defect at the fast
/// side (`θ = π/2`, speed 16); the slow side (`θ = 0`, speed 1) does not
/// move, and nothing moves between admitted and refused:
/// - where the floor reads the defect `Zero` and both bounds over the
///   defect read it in the band (`0.5·ε` of the floor's length, and `3·ε`
///   and `5·ε` of the sup's), the arc was admitted and now escalates —
///   over-wound and short alike, since neither bound settles whether the
///   extent is a whole turn or short of one;
/// - where the floor reads the defect in the band and the inf over the
///   defect reads it definitely past (`3·ε` and `5·ε` of the floor's
///   length, `20·ε` and `30·ε` of the sup's), the arc escalated and is
///   now admitted short of a turn, or refused over one.
#[test]
fn the_cone_ellipse_door_agrees_with_the_old_pair() {
    let band = band();
    let names = [
        "props_cone_ellipse_span_forward",
        "props_cone_ellipse_span_winding",
    ];
    // (side, the rate the defect is a multiple of, k)
    let escalates: [(&str, f64, f64); 6] = [
        ("fast", 1.0, 0.5),
        ("fast", 1.0, -0.5),
        ("fast", 16.0, 3.0),
        ("fast", 16.0, -3.0),
        ("fast", 16.0, 5.0),
        ("fast", 16.0, -5.0),
    ];
    let admits: [(&str, f64, f64); 4] = [
        ("fast", 1.0, -3.0),
        ("fast", 1.0, -5.0),
        ("fast", 16.0, -20.0),
        ("fast", 16.0, -30.0),
    ];
    let refuses: [(&str, f64, f64); 4] = [
        ("fast", 1.0, 3.0),
        ("fast", 1.0, 5.0),
        ("fast", 16.0, 20.0),
        ("fast", 16.0, 30.0),
    ];
    let mut seen = 0;
    for (side, centre, sup) in [("fast", FRAC_PI_2, 16.0), ("slow", 0.0, 1.0)] {
        for rate in [1.0, sup] {
            for (k, d) in defects(band, rate) {
                let t0 = centre - 0.5 * d;
                let span = (t0, t0 + TAU + d);
                let old = old_span(span, 1.0, band, names);
                let new = cone_ellipse(span, band);
                if escalates.contains(&(side, rate, k)) {
                    seen += 1;
                    assert_eq!(old, Outcome::Admitted, "{side} {k}: the listed delta");
                    assert!(
                        matches!(new, Outcome::Escalated(_)),
                        "{side} {k}: the listed delta, got {new:?}"
                    );
                } else if admits.contains(&(side, rate, k)) {
                    seen += 1;
                    assert!(
                        matches!(old, Outcome::Escalated(_)),
                        "{side} {k}: the listed delta, was {old:?}"
                    );
                    assert_eq!(new, Outcome::Admitted, "{side} {k}: the listed delta");
                } else if refuses.contains(&(side, rate, k)) {
                    seen += 1;
                    assert!(
                        matches!(old, Outcome::Escalated(_)),
                        "{side} {k}: the listed delta, was {old:?}"
                    );
                    assert_eq!(
                        new,
                        Outcome::Refused(names[1]),
                        "{side} {k}: the listed delta"
                    );
                } else {
                    assert_eq!(new, old, "{side} at rate {rate}, k = {k}: unlisted");
                }
            }
        }
        let quarter = (centre, centre + FRAC_PI_2);
        assert_eq!(cone_ellipse(quarter, band), Outcome::Admitted, "{side}");
        let reversed = (centre, centre - 0.5);
        assert_eq!(
            cone_ellipse(reversed, band),
            Outcome::Refused(names[0]),
            "{side}: reversed"
        );
    }
    assert_eq!(
        seen,
        escalates.len() + admits.len() + refuses.len(),
        "every listed delta is a case"
    );
}

/// **The oracle**: the apex loop's sweep as it was spelled, the headroom
/// `(τ − |total|)·arm` not definitely negative.
fn old_apex_loop<T: Decide>(total: T, arm: T, band: Band) -> Outcome {
    let name = "props_cone_apex_loop_sweep";
    match decide(name, Margin::levered(T::tau() - total.abs(), arm), band) {
        Err(cause) => escalated(cause),
        Ok(Sign::Negative) => Outcome::Refused(name),
        Ok(Sign::Positive | Sign::Zero) => Outcome::Admitted,
    }
}

/// **A loop through the apex agrees with its old sweep test, but for the
/// listed deltas**, in both senses and on every defect at an arm that is
/// not one.
///
/// **The expected deltas**: a net sweep within the zero band (none at
/// all, or `0.5·ε` of the arm's length) refuses
/// (`props_cone_apex_loop_net`) where the old test read its headroom as a
/// whole turn's and admitted it; one in the band escalates. Such a loop
/// bounds no sector.
#[test]
fn the_apex_loop_sweep_agrees_with_the_old_test() {
    let band = band();
    let arm = 0.75;
    for sense in [1.0, -1.0] {
        for (k, d) in defects(band, arm) {
            let total = sense * (TAU + d);
            assert_eq!(
                outcome(apex_loop_sweep(total, arm, band)),
                old_apex_loop(total, arm, band),
                "sense {sense}, k = {k}"
            );
        }
        let sector = sense * 1.0;
        assert_eq!(
            outcome(apex_loop_sweep(sector, arm, band)),
            Outcome::Admitted
        );
    }
    let eps = band.zero();
    for (total, want) in [
        (0.0, Outcome::Refused("props_cone_apex_loop_net")),
        (
            0.5 * eps / arm,
            Outcome::Refused("props_cone_apex_loop_net"),
        ),
    ] {
        assert_eq!(
            old_apex_loop(total, arm, band),
            Outcome::Admitted,
            "{total}"
        );
        assert_eq!(outcome(apex_loop_sweep(total, arm, band)), want, "{total}");
    }
    let in_band = 3.0 * eps / arm;
    assert_eq!(old_apex_loop(in_band, arm, band), Outcome::Admitted);
    assert!(matches!(
        outcome(apex_loop_sweep(in_band, arm, band)),
        Outcome::Escalated(_)
    ));
}

/// **The oracle**: `props_rim_only_closed` as it was spelled, the excess
/// `(Δu − τ)·arm` within the zero band.
fn old_rim_only_closed<T: Decide>(du: T, arm: T, band: Band) -> Outcome {
    let name = "props_rim_only_closed";
    match decide(name, Margin::levered(du - T::tau(), arm), band) {
        Err(cause) => escalated(cause),
        Ok(Sign::Zero) => Outcome::Admitted,
        Ok(Sign::Positive | Sign::Negative) => Outcome::Refused(name),
    }
}

/// **The rim-only closure agrees with its old excess test**, on every
/// defect and a half rim; its joins are not asked (no edges).
///
/// **The expected delta**: the margin is the headroom `τ − Δu` where it
/// was the excess `Δu − τ`, so an in-band escalation carries the old
/// margin's negation. No classification moves.
#[test]
fn the_rim_only_closure_agrees_with_the_old_excess() {
    let band = band();
    let arm = 1.25;
    let mut negated = 0;
    for (k, d) in defects(band, arm)
        .into_iter()
        .chain([(0.0, -core::f64::consts::PI)])
    {
        let du = TAU + d;
        let extent = Extent::between(0.0, du);
        let old = old_rim_only_closed(du, arm, band);
        let new = outcome(require_rim_only_closed::<f64>(&[], extent, arm, band));
        match (old, new) {
            (Outcome::Escalated(Some(m_old)), Outcome::Escalated(Some(m_new))) => {
                negated += 1;
                assert_eq!(m_new, -m_old, "k = {k}: the margin is negated");
            }
            _ => assert_eq!(new, old, "k = {k}"),
        }
    }
    assert_eq!(negated, 4, "the in-band cases: 3·ε and 5·ε, either side");
}

/// **The oracle**: `props_sphere_pole_arc_full` as it was spelled, the
/// headroom `(τ − span)·r` within the zero band.
fn old_pole_arc_full<T: Decide>(span: T, radius: T, band: Band) -> Result<bool, Outcome> {
    let name = "props_sphere_pole_arc_full";
    decide(name, Margin::levered(T::tau() - span, radius), band)
        .map(|sign| sign == Sign::Zero)
        .map_err(escalated)
}

/// **A sphere arc's whole-circle read agrees with its old test, bit for
/// bit**, on every defect at a radius that is not one: a whole turn is a
/// full circle, any other settled reading is not.
#[test]
fn the_pole_arc_full_read_agrees_with_the_old_test() {
    let band = band();
    let radius = 3.0;
    for (k, d) in defects(band, radius).into_iter().chain([(0.0, -1.0)]) {
        let span = TAU + d;
        let old = old_pole_arc_full(span, radius, band);
        let new = pole_arc_full(Extent::between(0.0, span), radius, band)
            .map_err(|e| outcome::<()>(Err(e)));
        assert_eq!(new, old, "k = {k}");
    }
}
