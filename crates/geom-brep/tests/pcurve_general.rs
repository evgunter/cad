//! **U2's `General` arm** (PCURVE P-1a, spec item 2): the general
//! curve-in-UV certifying at the honest Fitted grade.
//!
//! `PcurveCache::certify_fitted` was callerless, and its own docs
//! named this arm as the waiting consumer. The rows here are the
//! ε-row for the new door — its three outcomes, each drawn rather
//! than asserted.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::shared::fixture;
use crate::shared::surf;
use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::recourse::Reading;
use geom_brep::{FittedLane, Pcurve, PcurveCache, PcurveCertifyError};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3, Vec3};

/// A rational quarter-cylinder wall of radius 1 about the z axis: the
/// `u = 0` boundary column is the ruling `x = 1, y = 0`.
fn quarter_cylinder_wall() -> Surface<f64> {
    Surface::Nurbs(Arc::new(fixture::quarter_cylinder_wall()))
}

/// The wall's `u = 0` ruling as a rung-3 carrier.
fn ruling() -> Curve3<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("knots");
    let n = NurbsCurve3::new(
        k,
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0)],
        vec![1.0, 1.0],
    )
    .expect("the carrier builds");
    Curve3::Nurbs(Arc::new(n))
}

/// The chart image `(0, v)` as a general curve-in-UV.
fn image(u: f64) -> Arc<NurbsCurve2<f64>> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("image knots");
    Arc::new(
        NurbsCurve2::new(
            k,
            vec![Point2::new(u, 0.0), Point2::new(u, 1.0)],
            vec![1.0, 1.0],
        )
        .expect("the image builds"),
    )
}

/// **ε-row, outcome CERTIFY**: the ruling's own column as a general
/// image, certified against its chart alone — the row reads no mate
/// surface, since an intersection carrier's uniqueness tube is the
/// edge's certificate.
#[test]
fn a_general_image_certifies_against_its_chart_alone() {
    let got = PcurveCache::certify_general(
        image(0.0),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        band(),
        Some(FittedLane::certified()),
    )
    .expect("the ruling's column certifies with no mate in hand");
    assert!(
        got.certificate().envelope <= band().zero(),
        "the hull bound is within ε: {:?}",
        got.certificate()
    );
}

/// **ε-row, outcome REFUSE**: the wall's OTHER boundary column as the
/// claimed image of this ruling. It is a legal curve-in-UV on a legal
/// chart, and it is not this carrier's image — a definite, measured
/// failure, never an escalation.
#[test]
fn a_general_image_of_the_wrong_column_refuses_definitely() {
    let got = PcurveCache::certify_general(
        image(1.0),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        band(),
        Some(FittedLane::certified()),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::ResidualExceeded { .. }
                | PcurveCertifyError::FittedLimb { .. }
                | PcurveCertifyError::FittedCertificate { .. })
        ),
        "the wrong column is a definite refusal, not an escalation: {got:?}"
    );
}

/// The closed-form door refuses a general image by kind: `General` is
/// the fitted GRADE's arm, and `certify` is the closed-form lane's
/// door. Two doors, one grade each — no arm is ever a catch-all for
/// the other.
#[test]
fn the_closed_form_door_refuses_a_general_image() {
    let got = PcurveCache::certify(
        Pcurve::General(image(0.0)),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        band(),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::ImageMismatch {
                image: geom_brep::PcurveKind::General,
                ..
            })
        ),
        "the closed-form door has no general arm: {got:?}"
    );
}

/// **An analytic chart holds no fitted-grade image.** Its image of a
/// carrier with no closed form is the projected one, so the general
/// door refuses a spline image on a cylinder or a sphere chart by kind,
/// whatever the carrier.
#[test]
fn an_analytic_chart_refuses_a_fitted_grade_image() {
    let cylinder = surf::cylinder::<f64>(1.0);
    let got = PcurveCache::certify_general(
        image(0.0),
        0.0,
        1.0,
        &ruling(),
        &cylinder,
        band(),
        Some(FittedLane::certified()),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::ImageMismatch {
                image: geom_brep::PcurveKind::Fitted,
                ..
            })
        ),
        "an analytic chart refuses the fitted grade by kind: {got:?}"
    );
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: geom_core::Vec3::new(0.6_f64.sin(), 0.0, 0.6_f64.cos()),
        radius: 1.0,
        u_ref: geom_core::Vec3::new(0.6_f64.cos(), 0.0, -0.6_f64.sin()),
    };
    let got = PcurveCache::certify_general(
        image(0.0),
        0.0,
        1.0,
        &circle,
        &surf::sphere::<f64>(1.0),
        band(),
        Some(FittedLane::certified()),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::ImageMismatch {
                image: geom_brep::PcurveKind::Fitted,
                ..
            })
        ),
        "the chart decides, the circle carrier nothing: {got:?}"
    );
}

/// **The two fitted-grade doors run one check sequence**: the same
/// inputs through the general and the fitted door produce the same
/// certificate. `General` is the fitted GRADE, and the two doors differ
/// only in what their callers may assume, never in what the kernel
/// measured.
#[test]
fn the_general_and_fitted_doors_certify_alike() {
    let general = PcurveCache::certify_general(
        image(0.0),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        band(),
        Some(FittedLane::certified()),
    )
    .expect("the ruling certifies through the general door");
    assert!(
        matches!(general.pcurve(), Pcurve::General(_)),
        "the door stores the arm it was entered through"
    );
    let fitted = PcurveCache::certify_fitted(
        image(0.0),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        band(),
        FittedLane::certified(),
    )
    .expect("the same inputs certify through the fitted door");
    assert_eq!(
        format!("{:?}", general.certificate()),
        format!("{:?}", fitted.certificate()),
        "the two doors run one check sequence"
    );
}

/// The quarter-cylinder wall's chart `u` at azimuth `theta`: its
/// rational-quadratic arc's angle, inverted by bisection to roundoff.
fn wall_u(theta: f64) -> f64 {
    let w = core::f64::consts::FRAC_1_SQRT_2;
    let angle = |u: f64| {
        let (a, b, c) = ((1.0 - u) * (1.0 - u), 2.0 * u * (1.0 - u) * w, u * u);
        (b + c).atan2(a + b)
    };
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if angle(mid) < theta {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// The image of a carrier on the quarter-cylinder wall over `[t0, t1]`
/// a construction would state: its exact chart feet at 257 stations
/// (`u` from the azimuth, `v` the height), interpolated at degree 5 on
/// its own parameter.
fn stated_image(carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> Arc<NurbsCurve2<f64>> {
    let n = 256;
    let fracs: Vec<f64> = (0..=n).map(|i| f64::from(i) / f64::from(n)).collect();
    let feet: Vec<Point2<f64>> = fracs
        .iter()
        .map(|f| {
            let p = carrier.eval(t0 + (t1 - t0) * f);
            Point2::new(wall_u(p.y.atan2(p.x)), p.z)
        })
        .collect();
    let fit = NurbsCurve2::<f64>::interpolate_with_params(&feet, 5, &fracs).expect("the fit");
    Arc::new(fit.on_domain(t0, t1).expect("on the carrier's interval"))
}

/// A height-`z` circle on the quarter-cylinder wall, `t` its angle.
fn circle(z: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The wall's section by the plane `z = 0.4 + 0.3·x`:
/// `(cos t, sin t, 0.4 + 0.3·cos t)`.
fn ellipse() -> Curve3<f64> {
    let k = 0.3_f64;
    let s = (1.0 + k * k).sqrt();
    Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 0.4),
        axis: Vec3::new(-k / s, 0.0, 1.0 / s),
        semi_u: s,
        semi_v: 1.0,
        u_ref: Vec3::new(1.0 / s, 0.0, k / s),
    }
}

/// **A circle and an ellipse on a spline chart certify at the fitted
/// grade**, each carrier enclosed span by span in certification
/// arithmetic; the same image offered for a carrier a millimetre off
/// the wall refuses at the hull.
#[test]
fn a_circle_and_an_ellipse_certify_on_a_spline_chart() {
    let wall = quarter_cylinder_wall();
    let span = (0.1, 1.4);
    for (name, carrier, off) in [
        ("circle", circle(0.5), circle(0.501)),
        ("ellipse", ellipse(), {
            let Curve3::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } = ellipse()
            else {
                unreachable!()
            };
            Curve3::Ellipse {
                center: center + Vec3::new(0.0, 0.0, 1e-3),
                axis,
                semi_u,
                semi_v,
                u_ref,
            }
        }),
    ] {
        let image = stated_image(&carrier, span);
        let got = PcurveCache::certify_general(
            Arc::clone(&image),
            span.0,
            span.1,
            &carrier,
            &wall,
            band(),
            Some(FittedLane::certified()),
        )
        .unwrap_or_else(|e| panic!("{name}: certifies on the spline chart: {e}"));
        assert!(
            got.certificate().envelope <= band().zero(),
            "{name}: the hull bound is within ε: {:?}",
            got.certificate()
        );
        let refused = PcurveCache::certify_general(
            image,
            span.0,
            span.1,
            &off,
            &wall,
            band(),
            Some(FittedLane::certified()),
        );
        assert!(
            matches!(
                refused,
                Err(PcurveCertifyError::ResidualExceeded { .. }
                    | PcurveCertifyError::FittedLimb { .. }
                    | PcurveCertifyError::FittedCertificate { .. })
            ),
            "{name}: a millimetre off the wall refuses definitely: {refused:?}"
        );
    }
}

/// **A stated image wrong only between samples refuses at the hull**,
/// over the ruling as a spline carrier and as a line alike. The
/// ruling's column, bent a millimetre off the wall's `u = 0` edge
/// midway between each pair of the certificate's samples and exact at
/// every sample: check 3 passes and check 4's hull refuses.
/// The circle's or ellipse's image with exact feet at every
/// certificate sample and a foot pushed `bow` in chart `u` midway
/// between each pair, interpolated at degree 3 through all of them on
/// the carrier's parameter: the checks that sample see nothing, and
/// only the hull sees the bow.
fn bowed_image(carrier: &Curve3<f64>, (t0, t1): (f64, f64), bow: f64) -> Arc<NurbsCurve2<f64>> {
    let n = geom_brep::CERT_SAMPLES;
    let foot = |t: f64| {
        let p = carrier.eval(t);
        Point2::new(wall_u(p.y.atan2(p.x)), p.z)
    };
    let (mut fracs, mut feet) = (Vec::new(), Vec::new());
    for i in 0..n {
        let t = geom_brep::certify::sample_param(t0, t1, i);
        fracs.push((t - t0) / (t1 - t0));
        feet.push(foot(t));
        if i + 1 < n {
            let fm = (f64::from(i) + 0.5) / f64::from(n - 1);
            let mut q = foot(t0 + (t1 - t0) * fm);
            q.x += bow;
            fracs.push(fm);
            feet.push(q);
        }
    }
    let fit = NurbsCurve2::<f64>::interpolate_with_params(&feet, 3, &fracs).expect("the fit");
    Arc::new(fit.on_domain(t0, t1).expect("on the carrier's interval"))
}

/// **Check 4 holds a curved carrier's image to its chart between the
/// samples.** A circle and an ellipse on the rational quarter-cylinder,
/// their images exact at every certificate sample and bowed 1e-3,
/// 1e-5 and 1e-7 in `u` between them (each where it clears the band):
/// checks 1–3 see an exact image, and the hull refuses it
/// (`FittedCertificate`, check 4) at `f64` and at `Interval`. The exact
/// images certify at both, except at Interval under the tightest band
/// (body comment).
#[test]
fn a_curved_carriers_image_bowed_between_samples_refuses_at_the_hull_at_every_scalar() {
    use geom_core::Real;
    use geom_core::interval::Interval;
    let wall = quarter_cylinder_wall();
    let span = (0.1, 1.4);
    let lift = Interval::from_f64;
    let at_interval = |image: &NurbsCurve2<f64>, carrier: &Curve3<f64>| {
        PcurveCache::certify_general(
            Arc::new(image.map_scalar(lift)),
            lift(span.0),
            lift(span.1),
            &carrier.map_scalar(lift),
            &wall.map_scalar(lift),
            band(),
            Some(FittedLane::<Interval>::certified()),
        )
        .map(|_| ())
    };
    let at_f64 = |image: &Arc<NurbsCurve2<f64>>, carrier: &Curve3<f64>| {
        PcurveCache::certify_general(
            Arc::clone(image),
            span.0,
            span.1,
            carrier,
            &wall,
            band(),
            Some(FittedLane::certified()),
        )
        .map(|_| ())
    };
    // A bow is only a defect past the band: at ε = 1e-6 a 1e-7 bow is
    // inside it and certifies, as it should. Each bow is read where it
    // clears the escalation edge by a decade.
    let bows: Vec<f64> = [1e-3, 1e-5, 1e-7]
        .into_iter()
        .filter(|bow| *bow > 10.0 * band().escalate())
        .collect();
    assert!(
        !bows.is_empty(),
        "at least the 1e-3 bow clears every CI band"
    );
    for (name, carrier) in [("circle", circle(0.5)), ("ellipse", ellipse())] {
        let exact = stated_image(&carrier, span);
        assert_eq!(
            at_f64(&exact, &carrier),
            Ok(()),
            "{name}: the exact image certifies at f64"
        );
        // At Interval the rational wall's own evaluation is a few 1e-12
        // m wide, so at the tightest band the exact image's sampled
        // checks escalate before check 4 is asked: there the Interval
        // leg has no hull to reach, and says so rather than passing.
        let interval = match at_interval(&exact, &carrier) {
            Ok(()) => true,
            Err(PcurveCertifyError::Escalated {
                check: geom_brep::PcurveCheck::MapResidual,
                ..
            }) if band().escalate() < 1e-10 => false,
            other => panic!("{name}: the exact image certifies at Interval: {other:?}"),
        };
        for &bow in &bows {
            let bent = bowed_image(&carrier, span, bow);
            let mut legs = vec![("f64", at_f64(&bent, &carrier))];
            if interval {
                legs.push(("Interval", at_interval(&bent, &carrier)));
            }
            for (scalar, got) in legs {
                assert!(
                    matches!(
                        got,
                        Err(PcurveCertifyError::FittedLimb { .. }
                            | PcurveCertifyError::FittedCertificate { .. })
                    ),
                    "{name} bowed {bow:e} at {scalar}: refused at the hull (check 4), no \
                     earlier: {got:?}"
                );
            }
        }
    }
}

/// **A spline image short of the edge refuses.** The bound is over the
/// image's domain, and a spline carrier evaluated past its knot domain
/// clamps, so nothing else notices an edge interval that overruns it:
/// check 4 refuses it, at either end, and certifies one inside.
#[test]
fn a_spline_image_short_of_the_edge_refuses() {
    for ((a, b), certifies) in [
        ((0.0, 1.5), false),
        ((-0.5, 1.0), false),
        ((0.2, 0.8), true),
    ] {
        let got = PcurveCache::certify_general(
            image(0.0),
            a,
            b,
            &ruling(),
            &quarter_cylinder_wall(),
            band(),
            Some(FittedLane::certified()),
        );
        if certifies {
            assert!(got.is_ok(), "[{a}, {b}] lies in the image: {got:?}");
        } else {
            assert!(
                matches!(got, Err(PcurveCertifyError::FittedCertificate { .. })),
                "[{a}, {b}] overruns the image's [0, 1]: {got:?}"
            );
        }
    }
}

#[test]
fn a_general_image_wrong_between_samples_refuses_at_the_hull() {
    let n = geom_brep::CERT_SAMPLES;
    let mut points = Vec::new();
    for i in 0..n {
        let t = f64::from(i) / f64::from(n - 1);
        points.push(Point2::new(0.0, t));
        if i + 1 < n {
            let mid = (f64::from(i) + 0.5) / f64::from(n - 1);
            points.push(Point2::new(1e-3, mid));
        }
    }
    let mut knots = vec![0.0, 0.0];
    let count = points.len();
    knots.extend((1..count - 1).map(|k| points[k].y));
    knots.extend([1.0, 1.0]);
    let bent = NurbsCurve2::new(
        KnotVector::clamped(knots, 1).expect("knots"),
        points,
        vec![1.0; count],
    )
    .expect("the bent image builds");
    let bent = Arc::new(bent);
    let line = Curve3::Line {
        origin: Point3::new(1.0, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    for (name, carrier) in [("spline", ruling()), ("line", line)] {
        let got = PcurveCache::certify_general(
            Arc::clone(&bent),
            0.0,
            1.0,
            &carrier,
            &quarter_cylinder_wall(),
            band(),
            Some(FittedLane::certified()),
        );
        let Err(refusal @ PcurveCertifyError::FittedLimb { limb, .. }) = got else {
            panic!("{name}: the bend between samples refuses at the hull: {got:?}");
        };
        // The bend sits on a break of the composite, so the hull reads
        // the stored image's value there past the band: the image's own
        // miss, which no loosening repairs.
        assert_eq!(
            limb,
            geom_brep::SsiLimb::HullValueChart,
            "{name}: check 4's hull reads the bend at its break"
        );
        assert_eq!(
            refusal.ending(Reading::AtRest).as_deref(),
            Some(geom_brep::recourse::defect_ending(Reading::AtRest)),
            "{name}: a stored image a millimetre off is a stored contradiction: {refusal}"
        );
    }
}

/// The bilinear wall `z = 0` over `[0, 1]²`, its chart the identity.
fn flat_wall() -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("knots");
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .into_iter()
        .map(|(x, y)| Point3::new(x, y, 0.0))
        .collect();
    NurbsSurface::new(k.clone(), k, control, vec![1.0; 4]).expect("the wall builds")
}

/// **The derived route for a closed-form carrier**: the mint's two
/// doors in its order, `general_image` over the edge's interval and
/// then `certify_general`. An off-iso line on a flat wall has a
/// straight foot path, so the degree-1 image certifies. A circle on the
/// curved wall does not: the image's chord error is measured at the
/// hull and refused typed, never excused.
#[test]
fn the_derived_route_certifies_a_line_and_refuses_a_circle_at_the_hull() {
    let lane = FittedLane::<f64>::certified();
    let r = core::f64::consts::FRAC_1_SQRT_2;
    let diagonal = Curve3::Line {
        origin: Point3::new(0.1, 0.2, 0.0),
        dir: Vec3::new(r, r, 0.0),
    };
    let (t0, t1) = (0.0, 0.9);
    let wall = flat_wall();
    let image = lane
        .general_image(&diagonal, (t0, t1), &wall)
        .expect("the diagonal's image derives");
    let got = PcurveCache::certify_general(
        Arc::new(image),
        t0,
        t1,
        &diagonal,
        &Surface::Nurbs(Arc::new(wall)),
        band(),
        Some(lane),
    )
    .expect("an off-iso line's derived image certifies on a spline chart");
    assert!(
        matches!(got.pcurve(), Pcurve::General(_)) && got.certificate().envelope <= band().zero(),
        "a General row within ε: {:?}",
        got.certificate()
    );

    let (t0, t1) = (0.1, 1.4);
    let image = lane
        .general_image(&circle(0.5), (t0, t1), &fixture::quarter_cylinder_wall())
        .expect("the circle's image derives");
    let got = PcurveCache::certify_general(
        Arc::new(image),
        t0,
        t1,
        &circle(0.5),
        &quarter_cylinder_wall(),
        band(),
        Some(lane),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::FittedLimb {
                limb: geom_brep::SsiLimb::HullSupChart | geom_brep::SsiLimb::HullValueChart,
                ..
            })
        ),
        "a curved foot path's degree-1 image refuses at the hull: {got:?}"
    );
}
