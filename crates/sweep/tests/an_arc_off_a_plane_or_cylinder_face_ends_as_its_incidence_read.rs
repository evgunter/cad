//! **An arc set onto a plane or cylinder face it does not lie on ends
//! as the mint's incidence read, never as the residual's meter** (D3,
//! D4 ¶1 (iv)). The plane and cylinder mints read incidence before they
//! image, as the sphere, cone and torus mints do, so a definite miss is
//! `CarrierOffChart` under `ChartIncidence`: the kernel's defect at a
//! build, the stored culprit at rest and the damaged file at the import
//! door, whatever the file's coincidence distance (no ε_in is offered).
//! `ChartResidual` is left metering the image's own fidelity.
//!
//! Each row re-describes one edge of an extruded body through
//! `set_edge_curve` as a sketch arc through the edge's own endpoints:
//! - a cylinder's seam as an arc in a plane holding the axis, bowed into
//!   the solid (the meridian arm, `pcurve_chart_radial_moving` Zero);
//! - a cylinder's rim half as the half circle through the same ends,
//!   tilted off the rim's plane (the moving arm, its radial part winding
//!   once);
//! - a box's vertical edge as an arc bowed out across the corner, off
//!   both plane sides.
//!
//! Read at `f64` and at `Interval`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::recourse::Reading;
use geom_brep::{CertCheck, CertifyError, EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Bounds, Decide, FileCoincidence, Interval, Point2, Point3, Tol, Vec3};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use sweep::test_support::extruded;
use topo::construction::Construct;
use topo::{AtRestPolicy, Body, CurveGeom, EdgeKey, EulerOpError, ValidationError};

const H: f64 = 2.0;

/// The unit disc, extruded `H`.
fn cylinder<T: Decide + AtRestPolicy>() -> Body<T> {
    let f = T::from_f64;
    extruded(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(f(1.0), f(0.0)), f(1.0)),
            (Point2::new(f(-1.0), f(0.0)), f(1.0)),
        ])],
        f(H),
        Tol::witness(),
    )
}

/// The unit square, extruded `H`.
fn cube<T: Decide + AtRestPolicy>() -> Body<T> {
    let f = T::from_f64;
    let corner = |x: f64, y: f64| (Point2::new(f(x), f(y)), f(0.0));
    extruded(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            corner(0.0, 0.0),
            corner(1.0, 0.0),
            corner(1.0, 1.0),
            corner(0.0, 1.0),
        ])],
        f(H),
        Tol::witness(),
    )
}

/// The edge whose ends sit at `a` and `b` (either way round), its two
/// endpoints, and the surface of a face it bounds that `chart` picks.
fn edge_at<T: Decide + Bounds>(
    body: &Body<T>,
    a: [f64; 3],
    b: [f64; 3],
    chart: impl Fn(&Surface<T>) -> bool,
) -> (EdgeKey, Point3<T>, Point3<T>, geom_brep::SurfaceKey) {
    let near = |p: Point3<T>, q: [f64; 3]| {
        (p.x.hi() - q[0]).abs() + (p.y.hi() - q[1]).abs() + (p.z.hi() - q[2]).abs() < 1e-6
    };
    let (edge, start, end) = body
        .edges()
        .find_map(|(k, e)| {
            let c = body
                .get_curve_geom(e.curve)
                .and_then(CurveGeom::certified)?;
            let (t0, t1) = c.params();
            let (p, q) = (c.carrier().eval(t0), c.carrier().eval(t1));
            ((near(p, a) && near(q, b)) || (near(p, b) && near(q, a))).then_some((k, p, q))
        })
        .expect("the row's edge");
    let surface = body
        .faces()
        .map(|(_, f)| f.surface)
        .find(|&k| body.get_surface(k).is_some_and(&chart))
        .expect("the row's chart");
    (edge, start, end, surface)
}

/// A circle through `start` and `end` about their midpoint, in the
/// plane spanned by the chord and `bow`, bowed toward `bow` over its
/// half turn `[0, π]`.
fn arc<T: Decide>(start: Point3<T>, end: Point3<T>, bow: Vec3<T>) -> Curve3<T> {
    let two = T::from_f64(2.0);
    let chord = end - start;
    let half = chord.norm() / two;
    let u_ref = -(chord / (half * two));
    let axis = u_ref.cross(bow).normalize();
    Curve3::Circle {
        center: start + chord / two,
        axis,
        radius: half,
        u_ref,
    }
}

type Row<T> = (
    &'static str,
    Body<T>,
    EdgeKey,
    Curve3<T>,
    geom_brep::SurfaceKey,
);

fn rows<T: Decide + AtRestPolicy + Bounds>() -> Vec<Row<T>> {
    let f = T::from_f64;
    let is_cylinder = |s: &Surface<T>| matches!(s, Surface::Cylinder { .. });
    let mut v = Vec::new();

    let body = cylinder::<T>();
    let (edge, start, end, chart) = edge_at(&body, [1.0, 0.0, 0.0], [1.0, 0.0, H], is_cylinder);
    let bowed_in = arc(start, end, Vec3::new(f(-1.0), f(0.0), f(0.0)));
    v.push((
        "the seam bowed into the cylinder",
        body,
        edge,
        bowed_in,
        chart,
    ));

    let body = cylinder::<T>();
    let (edge, start, end, chart) = edge_at(&body, [1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], is_cylinder);
    let tilted = arc(start, end, Vec3::new(f(0.0), f(0.5), f(0.75_f64.sqrt())));
    v.push((
        "the rim half tilted off the cylinder",
        body,
        edge,
        tilted,
        chart,
    ));

    let body = cube::<T>();
    let side = |s: &Surface<T>| match s {
        Surface::Plane { normal, .. } => normal.x.hi().abs() > 0.5,
        _ => false,
    };
    let (edge, start, end, chart) = edge_at(&body, [1.0, 1.0, 0.0], [1.0, 1.0, H], side);
    let across = arc(start, end, Vec3::new(f(1.0), f(1.0), f(0.0)));
    v.push((
        "the box's corner bowed off both sides",
        body,
        edge,
        across,
        chart,
    ));
    v
}

fn check<T: Decide + AtRestPolicy + Bounds>() {
    let lane = T::NAME;
    for (what, mut body, edge, carrier, chart) in rows::<T>() {
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(chart),
            carrier,
            param_start: T::from_f64(0.0),
            param_end: T::from_f64(core::f64::consts::PI),
        };
        let refusal = match body.set_edge_curve(edge, spec, Tol::witness()) {
            Err(EulerOpError::Certification { error }) => error,
            other => panic!("{lane}, {what}: the mint reads the arc off its face: {other:?}"),
        };
        assert!(
            matches!(
                refusal,
                CertifyError::CarrierOffChart {
                    margin: Some(geom_brep::OffMargin::Past(_)),
                    ..
                }
            ),
            "{lane}, {what}: {refusal:?}"
        );
        assert_eq!(
            refusal.decision().map(|(check, _)| check),
            Some(CertCheck::ChartIncidence),
            "{lane}, {what}: the incidence read, never the residual's meter"
        );
        assert_eq!(
            refusal.ending(Reading::Build).as_deref(),
            Some(geom_core::KERNEL_DEFECT_ENDING),
            "{lane}, {what}: at a build"
        );
        let at_rest = ValidationError::EdgeCertification {
            edge: EdgeKey::default(),
            error: refusal,
        }
        .to_string();
        assert!(
            at_rest.contains(topo::test_support::incidence_reasons::OFF)
                && at_rest.ends_with(geom_core::KERNEL_OR_FILE_DEFECT_ENDING),
            "{lane}, {what}: at rest: {at_rest}"
        );
        // A file whose coincidence distance is under the miss and one
        // past it end alike: a definite miss is the file's damage.
        for eps_in in [1e-6, 10.0] {
            assert_eq!(
                refusal.ending(FileCoincidence::new(eps_in)).as_deref(),
                Some(geom_core::DAMAGED_FILE_ENDING),
                "{lane}, {what}: at import, ε_in = {eps_in:e}"
            );
        }
    }
}

#[test]
fn an_arc_off_a_plane_or_cylinder_face_ends_as_its_incidence_read_at_f64() {
    check::<f64>();
}

#[test]
fn an_arc_off_a_plane_or_cylinder_face_ends_as_its_incidence_read_at_interval() {
    check::<Interval>();
}
