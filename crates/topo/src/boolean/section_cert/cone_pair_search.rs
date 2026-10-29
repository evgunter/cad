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
//! - every witness on both carriers to within its own rounding
//!   ([`WITNESS_ROUNDING`]), and on a traced component of its
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
    "crates/geom-brep/src/half_angle.rs",
    "crates/geom-core/src/bisect.rs",
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/linalg/",
];

use super::cone_search::{Class, Frame, Traced, random_cone, trace};
use super::search_kit::{band, matched, offset, unit_vec};
use super::*;
use core::f64::consts::{FRAC_PI_2, TAU};
use geom::Surface;
use geom_core::{Point3, Vec3};
use test_utils::fuzz::{self, Rng};

/// How close to `±π/2` the compactified chart runs: `L·tan ψ` reaches
/// `10⁴ L`.
const EDGE: f64 = 1e-4;

/// A section traced on a ruling chart whose columns stand `shift` off
/// `θ = 0`.
struct Ruled {
    traced: Traced,
    shift: f64,
}

impl Ruled {
    /// The components crossing the cells about chart point `(θ, ψ)`.
    fn near(&self, theta: f64, psi: f64) -> Vec<usize> {
        self.traced.near(theta - self.shift, psi)
    }
}

/// A ruling chart's line at an angle, as `(B, v)`.
type LineAt<'a> = &'a dyn Fn(f64) -> (Point3<f64>, Vec3<f64>);

/// The share of a column's width the columns stand off `θ = 0`: `2/φ³`,
/// no short dyadic share, so a doubled grid shares no column with the
/// last, and a pose symmetric about a dyadic angle puts no root on one.
const SHIFT: f64 = 0.472_135_954_999_579_4;

/// The section traced on the ruling chart `line(θ) = (B, v)`, the
/// distance along `v` compactified as `scale·tan ψ`, on the coarsest grid
/// from `720 × 721` cells up that [`resolves`] the section, each refusal
/// doubling the cells round the turn, along the lines, or both; `None`
/// past [`GRID_BUDGET`].
fn ruled((line, slop): (LineAt<'_>, Slop), scale: f64, partner: &Surface<f64>) -> Option<Ruled> {
    let window = (-FRAC_PI_2 + EDGE, FRAC_PI_2 - EDGE);
    let (mut nu, mut nv) = (720, 721);
    while nu * nv <= GRID_BUDGET {
        let grid = (nu, nv, window);
        let shift = SHIFT * TAU / nu as f64;
        // The column lines alone first; the grid is traced only on a
        // size they accept.
        let short = match resolves(grid, None, (line, slop), scale, partner) {
            Ok(()) => {
                let traced = trace(
                    &|u, psi| {
                        let (b, v) = line(u + shift);
                        b + v * (scale * psi.tan())
                    },
                    &|q| geom_brep::implicit_residual(partner, q),
                    window,
                    nu,
                    nv,
                );
                match resolves(grid, Some(&traced), (line, slop), scale, partner) {
                    Ok(()) => return Some(Ruled { traced, shift }),
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

/// How far a chart's computed line at a column stands from the exact
/// line at the exact column angle (metres, and unit): its base within
/// `base` plus two units of rounding of its distance from the origin,
/// its direction within `dir`. A column angle `2π(i + s)/N` rounds by
/// at most `20ε` rad, each sine and cosine by `ε`, and a sum or product
/// by `ε` of its size: a cone's generator is within `64ε` (its base is
/// the apex), a wall's ruling's base within `64ε·r` of its own radius.
#[derive(Clone, Copy)]
struct Slop {
    base: f64,
    dir: f64,
}

/// The [`Slop`] of the ruling chart on `carrier`.
fn slop_of(carrier: &Surface<f64>) -> Slop {
    let unit = 64.0 * f64::EPSILON;
    match *carrier {
        Surface::Cone { .. } => Slop {
            base: 0.0,
            dir: unit,
        },
        Surface::Cylinder { radius, .. } => Slop {
            base: unit * radius,
            dir: 0.0,
        },
        _ => unreachable!("the search's charts rule cylinders and cones"),
    }
}

/// `(Q₂, Q₁, Q₀)`: the partner's quadric `Q` along the line `B + t·v`,
/// `Q` positive off the carrier as its implicit residual is — against a
/// cone `|w|²cos²α − (w·a)²`, against a cylinder `|w|² − (w·d)² − r²` —
/// and a bound on how far each stands from its value on the exact line
/// `slop` puts within reach: the line's own error moves `Q₂` by at most
/// `4|δv|`, `Q₁` by `4(|δB| + |w||δv| + |δB||δv|)` and `Q₀` by
/// `4|w||δB| + 2|δB|²` (`w = B − P`, `|v| = 1`, the form's matrix of norm
/// at most one), and each sum rounds within `16ε` of its terms'
/// magnitudes.
fn along(partner: &Surface<f64>, b: Point3<f64>, v: Vec3<f64>, slop: Slop) -> ([f64; 3], [f64; 3]) {
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
    let rounding = |x: f64| 16.0 * f64::EPSILON * x;
    let db = slop.base + 2.0 * f64::EPSILON * (b - Point3::origin()).norm();
    let dv = slop.dir;
    (
        [
            k * v.dot(v) - hv * hv,
            2.0 * (k * w0.dot(v) - hw * hv),
            k * ww - hw * hw - r2,
        ],
        [
            rounding(k + hv * hv) + 4.0 * dv,
            rounding(2.0 * (k * wv + (hw * hv).abs())) + 4.0 * (db + wv * dv + db * dv),
            rounding(k * ww + hw * hw + r2) + 4.0 * wv * db + 2.0 * db * db,
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
///
/// **What each harmonic is charged.** Harmonic `k` turned to any phase,
/// `(2/N)·Σ fᵢ·cos(k(u − 2πi/N) + φ)`, moves by at most `2·noise` with
/// the samples, a single sum of their errors (so its cosine and sine
/// parts together, not `√2` times each). Its two sums round, each of
/// `N` terms whose angle `k·h·i` (under `8π`) rounds within `50ε` rad,
/// by at most `(N + 52)ε·Σ|fᵢ|`: `2(N + 52)ε` times the samples' mean
/// magnitude once halved by `N`, and `√2` that for the pair. A
/// derivative's own sum rounds within `16ε` of its terms, and so does
/// `keeps`' arithmetic.
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
    let magnitude = samples.iter().map(|f| f.abs()).sum::<f64>() / n as f64;
    let summed = core::f64::consts::SQRT_2 * 2.0 * (n as f64 + 52.0) * f64::EPSILON * magnitude;
    let per_harmonic = 2.0 * noise + summed;
    assert!(
        amp(h3) + amp(h4) <= 1e-9 * (mean.abs() + amp(h1) + amp(h2)) + 2.0 * per_harmonic,
        "a chart's coefficient of degree past 2: harmonics {h3:?} {h4:?}"
    );
    let rounded = |x: f64| 16.0 * f64::EPSILON * x;
    // `f`'s `m`-th derivative at column `i`, from the harmonics, and its
    // ceiling round the turn, each charged `(1 + 2ᵐ)` harmonics' error
    // and its own sum's rounding.
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
    let ceiling = |m: i32| amp(h1) + 2f64.powi(m) * amp(h2);
    let charge = |m: i32| (1.0 + 2f64.powi(m)) * per_harmonic + rounded(ceiling(m));
    let ceiling = |m: i32| ceiling(m) + charge(m);
    let w = h / 2.0;
    // Whether `g`, read from one end at `(g₀, g₁)` with `|g″| ≤ bend`,
    // keeps one sign over the half next to it.
    let keeps = |g0: f64, g1: f64, bend: f64, slack: f64| {
        let far = g0 + g1 * w;
        let curve = bend * w * w / 2.0;
        (g0 > 0.0) == (far > 0.0)
            && g0.abs().min(far.abs()) - slack - rounded(g0.abs() + (g1 * w).abs() + curve) > curve
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
/// Against a twin, where `Q₂` vanishes on every line, `Q₁` takes `Q₂`'s
/// place and no strip holds a fold.
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
    (line, slop): (LineAt<'_>, Slop),
    scale: f64,
    partner: &Surface<f64>,
) -> Result<(), Short> {
    if traced.is_some_and(|t| t.saddles > 0) {
        return Err(Short::Both);
    }
    let (coefficients, errors): (Vec<[f64; 3]>, Vec<[f64; 3]>) = (0..n)
        .map(|i| {
            let (b, v) = line(TAU * (i as f64 + SHIFT) / n as f64);
            along(partner, b, v, slop)
        })
        .unzip();
    // `E` is true to within what its coefficients' errors and its own two
    // products move.
    let rounding = |x: f64| 16.0 * f64::EPSILON * x;
    let e: Vec<f64> = coefficients
        .iter()
        .map(|&[a, b, c]| b * b - 4.0 * a * c)
        .collect();
    let e_noise = coefficients
        .iter()
        .zip(&errors)
        .map(|(&[a, b, c], &[da, db, dc])| {
            (2.0 * b.abs() + db) * db
                + 4.0 * (a.abs() * dc + c.abs() * da + da * dc)
                + rounding(b * b + 4.0 * (a * c).abs())
        })
        .fold(0.0, f64::max);
    let lead: Vec<f64> = coefficients.iter().map(|c| c[0]).collect();
    let lead_noise = errors.iter().map(|t| t[0]).fold(0.0, f64::max);
    // Against a twin (`Q₂` vanishing on every line) each line meets the
    // partner once, at `−Q₀/Q₁`, and that root leaves along an asymptote
    // where `Q₁` vanishes.
    let twin = lead.iter().all(|a| a.abs() <= lead_noise);
    let (folds, asymptotes) = if twin {
        let middle: Vec<f64> = coefficients.iter().map(|c| c[1]).collect();
        let middle_noise = errors.iter().map(|t| t[1]).fold(0.0, f64::max);
        (Some(vec![0; n]), strip_roots(&middle, middle_noise))
    } else {
        (strip_roots(&e, e_noise), strip_roots(&lead, lead_noise))
    };
    let (Some(folds), Some(asymptotes)) = (folds, asymptotes) else {
        return Err(Short::Turn);
    };
    // The cell of a root `t` along a column, if it is in the window.
    let cell = |t: f64| {
        let x = ((t / scale).atan() - v0) / (v1 - v0);
        (0.0..1.0).contains(&x).then_some((x * nv as f64) as usize)
    };
    let positive = |i: usize| twin || e[i] > 0.0;
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
        let roots = if twin {
            vec![-c / b]
        } else {
            let root = e[i].sqrt();
            let q = -0.5 * (b + if b < 0.0 { -root } else { root });
            vec![q / a, c / q]
        };
        let mut cells: Vec<usize> = roots.into_iter().filter_map(cell).collect();
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
/// apart from a finite tangency (metres): either apex off the other
/// carrier, a generator along the cylinder's axis, an asymptotic
/// direction the cones share doubly, parallel cones of nearly one
/// aperture (one aperture exactly is the plane conic, not a degeneracy),
/// and a tangency at infinity ([`off_asymptotic_planes`]). Angles are
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
            let planes = off_asymptotic_planes((k.apex, k.a, k.alpha), (apex, a2, half_angle)).min(
                off_asymptotic_planes((apex, a2, half_angle), (k.apex, k.a, k.alpha)),
            );
            apexes.min(directions).min(planes)
        }
        _ => f64::INFINITY,
    }
}

/// **A tangency at infinity**: how far the apex of cone `x` stands off
/// cone `y`'s tangent plane along each of `y`'s generators parallel to
/// one of `x`'s (metres). Each such generator direction `v` is an
/// asymptotic direction of the section; `x`'s line through its apex along
/// `v` meets `y` once more where `y`'s form along it has its linear term,
/// `2w·(cos²β·v − (v·b)b)` (`w` from `y`'s apex, `b` its axis, `β` its
/// half-angle), and that term over `2 cos β sin β` is the apex's distance
/// from the plane. At zero the branch leaving along `v` meets infinity
/// doubly. Parallel axes give no such direction but at one aperture, the
/// plane conic.
fn off_asymptotic_planes(
    (ax, a, alpha): (Point3<f64>, Vec3<f64>, f64),
    (ay, b, beta): (Point3<f64>, Vec3<f64>, f64),
) -> f64 {
    let (e1, e2) = a.orthonormal_basis();
    let (sa, ca) = alpha.sin_cos();
    let (sb, cb) = beta.sin_cos();
    // `v(θ)·b = c0 + r·cos(θ − φ)` round `x`'s generators.
    let c0 = ca * a.dot(b);
    let (x1, x2) = (sa * e1.dot(b), sa * e2.dot(b));
    let r = x1.hypot(x2);
    let phi = x2.atan2(x1);
    let w = ax - ay;
    [cb, -cb]
        .iter()
        .filter_map(|&target| {
            let c = (target - c0) / r;
            (r > 0.0 && c.abs() <= 1.0).then(|| c.acos())
        })
        .flat_map(|d| [phi + d, phi - d])
        .map(|theta| {
            let v = a * ca + (e1 * theta.cos() + e2 * theta.sin()) * sa;
            (w.dot(v * (cb * cb) - b * v.dot(b))).abs() / (cb * sb)
        })
        .fold(f64::INFINITY, f64::min)
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
        (&|u| (k.apex, k.a * c + k.radial(u) * s), slop_of(&cone)),
        k.scale,
        &partner.surface,
    )?;
    let theirs = ruled((&*partner.line, slop_of(&partner.surface)), k.scale, &cone)?;
    for (side, traced, flag) in [("cone", &mine, true), ("partner", &theirs, false)] {
        let mut want = traced.traced.classes.clone();
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
        // Each witness's traced components of its part's class, matched
        // one to one: two components can pass within a cell of one
        // witness.
        let mut near = Vec::new();
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
                .filter(|&i| traced.traced.classes[i] == want)
                .collect();
            near.push((w, want, on));
        }
        let candidates: Vec<Vec<usize>> = near.iter().map(|(_, _, on)| on.clone()).collect();
        if !matched(&candidates) {
            for (w, want, on) in &near {
                bad.push(format!(
                    "on the {side}'s chart the witness {w:?} of a {want:?} part is on \
                     traced components {on:?}, not each on its own"
                ));
            }
        }
    }
    if single && mine.traced.classes.len() != 1 {
        bad.push(format!("single, traced {:?}", mine.traced.classes));
    }
    Some(bad)
}

/// How far a witness may stand off each carrier, in units of rounding
/// of its distance from the cone's apex (plus the scale): the section's
/// premise that a witness is within rounding of the section
/// (`section_cert.rs`). Measured over `1.7·10⁴` draws across the three
/// ε: at most `4.1`.
const WITNESS_ROUNDING: f64 = 64.0;

/// The mismatches of one pose, as sentences.
fn check(k: &Frame, pose: &Pose) -> (Outcome, Vec<String>) {
    let cone = k.surface();
    let partner = &pose.partner.surface;
    let reach = reach_of(k);
    let b = band();
    let sec = classify(
        &cone,
        partner,
        reach,
        Some(geom_brep::SsiLane::marched()),
        b,
    );
    let back = classify(
        partner,
        &cone,
        reach,
        Some(geom_brep::SsiLane::marched()),
        b,
    );
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
        let rounding = WITNESS_ROUNDING * f64::EPSILON * ((w - k.apex).norm() + k.scale);
        for surf in [&cone, partner] {
            let r = geom_brep::implicit_residual(surf, w);
            if r.abs() > rounding {
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
            let sec = classify(
                &k.surface(),
                &pose.partner.surface,
                reach_of(&k),
                Some(geom_brep::SsiLane::marched()),
                band(),
            );
            println!(
                "CPDUMP {}",
                dump_line(name, k.scale, &k.surface(), &pose.partner.surface, &sec)
            );
        }
    }
}

/// A pinned pose: `check` finds no mismatch, and an answer is traced.
fn agrees(k: &Frame, pose: &Pose) -> Outcome {
    let (outcome, bad) = check(k, pose);
    assert!(bad.is_empty(), "{}: {}", pose.what, bad.join("\n"));
    assert!(
        !matches!(outcome, Outcome::Untraced),
        "{}: answered, and no grid within the budget resolves it",
        pose.what
    );
    outcome
}

/// A pinned pose whose subject is its answer — the trace, or its
/// witnesses — and which refuses at some `ε`: [`agrees`], and a
/// refusal at this run's says so by name.
fn answers_or_stands_down(k: &Frame, pose: &Pose) {
    if let Outcome::Refused(row) = agrees(k, pose) {
        test_utils::vacuity::stood_down(
            &pose.what,
            &format!(
                "refused {row} at a zero band of {:e} m: no answer here to compare",
                band().zero()
            ),
        );
    }
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
/// Traced at the default ε and 1e-6; it refuses its precision floor at
/// 1e-12.
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
    answers_or_stands_down(&k, &generic("parallel cone (0x9241…)", partner));
}

/// The needle as the first carrier (α 0.064) against an antiparallel
/// wide cone (α 1.07): the loop is 0.41° of the wide cone's turn.
/// Seed `0xc70ed950ae541691` at effort 1. Traced at the default ε; it
/// refuses at 1e-6 and 1e-12.
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
    answers_or_stands_down(&k, &generic("parallel cone (0xc70e…)", partner));
}

/// A needle (α 0.085) tilted 28° off a wide cone's axis (α 0.94): a
/// loop 0.47° of the wide cone's turn. Seed `0x36a100199db4330e` at
/// effort 1. Traced at the default ε; it refuses at 1e-6 and 1e-12.
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
    answers_or_stands_down(&k, &generic("tilted cone (0x36a1…)", partner));
}

/// A needle (α 0.076, scale 114) through a wall of radius 14.1: on the
/// wall's chart the loop is under a cell thick along the rulings
/// (`scale·tan ψ` stretches a cell to a metre there), and its witness
/// sits at its tip. Seed `0x8cb02b1f1a051f77` at effort 1. Traced at the
/// default ε and 1e-6; it refuses its precision floor at 1e-12.
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
    answers_or_stands_down(&k, &generic("oblique cylinder (0x8cb0…)", partner));
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

/// A wide tilted cone (α 1.36) whose apex stands 0.013 m off the
/// tangent plane of a slender one (α 0.18, scale 68) along a generator
/// parallel to its own: the branch leaving along that asymptote meets
/// infinity nearly doubly, and the refusal is the asymptote's. Seed
/// `0x38f303e137aad750` at effort 1, ε 1e-6.
#[test]
fn an_apex_by_a_tangent_plane_at_infinity_refuses_the_asymptote() {
    let k = frame(
        Point3::new(-23.1811895614227, 36.55607444236934, 31.611920273293805),
        Vec3::new(0.3979800335094433, 0.36674621230039406, 0.8408977991950847),
        0.17576210990764,
        68.12699269259281,
    );
    let partner = cone(frame(
        Point3::new(-84.12143124448805, -37.638052089923974, 18.760784823979204),
        Vec3::new(0.7541113454665372, 0.5683166465450292, -0.32913867578645545),
        1.3624890584925424,
        k.scale,
    ));
    let clear = clear_of(&k, &partner.surface);
    assert!(
        (clear - 0.0128114).abs() < 1e-5,
        "the apex's distance off the tangent plane at infinity: {clear}"
    );
    agrees(&k, &generic("tilted cone", partner));
}

/// A wall 7.2e-4 rad off a generator's direction, 0.128 m off the
/// aperture degeneracy at scale 10: `E` has a close pair 7.2e-4 rad
/// apart on the cone's chart, 0.03 m at the reach speed, but there the
/// clearance turns at 1e4 m/rad and steps 4.3e-12 m between adjacent
/// `f64` angles, past twice the zero band at ε 1e-12, so no angle reads
/// ON. Answering or refusing the precision floor are both true; refusing
/// a fold is not. Seed `0x870` at effort 1, ε 1e-12.
#[test]
fn a_wall_by_a_generator_with_a_close_fold_pair_refuses_its_representation() {
    let k = frame(
        Point3::new(-2.609741815146366, 3.2782193258260204, 3.2082599255876882),
        Vec3::new(-0.7253581497355058, 0.20710007646723827, 0.6564793316925884),
        1.0661581755130216,
        10.093963919997702,
    );
    let partner = cylinder(
        Point3::new(5.322661375079078, -2.973112552486236, -3.4648815632328605),
        Vec3::new(
            -0.7069885006214471,
            0.6607447610653276,
            -0.25215792811998045,
        ),
        3.2894046612474663,
        k.scale,
    );
    agrees(
        &k,
        &Pose {
            what: "cylinder near the aperture".to_owned(),
            partner,
            margin: Some(-0.12780809882403693),
        },
    );
}

/// A needle (α 0.053) whose apex stands 0.031 m off a cone of α 0.43
/// at scale 6.4, 1.9 of the must-answer floor. One of `E`'s roots on the
/// wide cone's chart lies just past the half-turn, where the walk's
/// bracket closes, and is reported reduced by `τ`, which is `2π` only to
/// rounding; the clearance turns fast enough there that the reduced
/// angle reads off the surface by more than its running bound, which
/// the bracket's own end covered. Answering or refusing the precision
/// floor are both true; refusing a fold is not. Seed
/// `0x572bf2ae01e17032` at effort 1, ε 1e-12.
#[test]
fn a_needle_by_an_apex_with_a_root_past_the_half_turn_refuses_its_representation() {
    let k = frame(
        Point3::new(4.89405100235706, 0.41071804295093983, 5.809918875679815),
        Vec3::new(
            -0.6622538544332499,
            -0.36045557775128784,
            0.6568802088328505,
        ),
        0.42807173332581183,
        6.385682302301973,
    );
    let partner = cone(frame(
        Point3::new(-3.254792372349779, -2.8168292596737783, 8.469314693292194),
        Vec3::new(0.10084839967860032, -0.4412079243489156, 0.8917203416845366),
        0.05318633425268832,
        k.scale,
    ));
    agrees(
        &k,
        &Pose {
            what: "cone near an apex".to_owned(),
            partner,
            margin: Some(0.03103554640036772),
        },
    );
}

/// A wall 3.3e-6 rad off a generator's direction cuts the cone in two
/// essential loops that run kilometres out along that generator, and
/// each is witnessed where it crosses the generator at `θ = 0`: 1.7e3 m
/// and 2.4e4 m from the apex, where the quadric form's coefficients
/// round as the square of that reach. Each witness stands on both
/// carriers to within its own rounding. Seed `0xc7c` at effort 1, ε
/// 1e-12; at the wider bands the pose refuses.
#[test]
fn far_loops_along_a_generator_are_witnessed_on_both_carriers() {
    let k = frame(
        Point3::new(-2.959649161888848, 10.094646702735995, -6.407224945785446),
        Vec3::new(
            -0.6590448592312982,
            0.18085875974150137,
            -0.7300342338175412,
        ),
        0.1976598600240478,
        10.095503538065289,
    );
    let partner = cylinder(
        Point3::new(-0.014701251428220541, 19.2492506498769, -6.539425000908243),
        Vec3::new(
            -0.7918717701256246,
            0.17816400892102396,
            -0.5841204375839792,
        ),
        11.031335478189714,
        k.scale,
    );
    answers_or_stands_down(
        &k,
        &Pose {
            what: "cylinder near the aperture (0xc7c)".to_owned(),
            partner,
            margin: Some(-0.0001310711404777768),
        },
    );
}

/// A needle (α 0.057) beside a wall of radius 3.8 it nearly touches
/// (0.21 m off at scale 2.8): on the wall's chart the section is one
/// loop, which the first grid reads as two unless the strips about its
/// tip are certified, by Taylor's theorem over each whole half-strip.
/// Seed `0x7f5` at effort 1. Traced at the default ε; it refuses at 1e-6
/// and 1e-12.
#[test]
fn a_needle_by_a_tangent_wall_reads_one_loop_once_its_strips_are_certified() {
    let k = frame(
        Point3::new(1.6746122268362504, 2.3879445553066465, 2.2105850668630107),
        Vec3::new(-0.5090458070579811, 0.5273353750005444, -0.6802865341822694),
        0.05682130246444809,
        2.8132633005687366,
    );
    let q = Point3::new(1.1502064488726855, 2.940870688940655, 1.4186465344299397);
    let n = geom_brep::implicit_gradient(&k.surface(), q);
    let n = n / n.norm();
    let (r, shift) = (3.8358826743617307, -0.21385489039692648);
    let partner = cylinder(
        q + n * (-r + shift),
        Vec3::new(
            -0.7568754963190673,
            0.03238056845668981,
            -0.6527564491126743,
        ),
        r,
        k.scale,
    );
    answers_or_stands_down(
        &k,
        &Pose {
            what: "cylinder near a tangency (0x7f5)".to_owned(),
            partner,
            margin: Some(shift),
        },
    );
}

/// `x` turned by `angle` about `about`'s axis through its apex.
fn turned(about: &Frame, x: &Frame, angle: f64) -> Frame {
    let a = about.a;
    let (s, c) = angle.sin_cos();
    let turn = |v: Vec3<f64>| v * c + a.cross(v) * s + a * (a.dot(v) * (1.0 - c));
    frame(
        about.apex + turn(x.apex - about.apex),
        turn(x.a),
        x.alpha,
        x.scale,
    )
}

/// The three needle rows' needles turned about the wide cone's axis by
/// a share of a first-grid column (`2π/720`): there each loop falls
/// between two columns of whichever chart it is narrow on, and only the
/// strip certificate, reading `E` between columns, refines the grid. Of
/// 48 turns per needle across two columns, these are the first each
/// needle needs it at. Traced at the default ε, the first also at 1e-6;
/// each refuses its precision floor at 1e-12.
#[test]
fn turned_needles_put_their_loops_between_columns() {
    let column = TAU / 720.0;
    let wide = frame(
        Point3::new(-1.2242364198117168, 2.6908604663467344, 2.2697869874119023),
        Vec3::new(-0.7766578854319418, 0.6257266832420425, 0.07258544534062752),
        0.9780387075993603,
        3.480389269348853,
    );
    let needle = frame(
        Point3::new(3.1463864208946175, 3.8911479163575393, -1.0672946609789895),
        wide.a,
        0.0547163656830942,
        wide.scale,
    );
    let n = turned(&wide, &needle, column / 24.0);
    answers_or_stands_down(&wide, &generic("parallel needle, 1/24 column", cone(n)));

    let a = Vec3::new(
        -0.36028068755660553,
        -0.8366754476210794,
        0.41251911655316803,
    );
    let wide = frame(
        Point3::new(0.07693663316657151, 2.32856615980017, -3.306580468032254),
        a * -1.0,
        1.0732269008406512,
        2.0304779108234405,
    );
    let needle = frame(
        Point3::new(1.7607851645201547, 0.6830273125948181, -1.406044550949951),
        a,
        0.0641900968100325,
        wide.scale,
    );
    let n = turned(&wide, &needle, column * 6.0 / 24.0);
    answers_or_stands_down(&n, &generic("antiparallel needle, 6/24 column", cone(wide)));

    let wide = frame(
        Point3::new(-0.03741468578592999, 0.7404188176378705, -0.871689350005166),
        Vec3::new(
            -0.48238260865339555,
            0.3281324536304198,
            -0.8121798518451596,
        ),
        0.9443691848509118,
        1.01447310159946,
    );
    let needle = frame(
        Point3::new(-0.8756810204060506, -0.2863071260418617, 0.5783643361779968),
        Vec3::new(0.6388220151868295, -0.6278931744885514, 0.44458586835765235),
        0.08454365565732458,
        wide.scale,
    );
    let n = turned(&wide, &needle, column * 9.0 / 24.0);
    answers_or_stands_down(&wide, &generic("tilted needle, 9/24 column", cone(n)));
}

/// A needle (α 0.067) whose apex stands 0.27 m inside a cone of α 0.81
/// at scale 8.9: the loop about it is narrower than a first-grid
/// column on the wide cone's chart, at ε 1e-6, where most needle rows
/// above refuse. Seed `0x2a22bc5af3d38eb6` at effort 1,
/// ε 1e-6; it refuses its precision floor at the default ε and 1e-12.
#[test]
fn a_needle_by_an_apex_pokes_a_loop_narrower_than_a_column() {
    let k = frame(
        Point3::new(7.007927717132548, 5.935038939797888, 1.0683566068835955),
        Vec3::new(0.11787840712768272, 0.9498930025192974, 0.28949605333737183),
        0.8074463415124604,
        8.895543835409493,
    );
    let partner = cone(frame(
        Point3::new(12.195667561631637, 12.158543985662424, 11.593116202396011),
        Vec3::new(0.4762337488272966, -0.7122064037179582, 0.5157164482358809),
        0.06739976359492808,
        k.scale,
    ));
    answers_or_stands_down(
        &k,
        &Pose {
            what: "cone near an apex (0x2a22…)".to_owned(),
            partner,
            margin: Some(-0.27355885024405024),
        },
    );
}

/// A needle (α 0.061) 0.57 m off a wall of radius 14.9 at scale 9.8:
/// on the wall's chart the loop about the needle's apex falls between
/// two columns. Seed `0x4d4a748fe1ea09f3` at effort 1, ε 1e-6. Traced
/// at the default ε and 1e-6; it refuses its precision floor at 1e-12.
#[test]
fn a_needle_by_a_wall_pokes_a_loop_between_its_columns() {
    let k = frame(
        Point3::new(-1.9565185656439894, 9.341628564204072, -7.691738381156591),
        Vec3::new(-0.5671111761686662, 0.40442484612043345, 0.717513385035468),
        0.061487051760289235,
        9.784580737670684,
    );
    let partner = cylinder(
        Point3::new(-8.528892498158598, 21.428434512782083, -3.8355493001087804),
        Vec3::new(0.7091524552411419, 0.171159149913717, 0.6839644293574585),
        14.85710036669459,
        k.scale,
    );
    answers_or_stands_down(
        &k,
        &Pose {
            what: "cylinder near the apex (0x4d4a…)".to_owned(),
            partner,
            margin: Some(0.5688147518486038),
        },
    );
}

/// A needle (α 0.053) 0.30 m off a wall of radius 2.7 at scale 4.9: the
/// loop about its apex is narrower than a first-grid column on the
/// wall's chart. Seed `0x6d588b00cc85b2e5` at effort 1, ε 1e-12. Traced
/// at the default ε and 1e-12; it refuses at 1e-6.
#[test]
fn a_needle_by_a_wall_pokes_a_loop_narrower_than_a_column_at_the_tightest_band() {
    let k = frame(
        Point3::new(4.02766137773561, 3.3345851995018396, 4.44327040420503),
        Vec3::new(0.7720196820782566, -0.05865340811414651, 0.6328865523933837),
        0.05343852982314742,
        4.880615277204269,
    );
    let partner = cylinder(
        Point3::new(8.558256209321769, 5.070386816120662, -0.6322328209182313),
        Vec3::new(0.8702771530629149, 0.16305044896520904, -0.4647926720043585),
        2.737549314299164,
        k.scale,
    );
    answers_or_stands_down(
        &k,
        &Pose {
            what: "cylinder near the apex (0x6d58…)".to_owned(),
            partner,
            margin: Some(0.30066148297649686),
        },
    );
}

/// A wall 7.0e-3 rad off a needle's generator, 0.35 m off the aperture
/// degeneracy at scale 48, 7.9 of the must-answer floor: on the cone's
/// chart a root of `E` lies just past the half-turn, and the line its
/// reduced angle reads is rounded apart from the bracket's by more than
/// the band. Answering or refusing the precision floor are both true;
/// refusing a fold is not. Seed `0x35cb9450aa39852b` at effort 1, ε
/// 1e-12.
#[test]
fn a_root_past_the_half_turn_is_read_where_it_is_reported() {
    let k = frame(
        Point3::new(30.85256699820465, 33.399133183417774, -0.896160576511555),
        Vec3::new(-0.6589461467203804, 0.223172319273962, 0.7183203266177587),
        0.05136207766015525,
        48.06445289585905,
    );
    let partner = cylinder(
        Point3::new(-14.968395079601919, 12.500056053578433, -2.8029165224851975),
        Vec3::new(-0.6916117517746735, 0.21262582048338052, 0.690263315895417),
        40.17002597820959,
        k.scale,
    );
    agrees(
        &k,
        &Pose {
            what: "cylinder near the aperture".to_owned(),
            partner,
            margin: Some(-0.34669192291764195),
        },
    );
}

/// The same past a wider cone (α 0.48), 2.0e-3 rad off its generator,
/// 0.16 m off the aperture degeneracy at scale 8.7, 8.7 of the
/// must-answer floor. Seed `0x0f100caef54214cc` at effort 1, ε 1e-12.
#[test]
fn a_root_past_the_half_turn_of_a_wide_cone_is_read_where_it_is_reported() {
    let k = frame(
        Point3::new(5.461730655310266, -0.16590273772749597, -1.2234041130359126),
        Vec3::new(-0.0861167243402746, 0.4366901588111214, -0.895480661425147),
        0.4771665364737431,
        8.683318541542752,
    );
    let partner = cylinder(
        Point3::new(-1.7785859222751252, 2.2198749865757583, -7.100300325282606),
        Vec3::new(0.37301133569430817, 0.3187888779729594, -0.871341606216695),
        8.233289199833646,
        k.scale,
    );
    agrees(
        &k,
        &Pose {
            what: "cylinder near the aperture".to_owned(),
            partner,
            margin: Some(0.16306054356518696),
        },
    );
}

/// A wide cone (α 1.45) through a wall of radius 830 at scale 757: on
/// the wall's chart the section is two essential loops, and they pass
/// within a cell of each other where the first witness stands. Each
/// witness is on a component of its own, though one of them is near
/// both. Seed `0x6dd3eb9ff55f6ca3` at effort 1. Traced at the default ε
/// and 1e-6; it refuses its precision floor at 1e-12.
#[test]
fn two_wall_loops_passing_within_a_cell_each_hold_a_witness() {
    let k = frame(
        Point3::new(140.09499842738194, 387.52215537341806, 444.1330514363964),
        Vec3::new(
            -0.8802943420054558,
            0.44248548073086597,
            -0.1711387471484936,
        ),
        1.4466280477228333,
        756.9589479493708,
    );
    let partner = cylinder(
        Point3::new(761.1434654670526, -420.1760074350959, 787.5225453895903),
        Vec3::new(-0.9000764236478213, 0.24009220903110304, 0.363618155151682),
        830.5208006897169,
        k.scale,
    );
    answers_or_stands_down(&k, &generic("oblique cylinder (0x6dd3…)", partner));
}
