//! Delta-review probes for PR 3733's fix pass (the Hermite image and
//! `MapResidualHermite`): the bound against adversarial arcs and
//! against images the producer did not make.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, Surface};
use geom_brep::{FittedLane, Pcurve, PcurveCache};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point2, Point3, Tol, Vec3};

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn section(tilt: f64, h: f64) -> Curve3<f64> {
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    let e = Vec3::new(-tilt.cos(), 0.0, tilt.sin());
    Curve3::Circle {
        center: Point3::origin() + n * h,
        axis: n,
        radius: (1.0 - h * h).sqrt(),
        u_ref: e,
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn dense(image: &NurbsCurve2<f64>, c: &Curve3<f64>, t0: f64, t1: f64) -> f64 {
    let s = sphere();
    let pc = Pcurve::Fitted(Arc::new(image.clone()));
    (0..=20000)
        .map(|k| {
            let t = t0 + (t1 - t0) * k as f64 / 20000.0;
            let p = pc.eval(t);
            (s.eval(p.x, p.y) - c.eval(t)).norm()
        })
        .fold(0.0, f64::max)
}

fn certify(image: &NurbsCurve2<f64>, c: &Curve3<f64>, t0: f64, t1: f64) -> Result<f64, String> {
    let pc = Pcurve::Fitted(Arc::new(image.clone()));
    PcurveCache::certify_fitted(
        Arc::new(image.clone()),
        t0,
        t1,
        c,
        &sphere(),
        None,
        pc.chart_box(t0, t1),
        band(),
        FittedLane::certified(),
    )
    .map(|c| c.certificate().envelope)
    .map_err(|e| format!("{e:?}"))
}

/// Claim 5: `dense ≤ envelope ≤ band` on adversarial arcs, and the
/// ratio envelope/dense.
#[test]
fn dense_le_envelope_le_band_on_adversarial_arcs() {
    let pi = core::f64::consts::PI;
    let eps = Tol::witness().get().eps;
    let mut rows: Vec<(String, Curve3<f64>, f64, f64)> = vec![
        ("quarter".into(), section(0.6, 0.3), 0.3, 0.3 + pi / 2.0),
        (
            "1.9π off pole".into(),
            section(0.6, 0.3),
            0.1,
            0.1 + 1.9 * pi,
        ),
        ("seam".into(), section(0.6, 0.3), pi - 0.7, pi + 0.7),
        ("cap 1.9π".into(), section(0.3, 0.8), 0.05, 0.05 + 1.9 * pi),
        (
            "full-ish 2π-1e-9".into(),
            section(0.6, 0.3),
            0.0,
            2.0 * pi - 1e-9,
        ),
        (
            "great circle tilt 1.2".into(),
            section(1.2, 0.0),
            0.0,
            1.9 * pi,
        ),
        (
            "tiny circle near pole".into(),
            section(0.01, 0.99999),
            0.0,
            1.9 * pi,
        ),
        ("short 1e-7".into(), section(0.6, 0.3), 0.3, 0.3 + 1e-7),
    ];
    for gap in [1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6] {
        let tilt: f64 = 0.6;
        rows.push((
            format!("pole gap {gap:e} ±0.5"),
            section(tilt, tilt.cos() - gap),
            -0.5,
            0.5,
        ));
        rows.push((
            format!("pole gap {gap:e} 0.02..1"),
            section(tilt, tilt.cos() - gap),
            0.02,
            1.0,
        ));
    }
    // pole-through circle: arcs ending at distance d (in t) from the pole
    for d in [1e-1, 1e-2, 1e-3, 1e-4] {
        rows.push((
            format!("pole-through, ends {d:e} from pole"),
            section(0.6, 0.6f64.cos()),
            d,
            2.0,
        ));
    }
    let mut bad = vec![];
    for (name, c, t0, t1) in &rows {
        let image =
            FittedLane::<f64>::certified().sphere_circle_image(c, *t0, *t1, &sphere(), band());
        match image {
            Err(e) => println!("{name:<36} image refused: {e:?}"),
            Ok(img) => {
                let spans = (img.control().len() - 1) / 5;
                let d = dense(&img, c, *t0, *t1);
                let env = certify(&img, c, *t0, *t1);
                println!("{name:<36} spans {spans:6} dense {d:9.2e} envelope {env:?}");
                if let Ok(e) = env
                    && !(d <= e + 2e-15 && e <= eps)
                {
                    bad.push(name.clone());
                }
            }
        }
    }
    assert!(bad.is_empty(), "dense ≤ envelope ≤ band fails on {bad:?}");
}

/// Claim 3: the stored image against the object the bound compares it
/// to. Each span's end values of `g` are chosen INDEPENDENTLY, as the
/// branch nearest the image's own end value. An image whose one span
/// winds the azimuth by a whole turn (end data `g(a)` → `g(b) ± 2π`,
/// every later control shifted by the same turn) has control distance
/// ZERO from that "Hermite data", yet sweeps a whole latitude away from
/// the circle between its ends. Pick a span holding no CERT sample.
#[test]
fn a_span_winding_a_whole_turn_certifies() {
    let pi = core::f64::consts::PI;
    let tau = 2.0 * pi;
    let (c, t0, t1) = (section(0.6, 0.3), 0.3, 0.3 + pi / 2.0);
    let img = FittedLane::<f64>::certified()
        .sphere_circle_image(&c, t0, t1, &sphere(), band())
        .unwrap();
    let knots = img.knots().knots().to_vec();
    let ctl = img.control().to_vec();
    let spans = (ctl.len() - 1) / 5;
    let samples: Vec<f64> = (0..9).map(|k| t0 + (t1 - t0) * k as f64 / 8.0).collect();
    let at = |j: usize| knots[5 * j + 5];
    println!(
        "honest image: spans {spans}, envelope {:?}",
        certify(&img, &c, t0, t1)
    );
    let mut certified_any = false;
    for sign in [1.0, -1.0] {
        for j in 0..spans {
            let (a, b) = (at(j), at(j + 1));
            if samples.iter().any(|&s| s >= a && s <= b) {
                continue;
            }
            let mut p = ctl.clone();
            for (i, q) in p.iter_mut().enumerate() {
                if i >= 5 * j + 3 {
                    *q = Point2::new(q.x + sign * tau, q.y);
                }
            }
            let (lo, hi) = p
                .iter()
                .fold((f64::MAX, f64::MIN), |(l, h), q| (l.min(q.x), h.max(q.x)));
            if hi - lo >= tau - 1e-7 {
                continue;
            }
            let bad = NurbsCurve2::new(
                KnotVector::clamped(knots.clone(), 5).unwrap(),
                p,
                vec![1.0; ctl.len()],
            )
            .unwrap();
            let env = certify(&bad, &c, t0, t1);
            let d = dense(&bad, &c, t0, t1);
            println!(
                "sign {sign} span {j} [{a:.4},{b:.4}] u-range {:.4}: dense {d:.3e} certify {env:?}",
                hi - lo
            );
            if env.is_ok() {
                certified_any = true;
                assert!(d > 0.1, "the corruption is visible in 3-D");
            }
            break;
        }
    }
    assert!(
        !certified_any,
        "an image a whole latitude off its circle certified"
    );
}

fn sphere_t<T: geom_core::Real>() -> Surface<T> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: T::from_f64(1.0),
        axis: Vec3::new(T::zero(), T::zero(), T::one()),
        u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
    }
}

fn section_t<T: geom_core::Real>(tilt: f64, h: f64) -> Curve3<T> {
    let f = T::from_f64;
    Curve3::Circle {
        center: Point3::origin() + Vec3::new(f(tilt.sin() * h), T::zero(), f(tilt.cos() * h)),
        axis: Vec3::new(f(tilt.sin()), T::zero(), f(tilt.cos())),
        radius: f((1.0 - h * h).sqrt()),
        u_ref: Vec3::new(f(-tilt.cos()), T::zero(), f(tilt.sin())),
    }
}

/// Sample-free spans of an image, by index.
fn sample_free_spans(img: &NurbsCurve2<f64>, t0: f64, t1: f64) -> Vec<usize> {
    let knots = img.knots().knots();
    let spans = (img.control().len() - 1) / 5;
    let samples: Vec<f64> = (0..9).map(|k| t0 + (t1 - t0) * k as f64 / 8.0).collect();
    (0..spans)
        .filter(|&j| {
            let (a, b) = (knots[5 * j + 5], knots[5 * j + 10]);
            !samples.iter().any(|&s| s >= a && s <= b)
        })
        .collect()
}

fn with_control(
    img: &NurbsCurve2<f64>,
    f: impl Fn(usize, Point2<f64>) -> Point2<f64>,
) -> NurbsCurve2<f64> {
    let p = img
        .control()
        .iter()
        .enumerate()
        .map(|(i, q)| f(i, *q))
        .collect();
    NurbsCurve2::new(img.knots().clone(), p, img.weights().to_vec()).unwrap()
}

/// Claim 5 / the m6_3 mutation row's premise: the corruption planted in
/// a span with NO certification sample must be refused by check 4
/// itself (Envelope), not by check 3.
#[test]
fn a_corruption_in_a_sample_free_span_refuses_at_the_envelope() {
    let pi = core::f64::consts::PI;
    let (c, t0, t1) = (section(0.6, 0.3), 0.3, 0.3 + pi / 2.0);
    let img = FittedLane::<f64>::certified()
        .sphere_circle_image(&c, t0, t1, &sphere(), band())
        .unwrap();
    let free = sample_free_spans(&img, t0, t1);
    let j = free[free.len() / 2];
    for (mag, ch) in [(1e-3, 'u'), (1e-3, 'v'), (1e-8, 'u'), (3e-10, 'v')] {
        let bad = with_control(&img, |i, q| {
            if i == 5 * j + 2 {
                if ch == 'u' {
                    Point2::new(q.x + mag, q.y)
                } else {
                    Point2::new(q.x, q.y + mag)
                }
            } else {
                q
            }
        });
        let d = dense(&bad, &c, t0, t1);
        let env = certify(&bad, &c, t0, t1);
        println!("span {j} control 2 {ch} += {mag:e}: dense {d:.3e} certify {env:?}");
        if let Ok(e) = env {
            assert!(d <= e, "under-bound");
        } else {
            assert!(env.unwrap_err().contains("Envelope"));
        }
    }
    // The m6_3 row's own control index: len/2 — which span holds it?
    let k = img.control().len() / 2;
    let spans = (img.control().len() - 1) / 5;
    println!(
        "m6_3-style control {k} of {spans} spans: sample-free spans contain it? {}",
        free.iter().any(|&j| 5 * j < k && k < 5 * j + 5)
    );
}

/// The whole-turn winding at the INTERVAL scalar.
#[test]
fn a_span_winding_a_whole_turn_certifies_at_the_interval_scalar() {
    use geom_core::interval::Interval;
    let pi = core::f64::consts::PI;
    let (t0, t1) = (0.3, 0.3 + pi / 2.0);
    let c = section(0.6, 0.3);
    let img = FittedLane::<f64>::certified()
        .sphere_circle_image(&c, t0, t1, &sphere(), band())
        .unwrap();
    let j = sample_free_spans(&img, t0, t1)[0];
    let bad = with_control(&img, |i, q| {
        if i >= 5 * j + 3 {
            Point2::new(q.x - 2.0 * pi, q.y)
        } else {
            q
        }
    });
    let d = dense(&bad, &c, t0, t1);
    let lift = |c: &NurbsCurve2<f64>| {
        let control = c
            .control()
            .iter()
            .map(|p| p.map(<Interval as geom_core::Real>::from_f64))
            .collect();
        NurbsCurve2::new(c.knots().clone(), control, c.weights().to_vec()).unwrap()
    };
    let bi = lift(&bad);
    let window = Pcurve::Fitted(Arc::new(bi.clone())).chart_box(
        <Interval as geom_core::Real>::from_f64(t0),
        <Interval as geom_core::Real>::from_f64(t1),
    );
    let r = PcurveCache::<Interval>::certify_fitted(
        Arc::new(bi),
        <Interval as geom_core::Real>::from_f64(t0),
        <Interval as geom_core::Real>::from_f64(t1),
        &section_t::<Interval>(0.6, 0.3),
        &sphere_t::<Interval>(),
        None,
        window,
        band(),
        FittedLane::certified(),
    )
    .map(|c| c.certificate().envelope);
    println!("interval: dense {d:.3e} certify {r:?}");
    assert!(
        r.is_err(),
        "an image 1 m off its circle certified at the interval scalar"
    );
}

/// The m6_3 mutation row's configuration (great circle, tilt 0.6, the
/// quarter turn from 0.3; control `len/2` moved 1e-3 in u): which check
/// refuses it?
#[test]
fn which_check_refuses_the_m6_3_mutation() {
    let pi = core::f64::consts::PI;
    let (c, t0, t1) = (section(0.6, 0.0), 0.3, 0.3 + pi / 2.0);
    let img = FittedLane::<f64>::certified()
        .sphere_circle_image(&c, t0, t1, &sphere(), band())
        .unwrap();
    let k = img.control().len() / 2;
    let bad = with_control(&img, |i, q| {
        if i == k {
            Point2::new(q.x + 1e-3, q.y)
        } else {
            q
        }
    });
    println!(
        "m6_3 mutation: in a sample-free span? {}; certify {:?}",
        sample_free_spans(&img, t0, t1)
            .iter()
            .any(|&j| 5 * j < k && k < 5 * j + 5),
        certify(&bad, &c, t0, t1)
    );
}

/// The derivation is `|S(P)−C| ≤ |S(P)−S(g)| + |S(g)−C|`, but
/// `circle_image_envelope` folds `sup = off_sphere.max(span metres)`.
/// A circle off its sphere by `off` (radial) with an image shifted in
/// azimuth so the tangential gap is `≈ 2·off` (the off-sphere reading is
/// `Σ|kᵢ|/R ≈ 2·off`): the true sup is `√(off² + (2off)²) ≈ 2.24·off`,
/// the envelope `≈ 2·off`.
#[test]
fn the_envelope_takes_the_max_of_two_terms_it_derives_as_a_sum() {
    let pi = core::f64::consts::PI;
    let (tilt, h, off) = (0.6_f64, 0.3_f64, 2e-10_f64);
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    let c = Curve3::Circle {
        center: Point3::origin() + n * h,
        axis: n,
        radius: ((1.0 + off).powi(2) - h * h).sqrt(),
        u_ref: Vec3::new(-tilt.cos(), 0.0, tilt.sin()),
    };
    let (t0, t1) = (0.3, 0.3 + pi / 2.0);
    let img = FittedLane::<f64>::certified()
        .sphere_circle_image(&c, t0, t1, &sphere(), band())
        .unwrap();
    let honest = certify(&img, &c, t0, t1);
    let cos_max = (0..=20000)
        .map(|k| {
            let p = c.eval(t0 + (t1 - t0) * k as f64 / 20000.0) - Point3::origin();
            (p.x * p.x + p.y * p.y).sqrt() / p.norm()
        })
        .fold(0.0, f64::max);
    let delta = 2.0 * off / cos_max;
    let shifted = with_control(&img, |_, q| Point2::new(q.x + delta, q.y));
    let d = dense(&shifted, &c, t0, t1);
    let env = certify(&shifted, &c, t0, t1);
    println!("honest envelope {honest:?}; shifted by {delta:e}: dense {d:.4e} envelope {env:?}");
    if let Ok(e) = env {
        assert!(
            d <= e,
            "dense {d:e} > envelope {e:e}: the envelope under-bounds"
        );
    }
}

/// Check 4 bounds the image's OWN spans; nothing compares the image's
/// knot domain with `[t0, t1]`. An image made on `[t0 + s, t1]`,
/// certified on `[t0, t1]` (P extrapolated on the uncovered piece).
#[test]
fn an_image_shorter_than_its_edge() {
    let pi = core::f64::consts::PI;
    let (c, t0, t1) = (section(0.6, 0.3), 0.3, 0.3 + pi / 2.0);
    for s in [1e-6, 1e-2, 2e-2, 3e-2, 5e-2] {
        let img = FittedLane::<f64>::certified()
            .sphere_circle_image(&c, t0 + s, t1, &sphere(), band())
            .unwrap();
        let d = dense(&img, &c, t0, t1);
        println!(
            "image on [t0+{s:e}, t1], edge [t0, t1]: dense {d:.3e} certify {:?}",
            certify(&img, &c, t0, t1)
        );
    }
}

/// Round 2: the domain check is exact at f64 — an image one ulp (and
/// 1e-12) short of the edge at either end refuses.
#[test]
fn round2_a_domain_short_by_an_ulp_refuses() {
    let pi = core::f64::consts::PI;
    let (c, t0, t1) = (section(0.6, 0.3), 0.3_f64, 0.3 + pi / 2.0);
    for (a, b) in [
        (f64::from_bits(t0.to_bits() + 1), t1),
        (t0, f64::from_bits(t1.to_bits() - 1)),
        (t0 + 1e-12, t1),
        (t0 - 1e-12, t1),
    ] {
        let img = FittedLane::<f64>::certified()
            .sphere_circle_image(&c, a, b, &sphere(), band())
            .unwrap();
        let r = certify(&img, &c, t0, t1);
        println!("image [{a:.17}, {b:.17}] edge [{t0}, {t1}]: {r:?}");
        assert!(r.is_err(), "a domain other than the edge's certified");
    }
}
