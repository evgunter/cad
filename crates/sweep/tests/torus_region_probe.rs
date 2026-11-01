//! **The torus region reading against the truth, at random queries**:
//! plane × torus bodies, and a filleted puck whose torus face is bounded
//! by the tube's bottom circle, each op's result asked by
//! `point_in_solid` against the analytic memberships of its operands
//! under the op. Three populations:
//!
//! - points off every surface, whose rays cross the result's torus walls
//!   and ask each wall's region at the hit;
//! - points ON the torus, which the boundary pre-pass hands the wall's
//!   region directly: on a kept wall the answer is `OnBoundary`, on a
//!   wall the op cut away the side of the material;
//! - points NEAR the torus, `10⁻⁷` to `10⁻⁴` off it along its normal:
//!   their side, never `OnBoundary`, except within the band's zero of
//!   the tube, where the door reads the point on it (a ε row whose zero
//!   reaches `10⁻⁷` draws such points), so the truth is the tube's.
//!
//! The tori are the donut, the one-face torus, a thin tube and the wide
//! ring `(100, 1)`, the poses scaled to each.
//!
//! Shapes (implementer discipline §8):
//!
//! - `random_queries_read_the_truth` is a counterexample search: the seed
//!   varies, is logged, and the count rides `CAD_FUZZ_EFFORT`. It asserts
//!   no wrong answer and no `VolumeUncertified`, the side at infinity of
//!   a body whose volume is far from zero; any other refusal is not a
//!   failure, and the counts of each are printed.
//! - `a_lattice_on_the_tube_reads_the_truth` is its pinned subset: a
//!   fixed lattice on the tube of the notched bodies, asserted exactly.
//! - `the_puck_is_the_shape_it_is_built_as` holds the puck's fixture to
//!   its closed-form volume.
//!
//! A clockwise-wound torus face, which only a hand `mef` mints, is probed
//! in `topo` (`solid_contain`'s `stated_periods_rows`), and the region's
//! boundary pre-pass by `torus_region`'s rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!(
    "crates/topo/src/boolean/torus_region.rs",
    "crates/topo/src/boolean/region.rs",
    "crates/topo/src/boolean/solid_contain.rs",
    "crates/topo/src/boolean/contain.rs",
    "crates/topo/src/boolean/circle_roots.rs",
    "crates/topo/src/splitting/",
    "crates/sweep/tests/germ_plane_torus_chord.rs",
    "crates/sweep/tests/germ_plane_torus_frame.rs",
    "crates/sweep/tests/revolve_common/",
);

use core::f64::consts::{PI, TAU};
use std::collections::BTreeMap;

use crate::germ_plane_torus_chord::{boxed, half, kept, points};
use crate::germ_plane_torus_frame::{SIX, one_face_torus, six, torus};
use crate::revolve_common::{axis_y, validated};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, BooleanOp, SolidContainment};

/// How far off every operand's boundary a point off the surfaces must
/// lie, and how far a point on or near the tube must clear an edge of
/// the torus face: past the band at every ε row.
const MARGIN: f64 = 1e-4;

/// The bulge of a quarter arc turning counterclockwise.
const QUARTER_CCW: f64 = 0.414_213_562_373_095_03;

/// An operand's analytic shape, as a signed distance (negative inside)
/// whose sign is exact and whose size is at most the true distance.
#[derive(Clone, Copy, Debug)]
enum Shape {
    /// The ring torus `(R, r)` about `y` through the origin.
    Torus(f64, f64),
    /// The box `x × y × z`.
    Box((f64, f64), (f64, f64), (f64, f64)),
    /// The half-space bar of `half_space(o, n)`.
    Half(Point3<f64>, Vec3<f64>),
    /// The puck: `ρ ≤ 2`, `0 ≤ y ≤ 1`, its bottom rim filleted by the
    /// torus `(1.5, 0.5)` about `y` through `(0, 0.5, 0)`.
    Puck,
}

/// The puck's fillet torus: centre height, major and minor radius.
const FILLET: (f64, f64, f64) = (0.5, 1.5, 0.5);

impl Shape {
    fn sd(self, q: Point3<f64>) -> f64 {
        let slab = |x: f64, (lo, hi): (f64, f64)| (lo - x).max(x - hi);
        let rho = q.x.hypot(q.z);
        match self {
            Self::Torus(big_r, r) => (rho - big_r).hypot(q.y) - r,
            Self::Box(x, y, z) => slab(q.x, x).max(slab(q.y, y)).max(slab(q.z, z)),
            Self::Half(o, n) => {
                let d = n.normalize();
                let u = d.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
                let v = d.cross(u);
                let w = q - o;
                slab(w.dot(d), (0.0, 12.0))
                    .max(slab(w.dot(u), (-6.0, 6.0)))
                    .max(slab(w.dot(v), (-6.0, 6.0)))
            }
            Self::Puck => {
                let (h, big_r, r) = FILLET;
                let top = (rho - 2.0).max(slab(q.y, (h, 1.0)));
                let core = (rho - big_r).max(slab(q.y, (0.0, h)));
                let fillet = (rho - big_r).hypot(q.y - h) - r;
                let fillet = fillet.max(q.y - h).max(big_r - rho);
                top.min(core).min(fillet)
            }
        }
    }

    /// The torus this shape's torus face lies on: centre height, `R`, `r`.
    fn tube(self) -> Option<(f64, f64, f64)> {
        match self {
            Self::Torus(big_r, r) => Some((0.0, big_r, r)),
            Self::Puck => Some(FILLET),
            Self::Box(..) | Self::Half(..) => None,
        }
    }

    /// Whether the point at chart `(u, v)` of [`Self::tube`] is on this
    /// shape's boundary, clear of its face's edges by [`MARGIN`]: every
    /// point of a torus; on the puck, the fillet's quarter `v ∈ [−π/2, 0]`.
    fn tube_face(self, v: f64) -> Option<bool> {
        match self {
            Self::Torus(..) => Some(true),
            Self::Puck => {
                let v = (v + PI).rem_euclid(TAU) - PI;
                let clear = (v + PI / 2.0).abs().min(v.abs()) > MARGIN;
                clear.then_some((-PI / 2.0..0.0).contains(&v))
            }
            Self::Box(..) | Self::Half(..) => None,
        }
    }
}

/// The point at chart `(u, v)` on the torus `(h, R, r)` about `y`, and its
/// outward normal.
fn on_tube((h, big_r, r): (f64, f64, f64), u: f64, v: f64) -> (Point3<f64>, Vec3<f64>) {
    let rho = big_r + r * v.cos();
    let n = Vec3::new(v.cos() * u.cos(), v.sin(), v.cos() * u.sin());
    (
        Point3::new(rho * u.cos(), h + r * v.sin(), rho * u.sin()),
        n,
    )
}

/// The puck's body: the profile revolved whole about `y`.
fn puck() -> AtRestBody<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.5, 0.0), QUARTER_CCW),
        (Point2::new(2.0, 0.5), 0.0),
        (Point2::new(2.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let b = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the puck revolves")
    .body;
    finished("the puck", b, Tol::witness())
}

/// A run of the six ops: what it is, the two operands' shapes and bodies.
struct Pair {
    what: String,
    shapes: (Shape, Shape),
    bodies: (AtRestBody<f64>, AtRestBody<f64>),
}

/// The poses for the torus `(R, r)`: the donut's, scaled by `R / 2`
/// across the axis and `r / 0.5` along it — the spiric box, the box
/// corner, the notch and S1 — and, where `all`, B1 and B1 through the
/// tube.
fn poses(big_r: f64, r: f64, all: bool) -> Vec<(&'static str, Shape)> {
    let (k, ky) = (big_r / 2.0, r / 0.5);
    let s = |a: f64, b: f64| (a * k, b * k);
    let sy = |a: f64, b: f64| (a * ky, b * ky);
    let mut out = vec![
        (
            "the spiric box",
            Shape::Box(s(0.3, 1.0), sy(-1.0, 1.0), s(-3.0, 3.0)),
        ),
        (
            "the corner",
            Shape::Box(s(0.0, 6.0), (-0.1 * ky, 11.9 * ky), s(-6.0, 6.0)),
        ),
        (
            "the notch",
            Shape::Box(s(0.0, 3.0), sy(0.0, 1.0), s(-3.0, 3.0)),
        ),
    ];
    // The half-space's bar is 12 m deep, which reaches no wall of a
    // ring much wider than that.
    if big_r <= 10.0 {
        out.push((
            "S1",
            Shape::Half(Point3::new(0.3 * k, 0.0, 0.0), Vec3::unit_x()),
        ));
    }
    if all {
        out.push(("B1", Shape::Box(s(-0.1, 0.1), sy(-0.1, 0.1), s(-2.0, 2.0))));
        out.push((
            "B1 through the tube",
            Shape::Box(s(-0.1, 0.1), sy(-0.1, 0.1), s(-3.0, 3.0)),
        ));
    }
    out
}

/// A shape's body, for a planar pose.
fn pose_body(shape: Shape) -> AtRestBody<f64> {
    match shape {
        Shape::Box(x, y, z) => boxed(x, y, z).body,
        Shape::Half(o, n) => half(o, n).body,
        Shape::Torus(..) | Shape::Puck => unreachable!("a pose is planar"),
    }
}

/// Every pair the probe runs.
fn pairs() -> Vec<Pair> {
    let mut out = Vec::new();
    let tori: [(&str, Shape, AtRestBody<f64>, bool); 4] = [
        ("the donut", Shape::Torus(2.0, 0.5), torus(2.0, 0.5), true),
        (
            "the one-face torus",
            Shape::Torus(2.0, 0.5),
            one_face_torus(2.0, 0.5, 1.0),
            false,
        ),
        (
            "the thin tube",
            Shape::Torus(2.0, 0.1),
            torus(2.0, 0.1),
            false,
        ),
        (
            "the wide ring",
            Shape::Torus(100.0, 1.0),
            torus(100.0, 1.0),
            false,
        ),
    ];
    for (name, shape, body, all) in tori {
        let Shape::Torus(big_r, r) = shape else {
            unreachable!()
        };
        for (pose, p) in poses(big_r, r, all) {
            if name == "the one-face torus" && pose == "the notch" {
                continue;
            }
            out.push(Pair {
                what: format!("{name}, {pose}"),
                shapes: (shape, p),
                bodies: (body.clone(), pose_body(p)),
            });
        }
    }
    let disc = puck();
    for (pose, p) in [
        (
            "x ≥ 0.3",
            Shape::Half(Point3::new(0.3, 0.0, 0.0), Vec3::unit_x()),
        ),
        (
            "a bar over its rim",
            Shape::Box((0.0, 3.0), (0.2, 0.8), (-0.4, 0.4)),
        ),
    ] {
        out.push(Pair {
            what: format!("the puck, {pose}"),
            shapes: (Shape::Puck, p),
            bodies: (disc.clone(), pose_body(p)),
        });
    }
    out
}

/// An op's result that builds, with what it is, the op, its pair and
/// whether the torus-bearing operand is the first.
struct Built {
    what: String,
    body: AtRestBody<f64>,
    op: BooleanOp,
    pair: usize,
    torus_first: bool,
}

/// The built bodies over `pairs`, and the ops that refused, each with
/// its refusal's name.
fn built(pairs: &[Pair]) -> (Vec<Built>, Vec<String>) {
    let (mut out, mut refused) = (Vec::new(), Vec::new());
    for (i, pair) in pairs.iter().enumerate() {
        for ((label, r), &(_, op, swap)) in
            six(&pair.bodies.0, &pair.bodies.1).into_iter().zip(&SIX)
        {
            match r.map(|r| r.body().map(|b| b.body.clone())) {
                Ok(Some(body)) => out.push(Built {
                    what: format!("{}, {label}", pair.what),
                    body,
                    op,
                    pair: i,
                    torus_first: !swap,
                }),
                Ok(None) => refused.push(format!("{}, {label}: empty", pair.what)),
                Err(e) => {
                    let name = format!("{e:?}");
                    let name = name.split([' ', '(', '{']).next().unwrap_or("").to_string();
                    refused.push(format!("{}, {label}: {name}", pair.what));
                }
            }
        }
    }
    (out, refused)
}

/// Where a point stands against an operand.
#[derive(Clone, Copy, Debug, PartialEq)]
enum At {
    In,
    Out,
    /// On its boundary, clear of the boundary's edges.
    On,
}

impl At {
    /// A point's place against `shape`, read off its signed distance:
    /// `None` within [`MARGIN`] of the boundary.
    fn of(shape: Shape, q: Point3<f64>) -> Option<Self> {
        let d = shape.sd(q);
        (d.abs() > MARGIN).then_some(if d < 0.0 { Self::In } else { Self::Out })
    }
}

/// What the result of `op` holds at a point placed `(t, p)` against the
/// torus-bearing operand and the pose.
fn truth(op: BooleanOp, torus_first: bool, t: At, p: At) -> SolidContainment {
    let pose = p == At::In;
    let read = |t_in: bool| {
        if torus_first {
            kept(op, t_in, pose)
        } else {
            kept(op, pose, t_in)
        }
    };
    let side = |inside: bool| {
        if inside {
            SolidContainment::In
        } else {
            SolidContainment::Out
        }
    };
    match t {
        At::In => side(read(true)),
        At::Out => side(read(false)),
        // On the wall: the material is the same on both sides exactly
        // where the op discarded the wall.
        At::On if read(true) == read(false) => side(read(true)),
        At::On => SolidContainment::OnBoundary,
    }
}

/// The tally of one population.
#[derive(Default)]
struct Tally {
    asked: usize,
    right: usize,
    wrong: Vec<String>,
    refused: BTreeMap<String, usize>,
}

impl Tally {
    /// Reads `got` against `want` at `q` on `what`.
    fn read(
        &mut self,
        what: &str,
        q: Point3<f64>,
        want: SolidContainment,
        got: Result<SolidContainment, topo::PointInSolidError>,
    ) {
        self.asked += 1;
        match got {
            Ok(got) if got == want => self.right += 1,
            Ok(got) => self
                .wrong
                .push(format!("{what} at {q:?}: {got:?}, the truth {want:?}")),
            Err(e) => {
                let name = format!("{e:?}");
                let name = name.split([' ', '(', '{']).next().unwrap_or("").to_string();
                *self.refused.entry(name).or_default() += 1;
            }
        }
    }
}

/// One query: a point and where it stands against the torus-bearing
/// operand, or `None` to read it off the operand's shape.
type Query = (Point3<f64>, Option<At>);

/// Asks every body of `built` at each query of `of(pair)`, the pose's
/// place read off its shape, and tallies against the truth.
fn probe(pairs: &[Pair], built: &[Built], mut of: impl FnMut(&Pair) -> Vec<Query>) -> Tally {
    let band = band();
    let mut tally = Tally::default();
    for (i, pair) in pairs.iter().enumerate() {
        let queries = of(pair);
        for b in built.iter().filter(|b| b.pair == i) {
            for &(q, t) in &queries {
                let (Some(t), Some(p)) = (
                    t.or_else(|| At::of(pair.shapes.0, q)),
                    At::of(pair.shapes.1, q),
                ) else {
                    continue;
                };
                let got = topo::point_in_solid(&b.body, q, band, Tol::witness());
                tally.read(&b.what, q, truth(b.op, b.torus_first, t, p), got);
            }
        }
    }
    tally
}

/// The band every query is asked under: the run's.
fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's band")
}

/// The box a pair's queries are drawn in: about the torus-bearing shape.
fn about(shape: Shape) -> (Point3<f64>, Point3<f64>) {
    let (h, big_r, r) = shape.tube().unwrap();
    let w = 1.1 * big_r + r;
    let hy = if matches!(shape, Shape::Puck) {
        0.7
    } else {
        r + 0.2
    };
    (Point3::new(-w, h - hy, -w), Point3::new(w, h + hy, w))
}

/// **No random query reads a wrong answer.** Every op's result over the
/// pairs, asked at random points off every surface, on the tube and near
/// it; refusals are counted, never wrong answers.
#[test]
fn random_queries_read_the_truth() {
    let pairs = pairs();
    let (built, ops_refused) = built(&pairs);
    assert!(
        built.len() >= 80,
        "{} bodies built; refused {ops_refused:?}",
        built.len()
    );
    let mut rng = test_utils::fuzz::start("torus_region_probe");
    let n = test_utils::fuzz::scaled(4);
    let on = band().zero();
    let mut draw = |pair: &Pair, kind: u8| -> Vec<Query> {
        let shape = pair.shapes.0;
        let tube = shape.tube().unwrap();
        let (lo, hi) = about(shape);
        (0..n)
            .filter_map(|_| match kind {
                0 => Some((
                    Point3::new(
                        rng.range(lo.x, hi.x),
                        rng.range(lo.y, hi.y),
                        rng.range(lo.z, hi.z),
                    ),
                    None,
                )),
                1 => {
                    let (u, v) = (rng.range(0.0, TAU), rng.range(0.0, TAU));
                    let (q, _) = on_tube(tube, u, v);
                    match shape.tube_face(v)? {
                        true => Some((q, Some(At::On))),
                        false => Some((q, None)),
                    }
                }
                _ => {
                    let (u, v) = (rng.range(0.0, TAU), rng.range(0.0, TAU));
                    let (q, n) = on_tube(tube, u, v);
                    let delta = rng.log_uniform(1e-7, MARGIN);
                    let out = rng.unit() < 0.5;
                    let q = q + n * if out { delta } else { -delta };
                    let t = match (delta <= on, out) {
                        (true, _) => At::On,
                        (false, true) => At::Out,
                        (false, false) => At::In,
                    };
                    shape.tube_face(v)?.then_some((q, Some(t)))
                }
            })
            .collect()
    };
    let away = probe(&pairs, &built, |p| draw(p, 0));
    let tube = probe(&pairs, &built, |p| draw(p, 1));
    let near = probe(&pairs, &built, |p| draw(p, 2));
    println!("{} bodies built; refused {ops_refused:?}", built.len());
    for (name, t) in [
        ("off every surface", &away),
        ("on the tube", &tube),
        ("near the tube", &near),
    ] {
        println!(
            "{name}: {} asked, {} right, {} wrong, refused {:?}",
            t.asked,
            t.right,
            t.wrong.len(),
            t.refused
        );
        assert!(
            t.wrong.is_empty(),
            "{name}: {} wrong answers ({}):\n{}",
            t.wrong.len(),
            test_utils::fuzz::replay(),
            t.wrong.join("\n")
        );
        // Every body here holds a volume far from zero, which the
        // quadrature measures through its torus walls, so a ray that
        // meets nothing always reads its side.
        assert_eq!(
            t.refused.get("VolumeUncertified"),
            None,
            "{name}: a point whose rays all miss went unread ({})",
            test_utils::fuzz::replay()
        );
    }
}

/// **A lattice on the tube of the notched donut reads the truth**: the
/// pinned subset of [`random_queries_read_the_truth`]. The donut less
/// the notch bar, and its meet with the bar, each asked at a `24 × 12`
/// lattice of the tube's chart and at `200` fixed points off it, every
/// one the operands place answered and right.
#[test]
fn a_lattice_on_the_tube_reads_the_truth() {
    let notch = Shape::Box((0.0, 3.0), (0.0, 1.0), (-3.0, 3.0));
    let pairs = [Pair {
        what: "the donut, the notch".to_string(),
        shapes: (Shape::Torus(2.0, 0.5), notch),
        bodies: (torus(2.0, 0.5), pose_body(notch)),
    }];
    let (built, _) = built(&pairs);
    let built: Vec<Built> = built
        .into_iter()
        .filter(|b| matches!(b.op, BooleanOp::Intersect | BooleanOp::Subtract) && b.torus_first)
        .collect();
    assert_eq!(built.len(), 2, "the notch's T ∩ P and T ∖ P build");
    let (lo, hi) = about(Shape::Torus(2.0, 0.5));
    let off: Vec<Query> = points(lo, hi, 200).map(|q| (q, None)).collect();
    let on: Vec<Query> = (0..24)
        .flat_map(|i| {
            (0..12).map(move |k| {
                let u = (f64::from(i) + 0.13) * TAU / 24.0;
                let v = (f64::from(k) + 0.29) * TAU / 12.0;
                (on_tube((0.0, 2.0, 0.5), u, v).0, Some(At::On))
            })
        })
        .collect();
    for (name, queries) in [("off every surface", off), ("on the tube", on)] {
        let t = probe(&pairs, &built, |_| queries.clone());
        assert!(
            t.wrong.is_empty() && t.refused.is_empty() && t.asked > 100,
            "{name}: {} asked, refused {:?}, wrong:\n{}",
            t.asked,
            t.refused,
            t.wrong.join("\n")
        );
    }
}

/// **The puck is the shape it is built as**: its volume is the closed
/// form — the cylinder above the fillet, the core under it, and the
/// fillet's quarter disc swept round the axis (Pappus) — so its bulge
/// turns the way [`Shape::Puck`] says.
#[test]
fn the_puck_is_the_shape_it_is_built_as() {
    let (h, big_r, r) = FILLET;
    let centroid = big_r + 4.0 * r / (3.0 * PI);
    let want = PI * 4.0 * (1.0 - h) + PI * big_r * big_r * h + TAU * centroid * PI * r * r / 4.0;
    let v = topo::test_support::pinned_volume("the measured body", &puck(), Tol::witness()).volume;
    assert!(
        (v - want).abs() < 1e-9 * want,
        "the puck's volume {v} against {want}"
    );
}
