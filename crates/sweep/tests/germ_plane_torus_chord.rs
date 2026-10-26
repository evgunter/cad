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
pub(crate) type Inside = Box<dyn Fn(Point3<f64>) -> Option<bool>>;

/// How far off every operand's boundary a sampled point must lie: far
/// past the band at every ε row, so the kernel's answer is definite.
pub(crate) const MARGIN: f64 = 1e-4;

/// `x` against the open interval `(lo, hi)`, `None` within the margin of
/// either end.
pub(crate) fn within(x: f64, (lo, hi): (f64, f64)) -> Option<bool> {
    let d = (x - lo).min(hi - x);
    (d.abs() > MARGIN).then_some(d > 0.0)
}

/// Every membership in `all` decided, and all inside.
pub(crate) fn all(of: &[Option<bool>]) -> Option<bool> {
    of.iter()
        .try_fold(true, |acc, m| m.map(|inside| acc && inside))
}

/// A solid of a row: its body, its closed-form volume and its analytic
/// membership.
pub(crate) struct Solid {
    pub(crate) body: AtRestBody<f64>,
    pub(crate) volume: f64,
    pub(crate) inside: Inside,
}

/// The ring torus `(R, r)` about `y` through the origin, as `body`.
pub(crate) fn ring(body: AtRestBody<f64>, (big_r, r): (f64, f64)) -> Solid {
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
pub(crate) fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Solid {
    Solid {
        body: bar(x, y, z),
        volume: (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0),
        inside: Box::new(move |p| all(&[within(p.x, x), within(p.y, y), within(p.z, z)])),
    }
}

/// The half-space bar of `half_space(o, n)`: 12 m along `n` from the
/// plane, 12 m square about `o` across it (`operands::framed_bar`).
pub(crate) fn half(o: Point3<f64>, n: Vec3<f64>) -> Solid {
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
pub(crate) fn points(
    lo: Point3<f64>,
    hi: Point3<f64>,
    n: usize,
) -> impl Iterator<Item = Point3<f64>> {
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
pub(crate) fn kept(op: BooleanOp, x: bool, y: bool) -> bool {
    match op {
        BooleanOp::Union => x || y,
        BooleanOp::Intersect => x && y,
        BooleanOp::Subtract => x && !y,
    }
}

/// **`t` against `p` builds its true body in all six ops**, the two
/// meeting in `meet` (a closed form). `meet` is first held to a Monte
/// Carlo of the analytic operands over `(lo, hi)`, which must hold every
/// point either operand holds there. Then each op's body: its volume
/// against the closed form, tier 3, and `point_in_solid` at sampled
/// points of `(lo, hi)` against the analytic memberships under the op.
pub(crate) fn holds_to_the_truth(
    pose: &str,
    t: &Solid,
    p: &Solid,
    meet: f64,
    within: (Point3<f64>, Point3<f64>),
) {
    meet_is_the_monte_carlo(pose, t, p, meet, within);
    for ((label, r), &(_, op, swap)) in six(&t.body, &p.body).into_iter().zip(&SIX) {
        let r = r.unwrap_or_else(|e| panic!("{pose}, {label}: refused {e:?}"));
        let b = topo::test_support::finished(
            "b",
            topo::Body::clone(
                &r.body()
                    .unwrap_or_else(|| panic!("{pose}, {label}: empty"))
                    .body,
            ),
            Tol::witness(),
        );
        body_holds(
            &format!("{pose}, {label}"),
            (op, swap),
            (t, p),
            meet,
            &b,
            within,
        );
    }
}

/// `meet`, the volume `t` and `p` share, against a Monte Carlo of the
/// analytic operands over `(lo, hi)`, which must hold every point either
/// operand holds there.
pub(crate) fn meet_is_the_monte_carlo(
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
}

/// **`b`, the body `op` built of `t` against `p`, is the truth**
/// (`swap`: `p` is the first member): its volume against `meet`'s
/// arithmetic under the op, tier 3, and `point_in_solid` at sampled
/// points of `(lo, hi)` against the analytic memberships under the op,
/// at least 40 of them read.
pub(crate) fn body_holds(
    what: &str,
    ops: (BooleanOp, bool),
    solids: (&Solid, &Solid),
    meet: f64,
    b: &topo::Body<f64>,
    within: (Point3<f64>, Point3<f64>),
) {
    let read = volume_and_points(what, ops, solids, meet, b, within);
    assert!(read >= 40, "{what}: {read} points read");
}

/// [`body_holds`]'s volume and tier 3, and its `point_in_solid` reading
/// at every sampled point the operands' memberships place, each one
/// answered and right. The points read.
pub(crate) fn volume_and_points(
    what: &str,
    (op, swap): (BooleanOp, bool),
    (t, p): (&Solid, &Solid),
    meet: f64,
    b: &topo::Body<f64>,
    (lo, hi): (Point3<f64>, Point3<f64>),
) -> usize {
    let (x, y) = if swap { (p, t) } else { (t, p) };
    let want = match op {
        BooleanOp::Union => x.volume + y.volume - meet,
        BooleanOp::Intersect => meet,
        BooleanOp::Subtract => x.volume - meet,
    };
    assert_eq!(
        topo::validate_geometric(b, Tol::witness()),
        Ok(()),
        "{what}: tier 3"
    );
    let v = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(b), Tol::witness()),
        Tol::witness(),
    )
    .volume;
    assert!(
        (v - want).abs() <= volume_tol(want),
        "{what}: volume {v} against the truth {want}"
    );
    let mut read = 0usize;
    for q in points(lo, hi, 48) {
        let (Some(a), Some(c)) = ((x.inside)(q), (y.inside)(q)) else {
            continue;
        };
        assert_eq!(
            contains(b, q),
            Ok(kept(op, a, c)),
            "{what}: point_in_solid at {q:?}"
        );
        read += 1;
    }
    read
}

/// `b`'s containment of `q`: inside, outside, or its refusal; a point
/// it reads on the boundary fails.
pub(crate) fn contains(
    b: &topo::Body<f64>,
    q: Point3<f64>,
) -> Result<bool, topo::PointInSolidError> {
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    match topo::point_in_solid(b, q, band, Tol::witness())? {
        topo::SolidContainment::In => Ok(true),
        topo::SolidContainment::Out => Ok(false),
        other => panic!("a sampled point off every boundary reads {other:?} at {q:?}"),
    }
}

/// The donut's box: every point of the tube, and room round it.
pub(crate) fn about_the_tube(big_r: f64, r: f64) -> (Point3<f64>, Point3<f64>) {
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
/// op. Where an equator is an edge of the torus (the one-face torus's
/// wrap parallel with its vertex on it, both of the seamed torus's), the
/// plane `y = 0` holds that closed edge in its face: a section loop
/// whose one site is the edge's own vertex.
#[test]
fn two_loops_on_one_face_pair_build_their_true_bodies() {
    let (big_r, r) = (2.0, 0.5);
    for (name, body) in tori_of_one_face_pair(big_r, r) {
        let t = ring(body, (big_r, r));
        for (pose, p, meet) in planar_poses() {
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
/// seamed on `y = 0`, each named. An equator (`y = 0`) is an edge of the
/// one-face torus whose vertex lies on it (its wrap parallel), and both
/// are edges of the seamed torus.
pub(crate) fn tori_of_one_face_pair(big_r: f64, r: f64) -> Vec<(String, AtRestBody<f64>)> {
    let mut tori: Vec<_> = [0.0, 1.0, PI]
        .into_iter()
        .map(|phase| {
            (
                format!("the one-face torus, vertex at {phase}"),
                one_face_torus(big_r, r, phase),
            )
        })
        .collect();
    tori.push(("the seamed torus".to_string(), seamed_torus(big_r, r)));
    tori
}

/// **A slab across a torus of one face pair builds its true body**: the
/// slab `[−6, 6] × [h, h2] × [−6, 6]`, whose two faces each cut the
/// torus in two concentric circles, against the one-face torus (its
/// vertex at three phases) and the torus seamed on `y = 0`. Every circle
/// crosses its face's wrap meridian once, so one face carries up to four
/// one-site loops, and each loop's chord must keep the others' two
/// halves on one side of it.
#[test]
fn a_slab_across_a_torus_of_one_face_pair_builds_its_true_body() {
    let (big_r, r) = (2.0, 0.5);
    let mut tori: Vec<_> = [0.0, 1.0, PI]
        .into_iter()
        .map(|phase| {
            (
                format!("the one-face torus, vertex at {phase}"),
                one_face_torus(big_r, r, phase),
            )
        })
        .collect();
    tori.push(("the seamed torus".to_string(), seamed_torus(big_r, r)));
    for (name, body) in tori {
        let t = ring(body, (big_r, r));
        for (h, h2) in [(0.1, 0.3), (-0.2, 0.3), (-0.4, -0.1), (0.05, 0.45)] {
            holds_to_the_truth(
                &format!("{name}, the slab {h} < y < {h2}"),
                &t,
                &boxed((-6.0, 6.0), (h, h2), (-6.0, 6.0)),
                cap(big_r, r, h) - cap(big_r, r, h2),
                about_the_tube(big_r, r),
            );
        }
    }
}

/// The volume of the ring torus `(R, r)` above the height `h`, at any
/// `h`: all of it under the tube, none over it.
fn cap_at(big_r: f64, r: f64, h: f64) -> f64 {
    cap(big_r, r, h.clamp(-r, r))
}

/// The comb whose teeth `[−3, 3.5] × [a, b] × [−3, 3]` (one per entry of
/// `teeth`, ascending and apart) hang off the spine
/// `[3, 3.5] × [a₀, bₙ] × [−3, 3]`, clear of a torus `R + r < 3`: the
/// profile in `z = −3` extruded 6 m along `z`.
fn comb(teeth: &[(f64, f64)]) -> Solid {
    let (x0, x1, x2) = (-3.0, 3.0, 3.5);
    let (lo, hi) = (teeth[0].0, teeth[teeth.len() - 1].1);
    let mut ring = vec![
        Point2::new(x0, lo),
        Point2::new(x2, lo),
        Point2::new(x2, hi),
    ];
    for (k, &(a, b)) in teeth.iter().enumerate().rev() {
        ring.push(Point2::new(x0, b));
        ring.push(Point2::new(x0, a));
        if k > 0 {
            ring.push(Point2::new(x1, a));
            ring.push(Point2::new(x1, teeth[k - 1].1));
        }
    }
    ring.pop();
    let plane = profile::SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(
            Mat3::from_cols(
                Vec3::unit_x(),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            Vec3::new(0.0, 0.0, -3.0),
        ),
        geom_core::Tol::witness(),
    ));
    let vp = profile::Profile::new(plane, vec![profile::ProfileLoop::polygon(ring)])
        .validate(Tol::witness())
        .expect("the comb's profile validates");
    let body = sweep::extrude(
        &vp,
        sweep::Extrusion::Distance {
            depth: 6.0,
            side: sweep::ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .expect("the comb extrudes")
    .body;
    let teeth = teeth.to_vec();
    let area = teeth.iter().map(|(a, b)| (x2 - x0) * (b - a)).sum::<f64>()
        + (x2 - x1) * ((hi - lo) - teeth.iter().map(|(a, b)| b - a).sum::<f64>());
    Solid {
        body: sweep::test_support::finished("the comb", body, Tol::witness()),
        volume: area * 6.0,
        inside: Box::new(move |p| {
            let parts: Vec<_> = teeth
                .iter()
                .map(|&(a, b)| all(&[within(p.x, (x0, x2)), within(p.y, (a, b))]))
                .chain([all(&[within(p.x, (x1, x2)), within(p.y, (lo, hi))])])
                .collect();
            let plan = if parts.contains(&Some(true)) {
                Some(true)
            } else {
                parts.iter().all(|m| *m == Some(false)).then_some(false)
            };
            all(&[plan, within(p.z, (-3.0, 3.0))])
        }),
    }
}

/// **A comb across a torus of one face pair builds its true body**: the
/// combs of two to four teeth, each tooth a slab across the tube or one
/// with a face past it (or the whole tooth clear of it), against the
/// one-face torus (its vertex at five phases) and the torus seamed on
/// `y = 0`. Each comb's faces cut the torus in six or eight concentric
/// circles, every one a one-site loop on one face, and each loop's chord
/// must keep every other's two halves on one side of it.
#[test]
fn a_comb_across_a_torus_of_one_face_pair_builds_its_true_body() {
    let (big_r, r) = (2.0, 0.5);
    let combs: [&[(f64, f64)]; 4] = [
        &[(-0.4, -0.15), (0.1, 0.35)],
        &[(-0.7, -0.25), (0.15, 0.3)],
        &[(-0.7, -0.3), (-0.1, 0.1), (0.3, 0.7)],
        &[(-0.9, -0.6), (-0.4, -0.2), (0.05, 0.25), (0.6, 0.9)],
    ];
    let mut tori: Vec<_> = [0.0, 1.0, 2.0, PI, 5.5]
        .into_iter()
        .map(|phase| {
            (
                format!("the one-face torus, vertex at {phase}"),
                one_face_torus(big_r, r, phase),
            )
        })
        .collect();
    tori.push(("the seamed torus".to_string(), seamed_torus(big_r, r)));
    for (name, body) in tori {
        let t = ring(body, (big_r, r));
        for teeth in combs {
            let meet = teeth
                .iter()
                .map(|&(a, b)| cap_at(big_r, r, a) - cap_at(big_r, r, b))
                .sum();
            holds_to_the_truth(
                &format!("{name}, the comb {teeth:?}"),
                &t,
                &comb(teeth),
                meet,
                about_the_tube(big_r, r),
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
    let plane = profile::SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(Mat3::from_cols(u, v, d), at - Point3::origin()),
        geom_core::Tol::witness(),
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

/// **A box edge piercing a torus face leaves a ring the join homes**,
/// in all six ops: the box `x ≥ 0 ∧ y ≥ h` against the one-face torus
/// with its vertex on the inner equator, `h = −0.1, −0.3, −0.45`, whose
/// edge `x = 0, y = h` pierces the tube's outer side in a point ring of
/// the face. That point lies in the planes of two of the run's arcs (a
/// box face's meridian circle, off its span, and the other loop of a
/// box face's parallels), and the face's outer loop on the carriers of
/// the other two (the wrap edges), so every path between them runs
/// along an arc's plane. The meet is half the torus above `h`.
#[test]
fn a_box_edge_piercing_a_one_face_torus_builds_its_true_body() {
    let (big_r, r) = (2.0, 0.5);
    for h in [-0.1, -0.3, -0.45] {
        let t = ring(one_face_torus(big_r, r, PI), (big_r, r));
        let p = boxed((0.0, 6.0), (h, h + 12.0), (-6.0, 6.0));
        holds_to_the_truth(
            &format!("x ≥ 0 ∧ y ≥ {h}"),
            &t,
            &p,
            cap(big_r, r, h) / 2.0,
            about_the_tube(big_r, r),
        );
    }
}

/// **A bar whose face holds the donut's axis homes the ring its edge
/// pierces**, in all six ops: `[−3, 3] × [0.2, 1.2] × [0, 3]`, whose
/// edges `y = 0.2, z = 0` pierce the tube in point rings on the
/// meridian plane `z = 0`, which holds the run's two arcs on the plane's
/// other loop. The meet is half the donut above `y = 0.2`.
#[test]
fn a_bar_on_the_donuts_meridian_plane_builds_its_true_body() {
    let (big_r, r) = (2.0, 0.5);
    let t = ring(torus(big_r, r), (big_r, r));
    let p = boxed((-3.0, 3.0), (0.2, 1.2), (0.0, 3.0));
    holds_to_the_truth(
        "the bar on z = 0",
        &t,
        &p,
        cap(big_r, r, 0.2) / 2.0,
        about_the_tube(big_r, r),
    );
}

/// **A one-face torus whose vertex lies on the box's face builds its
/// true body**, in all six ops: the box `x ≥ 0 ∧ y ≥ −0.3` against the
/// one-face torus with its vertex at `π + asin 0.6`, the point
/// `(1.6, −0.3)` of its profile. So its wrap parallel is the section's
/// inner circle on the face `y = −0.3`, and its vertex lies inside that
/// face. The face cuts the tube transversally there. The coincidence is
/// an edge and a vertex on a face, decided Zero by the crossing layer's
/// margins, not a sector of the torus lying on the plane. The join
/// glues it, and the ring the edge `x = 0, y = −0.3` pierces is homed
/// as in [`a_box_edge_piercing_a_one_face_torus_builds_its_true_body`].
#[test]
fn a_one_face_torus_whose_vertex_lies_on_the_box_face_builds_its_true_body() {
    let (big_r, r, h) = (2.0, 0.5, -0.3);
    let t = ring(one_face_torus(big_r, r, PI + (0.6f64).asin()), (big_r, r));
    let p = boxed((0.0, 6.0), (h, h + 12.0), (-6.0, 6.0));
    holds_to_the_truth(
        "the vertex on y = −0.3",
        &t,
        &p,
        cap(big_r, r, h) / 2.0,
        about_the_tube(big_r, r),
    );
}
