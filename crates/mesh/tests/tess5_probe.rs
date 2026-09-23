//! TESS-5 review probes (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common;
use common::witness_bodies::one_circle_cut;
use core::f64::consts::PI;
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, SurfaceKind};
use geom_core::{Point3, Tol, Vec3};
use topo::{Body, FaceSurface, MefSite, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
fn meridian_from_the_pole() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    }
}

/// Two DISTINCT edges on one carrier, pole to pole: a slit bounded by
/// coincident edges. Two openings (both poles), two edge keys.
#[test]
fn probe_coincident_edge_sphere_slit() {
    let tol = Tol::witness();
    let seam = meridian_from_the_pole();
    let mut body = Body::<f64>::new();
    let start = body.mvfs(seam.eval(0.0)).unwrap();
    body.set_face_surface(start.face, FaceSurface::New(unit_sphere()))
        .unwrap();
    let m = body
        .mev(
            MevSite::Lone {
                r#loop: start.r#loop,
            },
            seam.eval(PI),
            EdgeCurveSpec::arc_of_circle(seam.clone(), 0.0, PI).unwrap(),
            tol,
        )
        .unwrap();
    eprintln!("after mev: {:?}", topo::readback::euler_counts(&body));
    // Second edge, same carrier, same endpoints: mef between the two vertices in the one loop.
    let r = body.mef(
        MefSite::Chords {
            he1: m.he_plus,
            he2: m.he_minus,
        },
        EdgeCurveSpec::arc_of_circle(seam.clone(), 0.0, PI).unwrap(),
        FaceSurface::Inherit,
        tol,
    );
    eprintln!(
        "mef: {:?}",
        r.as_ref().map(|_| ()).map_err(|e| format!("{e:?}"))
    );
    let Ok(_) = r else {
        return;
    };
    eprintln!("counts: {:?}", topo::readback::euler_counts(&body));
    eprintln!("tier3: {:?}", topo::validate_geometric(&body, tol));
    let got = mesh::tessellate(&body, 0.05, tol);
    match &got {
        Ok(m) => eprintln!(
            "tessellate Ok, patches {:?}, check_mesh {:?}",
            m.patches
                .iter()
                .map(|p| p.triangles.len())
                .collect::<Vec<_>>(),
            mesh::validate::check_mesh(m)
        ),
        Err(e) => eprintln!("tessellate Err {e:?}"),
    }
    let _ = SurfaceKind::Sphere;
    let _ = one_circle_cut;
}

/// Cylinder: two distinct coincident generators (a slit). No pole, so
/// both junctions are continuations: one opening.
#[test]
fn probe_coincident_edge_cylinder_slit() {
    let tol = Tol::witness();
    let cyl = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let mut body = Body::<f64>::new();
    let start = body.mvfs(p3(1.0, 0.0, 0.0)).unwrap();
    body.set_face_surface(start.face, FaceSurface::New(cyl))
        .unwrap();
    let m = body
        .mev(
            MevSite::Lone {
                r#loop: start.r#loop,
            },
            p3(1.0, 0.0, 1.0),
            EdgeCurveSpec::line_between(p3(1.0, 0.0, 0.0), p3(1.0, 0.0, 1.0)),
            tol,
        )
        .unwrap();
    let r = body.mef(
        MefSite::Chords {
            he1: m.he_plus,
            he2: m.he_minus,
        },
        EdgeCurveSpec::line_between(p3(1.0, 0.0, 0.0), p3(1.0, 0.0, 1.0)),
        FaceSurface::Inherit,
        tol,
    );
    eprintln!(
        "mef: {:?}",
        r.as_ref().map(|_| ()).map_err(|e| format!("{e:?}"))
    );
    let Ok(_) = r else {
        return;
    };
    let got = mesh::tessellate(&body, 0.05, tol);
    match &got {
        Ok(m) => eprintln!(
            "tessellate Ok, patches {:?}",
            m.patches
                .iter()
                .map(|p| p.triangles.len())
                .collect::<Vec<_>>()
        ),
        Err(e) => eprintln!("tessellate Err {e:?}"),
    }
}
