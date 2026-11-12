//! **A partner across a lofted wall is joined on the pair's section
//! value.** The loft prism's walls are splines; a ball or a brick across
//! one meets it in an arc from the wall's rim to its rim, or from side to
//! side, which the join reads as the carried component the section value
//! lists, its chords cut from the reading's own certified carrier on
//! both operands. Each run passes the join's lanes and stops at the
//! at-rest certificate of the chord the join mints, or past the join:
//! a curved partner's arc against a spline has no at-rest edge
//! certificate yet, and a plane's arc across a curved wall stops at the
//! plane × spline lane's own image. Neither is the join's.
//!
//! Each run is a union at the witness tolerance. Red against main before
//! the section value carried the pair: each refused at the join, where a
//! germ on a spline wall has no frame (`GermFrameUnsupported`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled_z, finished, loft_prism};
use topo::{BooleanError, EulerOpError, SplitJoinError};

/// The run's refusal as the chord edge's certification, if it is one.
fn chord_certificate(e: &BooleanError) -> Option<&geom_brep::CertifyError> {
    let BooleanError::Join(join) = e else {
        return None;
    };
    match join.error() {
        SplitJoinError::Euler(EulerOpError::Certification { error }) => Some(error),
        _ => None,
    }
}

/// **A ball across the prism's wall and its bottom cap**: the ball meets
/// the wall in an arc from the wall's bottom rim back to it, which the
/// join carries; the chord it mints on the wall and the ball is an
/// intersection of a sphere and a spline, which the at-rest certificate
/// has no arm for, so the union stops there.
///
/// Red against: the join reading a spline × sphere germ through a frame
/// (`GermFrameUnsupported`); a table that does not hold the pair.
#[test]
fn a_ball_across_a_lofted_wall_is_joined_on_its_carried_arc() {
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
        let refusal = topo::union(&prism, &ball, t).expect_err("no sphere × spline edge certifies");
        assert!(
            matches!(
                chord_certificate(&refusal),
                Some(geom_brep::CertifyError::Unimplemented)
            ),
            "{what}: the join mints the carried chord, which the at-rest certificate refuses: \
             {refusal:?}"
        );
    }
}

/// **A brick across the prism's wall**: the brick's planes meet the wall
/// in arcs the join carries, each plane's chord an arc of the same
/// carrier as the wall's. Across the flat wall at its rim the join
/// completes, and the union stops past it; across the flared wall the
/// plane × spline certificate's own image of the chord refuses it.
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
    let refusal = topo::union(&prism, &flat, t).expect_err("the result's containment refuses");
    assert!(
        matches!(refusal, BooleanError::Containment(_)),
        "the flat wall: the join completes: {refusal:?}"
    );
    let flared = brick((-0.25, 0.35), (-1.3, -0.8), (0.75, 1.25));
    let refusal = topo::union(&prism, &flared, t).expect_err("the plane × spline image refuses");
    assert!(
        matches!(
            chord_certificate(&refusal),
            Some(geom_brep::CertifyError::PlaneNurbs(_))
        ),
        "the flared wall: the join mints the carried chord: {refusal:?}"
    );
}
