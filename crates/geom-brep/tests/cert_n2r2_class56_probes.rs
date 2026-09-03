//! CERT-N2 R2 reviewer probes: classes 5 and 6 (edge×NURBS chart-image
//! lane and certify's plane×NURBS / iso lanes) with the S99 masquerade
//! as the DESCRIBED operand. Probe file — not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::keys::SurfaceKey;
use geom_brep::{EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, EdgeNurbsLane};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use slotmap::SlotMap;

fn wall(f: impl Fn(usize, Point3<f64>) -> Point3<f64>) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<Point3<f64>> = [
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 1.0),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, p)| f(i, p))
    .collect();
    let w = core::f64::consts::FRAC_1_SQRT_2;
    NurbsSurface::new(ku, kv, control, vec![1.0, 1.0, w, w, 1.0, 1.0]).unwrap()
}
fn segment(a: Point3<f64>, b: Point3<f64>) -> NurbsCurve3<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsCurve3::new(k, vec![a, b], vec![1.0, 1.0]).unwrap()
}
fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
fn poison_x(_: usize, p: Point3<f64>) -> Point3<f64> {
    Point3::new(f64::NAN, p.y, p.z)
}
fn poison_z(_: usize, p: Point3<f64>) -> Point3<f64> {
    Point3::new(p.x, p.y, f64::NAN)
}
fn poison_one(i: usize, p: Point3<f64>) -> Point3<f64> {
    if i == 3 {
        Point3::new(f64::NAN, p.y, p.z)
    } else {
        p
    }
}

/// Class 5 (edge_nurbs lane, `T::plane_nurbs_limbs` door).
#[test]
fn n2r2_class5_plane_nurbs_limbs_on_masquerade() {
    let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    for (name, f) in [
        (
            "x-poison",
            poison_x as fn(usize, Point3<f64>) -> Point3<f64>,
        ),
        ("z-poison", poison_z),
        ("one-point", poison_one),
    ] {
        let w = wall(f);
        assert!(!w.is_placeholder());
        let r = f64::plane_nurbs_limbs(&carrier, &plane(), &w, 1.0, band());
        eprintln!("[class 5 {name}] plane_nurbs_limbs -> {r:?}");
    }
}

/// Class 6 (certify's plane×NURBS lane and the iso/chart lane).
#[test]
fn n2r2_class6_certify_lanes_on_masquerade() {
    for (name, f) in [
        (
            "x-poison",
            poison_x as fn(usize, Point3<f64>) -> Point3<f64>,
        ),
        ("z-poison", poison_z),
        ("one-point", poison_one),
    ] {
        let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
        let s1 = arena.insert(plane());
        let s2 = arena.insert(Surface::Nurbs(std::sync::Arc::new(wall(f))));
        let carrier = Curve3::Nurbs(std::sync::Arc::new(segment(
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
        )));
        let witness = carrier.eval(0.5);
        let ends = (carrier.eval(0.0), carrier.eval(1.0));
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection { s1, s2, witness },
            carrier: carrier.clone(),
            param_start: 0.0,
            param_end: 1.0,
        };
        let r =
            EdgeCurve::certify_nurbs_lane(spec, ends.0, ends.1, |k| arena.get(k).cloned(), band());
        eprintln!(
            "[class 6 plane×NURBS {name}] certify_nurbs_lane -> {}",
            match &r {
                Ok(e) => format!("Ok(cert {:?})", e.certificate()),
                Err(e) => format!("Err({e:?})"),
            }
        );
        // The chart (iso) lane: the carrier is the wall's u=0 boundary row.
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(s2),
            carrier: carrier.clone(),
            param_start: 0.0,
            param_end: 1.0,
        };
        let r = EdgeCurve::certify(spec, ends.0, ends.1, |k| arena.get(k).cloned(), band());
        eprintln!(
            "[class 6 chart/iso {name}] certify -> {}",
            match &r {
                Ok(e) => format!("Ok(cert {:?})", e.certificate()),
                Err(e) => format!("Err({e:?})"),
            }
        );
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(s2),
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        };
        let r =
            EdgeCurve::certify_nurbs_lane(spec, ends.0, ends.1, |k| arena.get(k).cloned(), band());
        eprintln!(
            "[class 6 chart/iso via nurbs lane {name}] -> {}",
            match &r {
                Ok(e) => format!("Ok(cert {:?})", e.certificate()),
                Err(e) => format!("Err({e:?})"),
            }
        );
    }
}
