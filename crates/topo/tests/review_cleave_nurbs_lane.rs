//! **The plane × NURBS lane read off the scalar's policy, through the
//! public doors** — the M7-8 cube (`fixture::m7_8`) driven through the
//! transform at every scalar that holds the lane, the void door, and
//! the two boolean fallbacks that graft. Authored across PR 3678's lane
//! and both of its review lanes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use crate::fixture::m7_8::{carried, edge_findings, m7_8_cube, m7_8_edges};

use geom_core::{Affine3, Point3, Real, Tol, Vec3};
use topo::{AtRestPolicy, BooleanResult, transform_rigid, validate_closed};

/// A rotation about a skew axis through a non-dyadic angle, then its
/// inverse: every coordinate of the wall's net and every carrier moves
/// inexactly, so the lane's limbs are re-derived on rounded geometry
/// both ways, and the class survives both maps.
#[test]
fn the_m7_8_cube_turns_about_a_skew_axis_and_back_at_f64() {
    let body = m7_8_cube::<f64>();
    let map = Affine3::rotation_about_axis(
        Point3::new(0.3, -0.7, 0.1),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    );
    let moved = transform_rigid(&body, &map, Tol::witness()).expect("an M7-8 body turns at f64");
    assert_eq!(m7_8_edges(&moved), 4, "the class survives the turn");
    assert_eq!(edge_findings(&moved), 0, "check 2 is clean after the turn");
    let back = transform_rigid(&moved, &map.inverse(), Tol::witness()).expect("and turns back");
    assert_eq!(m7_8_edges(&back), 4, "the class survives the inverse");
    assert_eq!(
        edge_findings(&back),
        0,
        "check 2 is clean after the inverse"
    );
    for (k, p0) in body.points() {
        let p1 = back.get_point(k).unwrap();
        assert!(
            p1.distance(*p0) < 1e-12,
            "the round trip returns {k:?}: {p0:?} -> {p1:?}"
        );
    }
}

/// A mirror is refused by the rigidity door, before any carrier is
/// re-certified: the lane opens no reflection.
#[test]
fn a_mirror_of_the_m7_8_cube_refuses_not_rigid() {
    let body = m7_8_cube::<f64>();
    let mirror = geom_core::linalg::frame::mirror_across_plane::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Tol::witness(),
    )
    .unwrap();
    match transform_rigid(&body, &mirror, Tol::witness()) {
        Err(topo::TransformError::NotRigid { .. }) => {}
        other => panic!("expected NotRigid, got {other:?}"),
    }
}

/// The class moves at `T` through the plain door, because `T`'s policy
/// holds the lane, and re-derives at rest after the map.
fn moves_at<T>()
where
    T: geom_core::Decide + geom_core::CertifiedBounds + AtRestPolicy,
{
    let body = m7_8_cube::<T>();
    assert_eq!(edge_findings(&body), 0, "the cube certifies at rest");
    let map = Affine3::translation(Vec3::new(0.25, -1.5, 8.0)).map(T::from_f64);
    let moved = transform_rigid(&body, &map, Tol::witness()).expect("an M7-8 body moves");
    assert_eq!(m7_8_edges(&moved), 4, "the class survives the map");
    assert_eq!(edge_findings(&moved), 0, "check 2 is clean after the map");
}

#[test]
fn the_m7_8_cube_moves_at_interval() {
    moves_at::<geom_core::interval::Interval>();
}

#[test]
fn the_m7_8_cube_moves_at_sym_over_f64() {
    moves_at::<geom_core::Sym<f64>>();
}

/// At a dual the transform reads `None` from the policy, and a body
/// without the class moves to the f64 run's points bit for bit: the
/// lane's absence is not a second code path for the ordinary classes.
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

/// **The void door carries an M7-8 cavity's certificates** at `T`: the
/// cavity inserts, its four carriers survive the graft AS the class,
/// check 2 re-derives them at rest, the door logs no funnel verdict,
/// and the voided body then moves through the transform.
fn void_door_keeps_the_class<T>()
where
    T: geom_core::Decide + geom_core::CertifiedBounds + AtRestPolicy,
{
    let cavity = m7_8_cube::<T>();
    let mut dst = common::brick::<T>((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), Tol::witness());
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = carried(&cavity);
    let bracket = geom_core::k_stats::Bracket::open();
    topo::insert_void(&mut dst, solid, cavity, &evidence).expect("an M7-8 cavity inserts");
    let verdicts = bracket.finish().verdicts;
    assert!(
        verdicts.is_empty(),
        "the door carries certificates and decides nothing: {} verdicts",
        verdicts.len()
    );
    assert_eq!(dst.shells().count(), 2, "outer shell and the cavity");
    assert_eq!(validate_closed(&dst), Ok(()));
    assert_eq!(m7_8_edges(&dst), 4, "the carriers survive the graft");
    assert_eq!(edge_findings(&dst), 0, "check 2 re-derives them at rest");
    // A dyadic translation, exact at every scalar: a rotation widens an
    // interval plane's structural parameters, which the lane refuses
    // (`PlaneNurbsRefusal::Unsupported`) as a family of surfaces.
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

/// The control for the row above's zero-verdict assertion: a plain
/// brick cavity, which any re-certifying graft would meter (and so log
/// verdicts for), inserts with none, and the result is valid at rest.
#[test]
fn a_plain_cavity_through_the_void_door_decides_nothing() {
    let cavity = common::brick::<f64>((0.25, 0.75), (0.25, 0.75), (0.25, 0.75), Tol::witness());
    let mut dst = common::brick::<f64>((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), Tol::witness());
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = carried(&cavity);
    let bracket = geom_core::k_stats::Bracket::open();
    topo::insert_void(&mut dst, solid, cavity, &evidence).expect("inserts");
    let verdicts = bracket.finish().verdicts;
    assert!(verdicts.is_empty(), "{} verdicts", verdicts.len());
    assert_eq!(topo::validate_geometric(&dst, Tol::witness()), Ok(()));
}

/// The boolean subtract of an enclosed M7-8 cube refuses typed BEFORE
/// its fallback reaches the void door: the revert roster has no
/// plane × NURBS pair. The row goes red the day the roster admits the
/// pair, which is the day the void graft's carried certificates become
/// reachable from the boolean.
#[test]
fn subtracting_an_enclosed_m7_8_cube_refuses_before_the_void_door() {
    let cube = m7_8_cube::<f64>();
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

/// A disjoint union with the M7-8 cube refuses typed BEFORE its
/// fallback reaches the assembly graft: the continuation scan cannot
/// bound the cube's face that a NURBS-carried edge bounds, and names
/// that edge. The row goes red the day that face has a box, which is
/// the day the assembly's carried certificates become reachable from
/// the boolean.
#[test]
fn a_disjoint_union_with_the_m7_8_cube_refuses_before_the_graft() {
    let cube = m7_8_cube::<f64>();
    let brick = common::brick::<f64>((4.0, 5.0), (4.0, 5.0), (4.0, 5.0), Tol::witness());
    match topo::union(&brick, &cube, Tol::witness()) {
        Err(topo::BooleanError::EdgeCarrierUnsupported {
            operand: topo::Operand::B,
            site: topo::EdgeCarrierSite::FaceExtent,
            ..
        }) => {}
        Err(e) => panic!("expected the spline-bounded face's refusal, got {e:?}"),
        Ok(BooleanResult::Empty) => panic!("expected the spline-bounded face's refusal, got Empty"),
        Ok(BooleanResult::Body(b)) => {
            panic!(
                "expected the spline-bounded face's refusal, got a {:?} body",
                b.kind
            )
        }
    }
}
