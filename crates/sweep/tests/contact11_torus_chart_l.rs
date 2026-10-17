//! **A boolean mints an L-shaped torus face.** A
//! half donut less a bar over one quarter of its upper tube leaves both
//! torus walls L-shaped in the chart (a parallel cut by the bar's floor,
//! a meridian by its side). The chart-box check that refuses such a face
//! at the containment doors is pinned in `topo`'s
//! `an_l_shaped_torus_face_refuses_rather_than_trim_by_its_hull`, and
//! it refuses here; the op answers its volume in closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::bar;
use crate::revolve_common;
use geom_core::{Point3, Tol};
use revolve_common::{axis_y, validated};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};

/// **The notched half donut**: the half donut (`R = 2`, `r = 1/2`, half a
/// turn about `y`, `π²/2`) less the bar `[0, 3] × [0, 1] × [−3, 3]`,
/// which takes the upper half of its tube over the quarter turn at
/// `x > 0` (`π²/8`). The result is `3π²/8`, valid at tier 3.
#[test]
fn a_notched_half_donut_answers_its_volume() {
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
    let r = topo::subtract(&half, &cutter, Tol::witness()).expect("the notch builds");
    let b = &r.body().expect("non-empty").body;
    assert_eq!(topo::validate_geometric(b, Tol::witness()), Ok(()));
    let pi2 = core::f64::consts::PI.powi(2);
    let v = topo::mass_properties(b, Tol::witness()).unwrap().volume;
    let want = 3.0 * pi2 / 8.0;
    assert!(
        (v - want).abs() <= want * 1e-9_f64.max(1e3 * Tol::witness().eps()),
        "the notched half donut's volume {v} against {want}"
    );
    // Containment on the result refuses typed: its torus walls are L
    // shapes in the chart, in neither class the torus chart trim reads
    // (`work/inside/point-in-solid-refuses-an-l-shaped-torus-face.md`).
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    let c = core::f64::consts::FRAC_1_SQRT_2 * 2.0;
    for q in [Point3::new(c, 0.25, -c), Point3::new(-c, 0.25, c)] {
        let got = topo::point_in_solid(b, q, band, Tol::witness());
        assert!(
            matches!(got, Err(topo::PointInSolidError::PartialTorusFace { .. })),
            "{q:?}: {got:?}"
        );
    }
}
