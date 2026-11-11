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

use std::sync::Arc;

use geom::{NurbsCurve2, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Interval, Point2, Real, Vec2};

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
    /// `None` where no bilinear patch agrees with the chart along it: a
    /// spline chart (its own door's), a closed-form image, and an image
    /// on a curved chart that leaves its ruling.
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
            Surface::Cylinder { .. } | Surface::Cone { .. } => on_ruling().then_some(Self::Ruling),
            Surface::Sphere { .. }
            | Surface::Torus { .. }
            | Surface::Spindle { .. }
            | Surface::Nurbs(_)
            | Surface::Approx(_) => None,
        }
    }
}

/// A spline chart operand of limb 2's composite, owned: its two knot
/// vectors, weights and certified control coordinates
/// (`ssi::certify::ChartOperand`).
pub(crate) type Patch = (KnotVector, KnotVector, Vec<f64>, Vec<Vec<Interval>>);

/// **The patch** of `class` on `chart` (the chart lifted to intervals),
/// over the window `u`, `v` that holds the image's control coordinates:
/// widened by one either side, so a window of one value is a patch, and
/// the image's convex hull lies inside it. On a ruling the `u` window is
/// read for its value `u` (one value, a point or an enclosure of it).
///
/// `None` for a window that is no number.
pub(crate) fn patch(
    class: PatchClass,
    chart: &Surface<Interval>,
    (u, v): ((f64, f64), (f64, f64)),
) -> Option<Patch> {
    let widened = |(lo, hi): (f64, f64)| {
        let (lo, hi) = (lo - 1.0, hi + 1.0);
        (lo.is_finite() && hi.is_finite()).then_some((lo, hi))
    };
    let ((u0, u1), (v0, v1)) = (widened(u)?, widened(v)?);
    let knots = |lo, hi| KnotVector::clamped(vec![lo, lo, hi, hi], 1).ok();
    let (ku, kv) = (knots(u0, u1)?, knots(v0, v1)?);
    // On a ruling both `u` columns are the ruling itself.
    let column = |at: f64| match class {
        PatchClass::Plane => Interval::from_bounds(at, at),
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
    Some((ku, kv, vec![1.0; 4], coords))
}
