//! **Straight edges kilometres long lie past the session box, and the
//! finished-body gate refuses them by the box.**
//!
//! The census reads two collinear edges' overlap from their ends'
//! projections on one line (`geom_core::one_line`). The projections'
//! rounding passes the band's room `(K − 1)·ε` only for spans of
//! several kilometres, past the session box (D4 ¶4), which the run's
//! admission rule keeps every in-box span inside. So the bars that
//! reached that regime, 3 and 6 km long, are refused at the
//! finished-body gate for their reach, alone and before any check
//! reads a margin on them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::Pose;
use common::{FaceGeometry, describe_as_intersections, prism_ops};
use geom_core::Tol;
use topo::{Body, ValidationError, validate_geometric};

/// **A 6 km bar, turned off every axis, is refused by the box**: its
/// far end lies 6 km out, and the gate's one finding is the box's.
#[test]
fn a_six_kilometre_bar_is_refused_by_the_session_box() {
    let tol = Tol::witness();
    let pose = Pose::turn("turned 0.65 rad about z", [0.0, 0.0, 1.0], 0.65, [0.0; 3]);
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &[(0.0, 0.0), (6000.0, 0.0), (6000.0, 1.0), (0.0, 1.0)],
        (0.0, 1.0),
        |x, y, z| pose.at([x, y, z]),
        FaceGeometry::Certified,
        tol,
    );
    describe_as_intersections(&mut body, tol);
    let errors = validate_geometric(&body, tol).unwrap_err();
    assert!(
        matches!(&errors[..], [ValidationError::OutsideSessionBox { .. }]),
        "the box's finding, alone: {errors:?}"
    );
    assert!(
        errors[0].to_string().contains("session box"),
        "{}",
        errors[0]
    );
}
