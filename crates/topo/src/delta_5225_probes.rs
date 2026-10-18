//! Delta-verifier probes for PR 5225's fix pass.
#![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]

use core::f64::consts::{PI, TAU};

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Point3, Tol, Vec3};

use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::Body;

fn tol() -> Tol {
    Tol::witness()
}
fn band() -> Band {
    Band::linear(tol()).unwrap()
}
fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle { center, axis, radius, u_ref }
}
fn rows(body: &Body<f64>) -> Vec<String> {
    let mut out: Vec<String> = body
        .pcurves()
        .map(|(he, c)| format!("{he:?} {:?} {:?} {:?} {:?}", c.params(), c.pcurve(), c.certificate(), body.joint(he)))
        .collect();
    out.sort();
    out
}

fn cases(axis_sign: f64) -> Vec<(&'static str, Surface<f64>, Curve3<f64>)> {
    let (o, z, x) = (Point3::new(0.0, 0.0, 0.0), Vec3::unit_z(), Vec3::unit_x());
    let lat: f64 = 0.5;
    let a = z * axis_sign;
    vec![
        ("cylinder rim", Surface::Cylinder { origin: o, axis: z, radius: 1.0, u_ref: x }, circle(Point3::new(0.0, 0.0, 0.5), a, 1.0, x)),
        ("sphere parallel", Surface::Sphere { center: o, radius: 1.0, axis: z, u_ref: x }, circle(Point3::new(0.0, 0.0, lat.sin()), a, lat.cos(), x)),
        ("torus parallel", Surface::Torus { center: o, axis: z, major_radius: 2.0, minor_radius: 0.5, u_ref: x }, circle(Point3::new(0.0, 0.0, 0.5), a, 2.0, x)),
        // A torus meridian loop: the v channel's own whole turn.
        ("torus meridian", Surface::Torus { center: o, axis: z, major_radius: 2.0, minor_radius: 0.5, u_ref: x }, circle(Point3::new(2.0, 0.0, 0.0), -Vec3::unit_y() * axis_sign, 0.5, x)),
    ]
}

/// F1, every t: a one-edge loop split at t on a dense grid, both carrier
/// orientations, the u_ref rotated so the cut lands elsewhere.
#[test]
fn d1_one_edge_loop_split_every_t() {
    let mut bad = Vec::new();
    let mut n = 0;
    for sign in [1.0, -1.0] {
        for (name, surface, carrier0) in cases(sign) {
            for rot in [0.0, 1.0, 2.5] {
                let carrier = match carrier0.clone() {
                    Curve3::Circle { center, axis, radius, u_ref } => {
                        let v = axis.cross(u_ref);
                        circle(center, axis, radius, u_ref * f64::cos(rot) + v * f64::sin(rot))
                    }
                    _ => unreachable!(),
                };
                for i in 1..126 {
                    let t = 0.05 * f64::from(i);
                    let mut body = Body::<f64>::new();
                    let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
                    body.set_face_surface(seed.face, FaceSurface::New { surface: surface.clone(), sense: true }).unwrap();
                    let spec = EdgeCurveSpec::arc_of_circle(carrier.clone(), 0.0, TAU).unwrap();
                    let made = body.mef(MefSite::Lone { r#loop: seed.r#loop }, spec, FaceSurface::Inherit, tol()).unwrap();
                    crate::mint_pcurves(&mut body, tol()).unwrap();
                    let edge = body.get_half_edge(made.he_plus).unwrap().edge;
                    n += 1;
                    let label = format!("{name} sign {sign} rot {rot} t {t:.2}");
                    if let Err(e) = body.split_edge(edge, t, tol()) {
                        bad.push(format!("{label}: refused {e:?}"));
                        continue;
                    }
                    let v = crate::pcurves::validate_pcurves(&body, band());
                    if !v.is_empty() {
                        bad.push(format!("{label}: {v:?}"));
                        continue;
                    }
                    // and a second split of one child
                    let mut fresh = body.clone();
                    crate::mint_pcurves(&mut fresh, tol()).unwrap();
                    if rows(&body) != rows(&fresh) {
                        bad.push(format!("{label}: not the mint's rows"));
                    }
                }
            }
        }
    }
    eprintln!("d1: {n} splits, {} bad", bad.len());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// F1, two edges: a loop of two half-circles; one edge split at a dense t;
/// then split the second child again (a three/four-edge loop).
#[test]
fn d2_two_edge_loop_one_split() {
    let mut bad = Vec::new();
    let mut n = 0;
    for sign in [1.0, -1.0] {
        for (name, surface, carrier) in cases(sign) {
            for (s0, s1) in [(PI, TAU), (0.0, PI), (0.3, 0.3 + PI)] {
                for i in 1..40 {
                    let mut body = Body::<f64>::new();
                    let p0 = carrier.eval(0.0);
                    let seed = body.mvfs(p0, true).unwrap();
                    body.set_face_surface(seed.face, FaceSurface::New { surface: surface.clone(), sense: true }).unwrap();
                    // first half [0, π] via mev, second [π, 2π] via mef
                    let first = EdgeCurveSpec::arc_of_circle(carrier.clone(), 0.0, PI).unwrap();
                    let m = body.mev(MevSite::Lone { r#loop: seed.r#loop }, carrier.eval(PI), first, tol()).unwrap();
                    let second = EdgeCurveSpec::arc_of_circle(carrier.clone(), PI, TAU).unwrap();
                    body.mef(MefSite::Chords { he1: m.he_minus, he2: m.he_plus }, second, FaceSurface::Inherit, tol())
                        .map_err(|e| panic!("{name}: mef {e:?}")).unwrap();
                    if let Err(e) = crate::mint_pcurves(&mut body, tol()) {
                        bad.push(format!("{name}: mint {e:?}"));
                        break;
                    }
                    if !crate::pcurves::validate_pcurves(&body, band()).is_empty() {
                        bad.push(format!("{name}: base invalid"));
                        break;
                    }
                    // pick the edge whose carrier interval is (s0, s1)
                    let t = s0 + (s1 - s0) * f64::from(i) / 40.0;
                    let target = body.edges().map(|(e, _)| e).find(|&e| {
                        let he = body.get_edge(e).unwrap().he_plus;
                        let (a, b) = body.pcurve(he).unwrap().params();
                        a.min(b) < t && t < a.max(b)
                    });
                    let Some(target) = target else { continue };
                    n += 1;
                    let label = format!("{name} sign {sign} t {t:.3}");
                    match body.split_edge(target, t, tol()) {
                        Err(e) => { bad.push(format!("{label}: refused {e:?}")); continue; }
                        Ok(_) => {}
                    }
                    let v = crate::pcurves::validate_pcurves(&body, band());
                    if !v.is_empty() { bad.push(format!("{label}: {v:?}")); continue; }
                    let mut fresh = body.clone();
                    crate::mint_pcurves(&mut fresh, tol()).unwrap();
                    if rows(&body) != rows(&fresh) { bad.push(format!("{label}: not the mint's rows")); }
                    // split again, every edge at its midpoint
                    let edges: Vec<_> = body.edges().map(|(e, _)| e).collect();
                    for e in edges {
                        let he = body.get_edge(e).unwrap().he_plus;
                        let (a, b) = body.pcurve(he).unwrap().params();
                        if let Err(err) = body.split_edge(e, 0.5 * (a + b), tol()) {
                            bad.push(format!("{label}: resplit refused {err:?}"));
                        }
                    }
                    let v = crate::pcurves::validate_pcurves(&body, band());
                    if !v.is_empty() { bad.push(format!("{label} resplit: {v:?}")); }
                }
            }
        }
    }
    eprintln!("d2: {n} splits, {} bad", bad.len());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// F3: the sphere patch of P2 with its meridians' carriers started on
/// the far side of their great circle (u_ref negated, span moved by π):
/// the same edges, stated from the antipode.
#[test]
fn d3_sphere_patch_meridians_started_far() {
    let mut bad = Vec::new();
    for far in [false, true] {
        for (va, vb) in [(-1.4_f64, -1.1_f64), (-0.9, -0.3), (0.3, 1.3), (-0.2, 0.4)] {
            let (u0, u1) = (0.0_f64, 1.0_f64);
            let o = Point3::new(0.0, 0.0, 0.0);
            let z = Vec3::unit_z();
            let p = |u: f64, v: f64| Point3::new(v.cos() * u.cos(), v.cos() * u.sin(), v.sin());
            let rhat = |u: f64| Vec3::new(u.cos(), u.sin(), 0.0);
            let (flip, sh) = if far { (-1.0, PI) } else { (1.0, 0.0) };
            let mut body = Body::<f64>::new();
            let seed = body.mvfs(p(u0, va), true).unwrap();
            body.set_face_surface(seed.face, FaceSurface::New { surface: Surface::Sphere { center: o, radius: 1.0, axis: z, u_ref: Vec3::unit_x() }, sense: true }).unwrap();
            let bottom = EdgeCurveSpec::arc_of_circle(circle(Point3::new(0.0, 0.0, va.sin()), z, va.cos(), rhat(u0)), 0.0, u1 - u0).unwrap();
            let b = body.mev(MevSite::Lone { r#loop: seed.r#loop }, p(u1, va), bottom, tol()).unwrap();
            let right = EdgeCurveSpec::arc_of_circle(circle(o, rhat(u1).cross(z), 1.0, rhat(u1) * flip), va + sh, vb + sh).unwrap();
            assert!((right.carrier.eval(va + sh) - p(u1, va)).norm() < 1e-12);
            let r = body.mev(MevSite::Fan { he1: b.he_minus, he2: b.he_minus }, p(u1, vb), right, tol()).unwrap();
            let top = EdgeCurveSpec::arc_of_circle(circle(Point3::new(0.0, 0.0, vb.sin()), -z, vb.cos(), rhat(u1)), 0.0, u1 - u0).unwrap();
            let tp = body.mev(MevSite::Fan { he1: r.he_minus, he2: r.he_minus }, p(u0, vb), top, tol()).unwrap();
            let left = EdgeCurveSpec::arc_of_circle(circle(o, z.cross(rhat(u0)), 1.0, rhat(u0) * flip), -vb + sh, -va + sh).unwrap();
            assert!((left.carrier.eval(-vb + sh) - p(u0, vb)).norm() < 1e-12);
            body.mef(MefSite::Chords { he1: tp.he_minus, he2: b.he_plus }, left, FaceSurface::Inherit, tol()).unwrap();
            let label = format!("far {far} [{va}, {vb}]");
            if let Err(e) = crate::mint_pcurves(&mut body, tol()) { bad.push(format!("{label}: mint {e:?}")); continue; }
            for (he, c) in body.pcurves() {
                let (a, bb) = c.params();
                eprintln!("{label} {he:?} mid {:?}", c.pcurve().eval(0.5 * (a + bb)));
            }
            let v = crate::pcurves::validate_pcurves(&body, band());
            if !v.is_empty() { bad.push(format!("{label}: validate {v:?}")); }
            for (f, face) in body.faces() {
                let s = body.get_surface(face.surface).unwrap().clone();
                if let Err(e) = crate::pcurves::chart_boundary(&body, f, &s, band()) { bad.push(format!("{label} {f:?}: boundary {e:?}")); }
                let o = crate::declared_pair_overlap(&body, f, &body, f, crate::contact::ContactVerdict::Definite, band());
                if !matches!(o, Ok(crate::ChartOverlap::PositiveArea)) { bad.push(format!("{label} {f:?}: overlap {o:?}")); }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
