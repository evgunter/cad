//! **M6-3 Leg E acceptance (walk row 4): the analytic-chart pcurve
//! completion** — the sphere GENERAL-circle route, at rest.
//!
//! The closed-form arms (cone rim/ruling, sphere polar/meridian,
//! torus parallel/meridian) are exercised where they LIVE: every
//! revolve/boolean/fillet body now mints them (`chart_mints` flipped;
//! ball/cone/donut/die-octant coverage rides those suites and the
//! corpus). What no construction mints today is the sphere chart's
//! azimuth-NON-harmonic class — a circle neither polar nor meridian —
//! and this file pins its whole route:
//!
//! - the CLOSED-FORM door answers its routed image: the PROJECTED one
//!   (`Pcurve::Projected`, the chart's inverse of the circle), which
//!   certifies against the sphere alone with the `MapResidualProjected`
//!   statement at every scalar;
//! - the mint reaches that route at `f64` and at the interval scalar,
//!   refuses an arc over a pole (`SectorRefused`) while minting an arc
//!   of the same circle that avoids it, and tier 3 re-certifies it;
//! - a stored row moved off its deck refuses.
//!
//! ε posture: no ε literal; every margin is the run's own band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
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

/// The closed-form door answers the class's ROUTED image, the
/// projected one, and it certifies at the closed-form door itself: a
/// circle's projected row reads no fitted door.
#[test]
fn the_closed_form_door_answers_the_projected_image() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (t0, t1) = ARC;
    let image =
        geom_brep::chart_pcurve_over(&general_circle::<f64>(), t0, t1, &sphere::<f64>(), band)
            .expect("the general circle has a routed image");
    assert!(matches!(image, Pcurve::Projected(_)), "{image:?}");
    let cache =
        PcurveCache::certify(image, t0, t1, &general_circle::<f64>(), &sphere::<f64>(), band)
            .expect("a circle's projected row certifies at the closed-form door");
    assert_eq!(
        cache.certificate().statement,
        EnvelopeStatement::MapResidualProjected
    );
}

/// **The MINT reaches the route**: the same spur-edge body, storing no
/// row, minted by the public pass. Both half-edges come back
/// `Projected`, certified against the chart alone, the face is
/// complete, and the tier-3 pass re-certifies it clean. Red if the mint
/// stops routing the class.
fn the_mint_derives_and_certifies_a_general_circle_row<T: topo::AtRestPolicy>() {
    let (mut body, he_plus, he_minus) = spur_body(&general_circle::<T>(), tilted_plane(), ARC);
    topo::mint_pcurves(&mut body, Tol::witness()).expect("the general circle mints");
    for he in [he_plus, he_minus] {
        let cache = body.pcurve(he).expect("the mint stored the row");
        assert!(
            matches!(cache.pcurve(), Pcurve::Projected(_)),
            "a general circle's row is its projected image: {cache:?}"
        );
        assert_eq!(
            cache.certificate().statement,
            EnvelopeStatement::MapResidualProjected
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
/// one-branch image — the azimuth has no value there — so the
/// derivation refuses `SectorRefused` and the mint propagates it rather
/// than leaving the face uncached (red if the class's exemption comes
/// back). An arc
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
                error: geom_brep::PcurveCertifyError::SectorRefused {
                    channel: geom_brep::SectorChannel::Azimuth,
                    ..
                },
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
            Some(Pcurve::Projected(_))
        ),
        "the clear arc stores its projected row"
    );
    let band = Band::linear(Tol::witness()).unwrap();
    let findings = topo::pcurves::validate_pcurves(&clear, band);
    assert!(findings.is_empty(), "{findings:?}");
}

/// **A stored row off its deck refuses.** The minted row moved by a
/// thousandth of a radian in azimuth — no whole period, no twin — names
/// a different curve, and check 4 refuses it on the deck term. At the
/// interval scalar, where the envelope is the whole certified statement
/// and no schedule runs before it.
#[test]
fn a_projected_row_off_its_deck_refuses() {
    use geom_core::interval::Interval;
    let (f0, f1) = ARC;
    let (t0, t1) = (Interval::from_f64(f0), Interval::from_f64(f1));
    let band = Band::linear(Tol::witness()).unwrap();
    let carrier = general_circle::<Interval>();
    let image = geom_brep::chart_pcurve_over(&carrier, t0, t1, &sphere::<Interval>(), band)
        .unwrap();
    let moved = image.map_affine(
        |p| geom_core::Point2::new(p.x + Interval::from_f64(1e-3), p.y),
        |v| v,
    );
    let err = PcurveCache::certify(moved, t0, t1, &carrier, &sphere::<Interval>(), band)
        .expect_err("a row off its deck does not certify");
    assert!(
        matches!(
            err,
            geom_brep::PcurveCertifyError::ResidualExceeded {
                check: geom_brep::PcurveCheck::EnvelopeTerm(geom_brep::EnvelopeTerm::FidelityU),
                ..
            }
        ),
        "the refusal names the deck's term: {err:?}"
    );
}
