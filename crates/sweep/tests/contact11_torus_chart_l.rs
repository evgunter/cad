//! **No boolean mints an L-shaped torus face yet**, and this row says
//! so. A half donut less a bar over one quarter of its upper tube would
//! leave both torus walls L-shaped in the chart (a parallel cut by the
//! bar's floor, a meridian by its side); the pipeline refuses the torus
//! × plane pair (the germ frame has no arm for it) before any
//! containment is asked. The chart-box check
//! that refuses such a face at the containment doors is pinned in
//! `topo`'s `an_l_shaped_torus_face_refuses_rather_than_trim_by_its_hull`;
//! what this row asserts once the pair is admitted is scheduled by
//! `work/contacthold/notched-half-donut-owes-its-notch-and-volume-when-torus-plane-lands.md`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::bar;
use crate::revolve_common;
use geom::SurfaceKind;
use geom_core::Tol;
use revolve_common::{axis_y, validated};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::BooleanError;

#[test]
fn a_notched_half_donut_refuses_at_the_torus_plane_pair() {
    let vp = validated(vec![revolve_common::donut_profile()]);
    let half = revolve(
        &vp,
        axis_y(),
        Revolution::Partial(core::f64::consts::PI),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let half = finished("the half donut", half, Tol::witness());
    let cutter = bar((0.0, 3.0), (0.0, 1.0), (-3.0, 3.0));
    let Err(err) = topo::subtract(&half, &cutter, Tol::witness()) else {
        panic!("the torus × plane pair is refused");
    };
    assert!(
        matches!(
            err,
            BooleanError::GermFrameUnsupported {
                a_kind: SurfaceKind::Torus,
                b_kind: SurfaceKind::Plane,
                ..
            }
        ),
        "refused where the germ frame has no torus × plane arm: {err:?}"
    );
}
