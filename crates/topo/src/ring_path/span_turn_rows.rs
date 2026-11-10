//! **[`span_readings`] against a box named a turn from its copy.** At
//! `Interval`, a circle's or a section arc's parameter is named about
//! its span's middle (`Conic::param_in`, `Oval::param`), and a box that
//! reaches across the middle's antipode is named on one copy whole. On a
//! near-full arc the antipode is the middle of the arc's gap, so a box
//! that starts in the gap and reaches past the start end a turn away is
//! named past the stop end, while some of its points lie inside the
//! span. A reading decided outside there is a wrong answer: the parity
//! passes over a crossing the box may hold.

use super::*;
use core::f64::consts::TAU;
use geom_core::{Band, Interval, Tol};

fn iv(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}

/// The unit circle about `z` at `Interval`, over `span`.
fn arc(span: (f64, f64)) -> CircleArc<Interval> {
    let p = |x, y, z| Point3::new(iv(x), iv(y), iv(z));
    let v = |x, y, z| Vec3::new(iv(x), iv(y), iv(z));
    CircleArc {
        circle: geom::Circle::new(
            geom::Frame::new(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
            iv(1.0),
        ),
        span: (iv(span.0), iv(span.1)),
    }
}

/// The box holding the unit circle's points at angles `a` and `b`.
fn hull(a: f64, b: f64) -> Point3<Interval> {
    let (sa, ca) = a.sin_cos();
    let (sb, cb) = b.sin_cos();
    let h = |x: f64, y: f64| Interval::from_bounds(x.min(y), x.max(y));
    Point3::new(h(ca, cb), h(sa, sb), iv(0.0))
}

/// A near-full arc `(0, τ − g)`, and boxes from just past its stop end
/// across the gap to `reach` past its start end a turn away: each holds
/// the circle's point at `reach`, inside the span, so neither of the
/// span's readings may be decided outside (a straddle, or an
/// escalation, is the honest answer).
#[test]
fn a_box_across_a_near_full_arcs_gap_is_not_read_outside() {
    let band = Band::linear_at(Tol::witness(), 1e-9).unwrap();
    for gap in [0.2, 0.05] {
        let span = (0.0, TAU - gap);
        let a = arc(span);
        for (past_stop, reach) in [(0.01, 0.1), (0.3 * gap, 0.02), (0.45 * gap, 0.5)] {
            let x = hull(span.1 + past_stop, TAU + reach);
            let t = a.param(x);
            let turn = Some(Period::new(Interval::tau()));
            let readings = span_readings(t, a.span, turn, a.circle.radius, iv(0.0), band);
            assert!(
                !readings.contains(&Ok(Sign::Negative)),
                "gap {gap}, from {past_stop} past the stop to {reach} past the start: \
                 named {t:?}, read {readings:?}, but the box holds the point at {reach}, \
                 inside the span"
            );
        }
    }
}
