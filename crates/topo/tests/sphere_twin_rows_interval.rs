//! **A row stored as the sphere's involution twin certifies at the
//! certifying scalar.** The pole-crossing half cap's meridian arc is
//! met by the loop walk through the sphere's twin `(u + π, π − v)`
//! (`topo::pcurves::sphere_twin`, the joint's element), and check 4's fidelity reads it
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

/// **Which representation the walk picks**: the half cap is two faces,
/// each bounded by a rim and the pole-crossing arc, and of its four
/// joints exactly two — one in each face's loop — carry the involution
/// twin (azimuth `π` on, polar `π − v`), at both scalars, while every
/// row is its derivation. Each loop closes through the twin, and the
/// joint that carries it lifts only through the twin: the derivation's
/// own azimuth sits on a half-period mark of a whole-period decision.
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
        assert!(
            near(s.x - d.x, 0.0) && near(s.y - d.y, 0.0),
            "{he:?}: a row is its derivation"
        );
        let element = body
            .joint(he)
            .expect("every joint of the cap stores its element");
        twins += usize::from(element.deck().twin);
    }
    twins
}

#[test]
fn the_walk_stores_one_joint_as_the_twin() {
    assert_eq!(twins(&half_cap::<f64>(0.0).unwrap()), 2, "at f64");
    assert_eq!(twins(&half_cap::<Interval>(0.0).unwrap()), 2, "at Interval");
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
/// exact azimuth, past `K·ε`. The walk still stores the same two twin
/// joints, and the body reads clean. Restating the row after that
/// joint on its OTHER sheet (its own twin) leaves the stored element
/// naming a sheet the row is no longer on, which tier 3 refuses as a
/// discontinuity.
fn tight_k<T: Real + SpanLocate + AtRestPolicy + Bounds>(lane: &str, eps: f64) {
    use geom_brep::{Pcurve, PcurveCache};
    let r = 0.75_f64.sqrt();
    let mut body = half_cap::<T>(0.99 * eps / r).unwrap_or_else(|e| panic!("{lane}: {e}"));
    assert_eq!(twins(&body), 2, "{lane}: the walk stores the twin joints");
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
            let twin = body.joint(he)?.deck().twin;
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
        .expect("the row after the twin joint");
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

/// **A split at the pole carries a reset.** The half cap's meridian arc
/// crosses the north pole at `t = π/3`; splitting it there puts the new
/// vertex on the chart's singular set, so the joint between the two
/// children is a reset, decided at the split point on the carrier
/// (`split_cache`), and tier 3, which re-decides it at the new vertex,
/// reads the body clean. Read at any other point of the carrier the
/// joint would be an ordinary identity, which tier 3 refuses.
/// The half cap with its meridian arc split at parameter `t` (the arc
/// crosses the north pole at `t = π/3`).
fn split_meridian<T: Real + SpanLocate + AtRestPolicy + Bounds>(
    lane: &str,
    t: f64,
) -> (Body<T>, topo::SplitEdgeCreated) {
    let mut body = half_cap::<T>(0.0).unwrap_or_else(|e| panic!("{lane}: {e}"));
    let meridian = body
        .edges()
        .find_map(|(key, edge)| {
            let Some(topo::CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
                return None;
            };
            matches!(curve.carrier(), Curve3::Circle { center, .. } if center.z.hi().abs() < 1e-12)
                .then_some(key)
        })
        .expect("the half cap's meridian arc");
    let made = body
        .split_edge(meridian, T::from_f64(t), Tol::witness())
        .unwrap_or_else(|e| panic!("{lane}: the split at t = {t}: {e:?}"));
    (body, made)
}

fn split_at_the_pole<T: Real + SpanLocate + AtRestPolicy + Bounds>(lane: &str) {
    let (body, made) = split_meridian::<T>(lane, core::f64::consts::FRAC_PI_3);
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let at_the_pole: Vec<_> = [made.he_plus, made.he_minus]
        .into_iter()
        .filter(|&he| body.get_half_edge(he).unwrap().start == made.vertex)
        .collect();
    assert_eq!(
        at_the_pole.len(),
        1,
        "{lane}: one child enters at the new vertex"
    );
    for he in at_the_pole {
        let element = body.joint(he).expect("the split writes each child's joint");
        assert!(
            element.is_reset(),
            "{lane}: the joint at the pole is a reset: {element:?}"
        );
    }
    assert_eq!(
        topo::pcurves::validate_pcurves(&body, band),
        vec![],
        "{lane}: the split cap reads clean"
    );
}

/// **A strut from the pole reads its joints at its own ends.** On the
/// half cap split at the pole, `mev` adds a strut from the pole vertex
/// down the meridian at azimuth `π/2`, and the site mint decides the
/// joints of its two new halves at the vertices they enter at: the half
/// leaving the pole is a reset there (every azimuth names the pole), and
/// the turn at the strut's tip is ordinary. The new vertex does not exist
/// before the surgery, so the mint reads each new half's entry off its
/// carrier, at the end the half starts from.
fn strut_from_the_pole<T: Real + SpanLocate + AtRestPolicy + Bounds>(lane: &str) {
    let tol = Tol::witness();
    let f = T::from_f64;
    let (mut body, made) = split_meridian::<T>(lane, core::f64::consts::FRAC_PI_3);
    let theta = 0.8_f64.acos();
    let side = Curve3::Circle {
        center: Point3::new(f(0.0), f(0.0), f(0.0)),
        axis: Vec3::new(f(-1.0), f(0.0), f(0.0)),
        radius: f(1.0),
        u_ref: Vec3::new(f(0.0), f(0.0), f(1.0)),
    };
    let tip = Point3::new(f(0.0), f(theta.sin()), f(theta.cos()));
    let strut = body
        .mev(
            MevSite::Fan {
                he1: made.he_plus,
                he2: made.he_plus,
            },
            tip,
            EdgeCurveSpec::arc_of_circle(side, f(0.0), f(theta)).unwrap(),
            tol,
        )
        .unwrap_or_else(|e| panic!("{lane}: the strut: {e:?}"));
    let leaving = body
        .joint(strut.he_plus)
        .expect("the site mint writes the strut's joints");
    let turning = body
        .joint(strut.he_minus)
        .expect("the site mint writes the strut's joints");
    assert!(
        leaving.is_reset(),
        "{lane}: the strut leaves the pole on a reset: {leaving:?}"
    );
    assert_eq!(
        turning,
        topo::JointElement::IDENTITY,
        "{lane}: the turn at the strut's tip is ordinary"
    );
    let band = geom_core::Band::linear(tol).unwrap();
    assert_eq!(
        topo::pcurves::validate_pcurves(&body, band),
        vec![],
        "{lane}: the strut's rows read clean"
    );
}

#[test]
fn a_strut_from_the_pole_reads_its_joints_at_its_own_ends() {
    strut_from_the_pole::<f64>("f64");
    strut_from_the_pole::<Interval>("Interval");
}

#[test]
fn a_split_at_the_pole_carries_a_reset() {
    split_at_the_pole::<f64>("f64");
    split_at_the_pole::<Interval>("Interval");
}

/// **Near a pole, the face description refuses before it reads a
/// winding it cannot trust.** The half cap's meridian split `c·K·ε` off
/// the pole puts a joint there, and `chart_boundary` reads each face's
/// loops at the split body:
/// - `c = 0.5`: within the band of the pole (`singular_at` undecided),
///   refused as a singular joint, never let through to the winding;
/// - `c = 2`: off the pole, but the quarter-period room at the vertex's
///   lever, `(π/2)·2·K·ε`, is not past four times the band, so a mark
///   there may have named the other sheet: refused for want of room;
/// - `c = 4`: the room is past it, so the joint's integer is the joint's
///   own and the description reads the winding, which goes through the
///   twin (`LoopWraps`).
///
/// The body itself reads clean at rest in all three.
fn near_the_pole<T: Real + SpanLocate + AtRestPolicy + Bounds>(lane: &str) {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let k_eps = tol.k() * tol.eps();
    for (c, want) in [(0.5, "singular"), (2.0, "room"), (4.0, "wraps")] {
        let (body, _) = split_meridian::<T>(lane, core::f64::consts::FRAC_PI_3 + c * k_eps);
        assert_eq!(
            topo::pcurves::validate_pcurves(&body, band),
            vec![],
            "{lane} c = {c}: the split cap reads clean"
        );
        for (face, record) in body.faces() {
            let chart = body.get_surface(record.surface).unwrap().clone();
            let read = topo::pcurves::chart_boundary(&body, face, &chart, band);
            let got = match read {
                Err(topo::PcurveMintError::SingularChartJoint { .. }) => "singular",
                Err(topo::PcurveMintError::JointWithoutRoom { .. }) => "room",
                Err(topo::PcurveMintError::LoopWraps { .. }) => "wraps",
                other => panic!("{lane} c = {c}: {face:?} reads {other:?}"),
            };
            assert_eq!(got, want, "{lane} c = {c}: {face:?}");
        }
    }
}

#[test]
fn near_a_pole_the_description_refuses_before_the_winding() {
    near_the_pole::<f64>("f64");
    near_the_pole::<Interval>("Interval");
}
