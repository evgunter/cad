//! **An annulus rim's host OUTER boundary is metered in closed form.**
//!
//! The carve excises the host strip between a closed rim and its host
//! trim, so every other edge of the host's outer cycle must lie wholly
//! beyond the trim. Predicate 2's screen reads each boundary pair at
//! its sample stations only, and on these bodies the closest approach
//! falls between them: the screen passes, and before the host-boundary
//! meter each of the four refusing rows below carved a tier-3-valid
//! body whose band spans the notch or the cut. Each row pins the exact
//! backstop's reading (`fillet3_ring_clearance`) and carves its twin on
//! the other side of the same zero.
//!
//! - a washer whose bottom annulus carries a box notch from the BORE
//!   side: the outer rim's trim CONTAINS the boundary, so the notch's
//!   farthest corner decides;
//! - the same washer notched from the OUTSIDE: the inner rim's trim
//!   lies outside its rim, so the notch's nearest reach decides;
//! - a cone-and-cylinder shaft whose cylinder wall a tilted cut leaves
//!   an ELLIPSE for a far boundary: a curved host, read along the axis
//!   — and its twin with the cylinder on the MATE side, whose trim the
//!   carve's strip ends at just as the host's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Mat3, Point2, Sign, Tol, Vec3};
use profile::SketchPlane;
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{prism_on, revolved_about_y, rim_arcs_at};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .expect("the subtraction runs")
    .body()
    .expect("the subtraction leaves a body")
    .body
    .clone()
}

fn moved(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, m, tol()).expect("a rigid motion")
}

/// The notch's tangential half-width, meters.
const HALF: f64 = 0.3;
/// The notch's azimuth from the rim vertex: midway between two of the
/// rim's samples, which sit every 45° from it.
const NOTCH_AZ: f64 = 22.5;

/// A washer `r ∈ [1, 2]`, `y ∈ [0, 1]` about the `y` axis — every wall a
/// one-edge revolution wall, the bottom annulus carrying both bottom
/// rims and its seam in one cycle — with a box notch spanning radii
/// `[from, to]` and `±HALF` tangentially at azimuth [`NOTCH_AZ`], cut
/// through the whole height.
fn notched_washer(from: f64, to: f64) -> Body<f64> {
    let washer = revolved_about_y(
        vec![
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0, 1.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let (c, s) = (NOTCH_AZ.to_radians().cos(), NOTCH_AZ.to_radians().sin());
    // A `zx` sketch point is world `(x, z)` written `(z, x)`.
    let at = |r: f64, t: f64| (Point2::new(r * s + t * c, r * c - t * s), 0.0);
    let block = prism_on(
        SketchPlane::zx(),
        vec![at(from, -HALF), at(to, -HALF), at(to, HALF), at(from, HALF)],
        2.0,
        tol(),
    );
    subtract(
        &washer,
        &moved(&block, &Affine3::translation(Vec3::new(0.0, -0.5, 0.0))),
    )
}

/// The definite `fillet3_ring_clearance` refusal a request reads, as
/// `(reading, bounded)`.
fn refusal(name: &str, err: BlendError) -> (f64, bool) {
    let BlendError::RingClearance {
        margin, bounded, ..
    } = err
    else {
        panic!("{name}: the support-boundary meter answers, not the sampled screen; got {err:?}")
    };
    assert_eq!(margin.sign, Sign::Negative, "{name}: a definite refusal");
    let read = margin
        .reading
        .diagnostic_f64_for_error_text()
        .value()
        .expect("a definite reading");
    (read, bounded)
}

fn carves(name: &str, body: &Body<f64>, rim: (f64, f64), r: f64) {
    validate_geometric(body, tol()).expect("the fixture is tier-3 valid");
    let arcs = rim_arcs_at(body, rim.0, rim.1);
    let out =
        fillet_edges(body, &arcs, r, tol()).unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{name}: tier-3 valid, {e:?}"));
    assert_eq!(out.band_faces.len(), 1, "{name}: one band");
}

/// The reading of a refusal, and whether it was a bound (an ellipse
/// edge's certified box) rather than a measurement (a line or circle).
fn refuses(name: &str, body: &Body<f64>, rim: (f64, f64), r: f64, bounded: bool) -> f64 {
    validate_geometric(body, tol()).expect("the fixture is tier-3 valid");
    let arcs = rim_arcs_at(body, rim.0, rim.1);
    let err = fillet_edges(body, &arcs, r, tol()).expect_err(name).error;
    let (read, was) = refusal(name, err);
    assert_eq!(
        was, bounded,
        "{name}: the refusal says whether it measured or bounded"
    );
    read
}

/// **The outer rim's trim contains the boundary: the notch's farthest
/// corner decides.** At `r = 0.2` the bottom outer rim's plane trim has
/// radius `1.8`; a notch reaching radius `1.85` puts its outer corners
/// at `√(1.85² + 0.3²) ≈ 1.874`, so the margin is `1.8 − 1.874`. Those
/// corners sit `22.5° ± 9.2°` round from the rim vertex, off the rim's
/// 45° sample lattice, where the sampled gap is `≈ 0.76`. Reaching
/// `1.7` instead clears by `1.8 − √(1.7² + 0.3²) ≈ +0.074`.
#[test]
fn a_notch_reaching_past_the_outer_rims_trim_between_samples_refuses() {
    let read = refuses(
        "a notch corner past the trim",
        &notched_washer(0.6, 1.85),
        (2.0, 0.0),
        0.2,
        false,
    );
    let want = 1.8 - 1.85f64.hypot(HALF);
    assert!(
        (read - want).abs() < 1e-12,
        "the containment reading `si − |corner|` (read {read}, derived {want})"
    );
    carves(
        "a notch inside the trim",
        &notched_washer(0.6, 1.7),
        (2.0, 0.0),
        0.2,
    );
}

/// **The inner rim's trim lies outside its rim: the notch's nearest
/// reach decides.** At `r = 0.2` the bottom bore rim's plane trim has
/// radius `1.2`; a notch from outside down to radius `1.15` crosses it
/// along its inner side (nearest `1.15`, mid-segment) and its two
/// radial sides (nearest at their inner corners, `√(1.15² + 0.3²)`).
/// Whichever of the three the cycle reaches first is the reading. Down
/// to `1.25` instead, every reach clears.
#[test]
fn a_notch_reaching_inside_the_bore_rims_trim_between_samples_refuses() {
    let read = refuses(
        "a notch past the trim",
        &notched_washer(1.15, 2.5),
        (1.0, 0.0),
        0.2,
        false,
    );
    let side = 1.15f64.hypot(HALF) - 1.2;
    let inner = 1.15 - 1.2;
    assert!(
        (read - side).abs() < 1e-12 || (read - inner).abs() < 1e-12,
        "the reach reading `|nearest| − si` of a side ({side}) or the inner side ({inner}), \
         read {read}"
    );
    carves(
        "a notch outside the trim",
        &notched_washer(1.25, 2.5),
        (1.0, 0.0),
        0.2,
    );
}

/// Which wall of [`tilted_cut_shaft`] the tilted plane cuts.
#[derive(Clone, Copy)]
enum Cut {
    /// A cone from radius `0.5` at `y = 0` out to `1` at `y = 0.5`, then
    /// a cylinder of radius 1 up to `y = 3`, cut from ABOVE; the rim's
    /// HOST is the cylinder.
    HostFromAbove,
    /// A cylinder of radius 1 from `y = −2` up to `0.5`, then a cone in
    /// to radius `0.5` at `y = 1`, cut from BELOW; the rim's host is the
    /// cone, and the cylinder is its MATE.
    MateFromBelow,
}

/// A shaft about `y` whose rim at radius 1, height `0.5` joins a cone
/// and a cylinder, with the cylinder wall cut by a plane tilted 45°:
/// its extreme point on the cylinder at height `level` and azimuth
/// `180° + spin` ([`Cut`] says which wall is which).
fn tilted_cut_shaft(cut: Cut, level: f64, spin: f64) -> Body<f64> {
    let profile = match cut {
        Cut::HostFromAbove => [(0.0, 0.0), (0.5, 0.0), (1.0, 0.5), (1.0, 3.0), (0.0, 3.0)],
        Cut::MateFromBelow => [(0.0, -2.0), (1.0, -2.0), (1.0, 0.5), (0.5, 1.0), (0.0, 1.0)],
    };
    let mut shaft = revolved_about_y(
        profile
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
        Revolution::Full,
        tol(),
    );
    shaft
        .merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    // A block whose underside, once tilted, is the cutting plane; its
    // near edge stays outside the shaft so only that plane cuts it.
    let w = |x: f64, z: f64| (Point2::new(z, x), 0.0);
    let block = prism_on(
        SketchPlane::zx(),
        vec![w(-1.42, -5.0), w(5.0, -5.0), w(5.0, 5.0), w(-1.42, 5.0)],
        5.0,
        tol(),
    );
    let q = core::f64::consts::FRAC_1_SQRT_2;
    let tilt = Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(q, q, 0.0),
            Vec3::new(-q, q, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let (c, s) = (spin.to_radians().cos(), spin.to_radians().sin());
    let turn = Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(c, 0.0, -s),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(s, 0.0, c),
        ),
        Vec3::new(0.0, 0.0, 0.0),
    );
    // Turned half a turn about `z`, the block lies BELOW its tilted top
    // face instead, its highest point on the cylinder where the lowest
    // was.
    let half_turn = Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let (block, lift) = match cut {
        Cut::HostFromAbove => (moved(&block, &tilt), level + 1.0),
        Cut::MateFromBelow => (moved(&moved(&block, &tilt), &half_turn), level - 1.0),
    };
    subtract(
        &shaft,
        &moved(
            &moved(&block, &turn),
            &Affine3::translation(Vec3::new(0.0, lift, 0.0)),
        ),
    )
}

/// **A curved host's far boundary is read along the axis, and an
/// ellipse through its whole-carrier range.** The cone–cylinder rim at
/// `y = 0.5` has its host trim on the cylinder one setback `r·tan(π/8)`
/// up (the wall turns 45° there). The tilted cut leaves the cylinder an
/// ellipse whose lowest point is at `level`; turned 11.25° off the
/// samples, the sampled gap overstates it by `≈ 0.019`, so at
/// `level = 0.535` the screen passes while the true margin is
/// `(0.535 − 0.5) − 0.1·tan(π/8) ≈ −0.0064`. At `0.56` it clears.
#[test]
fn a_tilted_cut_reaching_a_cylinder_hosts_trim_between_samples_refuses() {
    let read = refuses(
        "an ellipse past the trim",
        &tilted_cut_shaft(Cut::HostFromAbove, 0.535, 191.25),
        (1.0, 0.5),
        0.1,
        true,
    );
    let want = (0.535 - 0.5) - 0.1 * core::f64::consts::FRAC_PI_8.tan();
    assert!(
        (read - want).abs() < 1e-12,
        "the height reading `low − trim` (read {read}, derived {want})"
    );
    carves(
        "an ellipse beyond the trim",
        &tilted_cut_shaft(Cut::HostFromAbove, 0.56, 191.25),
        (1.0, 0.5),
        0.1,
    );
}

/// **The MATE support's boundary is the same question.** The shaft's
/// twin with the cylinder BELOW the rim makes it the mate, whose trim
/// runs one setback `r·tan(π/8)` down; a tilted cut from below whose
/// highest point on the cylinder is `0.465`, turned off the samples as
/// above, passes the screen while the true margin is
/// `(0.5 − 0.1·tan(π/8)) − 0.465 ≈ −0.0064`. At `0.44` it clears.
#[test]
fn a_tilted_cut_reaching_a_cylinder_mates_trim_between_samples_refuses() {
    let read = refuses(
        "an ellipse past the mate trim",
        &tilted_cut_shaft(Cut::MateFromBelow, 0.465, 191.25),
        (1.0, 0.5),
        0.1,
        true,
    );
    let want = (0.5 - 0.1 * core::f64::consts::FRAC_PI_8.tan()) - 0.465;
    assert!(
        (read - want).abs() < 1e-12,
        "the height reading `trim − high` (read {read}, derived {want})"
    );
    carves(
        "an ellipse beyond the mate trim",
        &tilted_cut_shaft(Cut::MateFromBelow, 0.44, 191.25),
        (1.0, 0.5),
        0.1,
    );
}
