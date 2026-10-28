//! Review probe, PR 5889: a stored row whose carrier is off its face at
//! the READING's ε, read by the real at-rest door (`validate_geometric`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Bounds, Decide, Interval, Point2, Point3, Tol, Vec3};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use sweep::test_support::extruded;
use topo::construction::Construct;
use topo::{AtRestPolicy, Body, CurveGeom};

const H: f64 = 2.0;
fn cyl<T: Decide + AtRestPolicy>(tol: Tol) -> Body<T> {
    let f = T::from_f64;
    extruded(SketchPlane::xy(), vec![bulge_loop(vec![(Point2::new(f(1.0), f(0.0)), f(1.0)), (Point2::new(f(-1.0), f(0.0)), f(1.0))])], f(H), tol)
}
fn cube<T: Decide + AtRestPolicy>(tol: Tol) -> Body<T> {
    let f = T::from_f64;
    let c = |x: f64, y: f64| (Point2::new(f(x), f(y)), f(0.0));
    extruded(SketchPlane::xy(), vec![bulge_loop(vec![c(0.0, 0.0), c(1.0, 0.0), c(1.0, 1.0), c(0.0, 1.0)])], f(H), tol)
}
/// The circle through start/end with sagitta `s` toward `bow` (unit), over [0, π]... as a
/// big circle: radius R from half-chord c: R = (c² + s²)/(2s), arc over a small angle.
fn bowed<T: Decide>(start: Point3<T>, end: Point3<T>, bow: Vec3<T>, s: f64) -> (Curve3<T>, T, T) {
    let f = T::from_f64;
    let chord = end - start;
    let c = chord.norm() / f(2.0);
    let r = (c * c + f(s * s)) / f(2.0 * s);
    let mid = start + chord / f(2.0);
    let center = mid - bow * (r - f(s));
    let u_ref = (start - center) / r;
    let w = (end - center) / r;
    let axis = u_ref.cross(w).normalize();
    let ang = (u_ref.dot(w)).acos();
    (Curve3::Circle { center, axis, radius: r, u_ref }, f(0.0), ang)
}
fn edge_at<T: Decide + Bounds>(body: &Body<T>, a: [f64; 3], b: [f64; 3], chart: impl Fn(&Surface<T>) -> bool) -> (topo::EdgeKey, Point3<T>, Point3<T>, geom_brep::SurfaceKey) {
    let near = |p: Point3<T>, q: [f64; 3]| (p.x.hi() - q[0]).abs() + (p.y.hi() - q[1]).abs() + (p.z.hi() - q[2]).abs() < 1e-6;
    let (edge, start, end) = body.edges().find_map(|(k, e)| {
        let c = body.get_curve_geom(e.curve).and_then(CurveGeom::certified)?;
        let (t0, t1) = c.params();
        let (p, q) = (c.carrier().eval(t0), c.carrier().eval(t1));
        if near(p, a) && near(q, b) { Some((k, p, q)) } else if near(p, b) && near(q, a) { Some((k, q, p)) } else { None }
    }).expect("edge");
    let surface = body.faces().map(|(_, f)| f.surface).find(|&k| body.get_surface(k).is_some_and(&chart)).expect("chart");
    (edge, start, end, surface)
}

fn run<T: Decide + AtRestPolicy + geom_core::CertifiedBounds + Bounds>(sag: f64) {
    let f = T::from_f64;
    let build = Tol::with_eps(1e-6);
    let read = Tol::with_eps(1e-9);
    for what in ["cube corner bowed out", "cylinder seam bowed out"] {
        let mut body = if what.starts_with("cube") { cube::<T>(build) } else { cyl::<T>(build) };
        let (edge, start, end, chart, bow) = if what.starts_with("cube") {
            let side = |s: &Surface<T>| matches!(s, Surface::Plane { normal, .. } if normal.x.hi().abs() > 0.5);
            let (e, a, b, k) = edge_at(&body, [1.0, 1.0, 0.0], [1.0, 1.0, H], side);
            (e, a, b, k, Vec3::new(f(1.0), f(1.0), f(0.0)).normalize())
        } else {
            let wall = |s: &Surface<T>| matches!(s, Surface::Cylinder { .. });
            let (e, a, b, k) = edge_at(&body, [1.0, 0.0, 0.0], [1.0, 0.0, H], wall);
            (e, a, b, k, Vec3::new(f(1.0), f(0.0), f(0.0)))
        };
        let (carrier, t0, t1) = bowed(start, end, bow, sag);
        let spec = EdgeCurveSpec { description: EdgeDescriptionSpec::chart(chart), carrier, param_start: t0, param_end: t1 };
        let set = body.set_edge_curve(edge, spec, build);
        eprintln!("REST {} {what} sag={sag:e}: set at 1e-6: {:?}", T::NAME, set.as_ref().map(|_| ()).map_err(|e| e.to_string().chars().take(200).collect::<String>()));
        if set.is_err() { continue; }
        eprintln!("REST {} {what} sag={sag:e}: validate at 1e-6: {:?}", T::NAME, topo::validate_geometric(&body, build).map_err(|e| e.len()));
        match topo::validate_geometric(&body, read) {
            Ok(()) => eprintln!("REST {} {what} sag={sag:e}: VALID at 1e-9", T::NAME),
            Err(errs) => {
                for e in errs.iter().filter(|e| matches!(e, topo::ValidationError::EdgeCertification { .. })).take(3) {
                    if let topo::ValidationError::EdgeCertification { error, .. } = e {
                        eprintln!("REST {} {what} sag={sag:e} at 1e-9: EDGE {:?} decision={:?}\n   || {}", T::NAME, std::mem::discriminant(error), error.decision(), e.to_string().chars().take(500).collect::<String>());
                    }
                }
                eprintln!("REST {} {what} sag={sag:e} at 1e-9: {} errors total; kinds: {:?}", T::NAME, errs.len(), errs.iter().map(|e| e.to_string().chars().take(70).collect::<String>()).collect::<Vec<_>>());
            }
        }
    }
}

#[test]
fn probe_rest_door() {
    // 5e-7: definitely off at 1e-9 (within 1e-6). 4e-9: in band at 1e-9.
    for sag in [5e-7, 4e-9] {
        run::<f64>(sag);
        run::<Interval>(sag);
    }
}
