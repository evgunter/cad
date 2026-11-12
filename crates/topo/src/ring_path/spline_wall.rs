//! **A ring lane's paths on a spline wall**: the cylinder's ruled paths
//! ([`Quadric::paths`]) on the wall's own chart, where a ruling and a
//! parallel are the chart's two iso curves.
//!
//! A path runs from one point's foot on the chart to the other's along
//! the iso curve through the first to the corner the two feet share,
//! then along the iso curve through the second: the two orders are the
//! wall's two paths. An iso curve is the de Boor collapse of the wall's
//! net ([`geom_brep::interior_iso_u`]), exact in ℝ, so a piece lies on
//! the wall and its meeting with a loop arc that lies on the wall within
//! the band ([`SplineArc`]) is a meeting in space, which the curve-pair
//! table reads ([`super::spline_crossings`]).
//!
//! The parity is the path's own to say where the curve bounds a disc of
//! the wall, the premise a torus's paths rest on (the module docs): the
//! curve lies on one face, and a loop of a face of a wall closed along
//! a parameter does not wind round it. A path stays in the chart's
//! rectangle and crosses no seam.

use std::sync::Arc;

use geom_core::{Band, Decide, Indeterminate, Lever, Margin, Point3, Real, Sign, Vec3};

use super::{Path, Piece, charge, resolution};
use crate::validate::decide;

/// A spline wall whose iso curves a path's pieces are cut from.
#[derive(Clone)]
pub(crate) struct SplineWall<T: Real> {
    chart: Arc<geom::NurbsSurface<T>>,
    /// The chart with its parameters swapped, whose rows are the chart's
    /// curves of constant `v`.
    swapped: Arc<geom::NurbsSurface<T>>,
    /// The chart's value channel, which a foot is sought on: a foot's
    /// parameters are structure, and the distance that admits them is
    /// read at `T` ([`Self::paths`]).
    values: Arc<geom::NurbsSurface<f64>>,
}

/// Which of the chart's parameters a piece holds fixed.
#[derive(Clone, Copy)]
enum Held {
    U,
    V,
}

impl<T: Decide> SplineWall<T> {
    /// The wall of `surface`; `None` for any other kind, for a chart
    /// with no net, and for a net whose iso curves have no shared
    /// weights ([`geom_brep::IsoRowError::WeightsNotSeparable`]).
    pub(crate) fn of(surface: &geom::Surface<T>) -> Option<Self> {
        let geom::Surface::Nurbs(chart) = surface else {
            return None;
        };
        if chart.is_placeholder() {
            return None;
        }
        let wall = Self {
            chart: chart.clone(),
            swapped: Arc::new(chart.transposed()),
            values: Arc::new(chart.map_scalar(T::value_channel)),
        };
        let (u, v) = (
            chart.knot_values_u().domain().0,
            chart.knot_values_v().domain().0,
        );
        (wall.iso(Held::U, u).is_some() && wall.iso(Held::V, v).is_some()).then_some(wall)
    }

    /// The iso curve holding `held` at `at`, its parameter the chart's
    /// other one.
    fn iso(&self, held: Held, at: T) -> Option<Arc<geom::NurbsCurve3<T>>> {
        let net = match held {
            Held::U => &self.chart,
            Held::V => &self.swapped,
        };
        geom_brep::interior_iso_u(net, at).ok().map(Arc::new)
    }

    /// The chart parameters of `p`'s foot on the wall; `None` where the
    /// projection settles on none, as from a point with no one value.
    fn foot(&self, p: Point3<T>) -> Option<(f64, f64)> {
        let p = Point3::new(
            p.x.value_channel(),
            p.y.value_channel(),
            p.z.value_channel(),
        );
        self.values.project(p).ok().map(|foot| (foot.u, foot.v))
    }

    /// The outward normal at `p`'s foot of a face of sense `sense`;
    /// `None` where `p` has no foot.
    pub(crate) fn outward(&self, p: Point3<T>, sense: bool) -> Option<Vec3<T>> {
        let (u, v) = self.foot(p)?;
        let jet = self.chart.ders(T::from_computed(u), T::from_computed(v));
        Some(geom_brep::OutwardNormal::from_chart(jet.du.cross(jet.dv).normalize(), sense).vec())
    }

    /// **The two paths from `from` to `to`**, in the order they are
    /// asked; none where an end has no foot on the wall, or its foot is
    /// not decided the end itself (**`split_ring_path_wall_foot`**: the
    /// distance between them).
    ///
    /// - along the `u` iso through `from`'s foot to `to`'s `v`, then
    ///   along that `v` iso, arriving along it;
    /// - along the `v` iso through `from`'s foot to `to`'s `u`, then
    ///   along that `u` iso, arriving along it.
    ///
    /// A path is asked only where the piece it arrives along is decided
    /// of some length (**`split_ring_path_wall_arrival`**: its chord,
    /// charged the [`resolution`] of the ends and the wall's net), so
    /// its arrival is a direction it travels; its first piece may have
    /// none. The pieces meet at the corner, each iso curve's own point
    /// at the other's held parameter.
    pub(crate) fn paths(
        &self,
        (from, to): (Point3<T>, Point3<T>),
        band: Band,
    ) -> Result<Vec<Path<T>>, Indeterminate> {
        let (Some(a), Some(b)) = (self.foot(from), self.foot(to)) else {
            return Ok(Vec::new());
        };
        let on_wall = |p: Point3<T>, (u, v): (f64, f64)| {
            let off = self.chart.eval(T::from_computed(u), T::from_computed(v)) - p;
            decide("split_ring_path_wall_foot", Margin::of(off.norm()), band)
                .map(|sign| sign == Sign::Zero)
        };
        if !(on_wall(from, a)? && on_wall(to, b)?) {
            return Ok(Vec::new());
        }
        let mut stored = vec![from, to];
        stored.extend_from_slice(self.chart.control());
        let slack = resolution(&stored);
        // The path whose first piece holds `first` at `from`'s foot and
        // whose second holds the other parameter at `to`'s.
        let path = |first: Held| -> Result<Option<Path<T>>, Indeterminate> {
            let (lead, arriving) = match first {
                Held::U => (
                    self.piece(Held::U, a.0, (a.1, b.1)),
                    self.piece(Held::V, b.1, (a.0, b.0)),
                ),
                Held::V => (
                    self.piece(Held::V, a.1, (a.0, b.0)),
                    self.piece(Held::U, b.0, (a.1, b.1)),
                ),
            };
            let (Some(lead), Some(arriving)) = (lead, arriving) else {
                return Ok(None);
            };
            let (chord, arrival) = arriving.travel();
            let arrives = decide(
                "split_ring_path_wall_arrival",
                Margin::of(charge(chord.length(), slack)),
                band,
            )?;
            Ok((arrives != Sign::Zero).then(|| Path {
                pieces: vec![Piece::Iso(lead), Piece::Iso(arriving)],
                arrival: arrival.normalize(),
            }))
        };
        Ok([path(Held::U)?, path(Held::V)?]
            .into_iter()
            .flatten()
            .collect())
    }

    /// The piece holding `held` at `at` from the other parameter's
    /// `from` to its `to`.
    fn piece(&self, held: Held, at: f64, (from, to): (f64, f64)) -> Option<IsoPiece<T>> {
        Some(IsoPiece {
            curve: self.iso(held, T::from_computed(at))?,
            span: (
                T::from_computed(from.min(to)),
                T::from_computed(from.max(to)),
            ),
            forward: from <= to,
        })
    }
}

/// A piece of a path on a spline wall: an iso curve of its chart over
/// the increasing `span`, travelled with the parameter (`forward`) or
/// against it.
#[derive(Clone)]
pub(crate) struct IsoPiece<T: Real> {
    pub(super) curve: Arc<geom::NurbsCurve3<T>>,
    pub(super) span: (T, T),
    pub(super) forward: bool,
}

impl<T: Decide> IsoPiece<T> {
    /// The piece's carrier.
    pub(super) fn carrier(&self) -> geom::Curve3<T> {
        geom::Curve3::Nurbs(self.curve.clone())
    }

    /// The piece's ends, in its order of travel.
    pub(super) fn ends(&self) -> (Point3<T>, Point3<T>) {
        let (lo, hi) = (self.curve.eval(self.span.0), self.curve.eval(self.span.1));
        if self.forward { (lo, hi) } else { (hi, lo) }
    }

    /// The piece's chord, the arm its readings are levered at (no longer
    /// than the piece), and its direction of travel where it stops.
    pub(super) fn travel(&self) -> (Lever<T>, Vec3<T>) {
        let (start, stop) = self.ends();
        let chord = geom_brep::ExtentBall::point(start).lever_from(stop);
        let carrier = self.carrier();
        let along = if self.forward {
            carrier.deriv(self.span.1)
        } else {
            -carrier.deriv(self.span.0)
        };
        (chord, along)
    }
}
