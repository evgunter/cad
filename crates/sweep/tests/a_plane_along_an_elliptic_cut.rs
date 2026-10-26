//! A cylinder cut obliquely, so its cap is an ellipse with semi-axes
//! 2.5 and 0.5 whose major ends sit on the wall's two rulings, then
//! split again by a plane through one major end that holds the
//! ellipse's tangent there. At that vertex both elliptic edges depart in
//! the plane, and the split reads their side by how they bend
//! (`geom_brep::conic_bend_side`, the boolean's reading too).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::Bracket;
use geom_core::{Point2, Point3, Sign, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::splitting::{SplitPart, split};
use topo::{AtRestBody, mass_properties, validate_closed};

const RADIUS: f64 = 0.5;
const DEPTH: f64 = 6.0;
/// The cut's tilt off the axis: `cos θ = 1/5`, so the ellipse's major
/// semi-axis is five times its minor.
const COS_TILT: f64 = 0.2;

/// The relative slack a volume carries: the mass properties of a
/// curved piece are read to the run's ε, not to rounding.
fn within(tol: Tol) -> f64 {
    1e-9_f64.max(100.0 * tol.eps())
}

/// The piece below the oblique cut through the cylinder's middle, its
/// measured volume, and the cap ellipse's major end on the `+x` ruling.
fn lower_piece(tol: Tol) -> (AtRestBody<f64>, f64, Point3<f64>) {
    let lp = bulge_loop(vec![
        (Point2::new(-RADIUS, 0.0), 1.0),
        (Point2::new(RADIUS, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let cylinder = extrude(
        &profile,
        Extrusion::Distance {
            depth: DEPTH,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    let body = topo::test_support::finished("the cylinder", cylinder, tol);
    let theta = COS_TILT.acos();
    let cut = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, DEPTH / 2.0),
        Vec3::new(-theta.sin(), 0.0, theta.cos()),
        tol,
    );
    let lower = split(&body, &cut, tol)
        .expect("the oblique cut answers")
        .below
        .body()
        .expect("material below the cut")
        .clone();
    let volume = mass_properties(&lower, tol, topo::test_support::pinned_resolution(tol))
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume;
    let want = core::f64::consts::PI * RADIUS * RADIUS * DEPTH / 2.0;
    assert!(
        (volume - want).abs() <= within(tol) * want,
        "the lower piece holds half the cylinder: {volume}, want {want}"
    );
    let apex = Point3::new(RADIUS, 0.0, DEPTH / 2.0 + RADIUS * theta.tan());
    (lower, volume, apex)
}

/// Splits `piece` by the plane through `apex` along `normal`, and holds
/// the whole piece to the Below side, closed and of `volume`. Returns
/// the verdicts the split decided.
fn lands_whole_below(
    label: &str,
    piece: &AtRestBody<f64>,
    volume: f64,
    apex: Point3<f64>,
    normal: Vec3<f64>,
    tol: Tol,
) -> Vec<geom_core::k_stats::Verdict> {
    let plane = topo::test_support::split_plane(apex, normal, tol);
    let bracket = Bracket::open();
    let out = split(piece, &plane, tol);
    let verdicts = bracket.finish().verdicts;
    let r = out.unwrap_or_else(|e| panic!("{label}: the split refused: {e:?}"));
    assert!(
        matches!(r.above, SplitPart::Empty),
        "{label}: nothing lands Above"
    );
    let below = r.below.body().expect("the piece lands Below");
    assert_eq!(validate_closed(below), Ok(()), "{label}: Below is closed");
    let got = mass_properties(below, tol, topo::test_support::pinned_resolution(tol))
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume;
    assert!(
        (got - volume).abs() <= within(tol) * volume,
        "{label}: Below holds the whole piece: {got}, want {volume}"
    );
    verdicts
}

/// A plane through the major end, turned about the ellipse's tangent
/// there by `±0.4·ε/a` off the cap: every point of the cap lies within
/// `0.8·ε` of it, so both elliptic edges lie in the plane to the band
/// and read On. The ellipse's curvature at its major end is `a/b² =
/// 10`, so its curvature alone, levered at the edge's extent (the
/// major diameter), stands `0.8·ε·(a/b)² = 20·ε` off the plane; read
/// that way the edges would take a side the cap does not have.
#[test]
fn a_plane_within_the_band_of_an_elliptic_cap_reads_its_edges_on() {
    let tol = Tol::witness();
    let (piece, volume, apex) = lower_piece(tol);
    let a = RADIUS / COS_TILT;
    for s in [-0.4, 0.4] {
        let label = format!("turned by {s}·ε/a");
        let phi = COS_TILT.acos() + (s * tol.eps() / a).asin();
        let verdicts = lands_whole_below(
            &label,
            &piece,
            volume,
            apex,
            Vec3::new(-phi.sin(), 0.0, phi.cos()),
            tol,
        );
        let flat = verdicts
            .iter()
            .filter(|v| v.predicate == "conic_flat" && v.sign == Sign::Zero)
            .count();
        assert!(
            flat >= 2,
            "{label}: both elliptic edges at the major end read within the band ({flat} did)"
        );
        let sided: Vec<_> = verdicts
            .iter()
            .filter(|v| v.predicate == "conic_bend_side")
            .collect();
        assert!(
            sided.is_empty(),
            "{label}: no elliptic edge takes a side: {sided:?}"
        );
    }
}

/// The vertical plane tangent to the wall along the `+x` ruling: the
/// ellipse departs its major end along `y`, in the plane, and bends
/// back toward `−x` into the material, so both elliptic edges read
/// Below and the piece lands whole there.
#[test]
fn a_plane_tangent_along_the_ruling_reads_the_elliptic_edges_by_their_bend() {
    let tol = Tol::witness();
    let (piece, volume, apex) = lower_piece(tol);
    let verdicts = lands_whole_below(
        "tangent along the ruling",
        &piece,
        volume,
        apex,
        Vec3::new(1.0, 0.0, 0.0),
        tol,
    );
    let below = verdicts
        .iter()
        .filter(|v| v.predicate == "conic_bend_side" && v.sign == Sign::Negative)
        .count();
    assert!(
        below >= 2,
        "both elliptic edges at the major end bend Below ({below} did)"
    );
}
