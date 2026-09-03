//! **R2 review probes for MESH-12** (issue 1601): the parse's span
//! decide `props_meridian_span_winding` against certification's
//! `interval_span_winding`, rung for rung; the lever each of them
//! actually uses; the fold on an admitted span whose pole sits at the
//! antipode of the span's midpoint (the direction the retired clamp
//! could not answer); and what a rim span past the period does.
//!
//! Every offset is derived from the run's own `Band`, so the rows read
//! the same at every ε row and on the interval lane.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{
    LoopEdge, PropsError, curved_face, require_iso_rectangle, require_one_chart_branch,
};
use geom_brep::{CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;
const NAME: &str = "props_meridian_span_winding";

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: RS,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
/// The meridian great circle at azimuth `u`, stated with radius `r`
/// (the sphere's, or a hair off it): `t` is the latitude on the `u`
/// side, `t = π/2` the north pole.
fn meridian(u: f64, r: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(u.sin(), -u.cos(), 0.0),
        radius: r,
        u_ref: Vec3::new(u.cos(), u.sin(), 0.0),
    }
}
fn great_r(u: f64, r: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    let (lo, hi, forward) = if t0 < t1 {
        (t0, t1, true)
    } else {
        (t1, t0, false)
    };
    LoopEdge::hand_built(meridian(u, r), lo, hi, forward, a, b)
}
fn great(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    great_r(u, RS, t0, t1, a, b)
}
/// A rimless pair on one great circle of radius `r`: `[t0, t0 + dt]`
/// and the complement back to `t0 + 4π`.
fn pair_r(r: f64, t0: f64, dt: f64) -> Vec<LoopEdge<f64>> {
    vec![
        great_r(0.0, r, t0, t0 + dt, 0, 1),
        great_r(0.0, r, t0 + dt, t0 + 4.0 * PI, 1, 0),
    ]
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rung {
    Admit,
    Escalate,
    Refuse,
}

/// Certification's own verdict on the span `[t0, t0 + dt]` of a
/// circle of radius `r_c`, through the public `EdgeCurve::certify`.
fn cert_rung(r_c: f64, t0: f64, dt: f64) -> Result<Rung, String> {
    let carrier = meridian(0.0, r_c);
    let spec = EdgeCurveSpec::arc_of_circle(carrier.clone(), t0, t0 + dt).unwrap();
    match EdgeCurve::certify(
        spec,
        carrier.eval(t0),
        carrier.eval(t0 + dt),
        |_| None,
        band(),
    ) {
        Ok(_) => Ok(Rung::Admit),
        Err(CertifyError::Escalated {
            check: CertCheck::ParamSpan,
            ..
        }) => Ok(Rung::Escalate),
        Err(CertifyError::WindingExceeded) => Ok(Rung::Refuse),
        Err(e) => Err(format!("{e:?}")),
    }
}
fn props_rung<V: core::fmt::Debug>(r: &Result<V, PropsError>) -> Result<Rung, String> {
    match r {
        Ok(_) => Ok(Rung::Admit),
        Err(PropsError::NotOneChartBranch { .. }) => Ok(Rung::Admit),
        Err(PropsError::Escalated { cause }) if cause.predicate == Some(NAME) => Ok(Rung::Escalate),
        Err(PropsError::NotIsoRectangle { what }) if *what == NAME => Ok(Rung::Refuse),
        Err(e) => Err(format!("{e:?}")),
    }
}
/// The parse's verdict (through the flux lane) and the branch door's,
/// on the same span of a carrier of radius `r_c` on the R sphere.
fn parse_rungs(r_c: f64, t0: f64, dt: f64) -> (Result<Rung, String>, Result<Rung, String>) {
    let edges = pair_r(r_c, t0, dt);
    let fc = curved_face(&sphere(), &edges, 1.0, band());
    let door = require_one_chart_branch(&sphere(), &edges, band());
    (props_rung(&fc), props_rung(&door))
}

/// **Rung for rung, on the sphere's own radius.** The ladder
/// `τ + η·zero/R` for η on both sides of both thresholds, plus the
/// exact period and a span inside it: certification, the parse and the
/// branch door must land on the same rung at every one.
#[test]
fn r2_the_two_decides_agree_rung_for_rung_on_the_sphere_radius() {
    let bd = band();
    let k = bd.escalate() / bd.zero();
    let t0 = 0.3;
    let mut disagreements = Vec::new();
    for eta in [
        -0.5,
        0.0,
        0.5,
        0.99,
        1.01,
        0.5 * (1.0 + k),
        0.99 * k,
        1.01 * k,
        10.0 * k,
    ] {
        let dt = TAU + eta * bd.zero() / RS;
        let cert = cert_rung(RS, t0, dt);
        let (parse, door) = parse_rungs(RS, t0, dt);
        println!("R2-LADDER eta={eta:+.3} cert={cert:?} parse={parse:?} door={door:?}");
        if !(cert == parse && parse == door) {
            disagreements.push((eta, cert, parse, door));
        }
    }
    assert!(
        disagreements.is_empty(),
        "the three decides disagree on: {disagreements:?}"
    );
}

/// **The lever is not the same quantity at the two sites.**
/// Certification levers the headroom at the CARRIER's radius
/// (`interval_span_winding`, `Margin::levered(τ − span, r_c)`); the
/// parse levers it at the SPHERE's (`Margin::levered(τ − Δt, radius)`).
/// `props_meridian_great` admits a carrier radius within `zero` of the
/// sphere's, so for `r_c = R − 0.9·zero` there is a window of spans,
/// `x ∈ (zero/R, zero/r_c]` past τ, where certification's margin is
/// inside the coincidence band and the parse's is just outside it:
/// the edge certifies and the parse ESCALATES under
/// `props_meridian_span_winding`. The window is ~`zero²/R²` radians
/// wide — about ten f64 ulps of τ at R = 10 mm — and the mirrored
/// carrier (`R + 0.9·zero`) opens the harmless direction (the parse
/// admits what certification escalates). Recorded, not weighed: the
/// PR's "same margin, band and lever" is true up to this window.
#[test]
fn r2_the_lever_is_the_carrier_radius_at_certification_and_the_sphere_radius_at_the_parse() {
    let bd = band();
    let t0 = 0.0;
    for (label, r_c) in [
        ("R - 0.9 zero", RS - 0.9 * bd.zero()),
        ("R + 0.9 zero", RS + 0.9 * bd.zero()),
    ] {
        let lo = bd.zero() / RS.max(r_c);
        let hi = bd.zero() / RS.min(r_c);
        let mut seen = Vec::new();
        // The window is ~zero²/R² radians wide; at ε = 1e-12 that is
        // below f64's resolution at τ (one ulp ≈ 8.9e-16 rad), so the
        // three rungs below collapse onto one span and both decides
        // land in the ambiguity band together. Asserted only where
        // the window is resolvable.
        let resolvable = (hi - lo) > 8.0 * f64::EPSILON * TAU;
        for frac in [0.25, 0.5, 0.75] {
            let x = lo + frac * (hi - lo);
            let dt = TAU + x;
            let cert = cert_rung(r_c, t0, dt);
            let (parse, door) = parse_rungs(r_c, t0, dt);
            println!(
                "R2-LEVER {label}: x={x:e} rad, cert margin {:e}, parse margin {:e}: cert={cert:?} parse={parse:?} door={door:?}",
                (TAU - dt) * r_c,
                (TAU - dt) * RS
            );
            seen.push((cert, parse, door));
        }
        if !resolvable {
            println!(
                "R2-LEVER {label}: window {:e} rad is below f64 resolution at τ; not asserted",
                hi - lo
            );
        } else if r_c < RS {
            assert!(
                seen.iter().all(|(c, p, d)| {
                    *c == Ok(Rung::Admit) && *p == Ok(Rung::Escalate) && *d == Ok(Rung::Escalate)
                }),
                "{label}: expected certify-admits / parse-escalates across the window: {seen:?}"
            );
        } else {
            assert!(
                seen.iter().all(|(c, p, d)| {
                    *c == Ok(Rung::Escalate) && *p == Ok(Rung::Admit) && *d == Ok(Rung::Admit)
                }),
                "{label}: expected certify-escalates / parse-admits across the window: {seen:?}"
            );
        }
    }
}

/// **An admitted span past τ, with the north pole at the antipode of
/// the span's midpoint** — the one direction the retired clamp's sign
/// vanished at, now reached with the unclamped `cos(dt/2)`. For
/// `η ∈ {0.5, 0.99}·zero/R` and the pole swept through `±η` of that
/// direction, forward and reversed: the hemisphere pair measures
/// `2πR²` to 1e-12 and the branch door refuses on the south pole,
/// which sits at the midpoint itself. Areas are printed to full
/// precision so the same row on the merge base can be diffed against
/// this head.
#[test]
fn r2_an_admitted_span_folds_the_pole_at_the_antipode_of_its_midpoint() {
    let bd = band();
    let exact = 2.0 * PI * RS * RS;
    let mut worst = 0.0f64;
    for eta in [0.5 * bd.zero() / RS, 0.99 * bd.zero() / RS] {
        let dt = TAU + eta;
        for shift in [-1.0, -0.5, -0.25, 0.0, 0.25, 0.5, 1.0] {
            // The antipode of the midpoint is `t0 + η/2`; put the pole
            // `shift·η` away from it.
            let t0 = PI / 2.0 - eta / 2.0 - shift * eta;
            for reversed in [false, true] {
                let edges = if reversed {
                    vec![
                        great(0.0, t0 + dt, t0, 1, 0),
                        great(0.0, t0 + 4.0 * PI, t0 + dt, 0, 1),
                    ]
                } else {
                    pair_r(RS, t0, dt)
                };
                let fc = curved_face(&sphere(), &edges, 1.0, bd)
                    .unwrap_or_else(|e| panic!("η={eta:e} shift={shift}: {e:?}"));
                let rel = (fc.area - exact) / exact;
                worst = worst.max(rel.abs());
                let door = require_one_chart_branch(&sphere(), &edges, bd);
                println!(
                    "R2-ANTIPODE eta={eta:e} shift={shift:+} rev={reversed}: area={:.17e} rel={rel:+.3e} door={door:?}",
                    fc.area
                );
                assert!(rel.abs() < 1e-12, "η={eta:e} shift={shift}: rel {rel:e}");
                assert!(
                    matches!(door, Err(PropsError::NotOneChartBranch { .. })),
                    "η={eta:e} shift={shift}: the south pole sits at the midpoint: {door:?}"
                );
            }
        }
    }
    println!("R2-ANTIPODE worst |rel| = {worst:e}");
}

/// **A rim span past the period is ANSWERED, not decided** (issue
/// 1618's premise, executed): a half-cap whose rim is stated as one
/// arc of span `3π` (it lands at `u = π`, so the loop closes) passes
/// the shape door and the flux lane measures THREE half-caps. The
/// meridian decide does not touch it, by design; nothing else does
/// either.
#[test]
fn r2_a_rim_span_past_the_period_is_answered_not_decided() {
    let bd = band();
    let v: f64 = 0.3;
    let rim = LoopEdge::hand_built(
        Curve3::Circle {
            center: Point3::new(0.0, 0.0, RS * v.sin()),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: RS * v.cos(),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        0.0,
        3.0 * PI,
        true,
        0,
        1,
    );
    let cap = vec![
        rim,
        great(PI, v, PI / 2.0, 1, 2),
        great(0.0, PI / 2.0, v, 2, 0),
    ];
    let exact_half_cap = RS * RS * PI * (1.0 - v.sin());
    let shape = require_iso_rectangle(&sphere(), &cap, bd);
    let fc = curved_face(&sphere(), &cap, 1.0, bd);
    println!(
        "R2-RIM-3PI shape={shape:?} flux={:?} (one half-cap = {exact_half_cap:e})",
        fc.as_ref().map(|c| c.area)
    );
    assert_eq!(shape, Ok(()), "the shape door admits the 3π rim");
    let area = fc.expect("the flux lane answers the 3π rim").area;
    assert!(
        ((area - 3.0 * exact_half_cap) / exact_half_cap).abs() < 1e-9,
        "the answer is three half-caps: {area:e}"
    );
}

/// **Forwardness is not re-decided.** Certification decides
/// `interval_span_forward` on the same span before the winding bound;
/// the parse re-decides only the bound, so a hand-built meridian of
/// span zero is not the winding decide's to refuse and lands wherever
/// the fold takes it. Printed; asserted only that the winding name is
/// not what answers.
#[test]
fn r2_a_zero_span_meridian_is_not_re_decided_for_forwardness() {
    let bd = band();
    let edges = vec![great(0.0, 0.3, 0.3, 0, 1), great(0.0, 0.3, 0.3 + TAU, 1, 0)];
    let fc = curved_face(&sphere(), &edges, 1.0, bd);
    let door = require_one_chart_branch(&sphere(), &edges, bd);
    let shape = require_iso_rectangle(&sphere(), &edges, bd);
    println!("R2-ZERO-SPAN flux={fc:?} door={door:?} shape={shape:?}");
    for r in [props_rung(&fc), props_rung(&door), props_rung(&shape)] {
        assert!(
            !matches!(r, Ok(Rung::Refuse) | Ok(Rung::Escalate)),
            "a zero span is not the winding bound's to answer: {r:?}"
        );
    }
}
