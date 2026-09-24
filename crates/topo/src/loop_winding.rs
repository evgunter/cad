//! The signed winding of a planar cycle loop around a normal — the
//! `bool_ring_run_winding` predicate over a loop's STORED traversal,
//! stated once for the two `topo` sites that ask it about a whole loop:
//! the merge's role assigner (`merge_faces`' `merged_outline_ring`),
//! which ASSIGNS outer/ring roles by it, and tier 3's check 6 planar
//! arm (`validate`), which FALSIFIES a stored role and sense by it.
//! One home is what keeps the assigner and the checker from answering
//! about different carrier sets or by different arithmetic.
//!
//! The margin is the plane's Newell functional — twice the enclosed
//! signed area — metered to a LENGTH by the loop's own perimeter:
//! `2A/P`, the region's mean width (audit F4). The derivation, and why
//! the predicate must state it identically at every site, is in
//! `boolean::join::ring_run_ccw`, whose run-level `run_term` is the
//! third site: it winds a chord RUN rather than a stored loop.
//!
//! # The carriers this answers about
//!
//! Line, Circle and Ellipse. A cycle carrying a NURBS or spiric edge
//! has no winding here — the honest remainder: a chord winding says
//! nothing about a fitted carrier's region and no closed form exists
//! for it.
//!
//! For the conic carriers the enclosed vector area decomposes EXACTLY,
//! per edge — a substitution, not an approximation:
//!
//! ```text
//!   2·A⃗ = Σ_edges       (p_prev − p₀) × (p − p₀)      [chord Newell]
//!        + Σ_conic-edges axis · sa·sb · (Δ − sin Δ)    [bulge]
//! ```
//!
//! A circular arc of radius `R` spanning signed angle `Δ` cuts off a
//! circular segment of area `R²(Δ − sin Δ)/2` between itself and its
//! chord; twice that is `R²(Δ − sin Δ)`, which is the cross-sum's own
//! `2A` convention, and it is ODD in `Δ` — so it carries the traversal
//! sign the winding question is about. The ellipse is the circle's
//! affine image, which scales every area by `major·minor/R²`, giving
//! `sa·sb`. The chord term is untouched for every edge, so the bulge is
//! a CORRECTION on a chord polygon and a mixed Line+Circle cycle needs
//! no case split beyond the per-edge carrier match.
//!
//! The perimeter lever moves with the area: a conic edge contributes
//! `|Δ|·sa` — the circle's exact arc length, the ellipse's upper bound;
//! an over-large `P` understates the width, i.e. escalates rather than
//! decides.
//!
//! A LINE-ONLY cycle is decided with exactly the chord arithmetic and
//! accumulation order: the correction block is structurally skipped,
//! not zero-added into a reordered sum.
//!
//! A cycle whose perimeter is exactly zero — every vertex coincident —
//! divides `0/0`, poisons, and escalates: a loop with no extent has no
//! winding to report.
//!
//! `normal` must be the face's OUTWARD normal: the caller folds the
//! sense into the chart normal exactly once, through
//! [`crate::face_normal`]'s door, and the sum here is left alone. It is
//! built from the loop's STORED cycle order, which `revert` reverses in
//! the same breath as it flips the sense bit, so it changes sign on its
//! own — threading the sense onto both factors would cancel.

use geom_core::{Decide, Indeterminate, Margin, Sign, Vec3};

use crate::body::Body;
use crate::entity::{HalfEdgeKey, LoopKey};

/// Which conic carriers a wound loop rides — the carrier set a caller
/// may choose to answer on. Ordered: a loop's class is the widest of
/// its edges'.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LoopCarriers {
    /// Every edge a `Line`: the chord polygon IS the region.
    Lines,
    /// Lines and at least one `Circle`, no `Ellipse`: the bulge is the
    /// circular segment and the arc-length lever is exact.
    Circular,
    /// At least one `Ellipse`: the bulge is exact, the lever an upper
    /// bound.
    Elliptic,
}

/// A lookup on the way from the loop to a point or a carrier failed —
/// the body is torn under the call (unreachable on tier-1 input).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TornLoop;

impl<T: Decide> Body<T> {
    /// The signed winding of cycle loop `l` around `normal` (module
    /// docs): `Positive` is counterclockwise about the normal
    /// (interior-left), `Negative` clockwise, `Zero` no signed area,
    /// `Err` an in-band margin.
    ///
    /// `Ok(None)` — no predicate is asked — for an empty loop (a
    /// lone-vertex ring bounds no area), for a cycle carrying a NURBS
    /// or spiric edge (the honest remainder), and for a cycle whose
    /// widest carrier lies beyond `reach`, the carrier set the caller
    /// answers on.
    pub(crate) fn planar_loop_winding(
        &self,
        l: LoopKey,
        normal: Vec3<T>,
        band: geom_core::Band,
        reach: LoopCarriers,
    ) -> Result<Option<Result<Sign, Indeterminate>>, TornLoop> {
        let crate::entity::LoopBoundary::Cycle { first } = self.get_loop(l).ok_or(TornLoop)?.boundary
        else {
            return Ok(None);
        };
        let cycle = self.loop_cycle(first).ok_or(TornLoop)?;
        let carrier_of = |he: HalfEdgeKey| -> Result<Option<LoopCarriers>, TornLoop> {
            let curve = self
                .get_half_edge(he)
                .and_then(|hd| self.get_edge(hd.edge))
                .and_then(|e| self.get_curve_geom(e.curve))
                .ok_or(TornLoop)?;
            // An uncertified carrier is a curve with no stated
            // geometry: nothing here can wind it.
            let Some(curve) = crate::null::CurveGeom::certified(curve) else {
                return Ok(None);
            };
            Ok(match curve.carrier() {
                geom::Curve3::Line { .. } => Some(LoopCarriers::Lines),
                geom::Curve3::Circle { .. } => Some(LoopCarriers::Circular),
                geom::Curve3::Ellipse { .. } => Some(LoopCarriers::Elliptic),
                geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => None,
            })
        };
        let mut carriers = LoopCarriers::Lines;
        for &he in &cycle {
            match carrier_of(he)? {
                Some(c) => carriers = carriers.max(c),
                None => return Ok(None),
            }
        }
        if carriers > reach {
            return Ok(None);
        }
        let point_of = |v| {
            self.get_vertex(v)
                .and_then(|vd| self.get_point(vd.point).copied())
                .ok_or(TornLoop)
        };
        let start_of = |he| point_of(self.get_half_edge(he).ok_or(TornLoop)?.start);
        let p0 = start_of(cycle[0])?;
        let mut newell = Vec3::new(T::zero(), T::zero(), T::zero());
        // The F4 metering lever: this cycle's perimeter, accumulated
        // with the area (chords here; the conic arm re-meters below).
        let mut perimeter = T::zero();
        let mut prev = p0;
        for &he in &cycle[1..] {
            let p = start_of(he)?;
            newell = newell + (prev - p0).cross(p - p0);
            perimeter = perimeter + (p - prev).norm();
            prev = p;
        }
        perimeter = perimeter + (p0 - prev).norm();
        // The conic correction (module docs): one curve lookup yields
        // both the vector area between an arc and its chord and the
        // half-edge's own boundary length. It runs only when some
        // carrier is a conic, so a line-only cycle keeps the chord
        // arithmetic and its accumulation order.
        if carriers != LoopCarriers::Lines {
            let zero = Vec3::new(T::zero(), T::zero(), T::zero());
            let arc_term = |he| -> Result<(Vec3<T>, T), TornLoop> {
                let edge = self
                    .get_half_edge(he)
                    .and_then(|hd| self.get_edge(hd.edge))
                    .ok_or(TornLoop)?;
                let curve = self
                    .get_curve_geom(edge.curve)
                    .and_then(crate::null::CurveGeom::certified)
                    .ok_or(TornLoop)?;
                let (t0, t1) = curve.params();
                let (axis, sa, sb) = match *curve.carrier() {
                    geom::Curve3::Circle { axis, radius, .. } => (axis, radius, radius),
                    geom::Curve3::Ellipse {
                        axis, major, minor, ..
                    } => (axis, major, minor),
                    geom::Curve3::Line { .. }
                    | geom::Curve3::Spiric { .. }
                    | geom::Curve3::Nurbs(_) => {
                        let end = point_of(self.half_edge_end(he).ok_or(TornLoop)?)?;
                        return Ok((zero, (end - start_of(he)?).norm()));
                    }
                };
                // Signed by traversal: the half-edge runs with
                // increasing carrier parameter iff it is the plus
                // half. `|Δ|·sa` is the circle's exact arc length and
                // the ellipse's upper bound.
                let span = if edge.he_plus == he { t1 - t0 } else { t0 - t1 };
                Ok((axis * (sa * sb * (span - span.sin())), span.abs() * sa))
            };
            let mut bulge = zero;
            let mut metered = T::zero();
            for &he in &cycle {
                let (b, len) = arc_term(he)?;
                bulge = bulge + b;
                metered = metered + len;
            }
            newell = newell + bulge;
            perimeter = metered;
        }
        Ok(Some(crate::validate::decide(
            "bool_ring_run_winding",
            Margin::over_lever(normal.dot(newell), perimeter),
            band,
        )))
    }
}
