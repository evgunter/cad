//! **A row stored as the sphere's involution twin certifies at the
//! certifying scalar.** The pole-crossing half cap's meridian arc is
//! stored by the loop walk as the sphere's twin `(u + π, π − v)`
//! (`topo::pcurves::sphere_twin`), and check 4's fidelity reads it
//! against the twin's branch (`geom_brep::whole_periods`, decided as
//! `pcurve_fidelity_twin`). An exact twin's azimuth offset is π, which
//! is the centred fold's jump: a selection read off that fold encloses
//! both signs over a box and kept the derivation's own name, so the cap
//! minted at f64 and refused at `Interval` (PR 3812's review R1,
//! MAJOR-1). Built at both scalars here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::spline::SpanLocate;
use geom_core::{Bounds, Interval, Point3, Real, Tol, Vec3};
use topo::{AtRestPolicy, Body, FaceSurface, MefSite, MevSite};

/// The cap with its rim turned `−phi` and its meridian `+phi` about the
/// pole axis: every carrier end then sits `phi·r` off its vertex (`r`
/// the rim's radius), both circles still exactly on the sphere.
fn half_cap<T: Real + SpanLocate + AtRestPolicy>(phi: f64) -> Result<Body<T>, String> {
    let tol = Tol::witness();
    let f = T::from_f64;
    let z = 0.5_f64;
    let r = (1.0 - z * z).sqrt();
    let p = |x: f64, y: f64, w: f64| Point3::new(f(x), f(y), f(w));
    let v = |x: f64, y: f64, w: f64| Vec3::new(f(x), f(y), f(w));
    let (a, b) = (p(r, 0.0, z), p(-r, 0.0, z));
    let rim = Curve3::Circle {
        center: p(0.0, 0.0, z),
        axis: v(0.0, 0.0, 1.0),
        radius: f(r),
        u_ref: v(phi.cos(), -phi.sin(), 0.0),
    };
    // Oriented as `mesh`'s witness body orients its pole-crossing half
    // cap (`mesh/tests/common/witness_bodies.rs`).
    let g = Curve3::Circle {
        center: p(0.0, 0.0, 0.0),
        axis: v(-phi.sin(), phi.cos(), 0.0),
        radius: f(1.0),
        u_ref: v(-r * phi.cos(), -r * phi.sin(), z),
    };
    let t_end = f(2.0 * core::f64::consts::FRAC_PI_3);
    let mut body = Body::<T>::new();
    let seed = body.mvfs(a, true).map_err(|e| format!("mvfs {e:?}"))?;
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Sphere {
                center: p(0.0, 0.0, 0.0),
                radius: f(1.0),
                axis: v(0.0, 0.0, 1.0),
                u_ref: v(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .map_err(|e| format!("surface {e:?}"))?;
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim, f(0.0), T::pi()).unwrap(),
            tol,
        )
        .map_err(|e| format!("mev {e:?}"))?;
    body.mef(
        MefSite::Chords {
            he1: e_rim.he_minus,
            he2: e_rim.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(g, f(0.0), t_end).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .map_err(|e| format!("mef {e:?}"))?;
    topo::pcurves::mint_pcurves(&mut body, tol).map_err(|e| format!("mint {e:?}"))?;
    Ok(body)
}

/// **Which representation the walk picks**: of the half cap's four
/// rows, exactly one — the pole-crossing arc's, met across the pole —
/// is stored as the involution twin of its derivation (azimuth `π` on,
/// polar `π − v`), and the rest as derived, at both scalars. The joint
/// before it lifts only through the twin: the derivation's own azimuth
/// sits on a half-period mark of the branch decision.
fn twins<T: Real + SpanLocate + AtRestPolicy + Bounds>(body: &Body<T>) -> usize {
    use geom_brep::{Pcurve, chart_pcurve};
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let near = |x: T, to: f64| (x.lo() - to).abs() < 1e-12 && (x.hi() - to).abs() < 1e-12;
    let mut twins = 0;
    for (he, h) in body.half_edges() {
        let row = body.pcurve(he).expect("every half of the cap stores a row");
        let edge = body.get_edge(h.edge).unwrap();
        let Some(topo::CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
            panic!("a certified edge")
        };
        let face = body.face_of_half_edge(he).unwrap();
        let sphere = body
            .get_surface(body.get_face(face).unwrap().surface)
            .unwrap();
        let derived = chart_pcurve(curve.carrier(), sphere, band).unwrap();
        let (Pcurve::Harmonic { p0: d, .. }, Pcurve::Harmonic { p0: s, .. }) =
            (&derived, row.pcurve())
        else {
            panic!("the cap's rows are harmonic")
        };
        if near(s.x - d.x, core::f64::consts::PI) && near(s.y + d.y, core::f64::consts::PI) {
            twins += 1;
        } else {
            assert!(
                near(s.x - d.x, 0.0) && near(s.y - d.y, 0.0),
                "{he:?}: a row is its derivation or its twin"
            );
        }
    }
    twins
}

#[test]
fn the_walk_stores_one_row_as_the_twin() {
    assert_eq!(twins(&half_cap::<f64>(0.0).unwrap()), 1, "at f64");
    assert_eq!(twins(&half_cap::<Interval>(0.0).unwrap()), 1, "at Interval");
}

#[test]
fn the_pole_crossing_half_cap_mints_at_f64_and_at_interval() {
    let at_f64 = half_cap::<f64>(0.0).map(|_| ());
    let at_iv = half_cap::<Interval>(0.0).map(|_| ());
    assert!(
        at_f64.is_ok() && at_iv.is_ok(),
        "f64 {at_f64:?} / Interval {at_iv:?}"
    );
}

/// **The twin holds at a tight K, and tier 3 refuses the wrong sheet.**
/// The cap is turned so every carrier end sits `0.99ε` off its vertex,
/// and the run's K is `1.5` (`Tolerance::init`, this process only): the
/// joint across the pole then sits `≈ 2ε` in metres off the twin's
/// exact azimuth, past `K·ε`. The walk still stores exactly one row as
/// the twin, and the body reads clean. Restating that row on its OTHER
/// sheet (the twin of the twin: the derivation a whole period over)
/// puts an interior twin jump in the loop, which tier 3 refuses as a
/// discontinuity.
fn tight_k<T: Real + SpanLocate + AtRestPolicy + Bounds>(lane: &str, eps: f64) {
    use geom_brep::{Pcurve, PcurveCache};
    let r = 0.75_f64.sqrt();
    let mut body = half_cap::<T>(0.99 * eps / r).unwrap_or_else(|e| panic!("{lane}: {e}"));
    assert_eq!(twins(&body), 1, "{lane}: the walk stores the twin");
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    assert_eq!(
        topo::pcurves::validate_pcurves(&body, band),
        vec![],
        "{lane}: the minted cap reads clean"
    );
    let (he, other) = body
        .half_edges()
        .find_map(|(he, h)| {
            let row = body.pcurve(he)?;
            let edge = body.get_edge(h.edge)?;
            let Some(topo::CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
                return None;
            };
            let face = body.face_of_half_edge(he)?;
            let sphere = body.get_surface(body.get_face(face)?.surface)?.clone();
            let Pcurve::Harmonic { p0, pa, pb, pl } = row.pcurve() else {
                return None;
            };
            let derived = geom_brep::chart_pcurve(curve.carrier(), &sphere, band).ok()?;
            let Pcurve::Harmonic { p0: d, .. } = derived else {
                return None;
            };
            let twin = (p0.x - d.x).hi() > 3.0;
            twin.then(|| {
                let flip = |w: geom_core::Vec2<T>| geom_core::Vec2::new(w.x, T::zero() - w.y);
                let other = Pcurve::Harmonic {
                    p0: geom_core::Point2::new(p0.x + T::pi(), T::pi() - p0.y),
                    pa: flip(*pa),
                    pb: flip(*pb),
                    pl: flip(*pl),
                };
                let (t0, t1) = row.params();
                let cache = PcurveCache::certify(other, t0, t1, curve.carrier(), &sphere, band)
                    .expect("the other sheet certifies: it is the same locus");
                (he, cache)
            })
        })
        .expect("the twin row");
    body.attach_pcurve(he, other);
    let findings = topo::pcurves::validate_pcurves(&body, band);
    assert!(
        findings
            .iter()
            .any(|f| matches!(f, topo::PcurveMintError::LoopDiscontinuity { .. })),
        "{lane}: an interior twin jump is a discontinuity at rest: {findings:?}"
    );
}

#[test]
fn a_turned_half_cap_at_a_tight_k_keeps_its_twin() {
    let eps = std::env::var(geom_core::tolerance::ENV_EPS)
        .ok()
        .and_then(|e| e.parse::<f64>().ok())
        .unwrap_or(geom_core::tolerance::DEFAULT_EPS);
    let k = 1.5;
    if geom_core::Tolerance::init(geom_core::Tolerance { eps, k }).is_err() {
        assert_eq!(
            Tol::witness().k(),
            k,
            "this row commits K = 1.5 for its own process; run it under nextest"
        );
    }
    tight_k::<f64>("f64", eps);
    tight_k::<Interval>("Interval", eps);
}
