//! **Limb 3's tube holds the traced arc and nothing else** (C2, C3).
//!
//! A search banks every cell inside a certified branch's tube as
//! accounted, so a second arc of the locus inside the tube is lost
//! without a word unless limb 3 proves there is none. A zero-free
//! transverse derivative proves only one solution per transverse line:
//! a second arc beside the first along the tube, leaving through the
//! domain's sides, passes it. These rows plant that second arc on both
//! lanes; each must be traced, or the op refuse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, BranchEnd, ChartAxis, ChartEnd, SsiDomain, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

/// A biquadratic Bézier graph `z = g(x) + h(y)` over `[x0, x1] × [0, l]`:
/// `g` and `h` are quadratics, given by their values and their slopes at
/// the low end, and the tensor net of a sum is the sum of the nets.
fn graph_wall(
    (x0, x1): (f64, f64),
    l: f64,
    (g, dg0): (impl Fn(f64) -> f64, f64),
    (h, dh0): (impl Fn(f64) -> f64, f64),
) -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let gb = [g(x0), g(x0) + 0.5 * (x1 - x0) * dg0, g(x1)];
    let hb = [h(0.0), h(0.0) + 0.5 * l * dh0, h(l)];
    let (xs, ys) = ([x0, 0.5 * (x0 + x1), x1], [0.0, 0.5 * l, l]);
    let control = (0..9)
        .map(|k| Point3::new(xs[k / 3], ys[k % 3], gb[k / 3] + hb[k % 3]))
        .collect();
    NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
}

/// **The fold: two arcs side by side along one transverse direction.**
/// The wall `z = c·x + a·x² + 4β·y(L − y)/L²` against `z = 0`, every
/// height scaled to ε (β = ε, c = 80ε, a = 0.28·c²/β, w = β/c, L = 1.2w),
/// over `x ∈ [−1.5w, 1.8w]`. `∂φ/∂x > 0` everywhere, and the locus is two
/// arcs: from `(0, 0)` and from `(0, L)` on the `v` sides, each to the low
/// `u` side, with `φ > 0` between them. A cubic from `(0, 0)` to `(0, L)`
/// stays within ε of the plane across that gap, and its one window — the
/// whole wall — is a graph over `x`-lines; the true pairing must come
/// back, with every zero of a dense grid beside a carrier.
#[test]
fn the_fold_answers_its_two_arcs_paired_as_the_locus_pairs_them() {
    let beta = eps();
    let c = 80.0 * beta;
    let a = 0.28 * c * c / beta;
    let w = beta / c;
    let l = 1.2 * w;
    let xr = (-1.5 * w, 1.8 * w);
    let g = move |x: f64| c * x + a * x * x;
    let h = move |y: f64| 4.0 * beta * y * (l - y) / (l * l);
    let wall = graph_wall(xr, l, (g, c + 2.0 * a * xr.0), (h, 4.0 * beta / l));
    let ground = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let domain = SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.0,
        extent: 1.0,
        floor_scale: 1.0,
    };
    let out = ssi::plane_nurbs_ssi(&ground, &wall, domain, band())
        .unwrap_or_else(|e| panic!("the fold: expected its two arcs, got {e}"));

    // Each branch joins a `v` side to the low `u` side.
    let key = |p: &[(ChartAxis, ChartEnd); 2]| format!("{p:?}");
    let sorted = |mut e: [(ChartAxis, ChartEnd); 2]| {
        e.sort_by_key(|s| format!("{s:?}"));
        e
    };
    let mut pairs: Vec<_> = out
        .branches
        .iter()
        .map(|b| match b.end {
            BranchEnd::Crossings { from, to } => sorted([
                (from.side.fixed, from.side.end),
                (to.side.fixed, to.side.end),
            ]),
            other => panic!("the fold: an open branch ended {other:?}"),
        })
        .collect();
    pairs.sort_by_key(key);
    let low_u = (ChartAxis::U, ChartEnd::Low);
    let mut want = vec![
        sorted([(ChartAxis::V, ChartEnd::Low), low_u]),
        sorted([(ChartAxis::V, ChartEnd::High), low_u]),
    ];
    want.sort_by_key(key);
    assert_eq!(pairs, want, "the fold: branches paired across the gap");

    let far = far_zeros(&|x, y| g(x) + h(y), xr, l, &out, 0.3 * w);
    assert_eq!(far, 0, "the fold: zeros 0.3w from every carrier");
}

/// How many zeros of `φ`, found as sign changes along a 400 × 400 grid
/// of `y` lines over `[x0, x1] × [0, l]`, lie farther than `far` in `xy`
/// from every carrier sampled at 4001 points.
fn far_zeros(
    phi: &impl Fn(f64, f64) -> f64,
    (x0, x1): (f64, f64),
    l: f64,
    out: &SsiOutcome,
    far: f64,
) -> usize {
    let pts: Vec<Point3<f64>> = out
        .branches
        .iter()
        .flat_map(|b| {
            let (t0, t1) = b.params;
            (0..=4000).map(move |k| b.carrier.eval(t0 + (t1 - t0) * f64::from(k) / 4000.0))
        })
        .collect();
    let n = 400;
    let at = |i: u32, lo: f64, hi: f64| lo + (hi - lo) * f64::from(i) / f64::from(n);
    let zeros: Vec<(f64, f64)> = (0..=n)
        .flat_map(|j| {
            let y = at(j, 0.0, l);
            (0..n).filter_map(move |i| {
                let (a, b) = (at(i, x0, x1), at(i + 1, x0, x1));
                (phi(a, y).signum() != phi(b, y).signum()).then_some((0.5 * (a + b), y))
            })
        })
        .collect();
    assert!(zeros.len() > 100, "FIXTURE: the grid finds the locus");
    zeros
        .iter()
        .filter(|(x, y)| {
            pts.iter()
                .all(|p| (p.x - x).powi(2) + (p.y - y).powi(2) > far * far)
        })
        .count()
}

/// **A short arc inside a long arc's end box, on the ℝ³ lane.** A unit
/// cylinder about `z` and a radius-3 sphere about the origin meet in the
/// circle of radius 1 at `z = √8`. The slab's `x` face at `cos 5°` and its
/// `y` face at `sin 11°` cut that circle into a long arc (11° round to
/// 355°) and a short one (5° to 11°) that continues it along the long
/// arc's end tangent. The long arc's end box at the widest rung that is a
/// graph holds the short arc, every slice of it once; the seeds trace the
/// long arc first, and a seed landing in a tube is skipped. The short arc
/// must be a branch.
#[test]
fn a_short_arc_in_a_long_arcs_end_box_is_traced() {
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 3.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let (alpha, beta) = (5.0f64.to_radians(), 11.0f64.to_radians());
    let half = 2.0;
    let domain = SsiDomain {
        center: Point3::new(alpha.cos() - half, beta.sin() - half, 2.0),
        half_extent: half,
        extent: 4.8,
        floor_scale: 1.0,
    };
    let out = ssi::cylinder_sphere_ssi(&cylinder, &sphere, domain, band())
        .unwrap_or_else(|e| panic!("the clipped circle: expected two arcs, got {e}"));
    let mid = 0.5 * (alpha + beta);
    let on_short = Point3::new(mid.cos(), mid.sin(), 8.0f64.sqrt());
    let carried = out.branches.iter().any(|b| {
        let (t0, t1) = b.params;
        (0..=2000).any(|i| {
            let q = b.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 2000.0);
            (q - on_short).norm() < 1e-3
        })
    });
    assert!(
        carried && out.branches.len() == 2,
        "the clipped circle: {} branches, the short arc carried: {carried}",
        out.branches.len()
    );
}
