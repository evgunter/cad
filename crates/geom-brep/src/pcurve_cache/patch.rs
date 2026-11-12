//! **An analytic chart written as a bilinear patch where its image
//! runs**: the spline operand the fitted door's composite reads
//! (`fitted_lane`), for an image on a chart that is affine along it.
//!
//! - A **plane** is affine everywhere, `S(u, v) = o + u·e₁ + v·e₂`, so
//!   the degree-(1, 1) patch on the four corners of any window IS the
//!   plane, wherever the image runs.
//! - A **cylinder** or **cone** is affine along a ruling, `S(u₀, v)`
//!   linear in `v`. An image whose `u` is one value `u₀` (an iso line
//!   moving in `v` alone, or a curve in UV whose controls share their
//!   `u`) runs on that ruling, so the patch whose two `u` columns are
//!   both `S(u₀, ·)` agrees with the chart wherever the image runs.
//!
//! Either way `W(P(t)) = S(P(t))` exactly, so the composite's bound on
//! `sup |W(P(t)) − C(t)|` is a bound on the image's miss from its
//! carrier, with no remainder. The corners are the chart's own
//! evaluation in interval arithmetic, so they enclose the chart the
//! stored scalars state at every scalar.
//!
//! - Anywhere else on a **curved** chart (a cylinder or cone off one
//!   ruling, a sphere, a torus, a spindle) no bilinear patch agrees with
//!   the chart along the image, so the patch **encloses** it instead
//!   ([`PatchClass::Enclosed`]): every angle the chart turns through is
//!   read on pieces of its window by its degree-[`DEGREE`] Taylor
//!   polynomial about each piece's midpoint, written in Bernstein form,
//!   with each control widened by the Lagrange remainder
//!   `d^(p+1)/(p+1)!` of a cosine or sine over the piece's half-width `d`.
//!   The chart is `O + radial(u)·ρ(v) + axis·h(v)`, linear in its slant on a
//!   cylinder or cone and a polynomial in `cos v`, `sin v` elsewhere, so the
//!   tensor product of those enclosures, `Σ B_i(u)·B_j(v)·P_ij`, holds
//!   `S(u, v)` at every `(u, v)` of the window: the Bernstein basis is
//!   non-negative and sums to one, so a control enclosure widened by `e`
//!   holds the polynomial's value moved by any `|E| ≤ e`. The composite's
//!   sup against it bounds `|S(P(t)) − C(t)|` again, its remainder inside
//!   the controls.

use std::sync::Arc;

use geom::{NurbsCurve2, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Interval, Point2, Point3, Real, Vec2};

use super::Pcurve;

/// **Where an analytic chart agrees with a bilinear patch along an
/// image**: decided by structure alone (the chart's kind and the image's
/// shape), never by a measured size, so the routing and the patch read
/// one answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PatchClass {
    /// A plane, under any image.
    Plane,
    /// A cylinder's or a cone's ruling, under an image whose `u` is one
    /// value.
    Ruling,
    /// A curved chart, under an image off one ruling: the patch encloses
    /// the chart over the window, its remainder in its controls.
    Enclosed,
}

/// **A stated image on a patch, as the composite reads it**: an iso
/// line, or a curve in UV it already is.
#[derive(Clone, Copy, Debug)]
pub(crate) enum PatchImage<'a, T: Real> {
    /// `p0 + pl·t`.
    Iso(Point2<T>, Vec2<T>),
    /// A curve in UV (`General` or `Fitted`).
    Net(&'a Arc<NurbsCurve2<T>>),
}

impl PatchClass {
    /// The class of `image` on `surface`, with the image's form, or
    /// `None` where no patch reads it: a spline chart (its own door's)
    /// and a closed-form image.
    pub(crate) fn of<'a, T: Real>(
        surface: &Surface<T>,
        image: &'a Pcurve<T>,
    ) -> Option<(Self, PatchImage<'a, T>)> {
        match image {
            Pcurve::IsoLine { p0, pl } => Self::along(surface, || geom_core::is_exact_zero(pl.x))
                .map(|class| (class, PatchImage::Iso(*p0, *pl))),
            Pcurve::General(net) | Pcurve::Fitted(net) => Self::along(surface, || {
                net.control().split_first().is_none_or(|(first, rest)| {
                    rest.iter().all(|p| geom_core::is_exact_zero(p.x - first.x))
                })
            })
            .map(|class| (class, PatchImage::Net(net))),
            Pcurve::Harmonic { .. }
            | Pcurve::FocalSection(_)
            | Pcurve::Spiric { .. }
            | Pcurve::IsoArc { .. }
            | Pcurve::Projected(_) => None,
        }
    }

    /// The class on `surface` of an image that runs on one ruling where
    /// `on_ruling` says so.
    fn along<T: Real>(surface: &Surface<T>, on_ruling: impl FnOnce() -> bool) -> Option<Self> {
        match surface {
            Surface::Plane { .. } => Some(Self::Plane),
            Surface::Cylinder { .. } | Surface::Cone { .. } => Some(if on_ruling() {
                Self::Ruling
            } else {
                Self::Enclosed
            }),
            Surface::Sphere { .. } | Surface::Torus { .. } | Surface::Spindle { .. } => {
                Some(Self::Enclosed)
            }
            Surface::Nurbs(_) | Surface::Approx(_) => None,
        }
    }
}

/// A spline chart operand of limb 2's composite, owned: its two knot
/// vectors, weights and certified control coordinates
/// (`ssi::certify::ChartOperand`).
pub(crate) type Patch = (KnotVector, KnotVector, Vec<f64>, Vec<Vec<Interval>>);

/// Why no patch was formed over a window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PatchRefusal {
    /// A window that is no number, or one that widening cannot open (a
    /// coordinate past `2^53`, where `x ± 1` rounds to `x`): the stated
    /// image's structure, not a miss, and no chart a construction mints
    /// holds such coordinates.
    NoWidth,
    /// An enclosing patch that would need more than [`MAX_PIECES`] pieces
    /// along one angle to hold its remainder under the band: an image
    /// turning that far, or a chart that large, is past this door.
    Pieces,
}

/// **The patch** of `class` on `chart` (the chart lifted to intervals),
/// over the window `u`, `v` that holds the image's control coordinates:
/// widened by one either side, so a window of one value is a patch, and
/// the image's convex hull lies inside it. On a ruling the `u` window is
/// read for its value `u` (one value, a point or an enclosure of it). An
/// enclosing patch reads `band` for the remainder its pieces hold.
///
/// # Errors
///
/// [`PatchRefusal`]: the edge certificate ends [`PatchRefusal::NoWidth`]
/// in the defect ending and [`PatchRefusal::Pieces`] as the lane's gap
/// (`certify.rs` `stated_refusal`).
pub(crate) fn patch(
    class: PatchClass,
    chart: &Surface<Interval>,
    (u, v): ((f64, f64), (f64, f64)),
    band: geom_core::Band,
) -> Result<Patch, PatchRefusal> {
    if class == PatchClass::Enclosed {
        return enclosed(chart, (u, v), band);
    }
    let ((ku, [u0, u1]), (kv, [v0, v1])) = (linear(u)?, linear(v)?);
    // On a ruling both `u` columns are the ruling itself.
    let column = |at: f64| match class {
        PatchClass::Plane | PatchClass::Enclosed => Interval::from_bounds(at, at),
        PatchClass::Ruling => Interval::from_bounds(u.0, u.1),
    };
    let mut coords: Vec<Vec<Interval>> = (0..3).map(|_| Vec::with_capacity(4)).collect();
    for cu in [column(u0), column(u1)] {
        for cv in [v0, v1] {
            let p = chart.eval(cu, Interval::from_bounds(cv, cv));
            for (lane, c) in coords.iter_mut().zip([p.x, p.y, p.z]) {
                lane.push(c);
            }
        }
    }
    Ok((ku, kv, vec![1.0; 4], coords))
}

/// The degree an enclosing patch reads each angle at.
pub(crate) const DEGREE: usize = 6;

/// The degree a window too wide for [`DEGREE`] in [`MAX_PIECES`] is read
/// at.
pub(crate) const WIDE_DEGREE: usize = 10;

/// The most pieces an enclosing patch cuts one angle's window into.
pub(crate) const MAX_PIECES: usize = 256;

/// One angle's window cut into pieces, with the cosine and sine of the
/// angle enclosed on each by its Bernstein controls: `cos[i]`, `sin[i]`
/// for the `i`-th control of the clamped spline on the knots
/// [`Angle::over`] returns beside it, a piece's end control shared with
/// the next (the hull of the two pieces' enclosures).
struct Angle {
    cos: Vec<Interval>,
    sin: Vec<Interval>,
}

/// `n!` as an `f64`, exact for the degrees read here.
fn factorial(n: usize) -> f64 {
    (1..=n).map(|k| k as f64).product()
}

/// `C(n, k)`, exact for the degrees read here.
fn binomial(n: usize, k: usize) -> f64 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

/// An enclosure of both `a` and `b`, `a + (b − a)·[0, 1]`, in interval
/// arithmetic, so a refused input's decoration rides through.
fn hull(a: Interval, b: Interval) -> Interval {
    a + (b - a) * Interval::from_bounds(0.0, 1.0)
}

/// `x` widened by `e` either side.
fn widen(x: Interval, e: f64) -> Interval {
    x + Interval::from_bounds(-e, e)
}

impl Angle {
    /// The angle over `[lo, hi]`, in pieces of half-width at most the one
    /// whose remainder, scaled by `arm` (the most the chart moves per unit
    /// of the cosine or sine), stays at `target`.
    fn over(
        (lo, hi): (f64, f64),
        arm: f64,
        target: f64,
    ) -> Result<(KnotVector, Self), PatchRefusal> {
        // `arm · d^(p+1)/(p+1)! ≤ target` for a piece of half-width `d`:
        // the least degree that holds the window in one piece, else
        // [`DEGREE`] in pieces. A low degree keeps the composite's low.
        // A radius that is no number reads no size: the chart's structure.
        if !arm.is_finite() {
            return Err(PatchRefusal::NoWidth);
        }
        let half_at = |p: usize| {
            (target * factorial(p + 1) / arm.max(f64::MIN_POSITIVE)).powf(1.0 / (p + 1) as f64)
        };
        let pieces_at = |p: usize| ((hi - lo) / (2.0 * half_at(p))).ceil();
        let p = (1..DEGREE)
            .find(|&p| hi - lo <= 2.0 * half_at(p))
            .unwrap_or(DEGREE);
        // A window too wide for [`DEGREE`]'s pieces reads at [`WIDE_DEGREE`].
        let p = if pieces_at(p) > MAX_PIECES as f64 {
            WIDE_DEGREE
        } else {
            p
        };
        let pieces = pieces_at(p);
        if !(pieces.is_finite() && pieces <= MAX_PIECES as f64) {
            return Err(PatchRefusal::Pieces);
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let pieces = (pieces as usize).max(1);
        let at = |k: usize| {
            if k == pieces {
                hi
            } else {
                lo + (hi - lo) * (k as f64 / pieces as f64)
            }
        };
        let mut knots = vec![lo; p + 1];
        for k in 1..pieces {
            knots.extend(vec![at(k); p]);
        }
        knots.extend(vec![hi; p + 1]);
        let knots = KnotVector::clamped(knots, p).map_err(|_| PatchRefusal::NoWidth)?;
        let count = pieces * p + 1;
        let (mut cos, mut sin) = (Vec::with_capacity(count), Vec::with_capacity(count));
        for k in 0..pieces {
            let (a, b) = (at(k), at(k + 1));
            // The piece's exact midpoint and half-width, enclosed.
            let (ia, ib) = (Interval::from_bounds(a, a), Interval::from_bounds(b, b));
            let d = (ib - ia) * Interval::from_bounds(0.5, 0.5);
            let m = ia + d;
            // The remainder over the piece, its half-width read up from the
            // two `f64` knots, whose difference rounds by at most half an
            // ulp.
            let e = ((b - a) * 0.5)
                .next_up()
                .next_up()
                .powi(i32::try_from(p + 1).unwrap_or(i32::MAX))
                / factorial(p + 1)
                * (1.0 + 1e-12);
            #[cfg(test)]
            let e = if super::mutants::patch_remainder_dropped() {
                0.0
            } else {
                e
            };
            let (sm, cm) = m.sin_cos();
            for (f0, f1, out) in [(cm, sm, &mut cos), (sm, -cm, &mut sin)] {
                // The derivatives of cos (sin) at m cycle cos, −sin, −cos,
                // sin (sin, cos, −sin, −cos): `f0`, `−f1`, `−f0`, `f1`.
                let deriv = |k: usize| match k % 4 {
                    0 => f0,
                    1 => -f1,
                    2 => -f0,
                    _ => f1,
                };
                for (i, c) in bernstein(p, d, deriv).into_iter().enumerate() {
                    let c = widen(c, e);
                    // A piece's first control is the last one's end: one
                    // control of the spline, holding both.
                    match out.last_mut() {
                        Some(held) if i == 0 => *held = hull(*held, c),
                        _ => out.push(c),
                    }
                }
            }
        }
        Ok((knots, Self { cos, sin }))
    }
}

/// The Bernstein controls on `[m − d, m + d]` of the degree-`p` Taylor
/// polynomial `Σ aₖ·xᵏ`, `aₖ = f⁽ᵏ⁾(m)·dᵏ/k!`, `x = 2s − 1 ∈ [−1, 1]`,
/// `deriv(k) = f⁽ᵏ⁾(m)`. The Bernstein coefficients of `xᵏ` are its
/// blossom at `p − i` arguments `−1` and `i` arguments `+1`,
/// `βₖᵢ = Σⱼ C(i, j)·C(p − i, k − j)·(−1)^(k−j) / C(p, k)`, each of
/// magnitude at most one, so `bᵢ = Σₖ aₖ·βₖᵢ` loses no more than the
/// terms' own rounding.
fn bernstein(p: usize, dd: Interval, deriv: impl Fn(usize) -> Interval) -> Vec<Interval> {
    let point = |x: f64| Interval::from_bounds(x, x);
    let mut a = Vec::with_capacity(p + 1);
    let mut power = point(1.0);
    for k in 0..=p {
        a.push(deriv(k) * power / point(factorial(k)));
        power = power * dd;
    }
    (0..=p)
        .map(|i| {
            (0..=p).fold(point(0.0), |acc, k| {
                // An integer of at most `C(p, k)·2^p`, exact in `f64`.
                let numerator: f64 = (k.saturating_sub(p - i)..=k.min(i))
                    .map(|j| {
                        let sign = if (k - j) % 2 == 0 { 1.0 } else { -1.0 };
                        sign * binomial(i, j) * binomial(p - i, k - j)
                    })
                    .sum();
                acc + a[k] * point(numerator) / point(binomial(p, k))
            })
        })
        .collect()
}

/// **A window widened by one either side**, as a clamped degree-1 knot
/// vector and its two ends: so a window of one value has width, and an
/// image's convex hull lies inside it. [`PatchRefusal::NoWidth`] where
/// the window is no number or widening cannot open it.
fn linear((lo, hi): (f64, f64)) -> Result<(KnotVector, [f64; 2]), PatchRefusal> {
    let (lo, hi) = (lo - 1.0, hi + 1.0);
    if !(lo.is_finite() && hi.is_finite() && lo < hi) {
        return Err(PatchRefusal::NoWidth);
    }
    let knots = KnotVector::clamped(vec![lo, lo, hi, hi], 1).map_err(|_| PatchRefusal::NoWidth)?;
    Ok((knots, [lo, hi]))
}

/// An angle's window, padded so a window of one value has width.
fn angular((lo, hi): (f64, f64)) -> Result<(f64, f64), PatchRefusal> {
    let pad = 2f64.powi(-10) * (1.0 + lo.abs().max(hi.abs()) * 2f64.powi(-40));
    let (lo, hi) = (lo - pad, hi + pad);
    if lo.is_finite() && hi.is_finite() && lo < hi {
        Ok((lo, hi))
    } else {
        Err(PatchRefusal::NoWidth)
    }
}

/// **The enclosing patch** of a curved chart over the window (the
/// module's last bullet).
fn enclosed(
    chart: &Surface<Interval>,
    (u, v): ((f64, f64), (f64, f64)),
    band: geom_core::Band,
) -> Result<Patch, PatchRefusal> {
    use geom_core::Bounds;
    // Each enclosed factor's remainder is held to a share of the zero.
    let target = band.zero() / 64.0;
    let mag = |x: Interval| x.lo().abs().max(x.hi().abs());
    let u = angular(u)?;
    let frame = |axis: geom_core::Vec3<Interval>, u_ref: geom_core::Vec3<Interval>| {
        (u_ref, axis.cross(u_ref))
    };
    // The radial direction's controls, `e₁·cos + e₂·sin`.
    let radial = |a: &Angle,
                  (e1, e2): (geom_core::Vec3<Interval>, geom_core::Vec3<Interval>)|
     -> Vec<geom_core::Vec3<Interval>> {
        a.cos
            .iter()
            .zip(&a.sin)
            .map(|(&c, &s)| e1 * c + e2 * s)
            .collect()
    };
    let (ku, kv, controls) = match *chart {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let (ka, a) = Angle::over(u, mag(radius), target)?;
            let (kv, vs) = linear(v)?;
            let rad = radial(&a, frame(axis, u_ref));
            let controls: Vec<Point3<Interval>> = rad
                .iter()
                .flat_map(|&r| vs.map(|w| origin + r * radius + axis * Interval::from_bounds(w, w)))
                .collect();
            (ka, kv, controls)
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let (kv, vs) = linear(v)?;
            let (sa, ca) = half_angle.sin_cos();
            let reach = vs[0].abs().max(vs[1].abs());
            let (ka, a) = Angle::over(u, reach * mag(sa), target)?;
            let rad = radial(&a, frame(axis, u_ref));
            let controls: Vec<Point3<Interval>> = rad
                .iter()
                .flat_map(|&r| {
                    vs.map(|w| {
                        let w = Interval::from_bounds(w, w);
                        apex + (axis * ca + r * sa) * w
                    })
                })
                .collect();
            (ka, kv, controls)
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            let r = mag(radius);
            let ((ka, a), (kb, b)) = (
                Angle::over(u, r, target)?,
                Angle::over(angular(v)?, r, target)?,
            );
            let rad = radial(&a, frame(axis, u_ref));
            let controls: Vec<Point3<Interval>> = rad
                .iter()
                .flat_map(|&r| {
                    b.cos
                        .iter()
                        .zip(&b.sin)
                        .map(move |(&c, &s)| center + (r * c + axis * s) * radius)
                        .collect::<Vec<_>>()
                })
                .collect();
            (ka, kb, controls)
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        }
        | Surface::Spindle {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let (big, small) = (mag(major_radius), mag(minor_radius));
            let ((ka, a), (kb, b)) = (
                Angle::over(u, big + small, target)?,
                Angle::over(angular(v)?, small, target)?,
            );
            let rad = radial(&a, frame(axis, u_ref));
            let controls: Vec<Point3<Interval>> = rad
                .iter()
                .flat_map(|&r| {
                    b.cos
                        .iter()
                        .zip(&b.sin)
                        .map(move |(&c, &s)| {
                            center
                                + r * (major_radius + minor_radius * c)
                                + axis * (minor_radius * s)
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
            (ka, kb, controls)
        }
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            unreachable!("an enclosing patch is a curved analytic chart's (`PatchClass::along`)")
        }
    };
    let count = controls.len();
    let coords = vec![
        controls.iter().map(|p| p.x).collect(),
        controls.iter().map(|p| p.y).collect(),
        controls.iter().map(|p| p.z).collect(),
    ];
    Ok((ku, kv, vec![1.0; count], coords))
}
