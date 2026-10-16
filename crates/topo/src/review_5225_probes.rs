//! Reviewer probes for PR 5225 (B1, placement over the span).
#![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]

use core::f64::consts::TAU;

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, Pcurve};
use geom_core::{Band, Point3, Tol, Vec3};

use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}
fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn deep(body: &Body<f64>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (k, c) in body.pcurves() {
        out.push(format!(
            "R {k:?} {:?} {:?} {:?} {:?}",
            c.params(),
            c.pcurve(),
            c.certificate(),
            body.joint(k)
        ));
    }
    out.sort();
    out
}

fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle { center, axis, radius, u_ref }
}

/// A sheet on `surface` bounded by ONE self-loop circle edge over [0, τ].
fn self_loop(surface: Surface<f64>, carrier: Curve3<f64>) -> (Body<f64>, EdgeKey) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New { surface, sense: true })
        .unwrap();
    let spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, TAU).unwrap();
    let made = body
        .mef(MefSite::Lone { r#loop: seed.r#loop }, spec, FaceSurface::Inherit, tol())
        .unwrap();
    crate::mint_pcurves(&mut body, tol()).unwrap();
    let edge = body.get_half_edge(made.he_plus).unwrap().edge;
    (body, edge)
}

fn report(name: &str, body: &mut Body<f64>) -> (Vec<String>, usize) {
    let v = crate::pcurves::validate_pcurves(body, band());
    let carried = deep(body);
    crate::mint_pcurves(body, tol()).unwrap();
    let minted = deep(body);
    let moved = carried.iter().zip(&minted).filter(|(a, b)| a != b).count();
    eprintln!("{name}: validate {v:?}");
    for (a, b) in carried.iter().zip(&minted) {
        if a != b {
            eprintln!("  carried {a}\n  minted  {b}");
        }
    }
    (v.iter().map(|e| format!("{e:?}")).collect(), moved)
}

/// P1: a one-edge circle loop on a curved chart, split near its end, so
/// the second child is placed a turn off the first. The self-joint
/// (second child's exit into the first child's entry) is the parent's
/// old element, kept because the neighbour is one of the edge's own
/// halves.
#[test]
fn p1_self_loop_split_near_its_end() {
    let z = Vec3::unit_z();
    let x = Vec3::unit_x();
    let o = Point3::new(0.0, 0.0, 0.0);
    let lat: f64 = 0.5;
    let cases: Vec<(&str, Surface<f64>, Curve3<f64>)> = vec![
        (
            "cylinder rim",
            Surface::Cylinder { origin: o, axis: z, radius: 1.0, u_ref: x },
            circle(Point3::new(0.0, 0.0, 0.5), z, 1.0, x),
        ),
        (
            "sphere parallel",
            Surface::Sphere { center: o, radius: 1.0, axis: z, u_ref: x },
            circle(Point3::new(0.0, 0.0, lat.sin()), z, lat.cos(), x),
        ),
        (
            "torus parallel",
            Surface::Torus { center: o, axis: z, major_radius: 2.0, minor_radius: 0.5, u_ref: x },
            circle(Point3::new(0.0, 0.0, 0.5), z, 2.0, x),
        ),
    ];
    let mut bad = Vec::new();
    for (name, s, c) in cases {
        for t in [1.0, 3.0, 5.0, 5.5, 6.0] {
            let (mut body, edge) = self_loop(s.clone(), c.clone());
            match body.split_edge(edge, t, tol()) {
                Ok(_) => {
                    let (v, moved) = report(&format!("{name} @ {t}"), &mut body);
                    if !v.is_empty() || moved != 0 {
                        bad.push(format!("{name} @ {t}: validate {v:?}, moved {moved}"));
                    }
                }
                Err(e) => bad.push(format!("{name} @ {t}: split refused {e:?}")),
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// P2: a sphere patch below latitude −1 rad bounded by two meridians and
/// two parallels: the meridians' polar channel is placed in (−1, τ−1].
#[test]
fn p2_sphere_patch_below_the_cut_latitude() {
    for (va, vb) in [(-1.4_f64, -1.1_f64), (-0.9, -0.3), (0.3, 1.3)] {
        let (u0, u1) = (0.0_f64, 1.0_f64);
        let o = Point3::new(0.0, 0.0, 0.0);
        let z = Vec3::unit_z();
        let p = |u: f64, v: f64| Point3::new(v.cos() * u.cos(), v.cos() * u.sin(), v.sin());
        let rhat = |u: f64| Vec3::new(u.cos(), u.sin(), 0.0);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(u0, va), true).unwrap();
        body.set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Sphere { center: o, radius: 1.0, axis: z, u_ref: Vec3::unit_x() },
                sense: true,
            },
        )
        .unwrap();
        // bottom parallel u0 -> u1 at va
        let bottom = EdgeCurveSpec::arc_of_circle(
            circle(Point3::new(0.0, 0.0, va.sin()), z, va.cos(), rhat(u0)),
            0.0,
            u1 - u0,
        )
        .unwrap();
        let b = body
            .mev(MevSite::Lone { r#loop: seed.r#loop }, p(u1, va), bottom, tol())
            .unwrap();
        // right meridian at u1 from va up to vb
        let ax = rhat(u1).cross(z);
        let right = EdgeCurveSpec::arc_of_circle(circle(o, ax, 1.0, rhat(u1)), va, vb).unwrap();
        let r = body
            .mev(MevSite::Fan { he1: b.he_minus, he2: b.he_minus }, p(u1, vb), right, tol())
            .unwrap();
        // top parallel from u1 back to u0 at vb
        let top = EdgeCurveSpec::arc_of_circle(
            circle(Point3::new(0.0, 0.0, vb.sin()), -z, vb.cos(), rhat(u1)),
            0.0,
            u1 - u0,
        )
        .unwrap();
        let tp = body
            .mev(MevSite::Fan { he1: r.he_minus, he2: r.he_minus }, p(u0, vb), top, tol())
            .unwrap();
        // left meridian at u0 from vb down to va: axis y-ish so axis × r̂ = −z
        let axl = z.cross(rhat(u0));
        let left = EdgeCurveSpec::arc_of_circle(circle(o, axl, 1.0, rhat(u0)), -vb, -va).unwrap();
        assert!((left.carrier.eval(-vb) - p(u0, vb)).norm() < 1e-12);
        body.mef(
            MefSite::Chords { he1: tp.he_minus, he2: b.he_plus },
            left,
            FaceSurface::Inherit,
            tol(),
        )
        .unwrap();
        let minted = crate::mint_pcurves(&mut body, tol());
        eprintln!("[{va}, {vb}] mint: {minted:?}");
        for (he, c) in body.pcurves() {
            let (a, bb) = c.params();
            let m = 0.5 * (a + bb);
            if let Pcurve::Harmonic { .. } = c.pcurve() {
                eprintln!(
                    "  {he:?} [{a:.3},{bb:.3}] entry {:?} mid {:?} joint {:?}",
                    c.pcurve().eval(a),
                    c.pcurve().eval(m),
                    body.joint(he)
                );
            }
        }
        let v = crate::pcurves::validate_pcurves(&body, band());
        eprintln!("  validate {v:?}");
        for (f, face) in body.faces() {
            let s = body.get_surface(face.surface).unwrap().clone();
            let bd = crate::pcurves::chart_boundary(&body, f, &s, band());
            match bd {
                Ok(bd) => eprintln!(
                    "  {f:?} boundary corners {:?}",
                    bd.loops
                        .iter()
                        .flat_map(|l| &l.edges)
                        .map(|e| (e.a().x, e.a().y))
                        .collect::<Vec<_>>()
                ),
                Err(e) => eprintln!("  {f:?} boundary ERR {e:?}"),
            }
        }
        for (f, _) in body.faces() {
            let o = crate::declared_pair_overlap(
                &body, f, &body, f, crate::contact::ContactVerdict::Definite, band());
            eprintln!("  {f:?} self-overlap {o:?}");
        }
        assert!(v.is_empty(), "[{va}, {vb}]: validate {v:?}");
    }
}
