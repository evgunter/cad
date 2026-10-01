//! **Review probes for PR 3678 (CLEAVE: the sealed plane x NURBS lane).**
//!
//! The M7-8 cube of `m4_pr2_transform`, built at any certifying scalar,
//! driven through the public doors the PR changes: the transform at
//! f64 and at `Interval`, a rotation and a mirror, the void door and the
//! two boolean fallbacks that graft (the subtract's cavity through the
//! void door, the disjoint union through the boolean graft).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Affine3, Point3, Real, Tol, Vec3};
use topo::{AtRestPolicy, Body, BooleanResult, transform_rigid};

/// `m4_pr2_transform`'s described NURBS wall on `y = 0`, at `T`.
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

/// `m4_pr2_transform::m7_8_cube`, at any scalar that holds the lane.
fn m7_8_cube_at<T: geom_core::Decide + geom_core::CertifiedBounds>() -> Body<T> {
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
        let p0 = *body
            .get_point(body.get_vertex(start).unwrap().point)
            .unwrap();
        let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
        ));
        let half = T::from_f64(0.5);
        body.set_edge_curve_nurbs_lane(
            edge_key,
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    s1,
                    s2,
                    witness: p0.lerp(p1, half),
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
    assert_eq!(lane_edges, 4, "the front wall has four M7-8 edges");
    body
}

/// Check-2 findings of the certified at-rest door, at `T`.
fn edge_findings<T>(body: &Body<T>) -> usize
where
    T: geom_core::Decide + geom_core::CertifiedBounds + AtRestPolicy,
{
    match topo::validate_pseudomanifold(body, &topo::ContactRecords::default(), Tol::witness()) {
        Ok(()) => 0,
        Err(errs) => errs
            .iter()
            .filter(|e| matches!(e, topo::ValidationError::EdgeCertification { .. }))
            .count(),
    }
}

/// The number of M7-8 carriers (`Nurbs` carriers under an
/// `Intersection`) a body holds — so a row that says "the class
/// survived" is not passing on a body that lost it.
fn m7_8_edges<T: geom_core::Real>(body: &Body<T>) -> usize {
    body.curves()
        .filter(|(_, c)| match c {
            topo::CurveGeom::Certified(c) => {
                matches!(c.carrier(), geom::Curve3::Nurbs(_))
                    && matches!(
                        c.description(),
                        geom_brep::EdgeDescription::Intersection { .. }
                    )
            }
            _ => false,
        })
        .count()
}

/// A rotation about a skew axis through a non-dyadic angle: every
/// coordinate of the wall's net and every carrier moves inexactly, so
/// the lane's limbs are re-derived on rounded geometry, not on a
/// translated copy of the input bits.
#[test]
fn the_m7_8_cube_turns_about_a_skew_axis_at_f64() {
    let body = m7_8_cube_at::<f64>();
    let map = Affine3::rotation_about_axis(
        Point3::new(0.5, -0.25, 0.75),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.3,
    );
    let moved = transform_rigid(&body, &map, Tol::witness()).expect("an M7-8 body turns at f64");
    assert_eq!(m7_8_edges(&moved), 4);
    assert_eq!(edge_findings(&moved), 0);
}

/// The certified interval scalar holds the lane too, so the same body
/// built at `Interval` moves through the same door.
#[test]
fn the_m7_8_cube_moves_at_interval() {
    use geom_core::interval::Interval;
    let body = m7_8_cube_at::<Interval>();
    assert_eq!(
        edge_findings(&body),
        0,
        "the cube certifies at rest at Interval"
    );
    let map = Affine3::translation(Vec3::new(0.25, -1.5, 8.0)).map(Interval::from_f64);
    let moved =
        transform_rigid(&body, &map, Tol::witness()).expect("an M7-8 body moves at Interval");
    assert_eq!(m7_8_edges(&moved), 4);
    assert_eq!(edge_findings(&moved), 0);
}

/// A mirror is refused by the rigidity door, before any carrier is
/// re-certified: the M7-8 class changes nothing about which maps a
/// body may take.
#[test]
fn a_mirror_of_the_m7_8_cube_refuses_not_rigid() {
    let body = m7_8_cube_at::<f64>();
    let mirror = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::zero(),
    );
    match transform_rigid(&body, &mirror, Tol::witness()) {
        Err(topo::TransformError::NotRigid { .. }) => {}
        other => panic!("expected NotRigid, got {other:?}"),
    }
}

/// The boolean subtract of an enclosed M7-8 cube refuses typed BEFORE
/// its fallback reaches the void door: the revert roster has no
/// plane x NURBS pair. So the void door's switch to carrying
/// certificates changes nothing a boolean caller can see for this
/// class, and the row goes red the day the roster admits the pair —
/// which is the day the void graft's carried certificates become
/// reachable from the boolean.
#[test]
fn subtracting_an_enclosed_m7_8_cube_refuses_before_the_void_door() {
    let cube = m7_8_cube_at::<f64>();
    let brick = common::brick::<f64>((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), Tol::witness());
    match topo::subtract(&brick, &cube, Tol::witness()) {
        Err(topo::BooleanError::CurvedPairUnsupported {
            operand: topo::Operand::B,
            ..
        }) => {}
        Err(e) => panic!("expected the revert roster's refusal, got {e:?}"),
        Ok(BooleanResult::Empty) => panic!("expected the revert roster's refusal, got Empty"),
        Ok(BooleanResult::Body(b)) => {
            panic!(
                "expected the revert roster's refusal, got a {:?} body",
                b.kind
            )
        }
    }
}

/// The disjoint union would graft through `graft_solid`, which still
/// re-certifies (`Bridge::Recertify`) through the lane-free door — the
/// PR leaves it alone. It is unreachable for this class at f64: the
/// boolean refuses the curved NURBS-carried edge first. The row goes
/// red the day that refusal lifts, which is the day the boolean
/// graft's lane-free re-certification starts refusing
/// `GraftRecertify(NurbsLaneUnsupported { scalar: "f64" })` at a scalar
/// that holds the lane.
#[test]
fn a_disjoint_union_with_the_m7_8_cube_refuses_before_the_graft() {
    let cube = m7_8_cube_at::<f64>();
    let brick = common::brick::<f64>((4.0, 5.0), (4.0, 5.0), (4.0, 5.0), Tol::witness());
    match topo::union(&brick, &cube, Tol::witness()) {
        Err(topo::BooleanError::CurvedEdgeUnsupported {
            operand: topo::Operand::B,
            ..
        }) => {}
        Err(e) => panic!("expected the curved-edge refusal, got {e:?}"),
        Ok(BooleanResult::Empty) => panic!("expected the curved-edge refusal, got Empty"),
        Ok(BooleanResult::Body(b)) => {
            panic!("expected the curved-edge refusal, got a {:?} body", b.kind)
        }
    }
}

/// A dual's policy holds no lane, through the trait every generic
/// operation reads.
#[test]
fn dual_policy_holds_no_lane_and_sym_over_f64_holds_one() {
    assert!(<geom_core::Dual64 as AtRestPolicy>::nurbs_lane().is_none());
    assert!(<geom_core::Sym<f64> as AtRestPolicy>::nurbs_lane().is_some());
    assert!(<geom_core::interval::Interval as AtRestPolicy>::nurbs_lane().is_some());
}

/// The void door at `T`: the cavity inserts, its four M7-8 carriers
/// survive the graft AS the class (a row counting findings alone would
/// pass on a body that lost them), and check 2 re-derives them at rest.
fn void_door_keeps_the_class<T>()
where
    T: geom_core::Decide + geom_core::CertifiedBounds + AtRestPolicy,
{
    let cavity = m7_8_cube_at::<T>();
    let mut dst = common::brick::<T>((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), Tol::witness());
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = topo::VoidEvidence {
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
    };
    topo::insert_void(&mut dst, solid, cavity, &evidence, Tol::witness())
        .expect("an M7-8 cavity inserts");
    assert_eq!(dst.shells().count(), 2);
    assert_eq!(
        m7_8_edges(&dst),
        4,
        "the cavity's M7-8 carriers survive the graft"
    );
    assert_eq!(edge_findings(&dst), 0);
    // And the voided body moves: the transform re-derives the carried
    // certificates through the policy's lane.
    let map = Affine3::translation(Vec3::new(0.25, -1.5, 8.0)).map(T::from_f64);
    let moved = transform_rigid(&dst, &map, Tol::witness()).expect("the voided body moves");
    assert_eq!(m7_8_edges(&moved), 4);
    assert_eq!(edge_findings(&moved), 0);
}

#[test]
fn the_void_door_keeps_the_m7_8_class_at_f64() {
    void_door_keeps_the_class::<f64>();
}

#[test]
fn the_void_door_keeps_the_m7_8_class_at_interval() {
    void_door_keeps_the_class::<geom_core::interval::Interval>();
}

/// The symbolic tier over f64 holds the lane by its where-clause, so
/// the M7-8 cube built at `Sym<f64>` moves through the plain door too.
#[test]
fn the_m7_8_cube_moves_at_sym_over_f64() {
    use geom_core::Sym;
    let body = m7_8_cube_at::<Sym<f64>>();
    let map = Affine3::translation(Vec3::new(0.25, -1.5, 8.0)).map(Sym::<f64>::from_f64);
    let moved =
        transform_rigid(&body, &map, Tol::witness()).expect("an M7-8 body moves at Sym<f64>");
    assert_eq!(m7_8_edges(&moved), 4);
}

/// At a dual the transform reads `None` from the policy, and a body
/// without the class moves exactly as before: the lane's absence is
/// not a second code path for the ordinary classes.
#[test]
fn a_dual_brick_moves_bit_for_bit_with_its_f64_twin() {
    use geom_core::Dual64;
    let map = Affine3::rotation_about_axis(
        Point3::new(0.5, -0.25, 0.75),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.3,
    );
    let f = common::brick::<f64>((0.0, 2.0), (0.0, 1.0), (0.0, 0.5), Tol::witness());
    let d = common::brick::<Dual64>((0.0, 2.0), (0.0, 1.0), (0.0, 0.5), Tol::witness());
    let fm = transform_rigid(&f, &map, Tol::witness()).expect("f64 brick turns");
    let dm =
        transform_rigid(&d, &map.map(Dual64::from_f64), Tol::witness()).expect("dual brick turns");
    let bits = |p: Point3<f64>| (p.x.to_bits(), p.y.to_bits(), p.z.to_bits());
    let fp: Vec<_> = fm.points().map(|(_, p)| bits(*p)).collect();
    let dp: Vec<_> = dm.points().map(|(_, p)| bits(p.map(|x| x.value))).collect();
    assert_eq!(fp, dp);
}
