//! [`Curve3::param_near`] — point-on-carrier parameter recovery, on
//! the branch nearest an anchor.
//!
//! The rows here are the consolidated corpus of the three call sites
//! that used to spell this themselves (a boolean edge split, an offset
//! door's re-anchor, a fillet's seam split). All THREE anchors the
//! sites pass are exercised, because the anchor is the per-site fact
//! the consolidation deliberately left visible:
//!
//! - the SPAN MIDPOINT (`topo`'s boolean edge split) — `near =
//!   (t0 + t1)/2` throughout the span rows;
//! - a moving endpoint's OWN OLD PARAMETER (`topo`'s offset door) — an
//!   arbitrary `near` in the nearest-branch rows;
//! - the CARRIER'S SEAM, `near = 0` (`sweep`'s fillet seam split),
//!   which is what a caller passes when the answer must not depend on
//!   a stored window at all — the anchor is the only input a span
//!   split can rewrite, so a window-derived one makes the recovered
//!   parameter order-dependent. That site's docs carry the
//!   measurement; here it is the `near = 0` column of the
//!   nearest-branch rows.

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
/// It is here as the reason the guard exists, and BOTH span-reading
/// consumers now spell one. `topo`'s edge split spells it as the
/// `bool_split_span_period` decide row. `sweep`'s fillet seam split
/// spells it as a bounds-lane refusal on the stored window — its
/// window test alone is NOT enough, which is exactly what this row
/// shows: an aliased parameter is still strictly interior, so a test
/// that only asks "inside the window?" passes it. (The fillet also
/// anchors at the carrier's seam rather than at the window, for a
/// separate reason its own docs carry.) The offset door anchors at an
/// endpoint's own old parameter and has no span to exceed.
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

/// **THE ANSWER MOVES WITH THE ANCHOR, at the ulp scale — and that is
/// why the anchor is a per-site DECISION and not a convenience.** Two
/// anchors that name the same branch of the same point do NOT have to
/// return the same bits: `near` enters both `atan2` arguments through
/// `eval(near)` and `deriv(near)`, so the frame is re-read at the
/// anchor and rounds differently there.
///
/// The row asserts the sensitivity in BOTH directions, because each
/// direction is load-bearing somewhere:
///
/// - the answers agree to within a few ulps (so no consumer's gate is
///   at risk whichever anchor it picks), AND
/// - at least one pair is not bit-equal (so a consumer that needs a
///   bit-stable answer cannot get one by picking a "nicer" anchor —
///   it has to pick an anchor that never CHANGES).
///
/// That second half is the fact `sweep::fillet::surgery::
/// seam_split_param` is built on. Its anchor used to be the stored
/// window's midpoint, and a stored window is rewritten every time the
/// meridian is split — so the same crossing came back with different
/// bits depending on the order the rims were filleted, and `sweep`'s
/// one-call-vs-sequential composition rows went red. It anchors at the
/// carrier's seam now, the one anchor a split cannot move. If this row
/// ever goes green on the bit-equality half, that site's reasoning has
/// changed and its docs are stale.
#[test]
fn the_answer_moves_with_the_anchor_which_is_why_the_anchor_is_a_site_decision() {
    let mut any_differ = false;
    for carrier in [axis_aligned(), tilted(), wound_negative()] {
        for t in [0.05, 0.4, 1.1, 2.0, 3.0] {
            let p = carrier.eval(t);
            // Anchors that all name the SAME branch of `p` (every one
            // is within half a turn of `t`), as a window sliding over
            // the point would produce.
            let anchors = [0.0, t * 0.5, t, t + 0.3, (t + PI * 0.9) * 0.5];
            let first = carrier.param_near(p, anchors[0]).unwrap();
            for near in anchors {
                let got = carrier.param_near(p, near).unwrap();
                assert!(
                    (got - t).abs() < 1e-12,
                    "t={t} near={near}: {got} is not the same branch"
                );
                assert!(
                    (got - first).abs() < 1e-12,
                    "t={t} near={near}: {got} vs {first} — more than an ulp-scale move"
                );
                if got.to_bits() != first.to_bits() {
                    any_differ = true;
                }
            }
        }
    }
    assert!(
        any_differ,
        "no anchor pair differed in a single bit — the premise the fillet's seam anchor \
         rests on has changed, and `seam_split_param`'s docs are now wrong"
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

/// The seam-anchored longhand, for a row to compare against.
///
/// `θ = atan2(w·v_ref, w·u_ref)`, then `k = floor((near − θ)/τ + ½)`,
/// then `t = θ + k·τ`. This is the retired `replace_face::
/// invert_carrier` spelling transcribed, and it is a FIXTURE here
/// rather than a claim: the rows below say where it agrees with
/// [`Curve3::param_near`] and where it provably does not.
fn retired_seam_anchored(carrier: &Curve3<f64>, p: Point3<f64>, near: f64) -> f64 {
    let (center, axis, u_ref) = match *carrier {
        Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        } => (center, axis, u_ref),
        _ => panic!("a circle fixture"),
    };
    let v_ref = axis.cross(u_ref);
    let w = p - center;
    let theta = w.dot(v_ref).atan2(w.dot(u_ref));
    let k = ((near - theta) / TAU + 0.5).floor();
    theta + k * TAU
}

/// **One question, one answer — STRICTLY INSIDE the half turn.** The
/// retired seam-anchored spelling computes the same branch, transcribed
/// longhand so that a future change to the shared body cannot quietly
/// redefine which branch "nearest" means. Agreement is asserted to a
/// few ulps of a radian, not bitwise: the two forms read the frame
/// differently (evaluator vs stored `u_ref`) and are not required to
/// round alike.
///
/// **The grid runs to ±π and the boundary itself is EXCLUDED here, on
/// purpose** — it is not a gap, it is a measured disagreement, and it
/// has its own row (`at_the_half_turn_boundary_the_two_forms_disagree_
/// by_a_turn_and_both_are_right`) immediately below. A row that stopped
/// the grid short of π would have read as agreement over the whole
/// domain, which is the claim this pair of rows exists to deny.
#[test]
fn the_retired_seam_anchored_form_selects_the_same_branch() {
    for carrier in [axis_aligned(), tilted(), wound_negative()] {
        for near in [-9.0, -1.0, 0.0, 2.5, 11.0] {
            // Right up to the boundary from both sides — the last
            // offsets before the collapse, not a grid that stops at
            // 3.1 and leaves the reader to assume the rest.
            for delta in [-PI + 1e-9, -3.1, -0.5, 0.0, 0.5, 3.1, PI - 1e-9] {
                let t = near + delta;
                let p = carrier.eval(t);
                let seam_anchored = retired_seam_anchored(&carrier, p, near);
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

/// **AT `|δ| = π` THE TWO FORMS DISAGREE BY EXACTLY ONE TURN, in about
/// half the cases, and NEITHER IS WRONG.** This is the boundary the row
/// above stops one ulp short of, and it is the reason that row cannot
/// simply be extended: at exactly half a turn the point has TWO
/// parameters at distance π from the anchor, `near ± π`, and the two
/// spellings pick by different tie-breaks. `param_near` inherits
/// `atan2`'s cut — `Real::atan2(0.0, −1.0) = π` against
/// `atan2(−0.0, −1.0) = −π`, so the sign bit of `w·τ̂` decides — while
/// the longhand's `floor((near − θ)/τ + ½)` breaks the tie by rounding
/// half UP, in the seam frame rather than the anchor frame. Which one
/// each case lands on is not a property of either derivation; it is a
/// coin flip decided by two unrelated last bits.
///
/// **So the property, not the agreement, is what this row asserts**:
/// whatever comes back is a parameter OF THE POINT (`eval` returns to
/// it) and is within half a turn of the anchor. Both spellings satisfy
/// that; only one number satisfies "equal to the other spelling", and
/// that number does not exist.
///
/// MEASURED, on this grid: 12 of the 30 boundary cases come back a full
/// `2π` apart. That is the number quoted at the consumers, and it is
/// why the endpoint-anchored consumer
/// (`topo::replace_face::plan_reanchors`) documents the pose rather
/// than relying on its own point-comparison gate, which compares
/// `carrier.eval(t_new)` to `point` and is blind to a `2π` change in
/// the span it is about to store.
#[test]
fn at_the_half_turn_boundary_the_two_forms_disagree_by_a_turn_and_both_are_right() {
    let mut disagreements = 0;
    let mut cases = 0;
    for carrier in [axis_aligned(), tilted(), wound_negative()] {
        for near in [-9.0, -1.0, 0.0, 2.5, 11.0] {
            for delta in [PI, -PI] {
                cases += 1;
                let t = near + delta;
                let p = carrier.eval(t);
                let got = carrier.param_near(p, near).unwrap();
                let seam_anchored = retired_seam_anchored(&carrier, p, near);

                // (1) The answer is a parameter OF THE POINT.
                assert!(
                    carrier.eval(got).distance(p) < 1e-12,
                    "near={near} delta={delta}: {got} is not a parameter of the point"
                );
                // (2) …on a branch within half a turn of the anchor.
                assert!(
                    (got - near).abs() <= PI + 1e-12,
                    "near={near} delta={delta}: {got} is over half a turn from the anchor"
                );
                // (3) The longhand also satisfies (1) and (2) — which
                // is exactly why the disagreement is a tie and not a
                // defect in either form.
                assert!(
                    carrier.eval(seam_anchored).distance(p) < 1e-12
                        && (seam_anchored - near).abs() <= PI + 1e-12,
                    "near={near} delta={delta}: the longhand {seam_anchored} is not a \
                     legitimate answer either"
                );

                let apart = (got - seam_anchored).abs();
                if apart > 1e-9 {
                    disagreements += 1;
                    assert!(
                        (apart - TAU).abs() < 1e-9,
                        "near={near} delta={delta}: the two forms are {apart} apart, \
                         which is neither agreement nor a whole turn"
                    );
                }
            }
        }
    }
    assert_eq!(cases, 30, "the boundary grid changed size");
    assert_eq!(
        disagreements, 9,
        "the measured boundary coin flip moved; the consumers' documented pose quotes \
         this number"
    );
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
