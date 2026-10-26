//! **A boolean mints an L-shaped torus face.** A
//! half donut less a bar over one quarter of its upper tube leaves both
//! torus walls L-shaped in the chart (a parallel cut by the bar's floor,
//! a meridian by its side). The op answers its volume in closed form,
//! and containment reads the walls by their regions (`topo`'s
//! `an_l_shaped_torus_face_reads_its_notch_not_its_hull` reads one face).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::bar;
use crate::germ_plane_torus_chord::{about_the_tube, contains, points};
use crate::revolve_common;
use geom_core::{Point3, Tol};
use revolve_common::{axis_y, validated};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};

/// **The notched half donut**: the half donut (`R = 2`, `r = 1/2`, half a
/// turn about `y`, `π²/2`) less the bar `[0, 3] × [0, 1] × [−3, 3]`,
/// which takes the upper half of its tube over the quarter turn at
/// `x > 0` (`π²/8`). The result is `3π²/8`, valid at tier 3, and
/// `point_in_solid` reads it as the analytic membership.
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
    let v = topo::test_support::pinned_volume("the measured body", b, Tol::witness()).volume;
    let want = 3.0 * pi2 / 8.0;
    assert!(
        (v - want).abs() <= want * 1e-9_f64.max(1e3 * Tol::witness().eps()),
        "the notched half donut's volume {v} against {want}"
    );
    // Containment reads the L-shaped walls by their regions: `Out` in
    // the notch, `In` under it, `In` on the other quarter's top and
    // `Out` past the half turn; and at a lattice over the tube, the
    // analytic membership.
    let c = core::f64::consts::FRAC_1_SQRT_2 * 2.0;
    for (q, inside) in [
        (Point3::new(c, 0.25, -c), false),
        (Point3::new(c, -0.25, -c), true),
        (Point3::new(-c, 0.25, -c), true),
        (Point3::new(-c, 0.25, c), false),
    ] {
        assert_eq!(contains(b, q), Ok(inside), "{q:?}");
    }
    let (lo, hi) = about_the_tube(2.0, 0.5);
    let mut read = 0;
    for q in points(lo, hi, 400) {
        let tube = ((q.x.hypot(q.z) - 2.0).powi(2) + q.y.powi(2)).sqrt() - 0.5;
        // Off every boundary the oracle names by a clear margin: the tube,
        // the half turn's plane `z = 0` and the bar's floor and side.
        if [tube, q.z, q.y, q.x].iter().any(|m| m.abs() < 1e-6) {
            continue;
        }
        let inside = tube < 0.0 && q.z < 0.0 && !(q.x > 0.0 && q.y > 0.0);
        assert_eq!(contains(b, q), Ok(inside), "{q:?}");
        read += 1;
    }
    assert!(read > 390, "{read} lattice points read");
}
