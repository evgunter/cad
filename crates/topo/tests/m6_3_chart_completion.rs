//! **M6-3 Leg E acceptance (walk row 4): the analytic-chart pcurve
//! completion** — the sphere GENERAL-circle route through the fitted
//! lane, at rest.
//!
//! The closed-form arms (cone rim/ruling, sphere polar/meridian,
//! torus parallel/meridian) are exercised where they LIVE: every
//! revolve/boolean/fillet body now mints them (`chart_mints` flipped;
//! ball/cone/donut/die-octant coverage rides those suites and the
//! corpus). What no construction mints today is the sphere chart's
//! azimuth-NON-harmonic class — a circle neither polar nor meridian —
//! and this file pins its whole route:
//!
//! - the CLOSED-FORM door refuses it typed (`UnsupportedCarrier`, the
//!   class named in `chart_pcurve`'s sphere arm);
//! - the LANE images it (`FittedLane::sphere_circle_image`: the
//!   piecewise cubic Hermite interpolant of its chart image on the
//!   CARRIER'S OWN angular parameter, OQ4), and the FITTED door
//!   (`PcurveCache::certify_fitted`) certifies it against the sphere
//!   alone with the `MapResidualHermite` statement — a bound on the
//!   image's distance from the circle over the whole span;
//! - a corruption of the image BETWEEN its certification samples is
//!   refused (`a_corrupted_image_refuses_between_its_samples`);
//! - the mint reaches that route, and refuses an arc over a pole while
//!   minting an arc of the same circle that avoids it;
//! - the cache survives AT REST: the tier-3 pcurve pass re-derives its
//!   certificate;
//! - the same body certifies at the INTERVAL scalar (`certified`
//!   module), enclosure-asserted.
//!
//! ε posture: no ε literal; every margin is the run's own band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::Surface;
use geom::{Curve3, NurbsCurve2};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, EnvelopeStatement, Pcurve, PcurveCache};
use geom_core::Tol;
use geom_core::{Band, Point3, Real, Vec3};
use topo::Body;

/// The chart sphere: unit-ish radius, polar axis +z.
fn sphere<T: Real>() -> Surface<T> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: T::from_f64(1.0),
        axis: Vec3::new(T::zero(), T::zero(), T::one()),
        u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
    }
}

/// The mate: the tilted cutting plane through the circle (the
/// intensional pair the edge's description names).
fn tilted_plane<T: Real>() -> Surface<T> {
    let tilt = 0.6_f64;
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(T::from_f64(tilt.sin()), T::zero(), T::from_f64(tilt.cos())),
        u_ref: Vec3::new(T::from_f64(tilt.cos()), T::zero(), T::from_f64(-tilt.sin())),
    }
}

/// The GENERAL circle: the tilted plane's great-circle section of the
/// sphere — its plane neither ⊥ the polar axis nor containing it.
fn general_circle<T: Real>() -> Curve3<T> {
    let tilt = 0.6_f64;
    Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(T::from_f64(tilt.sin()), T::zero(), T::from_f64(tilt.cos())),
        radius: T::from_f64(1.0),
        u_ref: Vec3::new(T::from_f64(tilt.cos()), T::zero(), T::from_f64(-tilt.sin())),
    }
}

/// The traversed arc: a quarter turn away from the azimuth seam.
const ARC: (f64, f64) = (0.3, 0.3 + core::f64::consts::FRAC_PI_2);

/// The chart image the LANE derives (`FittedLane::sphere_circle_image`),
/// at `f64` structure (C6) and the run's band — the image the mint
/// stores, not a fixture's own fit of it.
fn lane_image() -> NurbsCurve2<f64> {
    let (t0, t1) = ARC;
    geom_brep::FittedLane::<f64>::certified()
        .sphere_circle_image(
            &general_circle::<f64>(),
            t0,
            t1,
            &sphere::<f64>(),
            Band::linear(Tol::witness()).unwrap(),
        )
        .expect("the lane images the general circle")
}

fn lift2<T: Real>(c: &NurbsCurve2<f64>) -> NurbsCurve2<T> {
    let control = c.control().iter().map(|p| p.map(T::from_f64)).collect();
    NurbsCurve2::new(c.knots().clone(), control, c.weights().to_vec()).expect("lifted structure")
}

/// Build the at-rest body at `T`: a sphere face and a plane face
/// joined by the general-circle edge whose DESCRIPTION names the pair,
/// the sphere side carrying the fitted cache (the M6-2 fixture's
/// public-door pattern).
fn build<T>() -> (Body<T>, topo::HalfEdgeKey)
where
    T: topo::AtRestPolicy,
{
    try_build::<T>().expect("the general circle certifies through the fitted door")
}

/// The spur-edge body every row here builds: a sphere face whose loop
/// carries one `carrier` edge over `arc` (both half-edges, the
/// spur-edge shape), described as the intersection with `plane`, and
/// storing NO pcurve row.
fn spur_body<T>(
    carrier: &Curve3<T>,
    plane: Surface<T>,
    (f0, f1): (f64, f64),
) -> (Body<T>, topo::HalfEdgeKey, topo::HalfEdgeKey)
where
    T: topo::AtRestPolicy,
{
    let (t0, t1) = (T::from_f64(f0), T::from_f64(f1));
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
    let mut body = Body::<T>::new();
    let seed = body.mvfs(p0, true).unwrap();
    let sph_key = body
        .set_face_surface(
            seed.face,
            topo::FaceSurface::New {
                surface: sphere::<T>(),
                sense: true,
            },
        )
        .unwrap();
    let anchor = body.mvfs(p1, true).unwrap();
    let pl_key = body
        .set_face_surface(
            anchor.face,
            topo::FaceSurface::New {
                surface: plane,
                sense: true,
            },
        )
        .unwrap();
    let mid = T::from_f64(0.5 * (f0 + f1));
    let made = body
        .mev(
            topo::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: sph_key,
                    s2: pl_key,
                    witness: carrier.eval(mid),
                },
                carrier: carrier.clone(),
                param_start: t0,
                param_end: t1,
            },
            Tol::witness(),
        )
        .expect("the general-circle edge certifies");
    let edge = body.get_edge(made.edge).expect("edge resolves");
    let (he_plus, he_minus) = (edge.he_plus, edge.he_minus);
    (body, he_plus, he_minus)
}

/// The same construction with the fixture's own image attached, the
/// fitted door's refusal RETURNED rather than panicked on.
fn try_build<T>() -> Result<(Body<T>, topo::HalfEdgeKey), geom_brep::PcurveCertifyError>
where
    T: topo::AtRestPolicy,
{
    let band = Band::linear(Tol::witness()).unwrap();
    let carrier = general_circle::<T>();
    let (f0, f1) = ARC;
    let (t0, t1) = (T::from_f64(f0), T::from_f64(f1));
    let image = Arc::new(lift2::<T>(&lane_image()));
    let (mut body, he_plus, he_minus) = spur_body(&carrier, tilted_plane::<T>(), ARC);
    let window = Pcurve::Fitted(Arc::clone(&image)).chart_box(t0, t1);
    // Both half-edges live in the sphere face's loop (the spur-edge
    // shape), and a face with ANY cache must be complete — attach to
    // both, exactly as the M6-2 fixture does.
    for he in [he_plus, he_minus] {
        let cache = PcurveCache::<T>::certify_fitted(
            Arc::clone(&image),
            t0,
            t1,
            &carrier,
            &sphere::<T>(),
            Some(&tilted_plane::<T>()),
            window,
            band,
            T::fitted_lane().expect("a certifying scalar holds the fitted door"),
        )?;
        body.attach_pcurve(he, cache);
    }
    Ok((body, he_plus))
}

/// The closed-form door refuses the class typed — the fitted lane is
/// the ONLY route (never a silent fallback, C5). The circle lies on
/// the sphere, so the pair is uncovered, not off the chart.
#[test]
fn a_general_circle_refuses_the_closed_form_sphere_door_typed() {
    let band = Band::linear(Tol::witness()).unwrap();
    let err = geom_brep::chart_pcurve(&general_circle::<f64>(), &sphere::<f64>(), band)
        .expect_err("azimuth-non-harmonic");
    assert!(matches!(
        err,
        geom_brep::PcurveCertifyError::UnsupportedCarrier {
            chart: geom::SurfaceKind::Sphere,
            carrier: geom::CurveKind::Circle,
            class: geom_brep::UncoveredClass::SphereGeneralCircle,
        }
    ));
}

/// The at-rest row at `f64`: the cache is fitted, its statement is
/// `MapResidualHermite` with no pair certificate (an exact carrier has no tube
/// to prove), and the tier-3 pcurve pass RE-DERIVES the certificate.
#[test]
fn a_general_circle_sphere_cache_survives_the_at_rest_pass() {
    let (body, he) = build::<f64>();
    let cache = body.pcurve(he).expect("the cache is stored");
    assert!(matches!(cache.pcurve(), Pcurve::Fitted(_)));
    let cert = cache.certificate();
    assert_eq!(cert.statement, EnvelopeStatement::MapResidualHermite);
    assert!(
        cert.ssi.is_none(),
        "a Circle carrier certifies against the chart alone, so no pair certificate: {cert:?}"
    );
    // Schedule residual: every CERT sample is a collocation point of
    // the fit, so the sampled max sits at floating-point noise —
    // asserted against the band, not a literal.
    let band = Band::linear(Tol::witness()).unwrap();
    assert!(cert.max_residual.abs() <= band.zero());
    let findings = topo::pcurves::validate_pcurves(&body, band);
    assert!(findings.is_empty(), "{findings:?}");
}

/// Whether `error` is the honest interval-lane outcome below the
/// default ε: an ESCALATION of a residual check. The image's interval
/// evaluation between its nodes carries an enclosure a few 1e-12 m wide
/// (de Boor at a parameter inside a short span), which a 1e-12 band
/// cannot classify; the escalation says so rather than certify or
/// refuse. At the default ε and above the route certifies.
fn stands_down_below_default_eps(error: &geom_brep::PcurveCertifyError) -> bool {
    Tol::witness().eps() < geom_core::tolerance::DEFAULT_EPS
        && matches!(error, geom_brep::PcurveCertifyError::Escalated { .. })
}

/// **The MINT reaches the route**: the same spur-edge body, storing no
/// row, minted by the public pass. Both half-edges come back `Fitted`
/// (the image `FittedLane::sphere_circle_image` derives, certified by
/// `certify_fitted`'s Circle arm against the chart alone), the face is
/// complete, and the tier-3 pass re-certifies it clean. Red if the mint
/// stops routing the class: the closed-form door refuses it, and the
/// pass would leave the face rowless or refuse.
fn the_mint_derives_and_certifies_a_general_circle_row<T: topo::AtRestPolicy>() {
    let (mut body, he_plus, he_minus) = spur_body(&general_circle::<T>(), tilted_plane(), ARC);
    match topo::mint_pcurves(&mut body, Tol::witness()) {
        Ok(()) => {}
        Err(topo::pcurves::PcurveMintError::Certify { error, .. })
            if stands_down_below_default_eps(&error) =>
        {
            test_utils::vacuity::stood_down(
                &format!("the general circle's mint at {}", T::NAME),
                &format!(
                    "the residual check escalated ({error:?}) at eps = {:e}, so THIS RUN \
                     ASSERTS NO stored row — only that the escalation is the door's typed one",
                    Tol::witness().eps()
                ),
            );
            return;
        }
        Err(e) => panic!("the general circle mints: {e:?}"),
    }
    for he in [he_plus, he_minus] {
        let cache = body.pcurve(he).expect("the mint stored the row");
        assert!(
            matches!(cache.pcurve(), Pcurve::Fitted(_)),
            "a general circle's row is a fitted image: {cache:?}"
        );
        assert_eq!(
            cache.certificate().statement,
            EnvelopeStatement::MapResidualHermite
        );
    }
    let band = Band::linear(Tol::witness()).unwrap();
    let findings = topo::pcurves::validate_pcurves(&body, band);
    assert!(findings.is_empty(), "{findings:?}");
}

#[test]
fn the_mint_derives_and_certifies_a_general_circle_row_at_f64() {
    the_mint_derives_and_certifies_a_general_circle_row::<f64>();
}

#[test]
fn the_mint_derives_and_certifies_a_general_circle_row_at_the_interval_scalar() {
    the_mint_derives_and_certifies_a_general_circle_row::<geom_core::interval::Interval>();
}

/// The small circle through the chart's north pole: the tilted plane
/// moved to pass through the pole cuts the sphere in a circle neither
/// polar nor meridian, with the pole at angle π on it
/// (`pole − centre` is `−sin(tilt)·u_ref`). The cutting plane is its
/// mate.
fn pole_circle() -> (Curve3<f64>, Surface<f64>) {
    let tilt = 0.6_f64;
    let normal = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    let u_ref = Vec3::new(tilt.cos(), 0.0, -tilt.sin());
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal,
        u_ref,
    };
    let circle = Curve3::Circle {
        center: Point3::origin() + normal * tilt.cos(),
        axis: normal,
        radius: tilt.sin(),
        u_ref,
    };
    (circle, plane)
}

/// **The pole fence is the ARC's, and it is loud at the mint.** On the
/// circle through the pole, an arc that crosses the pole has no
/// one-branch image — the azimuth has no value there — so the lane
/// refuses `ArcNearPole` and the mint propagates it rather than leaving
/// the face uncached (red if the class's exemption comes back). An arc
/// of the SAME circle that stays clear of the pole mints and
/// re-certifies (red if the fence reads the circle's plane instead of
/// the arc).
#[test]
fn the_pole_fence_refuses_the_arc_over_the_pole_and_mints_the_arc_clear_of_it() {
    let pi = core::f64::consts::PI;
    let (circle, plane) = pole_circle();
    let (mut over, _, _) = spur_body(&circle, plane.clone(), (pi - 0.5, pi + 0.5));
    let err = topo::mint_pcurves(&mut over, Tol::witness())
        .expect_err("an arc over the pole has no one-branch image");
    assert!(
        matches!(
            err,
            topo::pcurves::PcurveMintError::Certify {
                error: geom_brep::PcurveCertifyError::ArcNearPole,
                ..
            }
        ),
        "the refusal is the arc fence's, typed: {err:?}"
    );
    assert!(
        err.to_string().contains("Recourse:"),
        "the refusal carries a recourse: {err}"
    );
    let (mut clear, he_plus, _) = spur_body(&circle, plane, (0.2, 1.2));
    topo::mint_pcurves(&mut clear, Tol::witness())
        .expect("an arc of the same circle clear of the pole mints");
    assert!(
        matches!(
            clear.pcurve(he_plus).map(|c| c.pcurve()),
            Some(Pcurve::Fitted(_))
        ),
        "the clear arc stores its fitted row"
    );
    let band = Band::linear(Tol::witness()).unwrap();
    let findings = topo::pcurves::validate_pcurves(&clear, band);
    assert!(findings.is_empty(), "{findings:?}");
}

/// **The certificate sees the image between its samples.** The lane's
/// image with one interior control of a span that holds NO
/// certification sample moved by 1e-3 rad in azimuth: check 3 cannot
/// see it, so it must refuse at check 4, the envelope. Red if the span
/// bound stops reading the image's own controls.
#[test]
fn a_corrupted_image_refuses_between_its_samples() {
    let (t0, t1) = ARC;
    let band = Band::linear(Tol::witness()).unwrap();
    let image = lane_image();
    let knots = image.knots().knots();
    let spans = (image.control().len() - 1) / 5;
    let samples: Vec<f64> = (0..9)
        .map(|k| t0 + (t1 - t0) * f64::from(k) / 8.0)
        .collect();
    let free = (0..spans)
        .find(|&j| {
            let (a, b) = (knots[5 * j + 5], knots[5 * j + 10]);
            !samples.iter().any(|&s| s >= a && s <= b)
        })
        .expect("a span with no certification sample");
    let mut control = image.control().to_vec();
    control[5 * free + 2].x += 1e-3;
    let corrupted = Arc::new(
        NurbsCurve2::new(image.knots().clone(), control, image.weights().to_vec())
            .expect("same structure"),
    );
    let window = Pcurve::Fitted(Arc::clone(&corrupted)).chart_box(t0, t1);
    let err = PcurveCache::<f64>::certify_fitted(
        corrupted,
        t0,
        t1,
        &general_circle::<f64>(),
        &sphere::<f64>(),
        None,
        window,
        band,
        geom_brep::FittedLane::certified(),
    )
    .expect_err("a corrupted image does not certify");
    assert!(
        matches!(
            err,
            geom_brep::PcurveCertifyError::ResidualExceeded {
                check: geom_brep::PcurveCheck::Envelope,
                ..
            }
        ),
        "the refusal is the envelope's, check 4: {err:?}"
    );
}

/// The interval row: the same body at the interval scalar — the
/// evidence the lane genuinely left `f64`.
mod certified {
    use geom_core::Tol;
    use geom_core::interval::Interval;

    use super::*;

    /// The Circle arm's envelope is plain arithmetic at the run's scalar
    /// — the circle's distance from the sphere, whose square is a
    /// degree-2 trigonometric polynomial in `t`, bounded by its
    /// coefficients' magnitudes — so at the interval scalar it is an
    /// ENCLOSURE of that bound, and for a great circle of the chart's
    /// own sphere every coefficient encloses zero to the width of the
    /// lifted data. At the default ε the route certifies, and the row
    /// asserts the full at-rest statement with the envelope's SUPREMUM
    /// inside the band; below it, see `stands_down_below_default_eps`.
    #[test]
    fn the_general_circle_route_certifies_at_the_interval_scalar() {
        let (body, he) = match try_build::<Interval>() {
            Ok(built) => built,
            Err(error) if stands_down_below_default_eps(&error) => {
                test_utils::vacuity::stood_down(
                    "the general circle's interval route",
                    &format!(
                        "the residual check escalated ({error:?}) at eps = {:e}, so THIS RUN \
                         ASSERTS NO at-rest certificate — only that the escalation is the \
                         door's typed one",
                        Tol::witness().eps()
                    ),
                );
                return;
            }
            Err(e) => panic!("the general circle certifies through the fitted door: {e:?}"),
        };
        let cache = body.pcurve(he).expect("the cache is stored");
        let cert = cache.certificate();
        assert_eq!(cert.statement, EnvelopeStatement::MapResidualHermite);
        let band = Band::linear(Tol::witness()).unwrap();
        assert!(
            geom_core::Bounds::hi(cert.envelope) <= band.zero(),
            "the envelope's supremum ({:e}) is outside the band",
            geom_core::Bounds::hi(cert.envelope)
        );
        let findings = topo::pcurves::validate_pcurves(&body, band);
        assert!(findings.is_empty(), "{findings:?}");
    }
}
