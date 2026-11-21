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
//! - an arc of that circle that ENDS on the pole, stated so by its
//!   producer, images off the reduced radial and certifies, and a pole
//!   end stated off the pole refuses;
//! - a stored row moved off its deck refuses.
//!
//! ε posture: no ε literal; every margin is the run's own band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::{
    EdgeCurveSpec, EdgeDescriptionSpec, EnvelopeStatement, Pcurve, PcurveCache, SurfacePair,
};
use geom_core::Tol;
use geom_core::{Band, Point3, Real, Vec3};
use topo::Body;
use topo::construction::Construct;

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
    Curve3::circle(
        Point3::origin(),
        Vec3::new(T::from_f64(tilt.sin()), T::zero(), T::from_f64(tilt.cos())),
        T::from_f64(1.0),
        Vec3::new(T::from_f64(tilt.cos()), T::zero(), T::from_f64(-tilt.sin())),
    )
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
            topo::construction::FaceSurface::New {
                surface: sphere::<T>(),
                sense: true,
            },
        )
        .unwrap();
    let anchor = body.mvfs(p1, true).unwrap();
    let pl_key = body
        .set_face_surface(
            anchor.face,
            topo::construction::FaceSurface::New {
                surface: plane,
                sense: true,
            },
        )
        .unwrap();
    let mid = T::from_f64(0.5 * (f0 + f1));
    let made = body
        .mev(
            topo::construction::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(sph_key, pl_key),
                    selector: geom_brep::CurveSelector::Witness(carrier.eval(mid)),
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
    let cache = PcurveCache::certify(
        image,
        t0,
        t1,
        &general_circle::<f64>(),
        &sphere::<f64>(),
        band,
    )
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
/// mate. Turned by `delta` about `x` the circle stays on the sphere and
/// misses the pole by `sin(delta)` at `t = π`; at `delta = 0` the turn
/// moves no bit.
fn pole_circle<T: Real>(delta: f64) -> (Curve3<T>, Surface<T>) {
    let tilt = 0.6_f64;
    let (s, c) = tilt.sin_cos();
    let (sd, cd) = delta.sin_cos();
    let v = |x: f64, y: f64, z: f64| {
        Vec3::new(
            T::from_f64(x),
            T::from_f64(cd * y - sd * z),
            T::from_f64(sd * y + cd * z),
        )
    };
    let (normal, u_ref) = (v(s, 0.0, c), v(c, 0.0, -s));
    let plane = Surface::Plane {
        origin: Point3::origin() + v(0.0, 0.0, 1.0),
        normal,
        u_ref,
    };
    let circle = Curve3::circle(
        Point3::origin() + v(s * c, 0.0, c * c),
        normal,
        T::from_f64(s),
        u_ref,
    );
    (circle, plane)
}

/// **The pole fence is the ARC's, and it is loud at the mint.** On the
/// circle through the pole, an arc that crosses the pole has no
/// one-branch image — the azimuth has no value there — so the projected
/// lane's partition refuses its sector, which the lane choice that sent
/// the circle there reads as the lane's own gap, "not yet" with the
/// pose's lever (`NoGeneralLane { gap: LaneGap::Pose }`, PCERT E1's P3),
/// and the mint propagates it rather than leaving the face uncached (red
/// if the class's exemption comes back). An arc
/// of the SAME circle that stays clear of the pole mints and
/// re-certifies (red if the fence reads the circle's plane instead of
/// the arc).
#[test]
fn the_pole_fence_refuses_the_arc_over_the_pole_and_mints_the_arc_clear_of_it() {
    let pi = core::f64::consts::PI;
    let (circle, plane) = pole_circle::<f64>(0.0);
    let (mut over, _, _) = spur_body(&circle, plane.clone(), (pi - 0.5, pi + 0.5));
    let err = topo::mint_pcurves(&mut over, Tol::witness())
        .expect_err("an arc over the pole has no one-branch image");
    assert!(
        matches!(
            err,
            topo::pcurves::PcurveMintError::Certify {
                error: geom_brep::PcurveCertifyError::NoGeneralLane {
                    gap: geom_brep::LaneGap::Pose {
                        channel: geom_brep::SectorChannel::Azimuth,
                        ..
                    },
                    ..
                },
                ..
            }
        ),
        "the refusal is the arc fence's, typed: {err:?}"
    );
    assert!(
        err.render(geom_brep::recourse::Reading::Build)
            .contains("Recourse:"),
        "the refusal carries a recourse: {err_at}",
        err_at = err.render(geom_brep::recourse::Reading::Build)
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

/// The arcs of [`pole_circle`] that END on the pole, with the ends the
/// producer would state: one leaving it, one arriving, and the whole
/// circle from the pole round to it.
fn pole_ended_arcs() -> [((f64, f64), geom_brep::SingularEnds); 3] {
    let pi = core::f64::consts::PI;
    let ends = |start, end| geom_brep::SingularEnds { start, end };
    [
        ((pi, pi + 1.2), ends(true, false)),
        ((pi - 1.2, pi), ends(false, true)),
        ((pi, 3.0 * pi), ends(true, true)),
    ]
}

/// The certificate of [`pole_circle`]`(delta)`'s arc over `(a, b)` with
/// `ends` stated, at the scalar `T`: the derivation's refusal, or the
/// certificate's verdict.
fn pole_verdict<T: geom_core::Decide>(
    delta: f64,
    (a, b): (f64, f64),
    ends: geom_brep::SingularEnds,
) -> Result<(), geom_brep::PcurveCertifyError> {
    let band = Band::linear(Tol::witness()).unwrap();
    let (circle, _) = pole_circle::<T>(delta);
    let surface = sphere::<T>();
    let (t0, t1) = (T::from_f64(a), T::from_f64(b));
    let image = geom_brep::chart_pcurve_over_ends(&circle, t0, t1, ends, &surface, band)?;
    PcurveCache::certify(image, t0, t1, &circle, &surface, band).map(drop)
}

/// **An arc that ends on the pole images and certifies, at every
/// scalar.** Leaving the pole, arriving at it, or both, the image reads
/// its azimuth off the reduced radial and certifies against the sphere
/// alone. The same arc derived with no end stated still stops at the
/// piece holding the pole, refused or (at a tight ε) read in band: what
/// images it is the end's statement, not a wider sector (red if the plain
/// lane starts reading the axis).
fn an_arc_ending_on_the_pole_certifies<T: geom_core::Decide>() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (circle, _) = pole_circle::<T>(0.0);
    let surface = sphere::<T>();
    for ((a, b), ends) in pole_ended_arcs() {
        let (t0, t1) = (T::from_f64(a), T::from_f64(b));
        let image = geom_brep::chart_pcurve_over_ends(&circle, t0, t1, ends, &surface, band)
            .unwrap_or_else(|e| panic!("({a}, {b}) {ends:?}: the arc images, got {e:?}"));
        assert!(
            matches!(&image, Pcurve::Projected(p) if p.pole().is_some()),
            "({a}, {b}): the image reads its pole end: {image:?}"
        );
        PcurveCache::certify(image, t0, t1, &circle, &surface, band)
            .unwrap_or_else(|e| panic!("({a}, {b}) {ends:?}: the image certifies, got {e:?}"));
        let plain = geom_brep::chart_pcurve_over(&circle, t0, t1, &surface, band);
        assert!(
            matches!(
                plain,
                Err(geom_brep::PcurveCertifyError::NoGeneralLane {
                    gap: geom_brep::LaneGap::Pose { .. },
                    ..
                }) | Err(geom_brep::PcurveCertifyError::Escalated {
                    check: geom_brep::PcurveCheck::GeneralLane,
                    ..
                })
            ),
            "({a}, {b}): with no end stated the pole piece refuses, or reads its sector in \
             band, got {plain:?}"
        );
    }
}

#[test]
fn an_arc_ending_on_the_pole_certifies_at_f64() {
    an_arc_ending_on_the_pole_certifies::<f64>();
}

#[test]
fn an_arc_ending_on_the_pole_certifies_at_the_interval_scalar() {
    an_arc_ending_on_the_pole_certifies::<geom_core::interval::Interval>();
}

/// **The pole term is tight at its factor.** The pole circle turned off
/// the pole by `δ` with its end still stated on it: the reduced radial
/// then moves the image's point by up to `2δ` (the factor is attained:
/// near the end the image's azimuth and the carrier's are a half-turn
/// apart across a circle of radius `δ`). The certificate charges `2δ`, so
/// at `δ = 0.3·ε` every arc certifies, and at `δ = 0.55·ε` and `0.75·ε`,
/// just past `ε/2` and well past it, every arc refuses, at both scalars
/// (red if the factor falls to 1.5 or below, or if the interval lane
/// trusts a broken sign).
#[test]
fn the_pole_term_charges_a_stated_end_its_distance_from_the_pole() {
    use geom_core::interval::Interval;
    let eps = Tol::witness().eps();
    for (span, ends) in pole_ended_arcs() {
        for (k, certifies) in [(0.3, true), (0.55, false), (0.75, false)] {
            let delta = k * eps;
            for (scalar, got) in [
                ("f64", pole_verdict::<f64>(delta, span, ends)),
                ("interval", pole_verdict::<Interval>(delta, span, ends)),
            ] {
                assert_eq!(
                    got.is_ok(),
                    certifies,
                    "{span:?} {ends:?} at δ = {k}·ε, {scalar}: {got:?}"
                );
            }
        }
    }
}

/// **An interval past the turn the end's sine keeps its sign on
/// refuses.** Leaving the pole at `π`, the edge reads the reduced radial's
/// direction as the carrier's only up to `3π`; over `(π, 3π + 0.3)` the
/// image near `3π` points a half-turn away from the carrier, and the pole
/// term's excess charges that (red if the excess goes).
#[test]
fn an_arc_past_the_pole_ends_turn_refuses() {
    use geom_core::interval::Interval;
    let pi = core::f64::consts::PI;
    let span = (pi, 3.0 * pi + 0.3);
    let ends = geom_brep::SingularEnds {
        start: true,
        end: false,
    };
    for (scalar, got) in [
        ("f64", pole_verdict::<f64>(0.0, span, ends)),
        ("interval", pole_verdict::<Interval>(0.0, span, ends)),
    ] {
        assert!(got.is_err(), "{scalar}: past the turn refuses, got {got:?}");
    }
}

/// **At the pole the image is the end's limit along the edge.** The
/// point at the pole end has latitude `π/2` and the azimuth of the
/// carrier's radial direction there: its tangent leaving the pole, or
/// its tangent reversed arriving (an independent oracle, spelled here
/// from the carrier's derivative). Across the edge the image stays on one
/// branch, the azimuth moving by less than a turn (red if the end reads
/// the carrier's own rounded radial, whose direction at the pole is
/// noise).
#[test]
fn the_image_at_a_pole_end_is_its_limit_along_the_edge() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (circle, _) = pole_circle::<f64>(0.0);
    let surface = sphere::<f64>();
    for ((a, b), ends) in pole_ended_arcs() {
        let image = geom_brep::chart_pcurve_over_ends(&circle, a, b, ends, &surface, band).unwrap();
        for (at, leaving, stated) in [(a, true, ends.start), (b, false, ends.end)] {
            if !stated {
                continue;
            }
            let p = image.eval(at);
            assert!(
                (p.y - core::f64::consts::FRAC_PI_2).abs() < 1e-12,
                "({a}, {b}) at {at}: the pole's latitude, got {}",
                p.y
            );
            let (_, d) = circle.ders1(at);
            let d = if leaving { d } else { -d };
            let want = d.y.atan2(d.x);
            let off = (p.x - want).rem_euclid(core::f64::consts::TAU);
            assert!(
                off.min(core::f64::consts::TAU - off) < 1e-9,
                "({a}, {b}) at {at}: the azimuth is the radial direction's limit, got {} for {want}",
                p.x
            );
        }
        let (u0, u1) = (image.eval(a).x, image.eval(b).x);
        assert!(
            (u1 - u0).abs() < core::f64::consts::TAU,
            "({a}, {b}): one branch, got {u0} → {u1}"
        );
    }
}

/// **Each sub-span's chart box holds the image.** On every 64th of each
/// pole-ended arc, the box the image reports holds the image's own points
/// sampled across it (red if the reduced radial's box is read over any
/// parameter but `t/2`).
#[test]
fn a_pole_ended_images_chart_box_holds_it() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (circle, _) = pole_circle::<f64>(0.0);
    let surface = sphere::<f64>();
    for ((a, b), ends) in pole_ended_arcs() {
        let image = geom_brep::chart_pcurve_over_ends(&circle, a, b, ends, &surface, band).unwrap();
        let n = 64;
        for i in 0..n {
            let at = |k: u32| a + (b - a) * f64::from(k) / f64::from(n);
            let (s0, s1) = (at(i), at(i + 1));
            let w = image.chart_box(s0, s1);
            for j in 0..=8 {
                let t = s0 + (s1 - s0) * f64::from(j) / 8.0;
                let p = image.eval(t);
                let out = (w.u_min - p.x)
                    .max(p.x - w.u_max)
                    .max(w.v_min - p.y)
                    .max(p.y - w.v_max);
                assert!(
                    out <= 1e-12,
                    "({a}, {b}) on ({s0}, {s1}) at {t}: the box misses the image by {out:e}"
                );
            }
        }
    }
}

/// **The azimuth's speed bound holds the image's rate.** The pole-ended
/// image's `u` speed bound is at least the rate its azimuth moves, read by
/// central differences across each arc, and the pole end bounds it (red
/// if the reduced radial's rate is under-read, or if the end reads the
/// carrier's own radial, whose rate has no bound at the pole).
#[test]
fn a_pole_ended_images_speed_bound_holds_its_rate() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (circle, _) = pole_circle::<f64>(0.0);
    let surface = sphere::<f64>();
    for ((a, b), ends) in pole_ended_arcs() {
        let image = geom_brep::chart_pcurve_over_ends(&circle, a, b, ends, &surface, band).unwrap();
        let (u_bound, _) = image
            .speed_bound(a, b)
            .unwrap_or_else(|e| panic!("({a}, {b}): the image has a speed bound, got {e:?}"));
        let (n, h) = (2000, 1e-6);
        let mut fastest = 0.0_f64;
        for i in 1..n {
            let t = a + (b - a) * f64::from(i) / f64::from(n);
            let rate = (image.eval(t + h).x - image.eval(t - h).x).abs() / (2.0 * h);
            fastest = fastest.max(rate);
        }
        assert!(
            fastest <= u_bound * (1.0 + 1e-6),
            "({a}, {b}): the azimuth moves at {fastest}, past its bound {u_bound}"
        );
        assert!(fastest > 0.0, "({a}, {b}): the azimuth moves");
    }
}

/// **A pole end stated where there is none refuses.** The general circle
/// passes nowhere near the pole, so stating its start on the pole charges
/// the reduction's miss, the radial part there, as the envelope's
/// fidelity. At the interval scalar, where the envelope is the whole
/// certified statement, check 4 refuses it on that term (red if the pole
/// term goes); at `f64` the schedule's cross-check sees the miss first.
#[test]
fn a_pole_end_stated_off_the_pole_refuses() {
    use geom_core::interval::Interval;
    let band = Band::linear(Tol::witness()).unwrap();
    let ends = geom_brep::SingularEnds {
        start: true,
        end: false,
    };
    let (t0, t1) = ARC;
    let at_f64 = geom_brep::chart_pcurve_over_ends(
        &general_circle::<f64>(),
        t0,
        t1,
        ends,
        &sphere::<f64>(),
        band,
    )
    .and_then(|image| PcurveCache::certify(image, t0, t1, &general_circle(), &sphere(), band));
    assert!(
        matches!(
            at_f64,
            Err(geom_brep::PcurveCertifyError::ResidualExceeded {
                check: geom_brep::PcurveCheck::MapResidual,
                ..
            })
        ),
        "at f64 the cross-check refuses the stated pole end: {at_f64:?}"
    );
    let (t0, t1) = (Interval::from_f64(t0), Interval::from_f64(t1));
    let circle = general_circle::<Interval>();
    let surface = sphere::<Interval>();
    let at_interval = geom_brep::chart_pcurve_over_ends(&circle, t0, t1, ends, &surface, band)
        .and_then(|image| PcurveCache::certify(image, t0, t1, &circle, &surface, band));
    assert!(
        matches!(
            at_interval,
            Err(geom_brep::PcurveCertifyError::ResidualExceeded {
                check: geom_brep::PcurveCheck::EnvelopeTerm(geom_brep::EnvelopeTerm::Fidelity),
                ..
            })
        ),
        "at the interval scalar the envelope refuses on fidelity: {at_interval:?}"
    );
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
    let image =
        geom_brep::chart_pcurve_over(&carrier, t0, t1, &sphere::<Interval>(), band).unwrap();
    let moved = image.translate(Interval::from_f64(1e-3), Interval::from_f64(0.0));
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
