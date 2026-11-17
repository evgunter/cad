#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]
use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_z, brick, finished};
fn cut(x0: f64, theta: f64) -> Option<topo::Body<f64>> {
    let tol = Tol::witness();
    let ball = finished("ball", ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol), tol);
    let b = brick::<f64>((x0, 3.0), (-3.0, 3.0), (-3.0, 3.0), tol);
    let b = topo::transform_rigid(&b, &geom_core::test_support::rigid(&Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), theta), tol)).unwrap();
    let b = finished("box", b, tol);
    match topo::subtract(&ball, &b, tol) { Ok(r) => r.body().map(|b| topo::Body::clone(&b.body)), Err(e) => { eprintln!("ZZ boolean {e:?}"); None } }
}
#[test]
fn zz_tilted() {
    for x0 in [0.0, 0.3, 0.6] {
        for th in [2e-9, 3e-9, 5e-9, 8e-9, 2e-8, 1e-7] {
            let Some(body) = cut(x0, th) else { continue };
            let forms: Vec<_> = body.faces().filter(|(fk,_)| body.face_surface(*fk).unwrap().kind()==geom::SurfaceKind::Sphere).map(|(fk,f)| {
                let topo::LoopBoundary::Cycle{first} = body.get_loop(f.outer).unwrap().boundary else { return vec![] };
                body.loop_cycle(first).unwrap().into_iter().map(|h| body.pcurve(h).map(|c| c.pcurve().is_iso_line())).collect::<Vec<_>>()
            }).collect();
            let got = mesh::tessellate(&body, 5e-2, Tol::witness()).map(|m| m.patches.iter().map(|p| p.triangles.len()).sum::<usize>());
            eprintln!("ZZ x0={x0} th={th:e} iso={forms:?} -> {got:?}");
        }
    }
}
fn tilted_ball_cut(z0: f64, theta: f64) -> Option<topo::Body<f64>> {
    let tol = Tol::witness();
    let ball = ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol);
    let ball = topo::transform_rigid(&ball, &geom_core::test_support::rigid(&Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), theta), tol)).unwrap();
    let ball = finished("ball", ball, tol);
    let b = finished("box", brick::<f64>((-3.0, 3.0), (-3.0, 3.0), (z0, 3.0), tol), tol);
    match topo::subtract(&ball, &b, tol) { Ok(r) => r.body().map(|b| topo::Body::clone(&b.body)), Err(e) => { eprintln!("ZZ boolean {e:?}"); None } }
}
#[test]
fn zz_tilted_ball() {
    for z0 in [0.0, 0.3] {
        for th in [1e-9, 2e-9, 3e-9, 5e-9, 8e-9, 2e-8] {
            let Some(body) = tilted_ball_cut(z0, th) else { continue };
            let forms: Vec<_> = body.faces().filter(|(fk,_)| body.face_surface(*fk).unwrap().kind()==geom::SurfaceKind::Sphere).map(|(fk,f)| {
                let topo::LoopBoundary::Cycle{first} = body.get_loop(f.outer).unwrap().boundary else { return vec![] };
                body.loop_cycle(first).unwrap().into_iter().map(|h| body.pcurve(h).map(|c| c.pcurve().is_iso_line())).collect::<Vec<_>>()
            }).collect();
            let got = mesh::tessellate(&body, 5e-2, Tol::witness()).map(|m| m.patches.iter().map(|p| p.triangles.len()).sum::<usize>());
            eprintln!("ZZ ball z0={z0} th={th:e} iso={forms:?} -> {got:?}");
        }
    }
}
#[test]
fn zz_tilted_rim() {
    use geom::{Curve3, Surface};
    let tol = Tol::witness();
    for th in [1e-9f64, 2e-9, 3e-9, 5e-9, 8e-9, 1e-8, 3e-8] {
        let n = Vec3::new(0.0, th.sin(), th.cos());
        let c = Point3::new(0.0, 0.0, 0.0) + n * 0.3;
        let u = Vec3::new(1.0, 0.0, 0.0);
        let circle = Curve3::circle(c, n, (1.0f64 - 0.09).sqrt(), u);
        let sphere = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::new(0.0, 0.0, 1.0), u_ref: u };
        let plane = Surface::Plane { origin: c, normal: -n, u_ref: u };
        let Ok(body) = std::panic::catch_unwind(|| crate::common::witness_bodies::one_circle_cut(&circle, sphere, Some(plane))) else { eprintln!("ZZ rim th={th:e} build panics"); continue };
        let mut m = body.clone();
        let mint = topo::mint_pcurves(&mut m, tol);
        let iso: Vec<_> = m.pcurves().map(|(_, c)| c.pcurve().is_iso_line()).collect();
        let got = mesh::tessellate(&m, 5e-2, tol).map(|m| m.positions.len());
        eprintln!("ZZ rim th={th:e} mint={:?} iso={iso:?} -> {got:?}", mint.is_ok());
    }
}
