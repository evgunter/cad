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
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;
/// The winding decide every consumer refuses under.
const NAME: &str = "props_meridian_span_winding";
/// The forward decide every consumer refuses under.
const FORWARD: &str = "props_meridian_span_forward";

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
/// The meridian great circle at azimuth 0 with `t0` and `t1` stored
/// VERBATIM — no sorting — so a reversed span is stated as such.
fn great_raw(t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    let carrier = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: RS,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    LoopEdge::hand_built(carrier, t0, t1, true, a, b)
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
fn is_forward_refusal<V: core::fmt::Debug>(r: &Result<V, PropsError>) -> bool {
    matches!(r, Err(PropsError::NotIsoRectangle { what }) if *what == FORWARD)
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

/// **The forward half of the bound, at every consumer.** Certification
/// bounds a span from below too (`interval_span_forward`: `Δt`
/// definitely positive, else `IntervalNotForward`), and a decide on
/// the headroom `τ − Δt` alone cannot see a REVERSED span — stored
/// `[3π, 0]` has headroom `τ + 3π`, definitely `Positive`, and would
/// reach the fold with `dt = −3π`, which answers half the hemisphere
/// pair. The parse decides `Δt` itself, metered at the sphere radius,
/// under `props_meridian_span_forward`, and every consumer refuses
/// under that one name — the multi-wrap reversed span and a short one
/// (`[π/2, 0]`, within a period in magnitude) alike.
#[test]
fn a_reversed_span_refuses_at_every_consumer() {
    let bd = band();
    let multi_wrap = vec![
        great_raw(3.0 * PI, 0.0, 0, 1),
        great_raw(3.0 * PI, 4.0 * PI, 1, 0),
    ];
    let short = vec![
        great_raw(PI / 2.0, 0.0, 0, 1),
        great_raw(PI / 2.0, TAU, 1, 0),
    ];
    for (label, edges) in [("[3π, 0]", &multi_wrap), ("[π/2, 0]", &short)] {
        let fc = curved_face(&sphere::<f64>(), edges, 1.0, bd);
        assert!(is_forward_refusal(&fc), "{label}: flux lane {fc:?}");
        let ms = boundary_material_sign(&sphere::<f64>(), edges, bd);
        assert!(is_forward_refusal(&ms), "{label}: material sign {ms:?}");
        let sd = require_iso_rectangle(&sphere::<f64>(), edges, bd);
        assert!(is_forward_refusal(&sd), "{label}: shape door {sd:?}");
        let door = require_one_chart_branch(&sphere::<f64>(), edges, bd);
        assert!(is_forward_refusal(&door), "{label}: branch door {door:?}");
    }
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

/// **A rim is not decided here.** The bounds are re-decided on
/// MERIDIAN arcs, whose span feeds the pole fold; a rim's span feeds
/// the `Δu` sum, a different premise with a different home, and a
/// full-period rim (the scaffolding shape every revolve mints) must
/// keep passing the parse: the cap it closes is a chart rectangle at
/// the shape door and the flux lane measures it, `2πR²(1 − sin v)`.
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
    assert_eq!(require_iso_rectangle(&sphere::<f64>(), &cap, bd), Ok(()));
    let fc = curved_face(&sphere::<f64>(), &cap, 1.0, bd).unwrap_or_else(|e| panic!("{e:?}"));
    let exact = TAU * RS * RS * (1.0 - 0.3f64.sin());
    let rel = (fc.area - exact).abs() / exact;
    assert!(rel < 1e-12, "cap area {:e} != {exact:e}", fc.area);
}

// ---------------------------------------------------------------------
// The instrument: certification's own arithmetic beside the parse's
// ---------------------------------------------------------------------

/// A consumer's disposition of one span, read by the decide's name.
#[derive(Debug, PartialEq, Clone)]
enum Disp {
    Admit,
    Escalate,
    Refuse,
    Other(String),
}
fn disp<V: core::fmt::Debug>(r: &Result<V, PropsError>) -> Disp {
    match r {
        Ok(_) => Disp::Admit,
        Err(PropsError::Escalated { cause }) if cause.predicate == Some(NAME) => Disp::Escalate,
        Err(PropsError::NotIsoRectangle { what }) if *what == NAME => Disp::Refuse,
        Err(e) => Disp::Other(format!("{e:?}")),
    }
}
/// Certification's own winding decide on the same `(t0, t1)` —
/// `certify.rs`'s check 2 on a circle carrier, its arithmetic
/// verbatim: headroom `(τ − Δt)·r`, `Zero | Positive` admit.
fn certification_disp<T: Decide>(t0: T, t1: T, radius: T, bd: Band) -> Disp {
    let headroom = Margin::levered(T::tau() - (t1 - t0), radius);
    match decide("interval_span_winding", headroom, bd) {
        Ok(Sign::Positive | Sign::Zero) => Disp::Admit,
        Ok(Sign::Negative) => Disp::Refuse,
        Err(_) => Disp::Escalate,
    }
}

/// **The parse's decide IS certification's, rung by rung.** The four
/// consumers against `certify.rs`'s own arithmetic on the same
/// `(t0, t1)`, on a ladder dense around both band edges — the eight
/// `±k·ulp(τ)` rungs on either side of `zero/R` and of `escalate/R`,
/// where a re-spelling of the margin (`τ − (t1 − t0)` against
/// `τ − Δt` with `Δt` formed first) could round to a different sign —
/// at three anchors. Certification decides every edge of the loop, so
/// the expected disposition is the first edge's unless it admits, then
/// the complement's; the branch door answers edge 0's ARC
/// (`NotOneChartBranch`) once the span is admitted, its own question.
fn ladder_against_certification<T: Decide>() {
    let bd = band();
    let z = bd.zero() / RS;
    let e = bd.escalate() / RS;
    let ulp = f64::EPSILON * TAU;
    let mut etas = vec![0.0, 0.5 * z, 0.9 * z, 0.99 * z, 0.999 * z, z];
    for k in 1..=8 {
        let k = f64::from(k);
        etas.extend([z - k * ulp, z + k * ulp, e - k * ulp, e + k * ulp]);
    }
    etas.extend([
        1.001 * z,
        1.01 * z,
        0.5 * (z + e),
        0.99 * e,
        e,
        1.01 * e,
        10.0 * e,
        PI,
    ]);
    let mut mismatches = Vec::new();
    for &eta in &etas {
        for &t0 in &[0.0, 0.3, 1.7] {
            let edges = pair::<T>(t0, TAU + eta);
            let first = certification_disp(edges[0].t0, edges[0].t1, f::<T>(RS), bd);
            let want = if first == Disp::Admit {
                certification_disp(edges[1].t0, edges[1].t1, f::<T>(RS), bd)
            } else {
                first.clone()
            };
            let got = [
                disp(&curved_face(&sphere::<T>(), &edges, f(1.0), bd)),
                disp(&boundary_material_sign(&sphere::<T>(), &edges, bd)),
                disp(&require_iso_rectangle(&sphere::<T>(), &edges, bd)),
                disp(&require_one_chart_branch(&sphere::<T>(), &edges, bd)),
            ];
            for (i, g) in got.iter().enumerate() {
                let same = match g {
                    Disp::Other(s) => {
                        i == 3 && s.contains("NotOneChartBranch") && first == Disp::Admit
                    }
                    g => *g == want,
                };
                if !same {
                    mismatches.push(format!(
                        "t0={t0} eta/z={:.6}: certification {want:?}, consumer {i} {g:?}",
                        eta / z
                    ));
                }
            }
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} spans × 3 anchors × 4 consumers:\n{}",
        etas.len(),
        mismatches.join("\n")
    );
}

#[test]
fn the_parse_decide_matches_certifications_arithmetic_rung_by_rung() {
    ladder_against_certification::<f64>();
}

#[cfg(feature = "interval")]
#[test]
fn the_parse_decide_matches_certifications_arithmetic_at_the_interval_scalar() {
    ladder_against_certification::<geom_core::Interval>();
}

/// The north pole's membership margin for the meridian arc `[t0, t1]`
/// on the azimuth-0 great circle (`P(t) = (R cos t, 0, R sin t)`, pole
/// at `t = π/2`), formed two ways in `f64`: against the half-turn
/// CLAMPED edge `cos(min(dt/2, π))` and against the unclamped edge
/// `cos(dt/2)` the helper uses. Both carry the chord to the nearer
/// span endpoint; `(clamped, unclamped)`.
fn pole_margins_both_ways(t0: f64, t1: f64) -> (f64, f64) {
    let dt = t1 - t0;
    let (sa, ca) = (t0.sin(), t0.cos());
    let (sd2, cd2) = (dt * 0.5).sin_cos();
    let (_, c_clamped) = (dt * 0.5).min(PI).sin_cos();
    let (sdt, cdt) = dt.sin_cos();
    let f_clamped = sa * cd2 + ca * sd2 - c_clamped;
    let f_edge = sa * cd2 + ca * sd2 - cd2;
    let chord_a = ((sa - 1.0).powi(2) + ca.powi(2)).sqrt();
    let chord_b = ((sa - cdt).powi(2) + (ca - sdt).powi(2)).sqrt();
    let chord = chord_a.min(chord_b);
    (chord.copysign(f_clamped), chord.copysign(f_edge))
}
/// The fold's disposition of a pole margin: everything but a definite
/// `Negative` folds.
fn fold_skips(m: f64, bd: Band) -> bool {
    m < 0.0 && m.abs() * RS >= bd.escalate()
}

/// **On an admitted span the unclamped edge folds exactly as the
/// clamped one did.** Spans `τ + {0.5, 0.99}·zero/R` — inside the
/// coincidence band above τ, the only overrun the parse admits — with
/// the north pole placed at the span's start, its end, the direction
/// antipodal to its midpoint, across `±1.5·zero/2R` of the start and
/// far inside: the two edges never disagree on whether the pole
/// folds, the flux lane measures the hemisphere pair to rounding, and
/// the branch door's answer is its own (the arc contains a pole).
#[test]
fn an_admitted_spans_fold_is_the_same_with_and_without_the_clamp() {
    let bd = band();
    let z = bd.zero() / RS;
    let exact = 2.0 * PI * RS * RS;
    for eta in [0.5 * z, 0.99 * z] {
        let places = [
            ("antipode", eta / 2.0),
            ("t0", 0.0),
            ("t1", eta),
            ("0.25z", 0.25 * z),
            ("0.5z", 0.5 * z),
            ("0.75z", 0.75 * z),
            ("1.5z", 1.5 * z),
            ("-0.5z", -0.5 * z),
            ("-1.5z", -1.5 * z),
            ("far", 1.0),
        ];
        for (label, s) in places {
            let t0 = PI / 2.0 - s;
            let t1 = t0 + TAU + eta;
            let (m_clamped, m_edge) = pole_margins_both_ways(t0, t1);
            assert_eq!(
                fold_skips(m_clamped, bd),
                fold_skips(m_edge, bd),
                "eta/z={:.2} pole@{label}: clamped {m_clamped:+.3e}, edge {m_edge:+.3e}",
                eta / z
            );
            let edges = pair::<f64>(t0, TAU + eta);
            let fc = curved_face(&sphere::<f64>(), &edges, 1.0, bd)
                .unwrap_or_else(|e| panic!("eta/z={:.2} pole@{label}: {e:?}", eta / z));
            let rel = (fc.area - exact).abs() / exact;
            assert!(
                rel < 1e-14,
                "eta/z={:.2} pole@{label}: area rel {rel:e}",
                eta / z
            );
            let door = require_one_chart_branch(&sphere::<f64>(), &edges, bd);
            assert!(
                matches!(door, Err(PropsError::NotOneChartBranch { .. })),
                "eta/z={:.2} pole@{label}: {door:?}",
                eta / z
            );
        }
    }
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
