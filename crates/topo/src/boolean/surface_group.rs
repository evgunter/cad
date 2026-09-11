//! **One home for the surface-group scan**: is the set of faces sharing
//! one curved surface key CLOSED against the rest of the body, so that
//! their union covers the whole chart (or the whole of one chart
//! coordinate) and a trim has nothing left to do?
//!
//! Three containment arms ask that question and they ask the SAME
//! question, differing only in which boundary edges are allowed to go
//! unshared:
//!
//! - a **sphere** group is closed when every boundary edge is shared
//!   with another member — the union then has no boundary against any
//!   other surface, and a closed subsurface without boundary of a
//!   connected compact surface IS that surface;
//! - a **torus** group is closed by the same rule and the same argument
//!   (the torus is compact and connected too);
//! - a **cone** group need only WRAP THE AZIMUTH, so an edge that
//!   bounds the face in the SLANT coordinate — a rim, which the slant
//!   window already carries — is exempt from the sharing rule. On a
//!   cone the rims are exactly the circular carriers: a circle on a
//!   cone is centred on the axis, since an oblique plane cuts an
//!   ellipse.
//!
//! # Structure, not a margin
//!
//! The scan is exact-`f64` structure selection (C6): arena keys, mate
//! adjacency and curve VARIANTS only. It has no in-band twin and does
//! not move with ε. Whatever a caller must decide against the band — the
//! cone's agreement between members' slant windows is the standing one —
//! stays in the caller, where its predicate is named alongside the rest
//! of that kind's trim.
//!
//! Rings take a face out of every class: a ringed face is a trimmed one,
//! and a ring is a boundary the sharing scan cannot see.

use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary};
use geom_core::Decide;

/// Which boundary edges may go unshared without breaking closure — the
/// one per-kind difference between the scans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RimExemption {
    /// **Nothing is exempt**: every boundary edge must be shared, so the
    /// group closes over the WHOLE chart. The sphere's and the torus's
    /// rule.
    None,
    /// **Circular carriers are exempt**: on a cone a circle is centred
    /// on the axis and is therefore a slant iso-line — it bounds the
    /// face in the coordinate the slant window carries, never in
    /// azimuth. Every other carrier crosses the slant and must be
    /// shared, so what this scan certifies is that the group wraps the
    /// AZIMUTH.
    Circles,
}

/// A closed group: its members in face-arena order, and the
/// REPRESENTATIVE the arms act for.
///
/// Acting for one member is not an optimization. The group's members
/// share the trim their closure certifies, so a per-face arm would fold
/// the same root once per member and tie the closest-hit rule into a
/// permanent graze.
pub(super) struct SurfaceGroup {
    /// Every face carrying this surface key, arena order.
    pub(super) members: Vec<FaceKey>,
    /// The lowest face key in arena order — the member the arms act for.
    pub(super) representative: FaceKey,
}

/// The scan's answer.
///
/// * `Ok(Some(group))` — closed under `exempt`.
/// * `Ok(None)` — definitely NOT closed: a ring, an unwalkable
///   boundary, or an edge shared with another surface. The caller falls
///   through to its per-face class.
/// * `Err(face)` — an arena claim about a BROKEN body, naming the face
///   the walk lost an entity on. A caller that reports corruption raises
///   it; one whose class simply does not apply to a body it cannot walk
///   maps it to `None`.
///
/// # Errors
///
/// The face key whose walk could not be completed.
pub(super) fn surface_group<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    exempt: RimExemption,
) -> Result<Option<SurfaceGroup>, FaceKey> {
    let Some(surface) = body.get_face(face).map(|f| f.surface) else {
        return Err(face);
    };
    let members: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| f.surface == surface)
        .map(|(k, _)| k)
        .collect();
    for &member in &members {
        let Some(f) = body.get_face(member) else {
            return Err(member);
        };
        if !f.rings.is_empty() {
            return Ok(None);
        }
        let Some(LoopBoundary::Cycle { first }) = body.get_loop(f.outer).map(|l| l.boundary) else {
            return Ok(None);
        };
        let Some(cycle) = body.loop_cycle(first) else {
            return Err(member);
        };
        for he in cycle {
            if exempt == RimExemption::Circles {
                let Some(curve) = body
                    .get_half_edge(he)
                    .and_then(|h| body.get_edge(h.edge))
                    .map(|e| e.curve)
                else {
                    return Err(member);
                };
                if let Some(crate::null::CurveGeom::Certified(c)) = body.get_curve_geom(curve)
                    && matches!(c.carrier(), geom::Curve3::Circle { .. })
                {
                    continue;
                }
            }
            let Some(neighbour) = body
                .mate(he)
                .and_then(|m| body.get_half_edge(m))
                .and_then(|h| body.get_loop(h.parent_loop))
                .map(|l| l.face)
            else {
                return Err(member);
            };
            if !members.contains(&neighbour) {
                return Ok(None);
            }
        }
    }
    let Some(&representative) = members.first() else {
        return Err(face);
    };
    Ok(Some(SurfaceGroup {
        members,
        representative,
    }))
}
