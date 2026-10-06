//! **A cone sector whose rulings meet at the apex**, through the Euler
//! doors: the apex vertex, two rulings out to a rim, and the rim arc
//! between them, on a narrow cone. Near the apex a ruling's points sit
//! off the chart's singular set by a margin while their distance from
//! the axis, the joint's lever, is a fraction of that.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Bounds, Point3, Real, Tol, Vec3};
use topo::{AtRestPolicy, Body, FaceSurface, MefSite, MevSite};

/// Slant of the rim from the apex.
const SL: f64 = 1.0;

/// The sector between azimuths 0 and `PI/2` on a cone with apex at the
/// origin, axis `+Z` and half-angle `alpha`, minted.
fn sector<T: Real + AtRestPolicy>(alpha: f64) -> Result<Body<T>, String> {
    let tol = Tol::witness();
    let f = T::from_f64;
    let p = |x: f64, y: f64, z: f64| Point3::new(f(x), f(y), f(z));
    let v = |x: f64, y: f64, z: f64| Vec3::new(f(x), f(y), f(z));
    let (r, h) = (SL * alpha.sin(), SL * alpha.cos());
    let apex = p(0.0, 0.0, 0.0);
    let (a, b) = (p(r, 0.0, h), p(0.0, r, h));
    let mut body = Body::<T>::new();
    let seed = body.mvfs(apex, true).map_err(|e| format!("mvfs {e:?}"))?;
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Cone {
                apex,
                axis: v(0.0, 0.0, 1.0),
                half_angle: f(alpha),
                u_ref: v(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .map_err(|e| format!("surface {e:?}"))?;
    let line =
        |from: Point3<T>, to: Point3<T>| Ok::<_, String>(EdgeCurveSpec::line_between(from, to));
    let ruling_a = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            a,
            line(apex, a)?,
            tol,
        )
        .map_err(|e| format!("mev ruling {e:?}"))?;
    let rim = Curve3::Circle {
        center: p(0.0, 0.0, h),
        axis: v(0.0, 0.0, 1.0),
        radius: f(r),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let arc = body
        .mev(
            MevSite::Fan {
                he1: ruling_a.he_minus,
                he2: ruling_a.he_minus,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim, f(0.0), T::pi() * f(0.5)).ok_or("arc")?,
            tol,
        )
        .map_err(|e| format!("mev arc {e:?}"))?;
    body.mef(
        MefSite::Chords {
            he1: arc.he_minus,
            he2: ruling_a.he_plus,
        },
        line(b, apex)?,
        FaceSurface::Inherit,
        tol,
    )
    .map_err(|e| format!("mef {e:?}"))?;
    topo::pcurves::mint_pcurves(&mut body, tol).map_err(|e| format!("mint {e:?}"))?;
    Ok(body)
}

/// The edge of `sector`'s ruling to the rim point on azimuth 0, and
/// the carrier parameter at slant `slant` from the apex.
fn ruling_at<T: Real + AtRestPolicy + Bounds>(body: &Body<T>, slant: f64) -> (topo::EdgeKey, T) {
    let (key, curve) = body
        .edges()
        .find_map(|(key, edge)| {
            let Some(topo::CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
                return None;
            };
            let Curve3::Line { .. } = curve.carrier() else {
                return None;
            };
            let end = curve.carrier().eval(curve.params().1);
            (end.y.hi().abs() < 1e-12).then_some((key, curve.clone()))
        })
        .expect("the ruling to azimuth 0");
    let (t0, t1) = curve.params();
    (key, t0 + (t1 - t0) * T::from_f64(slant / SL))
}

/// **A split whose joint the decider cannot decide refuses** (the
/// split writes no element nothing decided). On a cone of half-angle
/// 0.1 the ruling's point at slant `1.5·K·ε` is off the apex by more
/// than the band (`singular_at` reads `Off`), but its distance from the
/// axis is `0.15·K·ε`, so the half-period marks of the joint's zero gap
/// sit at `π·0.15·K·ε`, inside the band: the decision escalates, and
/// `split_edge` refuses with the body untouched. A split well clear of
/// the apex decides the identity.
fn split_near_the_apex<T: Real + AtRestPolicy + Bounds>(lane: &str) {
    let tol = Tol::witness();
    let near = 1.5 * tol.k() * tol.eps();
    let mut body = sector::<T>(0.1).unwrap_or_else(|e| panic!("{lane}: {e}"));
    let edges = body.edges().count();
    let (edge, t) = ruling_at(&body, near);
    let refused = body.split_edge(edge, t, tol);
    assert!(
        matches!(refused, Err(topo::EulerOpError::SplitJointUndecided { .. })),
        "{lane}: the split near the apex refuses at its joint: {refused:?}"
    );
    assert_eq!(
        body.edges().count(),
        edges,
        "{lane}: the refused split leaves the body as found"
    );
    let (edge, t) = ruling_at(&body, 0.5);
    let made = body
        .split_edge(edge, t, tol)
        .unwrap_or_else(|e| panic!("{lane}: the split mid-ruling: {e:?}"));
    assert_eq!(
        body.joint(made.he_plus),
        Some(topo::JointElement::IDENTITY),
        "{lane}: mid-ruling the children meet on one image"
    );
    let band = geom_core::Band::linear(tol).unwrap();
    assert_eq!(
        topo::pcurves::validate_pcurves(&body, band),
        vec![],
        "{lane}"
    );
}

#[test]
fn a_split_whose_joint_escalates_refuses() {
    split_near_the_apex::<f64>("f64");
    split_near_the_apex::<geom_core::Interval>("Interval");
}

/// The half-turn sector between slants `inner` and `SL` on the cone of
/// `sector`, minted: two rulings at azimuths 0 and `π`, the outer rim
/// arc and the inner one.
fn truncated<T: Real + AtRestPolicy>(alpha: f64, inner: f64) -> Result<Body<T>, String> {
    let tol = Tol::witness();
    let f = T::from_f64;
    let p = |x: f64, y: f64, z: f64| Point3::new(f(x), f(y), f(z));
    let v = |x: f64, y: f64, z: f64| Vec3::new(f(x), f(y), f(z));
    let (s, c) = (alpha.sin(), alpha.cos());
    let (r_in, h_in, r_out, h_out) = (inner * s, inner * c, SL * s, SL * c);
    let (a1, a2) = (p(r_in, 0.0, h_in), p(r_out, 0.0, h_out));
    let (b1, b2) = (p(-r_in, 0.0, h_in), p(-r_out, 0.0, h_out));
    let rim = |h: f64, r: f64| Curve3::Circle {
        center: p(0.0, 0.0, h),
        axis: v(0.0, 0.0, 1.0),
        radius: f(r),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let mut body = Body::<T>::new();
    let seed = body.mvfs(a1, true).map_err(|e| format!("mvfs {e:?}"))?;
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Cone {
                apex: p(0.0, 0.0, 0.0),
                axis: v(0.0, 0.0, 1.0),
                half_angle: f(alpha),
                u_ref: v(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .map_err(|e| format!("surface {e:?}"))?;
    let e1 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            a2,
            EdgeCurveSpec::line_between(a1, a2),
            tol,
        )
        .map_err(|e| format!("mev ruling {e:?}"))?;
    let e2 = body
        .mev(
            MevSite::Fan {
                he1: e1.he_minus,
                he2: e1.he_minus,
            },
            b2,
            EdgeCurveSpec::arc_of_circle(rim(h_out, r_out), f(0.0), T::pi()).ok_or("outer arc")?,
            tol,
        )
        .map_err(|e| format!("mev outer arc {e:?}"))?;
    let e3 = body
        .mev(
            MevSite::Fan {
                he1: e2.he_minus,
                he2: e2.he_minus,
            },
            b1,
            EdgeCurveSpec::line_between(b2, b1),
            tol,
        )
        .map_err(|e| format!("mev ruling {e:?}"))?;
    body.mef(
        MefSite::Chords {
            he1: e3.he_minus,
            he2: e1.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(rim(h_in, r_in), T::pi(), T::tau()).ok_or("inner arc")?,
        FaceSurface::Inherit,
        tol,
    )
    .map_err(|e| format!("mef {e:?}"))?;
    topo::pcurves::mint_pcurves(&mut body, tol).map_err(|e| format!("mint {e:?}"))?;
    Ok(body)
}

/// **Near a narrow cone's apex, the face description refuses before it
/// reads a winding it cannot trust.** On a cone of half-angle 0.1 a
/// sector's inner rim at `1.1·K·ε` from the axis (a radius the rim's own
/// certificate decides) is ten times that off the apex (`singular_at`
/// reads `Off`), and the walk decides its joints there: the half-period
/// marks of a zero gap sit at `π·1.1·K·ε`, past the band. But an eighth
/// of the period at that lever, `(π/4)·1.1·K·ε`, is not past the band's
/// escalation, so the room is not shown past the joint bound, and a mark
/// there may have named another period, and `chart_boundary` refuses
/// for want of room rather than read the loop's winding. At `3·K·ε` the
/// room is past it and the sector describes. The body reads clean at
/// rest in both.
fn near_the_apex<T: Real + AtRestPolicy>(lane: &str) {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let alpha: f64 = 0.1;
    for (lever, room) in [(1.1, false), (3.0, true)] {
        let inner = lever * tol.k() * tol.eps() / alpha.sin();
        let body = truncated::<T>(alpha, inner).unwrap_or_else(|e| panic!("{lane}: {e}"));
        assert_eq!(
            topo::pcurves::validate_pcurves(&body, band),
            vec![],
            "{lane} lever {lever}: the sector reads clean"
        );
        for (face, record) in body.faces() {
            let chart = body.get_surface(record.surface).unwrap().clone();
            let read = topo::pcurves::chart_boundary(&body, face, &chart, band);
            let refused = matches!(read, Err(topo::PcurveMintError::JointWithoutRoom { .. }));
            assert_eq!(
                refused,
                !room,
                "{lane} lever {lever}: {face:?} reads {:?}",
                read.as_ref().err()
            );
        }
    }
}

#[test]
fn near_a_narrow_apex_the_description_refuses_before_the_winding() {
    near_the_apex::<f64>("f64");
    near_the_apex::<geom_core::Interval>("Interval");
}
