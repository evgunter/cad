//! Planes through a cylinder's cap vertex that hold a cap edge's
//! tangent there, so the edge departs in the plane and the split reads
//! its side by how it bends (`geom_brep::conic_bend_side`). Each side
//! read is held to `point_in_solid` over a grid of the operand, for
//! both orientations of the plane's normal.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::{Bracket, Verdict};
use geom_core::{Band, Point2, Point3, Sign, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::splitting::{SplitPart, split};
use topo::{AtRestBody, SolidContainment, point_in_solid};

const R: f64 = 0.5;
const DEPTH: f64 = 6.0;
/// The oblique cut's tilt off the axis: `cos θ = 1/5`, so the cap
/// ellipse's major semi-axis is five times its minor.
const COS_TILT: f64 = 0.2;

/// A cylinder of radius `R` along `+z`, its cap circle cut into arcs
/// at `points` with bulge `bulge` each.
fn cylinder(points: &[Point2<f64>], bulge: f64, tol: Tol) -> AtRestBody<f64> {
    let lp = bulge_loop(points.iter().map(|&p| (p, bulge)).collect());
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: DEPTH,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    topo::test_support::finished("the cylinder", body, tol)
}

/// A cylinder whose caps are two half circles.
fn half_arc_cylinder(tol: Tol) -> AtRestBody<f64> {
    cylinder(&[Point2::new(-R, 0.0), Point2::new(R, 0.0)], 1.0, tol)
}

/// The piece below an oblique cut through the cylinder's middle, and
/// its cap ellipse's major end on the `+x` ruling.
fn elliptic_piece(tol: Tol) -> (AtRestBody<f64>, Point3<f64>) {
    let theta = COS_TILT.acos();
    let cut = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, DEPTH / 2.0),
        Vec3::new(-theta.sin(), 0.0, theta.cos()),
        tol,
    );
    let lower = split(&half_arc_cylinder(tol), &cut, tol)
        .unwrap()
        .below
        .body()
        .unwrap()
        .clone();
    let apex = Point3::new(R, 0.0, DEPTH / 2.0 + R * theta.tan());
    (lower, apex)
}

/// Inside the cylinder, clear of its boundary.
fn inside_cylinder(p: Point3<f64>) -> bool {
    p.x * p.x + p.y * p.y < (R * 0.98).powi(2) && p.z > 0.02 && p.z < DEPTH - 0.02
}

/// Every grid point `inside` the operand and clear of the plane through
/// `origin` with normal `n` lies in the piece on its side and not in the
/// other; returns how many points were held.
fn hold_membership(
    label: &str,
    inside: &dyn Fn(Point3<f64>) -> bool,
    above: &SplitPart<f64>,
    below: &SplitPart<f64>,
    origin: Point3<f64>,
    n: Vec3<f64>,
    tol: Tol,
) -> usize {
    let band = Band::linear(tol).unwrap();
    let contains = |part: &SplitPart<f64>, p| match part.body() {
        Some(b) => point_in_solid(b, p, band, tol).unwrap(),
        None => SolidContainment::Out,
    };
    let mut held = 0;
    let (nx, ny, nz) = (5, 5, 10);
    for i in 0..nx {
        for j in 0..ny {
            for k in 0..nz {
                let p = Point3::new(
                    -R + (f64::from(i) + 0.5) * 2.0 * R / f64::from(nx),
                    -R + (f64::from(j) + 0.5) * 2.0 * R / f64::from(ny),
                    (f64::from(k) + 0.5) * DEPTH / f64::from(nz),
                );
                let d = (p - origin).dot(n);
                if !inside(p) || d.abs() < 1e-3 {
                    continue;
                }
                let (own, other) = if d > 0.0 {
                    (above, below)
                } else {
                    (below, above)
                };
                assert_eq!(
                    contains(own, p),
                    SolidContainment::In,
                    "{label}: {p:?} ({d:e} off the plane) is in the piece on its side"
                );
                assert_eq!(
                    contains(other, p),
                    SolidContainment::Out,
                    "{label}: {p:?} ({d:e} off the plane) is not in the other piece"
                );
                held += 1;
            }
        }
    }
    held
}

/// Splits `body` by the plane through `origin` with normal `n`, and the
/// verdicts the split read.
fn split_reading(
    body: &AtRestBody<f64>,
    origin: Point3<f64>,
    n: Vec3<f64>,
    tol: Tol,
) -> (topo::splitting::SplitResult<f64>, Vec<Verdict>) {
    let plane = topo::test_support::split_plane(origin, n, tol);
    let bracket = Bracket::open();
    let parts = split(body, &plane, tol).unwrap_or_else(|e| panic!("{n:?}: {e:?}"));
    (parts, bracket.finish().verdicts)
}

/// **The plane along the elliptic piece's ruling, either way up.** The
/// plane `x = R` touches the piece along its `+x` ruling and holds the
/// cap ellipse's tangent at the major end, where both elliptic edges
/// bend off it toward `−x`. With the normal along `+x` the piece lands
/// whole Below; flipped, whole Above.
#[test]
fn the_elliptic_piece_lands_on_the_side_of_its_bend_for_either_normal() {
    let tol = Tol::witness();
    let (piece, apex) = elliptic_piece(tol);
    let theta = COS_TILT.acos();
    let inside = |p: Point3<f64>| {
        inside_cylinder(p) && -theta.sin() * p.x + theta.cos() * (p.z - DEPTH / 2.0) < -0.02
    };
    for (n, lands_above) in [
        (Vec3::new(1.0, 0.0, 0.0), false),
        (Vec3::new(-1.0, 0.0, 0.0), true),
    ] {
        let (parts, verdicts) = split_reading(&piece, apex, n, tol);
        let (whole, empty, sign) = if lands_above {
            (&parts.above, &parts.below, Sign::Positive)
        } else {
            (&parts.below, &parts.above, Sign::Negative)
        };
        assert!(
            matches!(empty, SplitPart::Empty),
            "{n:?}: one side is empty"
        );
        assert!(whole.body().is_some(), "{n:?}: the piece lands whole");
        let bent = verdicts
            .iter()
            .filter(|v| v.predicate == "conic_bend_side" && v.sign == sign)
            .count();
        assert!(
            bent >= 2,
            "{n:?}: both elliptic edges bend {sign:?} ({bent} did)"
        );
        let held = hold_membership(
            "elliptic",
            &inside,
            &parts.above,
            &parts.below,
            apex,
            n,
            tol,
        );
        assert!(held > 30, "{n:?}: the grid held {held} points");
    }
}

/// **The plane along the circular cylinder's ruling, either way up.**
/// `x = R` touches the cylinder along a ruling and holds both cap
/// circles' tangents at its ends: the cylinder lands whole on the side
/// the circles bend to.
#[test]
fn the_cylinder_lands_on_the_side_of_its_bend_for_either_normal() {
    let tol = Tol::witness();
    let body = half_arc_cylinder(tol);
    let o = Point3::new(R, 0.0, 0.0);
    for (n, lands_above) in [
        (Vec3::new(1.0, 0.0, 0.0), false),
        (Vec3::new(-1.0, 0.0, 0.0), true),
    ] {
        let (parts, _) = split_reading(&body, o, n, tol);
        let (whole, empty) = if lands_above {
            (&parts.above, &parts.below)
        } else {
            (&parts.below, &parts.above)
        };
        assert!(
            matches!(empty, SplitPart::Empty),
            "{n:?}: one side is empty"
        );
        assert!(whole.body().is_some(), "{n:?}: the cylinder lands whole");
        let held = hold_membership(
            "ruling",
            &inside_cylinder,
            &parts.above,
            &parts.below,
            o,
            n,
            tol,
        );
        assert!(held > 30, "{n:?}: the grid held {held} points");
    }
}

/// **A plane through a cap vertex, tilted about the cap's tangent, cuts
/// the cylinder** with the cap arcs departing in it: each piece holds
/// exactly the operand's points on its side, for either normal.
#[test]
fn a_tilted_plane_through_a_cap_vertex_cuts_by_the_arcs_bend() {
    let tol = Tol::witness();
    let body = half_arc_cylinder(tol);
    let o = Point3::new(R, 0.0, 0.0);
    for alpha in [0.3_f64, 1.2] {
        for s in [1.0, -1.0] {
            let n = Vec3::new(alpha.cos(), 0.0, alpha.sin()) * s;
            let (parts, verdicts) = split_reading(&body, o, n, tol);
            assert!(
                verdicts.iter().any(|v| v.predicate == "conic_bend_side"),
                "{alpha} {s}: the in-plane arc departure is read by its bend"
            );
            let held = hold_membership(
                "tilted",
                &inside_cylinder,
                &parts.above,
                &parts.below,
                o,
                n,
                tol,
            );
            assert!(held > 30, "{alpha} {s}: the grid held {held} points");
        }
    }
}

/// **A plane tilted just off a quarter-arc cap.** The cylinder's caps
/// are four quarter arcs; the plane passes through the bottom cap's
/// `+x` vertex, holds the cap's tangent there, and tilts by `β` about
/// it so the cap's far points stand `R·sin β = m·ε` above. The whole
/// cylinder lands Above, touching the plane at that one vertex.
///
/// The arcs' lift is read at their arc length, where their rise is
/// exactly `R·sin β`; levered at their chord it falls to 0.82 of that,
/// inside the band for `m` just past it. And no arc whose far end
/// stands clear of the band reads as lying in the plane.
#[test]
fn a_plane_just_off_a_quarter_arc_cap_leaves_the_cylinder_above() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let q = (core::f64::consts::PI / 8.0).tan();
    let body = cylinder(
        &[
            Point2::new(R, 0.0),
            Point2::new(0.0, R),
            Point2::new(-R, 0.0),
            Point2::new(0.0, -R),
        ],
        q,
        tol,
    );
    let o = Point3::new(R, 0.0, 0.0);
    for m in [10.5, 11.0, 11.5, 12.0, 15.0] {
        let beta = (m * eps / R).asin();
        let (parts, verdicts) =
            split_reading(&body, o, Vec3::new(-beta.sin(), 0.0, beta.cos()), tol);
        assert!(
            matches!(parts.below, SplitPart::Empty) && parts.above.body().is_some(),
            "m = {m}: the cylinder lands whole Above"
        );
        assert!(
            verdicts
                .iter()
                .any(|v| v.predicate == "conic_bend_side" && v.sign == Sign::Positive),
            "m = {m}: the cap arcs bend Above"
        );
        assert!(
            !verdicts
                .iter()
                .any(|v| v.predicate == "conic_flat" && v.sign == Sign::Zero),
            "m = {m}: a cap arc standing {m}·ε off the plane does not lie in it"
        );
    }
}
