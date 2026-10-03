//! **A boundary edge requested in the same call as a closed rim is
//! metered where its own carve leaves the rim's support.**
//!
//! Predicate 2's screen reads every pair of a support's boundary
//! features at their sample stations, less both setbacks; the exact
//! readers behind it answer the same question in closed form. Three
//! co-requested pairs, one row each:
//!
//! - a RULED link lying in an annulus rim's host outer cycle: a rod on
//!   a washer's bottom face whose ends are transverse caps. Its closest
//!   approach to the rim falls between the screen's stations, so the
//!   support-boundary meter is what refuses — reading the rod's
//!   TRIMLINE, not its stored ruling;
//! - a box edge in a LADDER host's outer cycle: the ring arm already
//!   meters the edge's trimline against the rim's widened ring, exactly;
//! - two coaxial rims on one shared revolution WALL: the wall's seam
//!   meridian puts a vertex of each rim on one azimuth, a station of
//!   both, so the screen reads the closest approach itself.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Mat3, Point2, Sign, Tol, Vec3};
use profile::SketchPlane;
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{brick, prism_at, prism_on, revolved_about_y, rim_arcs_at};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn boolean(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    boolean_op_with(
        op,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .expect("the boolean runs")
    .body()
    .expect("the boolean leaves a body")
    .body
    .clone()
}

fn moved(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, m, tol()).expect("a rigid motion")
}

/// A block standing on the `zx` sketch from `y = −0.5` to `1.5`, over
/// the polygon `pts` given as world `(x, z)` pairs.
fn upright(pts: &[(f64, f64)]) -> Body<f64> {
    // A `zx` sketch point is world `(x, z)` written `(z, x)`.
    let loop_: Vec<_> = pts.iter().map(|&(x, z)| (Point2::new(z, x), 0.0)).collect();
    let block = prism_on(SketchPlane::zx(), loop_, 2.0, tol());
    moved(&block, &Affine3::translation(Vec3::new(0.0, -0.5, 0.0)))
}

fn ends(body: &Body<f64>, e: EdgeKey) -> [Vec3<f64>; 2] {
    let he = body.get_edge(e).unwrap().he_plus;
    let at = |v| {
        let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        Vec3::new(p.x, p.y, p.z)
    };
    [
        at(body.get_half_edge(he).unwrap().start),
        at(body.half_edge_end(he).unwrap()),
    ]
}

/// The one edge whose two ends both satisfy `on`.
fn the_edge(body: &Body<f64>, on: impl Fn(Vec3<f64>) -> bool) -> EdgeKey {
    let hits: Vec<EdgeKey> = body
        .edges()
        .map(|(k, _)| k)
        .filter(|&k| {
            let [a, b] = ends(body, k);
            on(a) && on(b) && (a - b).norm() > 1e-6
        })
        .collect();
    let [one] = hits[..] else {
        panic!("one edge, got {hits:?}")
    };
    one
}

/// A definite refusal's reading.
fn reading(m: &sweep::blend::ClassifiedMargin) -> f64 {
    assert_eq!(m.sign, Sign::Negative, "a definite refusal");
    m.reading
        .diagnostic_f64_for_error_text()
        .value()
        .expect("a definite reading")
}

// ---- The rod on the washer ----

/// The rod's normal azimuth: midway between two of the bore rim's
/// stations, which sit every 45° from its vertex at azimuth 0.
const ROD_AZ: f64 = 112.5;
/// The rod's nearest ruling, its distance from the axis.
const NEAR: f64 = 1.4;
/// The rod's radius.
const ROD: f64 = 0.34;

/// A washer `r ∈ [1, 3]`, `y ∈ [0, 1]` about `y`, cut to a square of
/// half-side 2.1 whose sides face azimuths `22.5° + 90°k`, with a round
/// rod of radius [`ROD`] lying along its bottom face: axis at `y = 0`,
/// parallel to the side facing [`ROD_AZ`], its near ruling [`NEAR`]
/// from the axis, cut off by two planes perpendicular to it at
/// `±2.0` along it. The bottom face carries the bore rim, its seam and
/// the rod's near ruling in one outer cycle, and both of the rod's
/// ends are transverse caps.
fn rodded_washer() -> Body<f64> {
    let washer = revolved_about_y(
        vec![
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(3.0, 0.0), 0.0),
            (Point2::new(3.0, 1.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let polar = |r: f64, az: f64| {
        let a = az.to_radians();
        (r * a.cos(), r * a.sin())
    };
    let half = 2.1 * 2f64.sqrt();
    let square = upright(&[
        polar(half, 67.5),
        polar(half, 157.5),
        polar(half, 247.5),
        polar(half, 337.5),
    ]);
    let squared = boolean(BooleanOp::Intersect, &washer, &square);
    let a = ROD_AZ.to_radians();
    let (n, t) = (
        Vec3::new(a.cos(), 0.0, a.sin()),
        Vec3::new(-a.sin(), 0.0, a.cos()),
    );
    let cx = NEAR + ROD;
    let rod = prism_at(
        vec![
            (Point2::new(cx - ROD, 0.0), 1.0),
            (Point2::new(cx + ROD, 0.0), 1.0),
        ],
        -4.0,
        8.0,
        tol(),
    );
    let rod = moved(
        &rod,
        &Affine3::from_parts(
            Mat3::from_cols(n, Vec3::new(0.0, 1.0, 0.0), t),
            Vec3::new(0.0, 0.0, 0.0),
        ),
    );
    let joined = boolean(BooleanOp::Union, &squared, &rod);
    let at = |nn: f64, tt: f64| {
        let p = n * nn + t * tt;
        (p.x, p.z)
    };
    let caps = upright(&[at(-5.0, -2.0), at(5.0, -2.0), at(5.0, 2.0), at(-5.0, 2.0)]);
    boolean(BooleanOp::Intersect, &joined, &caps)
}

fn near_ruling(body: &Body<f64>) -> EdgeKey {
    let a = ROD_AZ.to_radians();
    the_edge(body, |p| {
        p.y.abs() < 1e-9 && (p.x * a.cos() + p.z * a.sin() - NEAR).abs() < 1e-9
    })
}

/// **A ruled link co-requested with an annulus rim is metered at its
/// trimline.** The concave ruled band's trim on the bottom face lies
/// `√((ρ + r)² − r²) − ρ` in from the ruling, toward the bore, and the
/// bore rim's trim circle has radius `1 + r`. At `r = 0.25` they cross:
/// the margin is `NEAR − setback − (1 + r) ≈ −0.044`. The closest
/// approach sits at azimuth 112.5°, 22.5° off the rim's stations, where
/// the sampled gap clears both setbacks; read at the ruling's STORED
/// place the reach is `NEAR − (1 + r) = +0.15`, and the call carved a
/// tier-3-valid body whose bottom face boundary crosses itself. At
/// `r = 0.22` the two trims clear by `≈ 0.005` and the call carves.
#[test]
fn a_ruled_link_co_requested_with_an_annulus_rim_is_metered_at_its_trimline() {
    let body = rodded_washer();
    validate_geometric(&body, tol()).expect("the fixture is tier-3 valid");
    let request = |r: f64| {
        let mut edges = rim_arcs_at(&body, 1.0, 0.0);
        assert_eq!(edges.len(), 1, "the bore rim is one closed edge");
        edges.push(near_ruling(&body));
        fillet_edges(&body, &edges, r, tol())
    };
    let r = 0.25;
    let err = request(r).expect_err("the trims cross").error;
    let BlendError::RingClearance {
        margin, bounded, ..
    } = err
    else {
        panic!("the support-boundary meter answers, not the sampled screen; got {err:?}")
    };
    assert!(!bounded, "a line read exactly");
    let setback = ((ROD + r).powi(2) - r * r).sqrt() - ROD;
    let want = NEAR - setback - (1.0 + r);
    let read = reading(&margin);
    assert!(
        (read - want).abs() < 1e-12,
        "the trimline's reach less the rim's trim radius (read {read}, derived {want})"
    );
    let out = request(0.22).unwrap_or_else(|e| panic!("clear trims carve, got {e:?}"));
    validate_geometric(&out.body, tol()).expect("the carve is tier-3 valid");
    assert_eq!(
        (out.band_faces.len(), out.blend_faces.len()),
        (1, 1),
        "the rim's band and the rod's"
    );
}

// ---- The ladder plate ----

/// **A ladder host's co-requested outer edge is metered exactly by the
/// ring arm.** A plate `[0, 6] × [0, 4] × [0, 1]` with a through hole of
/// radius 1.5 about `(4, 2)`, its two vertices at azimuths 11.25° and
/// 191.25°, so no station of the hole's top rim faces the `x = 6` side
/// 0.5 away. Every box edge and the rim together: at `r = 0.26` the
/// sampled gap clears `2r`, and the ring arm reads the side's trimline
/// against the rim's widened ring, `0.5 − 2r`. At `r = 0.24` it carves.
#[test]
fn a_ladder_hosts_co_requested_box_edge_is_metered_exactly_by_the_ring_arm() {
    let plate = brick::<f64>((0.0, 6.0), (0.0, 4.0), (0.0, 1.0), tol());
    let a = 11.25f64.to_radians();
    let (c, s) = (1.5 * a.cos(), 1.5 * a.sin());
    let hole = prism_at(
        vec![
            (Point2::new(4.0 + c, 2.0 + s), 1.0),
            (Point2::new(4.0 - c, 2.0 - s), 1.0),
        ],
        -0.5,
        2.0,
        tol(),
    );
    let body = boolean(BooleanOp::Subtract, &plate, &hole);
    validate_geometric(&body, tol()).expect("the fixture is tier-3 valid");
    let request = |r: f64| {
        let edges: Vec<EdgeKey> = body
            .edges()
            .map(|(k, _)| k)
            .filter(|&k| {
                let [p, q] = ends(&body, k);
                // The top rim's arcs, and every edge of the box.
                let rim = p.z == 1.0 && q.z == 1.0 && (p.x - 4.0).hypot(p.y - 2.0) < 1.6;
                let boxy = |v: Vec3<f64>| (v.x == 0.0 || v.x == 6.0) && (v.y == 0.0 || v.y == 4.0);
                rim || (boxy(p) && boxy(q))
            })
            .collect();
        assert_eq!(edges.len(), 14, "two rim arcs and twelve box edges");
        fillet_edges(&body, &edges, r, tol())
    };
    let r = 0.26;
    let err = request(r).expect_err("the trims cross").error;
    let BlendError::RingClearance { margin, .. } = err else {
        panic!("the ring arm answers, not the sampled screen; got {err:?}")
    };
    let read = reading(&margin);
    let want = 0.5 - 2.0 * r;
    assert!(
        (read - want).abs() < 1e-12,
        "the side's trimline against the widened ring (read {read}, derived {want})"
    );
    let out = request(0.24).unwrap_or_else(|e| panic!("clear trims carve, got {e:?}"));
    validate_geometric(&out.body, tol()).expect("the carve is tier-3 valid");
}

// ---- The washer's bore ----

/// **Two coaxial rims on one shared wall are read exactly by the
/// screen.** A washer `r ∈ [1, 2]`, `y ∈ [0, 1]`: both bore rims on the
/// one bore wall, whose seam meridian joins their vertices. The screen's
/// gap is the wall's height, `1`, read at that shared azimuth, and the
/// refusal at `r = 0.51` is `1 − 2r`; at `r = 0.49` both bands carve.
#[test]
fn two_coaxial_rims_on_a_shared_wall_are_read_exactly_by_the_screen() {
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
    let mut edges = rim_arcs_at(&washer, 1.0, 0.0);
    edges.extend(rim_arcs_at(&washer, 1.0, 1.0));
    assert_eq!(edges.len(), 2, "two one-edge bore rims");
    let r = 0.51;
    let err = fillet_edges(&washer, &edges, r, tol())
        .expect_err("the trims cross on the wall")
        .error;
    let BlendError::FaceClearanceUncertified { margin, gap, .. } = err else {
        panic!("the screen answers; got {err:?}")
    };
    let gap = gap
        .diagnostic_f64_for_error_text()
        .value()
        .expect("a definite gap");
    assert_eq!(gap, 1.0, "the gap is the wall's height, read at the seam");
    let read = reading(&margin);
    assert!(
        (read - (1.0 - 2.0 * r)).abs() < 1e-12,
        "the wall's height less both setbacks (read {read})"
    );
    let out = fillet_edges(&washer, &edges, 0.49, tol())
        .unwrap_or_else(|e| panic!("clear trims carve, got {e:?}"));
    validate_geometric(&out.body, tol()).expect("the carve is tier-3 valid");
    assert_eq!(out.band_faces.len(), 2, "both rims' bands");
}
