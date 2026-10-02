//! **The general circle's certificate against images the lane did not
//! make.** `MapResidualHermite` is re-derived at rest from whatever
//! image a row stores, so it must hold for any image in the Hermite
//! form, not only the producer's: one whose span winds a whole turn of
//! azimuth, one whose domain is shorter than its edge, and a circle off
//! its sphere whose image is also off its circle (the bound's two terms
//! add, they do not max). Adopted from the PCERT delta review of PR
//! 3733.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, Surface};
use geom_brep::{FittedLane, Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck};
use geom_core::{Band, Point2, Point3, Real, Tol, Vec3};

fn sphere<T: Real>() -> Surface<T> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: T::from_f64(1.0),
        axis: Vec3::new(T::zero(), T::zero(), T::one()),
        u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
    }
}

/// The section of the unit sphere by the plane `n·x = h`, `n` tilted
/// by `tilt` from +z; `grow` scales its radius off the sphere.
fn section<T: Real>(tilt: f64, h: f64, grow: f64) -> Curve3<T> {
    let f = T::from_f64;
    Curve3::Circle {
        center: Point3::origin() + Vec3::new(f(tilt.sin() * h), T::zero(), f(tilt.cos() * h)),
        axis: Vec3::new(f(tilt.sin()), T::zero(), f(tilt.cos())),
        radius: f(((1.0 + grow).powi(2) - h * h).sqrt()),
        u_ref: Vec3::new(f(-tilt.cos()), T::zero(), f(tilt.sin())),
    }
}

const ARC: (f64, f64) = (0.3, 0.3 + core::f64::consts::FRAC_PI_2);

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn lane_image(carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> NurbsCurve2<f64> {
    FittedLane::<f64>::certified()
        .sphere_circle_image(carrier, t0, t1, &sphere(), band())
        .expect("the lane images the arc")
}

fn certify<T: geom_core::Decide + geom_core::CertifiedBounds>(
    image: &NurbsCurve2<f64>,
    carrier: &Curve3<T>,
    (t0, t1): (f64, f64),
) -> Result<PcurveCache<T>, PcurveCertifyError> {
    let control = image.control().iter().map(|p| p.map(T::from_f64)).collect();
    let image = Arc::new(
        NurbsCurve2::new(image.knots().clone(), control, image.weights().to_vec()).unwrap(),
    );
    let (t0, t1) = (T::from_f64(t0), T::from_f64(t1));
    let window = Pcurve::Fitted(Arc::clone(&image)).chart_box(t0, t1);
    PcurveCache::certify_fitted(
        image,
        t0,
        t1,
        carrier,
        &sphere(),
        None,
        window,
        band(),
        FittedLane::certified(),
    )
}

/// `max |S(P(t)) − C(t)|` over 20 001 points of `[t0, t1]`.
fn dense(image: &NurbsCurve2<f64>, carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> f64 {
    let s = sphere::<f64>();
    let pc = Pcurve::Fitted(Arc::new(image.clone()));
    (0..=20000)
        .map(|k| {
            let t = t0 + (t1 - t0) * f64::from(k) / 20000.0;
            let p = pc.eval(t);
            (s.eval(p.x, p.y) - carrier.eval(t)).norm()
        })
        .fold(0.0, f64::max)
}

/// The spans of a lane image that hold no certification sample.
fn sample_free_spans(image: &NurbsCurve2<f64>, (t0, t1): (f64, f64)) -> Vec<usize> {
    let knots = image.knots().knots();
    let spans = (image.control().len() - 1) / 5;
    let samples: Vec<f64> = (0..9)
        .map(|k| t0 + (t1 - t0) * f64::from(k) / 8.0)
        .collect();
    (0..spans)
        .filter(|&j| {
            let (a, b) = (knots[5 * j + 5], knots[5 * j + 10]);
            !samples.iter().any(|&s| s >= a && s <= b)
        })
        .collect()
}

fn with_control(
    image: &NurbsCurve2<f64>,
    f: impl Fn(usize, Point2<f64>) -> Point2<f64>,
) -> NurbsCurve2<f64> {
    let p = image
        .control()
        .iter()
        .enumerate()
        .map(|(i, q)| f(i, *q))
        .collect();
    NurbsCurve2::new(image.knots().clone(), p, image.weights().to_vec()).unwrap()
}

/// The lane's image with every control from span `j`'s second interior
/// control on shifted a whole period in azimuth: span `j` winds a turn
/// and the rest of the image sits on the next branch. In 3-D the image
/// runs round a latitude, metres off its circle, and span `j` holds no
/// certification sample.
fn wound_image() -> NurbsCurve2<f64> {
    let image = lane_image(&section(0.6, 0.3, 0.0), ARC);
    let j = sample_free_spans(&image, ARC)[0];
    with_control(&image, |i, q| {
        if i >= 5 * j + 3 {
            Point2::new(q.x - core::f64::consts::TAU, q.y)
        } else {
            q
        }
    })
}

fn assert_envelope_refusal(result: Result<(), PcurveCertifyError>, what: &str) {
    let err = result.expect_err(what);
    assert!(
        matches!(
            err,
            PcurveCertifyError::ResidualExceeded {
                check: PcurveCheck::Envelope,
                ..
            }
        ),
        "{what}: the refusal is the envelope's, check 4: {err:?}"
    );
}

/// **A span winding a whole turn refuses at f64.** Red if the bound
/// takes each end of a span on its own branch again.
#[test]
fn a_span_winding_a_whole_turn_refuses() {
    let image = wound_image();
    let carrier = section::<f64>(0.6, 0.3, 0.0);
    assert!(
        dense(&image, &carrier, ARC) > 0.1,
        "the winding is visible in 3-D"
    );
    assert_envelope_refusal(
        certify(&image, &carrier, ARC).map(|_| ()),
        "an image a turn off its branch",
    );
}

/// **The same, at the interval scalar.**
#[test]
fn a_span_winding_a_whole_turn_refuses_at_the_interval_scalar() {
    use geom_core::interval::Interval;
    assert_envelope_refusal(
        certify(&wound_image(), &section::<Interval>(0.6, 0.3, 0.0), ARC).map(|_| ()),
        "an image a turn off its branch, at the interval scalar",
    );
}

/// **An image shorter than its edge refuses.** An image the lane made
/// on `[t0 + s, t1]` is certified against the edge `[t0, t1]`; the
/// piece it does not cover would be read by extrapolating its end span,
/// which nothing bounds. Red if check 4 stops comparing the image's
/// domain with the edge's.
#[test]
fn an_image_shorter_than_its_edge_refuses() {
    let carrier = section::<f64>(0.6, 0.3, 0.0);
    let (t0, t1) = ARC;
    for s in [1e-6, 1e-2, 2e-2] {
        let image = lane_image(&carrier, (t0 + s, t1));
        let err = certify(&image, &carrier, ARC).expect_err("a short image");
        assert!(
            matches!(err, PcurveCertifyError::FittedCertificate { what, .. }
                if what.contains("knot domain")),
            "an image on [t0 + {s:e}, t1] refuses on its domain: {err:?}"
        );
    }
}

/// **The two terms of the bound add.** The circle sits 2e-10 off its
/// sphere and the image is shifted in azimuth so its tangential gap is
/// about twice that: the dense residual combines the two, and the
/// envelope must still bound it. Red if the envelope takes the larger
/// of the two terms instead of their sum.
#[test]
fn the_envelope_takes_the_max_of_two_terms_it_derives_as_a_sum() {
    let off = 2e-10_f64;
    let carrier = section::<f64>(0.6, 0.3, off);
    let image = lane_image(&carrier, ARC);
    let cos_max = (0..=20000)
        .map(|k| {
            let t = ARC.0 + (ARC.1 - ARC.0) * f64::from(k) / 20000.0;
            let p = carrier.eval(t) - Point3::origin();
            p.x.hypot(p.y) / p.norm()
        })
        .fold(0.0, f64::max);
    let shifted = with_control(&image, |_, q| Point2::new(q.x + 2.0 * off / cos_max, q.y));
    let d = dense(&shifted, &carrier, ARC);
    match certify(&shifted, &carrier, ARC) {
        Ok(cache) => {
            let envelope = cache.certificate().envelope;
            assert!(
                d <= envelope,
                "dense {d:e} m above the envelope {envelope:e} m: the bound under-reads"
            );
        }
        // A refusal is also sound: the envelope did not under-read.
        Err(PcurveCertifyError::ResidualExceeded { .. }) => {}
        Err(e) => panic!("an unexpected refusal: {e:?}"),
    }
}
