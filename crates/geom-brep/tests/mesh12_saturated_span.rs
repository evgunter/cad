//! **A sphere meridian span past the winding bound refuses at the
//! parse** (issue 1601). Certification bounds every edge's stored
//! span `0 < Δt ≤ τ` (`interval_span_winding`, banded at the linear
//! band with the carrier radius as the lever), and the closed form's
//! pole fold rests on that bound: its membership test has an empty
//! zero set only while the span is at most a period. A span the
//! certified world cannot produce is not a datum the closed form may
//! answer, so the sphere parse re-decides the bound per meridian arc
//! as `props_meridian_span_winding` — the same margin, the same band,
//! the same lever as certification — and refuses typed, on every
//! consumer, under one name.
//!
//! Every offset below is derived from the run's own `Band`, never from
//! an ε literal: this file is on CI's `eps ∈ {default, 1e-6, 1e-12}`
//! matrix and runs on the interval lane.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{
    LoopEdge, MaterialSign, PropsError, boundary_material_sign, curved_face, require_iso_rectangle,
    require_one_chart_branch,
};
use geom_core::Tol;
use geom_core::{Band, Point3, Real, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;
/// The decide every consumer refuses under.
const NAME: &str = "props_meridian_span_winding";

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn f<T: Real>(x: f64) -> T {
    T::from_f64(x)
}
fn sphere<T: Real>() -> Surface<T> {
    Surface::Sphere {
        center: Point3::new(f(0.0), f(0.0), f(0.0)),
        radius: f(RS),
        axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
    }
}
/// The meridian great circle at azimuth `u`; its parameter is the
/// latitude on the `u` side, so `t = π/2` is the north pole.
fn great<T: Real>(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
    let carrier = Curve3::Circle {
        center: Point3::new(f(0.0), f(0.0), f(0.0)),
        axis: Vec3::new(f(u.sin()), f(-u.cos()), f(0.0)),
        radius: f(RS),
        u_ref: Vec3::new(f(u.cos()), f(u.sin()), f(0.0)),
    };
    let (lo, hi, forward) = if t0 < t1 {
        (t0, t1, true)
    } else {
        (t1, t0, false)
    };
    LoopEdge::hand_built(carrier, f(lo), f(hi), forward, a, b)
}
/// A rimless pair on one great circle: the arc `[t0, t0 + dt]` and
/// the complementary arc back to `t0 + 4π`, so the two spans sum to
/// two periods and the closed form (when it answers) is the
/// hemisphere pair's `2πR²`.
fn pair<T: Real>(t0: f64, dt: f64) -> Vec<LoopEdge<T>> {
    vec![
        great(0.0, t0, t0 + dt, 0, 1),
        great(0.0, t0 + dt, t0 + 4.0 * PI, 1, 0),
    ]
}
/// A span past the bound whose north pole is δ inside it — the shape
/// MESH-11's review measured folding short on 36 of 400 spans.
fn saturated<T: Real>(delta: f64) -> Vec<LoopEdge<T>> {
    pair(PI / 2.0 - delta, TAU + 2.0 * delta)
}
fn is_winding_refusal<V: core::fmt::Debug>(r: &Result<V, PropsError>) -> bool {
    matches!(r, Err(PropsError::NotIsoRectangle { what }) if *what == NAME)
}
fn is_winding_escalation<V: core::fmt::Debug>(r: &Result<V, PropsError>) -> bool {
    matches!(
        r,
        Err(PropsError::Escalated { cause }) if cause.predicate == Some(NAME)
    )
}

// ---------------------------------------------------------------------
// One name, every consumer
// ---------------------------------------------------------------------

/// **The flux lane.** CERT-1's `3π + π` pair used to measure `2πR²`
/// through a clamp whose zero set was not empty; a 3π span is not an
/// arc certification produces and the closed form now refuses it by
/// the bound's own name rather than answering over an uncertified
/// premise.
#[test]
fn the_flux_lane_refuses_a_span_past_the_winding_bound() {
    let r = curved_face(&sphere::<f64>(), &pair::<f64>(0.0, 3.0 * PI), 1.0, band());
    assert!(is_winding_refusal(&r), "{r:?}");
}

/// **The material sign** shares the parse and refuses before it can
/// answer `Unencoded` for the rimless pair.
#[test]
fn the_material_sign_refuses_a_span_past_the_winding_bound() {
    let r = boundary_material_sign(&sphere::<f64>(), &pair::<f64>(0.0, 3.0 * PI), band());
    assert!(is_winding_refusal(&r), "{r:?}");
}

/// **The shape door** shares the parse too: a rimless band is a chart
/// rectangle, but not one stated over a span no edge may carry.
#[test]
fn the_shape_door_refuses_a_span_past_the_winding_bound() {
    let r = require_iso_rectangle(&sphere::<f64>(), &pair::<f64>(0.0, 3.0 * PI), band());
    assert!(is_winding_refusal(&r), "{r:?}");
}

/// **The branch door** does not run the parse; it decides the span
/// itself, before the pole test, so the refusal it reports is the
/// bound's and not `NotOneChartBranch` for a pole the span happens to
/// contain twice.
#[test]
fn the_branch_door_refuses_a_span_past_the_winding_bound() {
    let r = require_one_chart_branch(&sphere::<f64>(), &pair::<f64>(0.0, 3.0 * PI), band());
    assert!(is_winding_refusal(&r), "{r:?}");
}

/// **The 36-of-400 class, closed by refusal on every span**, not by
/// folding it right: each `2π + 2δ` span of the review's sweep refuses
/// under the one name at the flux lane and at the branch door.
#[test]
fn every_saturated_span_of_the_review_sweep_refuses() {
    let bd = band();
    let mut n = 0;
    for k in 1..=400 {
        let delta = 0.001 * f64::from(k) + 1e-7 * f64::from(k * k);
        if delta >= 1.0 {
            break;
        }
        n += 1;
        let edges = saturated::<f64>(delta);
        let fc = curved_face(&sphere::<f64>(), &edges, 1.0, bd);
        let door = require_one_chart_branch(&sphere::<f64>(), &edges, bd);
        assert!(is_winding_refusal(&fc), "δ = {delta}: flux lane {fc:?}");
        assert!(
            is_winding_refusal(&door),
            "δ = {delta}: branch door {door:?}"
        );
    }
    assert!(n >= 400, "the sweep must cover the review's 400 spans");
}

// ---------------------------------------------------------------------
// The band: certification's own
// ---------------------------------------------------------------------

/// **The bound is decided at certification's band, with certification's
/// dispositions.** `interval_span_winding` passes `Zero` and
/// `Positive` headroom and escalates the indeterminate band, so a
/// certified edge's span exceeds τ by at most `zero/R` radians; this
/// decide admits exactly what certification admits, escalates where it
/// escalates, and refuses where it refuses — a span the certified world
/// hands the parse is never refused here, and a span it never hands
/// over never gets an answer.
///
/// The admitted rungs are also the row the helper's doc owes: a span
/// inside the coincidence band above τ REACHES the pole arithmetic,
/// and the fold measures the hemisphere pair exactly there — the
/// membership edge cosine is within `(zero/2R)²/2` of its half-turn
/// value on such a span, which can reclassify only a pole within
/// `zero/2R` of the span's endpoint, whose margin is in-band on both
/// dispositions.
#[test]
fn the_span_decide_admits_and_refuses_at_certifications_band() {
    let bd = band();
    let exact = 2.0 * PI * RS * RS;
    let t0 = 0.3;
    let admitted = [0.0, 0.5 * bd.zero() / RS, 0.99 * bd.zero() / RS];
    for eta in admitted {
        let edges = pair::<f64>(t0, TAU + eta);
        let fc = curved_face(&sphere::<f64>(), &edges, 1.0, bd)
            .unwrap_or_else(|e| panic!("τ + {eta:e}: the flux lane must answer: {e:?}"));
        let rel = (fc.area - exact).abs() / exact;
        assert!(rel < 1e-12, "τ + {eta:e}: area {:e} != {exact:e}", fc.area);
        assert_eq!(
            require_iso_rectangle(&sphere::<f64>(), &edges, bd),
            Ok(()),
            "τ + {eta:e}: a rimless band is a chart rectangle"
        );
        assert_eq!(
            boundary_material_sign(&sphere::<f64>(), &edges, bd),
            Ok(MaterialSign::Unencoded),
            "τ + {eta:e}: the rimless pair encodes no side"
        );
        // The span is admitted; what the branch door then refuses is
        // the ARC, for the pole the full turn contains — its own
        // question, its own name.
        assert!(
            matches!(
                require_one_chart_branch(&sphere::<f64>(), &edges, bd),
                Err(PropsError::NotOneChartBranch { edge: 0, .. })
            ),
            "τ + {eta:e}: the span is admitted and the pole test runs"
        );
    }
    let mid = 0.5 * (bd.zero() + bd.escalate()) / RS;
    for eta in [1.01 * bd.zero() / RS, mid, 0.99 * bd.escalate() / RS] {
        let edges = pair::<f64>(t0, TAU + eta);
        let fc = curved_face(&sphere::<f64>(), &edges, 1.0, bd);
        let door = require_one_chart_branch(&sphere::<f64>(), &edges, bd);
        assert!(is_winding_escalation(&fc), "τ + {eta:e}: flux lane {fc:?}");
        assert!(
            is_winding_escalation(&door),
            "τ + {eta:e}: branch door {door:?}"
        );
    }
    for eta in [1.01 * bd.escalate() / RS, 20.0 * bd.escalate() / RS] {
        let edges = pair::<f64>(t0, TAU + eta);
        let fc = curved_face(&sphere::<f64>(), &edges, 1.0, bd);
        let door = require_one_chart_branch(&sphere::<f64>(), &edges, bd);
        assert!(is_winding_refusal(&fc), "τ + {eta:e}: flux lane {fc:?}");
        assert!(
            is_winding_refusal(&door),
            "τ + {eta:e}: branch door {door:?}"
        );
    }
}

/// **A rim is not decided here.** The bound is re-decided on MERIDIAN
/// arcs, whose span feeds the pole fold; a rim's span feeds the `Δu`
/// sum, a different premise with a different home, and a full-period
/// rim (the scaffolding shape every revolve mints) must keep passing
/// the parse.
#[test]
fn a_full_period_rim_is_not_a_meridian_span() {
    let bd = band();
    let cap: Vec<LoopEdge<f64>> = vec![
        LoopEdge::hand_built(
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, RS * 0.3f64.sin()),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: RS * 0.3f64.cos(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            0.0,
            TAU,
            true,
            0,
            0,
        ),
        great(0.0, 0.3, PI / 2.0, 0, 1),
        great(PI, PI / 2.0, 0.3, 1, 0),
    ];
    let r = require_iso_rectangle(&sphere::<f64>(), &cap, bd);
    assert!(
        !is_winding_refusal(&r) && !is_winding_escalation(&r),
        "{r:?}"
    );
}

/// **The interval lane decides the same way.** The margin is
/// `(τ − Δt)·R`, an enclosure of rounding width on any admitted span,
/// so the three dispositions land where the `f64` rows land: the 3π
/// pair refuses at every consumer, a span inside the coincidence band
/// above τ is answered, and the indeterminate band escalates.
#[cfg(feature = "interval")]
#[test]
fn the_span_decide_holds_at_the_interval_scalar() {
    use geom_core::Interval;
    let bd = band();
    let three_pi = pair::<Interval>(0.0, 3.0 * PI);
    let fc = curved_face(
        &sphere::<Interval>(),
        &three_pi,
        Interval::from_f64(1.0),
        bd,
    );
    assert!(is_winding_refusal(&fc), "{fc:?}");
    let ms = boundary_material_sign(&sphere::<Interval>(), &three_pi, bd);
    assert!(is_winding_refusal(&ms), "{ms:?}");
    let sd = require_iso_rectangle(&sphere::<Interval>(), &three_pi, bd);
    assert!(is_winding_refusal(&sd), "{sd:?}");
    let door = require_one_chart_branch(&sphere::<Interval>(), &three_pi, bd);
    assert!(is_winding_refusal(&door), "{door:?}");

    let inside = pair::<Interval>(0.3, TAU + 0.5 * bd.zero() / RS);
    assert!(
        curved_face(&sphere::<Interval>(), &inside, Interval::from_f64(1.0), bd).is_ok(),
        "a span inside the coincidence band above τ is answered at interval"
    );
    assert_eq!(
        require_iso_rectangle(&sphere::<Interval>(), &inside, bd),
        Ok(())
    );

    let mid = pair::<Interval>(0.3, TAU + 0.5 * (bd.zero() + bd.escalate()) / RS);
    let fc = curved_face(&sphere::<Interval>(), &mid, Interval::from_f64(1.0), bd);
    assert!(is_winding_escalation(&fc), "{fc:?}");
}
