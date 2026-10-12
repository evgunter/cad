//! **The general-pose cone rows against a traced section** (a
//! counterexample search, shape 1): a cone against an oblique cylinder
//! and against a tilted or parallel-axis cone (of another aperture, or
//! of the same one: a plane conic), at scales from 1 m to 1 km and
//! half-angles from `0.05` to `1.45`, each pose generic or built at a
//! signed offset from one degeneracy — the apex on the partner, a
//! tangency of the carriers at a chosen point, a generator along the
//! partner's asymptotic directions — the offset log-uniform from
//! `1e-12` of the scale up to half of it.
//!
//! The oracle traces the partner's residual on each carrier's own
//! ruling chart — azimuth periodic, the signed distance along the line
//! compactified as `L·tan ψ`, so a component reaching the window's edge
//! runs to infinity — counts its components by cell adjacency, and
//! classes each: touching the edge, unbounded; else essential iff it
//! crosses the seam an odd number of times. It shares no code with the
//! arm. The grid is refined until it resolves the section
//! ([`resolves`]), read off the partner's quadric along each column
//! line, which meets it at most twice: that check decides only where the
//! grid is too coarse, never a component or a class. An answer no grid
//! within the budget resolves is tallied `untraced`, and a fixed row
//! that comes to it fails. `classify` must agree with 0 mismatches:
//!
//! - on the cone's chart, part for part in count and class
//!   (`essential_f`), and on the partner's chart the same
//!   (`essential_g`);
//! - `single` only on one traced component;
//! - every witness on both carriers, and on a traced component of its
//!   part's class — distinct components for distinct parts;
//! - the swapped order the same classification.
//!
//! Below the trace's resolution (`0.03` of the scale) the class is not
//! traced: an offset inside the band's Zero (a quarter of it) must
//! refuse R-tan, and every answer's witnesses lie on both carriers. A
//! pose standing [`must_answer`] or farther from every degeneracy — the
//! built one, and each the search measures from the carriers itself
//! ([`clear_of`]) — must answer, or refuse a chart's precision floor
//! (`_precision`: the `f64` reading cannot resolve the band). Every
//! refusal is tallied by row, with the farthest each stood off its
//! degeneracies.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

test_utils::gated_to![
    "crates/topo/src/boolean/section_cert.rs",
    "crates/topo/src/boolean/section_cert/ruling.rs",
    "crates/topo/src/boolean/circle_roots.rs",
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/linalg/",
];

use super::cone_search::{Class, Frame, Traced, random_cone, trace};
use super::search_kit::{band, offset, unit_vec};
use super::*;
use core::f64::consts::{FRAC_PI_2, TAU};
use geom::Surface;
use geom_core::{Point3, Vec3};
use test_utils::fuzz::{self, Rng};

/// How close to `±π/2` the compactified chart runs: `L·tan ψ` reaches
/// `10⁴ L`.
const EDGE: f64 = 1e-4;

/// The section traced on the ruling chart `line(θ) = (B, v)`, the
/// distance along `v` compactified as `scale·tan ψ`, on the coarsest grid
/// from `720 × 721` cells up that [`resolves`] the section, each refusal
/// doubling the cells round the turn, along the lines, or both; `None`
/// past [`GRID_BUDGET`].
fn ruled(
    line: &dyn Fn(f64) -> (Point3<f64>, Vec3<f64>),
    scale: f64,
    partner: &Surface<f64>,
) -> Option<Traced> {
    let window = (-FRAC_PI_2 + EDGE, FRAC_PI_2 - EDGE);
    let (mut nu, mut nv) = (720, 721);
    while nu * nv <= GRID_BUDGET {
        let grid = (nu, nv, window);
        // The column lines alone first; the grid is traced only on a
        // size they accept.
        let short = match resolves(grid, None, line, scale, partner) {
            Ok(()) => {
                let traced = trace(
                    &|u, psi| {
                        let (b, v) = line(u);
                        b + v * (scale * psi.tan())
                    },
                    &|q| geom_brep::implicit_residual(partner, q),
                    window,
                    nu,
                    nv,
                );
                match resolves(grid, Some(&traced), line, scale, partner) {
                    Ok(()) => return Some(traced),
                    Err(short) => short,
                }
            }
            Err(short) => short,
        };
        match short {
            Short::Turn => nu *= 2,
            Short::Line => nv *= 2,
            Short::Both => (nu, nv) = (2 * nu, 2 * nv),
        }
    }
    None
}

/// The most cells [`ruled`] traces one chart on: sixteen times the
/// first grid's.
const GRID_BUDGET: usize = 16 * 720 * 721;

/// Where a grid is too coarse for the section: round the turn, along the
/// lines, or both.
enum Short {
    Turn,
    Line,
    Both,
}

/// `(Q₂, Q₁, Q₀)`: the partner's quadric `Q` along the line `B + t·v`,
/// `Q` positive off the carrier as its implicit residual is — against a
/// cone `|w|²cos²α − (w·a)²`, against a cylinder `|w|² − (w·d)² − r²` —
/// and the magnitudes of the terms each sums, which bound its rounding.
fn along(partner: &Surface<f64>, b: Point3<f64>, v: Vec3<f64>) -> ([f64; 3], [f64; 3]) {
    let (w0, n, k, r2) = match *partner {
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => (b - apex, axis.normalize(), half_angle.cos().powi(2), 0.0),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => (b - origin, axis.normalize(), 1.0, radius * radius),
        _ => unreachable!("the search's partners are cylinders and cones"),
    };
    let (hw, hv) = (w0.dot(n), v.dot(n));
    let (ww, wv) = (w0.dot(w0), w0.norm());
    (
        [
            k * v.dot(v) - hv * hv,
            2.0 * (k * w0.dot(v) - hw * hv),
            k * ww - hw * hw - r2,
        ],
        [
            k + hv * hv,
            2.0 * (k * wv + (hw * hv).abs()),
            k * ww + hw * hw + r2,
        ],
    )
}

/// The roots in each strip between consecutive columns of a
/// trigonometric polynomial `f` of degree at most 2 sampled at the `N`
/// columns `2πi/N`, each sample true to within `noise`; `None` where a
/// strip is not certified to hold none or exactly one. The harmonics are
/// read off the samples (exact for a degree under `N/2`), and give `f`'s
/// derivatives at each column and the bounds `|f⁽ᵐ⁾| ≤ Σ kᵐ·Aₖ`. Each
/// half of a strip is read by Taylor's theorem from its own end: a
/// function `g` with value `g₀` and slope `g₁` there and `|g″| ≤ B` keeps
/// the sign of its linear part, and at least the smaller of that part's
/// end magnitudes less `B·w²/2`, over a half of width `w`. A strip whose
/// halves both keep `f′`'s sign holds one root if its ends differ in sign
/// and none if they agree; one whose halves both keep `f`'s holds none.
fn strip_roots(samples: &[f64], noise: f64) -> Option<Vec<u8>> {
    let n = samples.len();
    let h = TAU / n as f64;
    let harmonic = |k: usize| {
        let (c, s) = samples
            .iter()
            .enumerate()
            .fold((0.0, 0.0), |(c, s), (i, &f)| {
                let (sn, cs) = (k as f64 * h * i as f64).sin_cos();
                (c + f * cs, s + f * sn)
            });
        (2.0 * c / n as f64, 2.0 * s / n as f64)
    };
    let [h1, h2, h3, h4] = [1, 2, 3, 4].map(harmonic);
    let amp = |(c, s): (f64, f64)| c.hypot(s);
    let mean = samples.iter().sum::<f64>() / n as f64;
    assert!(
        amp(h3) + amp(h4) <= 1e-9 * (mean.abs() + amp(h1) + amp(h2)) + 4.0 * noise,
        "a chart's coefficient of degree past 2: harmonics {h3:?} {h4:?}"
    );
    // `f`'s `m`-th derivative at column `i`, from the harmonics, and its
    // ceiling round the turn; each carries `2·Σ kᵐ` of the noise.
    let derivative = |m: i32, i: usize| {
        let u = h * i as f64;
        [(1.0, h1), (2.0, h2)]
            .iter()
            .fold(0.0, |acc, &(k, (c, s))| {
                let (sn, cs) = (k * u).sin_cos();
                let turned = match m.rem_euclid(4) {
                    0 => c * cs + s * sn,
                    1 => s * cs - c * sn,
                    2 => -(c * cs + s * sn),
                    _ => -(s * cs - c * sn),
                };
                acc + k.powi(m) * turned
            })
    };
    let charge = |m: i32| 2.0 * (1.0 + 2f64.powi(m)) * noise;
    let ceiling = |m: i32| amp(h1) + 2f64.powi(m) * amp(h2) + charge(m);
    let w = h / 2.0;
    // Whether `g`, read from one end at `(g₀, g₁)` with `|g″| ≤ bend`,
    // keeps one sign over the half next to it.
    let keeps = |g0: f64, g1: f64, bend: f64, slack: f64| {
        let far = g0 + g1 * w;
        (g0 > 0.0) == (far > 0.0) && g0.abs().min(far.abs()) - slack > bend * w * w / 2.0
    };
    (0..n)
        .map(|i| {
            let j = (i + 1) % n;
            let value = |c: usize, toward: f64| (samples[c], toward * derivative(1, c));
            let ((f0, f1), (g0, g1)) = (value(i, 1.0), value(j, -1.0));
            if f0.abs() <= noise || g0.abs() <= noise {
                return None;
            }
            let apart = (f0 > 0.0) != (g0 > 0.0);
            let slope = |c: usize, toward: f64| (derivative(1, c), toward * derivative(2, c));
            let ((d0, d1), (e0, e1)) = (slope(i, 1.0), slope(j, -1.0));
            let slack = charge(1) + charge(2) * w;
            if keeps(d0, d1, ceiling(3), slack)
                && keeps(e0, e1, ceiling(3), slack)
                && (d0 > 0.0) == (e0 > 0.0)
            {
                return Some(u8::from(apart));
            }
            let level = keeps(f0, f1, ceiling(2), noise + charge(1) * w)
                && keeps(g0, g1, ceiling(2), noise + charge(1) * w);
            (level && !apart).then_some(0)
        })
        .collect()
}

/// **Whether the grid `(nu, nv, window)` resolves the section** — once
/// `traced` on it, where given — on the ruling chart `line`
/// against `partner`, read from the partner's quadric along each column
/// line, which a line meets at most twice: its discriminant `E` and
/// leading coefficient `Q₂` are trigonometric polynomials of degree at
/// most 2 round the turn (one of `B`, `v` is constant on a ruling chart).
///
/// - **Round the turn**: every strip between columns holds at most one
///   root of `E` and of `Q₂` ([`strip_roots`]), so no loop, pinch or
///   asymptote pair hides between two columns, and each run of columns
///   on which `E > 0` holds a column whose roots in the window lie in
///   distinct cells.
/// - **Along the lines**: on each column the sign changes the grid sees
///   are the roots in the window less a pair sharing a cell, and the two
///   roots lie in distinct cells — two cells apart where they may be two
///   components (`E > 0` round the turn, or a run holding an asymptote) —
///   except beside a strip holding a fold, where they meet.
/// - **No saddle cell**, which cell adjacency would read as one
///   component.
fn resolves(
    (n, nv, (v0, v1)): (usize, usize, (f64, f64)),
    traced: Option<&Traced>,
    line: &dyn Fn(f64) -> (Point3<f64>, Vec3<f64>),
    scale: f64,
    partner: &Surface<f64>,
) -> Result<(), Short> {
    if traced.is_some_and(|t| t.saddles > 0) {
        return Err(Short::Both);
    }
    let (coefficients, terms): (Vec<[f64; 3]>, Vec<[f64; 3]>) = (0..n)
        .map(|i| {
            let (b, v) = line(TAU * i as f64 / n as f64);
            along(partner, b, v)
        })
        .unzip();
    // Each coefficient is true to within `16ε` of its terms' magnitude,
    // and `E` to within what those errors and its own two products move.
    let rounding = |x: f64| 16.0 * f64::EPSILON * x;
    let e: Vec<f64> = coefficients
        .iter()
        .map(|&[a, b, c]| b * b - 4.0 * a * c)
        .collect();
    let e_noise = coefficients
        .iter()
        .zip(&terms)
        .map(|(&[a, b, c], &[ma, mb, mc])| {
            let (da, db, dc) = (rounding(ma), rounding(mb), rounding(mc));
            (2.0 * b.abs() + db) * db
                + 4.0 * (a.abs() * dc + c.abs() * da + da * dc)
                + rounding(b * b + 4.0 * (a * c).abs())
        })
        .fold(0.0, f64::max);
    let lead: Vec<f64> = coefficients.iter().map(|c| c[0]).collect();
    let lead_noise = terms.iter().map(|t| rounding(t[0])).fold(0.0, f64::max);
    let (Some(folds), Some(asymptotes)) =
        (strip_roots(&e, e_noise), strip_roots(&lead, lead_noise))
    else {
        return Err(Short::Turn);
    };
    // The cell of a root `t` along a column, if it is in the window.
    let cell = |t: f64| {
        let x = ((t / scale).atan() - v0) / (v1 - v0);
        (0.0..1.0).contains(&x).then(|| (x * nv as f64) as usize)
    };
    let positive = |i: usize| e[i] > 0.0;
    let full_turn = (0..n).all(positive);
    // Runs of positive columns, each as its first column and length.
    let starts: Vec<usize> = if full_turn {
        vec![0]
    } else {
        (0..n)
            .filter(|&i| positive(i) && !positive((i + n - 1) % n))
            .collect()
    };
    let mut column_cells: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, &[a, b, c]) in coefficients.iter().enumerate() {
        if !positive(i) {
            continue;
        }
        let root = e[i].sqrt();
        let q = -0.5 * (b + if b < 0.0 { -root } else { root });
        let mut cells: Vec<usize> = [q / a, c / q].into_iter().filter_map(cell).collect();
        cells.sort_unstable();
        column_cells[i] = cells;
    }
    if let Some(traced) = traced {
        for (cells, &changes) in column_cells.iter().zip(&traced.column_changes) {
            let seen = match cells.as_slice() {
                [x, y] if x == y => 0,
                c => c.len(),
            };
            if seen != changes {
                return Err(Short::Line);
            }
        }
    }
    for &start in &starts {
        let len = if full_turn {
            n
        } else {
            (0..n).take_while(|&k| positive((start + k) % n)).count()
        };
        let columns: Vec<usize> = (0..len).map(|k| (start + k) % n).collect();
        // The strips the run spans, with those at its ends.
        let strips = (0..=len).map(|k| (start + n - 1 + k) % n);
        let apart = if full_turn || strips.clone().any(|s| asymptotes[s] == 1) {
            2
        } else {
            1
        };
        let beside_fold = |i: usize| folds[i] == 1 || folds[(i + n - 1) % n] == 1;
        let mut held = false;
        for &i in &columns {
            let distinct = match column_cells[i].as_slice() {
                [] => false,
                [_] => true,
                [x, y] => y - x >= 1,
                _ => unreachable!("a line meets a quadric at most twice"),
            };
            held |= distinct;
            if beside_fold(i) {
                continue;
            }
            if let [x, y] = column_cells[i].as_slice()
                && y - x < apart
            {
                return Err(Short::Line);
            }
        }
        if !held {
            return Err(Short::Both);
        }
    }
    Ok(())
}

/// A ruling chart: the line at each angle, as `(B, v)`.
type Lines = Box<dyn Fn(f64) -> (Point3<f64>, Vec3<f64>)>;

/// A partner's ruling chart and its carrier.
struct Partner {
    surface: Surface<f64>,
    line: Lines,
    /// The chart point `(θ, ψ)` of a point on the carrier.
    chart: Box<dyn Fn(Point3<f64>) -> (f64, f64)>,
}

fn cylinder(o: Point3<f64>, d: Vec3<f64>, r: f64, scale: f64) -> Partner {
    let (e1, e2) = d.orthonormal_basis();
    Partner {
        surface: Surface::Cylinder {
            origin: o,
            axis: d,
            radius: r,
            u_ref: e1,
        },
        line: Box::new(move |u| (o + (e1 * u.cos() + e2 * u.sin()) * r, d)),
        chart: Box::new(move |q| {
            let w = q - o;
            ((w.dot(e2)).atan2(w.dot(e1)), (w.dot(d) / scale).atan())
        }),
    }
}

fn cone(k: Frame) -> Partner {
    let surface = k.surface();
    let scale = k.scale;
    let k = std::rc::Rc::new(k);
    let (k1, k2) = (k.clone(), k);
    Partner {
        surface,
        line: Box::new(move |u| {
            let (s, c) = k1.alpha.sin_cos();
            (k1.apex, k1.a * c + k1.radial(u) * s)
        }),
        chart: Box::new(move |q| {
            let (u, t) = k2.chart(q);
            (u, (t / scale).atan())
        }),
    }
}

fn frame(apex: Point3<f64>, a: Vec3<f64>, alpha: f64, scale: f64) -> Frame {
    let (e1, e2) = a.orthonormal_basis();
    Frame {
        scale,
        apex,
        a,
        e1,
        e2,
        alpha,
    }
}

struct Pose {
    what: String,
    partner: Partner,
    /// The built degeneracy's margin in metres, `None` for a generic pose.
    margin: Option<f64>,
}

fn sign(rng: &mut Rng) -> f64 {
    if rng.unit() < 0.5 { 1.0 } else { -1.0 }
}

/// A unit direction at angle `beta` from `a`.
fn leaning(rng: &mut Rng, k: &Frame, beta: f64) -> Vec3<f64> {
    k.a * beta.cos() + k.radial(rng.range(0.0, TAU)) * beta.sin()
}

/// A point on the cone `t` along the generator at a random azimuth, and
/// the carrier's unit normal there.
fn on_cone(rng: &mut Rng, k: &Frame) -> (Point3<f64>, Vec3<f64>) {
    let t = k.scale * rng.range(0.3, 1.5) * sign(rng);
    let q = k.at(rng.range(0.0, TAU), t);
    let n = geom_brep::implicit_gradient(&k.surface(), q);
    (q, n / n.norm())
}

fn generic_cylinder(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let beta = rng.range(0.1, FRAC_PI_2);
    let d = leaning(rng, k, beta);
    if (d.dot(k.a).acos() - k.alpha).abs() < 0.05 {
        return None;
    }
    let o = k.apex + unit_vec(rng) * (sc * rng.range(0.0, 1.5));
    let r = sc * rng.range(0.1, 1.5);
    Some(Pose {
        what: format!("oblique cylinder o {o:?} d {d:?} r {r}"),
        partner: cylinder(o, d, r, sc),
        margin: None,
    })
}

fn cylinder_near_apex(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let beta = rng.range(0.1, FRAC_PI_2);
    let d = leaning(rng, k, beta);
    let o = k.apex + unit_vec(rng) * (sc * rng.range(0.2, 1.5));
    let dist = (k.apex - o).cross(d).norm();
    let shift = sc * offset(rng);
    let r = dist + shift;
    (r > 0.05 * sc).then(|| Pose {
        what: format!("cylinder near the apex o {o:?} d {d:?} r {r}"),
        partner: cylinder(o, d, r, sc),
        margin: Some(shift),
    })
}

fn cylinder_near_tangent(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let (q, n) = on_cone(rng, k);
    let d = n.cross(unit_vec(rng));
    if d.norm() < 0.2 {
        return None;
    }
    let d = d / d.norm();
    let r = sc * rng.range(0.1, 1.5);
    let shift = sc * offset(rng);
    let side = sign(rng);
    let o = q + n * (side * r + shift);
    Some(Pose {
        what: format!("cylinder tangent at {q:?} side {side} d {d:?} r {r}"),
        partner: cylinder(o, d, r, sc),
        margin: Some(shift),
    })
}

fn cylinder_near_aperture(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let lean = offset(rng) * 0.2;
    let d = leaning(rng, k, k.alpha + lean);
    let o = k.apex + unit_vec(rng) * (sc * rng.range(0.0, 1.5));
    let r = sc * rng.range(0.1, 1.5);
    let lever = 4.0 * reach_of(k).radius;
    Some(Pose {
        what: format!("cylinder near the aperture (lean {lean}) o {o:?} d {d:?} r {r}"),
        partner: cylinder(o, d, r, sc),
        margin: Some(lean * lever * k.alpha.sin()),
    })
}

/// A half-angle, log-uniform from a needle to a near-plane.
fn aperture(rng: &mut Rng) -> f64 {
    rng.range(0.05f64.ln(), 1.45f64.ln()).exp()
}

/// The search's cone: [`random_cone`]'s pose at any [`aperture`].
fn random_frame(rng: &mut Rng) -> Frame {
    let k = random_cone(rng);
    Frame {
        alpha: aperture(rng),
        ..k
    }
}

fn generic_cone(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let a2 = unit_vec(rng);
    let apex = k.apex + unit_vec(rng) * (sc * rng.range(0.2, 2.0));
    let alpha = aperture(rng);
    Some(Pose {
        what: format!("tilted cone apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: None,
    })
}

fn parallel_cone(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let alpha = aperture(rng);
    if (alpha.tan() - k.alpha.tan()).abs() < 0.1 {
        return None;
    }
    let apex = k.apex
        + k.radial(rng.range(0.0, TAU)) * (sc * rng.range(0.1, 1.5))
        + k.a * (sc * rng.range(-1.5, 1.5));
    let a2 = k.a * sign(rng);
    Some(Pose {
        what: format!("parallel cone apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: None,
    })
}

/// A parallel-axis cone of the same aperture: the section is a plane
/// conic.
fn parallel_twin(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let apex = k.apex
        + k.radial(rng.range(0.0, TAU)) * (sc * rng.range(0.1, 1.5))
        + k.a * (sc * rng.range(-1.5, 1.5));
    let a2 = k.a * sign(rng);
    Some(Pose {
        what: format!("parallel twin apex {apex:?} a {a2:?}"),
        partner: cone(frame(apex, a2, k.alpha, sc)),
        margin: None,
    })
}

fn cone_near_apex(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let (q, n) = on_cone(rng, k);
    let shift = sc * offset(rng);
    let apex = q + n * shift;
    let alpha = aperture(rng);
    let a2 = unit_vec(rng);
    Some(Pose {
        what: format!("cone near the apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: Some(shift * 0.5),
    })
}

fn cone_near_tangent(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let (q, n) = on_cone(rng, k);
    let g = n.cross(unit_vec(rng));
    if g.norm() < 0.2 {
        return None;
    }
    let g = g / g.norm();
    let alpha = aperture(rng);
    let a2 = g * alpha.cos() + n * (alpha.sin() * sign(rng));
    let shift = sc * offset(rng);
    let apex = q - g * (sc * rng.range(0.3, 2.0) * sign(rng)) + n * shift;
    Some(Pose {
        what: format!("cone tangent at {q:?}, apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: Some(shift * 0.5),
    })
}

fn cone_near_asymptote(rng: &mut Rng, k: &Frame) -> Option<Pose> {
    let sc = k.scale;
    let alpha = aperture(rng);
    let lean = offset(rng) * 0.2;
    let beta = if rng.unit() < 0.5 {
        k.alpha + alpha
    } else {
        (k.alpha - alpha).abs()
    } + lean;
    if !(0.05..core::f64::consts::PI - 0.05).contains(&beta) {
        return None;
    }
    let a2 = leaning(rng, k, beta);
    let apex = k.apex + unit_vec(rng) * (sc * rng.range(0.2, 2.0));
    let lever = 4.0 * reach_of(k).radius;
    Some(Pose {
        what: format!("cone near the asymptote (lean {lean}) apex {apex:?} a {a2:?} α {alpha}"),
        partner: cone(frame(apex, a2, alpha, sc)),
        margin: Some(lean * lever * 0.05),
    })
}

/// Whether the trace resolves the pose. Every asymptotic direction the
/// two carriers come near sharing is `0.05` rad clear of being shared,
/// so a bounded component stays well inside the window (near one, a
/// loop runs out to `~1/gap` scales, past the compactified chart's
/// resolution, and reads as reaching its edge); and each apex is `0.05`
/// of the scale off the other carrier, so the section does not pinch to
/// a point there. A loop about an apex is still as narrow as that cone's
/// aperture makes it, and [`ruled`] refines the grid for it.
fn traceable(k: &Frame, partner: &Surface<f64>) -> bool {
    let clear = |surface: &Surface<f64>, q: Point3<f64>| {
        geom_brep::implicit_residual(surface, q).abs() > 0.05 * k.scale
    };
    if !clear(partner, k.apex) {
        return false;
    }
    match *partner {
        Surface::Cylinder { axis, .. } => (axis.dot(k.a).abs().acos() - k.alpha).abs() > 0.05,
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            if !clear(&k.surface(), apex) {
                return false;
            }
            let beta = axis.dot(k.a).clamp(-1.0, 1.0).acos();
            [beta, core::f64::consts::PI - beta].iter().all(|&b| {
                (b - (k.alpha + half_angle)).abs() > 0.05
                    && (b - (k.alpha - half_angle).abs()).abs() > 0.05
            })
        }
        _ => true,
    }
}

/// The reach every search pose is classified with.
fn reach_of(k: &Frame) -> Reach<f64> {
    Reach {
        centre: k.apex,
        radius: 5.0 * k.scale,
    }
}

fn class_on(c: &Component<f64>, essential: bool) -> Class {
    match (c.unbounded, essential) {
        (true, _) => Class::Unbounded,
        (false, true) => Class::Essential,
        (false, false) => Class::Null,
    }
}

/// What one pose came to.
enum Outcome {
    Answered,
    /// Answered, and no grid within the budget resolves the section.
    Untraced,
    Refused(&'static str),
}

/// How far from every degeneracy, in units of [`must_answer`]'s scale,
/// a pose must answer. A tangency's discriminant bump is linear in its
/// offset, but an apex's, an aperture's and a doubled asymptote's are
/// quadratic (the discriminant vanishes to second order on a line
/// through the apex, along the axis, or doubly parallel to an asymptotic
/// direction), so their refusals reach `√(escalation · scale)`; and the
/// clearance the charts decide on is a lower bound short of the true one
/// by the partner's conditioning. Measured over 10⁴ draws at each of
/// the three ε: no refusal past `447` of these units but a precision
/// floor's.
const MUST_ANSWER: f64 = 2e3;

/// The distance from every degeneracy past which a pose at `scale` must
/// answer: [`MUST_ANSWER`] times the larger of the escalation threshold
/// and `√(escalation · scale)`.
fn must_answer(scale: f64) -> f64 {
    let e = band().escalate();
    MUST_ANSWER * e.max((e * scale).sqrt())
}

/// The pose's distance from every degeneracy the search can measure
/// apart from a tangency (metres): either apex off the other carrier, a
/// generator along the cylinder's axis, an asymptotic direction the
/// cones share doubly, parallel cones of nearly one aperture (one
/// aperture exactly is the plane conic, not a degeneracy). Angles are
/// levered by four reach radii. Read from the carriers, not the arm.
fn clear_of(k: &Frame, partner: &Surface<f64>) -> f64 {
    let lever = 4.0 * reach_of(k).radius;
    let off_cone = |apex: Point3<f64>, a: Vec3<f64>, alpha: f64, q: Point3<f64>| {
        let w = q - apex;
        let h = w.dot(a);
        ((w - a * h).norm() * alpha.cos() - h.abs() * alpha.sin()).abs()
    };
    match *partner {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let d = axis.normalize();
            let w = k.apex - origin;
            let apex = ((w - d * w.dot(d)).norm() - radius).abs();
            let aperture = (d.dot(k.a).abs().acos() - k.alpha).abs() * lever;
            apex.min(aperture)
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let a2 = axis.normalize();
            let apexes =
                off_cone(k.apex, k.a, k.alpha, apex).min(off_cone(apex, a2, half_angle, k.apex));
            let beta = a2.dot(k.a).clamp(-1.0, 1.0).acos();
            let directions = if a2.cross(k.a).norm() < 1e-12 {
                if half_angle == k.alpha {
                    f64::INFINITY
                } else {
                    (half_angle - k.alpha).abs() * lever
                }
            } else {
                [beta, core::f64::consts::PI - beta]
                    .iter()
                    .flat_map(|&b| {
                        [
                            (b - (k.alpha + half_angle)).abs(),
                            (b - (k.alpha - half_angle).abs()).abs(),
                        ]
                    })
                    .fold(f64::INFINITY, f64::min)
                    * lever
            };
            apexes.min(directions)
        }
        _ => f64::INFINITY,
    }
}

/// A chart's precision floor: the representation, not the band, cannot
/// resolve the pose, and any pose may refuse it.
fn precision_floor(row: &str) -> bool {
    row.ends_with("_precision")
}

/// A partner's ruling chart, read off its carrier.
fn partner_of(surface: &Surface<f64>, scale: f64) -> Partner {
    match *surface {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => cylinder(origin, axis.normalize(), radius, scale),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => cone(frame(apex, axis.normalize(), half_angle, scale)),
        _ => unreachable!("the search's partners are cylinders and cones"),
    }
}

/// `parts` against the section traced on both carriers' charts, the cone
/// `cone` at scale `scale`: the mismatches, as sentences (module docs).
/// The rows' check that every witness is on a component of its class.
pub(super) fn traced_mismatches(
    cone: &Surface<f64>,
    partner: &Surface<f64>,
    scale: f64,
    parts: &[Component<f64>],
    single: bool,
) -> Vec<String> {
    let &Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = cone
    else {
        unreachable!("the first carrier is a cone")
    };
    let k = frame(apex, axis.normalize(), half_angle, scale);
    against_the_trace(&k, &partner_of(partner, scale), parts, single).unwrap_or_else(|| {
        vec!["no grid within the budget resolves the section on both charts".to_owned()]
    })
}

/// The mismatches between `parts` and the traced section; `None` where
/// no grid within [`GRID_BUDGET`] resolves a chart.
fn against_the_trace(
    k: &Frame,
    partner: &Partner,
    parts: &[Component<f64>],
    single: bool,
) -> Option<Vec<String>> {
    let mut bad = Vec::new();
    let cone = k.surface();
    let (s, c) = k.alpha.sin_cos();
    let mine = ruled(
        &|u| (k.apex, k.a * c + k.radial(u) * s),
        k.scale,
        &partner.surface,
    )?;
    let theirs = ruled(&*partner.line, k.scale, &cone)?;
    for (side, traced, flag) in [("cone", &mine, true), ("partner", &theirs, false)] {
        let mut want = traced.classes.clone();
        want.sort();
        let mut got: Vec<Class> = parts
            .iter()
            .map(|p| class_on(p, if flag { p.essential_f } else { p.essential_g }))
            .collect();
        got.sort();
        if want != got {
            bad.push(format!(
                "on the {side}'s chart traced {want:?}, classified {got:?}"
            ));
        }
        let mut seen = Vec::new();
        for p in parts {
            let Some(w) = p.witness else {
                continue;
            };
            let (u, psi) = if flag {
                let (u, t) = k.chart(w);
                (u, (t / k.scale).atan())
            } else {
                (partner.chart)(w)
            };
            let want = class_on(p, if flag { p.essential_f } else { p.essential_g });
            // The traced chart's `v` is `ψ`; `near` reads it directly.
            let on: Vec<usize> = traced
                .near(u, psi)
                .into_iter()
                .filter(|&i| traced.classes[i] == want && !seen.contains(&i))
                .collect();
            match on.first() {
                Some(&i) => seen.push(i),
                None => bad.push(format!(
                    "on the {side}'s chart the witness {w:?} of a {want:?} part is on no such traced component"
                )),
            }
        }
    }
    if single && mine.classes.len() != 1 {
        bad.push(format!("single, traced {:?}", mine.classes));
    }
    Some(bad)
}

/// The mismatches of one pose, as sentences.
fn check(k: &Frame, pose: &Pose) -> (Outcome, Vec<String>) {
    let cone = k.surface();
    let partner = &pose.partner.surface;
    let reach = reach_of(k);
    let b = band();
    let sec = classify(&cone, partner, reach, b);
    let back = classify(partner, &cone, reach, b);
    let m = pose.margin.map_or(f64::INFINITY, f64::abs);
    let clear = m.min(clear_of(k, partner));
    let mut bad = Vec::new();
    let (parts, single) = match &sec {
        Section::Components { parts, single } => (parts.clone(), *single),
        Section::Tangent(name) => {
            if !matches!(back, Section::Tangent(_)) {
                bad.push(format!("one order R-tan, the other {back:?}"));
            }
            if clear >= must_answer(k.scale) && !precision_floor(name) {
                bad.push(format!(
                    "{clear:.3e} m off every degeneracy ({:.1e} of the must-answer floor), refused {name}",
                    clear / must_answer(k.scale)
                ));
            }
            return (Outcome::Refused(name), bad);
        }
        other => return (Outcome::Refused("other"), vec![format!("{other:?}")]),
    };
    if m <= 0.25 * b.zero() {
        bad.push(format!("margin {m} in the band, answered {sec:?}"));
    }
    match &back {
        Section::Components {
            parts: bp,
            single: sb,
        } if *sb == single
            && bp.len() == parts.len()
            && bp.iter().zip(&parts).all(|(x, y)| {
                (x.unbounded, x.essential_f, x.essential_g)
                    == (y.unbounded, y.essential_g, y.essential_f)
            }) => {}
        other => bad.push(format!("the swapped order disagrees: {other:?}")),
    }
    for c in &parts {
        let Some(w) = c.witness else {
            if !c.unbounded {
                bad.push("a bounded part without a witness".into());
            }
            continue;
        };
        for surf in [&cone, partner] {
            let r = geom_brep::implicit_residual(surf, w);
            if r.abs() > 1e-9 * k.scale {
                bad.push(format!("the witness {w:?} is {r} off {surf:?}"));
            }
        }
    }
    if m < 0.03 * k.scale || !traceable(k, partner) {
        return (Outcome::Answered, bad);
    }
    match against_the_trace(k, &pose.partner, &parts, single) {
        Some(traced) => {
            bad.extend(traced);
            (Outcome::Answered, bad)
        }
        None => (Outcome::Untraced, bad),
    }
}

type Poser = fn(&mut Rng, &Frame) -> Option<Pose>;

/// The pose families.
const ARMS: [(&str, Poser); 10] = [
    ("oblique cylinder", generic_cylinder),
    ("cylinder near the apex", cylinder_near_apex),
    ("cylinder near a tangency", cylinder_near_tangent),
    ("cylinder near the aperture", cylinder_near_aperture),
    ("tilted cone", generic_cone),
    ("parallel cone", parallel_cone),
    ("parallel twin", parallel_twin),
    ("cone near an apex", cone_near_apex),
    ("cone near a tangency", cone_near_tangent),
    ("cone near the asymptote", cone_near_asymptote),
];

/// **Every general-pose cone row against the traced section**, 0
/// mismatches.
#[test]
fn the_general_pose_cone_rows_agree_with_the_traced_section() {
    let mut rng = fuzz::start("section_cert_cone_pair_search");
    let mut failures = Vec::new();
    for (name, poser) in ARMS {
        let mut tally = std::collections::BTreeMap::<String, usize>::new();
        let mut farthest = std::collections::BTreeMap::<&str, f64>::new();
        let mut done = 0;
        while done < fuzz::scaled(12) {
            let k = random_frame(&mut rng);
            let Some(pose) = poser(&mut rng, &k) else {
                continue;
            };
            done += 1;
            let (outcome, bad) = check(&k, &pose);
            let key = match outcome {
                Outcome::Answered => "answered".to_owned(),
                Outcome::Untraced => "answered, untraced".to_owned(),
                Outcome::Refused(row) => {
                    // The farthest a refusal stood off every degeneracy,
                    // as a share of the must-answer floor.
                    let clear = pose
                        .margin
                        .map_or(f64::INFINITY, f64::abs)
                        .min(clear_of(&k, &pose.partner.surface));
                    let far = farthest.entry(row).or_insert(0.0f64);
                    *far = far.max(clear / must_answer(k.scale));
                    format!("R-tan {row}")
                }
            };
            *tally.entry(key).or_default() += 1;
            for b in bad {
                failures.push(format!(
                    "{name}: cone apex {:?} axis {:?} α {} scale {}; {}, margin {:?}: {b}",
                    k.apex, k.a, k.alpha, k.scale, pose.what, pose.margin
                ));
            }
        }
        println!(
            "[cone pair search] {name}: {tally:?}; farthest refusal (share of the must-answer floor) {farthest:?}"
        );
    }
    assert!(
        failures.is_empty(),
        "{} mismatches ({}):\n{}",
        failures.len(),
        fuzz::replay(),
        failures.join("\n")
    );
}

fn json_point(p: Point3<f64>) -> String {
    format!("[{:e}, {:e}, {:e}]", p.x, p.y, p.z)
}

fn json_vec(v: Vec3<f64>) -> String {
    format!("[{:e}, {:e}, {:e}]", v.x, v.y, v.z)
}

fn json_surface(s: &Surface<f64>) -> String {
    match *s {
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => format!(
            r#"{{"kind": "cone", "apex": {}, "axis": {}, "half_angle": {half_angle:e}}}"#,
            json_point(apex),
            json_vec(axis)
        ),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => format!(
            r#"{{"kind": "cylinder", "origin": {}, "axis": {}, "radius": {radius:e}}}"#,
            json_point(origin),
            json_vec(axis)
        ),
        _ => unreachable!("the search's partners are cylinders and cones"),
    }
}

/// One pose as the oracle reads it: the family, the scale, the cone,
/// the partner and `classify`'s answer, as JSON.
pub(super) fn dump_line(
    family: &str,
    scale: f64,
    cone: &Surface<f64>,
    partner: &Surface<f64>,
    sec: &Section<f64>,
) -> String {
    let answer = match sec {
        Section::Components { parts, single } => {
            let parts: Vec<String> = parts
                .iter()
                .map(|p| {
                    format!(
                        r#"{{"unbounded": {}, "ef": {}, "eg": {}, "witness": {}}}"#,
                        p.unbounded,
                        p.essential_f,
                        p.essential_g,
                        p.witness.map_or_else(|| "null".to_owned(), json_point)
                    )
                })
                .collect();
            format!(r#"{{"parts": [{}], "single": {single}}}"#, parts.join(", "))
        }
        Section::Tangent(name) => format!("\"R-tan {name}\""),
        other => format!("\"{}\"", format!("{other:?}").replace('"', "")),
    };
    format!(
        r#"{{"family": "{family}", "scale": {scale:e}, "cone": {}, "partner": {}, "answer": {answer}}}"#,
        json_surface(cone),
        json_surface(partner)
    )
}

/// **The draws, for the high-precision oracle.** Prints one JSON line
/// per pose, tagged `CPDUMP`: the family, the scale, the cone, the
/// partner and `classify`'s answer.
/// `scripts/oracles/cone_pair_sections_mpmath.py` re-reads each
/// answered pose's section at 50 digits on both carriers' charts and
/// checks the counts, the classes, every witness and `single`. Run:
/// `cargo nextest run -p topo --lib --run-ignored only
/// dump_for_the_mpmath_oracle --no-capture | sed -n 's/^CPDUMP //p' >
/// /tmp/cp.jsonl`, then
/// `python3 scripts/oracles/cone_pair_sections_mpmath.py /tmp/cp.jsonl`.
#[test]
#[ignore = "an oracle dump; run command in the docs"]
fn dump_for_the_mpmath_oracle() {
    let mut rng = fuzz::start("section_cert_cone_pair_search::dump_for_the_mpmath_oracle");
    for (name, poser) in ARMS {
        let mut done = 0;
        while done < fuzz::scaled(10) {
            let k = random_frame(&mut rng);
            let Some(pose) = poser(&mut rng, &k) else {
                continue;
            };
            done += 1;
            let sec = classify(&k.surface(), &pose.partner.surface, reach_of(&k), band());
            println!(
                "CPDUMP {}",
                dump_line(name, k.scale, &k.surface(), &pose.partner.surface, &sec)
            );
        }
    }
}

/// A pinned pose: `check` finds no mismatch, and an answer is traced.
fn agrees(k: &Frame, pose: &Pose) {
    let (outcome, bad) = check(k, pose);
    assert!(bad.is_empty(), "{}: {}", pose.what, bad.join("\n"));
    assert!(
        !matches!(outcome, Outcome::Untraced),
        "{}: answered, and no grid within the budget resolves it",
        pose.what
    );
}

fn generic(what: &str, partner: Partner) -> Pose {
    Pose {
        what: what.to_owned(),
        partner,
        margin: None,
    }
}

/// A needle cone (α 0.055) parallel to a wide one (α 0.98) pokes one
/// loop through it, null on the wide cone and 0.49° of its turn: inside
/// one column of the first grid. Seed `0x924171afe325def2` at effort 1.
#[test]
fn a_parallel_needle_pokes_a_loop_narrower_than_a_column() {
    let k = frame(
        Point3::new(-1.2242364198117168, 2.6908604663467344, 2.2697869874119023),
        Vec3::new(-0.7766578854319418, 0.6257266832420425, 0.07258544534062752),
        0.9780387075993603,
        3.480389269348853,
    );
    let partner = cone(frame(
        Point3::new(3.1463864208946175, 3.8911479163575393, -1.0672946609789895),
        k.a,
        0.0547163656830942,
        k.scale,
    ));
    agrees(&k, &generic("parallel cone", partner));
}

/// The needle as the first carrier (α 0.064) against an antiparallel
/// wide cone (α 1.07): the loop is 0.41° of the wide cone's turn.
/// Seed `0xc70ed950ae541691` at effort 1.
#[test]
fn an_antiparallel_wide_cone_takes_a_loop_narrower_than_a_column() {
    let k = frame(
        Point3::new(1.7607851645201547, 0.6830273125948181, -1.406044550949951),
        Vec3::new(
            -0.36028068755660553,
            -0.8366754476210794,
            0.41251911655316803,
        ),
        0.0641900968100325,
        2.0304779108234405,
    );
    let partner = cone(frame(
        Point3::new(0.07693663316657151, 2.32856615980017, -3.306580468032254),
        k.a * -1.0,
        1.0732269008406512,
        k.scale,
    ));
    agrees(&k, &generic("parallel cone", partner));
}

/// A needle (α 0.085) tilted 28° off a wide cone's axis (α 0.94): a
/// loop 0.47° of the wide cone's turn. Seed `0x36a100199db4330e` at
/// effort 1.
#[test]
fn a_tilted_needle_pokes_a_loop_narrower_than_a_column() {
    let k = frame(
        Point3::new(-0.03741468578592999, 0.7404188176378705, -0.871689350005166),
        Vec3::new(
            -0.48238260865339555,
            0.3281324536304198,
            -0.8121798518451596,
        ),
        0.9443691848509118,
        1.01447310159946,
    );
    let partner = cone(frame(
        Point3::new(-0.8756810204060506, -0.2863071260418617, 0.5783643361779968),
        Vec3::new(0.6388220151868295, -0.6278931744885514, 0.44458586835765235),
        0.08454365565732458,
        k.scale,
    ));
    agrees(&k, &generic("tilted cone", partner));
}

/// A needle (α 0.076, scale 114) through a wall of radius 14.1: on the
/// wall's chart the loop is under a cell thick along the rulings
/// (`scale·tan ψ` stretches a cell to a metre there), and its witness
/// sits at its tip. Seed `0x8cb02b1f1a051f77` at effort 1.
#[test]
fn a_needle_through_a_wall_cuts_a_loop_thinner_than_a_row() {
    let k = frame(
        Point3::new(-104.0899309254388, -19.15285302422717, -108.5326162148933),
        Vec3::new(
            -0.48167379885257966,
            0.026245568556438097,
            -0.8759574884833591,
        ),
        0.07597174100034275,
        114.1474179272722,
    );
    let partner = cylinder(
        Point3::new(-178.95139890292558, -116.2197552299878, -51.84854674756527),
        Vec3::new(0.40887315477552494, 0.7758904196716273, -0.4804339704534811),
        14.141020393308477,
        k.scale,
    );
    agrees(&k, &generic("oblique cylinder", partner));
}

/// A wall 6.3e-4 rad off a generator's direction, 0.103 m off the
/// aperture degeneracy at scale 30: near that generator the clearance
/// turns at 1.5e4 m/rad, so it steps 1.7e-12 m between adjacent `f64`
/// angles, past the zero band at ε 1e-12. Answering or refusing the
/// precision floor are both true; refusing a fold is not. Seed
/// `0x65a60eddf0c5bc9c` at effort 1, ε 1e-12.
#[test]
fn a_wall_near_the_aperture_refuses_no_fold_past_the_floor() {
    let k = frame(
        Point3::new(-28.924207277765834, -1.5123477261036697, 2.3533239296898643),
        Vec3::new(
            -0.31791937683018767,
            -0.6852549429731327,
            -0.655250282691121,
        ),
        0.27630426497190785,
        29.79211117877402,
    );
    let partner = cylinder(
        Point3::new(-21.855617919159616, -2.897427238216567, 35.35622257066682),
        Vec3::new(
            -0.38428223035007797,
            -0.8196199083608736,
            -0.42491219475990905,
        ),
        14.121612464241352,
        k.scale,
    );
    agrees(
        &k,
        &Pose {
            what: "cylinder near the aperture".to_owned(),
            partner,
            margin: Some(-0.10318818850341424),
        },
    );
}

/// A wall 4.6e-4 rad off a generator's direction, 0.044 m off the
/// aperture degeneracy at scale 6.2: the clearance's own rounding near
/// that generator reaches 1e-11 m, past the zero band at ε 1e-12, so the
/// reading refuses on its representation. Seed `0xebb7bc665258ed68` at
/// effort 1, ε 1e-12.
#[test]
fn a_wall_nearer_the_aperture_refuses_its_representation_not_a_fold() {
    let k = frame(
        Point3::new(0.9220795969198994, -3.8241686060474565, 4.924470233870227),
        Vec3::new(0.5665416119527679, -0.49198096939102726, 0.6610486575759947),
        0.8782954678083735,
        6.214184894696136,
    );
    let partner = cylinder(
        Point3::new(-2.525613531758197, 2.975569315614362, 5.319708258446635),
        Vec3::new(0.3666679340327699, 0.3048335465225907, 0.8789943885296425),
        2.48957242104901,
        k.scale,
    );
    agrees(
        &k,
        &Pose {
            what: "cylinder near the aperture".to_owned(),
            partner,
            margin: Some(-0.04405854296521363),
        },
    );
}

/// A wide cone (α 1.20) 1.6e-4 rad off the aperture against a wall of
/// radius 7.2, 0.056 m off the degeneracy at scale 19. Seed
/// `0x014b7a7e9ecc7c78` at effort 1, ε 1e-12.
#[test]
fn a_wide_cone_near_the_aperture_refuses_its_representation_not_a_fold() {
    let k = frame(
        Point3::new(-12.75796991721193, -6.266245911750467, -7.955083979702845),
        Vec3::new(
            -0.028382438080592175,
            -0.3147809720688025,
            -0.9487398889221541,
        ),
        1.1995092385461592,
        18.981826263359,
    );
    let partner = cylinder(
        Point3::new(-6.170523193628332, 5.7846579606421376, -5.25767955574565),
        Vec3::new(0.844445016148365, -0.4732990453473404, -0.2507999768251258),
        7.165849073817154,
        k.scale,
    );
    agrees(
        &k,
        &Pose {
            what: "cylinder near the aperture".to_owned(),
            partner,
            margin: Some(-0.0557884143286041),
        },
    );
}

/// A near-plane cone (α 1.30) leaning 2.3e-4 rad past the aperture
/// against a wall of radius 3.5, 0.029 m off the degeneracy at scale
/// 6.4. Seed `0xd9a828f83a67bee5` at effort 1, ε 1e-12.
#[test]
fn a_flat_cone_past_the_aperture_refuses_its_representation_not_a_fold() {
    let k = frame(
        Point3::new(-5.404455156173022, 6.3323224617304055, -0.7173140731405656),
        Vec3::new(0.7861875149531873, -0.3410594955141161, -0.515351930093784),
        1.3018342060975998,
        6.406465737516798,
    );
    let partner = cylinder(
        Point3::new(-2.344038495307002, 6.592652057059705, 3.3840830781823925),
        Vec3::new(0.31136961674256686, 0.7734749503390725, -0.5520737839886588),
        3.4535812614058807,
        k.scale,
    );
    agrees(
        &k,
        &Pose {
            what: "cylinder near the aperture".to_owned(),
            partner,
            margin: Some(0.028802076426786484),
        },
    );
}
