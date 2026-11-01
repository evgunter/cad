//! **The rational stretch bound** (`NurbsSurface::stretch_sup`, the
//! quotient bound of `geom_core::spline::derivative` on each cell's
//! dyadic pieces) dominates the true `‖S_u‖` and `‖S_v‖`, never reads
//! above the whole-patch homogeneous recipe ([`recipe_a`], as
//! `step-import`'s `derivative_sup` spelled it), and never above the
//! polynomial control-difference bound at unit weights.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom/src/surfaces/", "crates/geom-core/src/spline/",];

use geom::NurbsSurface;
use geom_core::spline::{KnotVector, span_derivative_scale};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz;

fn surface(
    (ku, pu): (Vec<f64>, usize),
    (kv, pv): (Vec<f64>, usize),
    pts: Vec<Point3<f64>>,
    w: Vec<f64>,
) -> NurbsSurface<f64> {
    NurbsSurface::new(
        KnotVector::clamped(ku, pu).unwrap(),
        KnotVector::clamped(kv, pv).unwrap(),
        pts,
        w,
    )
    .unwrap()
}

/// The greatest certified LOWER end of `(‖S_u‖, ‖S_v‖)` over an
/// `(n + 1)²` grid: at most the true sups, so a bound below either is
/// unsound with certainty.
fn sampled_sup_floor(s: &NurbsSurface<f64>, n: usize) -> (f64, f64) {
    let si = s.map_scalar(<Interval as Real>::from_f64);
    let (u0, u1) = s.knots_u_f64_interim().domain();
    let (v0, v1) = s.knots_v_f64_interim().domain();
    let (mut fu, mut fv) = (0.0f64, 0.0f64);
    for i in 0..=n {
        for j in 0..=n {
            #[allow(clippy::cast_precision_loss)]
            // Clamped: `lo + (hi − lo)·n/n` can round one ulp past
            // `hi`, where the last span's stretch keeps growing.
            let (u, v) = (
                (u0 + (u1 - u0) * i as f64 / n as f64).clamp(u0, u1),
                (v0 + (v1 - v0) * j as f64 / n as f64).clamp(v0, v1),
            );
            let jet = si.ders(Interval::from_f64(u), Interval::from_f64(v));
            fu = fu.max(jet.du.norm().lo());
            fv = fv.max(jet.dv.norm().lo());
        }
    }
    (fu, fv)
}

/// The whole-patch homogeneous recipe, as `step-import`'s
/// `derivative_sup` spelled it: about the control
/// centroid `c`, `(max ‖A_d‖ + d·max |w_d|)/min w` per direction, with
/// `A = w·(P − c)` and `d` the net's reach from `c`, over the
/// nonempty-support coefficients of the whole patch.
fn recipe_a(s: &NurbsSurface<f64>) -> (f64, f64) {
    let (nu, nv) = s.control_counts();
    let (ctl, w) = (s.control(), s.weights());
    #[allow(clippy::cast_precision_loss)]
    let n = ctl.len() as f64;
    let c = Point3::origin()
        + ctl
            .iter()
            .fold(Vec3::zero(), |a, p| a + (*p - Point3::origin()))
            / n;
    let reach = ctl.iter().map(|p| (*p - c).norm()).fold(0.0, f64::max);
    let least = w.iter().copied().fold(f64::INFINITY, f64::min);
    let at = |i: usize, j: usize| (ctl[i * nv + j] - c, w[i * nv + j]);
    let side = |along_u: bool| {
        let kv = if along_u {
            s.knots_u_f64_interim()
        } else {
            s.knots_v_f64_interim()
        };
        let (p, t) = (kv.degree(), kv.knots());
        let (n_out, n_in) = if along_u { (nu, nv) } else { (nv, nu) };
        let (mut a, mut b) = (0.0f64, 0.0f64);
        for i in 0..n_out - 1 {
            if t[i + p + 1] - t[i + 1] <= 0.0 {
                continue;
            }
            let k = span_derivative_scale(p, t[i + 1], t[i + p + 1]);
            for j in 0..n_in {
                let ((p0, w0), (p1, w1)) = if along_u {
                    (at(i, j), at(i + 1, j))
                } else {
                    (at(j, i), at(j, i + 1))
                };
                a = a.max((p1 * w1 - p0 * w0).norm() * k);
                b = b.max((w1 - w0).abs() * k);
            }
        }
        (a + reach * b) / least
    };
    (side(true), side(false))
}

/// One row, per direction: the `Interval` door's upper end is at least
/// the sampled sup's certified floor, the `f64` door lies in the
/// `Interval` door's bracket, and the bound is at most recipe A.
fn check(name: &str, s: &NurbsSurface<f64>) {
    let (mu, mv) = s.stretch_sup();
    let (iu, iv) = s.map_scalar(<Interval as Real>::from_f64).stretch_sup();
    let (fu, fv) = sampled_sup_floor(s, 60);
    let (au, av) = recipe_a(s);
    for (dir, m, mi, floor, a) in [
        ("u", mu.get(), iu.get(), fu, au),
        ("v", mv.get(), iv.get(), fv, av),
    ] {
        assert!(
            mi.hi() >= floor && mi.lo() <= m && m <= mi.hi(),
            "{name} ({dir}): bound {m} / [{}, {}] under the sampled sup {floor}",
            mi.lo(),
            mi.hi()
        );
        assert!(
            m <= a * (1.0 + 1e-12),
            "{name} ({dir}): the quotient bound {m} above recipe A's {a}"
        );
    }
}

/// A quarter arc of radius `r` in the xy plane at height `z`, as
/// `(points, weights)`.
fn arc(r: f64, z: f64) -> ([Point3<f64>; 3], [f64; 3]) {
    (
        [
            Point3::new(r, 0.0, z),
            Point3::new(r, r, z),
            Point3::new(0.0, r, z),
        ],
        [1.0, 0.5f64.sqrt(), 1.0],
    )
}

/// The named carriers: a quarter cylinder, a cone frustum quarter, a
/// sphere octant (rational in both directions) and a patch the weights
/// carry rather than the net.
#[test]
fn the_bound_covers_the_named_carriers_and_never_reads_above_recipe_a() {
    let quad = (vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2);
    let line = (vec![0.0, 0.0, 1.0, 1.0], 1);
    // Row-major `[iu * nv + iv]`: u around the arc, v along the axis.
    let ruled = |lo: ([Point3<f64>; 3], [f64; 3]), hi: ([Point3<f64>; 3], [f64; 3])| {
        let pts = (0..3).flat_map(|i| [lo.0[i], hi.0[i]]).collect();
        let w = (0..3).flat_map(|i| [lo.1[i], hi.1[i]]).collect();
        surface(quad.clone(), line.clone(), pts, w)
    };
    check("quarter cylinder", &ruled(arc(1.0, 0.0), arc(1.0, 2.0)));
    check("cone frustum quarter", &ruled(arc(2.0, 0.0), arc(0.5, 1.5)));
    // The octant: a meridian quarter arc (x–z) swept by the azimuth
    // arc, weights the products.
    let h = 0.5f64.sqrt();
    let meridian = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let (mut pts, mut w) = (Vec::new(), Vec::new());
    let (az, aw) = arc(1.0, 0.0);
    for (i, a) in az.iter().enumerate() {
        for (j, &(rho, z)) in meridian.iter().enumerate() {
            pts.push(Point3::new(a.x * rho, a.y * rho, z));
            w.push(aw[i] * [1.0, h, 1.0][j]);
        }
    }
    check(
        "sphere octant",
        &surface(quad.clone(), quad.clone(), pts, w),
    );
    check(
        "strongly rational patch",
        &surface(
            quad.clone(),
            line,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 1.0),
                Point3::new(0.0, 0.1, 0.0),
                Point3::new(0.2, 0.1, 1.0),
                Point3::new(-0.6, 1.7, 0.0),
                Point3::new(-0.6, 1.9, 1.3),
            ],
            vec![1.0, 2.5, 4.0, 0.3, 0.57, 9.0],
        ),
    );
}

/// **At unit weights the bound is at most the polynomial
/// control-difference bound** `max_{i,j} k_i‖P_{i+1,j} − P_{i,j}‖` per
/// direction: every piece's coefficient of `S_u` is a convex
/// combination of the derivative net's entries, and the weight power's
/// are exact ones. On this net, whose largest difference is a boundary
/// row's, it reads that bound to rounding.
#[test]
fn at_unit_weights_the_bound_is_at_most_the_control_difference_bound() {
    let pts: Vec<Point3<f64>> = (0..4)
        .flat_map(|i| {
            (0..3).map(move |j| {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (i as f64, j as f64);
                Point3::new(x + 0.3 * y * y, y - 0.2 * x * x, 0.1 * x * y)
            })
        })
        .collect();
    let s = surface(
        (vec![0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0], 2),
        (vec![0.0, 0.0, 0.0, 2.0, 2.0, 2.0], 2),
        pts.clone(),
        vec![1.0; 12],
    );
    let (au, av) = recipe_a(&s);
    let (mu, mv) = s.stretch_sup();
    for (dir, m, a) in [("u", mu.get(), au), ("v", mv.get(), av)] {
        assert!(
            (m - a).abs() <= 1e-14 * a,
            "{dir}: unit weights read {m}, the polynomial bound is {a}"
        );
    }
    check("unit-weight degree-2 patch", &s);
}

/// **Soundness fuzz** (counterexample search): random rational nets,
/// degree 1–5 per direction, weights over six decades, random
/// interior knots (some repeated): every row of [`check`].
#[test]
fn the_bound_covers_random_rational_nets() {
    let mut rng = fuzz::start("stretch_sup::random");
    let cases = fuzz::scaled(25);
    let knots = |rng: &mut fuzz::Rng, p: usize, n: usize| {
        let mut k = vec![0.0; p + 1];
        let mut u = 0.0;
        while k.len() < n {
            u += rng.range(0.1, 1.0);
            for _ in 0..(1 + rng.below(p)).min(n - k.len()) {
                k.push(u);
            }
        }
        u += rng.range(0.1, 1.0);
        k.extend(std::iter::repeat_n(u, p + 1));
        k
    };
    for case in 0..cases {
        let (pu, pv) = (1 + case % 5, 1 + (case / 5) % 5);
        let (nu, nv) = (pu + 1 + rng.below(3), pv + 1 + rng.below(3));
        let (ku, kv) = (knots(&mut rng, pu, nu), knots(&mut rng, pv, nv));
        let pts: Vec<Point3<f64>> = (0..nu * nv)
            .map(|_| {
                Point3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                )
            })
            .collect();
        let w: Vec<f64> = (0..nu * nv)
            .map(|_| 10f64.powf(rng.range(-3.0, 3.0)))
            .collect();
        // Multiplicities stay at most the degree, so every vector is
        // admissible: a refusal is a harness bug, and loud.
        let (ku, kv) = (
            KnotVector::clamped(ku, pu).expect("an admissible u vector"),
            KnotVector::clamped(kv, pv).expect("an admissible v vector"),
        );
        let s = NurbsSurface::new(ku, kv, pts, w).unwrap();
        check(
            &format!(
                "case {case} (deg {pu}×{pv}, {nu}×{nv}) — {}",
                fuzz::replay()
            ),
            &s,
        );
    }
}

/// **Six decades of weight are covered, on a surface.** The curve
/// `speed_upper_bound`'s `six_decades_of_weight_are_covered` net
/// (`P₀ = P₁`, a sharp sup near `658.6`), ruled along `v` between two
/// equal rows, so `S_u` is that curve's derivative.
#[test]
fn six_decades_of_weight_are_covered() {
    let (p0, p2) = (
        Point3::new(-0.76958, 0.67766, 0.0),
        Point3::new(-0.21312, -0.94362, 0.0),
    );
    let w = [563.52, 1.1505e-3, 1.6169e-3];
    let s = surface(
        (vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2),
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        vec![p0, p0, p0, p0, p2, p2],
        w.iter().flat_map(|&x| [x, x]).collect(),
    );
    check("ruled six decades of weight", &s);
}

/// **The bound is local, per cell.** Two `u` cells (bilinear, the two
/// rows `10⁻³` apart), the far column's weight `10⁻³` and its points
/// `10⁻³` from the middle column's: each cell's bound reads its own
/// weight power, so the near cell's `‖P₁ − P₀‖ = 1` stays near `1`, and
/// the far cell's tiny step over its tiny weight near `1` too. Dividing
/// the near cell's terms by the patch's least weight instead — sound,
/// but global — reads `1000`.
#[test]
fn each_cell_reads_its_own_least_weight() {
    let (tiny, step) = (1e-3, 1e-3);
    // The two rows `step` apart: the cell's points are what `S` ranges
    // over, so a far row would read in the far cell's own terms.
    let column = |x: f64| [Point3::new(x, 0.0, 0.0), Point3::new(x, 0.0, step)];
    let s = surface(
        (vec![0.0, 0.0, 1.0, 2.0, 2.0], 1),
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        [column(0.0), column(1.0), column(1.0 + step)].concat(),
        vec![1.0, 1.0, 1.0, 1.0, tiny, tiny],
    );
    let (mu, _) = s.stretch_sup();
    let global = 1.0 / tiny;
    assert!(
        mu.get() < global / 100.0,
        "bound {} reads the near cell over the far cell's weight (global {global})",
        mu.get()
    );
    check("a tiny weight in the far cell only", &s);
}
