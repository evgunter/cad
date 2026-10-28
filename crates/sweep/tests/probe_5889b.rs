//! Review probe (PR 5889, lane b): a stored body whose face chart is
//! moved off its edges, validated at rest through the real door, at f64
//! and Interval, definitely off and mid-band.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom::Surface;
use geom_core::{Bounds, Decide, Interval, Point2, Point3, Tol, Vec3};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use sweep::test_support::extruded;
use topo::construction::{Construct, FaceSurface};
use topo::{AtRestBody, AtRestPolicy, Body};

fn cylinder<T: Decide + AtRestPolicy>() -> Body<T> {
    let f = T::from_f64;
    extruded(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(f(1.0), f(0.0)), f(1.0)),
            (Point2::new(f(-1.0), f(0.0)), f(1.0)),
        ])],
        f(2.0),
        Tol::witness(),
    )
}
fn cube<T: Decide + AtRestPolicy>() -> Body<T> {
    let f = T::from_f64;
    let c = |x: f64, y: f64| (Point2::new(f(x), f(y)), f(0.0));
    extruded(
        SketchPlane::xy(),
        vec![bulge_loop(vec![c(0.0, 0.0), c(1.0, 0.0), c(1.0, 1.0), c(0.0, 1.0)])],
        f(2.0),
        Tol::witness(),
    )
}

fn at_rest<T: Decide + AtRestPolicy + Bounds + geom_core::CertifiedBounds>(
    mut body: Body<T>,
    pick: impl Fn(&Surface<T>) -> bool,
    moved: impl Fn(&Surface<T>) -> Surface<T>,
) -> String {
    let (fk, s) = body
        .faces()
        .map(|(k, f)| (k, f.surface))
        .find(|&(_, s)| body.get_surface(s).is_some_and(&pick))
        .expect("face");
    let new = moved(body.get_surface(s).unwrap());
    match body.set_face_surface_unvouched_for_tests(fk, FaceSurface::New { surface: new, sense: true }) {
        Ok(_) => {}
        Err(e) => return format!("RECHART REFUSED: {e}"),
    }
    match AtRestBody::validate(body, Tol::witness()) {
        Ok(_) => "VALIDATED".into(),
        Err(v) => v.iter().map(|e| format!("{e:?}\n  => {e}")).collect::<Vec<_>>().join("\n"),
    }
}

fn run<T: Decide + AtRestPolicy + Bounds + geom_core::CertifiedBounds>() {
    let eps = Tol::witness().eps();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let mid = 0.5 * (band.zero() + band.escalate());
    for (what, d) in [("0.5 m", 0.5), ("mid-band", mid)] {
        let cyl = at_rest(
            cylinder::<T>(),
            |s| matches!(s, Surface::Cylinder { .. }),
            |s| match s.clone() {
                Surface::Cylinder { origin, axis, radius, u_ref } => Surface::Cylinder {
                    origin,
                    axis,
                    radius: radius + T::from_f64(d),
                    u_ref,
                },
                _ => unreachable!(),
            },
        );
        println!("=== {} eps={eps:e} cylinder radius +{what} ({d:e}):\n{cyl}\n", T::NAME);
        let side = at_rest(
            cube::<T>(),
            |s| matches!(s, Surface::Plane { normal, .. } if normal.x.hi() > 0.5),
            |s| match s.clone() {
                Surface::Plane { origin, normal, u_ref } => Surface::Plane {
                    origin: origin + Vec3::new(T::from_f64(d), T::zero(), T::zero()),
                    normal,
                    u_ref,
                },
                _ => unreachable!(),
            },
        );
        println!("=== {} eps={eps:e} box +x side moved {what} ({d:e}):\n{side}\n", T::NAME);
    }
    let _ = Point3::<T>::origin();
}

#[test]
fn probe_5889b_at_rest_f64() {
    run::<f64>();
}
#[test]
fn probe_5889b_at_rest_interval() {
    run::<Interval>();
}
