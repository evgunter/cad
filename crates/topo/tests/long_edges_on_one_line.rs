//! **Straight edges kilometres long lie past the session box, and their
//! certification refuses them by the box.**
//!
//! The census reads two collinear edges' overlap from their ends'
//! projections on one line (`geom_core::one_line`). The projections'
//! rounding passes the band's room `(K − 1)·ε` only for spans of
//! several kilometres, past the session box (D4 ¶4), which the run's
//! admission rule keeps every in-box span inside. So a bar that reaches
//! toward that regime is refused at its first long edge for its reach,
//! before any check reads a margin on it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::Pose;
use geom_core::Tol;
use topo::Body;

/// **A 1.5 km bar, turned off the axes, is refused by the box**: its far
/// end's `x` is about 1194 m, past the box's kilometre (its `y`, 909 m,
/// is not), so the bar's first long edge, from the origin to that end,
/// refuses at its certification by the box, 194 m past it: the box bounds
/// each coordinate, not the distance, which is 1.5 km.
#[test]
fn a_bar_past_the_box_along_one_axis_is_refused_by_the_session_box() {
    use topo::construction::{Construct, MevSite};
    let tol = Tol::witness();
    let pose = Pose::turn("turned 0.65 rad about z", [0.0, 0.0, 1.0], 0.65, [0.0; 3]);
    let (near, far) = (pose.at([0.0, 0.0, 0.0]), pose.at([1500.0, 0.0, 0.0]));
    assert!(far.x > 1.0e3 && far.y.abs() < 1.0e3, "{far:?}");
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(near, true).unwrap();
    let got = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        far,
        geom_brep::EdgeCurveSpec::line_between(near, far),
        tol,
    );
    let Err(topo::EulerOpError::DescriptionUncertified {
        error: geom_brep::CertifyError::OutsideSessionBox { margin },
        ..
    }) = &got
    else {
        panic!("the box's refusal at the edge's certification: {got:?}");
    };
    let past = margin.readable_below_zero().to_string();
    assert!(past.starts_with("194"), "194 m past the box: {past}");
}
