//! **A cylinder split just off its rim vertex keeps its sliver.** The unit
//! cylinder (`z ∈ [0, 1]`, its seam vertices at `(±1, 0, 0)` and
//! `(±1, 0, 1)`) split by the plane through `(1, 0, d)` with normal
//! `(−1e-2, 0, 1)` leaves a wedge below the plane, `d` high at the seam
//! vertex and meeting the rim disc in a chord near it
//! (`work/spanfold/a-cylinder-split-just-off-its-rim-vertex-exhausts-the-volume-quadrature-at-rest.md`).
//!
//! The wedge's wall is a cylinder face whose height is a few `d`. Its
//! quadrature read each trim edge's end trig off the edge's vertex, a
//! bracket `ε` wide, and that width rode every piece of the edge: on the
//! sliver it alone was the wall's floor, and the split refused at rest,
//! `QuadratureBudget` after 13 rounds and about 30 s. Each end is now
//! read at the parameter the lane integrates from
//! (`geom_brep::props::quad::trig_at_limit`), 3.4e-13 wide at most over a
//! period.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, split};

/// The unit cylinder about `z`, `0 ≤ z ≤ 1`.
fn cylinder(tol: Tol) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(
        sweep::test_support::prism(
            vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)],
            1.0,
            tol,
        ),
        tol,
    )
    .expect("the unit cylinder")
}

/// The split by the plane through `(1, 0, d)` turned `−1e-2` about `y`.
fn sliver_split(d: f64, tol: Tol) -> Result<topo::splitting::SplitResult<f64>, SplitError> {
    let normal = Vec3::new(-1e-2, 0.0, 1.0);
    let plane =
        topo::test_support::split_plane(Point3::new(1.0, 0.0, d), normal / normal.norm(), tol);
    split(&cylinder(tol), &plane, tol)
}

/// **At 1e-9 and 1e-6 the sliver builds, and the two halves hold the
/// cylinder's volume**, `π`, within their pads, with the plane `50ε` above
/// the seam vertex (the issue's `d` at 1e-9, which refused) and one and
/// two orders further. The sliver below is `d` high at its thickest. (At
/// `5ε` the vertex is in band of the plane, and the split refuses the
/// input there, `SliverVertex`.) Only 1e-9 witnesses the fix: at 1e-6 the
/// split builds at every `d` here with each end read off its vertex too,
/// so that arm is a regression row.
#[test]
fn a_sliver_just_off_the_rim_vertex_builds_and_the_halves_hold_the_volume() {
    let tol = Tol::witness();
    let eps = tol.eps();
    if eps < 1e-9 {
        test_utils::vacuity::stood_down(
            "ε below 1e-9",
            "at 1e-12 the sliver's wall stops at its floor: the next row",
        );
        return;
    }
    for d in [50.0 * eps, 500.0 * eps, 5000.0 * eps] {
        let parts = sliver_split(d, tol).unwrap_or_else(|e| panic!("d = {d:e}: {e:?}"));
        let (mut volume, mut pad) = (0.0, 0.0);
        for part in [&parts.above, &parts.below] {
            let body = part
                .body()
                .unwrap_or_else(|| panic!("d = {d:e}: a side is empty"));
            let props = topo::test_support::pinned_volume("a half", body, tol);
            volume += props.volume;
            pad += props.volume_pad;
        }
        assert!(
            (volume - core::f64::consts::PI).abs() <= pad,
            "d = {d:e}: the halves sum to {volume} ± {pad}, not π"
        );
    }
}

/// **At 1e-12 the sliver builds, and its wall's measurement stops at
/// its floor with the bracket.** The at-rest gate decides the halves'
/// signs and reads no number, so the sliver is no longer refused at rest
/// (it was, `QuadratureFloor` after one round, while a number door's
/// unmet resolution was a refusal). Measured at the suites' pinned
/// resolution, the thick half meets it and the sliver stops at
/// [`topo::Stop::Floor`]: its wall's width is the part refinement does
/// not aim at, over the `1024·ε` resolution. The two brackets together
/// hold the cylinder's `π`.
#[test]
fn at_1e_12_the_sliver_builds_and_its_wall_stops_at_its_floor() {
    let tol = Tol::witness();
    if (tol.eps() - 1e-12).abs() > 1e-15 {
        test_utils::vacuity::stood_down(
            "ε other than 1e-12",
            "the sliver meets the resolution at 1e-9 and 1e-6: the previous row",
        );
        return;
    }
    for d in [5e-11, 1e-9] {
        let parts = sliver_split(d, tol).unwrap_or_else(|e| panic!("d = {d:e}: {e:?}"));
        let (mut volume, mut pad, mut stops) = (0.0, 0.0, Vec::new());
        for part in [&parts.above, &parts.below] {
            let body = part
                .body()
                .unwrap_or_else(|| panic!("d = {d:e}: a side is empty"));
            let m = topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol))
                .unwrap_or_else(|e| panic!("d = {d:e}: a half measures, never refuses: {e:?}"));
            volume += m.enclosure.volume;
            pad += m.enclosure.volume_pad;
            stops.push(m.stop);
        }
        stops.sort_by_key(|s| s.word());
        assert_eq!(
            stops,
            [topo::Stop::Floor, topo::Stop::Met],
            "d = {d:e}: the sliver stops at its floor, the thick half meets the resolution"
        );
        assert!(
            (volume - core::f64::consts::PI).abs() <= pad,
            "d = {d:e}: the halves sum to {volume} ± {pad}, not π"
        );
    }
}
