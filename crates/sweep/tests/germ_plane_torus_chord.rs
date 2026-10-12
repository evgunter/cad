//! **The plane × torus chord for the meridian and the concentric
//! circles, through the public ops**: every pose whose plane holds the
//! torus's axis or stands square to it builds its true body in all six
//! ops (`docs/GERM-TORUS-PLANE-FRAME-SPEC.md`, U-T3).
//!
//! Each body is held to the truth three ways: its volume against the
//! closed form (Pappus: a section of the tube's disc swept round the
//! axis), itself checked against a Monte Carlo of the analytic operands;
//! tier 3; and `point_in_solid` at sampled points against the analytic
//! operands' membership under the op. The poses run on the two-face
//! donut, on the one-face torus (its vertex at three phases) and on the
//! torus seamed on `y = 0`, where both loops of a section lie on one
//! face pair; and TANG's tube, standing on a torus's 45° latitude.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::bar;
use crate::germ_plane_torus_frame::{
    SIX, above, degrees, half_space, one_face_torus, seamed_torus, six, tang, torus,
};
use core::f64::consts::{FRAC_1_SQRT_2, PI};
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::RawLoop;
use topo::{AtRestBody, BooleanOp};

/// An operand's analytic membership at a point: `None` within `MARGIN`
/// of its boundary, where the sampled check reads nothing.
type Inside = Box<dyn Fn(Point3<f64>) -> Option<bool>>;

/// How far off every operand's boundary a sampled point must lie: far
/// past the band at every ε row, so the kernel's answer is definite.
const MARGIN: f64 = 1e-4;

/// `x` against the open interval `(lo, hi)`, `None` within the margin of
/// either end.
fn within(x: f64, (lo, hi): (f64, f64)) -> Option<bool> {
    let d = (x - lo).min(hi - x);
    (d.abs() > MARGIN).then_some(d > 0.0)
}

/// Every membership in `all` decided, and all inside.
fn all(of: &[Option<bool>]) -> Option<bool> {
    of.iter()
        .try_fold(true, |acc, m| m.map(|inside| acc && inside))
}

/// A solid of a row: its body, its closed-form volume and its analytic
/// membership.
struct Solid {
    body: AtRestBody<f64>,
    volume: f64,
    inside: Inside,
}

/// The ring torus `(R, r)` about `y` through the origin, as `body`.
fn ring(body: AtRestBody<f64>, (big_r, r): (f64, f64)) -> Solid {
    Solid {
        body,
        volume: 2.0 * PI * PI * big_r * r * r,
        inside: Box::new(move |p| {
            let q = p.x.hypot(p.z) - big_r;
            let g = q.hypot(p.y) - r;
            (g.abs() > MARGIN).then_some(g < 0.0)
        }),
    }
}

/// The box `x × y × z`.
fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Solid {
    Solid {
        body: bar(x, y, z),
        volume: (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0),
        inside: Box::new(move |p| all(&[within(p.x, x), within(p.y, y), within(p.z, z)])),
    }
}

/// The half-space bar of `half_space(o, n)`: 12 m along `n` from the
/// plane, 12 m square about `o` across it (`operands::framed_bar`).
fn half(o: Point3<f64>, n: Vec3<f64>) -> Solid {
    let d = n.normalize();
    let u = d.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
    let v = d.cross(u);
    Solid {
        body: half_space(o, n),
        volume: 12.0f64.powi(3),
        inside: Box::new(move |p| {
            let w = p - o;
            all(&[
                within(w.dot(d), (0.0, 12.0)),
                within(w.dot(u), (-6.0, 6.0)),
                within(w.dot(v), (-6.0, 6.0)),
            ])
        }),
    }
}

/// The half-space `y ≥ y0` of `above(y0)`.
fn upper(y0: f64) -> Solid {
    Solid {
        body: above(y0),
        ..boxed((-6.0, 6.0), (y0, y0 + 12.0), (-6.0, 6.0))
    }
}

/// The area of the tube's disc of radius `r` above the height `h`,
/// `|h| < r`.
fn segment(r: f64, h: f64) -> f64 {
    r * r * (h / r).acos() - h * (r * r - h * h).sqrt()
}

/// The volume of the ring torus `(R, r)` above the height `h` (Pappus:
/// the disc's segment above `h`, centred on the core, swept round it).
pub(crate) fn cap(big_r: f64, r: f64, h: f64) -> f64 {
    2.0 * PI * big_r * segment(r, h)
}

/// The run's ε, which every tolerance here scales by.
fn eps() -> f64 {
    Tol::witness().eps()
}

/// A tolerance on a volume of size `v`: rounding at the default ε, and
/// the band's reach of a boundary at a coarse one.
pub(crate) fn volume_tol(v: f64) -> f64 {
    v.abs().max(1.0) * 1e-9_f64.max(1e3 * eps())
}

/// A fixed stream of points in the box `lo..hi`: a Weyl sequence, so the
/// Monte Carlo and the sampled checks read the same points on every run
/// (a cross-check of a closed form, not a search).
fn points(lo: Point3<f64>, hi: Point3<f64>, n: usize) -> impl Iterator<Item = Point3<f64>> {
    let alpha = [
        0.754_877_666_246_692_7,
        0.569_840_290_998_053,
        0.438_289_683_515_448,
    ];
    (1..=n).map(move |k| {
        let t = |i: usize| (k as f64 * alpha[i]).fract();
        Point3::new(
            lo.x + (hi.x - lo.x) * t(0),
            lo.y + (hi.y - lo.y) * t(1),
            lo.z + (hi.z - lo.z) * t(2),
        )
    })
}

/// What `op` keeps of two memberships, `x` the first member's.
fn kept(op: BooleanOp, x: bool, y: bool) -> bool {
    match op {
        BooleanOp::Union => x || y,
        BooleanOp::Intersect => x && y,
        BooleanOp::Subtract => x && !y,
    }
}

/// `b`'s containment of `q`, `None` where it does not decide.
fn in_solid(b: &topo::Body<f64>, q: Point3<f64>) -> Option<bool> {
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    match topo::point_in_solid(b, q, band, Tol::witness()) {
        Ok(topo::SolidContainment::In) => Some(true),
        Ok(topo::SolidContainment::Out) => Some(false),
        _ => None,
    }
}

/// **`t` against `p` builds its true body in all six ops**, the two
/// meeting in `meet` (a closed form). `meet` is first held to a Monte
/// Carlo of the analytic operands over `(lo, hi)`, which must hold every
/// point either operand holds there. Then each op's body: its volume
/// against the closed form, tier 3, and `point_in_solid` at sampled
/// points of `(lo, hi)` against the analytic memberships under the op.
fn holds_to_the_truth(
    pose: &str,
    t: &Solid,
    p: &Solid,
    meet: f64,
    (lo, hi): (Point3<f64>, Point3<f64>),
) {
    let n = 200_000;
    let hits = points(lo, hi, n)
        .filter(|&q| (t.inside)(q) == Some(true) && (p.inside)(q) == Some(true))
        .count();
    let span = hi - lo;
    let box_volume = span.x * span.y * span.z;
    let estimate = hits as f64 / n as f64 * box_volume;
    let f = meet / box_volume;
    let sigma = box_volume * (f * (1.0 - f) / n as f64).sqrt();
    assert!(
        (estimate - meet).abs() <= 5.0 * sigma,
        "{pose}: the closed form {meet} against the Monte Carlo {estimate} ± {sigma}"
    );
    for ((label, r), &(_, op, swap)) in six(&t.body, &p.body).into_iter().zip(&SIX) {
        let (x, y) = if swap { (p, t) } else { (t, p) };
        let want = match op {
            BooleanOp::Union => x.volume + y.volume - meet,
            BooleanOp::Intersect => meet,
            BooleanOp::Subtract => x.volume - meet,
        };
        let r = r.unwrap_or_else(|e| panic!("{pose}, {label}: refused {e:?}"));
        let b = &r
            .body()
            .unwrap_or_else(|| panic!("{pose}, {label}: empty"))
            .body;
        assert_eq!(
            topo::validate_geometric(b, Tol::witness()),
            Ok(()),
            "{pose}, {label}: tier 3"
        );
        let v = topo::mass_properties(b, Tol::witness())
            .unwrap_or_else(|e| panic!("{pose}, {label}: the volume: {e:?}"))
            .volume;
        assert!(
            (v - want).abs() <= volume_tol(want),
            "{pose}, {label}: volume {v} against the truth {want}"
        );
        let mut read = 0usize;
        for q in points(lo, hi, 48) {
            let (Some(a), Some(c)) = ((x.inside)(q), (y.inside)(q)) else {
                continue;
            };
            assert_eq!(
                in_solid(b, q),
                Some(kept(op, a, c)),
                "{pose}, {label}: point_in_solid at {q:?}"
            );
            read += 1;
        }
        assert!(read >= 40, "{pose}, {label}: {read} points read");
    }
}

/// The donut's box: every point of the tube, and room round it.
fn about_the_tube(big_r: f64, r: f64) -> (Point3<f64>, Point3<f64>) {
    let w = big_r + r + 0.2;
    (Point3::new(-w, -r - 0.2, -w), Point3::new(w, r + 0.2, w))
}

/// The planar poses every torus `(2, 0.5)` about `y` runs, each with its
/// meeting in closed form: the meridian half-spaces `x ≥ 0` (M1) and the
/// one 30° round the axis (M2), each half the torus (`π²Rr²`); the
/// half-spaces `y ≥ 0.2` (P1), `y ≥ 0` (P2) and `y ≥ −0.3`, each the
/// torus above its plane; and the box `[−3, 3] × [0.2, 1] × [−3, 3]`,
/// whose face `y = 0.2` holds both circles, and the one whose face
/// `1e-3` under the tube's top cuts two circles `2√(r·1e-3)` apart
/// across it.
fn planar_poses() -> Vec<(&'static str, Solid, f64)> {
    let (big_r, r) = (2.0, 0.5);
    let m2 = Vec3::new(degrees(30.0).cos(), 0.0, degrees(30.0).sin());
    let half_torus = PI * PI * big_r * r * r;
    vec![
        ("M1", half(Point3::origin(), Vec3::unit_x()), half_torus),
        ("M2", half(Point3::origin(), m2), half_torus),
        ("P1", upper(0.2), cap(big_r, r, 0.2)),
        ("P2", upper(0.0), cap(big_r, r, 0.0)),
        ("y ≥ −0.3", upper(-0.3), cap(big_r, r, -0.3)),
        (
            "the concentric box",
            boxed((-3.0, 3.0), (0.2, 1.0), (-3.0, 3.0)),
            cap(big_r, r, 0.2),
        ),
        (
            "a box face under the tube's top",
            boxed((-3.0, 3.0), (r - 1e-3, 1.0), (-3.0, 3.0)),
            cap(big_r, r, r - 1e-3),
        ),
    ]
}

/// **The donut's meridian and concentric poses build their true
/// bodies**, in all six ops: M1's ∩ is half the donut, `π²Rr²`.
#[test]
fn the_donuts_meridian_and_concentric_poses_build_their_true_bodies() {
    let (big_r, r) = (2.0, 0.5);
    for (pose, p, meet) in planar_poses() {
        let t = ring(torus(big_r, r), (big_r, r));
        holds_to_the_truth(pose, &t, &p, meet, about_the_tube(big_r, r));
    }
}

/// **Both loops of a section on one face pair build the true body**: on
/// the one-face torus (its vertex on the outer equator, off it, and on
/// the inner one) each meridian or concentric circle crosses the face's
/// wrap edges once, and on the torus seamed on `y = 0` both circles of a
/// plane `y = h` lie on one face. Every pose of the donut's row, every
/// op, but for the plane `y = 0` where its circle is an edge of the
/// torus ([`a_section_circle_along_an_equator_edge_refuses_typed`]).
#[test]
fn two_loops_on_one_face_pair_build_their_true_bodies() {
    let (big_r, r) = (2.0, 0.5);
    for (name, body, equator_edge) in tori_of_one_face_pair(big_r, r) {
        let t = ring(body, (big_r, r));
        for (pose, p, meet) in planar_poses() {
            if equator_edge && pose == "P2" {
                continue;
            }
            holds_to_the_truth(
                &format!("{name}, {pose}"),
                &t,
                &p,
                meet,
                about_the_tube(big_r, r),
            );
        }
    }
}

/// The one-face torus with its vertex at three phases and the torus
/// seamed on `y = 0`, each named, and whether an equator (`y = 0`) is
/// one of its edges: the one-face torus's wrap parallel where its vertex
/// lies on an equator, and both of the seamed torus's.
fn tori_of_one_face_pair(big_r: f64, r: f64) -> Vec<(String, AtRestBody<f64>, bool)> {
    let mut tori: Vec<_> = [0.0, 1.0, PI]
        .into_iter()
        .map(|phase| {
            (
                format!("the one-face torus, vertex at {phase}"),
                one_face_torus(big_r, r, phase),
                phase != 1.0,
            )
        })
        .collect();
    tori.push(("the seamed torus".to_string(), seamed_torus(big_r, r), true));
    tori
}

/// **A section circle along an equator edge of the torus refuses typed**
/// in every op: the plane `y = 0` against the one-face torus whose
/// vertex lies on an equator, whose wrap parallel is that circle, and
/// against the torus seamed on `y = 0`, whose two equators are both
/// circles. A closed edge of one operand lying in the other's face is a
/// section loop with no site but its own vertex, which the join does not
/// represent (`work/sect/closed-in-face-section-loop-has-one-site.md`).
#[test]
fn a_section_circle_along_an_equator_edge_refuses_typed() {
    let (big_r, r) = (2.0, 0.5);
    for (name, body, equator_edge) in tori_of_one_face_pair(big_r, r) {
        if !equator_edge {
            continue;
        }
        for (label, got) in six(&body, &above(0.0)) {
            assert!(
                matches!(
                    got,
                    Err(topo::BooleanError::Join(
                        topo::SplitJoinError::SingleSiteSectionLoop { .. }
                    ))
                ),
                "{name}, P2, {label}: {:?}",
                got.map(|r| r.body().map(|b| b.kind))
            );
        }
    }
}

/// **TANG's tube on a torus's latitude builds its true body**: the torus
/// `R = 2`, `r = 1` about `y`, and a tube of radius `2 + 1/√2` standing
/// at `y = 1/√2`, its rim on the 45° latitude. Its outer circle is the
/// rim, an edge lying on the torus, so the wall side's chord takes the
/// concentric circles; the tube holds the whole torus above its floor,
/// so they meet in that cap, `2πR·(π/4 − 1/2)` for `r = 1`.
#[test]
fn tangs_tube_on_a_latitude_builds_its_true_body() {
    let (body_t, body_p) = tang();
    let (big_r, r, s) = (2.0, 1.0, FRAC_1_SQRT_2);
    let rim = big_r + s;
    let t = ring(body_t, (big_r, r));
    let p = Solid {
        body: body_p,
        volume: PI * rim * rim,
        inside: Box::new(move |q| {
            all(&[
                within(q.x.hypot(q.z), (-1.0, rim)),
                within(q.y, (s, s + 1.0)),
            ])
        }),
    };
    let meet = cap(big_r, r, s);
    holds_to_the_truth(
        "T1",
        &t,
        &p,
        meet,
        (Point3::new(-3.2, -1.2, -3.2), Point3::new(3.2, 1.9, 3.2)),
    );
}

/// The slab on a plane through the donut's axis at `(0, h, 0)`, its
/// normal turned off `y` toward `x` by `theta`, extruded 2 m along it:
/// in the plane, `u` turned off `x` by `theta` runs `−3..2s + 3` and `z`
/// runs `−3..3`, so its cap's stored origin, the profile's centroid,
/// stands `s` off the axis.
fn tilted_slab(h: f64, theta: f64, s: f64) -> Solid {
    let d = Vec3::new(theta.sin(), theta.cos(), 0.0);
    let u = Vec3::new(theta.cos(), -theta.sin(), 0.0);
    let v = d.cross(u);
    let at = Point3::new(0.0, h, 0.0);
    let (u_span, v_span) = ((-3.0, 2.0 * s + 3.0), (-3.0, 3.0));
    let lp = profile::ProfileLoop::polygon([
        Point2::new(u_span.0, v_span.0),
        Point2::new(u_span.1, v_span.0),
        Point2::new(u_span.1, v_span.1),
        Point2::new(u_span.0, v_span.1),
    ]);
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(u, v, d),
        at - Point3::origin(),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the slab's profile validates");
    let body = sweep::extrude(
        &vp,
        sweep::Extrusion::Distance {
            depth: 2.0,
            side: sweep::ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .expect("the slab extrudes")
    .body;
    Solid {
        body: sweep::test_support::finished("the slab", body, Tol::witness()),
        volume: (u_span.1 - u_span.0) * (v_span.1 - v_span.0) * 2.0,
        inside: Box::new(move |p| {
            let w = p - at;
            all(&[
                within(w.dot(d), (0.0, 2.0)),
                within(w.dot(u), u_span),
                within(w.dot(v), v_span),
            ])
        }),
    }
}

/// **A cap tilted inside the band builds its true body wherever its
/// plane's origin is stored**: the slab on `y = 0.2` against the donut,
/// its normal off the axis by `0.9` of the band over `R + r` (decided
/// square to the axis), its cap's origin stored on the axis and 5 m,
/// 50 m and 500 m off it. The concentric circles are read where the
/// plane crosses the axis; read at the stored origin they stood off the
/// cut by that distance times the tilt, and every op refused at
/// certification. The tilt moves the meeting's volume at second order
/// only (the donut is symmetric across the tilt's axis).
#[test]
fn a_tilted_cap_builds_its_true_body_wherever_its_origin_is_stored() {
    let (big_r, r, h) = (2.0, 0.5, 0.2);
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    let theta = 0.9 * band.zero() / (big_r + r);
    for s in [0.0, 5.0, 50.0, 500.0] {
        let t = ring(torus(big_r, r), (big_r, r));
        let p = tilted_slab(h, theta, s);
        holds_to_the_truth(
            &format!("the cap's origin {s} m off the axis"),
            &t,
            &p,
            cap(big_r, r, h),
            about_the_tube(big_r, r),
        );
    }
}
