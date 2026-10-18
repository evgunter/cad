//! **The torus region reading against the truth, at random queries**:
//! the plane × torus bodies of the germ rows, each op's result asked by
//! `point_in_solid` at random points against the analytic memberships of
//! its operands under the op. Two populations: points off every surface,
//! whose rays cross the result's torus walls and ask each wall's region
//! at the hit; and points ON the torus, which the boundary pre-pass hands
//! the wall's region directly — on a kept wall the answer is
//! `OnBoundary`, on a wall the op cut away the side of the material.
//!
//! Shapes (implementer discipline §8):
//!
//! - `random_queries_read_the_truth` is a counterexample search: the seed
//!   varies, is logged, and the count rides `CAD_FUZZ_EFFORT`. It asserts
//!   no wrong answer; a refusal is not one, and the counts of each are
//!   printed.
//! - `a_lattice_on_the_tube_reads_the_truth` is its pinned subset: a
//!   fixed lattice on the tube of the notched bodies, asserted exactly.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!(
    "crates/topo/src/boolean/torus_region.rs",
    "crates/topo/src/boolean/solid_contain.rs",
    "crates/topo/src/boolean/contain.rs",
    "crates/topo/src/boolean/circle_roots.rs",
    "crates/topo/src/splitting/",
    "crates/sweep/tests/germ_plane_torus_chord.rs",
    "crates/sweep/tests/germ_plane_torus_frame.rs",
);

use core::f64::consts::TAU;
use std::collections::BTreeMap;

use crate::germ_plane_torus_chord::{Solid, about_the_tube, boxed, half, kept, points, ring};
use crate::germ_plane_torus_frame::{SIX, one_face_torus, six, torus};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{BooleanOp, SolidContainment};

/// The torus `(R, r)` the rows run, about `y`.
const RING: (f64, f64) = (2.0, 0.5);

/// The planar solids: S1, the half-space whose plane cuts two ovals; the
/// box between two ovals' planes; B1, the bar across the hole; the box
/// corner `x ≥ 0 ∧ y ≥ −0.1`; and the bar over a quarter of the tube's
/// top (the notch).
fn poses() -> Vec<(&'static str, Solid)> {
    vec![
        ("S1", half(Point3::new(0.3, 0.0, 0.0), Vec3::unit_x())),
        (
            "the spiric box",
            boxed((0.3, 1.0), (-1.0, 1.0), (-3.0, 3.0)),
        ),
        ("B1", boxed((-0.1, 0.1), (-0.1, 0.1), (-2.0, 2.0))),
        ("the corner", boxed((0.0, 6.0), (-0.1, 11.9), (-6.0, 6.0))),
        ("the notch", boxed((0.0, 3.0), (0.0, 1.0), (-3.0, 3.0))),
    ]
}

/// The tori: the donut (two faces closed on each other) and the one-face
/// torus with its vertex off both equators.
fn tori() -> Vec<(&'static str, Solid)> {
    vec![
        ("the donut", ring(torus(RING.0, RING.1), RING)),
        (
            "the one-face torus",
            ring(one_face_torus(RING.0, RING.1, 1.0), RING),
        ),
    ]
}

/// An op's result that builds: what it is, the body, the op, the torus
/// and the pose it was built of (indices), and whether the torus is the
/// first operand.
struct Built {
    what: String,
    body: topo::AtRestBody<f64>,
    op: BooleanOp,
    torus: usize,
    pose: usize,
    torus_first: bool,
}

/// The built bodies over `tori` × `poses`.
fn built(tori: &[(&str, Solid)], poses: &[(&str, Solid)]) -> Vec<Built> {
    let mut out = Vec::new();
    for (torus, (tn, t)) in tori.iter().enumerate() {
        for (pose, (pn, p)) in poses.iter().enumerate() {
            for ((label, r), &(_, op, swap)) in six(&t.body, &p.body).into_iter().zip(&SIX) {
                let Some(b) = r.ok().and_then(|r| r.body().map(|b| b.body.clone())) else {
                    continue;
                };
                out.push(Built {
                    what: format!("{tn}, {pn}, {label}"),
                    body: b,
                    op,
                    torus,
                    pose,
                    torus_first: !swap,
                });
            }
        }
    }
    out
}

/// What the result of `op` holds at a point the operands place, the
/// torus's membership `t` (`None` on the tube) and the pose's `p`.
fn truth(op: BooleanOp, torus_first: bool, t: Option<bool>, p: bool) -> SolidContainment {
    let read = |t_in: bool| {
        if torus_first {
            kept(op, t_in, p)
        } else {
            kept(op, p, t_in)
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
        Some(t_in) => side(read(t_in)),
        // On the tube: the material is the same on both sides of the wall
        // exactly where the op discarded the wall.
        None if read(true) == read(false) => side(read(true)),
        None => SolidContainment::OnBoundary,
    }
}

/// The point at chart `(u, v)` on the torus about `y`.
fn on_tube(u: f64, v: f64) -> Point3<f64> {
    let rho = RING.0 + RING.1 * v.cos();
    Point3::new(rho * u.cos(), RING.1 * v.sin(), rho * u.sin())
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

/// Asks every body of `built` at each of `off`, points off every surface
/// that the operands place, and each of `on`, points on the tube the pose
/// places.
fn probe(
    tori: &[(&str, Solid)],
    poses: &[(&str, Solid)],
    built: &[Built],
    off: &[Point3<f64>],
    on: &[Point3<f64>],
) -> (Tally, Tally) {
    let band = Band::linear(Tol::witness()).expect("the run's band");
    let (mut away, mut tube) = (Tally::default(), Tally::default());
    for b in built {
        let (t, p) = (&tori[b.torus].1, &poses[b.pose].1);
        let ask = |q| topo::point_in_solid(&b.body, q, band, Tol::witness());
        for &q in off {
            if let (Some(ti), Some(pi)) = ((t.inside)(q), (p.inside)(q)) {
                away.read(&b.what, q, truth(b.op, b.torus_first, Some(ti), pi), ask(q));
            }
        }
        for &q in on {
            if let Some(pi) = (p.inside)(q) {
                tube.read(&b.what, q, truth(b.op, b.torus_first, None, pi), ask(q));
            }
        }
    }
    (away, tube)
}

/// **No random query reads a wrong answer.** Every op's result over the
/// tori and poses, asked at random points off every surface and on the
/// tube; refusals are counted, never wrong answers.
#[test]
fn random_queries_read_the_truth() {
    let (tori, poses) = (tori(), poses());
    let built = built(&tori, &poses);
    assert!(built.len() >= 40, "{} bodies built", built.len());
    let mut rng = test_utils::fuzz::start("torus_region_probe");
    let n = test_utils::fuzz::scaled(6);
    let (lo, hi) = about_the_tube(RING.0, RING.1);
    let mut unit = || rng.unit();
    let off: Vec<Point3<f64>> = (0..n)
        .map(|_| {
            Point3::new(
                lo.x + (hi.x - lo.x) * unit(),
                lo.y + (hi.y - lo.y) * unit(),
                lo.z + (hi.z - lo.z) * unit(),
            )
        })
        .collect();
    let on: Vec<Point3<f64>> = (0..n)
        .map(|_| on_tube(unit() * TAU, unit() * TAU))
        .collect();
    let (away, tube) = probe(&tori, &poses, &built, &off, &on);
    for (name, t) in [("off every surface", &away), ("on the tube", &tube)] {
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
    }
}

/// **A lattice on the tube of the notched donut reads the truth**: the
/// pinned subset of [`random_queries_read_the_truth`]. The donut less
/// the notch bar, and its meet with the bar, each asked at a `24 × 12`
/// lattice of the tube's chart and at `200` fixed points off it, every
/// one the operands place answered and right.
#[test]
fn a_lattice_on_the_tube_reads_the_truth() {
    let (tori, poses) = (tori(), poses());
    let notch = poses.iter().position(|(n, _)| *n == "the notch").unwrap();
    let built: Vec<Built> = built(&tori[..1], &poses[notch..=notch])
        .into_iter()
        .filter(|b| matches!(b.op, BooleanOp::Intersect | BooleanOp::Subtract) && b.torus_first)
        .collect();
    assert_eq!(built.len(), 2, "the notch's T ∩ P and T ∖ P build");
    let (lo, hi) = about_the_tube(RING.0, RING.1);
    let off: Vec<Point3<f64>> = points(lo, hi, 200).collect();
    let on: Vec<Point3<f64>> = (0..24)
        .flat_map(|i| {
            (0..12).map(move |k| {
                on_tube(
                    (f64::from(i) + 0.13) * TAU / 24.0,
                    (f64::from(k) + 0.29) * TAU / 12.0,
                )
            })
        })
        .collect();
    let (away, tube) = probe(&tori[..1], &poses[notch..=notch], &built, &off, &on);
    for (name, t) in [("off every surface", &away), ("on the tube", &tube)] {
        assert!(
            t.wrong.is_empty() && t.refused.is_empty() && t.asked > 100,
            "{name}: {} asked, refused {:?}, wrong:\n{}",
            t.asked,
            t.refused,
            t.wrong.join("\n")
        );
    }
}
