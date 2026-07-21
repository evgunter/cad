//! The ON-vertex neighborhood: orbit → typed sector array, with
//! wide/reflex sectors handled by **convex subdivision** (store twice,
//! classify the duplicate by the interior bisector — the book's device,
//! adopted by derivation, not by copy).
//!
//! # Sector geometry under OUR conventions (derived, mirror-checked)
//!
//! The orbit ([`crate::Body::vertex_orbit`], step `next(mate(he))`)
//! visits the half-edges leaving the base vertex `v` **clockwise viewed
//! from outside**. For orbit-consecutive `he_i`, `he_{i+1}`, the sector
//! between them is the corner of the face traversed
//! `… mate(he_i) → he_{i+1} …` — so the **sector's face is
//! `face(loop(mate(he_i)))`** (equivalently `face(loop(he_{i+1}))`),
//! uniform for duplicate entries because they share `he_i`. By the
//! interior-left rule the sector's interior directions sweep **from
//! `dir(he_{i+1}) counterclockwise (around the face's outward normal
//! `n`) to `dir(he_i)`** — the corner between the loop's incoming
//! direction reversed and its outgoing direction.
//!
//! # Why convex subdivision (the wide/reflex fork, decided)
//!
//! Every face is planar (F5 gate), so a sector's interior is exactly
//! the positive cone of its two bounding directions **iff the sector's
//! angle is < 180°** — then the bounding-edge verdicts determine every
//! interior direction's side (a positive combination of two same-side
//! vectors stays on that side), and one entry suffices. At ≥ 180° the
//! cone argument fails (the interior can cross the plane while both
//! bounds sit on one side), so the sector is split at an interior
//! direction into two sub-sectors, each < 180° — restoring the
//! argument. The book's store-twice-with-bisector is exactly this
//! convex subdivision; the paper's alternative (complement-and-negate
//! at 180°, interior-vector sign for > 180°) answers point-membership,
//! not side-classification, and offers no cone argument — so the book's
//! device wins **by derivation**. Bonus (also derived, §14.6): the
//! duplicate entry is what makes dangling null edges fall out of the
//! generic run scan.
//!
//! The subdivision direction need not be the exact bisector — ANY
//! interior direction with both sub-angles < 180° is valid. We use:
//! definite reflex ⇒ `−normalize(a + b)` (the true bisector of the
//! reflex span); straight-band (near 180°, where `a + b` collapses) ⇒
//! `n × b` (90° into the interior; sub-angles ≈ 90° — valid throughout
//! the band). The wideness trilean therefore has **no escalation
//! cliff**: duplication is sound for every angle (subdividing a convex
//! sector would merely be redundant), so Zero and in-band verdicts
//! both take the duplicate path — the deliberate, documented posture
//! for this one predicate (the decisive side verdicts stay strict).

use geom_core::{Band, Decide, Sign, Vec3};
use slotmap::SecondaryMap;

use super::rules;
use super::{PlaneSide, SectorEntry, SectorEntryKind, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, VertexKey};
use crate::validate::decide;

/// Resolves the sector face for the sector CW-after `he` (module docs:
/// `face(loop(mate(he)))`) together with its outward plane normal.
pub(super) fn sector_face<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<(FaceKey, Vec3<T>), SplitReduceError> {
    let corrupt = SplitReduceError::CorruptOperand { vertex };
    let mate = body
        .mate(he)
        .ok_or(SplitReduceError::CorruptOperand { vertex })?;
    let half_edge = body.get_half_edge(mate).ok_or(corrupt)?;
    let r#loop = body
        .get_loop(half_edge.parent_loop)
        .ok_or(SplitReduceError::CorruptOperand { vertex })?;
    let face_key = r#loop.face;
    let face = body
        .get_face(face_key)
        .ok_or(SplitReduceError::CorruptOperand { vertex })?;
    match body.get_surface(face.surface) {
        Some(geom_surfaces::Surface::Plane { normal, .. }) => Ok((face_key, *normal)),
        // Unreachable after the F5 gate; kept typed.
        _ => Err(SplitReduceError::CurvedBooleanUnsupported { face: face_key }),
    }
}

/// The chord direction of orbit half-edge `he` out of the base vertex:
/// `p(final vertex) − p(base)` (all carriers are lines after the F5
/// gate, so the chord IS the edge direction at the vertex).
fn chord<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<(VertexKey, Vec3<T>), SplitReduceError> {
    let corrupt = || SplitReduceError::CorruptOperand { vertex };
    let final_vertex = body.half_edge_end(he).ok_or_else(corrupt)?;
    let p_base = *body
        .get_point(body.get_vertex(vertex).ok_or_else(corrupt)?.point)
        .ok_or_else(corrupt)?;
    let p_final = *body
        .get_point(body.get_vertex(final_vertex).ok_or_else(corrupt)?.point)
        .ok_or_else(corrupt)?;
    Ok((final_vertex, p_final - p_base))
}

/// Builds and fully classifies the typed sector array of ON vertex
/// `vertex`: orbit walk (initial classes from the per-vertex cache),
/// wide-sector duplicates (module docs), then rule (a) and rule (b)
/// ([`rules`]). Public so tests and the PR 3 joining step can inspect
/// classification independently of insertion.
///
/// # Errors
///
/// [`SplitReduceError`] — sliver escalations, the consecutive-ON
/// invariant, or a corrupt/unwalkable neighborhood.
pub fn classify_neighborhood<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    sides: &SecondaryMap<VertexKey, PlaneSide>,
    vertex: VertexKey,
    band: Band,
) -> Result<Vec<SectorEntry>, SplitReduceError> {
    let corrupt = || SplitReduceError::CorruptOperand { vertex };
    let anchor = body
        .get_vertex(vertex)
        .ok_or_else(corrupt)?
        .emanating
        .ok_or_else(corrupt)?;
    let orbit = body.vertex_orbit(anchor).ok_or_else(corrupt)?;

    let mut entries = Vec::with_capacity(orbit.len());
    for (i, &he) in orbit.iter().enumerate() {
        let (final_vertex, dir_a) = chord(body, vertex, he)?;
        let class = *sides.get(final_vertex).ok_or_else(corrupt)?;
        entries.push(SectorEntry {
            he,
            kind: SectorEntryKind::Edge,
            class,
        });

        // Wideness of the sector CW-after `he` (bounded by `he` and the
        // next orbit edge): margin (b̂ × â)·n · arm = sin(interior
        // angle) metered at the shorter bounding chord.
        let next_he = orbit[(i + 1) % orbit.len()];
        let (_, dir_b) = chord(body, vertex, next_he)?;
        let (face, n_face) = sector_face(body, vertex, he)?;
        let sliver = |diag| SplitReduceError::SliverSector { vertex, face, diag };
        let arm = dir_a.norm().min(dir_b.norm());
        match decide("split_sector_arm", arm, band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(sliver(geom_core::Indeterminate {
                    margin: geom_core::MarginDiag::Invalid,
                    band,
                    predicate: Some("split_sector_arm"),
                }));
            }
            Err(diag) => return Err(sliver(diag)),
        }
        let (unit_a, unit_b) = (dir_a.normalize(), dir_b.normalize());
        let reflex_margin = unit_b.cross(unit_a).dot(n_face) * arm;
        let wide = match decide("split_sector_reflex", reflex_margin, band) {
            Ok(Sign::Positive) => None, // convex: cone argument holds
            // Definite reflex, θ ∈ (π, 2π): −(â + b̂) is the true
            // bisector of the reflex span (the collapse â + b̂ → 0
            // happens only at θ → π, which lands in the Zero band
            // below, never here).
            Ok(Sign::Negative) => Some(-((unit_a + unit_b).normalize())),
            // sin θ coincident with zero (or in-band): θ is near π, 0,
            // or 2π — disambiguate by the cosine (for unit chords sin
            // and cos cannot both vanish, so this second margin is
            // definite whenever the first is not).
            Ok(Sign::Zero) | Err(_) => {
                let straight_margin = unit_a.dot(unit_b) * arm;
                match decide("split_sector_straight", straight_margin, band) {
                    // θ ≈ π (straight): 90° into the interior is a
                    // valid subdivision throughout the band.
                    Ok(Sign::Negative) => Some(n_face.cross(unit_b)),
                    // θ ≈ 0 or ≈ 2π. A one-edge orbit (strut vertex)
                    // is the legitimate full-circle sector; a spike
                    // corner between two distinct edges is
                    // ill-conditioned geometry — escalate, never
                    // guess an interior direction.
                    Ok(Sign::Positive | Sign::Zero) if he == next_he => Some(n_face.cross(unit_b)),
                    Ok(Sign::Positive | Sign::Zero) => {
                        return Err(sliver(geom_core::Indeterminate {
                            margin: geom_core::MarginDiag::Invalid,
                            band,
                            predicate: Some("split_sector_straight"),
                        }));
                    }
                    Err(diag) => return Err(sliver(diag)),
                }
            }
        };
        if let Some(bisector) = wide {
            let margin = bisector.dot(plane.normal) * arm;
            let class = match decide("split_bisector_side", margin, band) {
                Ok(Sign::Negative) => PlaneSide::Below,
                Ok(Sign::Positive) => PlaneSide::Above,
                Ok(Sign::Zero) => PlaneSide::On,
                Err(diag) => return Err(sliver(diag)),
            };
            entries.push(SectorEntry {
                he,
                kind: SectorEntryKind::WideBisector,
                class,
            });
        }
    }

    rules::apply_rule_a(body, plane, vertex, &mut entries, band)?;
    rules::apply_rule_b(vertex, &mut entries)?;
    Ok(entries)
}
