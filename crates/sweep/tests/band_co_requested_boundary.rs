//! **A boundary edge requested in the same call as a closed rim is
//! metered where its own carve leaves the rim's support.**
//!
//! Predicate 2's screen reads every pair of a support's boundary
//! features at their sample stations, less both setbacks; exact readers
//! behind it answer the same question in closed form. Which reader each
//! row pins:
//!
//! - ARM (A) of the ring pass, at a co-requested rim's trim (the two
//!   rod rows): a RULED link in the outer cycle of the washer's bottom
//!   face, whose ring is the bore rim, a rod whose ends are transverse
//!   caps. Its closest approach to the rim falls between the screen's
//!   stations, so the exact reader is what refuses: the rod's TRIMLINE
//!   against the bore's widened ring, the clearance the support-
//!   boundary walk reads from the rim's side at the rod's trim; the
//!   sweep row walks that boundary across azimuths, offsets and radii,
//!   and checks each carved body's trims against the closed form;
//! - ARM (A) of the ring pass (green either way): a box edge in a
//!   LADDER host's outer cycle, its trimline read against the rim's
//!   widened ring;
//! - THE SCREEN (green either way): two coaxial rims on one shared
//!   revolution wall, whose seam meridian puts a vertex of each rim on
//!   one azimuth, a station of both;
//! - ADMISSION AND THE RING PASS: an open plane–plane link on an
//!   annulus host's outer cycle refuses at a chain vertex where its
//!   chain turns at an unrequested corner, and carves where the whole
//!   square is requested, the bore's widened trim read against each
//!   trimline.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Mat3, Point2, Sign, Tol, Vec3};
use profile::SketchPlane;
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{brick, prism_at, prism_on, realized, revolved_about_y, rim_arcs_at};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn boolean(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    realized(op, a, b, tol())
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
fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol())
        .expect("mass properties")
        .volume
}

/// The volume a convex fillet of radius `r` removes along the washer's
/// bottom bore rim (radius 1, material outward): the spandrel's area
/// `r²(1 − π/4)` swept about the axis at its centroid's radius.
fn bore_rim_spandrel(r: f64) -> f64 {
    let area = r * r * (1.0 - PI / 4.0);
    let centroid = r * (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI));
    2.0 * PI * (1.0 + centroid) * area
}

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

/// One rod on the washer: its normal azimuth, its near ruling's distance
/// from the axis, and its radius.
#[derive(Clone, Copy, Debug)]
struct Rod {
    az: f64,
    near: f64,
    rho: f64,
}

const THE_ROD: Rod = Rod {
    az: ROD_AZ,
    near: NEAR,
    rho: ROD,
};

impl Rod {
    fn frame(self) -> (Vec3<f64>, Vec3<f64>) {
        let a = self.az.to_radians();
        (
            Vec3::new(a.cos(), 0.0, a.sin()),
            Vec3::new(-a.sin(), 0.0, a.cos()),
        )
    }

    /// The concave ruled band's setback on the bottom face: the ball
    /// rests at depth `r`, `ρ + r` from the rod's axis.
    fn setback(self, r: f64) -> f64 {
        ((self.rho + r).powi(2) - r * r).sqrt() - self.rho
    }

    /// The closed-form clearance between the rod's trimline, nearest
    /// the axis at its foot `t = 0`, and the bore rim's trim circle of
    /// radius `1 + r`.
    fn clearance(self, r: f64) -> f64 {
        self.near - self.setback(r) - (1.0 + r)
    }

    /// The section area the concave rod–plane fillet adds, its ball
    /// tangent to the rod and to the plane through the rod's axis: the
    /// triangle (rod axis, ball centre, plane contact) less the ball's
    /// sector and the rod's.
    fn fillet_area(self, r: f64) -> f64 {
        let rho = self.rho;
        let h = ((rho + r).powi(2) - r * r).sqrt();
        let alpha = (r / (rho + r)).acos();
        let beta = (r / (rho + r)).asin();
        h * r / 2.0 - alpha * r * r / 2.0 - beta * rho * rho / 2.0
    }

    /// The co-requested carve's volume change: the rod's fillet along
    /// its whole length between the caps at `t = ±2`, less the bore
    /// rim's spandrel ring.
    fn carve_dv(self, r: f64) -> f64 {
        4.0 * self.fillet_area(r) - bore_rim_spandrel(r)
    }
}

/// A washer `r ∈ [1, 3]`, `y ∈ [0, 1]` about `y`, cut to a square of
/// half-side 2.1 whose sides face azimuths `az + 90°k`, with a round rod
/// lying along its bottom face: axis at `y = 0`, parallel to the side
/// facing `az`, its near ruling `near` from the axis, cut off by planes
/// perpendicular to it at `t = ±2`. The bottom face carries the rod's
/// near ruling in its outer cycle and the bore rim as its ring, and
/// both of the rod's ends are transverse caps.
fn rodded_washer(rod: Rod) -> Body<f64> {
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
        polar(half, rod.az - 45.0),
        polar(half, rod.az + 45.0),
        polar(half, rod.az + 135.0),
        polar(half, rod.az + 225.0),
    ]);
    let squared = boolean(BooleanOp::Intersect, &washer, &square);
    let (n, t) = rod.frame();
    let cx = rod.near + rod.rho;
    let body = prism_at(
        vec![
            (Point2::new(cx - rod.rho, 0.0), 1.0),
            (Point2::new(cx + rod.rho, 0.0), 1.0),
        ],
        -4.0,
        8.0,
        tol(),
    );
    let body = moved(
        &body,
        &Affine3::from_parts(
            Mat3::from_cols(n, Vec3::new(0.0, 1.0, 0.0), t),
            Vec3::new(0.0, 0.0, 0.0),
        ),
    );
    let joined = boolean(BooleanOp::Union, &squared, &body);
    let at = |nn: f64, tt: f64| {
        let p = n * nn + t * tt;
        (p.x, p.z)
    };
    let caps = upright(&[at(-5.0, -2.0), at(5.0, -2.0), at(5.0, 2.0), at(-5.0, 2.0)]);
    boolean(BooleanOp::Intersect, &joined, &caps)
}

fn near_ruling(body: &Body<f64>, rod: Rod) -> EdgeKey {
    let (n, _) = rod.frame();
    the_edge(body, |p| {
        p.y.abs() < 1e-9 && (p.x * n.x + p.z * n.z - rod.near).abs() < 1e-9
    })
}

fn rod_request(
    body: &Body<f64>,
    rod: Rod,
    r: f64,
) -> Result<sweep::blend::Filleted<f64>, BlendError> {
    let mut edges = rim_arcs_at(body, 1.0, 0.0);
    assert_eq!(edges.len(), 1, "the bore rim is one closed edge");
    edges.push(near_ruling(body, rod));
    fillet_edges(&sweep::test_support::at_rest(body), &edges, r, tol()).map_err(|e| e.error)
}

/// The carved bottom face's own clearance, read off the result: the
/// rod's trimline (the one line edge in the plane `y = 0` running along
/// `t` that is not a seam) at its nearest to the axis, less the bore
/// rim's trim circle (the plane's one circle about the axis).
fn carved_clearance(body: &Body<f64>, rod: Rod, r: f64) -> f64 {
    let (n, t) = rod.frame();
    let mut lines = Vec::new();
    let mut circles = Vec::new();
    for (_, e) in body.edges() {
        let g = body
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .expect("a certified carrier");
        let (t0, t1) = g.params();
        match *g.carrier() {
            geom::Curve3::Line { origin, dir } => {
                let (p, q) = (origin + dir * t0, origin + dir * t1);
                let d = q - p;
                let along = d.dot(t).abs() > (1.0 - 1e-12) * d.norm();
                if p.y.abs() < 1e-12 && q.y.abs() < 1e-12 && along {
                    // Nearest to the axis: the foot of the segment's
                    // line, clamped into it.
                    let w = Vec3::new(p.x, 0.0, p.z);
                    let s = (-w.dot(d) / d.dot(d)).clamp(0.0, 1.0);
                    let x = w + d * s;
                    lines.push((x.x.hypot(x.z), (p.x * n.x + p.z * n.z)));
                }
            }
            geom::Curve3::Circle { center, radius, .. }
                if center.x.hypot(center.y).hypot(center.z) < 1e-12 && radius < 2.0 =>
            {
                circles.push(radius);
            }
            _ => {}
        }
    }
    let want_n = rod.near - rod.setback(r);
    let [(near, _)] = lines
        .iter()
        .copied()
        .filter(|&(_, nn)| (nn - want_n).abs() < 1e-9)
        .collect::<Vec<_>>()[..]
    else {
        panic!("{rod:?} r {r}: one trimline at n = {want_n}, got {lines:?}")
    };
    let [trim] = circles[..] else {
        panic!("{rod:?} r {r}: one trim circle on the bottom face, got {circles:?}")
    };
    assert!(
        (trim - (1.0 + r)).abs() < 1e-12,
        "the rim's trim circle has radius 1 + r"
    );
    near - trim
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
    let body = rodded_washer(THE_ROD);
    validate_geometric(&body, tol()).expect("the fixture is tier-3 valid");
    let r = 0.25;
    let err = rod_request(&body, THE_ROD, r).expect_err("the trims cross");
    let BlendError::RingClearance {
        margin, bounded, ..
    } = err
    else {
        panic!("an exact reader answers, not the sampled screen; got {err:?}")
    };
    assert!(!bounded, "a line read exactly");
    let want = THE_ROD.clearance(r);
    let read = reading(&margin);
    assert!(
        (read - want).abs() < 1e-12,
        "the trimline's reach less the rim's trim radius (read {read}, derived {want})"
    );
    let out = rod_request(&body, THE_ROD, 0.22)
        .unwrap_or_else(|e| panic!("clear trims carve, got {e:?}"));
    validate_geometric(&out.body, tol()).expect("the carve is tier-3 valid");
    let (got, want) = (volume(&out.body) - volume(&body), THE_ROD.carve_dv(0.22));
    assert!(
        (got - want).abs() < 1e-9,
        "ΔV {got} is the rod's fillet less the rim's spandrel ring, {want}"
    );
    assert_eq!(
        (out.band_faces.len(), out.blend_faces.len()),
        (1, 1),
        "the rim's band and the rod's"
    );
}

/// **Across the boundary, the walk's refusal is the closed form, and a
/// carve keeps it.** Each rod and radius is a pair on either side of
/// zero: rod azimuths on and off the rim's 45° station lattice, and a
/// near ruling and rod radius that move the crossing radius. A pair whose clearance is negative refuses
/// — at the walk, reading exactly that clearance, or at the screen where
/// a station happens to face the closest approach — and a pair whose
/// clearance is positive carves a tier-3-valid body whose bottom-face
/// trims stand exactly that far apart.
#[test]
fn the_co_requested_rod_refuses_and_carves_on_its_closed_form_clearance() {
    let rods = [
        (THE_ROD, [0.215, 0.23]),
        (
            Rod {
                az: 135.0,
                ..THE_ROD
            },
            [0.215, 0.23],
        ),
        (
            Rod {
                az: 100.0,
                ..THE_ROD
            },
            [0.215, 0.23],
        ),
        (
            Rod {
                near: 1.45,
                rho: 0.3,
                ..THE_ROD
            },
            [0.24, 0.28],
        ),
    ];
    for (rod, radii) in rods {
        let body = rodded_washer(rod);
        validate_geometric(&body, tol()).expect("the fixture is tier-3 valid");
        for r in radii {
            let want = rod.clearance(r);
            assert!(
                want.abs() > 1e-3,
                "{rod:?} r {r}: a pair off the zero, {want}"
            );
            match rod_request(&body, rod, r) {
                Ok(out) => {
                    assert!(
                        want > 0.0,
                        "{rod:?} r {r}: carved through a {want} clearance"
                    );
                    validate_geometric(&out.body, tol())
                        .unwrap_or_else(|e| panic!("{rod:?} r {r}: tier-3 valid, {e:?}"));
                    let got = carved_clearance(&out.body, rod, r);
                    assert!(
                        (got - want).abs() < 1e-9,
                        "{rod:?} r {r}: the carved trims stand {got} apart, derived {want}"
                    );
                    let (dv, dv_want) = (volume(&out.body) - volume(&body), rod.carve_dv(r));
                    assert!(
                        (dv - dv_want).abs() < 1e-9,
                        "{rod:?} r {r}: ΔV {dv}, the rod's fillet less the rim's ring {dv_want}"
                    );
                }
                Err(BlendError::RingClearance { margin, .. }) => {
                    let read = reading(&margin);
                    assert!(
                        (read - want).abs() < 1e-12,
                        "{rod:?} r {r}: the walk reads {read}, derived {want}"
                    );
                }
                Err(BlendError::FaceClearanceUncertified { margin, .. }) => {
                    assert!(
                        want < 0.0,
                        "{rod:?} r {r}: the screen refused a {want} clearance"
                    );
                    assert_eq!(margin.sign, Sign::Negative, "{rod:?} r {r}: definite");
                }
                Err(e) => panic!("{rod:?} r {r}: clearance {want}, got {e:?}"),
            }
        }
    }
}

/// **An open plane–plane link on an annulus rim's host closes on the
/// host's outer cycle.** The squared washer's bottom face is ONE plane
/// face: the square's bottom edges are its outer cycle and the bore rim
/// its ring, with no seam meeting either. The bottom edges alone turn
/// at the square's corners, whose walls are square to the bottom, so
/// they meet in four mitres; every edge of the square closes its
/// corners in patches. Either carves beside the bore's band, the ring
/// pass reading the bore's widened trim against each trimline.
#[test]
fn a_plane_link_on_an_annulus_hosts_outer_cycle_closes_at_the_squares_corners() {
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
    let half = 2.1 * 2f64.sqrt();
    let polar = |az: f64| {
        let a = az.to_radians();
        (half * a.cos(), half * a.sin())
    };
    let square = upright(&[polar(67.5), polar(157.5), polar(247.5), polar(337.5)]);
    let body = boolean(BooleanOp::Intersect, &washer, &square);
    validate_geometric(&body, tol()).expect("the fixture is tier-3 valid");
    let on_square = |p: Vec3<f64>| p.x.hypot(p.z) > 2.05;
    let request = |bottom_only: bool| {
        let mut edges = rim_arcs_at(&body, 1.0, 0.0);
        edges.extend(body.edges().map(|(k, _)| k).filter(|&k| {
            let [p, q] = ends(&body, k);
            on_square(p) && on_square(q) && (!bottom_only || (p.y == 0.0 && q.y == 0.0))
        }));
        edges
    };
    let bottom = fillet_edges(
        &sweep::test_support::at_rest(&body),
        &request(true),
        0.2,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the bottom edges mitre at the square's corners, got {e:?}"));
    validate_geometric(&bottom.body, tol()).expect("the mitred carve is tier-3 valid");
    assert_eq!(
        bottom.naming.as_ref().expect("births").mitres.len(),
        4,
        "a mitre at each of the square's corners"
    );
    let edges = request(false);
    assert_eq!(
        edges.len(),
        13,
        "the bore rim and the square's twelve edges"
    );
    let out = fillet_edges(&sweep::test_support::at_rest(&body), &edges, 0.2, tol())
        .unwrap_or_else(|e| panic!("the whole square carves beside the bore, got {e:?}"));
    validate_geometric(&out.body, tol()).expect("the carve is tier-3 valid");
    assert_eq!(
        (
            out.band_faces.len(),
            out.blend_faces.len(),
            out.corner_faces.len()
        ),
        (1, 12, 8),
        "the bore's band, a blend per square edge and a corner per square corner"
    );
    // The rounded box (Steiner on the inner box `x·y·z`) less the bore
    // cylinder less the bore rim's spandrel ring.
    let r = 0.2;
    let (x, y, z) = (4.2 - 2.0 * r, 4.2 - 2.0 * r, 1.0 - 2.0 * r);
    let rounded = x * y * z
        + 2.0 * (x * y + y * z + z * x) * r
        + PI * (x + y + z) * r * r
        + 4.0 * PI * r.powi(3) / 3.0;
    let (got, want) = (volume(&out.body), rounded - PI - bore_rim_spandrel(r));
    assert!(
        (got - want).abs() < 1e-9,
        "the carved volume {got} vs the closed form {want}"
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
        fillet_edges(&sweep::test_support::at_rest(&body), &edges, r, tol())
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
    let err = fillet_edges(&sweep::test_support::at_rest(&washer), &edges, r, tol())
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
    let out = fillet_edges(&sweep::test_support::at_rest(&washer), &edges, 0.49, tol())
        .unwrap_or_else(|e| panic!("clear trims carve, got {e:?}"));
    validate_geometric(&out.body, tol()).expect("the carve is tier-3 valid");
    assert_eq!(out.band_faces.len(), 2, "both rims' bands");
}
