//! **A partner across a lofted wall is joined on the pair's section
//! value.** The loft prism's walls are splines; a ball or a brick across
//! one meets it in an arc from the wall's rim to its rim, or from side to
//! side, which the join reads as the carried component the section value
//! lists, its chords cut from the reading's own certified carrier on
//! both operands. Each run passes the join's lanes and stops past the
//! join, at the inside/outside test no spline face can be crossed by, or
//! at the at-rest certificate of the chord the join mints: a plane's arc
//! across a curved wall stops at the plane × spline lane's own image.
//! Neither is the join's. Two arcs of one pair closer than the pads their
//! points are read within carry neither, by name. A brick wholly inside
//! a wall's face meets it in a closed loop, a ring of the wall the join
//! walls off as an island.
//!
//! Each run is at the witness tolerance. Red against main before
//! the section value carried the pair: each refused at the join, where a
//! germ on a spline wall has no frame (`GermFrameUnsupported`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled_z, finished, loft_prism};
use topo::{BooleanError, EulerOpError, JoinError};

/// The run's refusal as the chord edge's certification, if it is one.
fn chord_certificate(e: &BooleanError) -> Option<&geom_brep::CertifyError> {
    let BooleanError::Join(join) = e else {
        return None;
    };
    match join.error() {
        JoinError::Euler(EulerOpError::Certification { error }) => Some(error),
        _ => None,
    }
}

/// **A ball across the prism's wall and its bottom cap**: the ball meets
/// the wall in an arc from the wall's bottom rim back to it, which the
/// join carries; the chord it mints on the wall and the ball is an
/// intersection of a sphere and a spline, which the at-rest certificate
/// holds through its curved-partner lane. The join completes, and the
/// union stops past it at the inside/outside test, which cannot cross a
/// spline face
/// (`work/inside/point-containments-ray-test-has-no-spline-arm.md`).
///
/// At `ε = 1e-12` the chord stops at its certificate instead: the lane's
/// image of the arc on the wall misses its foot path by more than that ε
/// (`work/sectarms/a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung.md`).
///
/// Red against: a certificate with no arm for a sphere beside a spline
/// (`CertifyError::Unimplemented`); the join reading a spline × sphere
/// germ through a frame (`GermFrameUnsupported`); a table that does not
/// hold the pair.
#[test]
fn a_ball_across_a_lofted_wall_is_joined_and_its_chord_certifies_at_rest() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    for (what, c) in [
        ("the flat wall", [0.0, 0.9, 0.1]),
        ("the flared wall", [0.2, -1.05, 0.1]),
    ] {
        let ball = finished(
            "the ball",
            ball_poled_z(0.25, Vec3::new(c[0], c[1], c[2]), t),
            t,
        );
        let refusal =
            topo::union(&prism, &ball, t).expect_err("no ray crosses the prism's spline walls");
        let read = match chord_certificate(&refusal) {
            Some(
                geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                    limb: geom_brep::SsiLimb::ImageMiss,
                    ..
                })
                | geom_brep::CertifyError::Escalated {
                    check: geom_brep::CertCheck::PlaneNurbsHullValue,
                    ..
                },
            ) => t.eps() == 1e-12,
            Some(_) => false,
            None => {
                t.eps() != 1e-12
                    && matches!(
                        refusal,
                        BooleanError::Containment(topo::PointInSolidError::KindUnsupported {
                            kind: geom::SurfaceKind::Nurbs,
                            ..
                        })
                    )
            }
        };
        assert!(
            read,
            "{what}: the chord certifies at rest and the join completes, the inside/outside \
             test refusing the wall (at ε = 1e-12, the chord's image misses): {refusal:?}"
        );
    }
}

/// **A ball across two close arcs of a lofted wall refuses by name**:
/// on the prism whose sections stand `0.2` apart, the ball of radius
/// `0.21` about `(0.3, 0.95, 0.2)` cuts a wall in two rim arcs closer
/// than the pads their points are read within, so a site on either would
/// read on both. The carry refuses `PadsMeet`, and the germ
/// `GermSectionUnbuilt` naming the wall and the ball, in union, subtract
/// and swapped union.
///
/// Red against: a carry that holds no two arcs apart, where a site on
/// one arc read on both and the run aborted `JoinDesync`.
#[test]
fn a_ball_across_two_close_arcs_of_a_lofted_wall_refuses_by_name() {
    let t = Tol::witness();
    let prism = finished(
        "the shallow prism",
        sweep::test_support::loft_prism_at(&[0.0, 0.2, 0.4], t),
        t,
    );
    let ball = finished(
        "the ball",
        ball_poled_z(0.21, Vec3::new(0.3, 0.95, 0.2), t),
        t,
    );
    for (what, run) in [
        ("prism ∪ ball", topo::union(&prism, &ball, t)),
        ("prism ∖ ball", topo::subtract(&prism, &ball, t)),
        ("ball ∪ prism", topo::union(&ball, &prism, t)),
    ] {
        let refusal = run.expect_err("two arcs within their pads carry neither");
        let BooleanError::GermSectionUnbuilt {
            a_kind,
            b_kind,
            why: topo::SectionUnbuilt::Carry(geom_brep::CarryRefusal::PadsMeet { pads, gap }),
            ..
        } = &refusal
        else {
            panic!("{what}: the carry refuses the two arcs by name: {refusal:?}");
        };
        let mut kinds = [*a_kind, *b_kind];
        kinds.sort_by_key(|k| format!("{k:?}"));
        assert_eq!(
            kinds,
            [geom::SurfaceKind::Nurbs, geom::SurfaceKind::Sphere],
            "{what}: the germ names the wall and the ball"
        );
        assert!(
            *gap <= pads[0] + pads[1],
            "{what}: the arcs stand {gap} apart, within the pads {pads:?}"
        );
    }
}

/// **A brick across the prism's wall**: the brick's planes meet the wall
/// in arcs the join carries, each plane's chord an arc of the same
/// carrier as the wall's. Across the flat wall at its rim the guard reads
/// each of the wall's pairs off its listing and clears it, the join
/// completes, and the union stops past it at the inside/outside test,
/// which cannot cross a spline face; a brick wholly inside the prism
/// stops there too, but with no pair crossing the wall. Across the
/// flared wall the plane × spline certificate refuses the chord, its own
/// image missing it (at `ε = 1e-6`, its hull value escalating first).
///
/// Red against: a carried chord refused on a planar face; the join
/// reading a plane × spline germ through a frame.
#[test]
fn a_brick_across_a_lofted_wall_is_joined_on_its_carried_arcs() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    let brick = |x, y, z| {
        finished(
            "the brick",
            topo::test_support::brick_with_fresh_keys::<f64>(x, y, z, t),
            t,
        )
    };
    let flat = brick((-0.25, 0.25), (0.75, 1.25), (-0.25, 0.25));
    let report = topo::test_support::section_report(topo::BooleanOp::Union, &prism, &flat, t)
        .expect("the reduction reads");
    let cleared: Vec<_> = report
        .iter()
        .filter(|(fa, _, verdict)| {
            matches!(prism.face_surface(*fa), Some(geom::Surface::Nurbs(_)))
                && verdict.starts_with("Ok(")
        })
        .collect();
    assert!(
        !cleared.is_empty(),
        "the flat wall: the brick crosses it, and the guard clears its pairs: {report:?}"
    );
    let refusal = topo::union(&prism, &flat, t).expect_err("the result's containment refuses");
    assert!(
        matches!(
            refusal,
            BooleanError::Containment(topo::PointInSolidError::KindUnsupported {
                kind: geom::SurfaceKind::Nurbs,
                ..
            })
        ),
        "the flat wall: the join completes, and the inside/outside test cannot cross the \
         wall: {refusal:?}"
    );
    let flared = brick((-0.25, 0.35), (-1.3, -0.8), (0.75, 1.25));
    let refusal = topo::union(&prism, &flared, t).expect_err("the plane × spline lane refuses");
    // At `ε = 1e-6` the lane's hull value escalates before its image is
    // read; at every other row the image misses the chord.
    let read = match chord_certificate(&refusal) {
        Some(geom_brep::CertifyError::Escalated {
            check: geom_brep::CertCheck::PlaneNurbsHullValue,
            ..
        }) => t.eps() == 1e-6,
        Some(geom_brep::CertifyError::PlaneNurbs(_)) => t.eps() != 1e-6,
        _ => false,
    };
    assert!(
        read,
        "the flared wall: the join mints the carried chord, which the plane × spline lane \
         reads: {refusal:?}"
    );
}

/// Whether the run stopped past the join, at the inside/outside test
/// that cannot cross a spline face
/// (`work/inside/point-containments-ray-test-has-no-spline-arm.md`).
fn stops_at_containment(refusal: &BooleanError) -> bool {
    matches!(
        refusal,
        BooleanError::Containment(topo::PointInSolidError::KindUnsupported {
            kind: geom::SurfaceKind::Nurbs,
            ..
        })
    )
}

/// **A brick wholly inside the flat wall's face closes a ring on the
/// wall**: its four planes meet the wall in a closed loop of four
/// carried arcs, and the chord that closes the loop walls an island off
/// the wall, wound by paths along the wall's own iso curves. Every
/// operation passes the join and stops at the inside/outside test.
///
/// Red against: a ring lane with no winding on a spline wall, where each
/// run refused `CurvedBooleanUnsupported` naming the wall. Which way
/// round the island winds is not read here, since the stop comes before
/// any reader of the wall's two faces: topo's
/// `a_spline_wall_island_winds_as_its_corners_turn_and_places_its_bystanders`
/// holds it.
#[test]
fn a_brick_inside_a_lofted_walls_face_closes_a_ring_on_the_wall() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    let brick = finished(
        "the brick",
        topo::test_support::brick_with_fresh_keys::<f64>(
            (-0.25, 0.25),
            (0.75, 1.25),
            (0.75, 1.25),
            t,
        ),
        t,
    );
    for (what, run) in [
        ("prism ∪ brick", topo::union(&prism, &brick, t)),
        ("prism ∖ brick", topo::subtract(&prism, &brick, t)),
        ("prism ∩ brick", topo::intersect(&prism, &brick, t)),
        ("brick ∪ prism", topo::union(&brick, &prism, t)),
        ("brick ∖ prism", topo::subtract(&brick, &prism, t)),
    ] {
        let refusal = run.expect_err("no ray crosses the prism's spline walls");
        assert!(
            stops_at_containment(&refusal),
            "{what}: the island closes on the wall and the join completes, the inside/outside \
             test refusing the wall: {refusal:?}"
        );
    }
}

/// **A fork with two prongs inside the flat wall's face closes two
/// rings on the wall**: when the first island is walled off, the other
/// prong's pierce rings are bystanders on the wall, each placed out of
/// the island by a path on the wall. The union passes the join and stops
/// at the inside/outside test.
///
/// Red against: a re-homing with no reading on a spline wall, where the
/// first island's chord refused `RingIslandUnread` naming the wall.
#[test]
fn a_fork_inside_a_lofted_walls_face_closes_two_rings_on_the_wall() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    let brick = |x, y| {
        finished(
            "a brick",
            topo::test_support::brick_with_fresh_keys::<f64>(x, y, (0.75, 1.25), t),
            t,
        )
    };
    let joined = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>| {
        topo::union(a, b, t)
            .expect("two bricks sharing a face unite")
            .body()
            .expect("a union is never empty")
            .body
            .clone()
    };
    let bridge = brick((-0.6, 0.6), (1.5, 1.8));
    let fork = joined(
        &joined(&brick((-0.6, -0.3), (0.75, 1.5)), &bridge),
        &brick((0.3, 0.6), (0.75, 1.5)),
    );
    let refusal =
        topo::union(&prism, &fork, t).expect_err("no ray crosses the prism's spline walls");
    assert!(
        stops_at_containment(&refusal),
        "both islands close on the wall and the join completes, the inside/outside test \
         refusing the wall: {refusal:?}"
    );
}
