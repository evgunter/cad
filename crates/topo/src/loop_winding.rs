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
//! `|Δ|·max(sa, sb)` — the circle's exact arc length, the ellipse's
//! upper bound; an over-large `P` understates the width, i.e. escalates
//! rather than decides.
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

use geom_brep::EdgeCurve;
use geom_core::{Decide, Indeterminate, Margin, Real, Sign, Vec3};

use crate::body::Body;
use crate::entity::LoopKey;

/// Which conic carriers a wound loop rides — ordered, so a walk
/// accumulates the widest class it meets. Every class is answered; the
/// class decides only whether the conic correction runs (a `Lines`
/// cycle keeps the bare chord sum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LoopCarriers {
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

/// **The conic term of one traversed edge** — the one statement of it:
/// `(axis · sa·sb · (Δ − sin Δ), |Δ|·max(sa, sb))`, the vector area between the
/// arc and its chord (the cross-sum's `2A` convention, odd in the
/// signed span `Δ`) and the edge's boundary length (exact for a
/// circle, an upper bound for an ellipse: `|Δ|` times the LARGER
/// semi-axis, whichever field stores it — an ellipse stored with
/// `minor > major` reaches rest, and `|Δ|·major` would then be a lower
/// bound, overstating the metered width). `forward` is whether the
/// traversal runs with increasing carrier parameter — the edge's plus
/// half. `None` for every carrier that is not a conic: its term is its
/// chord's, which the caller owns. Shared by
/// [`Body::planar_loop_winding`] and the boolean join's ring-run lane
/// (`boolean::join::ring_run_ccw`).
pub(crate) fn conic_segment_term<T: Real>(
    curve: &EdgeCurve<T>,
    forward: bool,
) -> Option<(Vec3<T>, T)> {
    let (t0, t1) = curve.params();
    // `(axis, sa, sb, the larger semi-axis)`. The circle's lever is its
    // radius itself, not `radius.max(radius)`: the same value, but at a
    // symbolic scalar a `max` node is opaque where the radius is not.
    let (axis, sa, sb, reach) = match *curve.carrier() {
        geom::Curve3::Circle { axis, radius, .. } => (axis, radius, radius, radius),
        geom::Curve3::Ellipse {
            axis, major, minor, ..
        } => (axis, major, minor, major.max(minor)),
        geom::Curve3::Line { .. } | geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
            return None;
        }
    };
    let span = if forward { t1 - t0 } else { t0 - t1 };
    Some((axis * (sa * sb * (span - span.sin())), span.abs() * reach))
}

impl<T: Decide> Body<T> {
    /// The signed winding of cycle loop `l` around `normal` (module
    /// docs): `Positive` is counterclockwise about the normal
    /// (interior-left), `Negative` clockwise, `Zero` no signed area,
    /// `Err` an in-band margin.
    ///
    /// `Ok(None)` — no predicate is asked — for an empty loop (a
    /// lone-vertex ring bounds no area), for a cycle carrying a NURBS
    /// or spiric edge or a null-edge scaffold (the honest remainder).
    /// There is no narrower reach to ask for: the assigner and the
    /// checker answer on the one carrier set, which is what keeps them
    /// from disagreeing about which loops have a winding.
    pub(crate) fn planar_loop_winding(
        &self,
        l: LoopKey,
        normal: Vec3<T>,
        band: geom_core::Band,
    ) -> Result<Option<Result<Sign, Indeterminate>>, TornLoop> {
        let crate::entity::LoopBoundary::Cycle { first } =
            self.get_loop(l).ok_or(TornLoop)?.boundary
        else {
            return Ok(None);
        };
        let cycle = self.loop_cycle(first).ok_or(TornLoop)?;
        // One walk per half-edge: its start point, its certified curve
        // and whether it runs with the carrier's parameter.
        let mut walked = Vec::with_capacity(cycle.len());
        let mut carriers = LoopCarriers::Lines;
        for &he in &cycle {
            let hd = self.get_half_edge(he).ok_or(TornLoop)?;
            let edge = self.get_edge(hd.edge).ok_or(TornLoop)?;
            // A null-edge scaffold states no geometry: nothing here can
            // wind it.
            let Some(curve) = self.get_curve_geom(edge.curve).ok_or(TornLoop)?.certified() else {
                return Ok(None);
            };
            carriers = carriers.max(match curve.carrier() {
                geom::Curve3::Line { .. } => LoopCarriers::Lines,
                geom::Curve3::Circle { .. } => LoopCarriers::Circular,
                geom::Curve3::Ellipse { .. } => LoopCarriers::Elliptic,
                geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => return Ok(None),
            });
            let start = self
                .get_vertex(hd.start)
                .and_then(|vd| self.get_point(vd.point).copied())
                .ok_or(TornLoop)?;
            walked.push((start, curve, edge.he_plus == he));
        }
        let p0 = walked[0].0;
        let mut newell = Vec3::new(T::zero(), T::zero(), T::zero());
        // The F4 metering lever: the boundary's own length, accumulated
        // with the area — a chord per straight edge, an arc length per
        // conic.
        let mut perimeter = T::zero();
        for (i, &(p, curve, forward)) in walked.iter().enumerate() {
            let next = walked[(i + 1) % walked.len()].0;
            if i + 1 < walked.len() {
                newell = newell + (p - p0).cross(next - p0);
            }
            perimeter = perimeter
                + match conic_segment_term(curve, forward) {
                    Some((_, len)) => len,
                    None => (next - p).norm(),
                };
        }
        // The conic correction (module docs), added only when some
        // carrier is a conic: a line-only cycle keeps the chord sum.
        // Summed on its own and added once, so the chord sum is never
        // re-associated.
        if carriers != LoopCarriers::Lines {
            let mut bulge = Vec3::new(T::zero(), T::zero(), T::zero());
            for &(_, curve, forward) in &walked {
                if let Some((b, _)) = conic_segment_term(curve, forward) {
                    bulge = bulge + b;
                }
            }
            newell = newell + bulge;
        }
        Ok(Some(crate::validate::decide(
            "bool_ring_run_winding",
            Margin::over_lever(normal.dot(newell), perimeter),
            band,
        )))
    }
}
