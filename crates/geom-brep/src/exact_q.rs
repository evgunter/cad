//! Exact rationals for the oracle rows: a finite `f64`'s exact value,
//! the four operations, and Boehm's knot insertion over a coefficient
//! line, so a row can hold a refinement made in the ring against the
//! exact refined coefficients of the described spline.
#![allow(clippy::unwrap_used)]

use num_bigint::{BigInt, Sign};

/// An exact rational: `n / d`, `d > 0`.
#[derive(Clone, Debug)]
pub(crate) struct Q {
    n: BigInt,
    d: BigInt,
}

impl Q {
    /// The exact value of a finite `f64` (`m · 2^e`).
    pub(crate) fn of(x: f64) -> Self {
        let bits = x.to_bits();
        let sign: i64 = if bits >> 63 == 1 { -1 } else { 1 };
        let exp = i32::try_from((bits >> 52) & 0x7ff).unwrap();
        let frac = i64::try_from(bits & ((1 << 52) - 1)).unwrap();
        let (m, e) = if exp == 0 {
            (frac, -1074)
        } else {
            (frac | (1 << 52), exp - 1075)
        };
        let m = BigInt::from(sign * m);
        if e >= 0 {
            Self {
                n: m << usize::try_from(e).unwrap(),
                d: BigInt::from(1),
            }
        } else {
            Self {
                n: m,
                d: BigInt::from(1) << usize::try_from(-e).unwrap(),
            }
        }
    }

    pub(crate) fn add(&self, o: &Self) -> Self {
        Self {
            n: &self.n * &o.d + &o.n * &self.d,
            d: &self.d * &o.d,
        }
    }

    pub(crate) fn sub(&self, o: &Self) -> Self {
        Self {
            n: &self.n * &o.d - &o.n * &self.d,
            d: &self.d * &o.d,
        }
    }

    pub(crate) fn mul(&self, o: &Self) -> Self {
        Self {
            n: &self.n * &o.n,
            d: &self.d * &o.d,
        }
    }

    pub(crate) fn div(&self, o: &Self) -> Self {
        let (n, d) = (&self.n * &o.d, &self.d * &o.n);
        if d.sign() == Sign::Minus {
            Self { n: -n, d: -d }
        } else {
            Self { n, d }
        }
    }

    /// `lo ≤ self ≤ hi`, exactly.
    pub(crate) fn within(&self, lo: f64, hi: f64) -> bool {
        let (lo, hi) = (Self::of(lo), Self::of(hi));
        &lo.n * &self.d <= &self.n * &lo.d && &self.n * &hi.d <= &hi.n * &self.d
    }
}

/// `line`, the coefficients of a degree-`p` spline on `knots`, with
/// `u` inserted once (Boehm), exactly. `u` must lie strictly inside
/// the domain; the vector after is [`with_knot`]`(knots, u)`.
pub(crate) fn insert(knots: &[f64], p: usize, line: &[Q], u: f64) -> Vec<Q> {
    let n = line.len();
    // The span `s` with `knots[s] ≤ u < knots[s + 1]`.
    let s = (p..n).rfind(|&i| knots[i] <= u).unwrap();
    let uq = Q::of(u);
    (0..=n)
        .map(|i| {
            if i + p <= s {
                line[i].clone()
            } else if i > s {
                line[i - 1].clone()
            } else {
                let lo = Q::of(knots[i]);
                let a = uq.sub(&lo).div(&Q::of(knots[i + p]).sub(&lo));
                a.mul(&line[i]).add(&Q::of(1.0).sub(&a).mul(&line[i - 1]))
            }
        })
        .collect()
}

/// `knots` with `u` inserted in order.
pub(crate) fn with_knot(knots: &[f64], u: f64) -> Vec<f64> {
    let mut out = knots.to_vec();
    let at = out.partition_point(|&k| k <= u);
    out.insert(at, u);
    out
}

/// A row-major `nu × nv` net of `C` channels, each `u`-line refined by
/// inserting `us` in order (Boehm, exactly) on the degree-`p` vector
/// `knots`: the refined net and vector.
pub(crate) fn refine_u<const C: usize>(
    net: &[[Q; C]],
    (nu, nv): (usize, usize),
    (knots, p): (&[f64], usize),
    us: &[f64],
) -> (Vec<[Q; C]>, Vec<f64>) {
    let (mut net, mut knots, mut nu) = (net.to_vec(), knots.to_vec(), nu);
    for &u in us {
        let lines: Vec<[Vec<Q>; C]> = (0..nv)
            .map(|j| {
                core::array::from_fn(|c| {
                    let line: Vec<Q> = (0..nu).map(|i| net[i * nv + j][c].clone()).collect();
                    insert(&knots, p, &line, u)
                })
            })
            .collect();
        net = (0..=nu)
            .flat_map(|i| {
                let lines = &lines;
                (0..nv).map(move |j| core::array::from_fn(|c| lines[j][c][i].clone()))
            })
            .collect();
        knots = with_knot(&knots, u);
        nu += 1;
    }
    (net, knots)
}

/// The transpose of a row-major `nu × nv` net.
pub(crate) fn transposed<T: Clone>(net: &[T], (nu, nv): (usize, usize)) -> Vec<T> {
    (0..nv)
        .flat_map(|j| (0..nu).map(move |i| net[i * nv + j].clone()))
        .collect()
}
