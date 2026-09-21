//! Reviewer r2 probes for TESS-1 (PR 2852). Measurement rows: each
//! prints what `tessellate` answers; assertions are added only where
//! the answer is a contract.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::witness_bodies::one_circle_cut;
use core::f64::consts::PI;
use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::{Body, FaceSurface, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn sphere(r: f64) -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: r,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
/// The great circle in the xz plane: eval(t) = (sin t, 0, cos t).
fn meridian0() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    }
}
/// The same great circle by latitude: eval(t) = (cos t, 0, sin t).
fn meridian_by_latitude() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
fn report(name: &str, body: &Body<f64>, delta: f64) -> String {
    let tol = Tol::witness();
    let v = topo::validate_geometric(body, tol);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(body, delta, tol)
    }));
    let line = match r {
        Err(p) => format!(
            "PANIC {}",
            p.downcast_ref::<String>().cloned().unwrap_or_default()
        ),
        Ok(Err(e)) => format!("Err({e:?})"),
        Ok(Ok(m)) => format!(
            "Ok patches={:?} check_mesh={:?}",
            m.patches
                .iter()
                .map(|p| p.triangles.len())
                .collect::<Vec<_>>(),
            mesh::validate::check_mesh(&m)
        ),
    };
    let out = format!("R2PROBE {name}: tier3={v:?} :: {line}");
    eprintln!("{out}");
    out
}

/// A rim-only sphere cap + disc with a SPUR: a meridian strut from the
/// rim's t = 0 vertex toward (not reaching) the pole, traversed twice by
/// the cap's loop. The loop now HAS meridian traversals, so the PR's
/// guard passes it; does the face still mesh as a hole?
#[test]
fn r2_cap_with_a_spur_that_stops_short_of_the_pole() {
    let z = 0.5_f64;
    let rr = (1.0 - z * z).sqrt();
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: rr,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let plane = Surface::Plane {
        origin: p3(0.0, 0.0, z),
        normal: Vec3::new(0.0, 0.0, -1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for (name, t_tip) in [
        ("short of the pole", core::f64::consts::FRAC_PI_2 - 0.3_f64),
        ("to the pole", core::f64::consts::FRAC_PI_2),
    ] {
        let mut body = one_circle_cut(&rim, sphere(1.0), Some(plane.clone()));
        let tol = Tol::witness();
        // the sphere face and a half-edge of its loop starting at rim.eval(0)
        let start = rim.eval(0.0);
        let (fk, _) = body
            .faces()
            .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
            .unwrap();
        let outer = body.get_face(fk).unwrap().outer;
        let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!()
        };
        let at = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&h| {
                let he = body.get_half_edge(h).unwrap();
                let v = body.get_vertex(he.start).unwrap();
                (*body.get_point(v.point).unwrap() - start).norm() < 1e-9
            })
            .unwrap();
        let t_rim = z.asin();
        let spec = EdgeCurveSpec::arc_of_circle(meridian_by_latitude(), t_rim, t_tip).unwrap();
        let made = body.mev(
            MevSite::Fan { he1: at, he2: at },
            meridian_by_latitude().eval(t_tip),
            spec,
            tol,
        );
        match made {
            Ok(_) => {
                report(&format!("cap + spur {name}"), &body, 0.01);
                report(&format!("cap + spur {name}, delta 0.1"), &body, 0.1);
            }
            Err(e) => eprintln!("R2PROBE cap + spur {name}: mev refused {e:?}"),
        }
    }
}

/// The one-face, one-seam sphere (V2/E1/F1): a loop of meridians only on
/// ONE column — the PR's sweep row 3, "not constructed".
#[test]
fn r2_one_seam_sphere_is_a_zero_width_band() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let start = body.mvfs(meridian0().eval(0.0)).unwrap();
    body.set_face_surface(start.face, FaceSurface::New(sphere(1.0)))
        .unwrap();
    let made = body.mev(
        MevSite::Lone {
            r#loop: start.r#loop,
        },
        meridian0().eval(PI),
        EdgeCurveSpec::arc_of_circle(meridian0(), 0.0, PI).unwrap(),
        tol,
    );
    match made {
        Ok(_) => {
            report("one-seam sphere", &body, 0.05);
        }
        Err(e) => eprintln!("R2PROBE one-seam sphere: mev refused {e:?}"),
    }
}

/// The all-meridians torus face the PR files rather than fixes
/// (`work/tess/rim-free-loop-on-a-poleless-chart-meshes-as-a-hole.md`):
/// re-measured as that row states it.
#[test]
fn r2_all_meridian_torus_face_as_filed() {
    let torus = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let minor = Curve3::Circle {
        center: p3(2.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let body = one_circle_cut(&minor, torus, None);
    report("all-meridian torus", &body, 0.05);
}
