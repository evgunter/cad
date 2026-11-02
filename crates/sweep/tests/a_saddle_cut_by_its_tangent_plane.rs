//! **A saddle cut by its tangent plane refuses before the join.** The
//! block `x, z ∈ [−1, 1]`, `−2 ≤ y ≤ x·z`, lofted along `z`, has the
//! hyperbolic paraboloid `y = x·z` for its top wall; the half-space
//! `y ≥ 0` is tangent to it at the origin and cuts it along the lines
//! `x = 0` and `y = 0`, a straight section of two lines crossing at a
//! point. Two smooth carriers meet in two crossing lines only at a
//! tangency, so the pair is a spline × plane tangent section, which no
//! lane builds yet: each op refuses typed at the spline × plane pair,
//! long before the join pairs germs across the crossing. Its closed
//! forms (the block `8`, its part above the plane `1/2`) are the row
//! `work/spliceface/a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form.md`
//! asks for.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind::{Nurbs, Plane};
use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::{finished, stacked_at};
use topo::{AtRestBody, BooleanError, PairRefusalSite, intersect, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

/// The saddle block (module docs).
fn saddle_block() -> AtRestBody<f64> {
    let section = |s: f64| {
        let p = |x: f64, y: f64| (Point2::new(x, y), 0.0);
        vec![bulge_loop(vec![
            p(-1.0, -2.0),
            p(1.0, -2.0),
            p(1.0, s),
            p(-1.0, -s),
        ])]
    };
    let lofted = sweep::loft_body(
        &[section(-1.0), section(1.0)],
        &stacked_at(&[-1.0, 1.0]),
        1,
        tol(),
    )
    .expect("the saddle block lofts");
    finished("the saddle block", lofted.body, tol())
}

/// The half-space `y ≥ 0`, as a brick with a face on its plane.
fn above() -> AtRestBody<f64> {
    finished(
        "the half-space",
        topo::test_support::brick((-4.0, 4.0), (0.0, 4.0), (-4.0, 4.0), tol()),
        tol(),
    )
}

/// **Every op, either way round, refuses at the spline × plane pair**:
/// intersect and subtract at the revert roster, union at the held
/// pierce of the saddle's wall on the plane.
#[test]
fn a_saddle_cut_by_its_tangent_plane_refuses_at_the_spline_plane_pair() {
    let (s, h) = (saddle_block(), above());
    let roster = |r: &Result<topo::BooleanResult<f64>, BooleanError>| {
        matches!(
            r,
            Err(BooleanError::CurvedPairUnsupported {
                site: PairRefusalSite::RevertRoster,
                kind: Nurbs,
                other_kind: Plane,
                ..
            })
        )
    };
    let pierce = |r: &Result<topo::BooleanResult<f64>, BooleanError>| {
        matches!(
            r,
            Err(BooleanError::HeldPierceUnbuilt {
                kind: Nurbs,
                pierced_kind: Plane,
                ..
            })
        )
    };
    for (op, r, at_pierce) in [
        ("s ∩ h", intersect(&s, &h, tol()), false),
        ("h ∩ s", intersect(&h, &s, tol()), false),
        ("s ∖ h", subtract(&s, &h, tol()), false),
        ("h ∖ s", subtract(&h, &s, tol()), false),
        ("s ∪ h", union(&s, &h, tol()), true),
        ("h ∪ s", union(&h, &s, tol()), true),
    ] {
        let refused = if at_pierce { pierce(&r) } else { roster(&r) };
        assert!(
            refused,
            "{op}: refuses typed at the spline × plane pair, not {r:?}"
        );
    }
}
