//! **A plane against a torus is the closed form's at one lever**: the
//! section value's pose (`geom_brep::PlaneTorusPose`) asks whether the
//! pose is the closed form's once, over the torus's ball, and the C5
//! table's pose question (`geom_brep::route_pose`) reads that answer
//! rather than asking again at the consumer's reach.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::Surface;
use geom_brep::{PlaneTorusPose, PtStance, Reach, SectionRead, route_pose};
use geom_core::{Point3, Vec3};

/// The ring `R = 0.75`, `r = 0.3` about `z` through the origin.
fn torus() -> Surface<f64> {
    Surface::Torus {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        major_radius: 0.75,
        minor_radius: 0.3,
    }
}

/// The plane through the centre holding the axis, turned off it by `tilt`
/// about `x`.
fn tilted(tilt: f64) -> Surface<f64> {
    Surface::Plane {
        frame: geom::Frame::new(
            Point3::origin(),
            Vec3::new(0.0, tilt.cos(), tilt.sin()),
            Vec3::unit_x(),
        ),
    }
}

/// **Every door reads the pose the value reads**, over tilts straddling
/// the value's band against a consumer reach a tenth and a hundred times
/// a metre: the pose reads closed below `5e-10` and tilted from it, so
/// the run holds both. At each tilt the section value reads a closed pose
/// elsewhere and builds a tilted one, its closed form serves the closed
/// one alone, and `route_pose` answers the arm implemented at both
/// reaches. Asked at the reach, the closed form read `2.6e-10` closed to
/// the value and tilted at a hundred metres, and `4e-9` tilted to the
/// value and in the band of `pt_axis_plane_gap` at a tenth, where the
/// gate escalated a pose the value builds.
#[test]
fn every_door_reads_the_pose_the_value_reads() {
    let b = band();
    let mut seen = (false, false);
    for tilt in [1e-11, 6.4e-11, 2.6e-10, 5.12e-10, 4.096e-9, 1.6e-8, 1e-3] {
        let (plane, torus) = (tilted(tilt), torus());
        let pose = PlaneTorusPose::decide(&plane, &torus, b)
            .expect("a plane and a torus")
            .unwrap_or_else(|e| panic!("tilt {tilt:e}: the pose decides, got {e:?}"));
        let closed = matches!(pose.stance(), PtStance::Closed);
        if closed {
            seen.0 = true;
        } else {
            seen.1 = true;
        }
        assert_eq!(
            pose.closed_form().is_ok(),
            closed,
            "tilt {tilt:e}: the closed form serves the closed pose alone, got {:?}",
            pose.closed_form()
        );
        let read = geom_brep::pair_section(&plane, &torus, None, None::<geom_core::NoLane>, b);
        assert!(
            matches!(
                (&read, closed),
                (Ok(SectionRead::Elsewhere(_)), true) | (Ok(SectionRead::Built(_)), false)
            ),
            "tilt {tilt:e}, closed {closed}: the value read {read:?}"
        );
        for length in [0.1, 100.0] {
            let reach = Reach::fixture(Point3::origin(), length);
            for (a, c) in [(&plane, &torus), (&torus, &plane)] {
                let routed = route_pose(a, c, &reach, b);
                assert!(
                    matches!(&routed, Ok(route) if route.implemented),
                    "tilt {tilt:e}, reach {length}: the gate reads the value's pose, got {routed:?}"
                );
            }
        }
    }
    assert_eq!(
        seen,
        (true, true),
        "the run holds a closed pose and a tilted one"
    );
}
