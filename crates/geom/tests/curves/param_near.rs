//! [`Curve3::param_near`] — point-on-carrier parameter recovery, on
//! the branch nearest an anchor.
//!
//! The rows here are the consolidated corpus of the three call sites
//! that used to spell this themselves (a boolean edge split, an offset
//! door's re-anchor, a fillet's seam split). Two of those three read
//! the parameter INSIDE a stored span by anchoring at the span's
//! MIDPOINT and one anchors at a moving endpoint's OWN old parameter,
//! so both postures are exercised: `near = (t0 + t1)/2` throughout the
//! span rows, and an arbitrary `near` in the nearest-branch rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Point3, Vec3};

/// A circle in an exactly-orthonormal tilted frame: an integer
/// orthogonal triple over 3, so the frame is unit and orthogonal to
/// rounding-free precision.
fn tilted() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(-0.5, 4.0, 1.25),
        axis: Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0),
        radius: 2.5,
        u_ref: Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0),
    }
}

fn axis_aligned() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(1.0, 2.0, 3.0),
        axis: Vec3::unit_z(),
        radius: 1.5,
        u_ref: Vec3::unit_x(),
    }
}

/// A circle wound the OTHER way (axis = −z), so increasing `t` runs
/// clockwise in the global frame. No claim here may depend on the
/// sense.
fn wound_negative() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(1.0, 2.0, 3.0),
        axis: -Vec3::unit_z(),
        radius: 1.5,
        u_ref: Vec3::unit_x(),
    }
}

fn center_of(c: &Curve3<f64>) -> Point3<f64> {
    match c {
        Curve3::Circle { center, .. } => *center,
        _ => panic!("a circle fixture"),
    }
}

fn mid(t0: f64, t1: f64) -> f64 {
    (t0 + t1) * 0.5
}

// ---------------------------------------------------------------
// The midpoint-anchored posture: a parameter recovered inside a span.
// ---------------------------------------------------------------

/// The parameter is recovered on the span's OWN period, for spans
/// placed anywhere on the parameter line — including one that
/// straddles the seam and one that sits entirely on negative
/// parameters, which is exactly what a seam-anchored `atan2` plus a
/// winding correction would have to select a branch for.
#[test]
fn the_midpoint_anchor_recovers_the_parameter_on_the_spans_own_period() {
    for carrier in [axis_aligned(), tilted()] {
        for (t0, t1) in [
            (0.0, PI),
            (0.4, 2.9),
            (-5.0, -2.0),
            (2.5, 2.5 + TAU * 0.9),
            (PI, 3.0 * PI),
        ] {
            for f in [0.05, 0.25, 0.5, 0.75, 0.95] {
                let t = t0 + (t1 - t0) * f;
                let p = carrier.eval(t);
                let got = carrier.param_near(p, mid(t0, t1)).unwrap();
                assert!(
                    (got - t).abs() < 1e-12,
                    "span ({t0}, {t1}) at {t}: got {got}"
                );
                assert!(got > t0 && got < t1, "span ({t0}, {t1}) at {t}: {got}");
            }
        }
    }
}

/// A FULL-period span has no interior seam problem either: the mid
/// anchor sits half a turn from both endpoints, so the two points
/// nearest the seam land just inside the span rather than a period
/// away.
#[test]
fn a_full_period_span_places_both_seam_neighbours_inside_itself() {
    let carrier = axis_aligned();
    for t in [1e-6, TAU - 1e-6, PI - 0.1, PI + 0.1] {
        let got = carrier.param_near(carrier.eval(t), mid(0.0, TAU)).unwrap();
        assert!((got - t).abs() < 1e-9, "at {t}: got {got}");
    }
}

/// Both winding senses, both frames, spans up to a hair under a
/// period: the "nearest branch to the anchor" claim is about the
/// parameterization, not about the global orientation.
#[test]
fn both_winding_senses_recover_the_parameter() {
    for carrier in [axis_aligned(), wound_negative(), tilted()] {
        for (t0, t1) in [(0.0, PI), (-5.0, -2.0), (2.5, 2.5 + TAU * 0.999)] {
            for f in [0.01, 0.5, 0.99] {
                let t = t0 + (t1 - t0) * f;
                let got = carrier.param_near(carrier.eval(t), mid(t0, t1)).unwrap();
                assert!((got - t).abs() < 1e-9, "span ({t0},{t1}) at {t}: {got}");
            }
        }
    }
}

/// A TINY span (a hair of arc): the mid anchor is a hair from both
/// endpoints, so `δ` is tiny and the answer is still exact and still
/// strictly inside.
#[test]
fn a_tiny_span_still_recovers_the_parameter() {
    let carrier = axis_aligned();
    for w in [1e-3, 1e-6, 1e-9] {
        let (t0, t1) = (1.0, 1.0 + w);
        let t = t0 + w * 0.5001;
        let got = carrier.param_near(carrier.eval(t), mid(t0, t1)).unwrap();
        assert!((got - t).abs() < w * 1e-6, "w={w} t={t} got={got}");
        assert!(got > t0 && got < t1, "w={w}: {got} outside ({t0},{t1})");
    }
}

/// **`δ = ±π` EXACTLY.** On a full-period span both endpoints sit at
/// `|δ| = π` from the midpoint, and `atan2`'s principal branch returns
/// `+π` for both — so `t₀` maps to `t₁`. This is the one place the
/// principal-branch sentence is not literally true, and the row pins
/// the reason it is harmless: the answer is an ENDPOINT, and every
/// consumer that splits at this parameter refuses a split at an
/// endpoint whichever of the two it names.
#[test]
fn the_exact_half_period_boundary_lands_on_an_endpoint() {
    let carrier = axis_aligned();
    let (t0, t1) = (0.0, TAU);
    for t in [t0, t1] {
        let got = carrier.param_near(carrier.eval(t), mid(t0, t1)).unwrap();
        // WHICH of the two endpoints comes back is decided by the last
        // bit of `sin(π)`, so the row asserts the property that matters
        // and not the coin flip.
        assert!(
            (got - t0).abs() < 1e-12 || (got - t1).abs() < 1e-12,
            "a period endpoint maps to an endpoint: t={t} got={got}"
        );
        assert!(
            !(got > t0 + 1e-12 && got < t1 - 1e-12),
            "an endpoint is never strictly interior: {got}"
        );
    }
}

/// **The alias past a period, made visible.** The doc's second
/// precondition says a caller reading a span wider than one period
/// owes its own period guard, because the answer aliases by `2π` and
/// the parameter still lands INSIDE the span — nothing downstream can
/// see it. This row is that failure, asserted: on a span of `1.5`
/// periods the point at `t = t₁ − ε` comes back a whole turn low, and
/// still strictly interior.
///
/// It is here as the reason the guard exists. `topo`'s edge split
/// spells the guard as the `bool_split_span_period` row; the fillet's
/// seam split never sees a span this wide and refuses anything outside
/// the span outright; the offset door anchors at an endpoint's own old
/// parameter and has no span to exceed.
#[test]
fn past_one_period_the_answer_aliases_by_a_turn_inside_the_span() {
    let carrier = axis_aligned();
    let (t0, t1) = (0.0, TAU * 1.5);
    let t = t1 - 1e-3;
    let got = carrier.param_near(carrier.eval(t), mid(t0, t1)).unwrap();
    assert!(
        (got - (t - TAU)).abs() < 1e-9,
        "the aliased answer is a turn low: t={t} got={got}"
    );
    assert!(
        got > t0 && got < t1,
        "and it is still strictly inside the span, which is why the caller owes the guard: {got}"
    );
}

// ---------------------------------------------------------------
// The endpoint-anchored posture: the branch nearest a given anchor.
// ---------------------------------------------------------------

/// The answer is the unique branch within half a turn of `near`, for
/// an anchor anywhere on the parameter line. This is the offset door's
/// posture: it anchors at the endpoint's OWN old parameter so the
/// re-anchored endpoint keeps the turn its stored range was on.
#[test]
fn the_answer_is_the_branch_within_half_a_turn_of_the_anchor() {
    for carrier in [axis_aligned(), tilted(), wound_negative()] {
        for near in [-9.0, -PI, 0.0, 0.7, PI, 7.0, 13.5] {
            for delta in [-3.0, -1.0, -1e-9, 0.0, 1e-9, 0.4, 3.0] {
                let t = near + delta;
                let got = carrier.param_near(carrier.eval(t), near).unwrap();
                assert!(
                    (got - t).abs() < 1e-9,
                    "near={near} delta={delta}: got {got}"
                );
                assert!(
                    (got - near).abs() <= PI + 1e-12,
                    "near={near} delta={delta}: {got} is over half a turn from the anchor"
                );
            }
        }
    }
}

/// **One question, one answer**: the retired seam-anchored spelling —
/// `θ = atan2(w·v_ref, w·u_ref)` then `k = floor((near − θ)/τ + ½)`,
/// `t = θ + k·τ` — computes the same branch, transcribed here longhand
/// so that a future change to the shared body cannot quietly redefine
/// which branch "nearest" means. Agreement is asserted to a few ulps
/// of a radian, not bitwise: the two forms read the frame differently
/// (evaluator vs stored `u_ref`) and are not required to round alike.
#[test]
fn the_retired_seam_anchored_form_selects_the_same_branch() {
    for carrier in [axis_aligned(), tilted(), wound_negative()] {
        let (center, axis, u_ref) = match carrier {
            Curve3::Circle {
                center,
                axis,
                u_ref,
                ..
            } => (center, axis, u_ref),
            _ => panic!("a circle fixture"),
        };
        let v_ref = axis.cross(u_ref);
        for near in [-9.0, -1.0, 0.0, 2.5, 11.0] {
            for delta in [-3.1, -0.5, 0.0, 0.5, 3.1] {
                let t = near + delta;
                let p = carrier.eval(t);
                let w = p - center;
                let theta = w.dot(v_ref).atan2(w.dot(u_ref));
                let k = ((near - theta) / TAU + 0.5).floor();
                let seam_anchored = theta + k * TAU;
                let got = carrier.param_near(p, near).unwrap();
                assert!(
                    (got - seam_anchored).abs() < 1e-12,
                    "near={near} delta={delta}: mid-anchored {got} vs seam-anchored \
                     {seam_anchored}"
                );
            }
        }
    }
}

// ---------------------------------------------------------------
// The other arms.
// ---------------------------------------------------------------

/// A LINE has no branch: the answer is the projection on `dir` and it
/// does not move with the anchor. The row pins the anchor-independence
/// rather than merely the value, because the shared signature carries
/// an argument this arm ignores.
#[test]
fn the_line_arm_is_the_projection_and_ignores_the_anchor() {
    let line = Curve3::Line {
        origin: Point3::new(0.5, -1.0, 2.0),
        dir: Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0),
    };
    for t in [-4.0, 0.0, 0.25, 17.0] {
        let p = line.eval(t);
        let first = line.param_near(p, 0.0).unwrap();
        for near in [-100.0, 0.0, 3.5, 1e6] {
            let got = line.param_near(p, near).unwrap();
            assert_eq!(
                got.to_bits(),
                first.to_bits(),
                "t={t} near={near}: the line arm moved with its anchor"
            );
        }
        assert!((first - t).abs() < 1e-12, "t={t}: got {first}");
    }
}

/// The kinds whose inversion is a solve refuse by returning `None` —
/// the typed refusal each consumer turns into its own error. An
/// ellipse's `θ` is the eccentric anomaly, NOT the polar angle, so the
/// circle arm's arithmetic would be silently wrong here rather than
/// merely imprecise; that is why the arm is absent and not
/// approximated.
#[test]
fn the_ellipse_and_nurbs_arms_refuse() {
    let ellipse = Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        major: 2.0,
        minor: 1.0,
        u_ref: Vec3::unit_x(),
    };
    assert!(ellipse.param_near(ellipse.eval(0.7), 0.7).is_none());

    let nurbs = Curve3::<f64>::nurbs_placeholder();
    assert!(nurbs.param_near(Point3::new(0.0, 0.0, 0.0), 0.0).is_none());
}

/// The answer is about the point's RADIAL PROJECTION — the caller's
/// on-carrier precondition is what makes that the event's own
/// parameter, and this row says what the arithmetic does without it.
#[test]
fn an_off_carrier_point_answers_about_its_radial_projection() {
    let carrier = axis_aligned();
    let center = center_of(&carrier);
    let t = 1.1;
    let on = carrier.eval(t);
    // Push off the circle radially and axially.
    let off = on + (on - center) * 0.3 + Vec3::unit_z() * 0.7;
    let got = carrier.param_near(off, mid(0.0, PI)).unwrap();
    assert!((got - t).abs() < 1e-12, "got {got}");
}
