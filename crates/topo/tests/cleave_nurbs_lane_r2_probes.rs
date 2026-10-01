//! Reviewer probes (CLEAVE, PR 3678's dual review, lane r2): the plane
//! x NURBS lane read off the scalar's policy, through public doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Affine3, Point3, Tol, Vec3};
use topo::{Body, transform_rigid, validate_closed};

/// `m4_pr2_transform`'s wall, at any scalar (built at f64, lifted).
fn nurbs_wall<T: geom_core::Real>() -> geom::Surface<T> {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let ticks = [-1.0, 0.5, 2.0];
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for &x in &ticks {
        for &z in &ticks {
            control.push(Point3::new(x, 0.0, z));
            weights.push(1.0);
        }
    }
    let n = geom::NurbsSurface::new(k.clone(), k, control, weights).unwrap();
    geom::Surface::Nurbs(std::sync::Arc::new(n)).map_scalar(T::from_f64)
}

/// `m4_pr2_transform::m7_8_cube` at any scalar holding the mint door.
/// Coordinates are read off an `f64` twin built by the same op
/// sequence (same keys), since a body has no scalar lift.
fn m7_8_cube<T>() -> Body<T>
where
    T: geom_core::Decide + geom_core::CertifiedBounds,
{
    let twin = common::geometric_cube::<f64>(Tol::witness()).body;
    let cube = common::geometric_cube::<T>(Tol::witness());
    let mut body = cube.body;
    let wall = body
        .set_face_surface(
            cube.mefs[1].face,
            topo::FaceSurface::New {
                surface: nurbs_wall::<T>(),
                sense: true,
            },
        )
        .unwrap();
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    let mut lane_edges = 0;
    for (edge_key, edge) in edges {
        let s1 = common::face_surface_of_he(&body, edge.he_plus);
        let s2 = common::face_surface_of_he(&body, edge.he_minus);
        if s1 != wall && s2 != wall {
            continue;
        }
        let start = body.get_half_edge(edge.he_plus).unwrap().start;
        let end = body.half_edge_end(edge.he_plus).unwrap();
        let p0 = *twin
            .get_point(twin.get_vertex(start).unwrap().point)
            .unwrap();
        let p1 = *twin.get_point(twin.get_vertex(end).unwrap().point).unwrap();
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
        ))
        .map_scalar(T::from_f64);
        body.set_edge_curve_nurbs_lane(
            edge_key,
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    s1,
                    s2,
                    witness: p0.lerp(p1, 0.5).map(T::from_f64),
                },
                carrier,
                param_start: T::zero(),
                param_end: T::one(),
            },
            Tol::witness(),
        )
        .unwrap();
        lane_edges += 1;
    }
    assert_eq!(lane_edges, 4);
    body
}

fn edge_findings(body: &Body<f64>) -> usize {
    match topo::validate_pseudomanifold(body, &topo::ContactRecords::default(), Tol::witness()) {
        Ok(()) => 0,
        Err(errs) => errs
            .iter()
            .filter(|e| matches!(e, topo::ValidationError::EdgeCertification { .. }))
            .count(),
    }
}

fn carried(cavity: &Body<f64>) -> topo::VoidEvidence {
    topo::VoidEvidence {
        shells: cavity
            .shells()
            .map(|(s, _)| {
                (
                    s,
                    topo::VoidContainment::Carried {
                        sign: geom_core::Sign::Positive,
                    },
                )
            })
            .collect(),
    }
}

/// A non-dyadic rotation about a skew axis, then its inverse: both
/// moves certify at f64 and the at-rest check 2 stays clean.
#[test]
fn rr2_m7_8_cube_rotates_about_a_skew_axis_and_back() {
    let body = m7_8_cube::<f64>();
    let map = Affine3::rotation_about_axis(
        Point3::new(0.3, -0.7, 0.1),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    );
    let moved = transform_rigid(&body, &map, Tol::witness()).expect("rotates");
    assert_eq!(edge_findings(&moved), 0);
    let back = transform_rigid(&moved, &map.inverse(), Tol::witness()).expect("rotates back");
    assert_eq!(edge_findings(&back), 0);
    for (k, p0) in body.points() {
        let p1 = back.get_point(k).unwrap();
        assert!((p1.x - p0.x).abs() < 1e-12 && (p1.y - p0.y).abs() < 1e-12);
    }
}

/// A mirror is refused by the rigidity door, before any certificate:
/// the lane does not open a reflection.
#[test]
fn rr2_m7_8_cube_mirror_refuses_not_rigid() {
    let body = m7_8_cube::<f64>();
    let mirror = geom_core::linalg::frame::mirror_across_plane::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Tol::witness(),
    )
    .unwrap();
    match transform_rigid(&body, &mirror, Tol::witness()) {
        Err(topo::TransformError::NotRigid { .. }) => {}
        other => panic!("mirror: {other:?}"),
    }
}

/// The class moves at the interval scalar too: its policy holds the
/// lane, end to end through `transform_rigid`.
#[test]
fn rr2_m7_8_cube_moves_at_interval() {
    use geom_core::interval::Interval;
    let body = m7_8_cube::<Interval>();
    let map = Affine3::translation(Vec3::new(0.25, -1.5, 8.0))
        .map(<Interval as geom_core::Real>::from_f64);
    transform_rigid(&body, &map, Tol::witness()).expect("moves at Interval");
}

/// The class moves at `Sym<f64>`, whose arm mirrors `fitted_lane`'s.
#[test]
fn rr2_m7_8_cube_moves_at_sym() {
    use geom_core::Sym;
    let body = m7_8_cube::<Sym<f64>>();
    let map = Affine3::translation(Vec3::new(0.25, -1.5, 8.0))
        .map(<Sym<f64> as geom_core::Real>::from_f64);
    transform_rigid(&body, &map, Tol::witness()).expect("moves at Sym<f64>");
}

/// A plain (no M7-8) cavity through the void door decides nothing.
/// This is the row the PR's zero-verdicts assertion needs to be able
/// to go red: under `Bridge::Recertify` it should log verdicts.
#[test]
fn rr2_plain_cavity_void_insert_decides_nothing() {
    let cavity = common::brick::<f64>((0.25, 0.75), (0.25, 0.75), (0.25, 0.75), Tol::witness());
    let mut dst = common::brick::<f64>((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), Tol::witness());
    let (solid, _) = dst.solids().next().unwrap();
    let ev = carried(&cavity);
    let bracket = geom_core::k_stats::Bracket::open();
    topo::insert_void(&mut dst, solid, cavity, &ev, Tol::witness()).expect("inserts");
    let verdicts = bracket.finish().verdicts;
    assert!(verdicts.is_empty(), "{} verdicts", verdicts.len());
    assert_eq!(topo::validate_geometric(&dst, Tol::witness()), Ok(()));
}

/// The inserted M7-8 cavity's body moves afterwards: the carried
/// certificates feed a transform that re-derives them.
#[test]
fn rr2_m7_8_cavity_inserted_then_moved() {
    let cavity = m7_8_cube::<f64>();
    let mut dst = common::brick::<f64>((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), Tol::witness());
    let (solid, _) = dst.solids().next().unwrap();
    let ev = carried(&cavity);
    topo::insert_void(&mut dst, solid, cavity, &ev, Tol::witness()).expect("inserts");
    let map =
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.3);
    let moved = transform_rigid(&dst, &map, Tol::witness()).expect("moves");
    assert_eq!(validate_closed(&moved), Ok(()));
    assert_eq!(edge_findings(&moved), 0);
}

/// The Dual chain on a body without the class: `transform_rigid` at
/// `Dual64` moves a brick to the f64 run's points, value for value.
#[test]
fn rr2_dual_transform_of_a_plain_body_matches_f64() {
    use geom_core::Dual64;
    let b64 = common::brick::<f64>((0.0, 2.0), (0.0, 1.0), (0.0, 0.5), Tol::witness());
    let bd = common::brick::<Dual64>((0.0, 2.0), (0.0, 1.0), (0.0, 0.5), Tol::witness());
    let map =
        Affine3::rotation_about_axis(Point3::new(0.1, 0.2, 0.3), Vec3::new(0.0, 0.6, 0.8), 0.9);
    let m64 = transform_rigid(&b64, &map, Tol::witness()).unwrap();
    let md = transform_rigid(
        &bd,
        &map.map(<Dual64 as geom_core::Real>::from_f64),
        Tol::witness(),
    )
    .unwrap();
    for (k, p) in m64.points() {
        let q = md.get_point(k).unwrap();
        assert_eq!(
            (p.x.to_bits(), p.y.to_bits(), p.z.to_bits()),
            (
                q.x.value.to_bits(),
                q.y.value.to_bits(),
                q.z.value.to_bits()
            )
        );
    }
}
