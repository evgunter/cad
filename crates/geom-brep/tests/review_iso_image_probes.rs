//! Review probes for the iso-image representation (PR 7393): the
//! certificate's verdict on rows and columns minted through
//! [`Pcurve::iso_row`] / [`Pcurve::iso_column`], whose signatures the
//! change kept, so this file compiles and runs unchanged on the tree
//! before the change. Each probe prints one line per body, and the
//! review compared the two trees' lines.
//!
//! The bodies attack the reads the change removed from tier 3: a map
//! that runs past its edge's end, short of it, off its chart's domain, a
//! period away, with its sense reversed, by more than the band and
//! inside it, on a line carrier and on a spline one, at `f64` and at
//! `Interval`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{Pcurve, PcurveCache};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Decide, Interval, Point3, Real, Vec3};
use std::sync::Arc;

/// The bilinear chart over `u ∈ [a, b]`, `v ∈ [0, 1]`, `(u, v) ↦
/// ((u − a)/(b − a), v, 0)`.
fn chart<T: Real>(a: f64, b: f64) -> Surface<T> {
    let ku = KnotVector::clamped(vec![a, a, b, b], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| Point3::new(x, y, 0.0))
        .to_vec();
    let net = NurbsSurface::new(ku, kv, control, vec![1.0; 4]).unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

/// A carrier along the row `v = 0` (`along_u`) or the column `u = 0` of
/// the unit chart, displaced by `off` metres across it in the chart's
/// plane and `lift` out of it, run backwards where `back`; a line, or
/// the degree-1 spline on the row's own knots where `spline`.
fn carrier<T: Real>(along_u: bool, off: f64, lift: f64, back: bool, spline: bool) -> Curve3<T> {
    let at = |s: f64| {
        let s = if back { 1.0 - s } else { s };
        if along_u {
            Point3::new(s, off, lift)
        } else {
            Point3::new(off, s, lift)
        }
    };
    if spline {
        let c = NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![at(0.0), at(1.0)],
            vec![1.0, 1.0],
        )
        .unwrap();
        Curve3::Nurbs(Arc::new(c.map_scalar(T::from_f64)))
    } else {
        let (a, b) = (at(0.0), at(1.0));
        let f = T::from_f64;
        Curve3::line(
            Point3::new(f(a.x), f(a.y), f(a.z)),
            Vec3::new(f(b.x - a.x), f(b.y - a.y), f(b.z - a.z)),
        )
    }
}

/// One body's verdict, as text: the envelope and the schedule's residual
/// where it certifies, the refusal's variant and check where it does not.
fn verdict<T: Decide + Bounds>(
    image: Pcurve<T>,
    (t0, t1): (f64, f64),
    carrier: &Curve3<T>,
    surface: &Surface<T>,
) -> String {
    let f = T::from_f64;
    match PcurveCache::certify(image, f(t0), f(t1), carrier, surface, band()) {
        Ok(cache) => {
            let c = cache.certificate();
            format!(
                "OK env={:.3e} res={:.3e}",
                c.envelope.hi(),
                c.max_residual.hi()
            )
        }
        Err(e) => {
            let text = format!("{e:?}");
            let head: String = text.chars().take(70).collect();
            format!("ERR {}", head.replace('\n', " "))
        }
    }
}

/// A named body on the unit chart: the held value, the map's two placed
/// values over the span, the span, and the carrier's variant.
struct Body {
    name: &'static str,
    along_u: bool,
    held: f64,
    placed: (f64, f64),
    span: (f64, f64),
    off: f64,
    lift: f64,
    back: bool,
}

fn run<T: Decide + Bounds>(scalar: &str, spline: bool, b: &Body) -> String {
    let f = T::from_f64;
    let image = if b.along_u {
        Pcurve::iso_row(
            f(b.held),
            (f(b.placed.0), f(b.placed.1)),
            (f(b.span.0), f(b.span.1)),
        )
    } else {
        Pcurve::iso_column(
            f(b.held),
            (f(b.placed.0), f(b.placed.1)),
            (f(b.span.0), f(b.span.1)),
        )
    };
    let c = carrier::<T>(b.along_u, b.off, b.lift, b.back, spline);
    let got = verdict(image, b.span, &c, &chart::<T>(0.0, 1.0));
    format!(
        "PROBE {scalar} {} {} {}: {got}",
        if spline { "spline" } else { "line" },
        if b.along_u { "row" } else { "col" },
        b.name
    )
}

/// The named attacks, on a row and on a column, on both carriers, at
/// both scalars.
#[test]
fn named_bodies_the_removed_reads_stood_in_front_of() {
    let e = eps();
    let mut lines = Vec::new();
    for along_u in [true, false] {
        let b = |name, held, placed, span, off, lift, back| Body {
            name,
            along_u,
            held,
            placed,
            span,
            off,
            lift,
            back,
        };
        let bodies = [
            b("control", 0.0, (0.0, 1.0), (0.0, 1.0), 0.0, 0.0, false),
            b(
                "past_by_tenth",
                0.0,
                (0.0, 1.0),
                (0.0, 0.9),
                0.0,
                0.0,
                false,
            ),
            b(
                "past_by_3eps",
                0.0,
                (0.0, 1.0),
                (0.0, 1.0 - 3.0 * e),
                0.0,
                0.0,
                false,
            ),
            b(
                "past_by_30eps",
                0.0,
                (0.0, 1.0),
                (0.0, 1.0 - 30.0 * e),
                0.0,
                0.0,
                false,
            ),
            b(
                "past_in_band",
                0.0,
                (0.0, 1.0),
                (0.0, 1.0 - 0.5 * e),
                0.0,
                0.0,
                false,
            ),
            b(
                "short_by_tenth",
                0.0,
                (0.0, 0.9),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "short_by_30eps",
                0.0,
                (0.0, 1.0 - 30.0 * e),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "short_in_band",
                0.0,
                (0.0, 1.0 - 0.5 * e),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "start_off_tenth",
                0.0,
                (0.1, 1.0),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "off_domain_tenth",
                0.0,
                (0.0, 1.1),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "off_domain_30eps",
                0.0,
                (0.0, 1.0 + 30.0 * e),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "off_domain_in_band",
                0.0,
                (0.0, 1.0 + 0.5 * e),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "below_domain_30eps",
                0.0,
                (-30.0 * e, 1.0),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "reversed_image",
                0.0,
                (1.0, 0.0),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "reversed_carrier",
                0.0,
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                0.0,
                true,
            ),
            b("both_reversed", 0.0, (1.0, 0.0), (0.0, 1.0), 0.0, 0.0, true),
            b(
                "held_other_side",
                1.0,
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "held_interior",
                0.5,
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "held_interior_on",
                0.5,
                (0.0, 1.0),
                (0.0, 1.0),
                0.5,
                0.0,
                false,
            ),
            b(
                "held_period_away",
                1.0,
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b("held_two_out", 2.0, (0.0, 1.0), (0.0, 1.0), 0.0, 0.0, false),
            b(
                "held_out_in_band",
                -0.5 * e,
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "held_out_30eps",
                -30.0 * e,
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "held_in_30eps",
                30.0 * e,
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b(
                "carrier_off_in_band",
                0.0,
                (0.0, 1.0),
                (0.0, 1.0),
                0.5 * e,
                0.0,
                false,
            ),
            b(
                "carrier_off_30eps",
                0.0,
                (0.0, 1.0),
                (0.0, 1.0),
                30.0 * e,
                0.0,
                false,
            ),
            b(
                "carrier_lift_30eps",
                0.0,
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                30.0 * e,
                false,
            ),
            b("point_image", 0.0, (0.5, 0.5), (0.0, 1.0), 0.0, 0.0, false),
            b(
                "point_in_band",
                0.0,
                (0.5, 0.5 + 0.5 * e),
                (0.0, 1.0),
                0.0,
                0.0,
                false,
            ),
            b("sub_span", 0.0, (0.25, 0.75), (0.25, 0.75), 0.0, 0.0, false),
            b(
                "sub_span_past",
                0.0,
                (0.25, 0.80),
                (0.25, 0.75),
                0.0,
                0.0,
                false,
            ),
        ];
        for body in &bodies {
            for spline in [false, true] {
                lines.push(run::<f64>("f64", spline, body));
                lines.push(run::<Interval>("Interval", spline, body));
            }
        }
    }
    for l in &lines {
        println!("{l}");
    }
}

/// A deterministic draw in `[0, 1)`.
fn draw(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    #[allow(clippy::cast_precision_loss)]
    let x = (*state >> 11) as f64 / (1u64 << 53) as f64;
    x
}

/// Random bodies around the band's two thresholds: every perturbation is
/// one of `0`, in band, between the band's marks, past it.
#[test]
fn random_bodies_around_the_band() {
    let e = eps();
    let mut state = 0x5eed_u64;
    let nudges = [
        0.0,
        0.0,
        0.0,
        0.3 * e,
        -0.3 * e,
        3.0 * e,
        -3.0 * e,
        30.0 * e,
        -30.0 * e,
        0.1,
    ];
    let pick = |state: &mut u64| {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let i = (draw(state) * nudges.len() as f64) as usize;
        nudges[i.min(nudges.len() - 1)]
    };
    for i in 0..600 {
        let along_u = draw(&mut state) < 0.5;
        let back_image = draw(&mut state) < 0.15;
        let (p0, p1) = (pick(&mut state), 1.0 + pick(&mut state));
        let body = Body {
            name: "random",
            along_u,
            held: pick(&mut state),
            placed: if back_image { (p1, p0) } else { (p0, p1) },
            span: (0.0, 1.0 + pick(&mut state).min(0.0)),
            off: pick(&mut state),
            lift: pick(&mut state),
            back: draw(&mut state) < 0.15,
        };
        for spline in [false, true] {
            let a = run::<f64>("f64", spline, &body);
            let b = run::<Interval>("Interval", spline, &body);
            println!("{i:04} {a}");
            println!("{i:04} {b}");
        }
    }
}

/// On a chart whose domain is not `[0, 1]`: a column held at each end,
/// one period past the far end, and an ulp off each end.
#[test]
fn columns_on_a_domain_not_starting_at_zero() {
    fn at<T: Decide + Bounds>(scalar: &str) {
        let f = T::from_f64;
        let (lo, hi) = (0.3_f64, 1.7_f64);
        let up = |x: f64| f64::from_bits(x.to_bits() + 1);
        let down = |x: f64| f64::from_bits(x.to_bits() - 1);
        for (name, u, x) in [
            ("lo", lo, 0.0),
            ("hi", hi, 1.0),
            ("lo_computed_hi", lo + (hi - lo), 1.0),
            ("hi_plus_period", hi + (hi - lo), 1.0),
            ("lo_minus_ulp", down(lo), 0.0),
            ("lo_plus_ulp", up(lo), 0.0),
            ("hi_plus_ulp", up(hi), 1.0),
            ("hi_minus_ulp", down(hi), 1.0),
        ] {
            let image = Pcurve::iso_column(f(u), (T::zero(), T::one()), (T::zero(), T::one()));
            let c = Curve3::line(
                Point3::new(f(x), T::zero(), T::zero()),
                Vec3::new(T::zero(), T::one(), T::zero()),
            );
            let got = verdict(image, (0.0, 1.0), &c, &chart::<T>(lo, hi));
            println!("PROBE {scalar} domain {name}: {got}");
        }
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
}
