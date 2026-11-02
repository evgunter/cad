//! **A saddle cut by its tangent plane** (probe): measures whether the
//! hyperbolic paraboloid `y = x·z`, cut by its tangent plane `y = 0`,
//! reaches the join. Every op refuses before it today (the spline ×
//! plane pair), so the row is ignored; run it with
//! `cargo nextest run -p sweep --test all a_saddle_cut_by_its_tangent_plane --run-ignored all`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region};
use geom_core::{Point2, Point3, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::{finished, stacked_at};
use topo::{AtRestBody, BooleanResult, intersect, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

/// The block `x, z ∈ [−1, 1]`, `−2 ≤ y ≤ x·z`: a loft along `z` whose
/// top wall is the hyperbolic paraboloid `y = x·z`.
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

fn the_block() -> Region {
    Box::new(|p: Point3<f64>| {
        (1.0 - p.x.abs())
            .min(1.0 - p.z.abs())
            .min(p.y + 2.0)
            .min(p.x * p.z - p.y)
    })
}

fn above() -> AtRestBody<f64> {
    finished(
        "the half-space",
        topo::test_support::brick((-4.0, 4.0), (0.0, 4.0), (-4.0, 4.0), tol()),
        tol(),
    )
}

fn the_above() -> Region {
    Box::new(|p: Point3<f64>| {
        (4.0 - p.x.abs())
            .min(4.0 - p.z.abs())
            .min(p.y)
            .min(4.0 - p.y)
    })
}

#[test]
#[ignore = "probe: the spline × plane pair refuses before the join"]
fn a_saddle_cut_by_its_tangent_plane_builds_every_op_at_its_closed_form() {
    let (s, h) = (saddle_block(), above());
    let (vs, vh, share) = (8.0, 256.0, 0.5);
    let lattice = (
        Point3::new(-1.25, -2.25, -1.25),
        Point3::new(1.25, 1.25, 1.25),
    );
    let both = || -> Region {
        let (a, b) = (the_block(), the_above());
        Box::new(move |p| a(p).min(b(p)))
    };
    let either = || -> Region {
        let (a, b) = (the_block(), the_above());
        Box::new(move |p| a(p).max(b(p)))
    };
    let s_less = || -> Region {
        let (a, b) = (the_block(), the_above());
        Box::new(move |p| a(p).min(-b(p)))
    };
    let h_less = || -> Region {
        let (a, b) = (the_block(), the_above());
        Box::new(move |p| b(p).min(-a(p)))
    };
    let mut fails = vec![];
    for (op, result, want, region) in [
        ("s ∩ h", intersect(&s, &h, tol()), share, both()),
        ("h ∩ s", intersect(&h, &s, tol()), share, both()),
        ("s ∪ h", union(&s, &h, tol()), vs + vh - share, either()),
        ("h ∪ s", union(&h, &s, tol()), vs + vh - share, either()),
        ("s ∖ h", subtract(&s, &h, tol()), vs - share, s_less()),
        ("h ∖ s", subtract(&h, &s, tol()), vh - share, h_less()),
    ] {
        match result {
            Ok(BooleanResult::Body(b)) => {
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    membership::holds_short_of_tier_3_prime(op, &b.body, want, &region, lattice, 7)
                }));
                if r.is_err() {
                    fails.push(format!("{op}: membership/volume"));
                }
            }
            Ok(BooleanResult::Empty) => fails.push(format!("{op}: empty")),
            Err(e) => fails.push(format!("{op}: {e:?}")),
        }
    }
    assert!(fails.is_empty(), "{fails:#?}");
}
