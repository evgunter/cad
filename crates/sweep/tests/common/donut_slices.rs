//! **The donut slicing oracle**: the volume of the donut (`R = 2`,
//! `r = 1/2`) against a straight rod by a boolean op, read slice by slice
//! along the donut's axis and line by line across each slice, each line's
//! measure the interval sets' own (the annulus the donut cuts and the
//! band or disc the rod cuts), so nothing of the kernel's reading enters
//! it and no body's volume is formed from another's. Coordinates are the
//! donut's: `z` its axis. Each integral runs under `x = m + h·sin θ` on the
//! pieces between its square-root edges, which reads those edges as
//! smooth, by the 7–15 panel halved to `1e-13`.

use crate::common::quadrature::gk15;
use core::f64::consts::FRAC_PI_2;

const BIG: f64 = 2.0;
const SMALL: f64 = 0.5;

/// The boolean op, the donut first.
#[derive(Clone, Copy, Debug)]
pub enum Op {
    Union,
    Intersect,
    /// The donut less the rod.
    Subtract,
}

/// A rod of radius `rc`, in the donut's frame.
#[derive(Clone, Copy, Debug)]
pub enum Rod {
    /// Along `z`, `|z| ≤ 1`, through `(x, y)`.
    AlongZ { at: (f64, f64), rc: f64 },
    /// Along `x` from `x₀` to `x₁`, its axis at `(y, z)`.
    AlongX {
        x: (f64, f64),
        at: (f64, f64),
        rc: f64,
    },
}

fn adapt(f: &dyn Fn(f64) -> f64, a: f64, b: f64, tol: f64, depth: u32) -> f64 {
    let (k, e) = gk15(f, a, b);
    if e <= tol || depth >= 40 {
        return k;
    }
    let m = 0.5 * (a + b);
    adapt(f, a, m, 0.5 * tol, depth + 1) + adapt(f, m, b, 0.5 * tol, depth + 1)
}

/// `∫ f` over the pieces between `cuts`, each under the sine map.
fn integral(f: &dyn Fn(f64) -> f64, mut cuts: Vec<f64>) -> f64 {
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();
    cuts.windows(2)
        .map(|w| {
            let (m, h) = (0.5 * (w[0] + w[1]), 0.5 * (w[1] - w[0]));
            let g = |th: f64| f(m + h * th.sin()) * h * th.cos();
            adapt(&g, -FRAC_PI_2, FRAC_PI_2, 1e-13, 0)
        })
        .sum()
}

/// The total length of a set of intervals, merged.
fn length(mut set: Vec<(f64, f64)>) -> f64 {
    set.retain(|(a, b)| b > a);
    set.sort_by(|p, q| p.0.total_cmp(&q.0));
    let mut total = 0.0;
    let mut reach = f64::NEG_INFINITY;
    for (a, b) in set {
        let a = a.max(reach);
        if b > a {
            total += b - a;
        }
        reach = reach.max(b);
    }
    total
}

fn overlap(a: &[(f64, f64)], b: &[(f64, f64)]) -> Vec<(f64, f64)> {
    a.iter()
        .flat_map(|p| b.iter().map(move |q| (p.0.max(q.0), p.1.min(q.1))))
        .filter(|(lo, hi)| hi > lo)
        .collect()
}

fn root(x: f64) -> f64 {
    x.max(0.0).sqrt()
}

impl Rod {
    /// The rod's `y`-set on the line at `x` of the slice at `z`.
    fn across(self, x: f64, z: f64) -> Vec<(f64, f64)> {
        let (y, h) = match self {
            Self::AlongZ { at, rc } if z.abs() <= 1.0 => (at.1, root(rc * rc - (x - at.0).powi(2))),
            Self::AlongX {
                x: (x0, x1),
                at,
                rc,
            } if x0 <= x && x <= x1 => (at.0, root(rc * rc - (z - at.1).powi(2))),
            _ => return Vec::new(),
        };
        if h > 0.0 {
            vec![(y - h, y + h)]
        } else {
            Vec::new()
        }
    }

    /// Where the rod's `y`-set has an edge across a slice.
    fn x_cuts(self) -> [f64; 2] {
        match self {
            Self::AlongZ { at, rc } => [at.0 - rc, at.0 + rc],
            Self::AlongX { x, .. } => [x.0, x.1],
        }
    }

    /// The rod's extent along `z`, and where its slices change shape.
    fn z_cuts(self) -> Vec<f64> {
        match self {
            Self::AlongZ { at, rc } => {
                let d = at.0.hypot(at.1);
                let mut cuts = vec![-1.0, 1.0];
                // Where the disc's circle touches one of the annulus's.
                for w in [BIG - d - rc, BIG - d + rc, d - rc - BIG, d + rc - BIG] {
                    if w > 0.0 && w < SMALL {
                        let z = root(SMALL * SMALL - w * w);
                        cuts.extend([-z, z]);
                    }
                }
                cuts
            }
            Self::AlongX { at, rc, .. } => vec![at.1 - rc, at.1 + rc],
        }
    }
}

/// The area of `op` on the slice at height `z`.
fn slice(z: f64, op: Op, rod: Rod) -> f64 {
    let w = root(SMALL * SMALL - z * z);
    let (a, b) = (BIG - w, BIG + w);
    let annulus = |x: f64| -> Vec<(f64, f64)> {
        if w <= 0.0 || x.abs() >= b {
            return Vec::new();
        }
        let yb = root(b * b - x * x);
        if x.abs() < a {
            let ya = root(a * a - x * x);
            vec![(-yb, -ya), (ya, yb)]
        } else {
            vec![(-yb, yb)]
        }
    };
    let measure = |x: f64| {
        let (p, q) = (annulus(x), rod.across(x, z));
        match op {
            Op::Union => length(p.into_iter().chain(q).collect()),
            Op::Intersect => length(overlap(&p, &q)),
            Op::Subtract => length(p.clone()) - length(overlap(&p, &q)),
        }
    };
    let mut cuts = rod.x_cuts().to_vec();
    if w > 0.0 {
        cuts.extend([-b, -a, a, b]);
    }
    integral(&measure, cuts)
}

/// The volume of `op` of the donut and `rod`.
pub fn volume(op: Op, rod: Rod) -> f64 {
    let mut cuts = rod.z_cuts();
    cuts.extend([-SMALL, SMALL]);
    let (lo, hi) = match op {
        Op::Union => (
            cuts.iter().copied().fold(f64::INFINITY, f64::min),
            cuts.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        ),
        Op::Intersect | Op::Subtract => (-SMALL, SMALL),
    };
    cuts.retain(|&z| z >= lo && z <= hi);
    integral(&|z| slice(z, op, rod), cuts)
}

/// **The oracle reads the closed forms**: the donut alone `2π²Rr²` (a rod
/// clear of it subtracts nothing and meets nothing), and a rod clear of
/// it beside it, along either axis.
#[test]
fn the_donut_slicing_oracle_reads_the_closed_forms() {
    let pi = core::f64::consts::PI;
    let donut = 2.0 * pi * pi * BIG * SMALL * SMALL;
    for (rod, alone) in [
        (
            Rod::AlongZ {
                at: (5.0, 0.0),
                rc: 0.3,
            },
            2.0 * pi * 0.09,
        ),
        (
            Rod::AlongX {
                x: (-1.0, 1.0),
                at: (0.0, 0.0),
                rc: 0.3,
            },
            2.0 * pi * 0.09,
        ),
    ] {
        let got = volume(Op::Subtract, rod);
        assert!((got - donut).abs() < 1e-10, "{rod:?}: {got} vs {donut}");
        let union = volume(Op::Union, rod);
        assert!((union - donut - alone).abs() < 1e-10, "{rod:?}: {union}");
        assert!(volume(Op::Intersect, rod).abs() < 1e-12, "{rod:?}");
    }
}
