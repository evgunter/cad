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
//! The arm / wideness / subdivision-direction rungs this section
//! derives are implemented ONCE, in [`crate::sector_shape`], and
//! called from here and from the boolean lane's sector walk under each
//! lane's own K names (smell scan S5). The derivation stays here; the
//! code lives there.
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

use geom_core::{Band, Decide, Margin, Sign, Vec3};
use slotmap::SecondaryMap;

use super::rules;
use super::{PlaneSide, SectorEntry, SectorEntryKind, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, VertexKey};
use crate::sector_shape::{SPLIT_SECTOR_PREDICATES, SectorShape, sector_shape};
use crate::validate::decide;

/// Resolves the sector face for the sector CW-after `he` (module docs:
/// `face(loop(mate(he)))`) together with its outward normal **at the
/// base vertex** and whether the surface is a plane. For a `Plane` the
/// normal is the stored one (the M3 path); for a `Cylinder` (M5 PR 5)
/// it is the chart-outward radial at the vertex point — the local
/// normal every sector predicate meters through. Kinds the gate
/// refuses are typed here too (unreachable post-gate).
///
/// **Both arms are multiplied by the face's `sense_sign`** (S10):
/// each reads a chart normal and returns it as the face's OUTWARD
/// normal, and the chart is the only orientation encoding they have.
/// This is the splitting lane's chokepoint, the twin of the boolean's
/// `boolean::sectors::sector_face`: `rules::apply_rule_a`'s
/// `enters_material` call, the sector-shape rungs (since S5 part 1 they
/// are [`crate::sector_shape`], called below rather than written below),
/// and the departure trileans all consume this value and are
/// sense-invariant GIVEN it — they pair it with the STORED orbit order, which `revert`
/// reverses together with the sense bit, so a second `sense_sign`
/// factor at any of those sites would cancel this one.
pub(super) fn sector_face<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<(FaceKey, Vec3<T>, bool), SplitReduceError> {
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
    let sense = face.sense_sign::<T>();
    match body.get_surface(face.surface) {
        Some(geom_surfaces::Surface::Plane { normal, .. }) => Ok((face_key, *normal * sense, true)),
        Some(geom_surfaces::Surface::Cylinder { origin, axis, .. }) => {
            let p = *body
                .get_point(
                    body.get_vertex(vertex)
                        .ok_or(SplitReduceError::CorruptOperand { vertex })?
                        .point,
                )
                .ok_or(SplitReduceError::CorruptOperand { vertex })?;
            let w = p - *origin;
            let radial = w - *axis * w.dot(*axis);
            Ok((face_key, radial.normalize() * sense, false))
        }
        Some(s) => Err(SplitReduceError::CurvedBooleanUnsupported {
            face: face_key,
            kind: geom_brep::SurfaceKind::of(s),
        }),
        None => Err(SplitReduceError::CurvedBooleanUnsupported {
            face: face_key,
            kind: geom_brep::SurfaceKind::Nurbs,
        }),
    }
}

/// The outgoing direction of orbit half-edge `he` out of the base
/// vertex, scaled to the edge's honest extent (its `.norm()` is the
/// sector predicates' lever arm, its `.normalize()` the direction):
///
/// - **Line** carriers: the chord `p(final) − p(base)` — the M3 path,
///   bit-identical (the chord IS the direction, and its length the
///   extent).
/// - **Circle/Ellipse** carriers (M5 PR 5): the carrier's outgoing
///   unit tangent at the base vertex (`+deriv(t₀)` when `he` is the
///   plus half, `−deriv(t₁)` when the minus half — the `he_plus`
///   forward contract), scaled by [`geom_brep::edge_extent`] (the
///   certified point-set-diameter lower bound; the chord collapses on
///   near-closed arcs).
#[allow(clippy::type_complexity)] // (far vertex, scaled dir, conic jet) — one internal tuple
fn chord<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<(VertexKey, Vec3<T>, Option<(Vec3<T>, T)>), SplitReduceError> {
    let corrupt = || SplitReduceError::CorruptOperand { vertex };
    let final_vertex = body.half_edge_end(he).ok_or_else(corrupt)?;
    let p_base = *body
        .get_point(body.get_vertex(vertex).ok_or_else(corrupt)?.point)
        .ok_or_else(corrupt)?;
    let p_final = *body
        .get_point(body.get_vertex(final_vertex).ok_or_else(corrupt)?.point)
        .ok_or_else(corrupt)?;
    let he_data = body.get_half_edge(he).ok_or_else(corrupt)?;
    let edge = body.get_edge(he_data.edge).ok_or_else(corrupt)?;
    let curve = body
        .get_curve_geom(edge.curve)
        .and_then(crate::null::CurveGeom::certified)
        .ok_or_else(corrupt)?;
    match curve.carrier() {
        geom_curves::Curve3::Line { .. } | geom_curves::Curve3::Nurbs(_) => {
            Ok((final_vertex, p_final - p_base, None))
        }
        geom_curves::Curve3::Circle { .. } | geom_curves::Curve3::Ellipse { .. } => {
            let (t0, t1) = curve.params();
            // The base-endpoint jet: outgoing tangent, plus the raw
            // second derivative and squared speed for the C12.2
            // second-order descent (M5 PR 9). Walking the minus half
            // reverses the FIRST derivative only — position along the
            // walk is c(t₁ − τ), so d²/dτ² = +c″(t₁): no sign flip on
            // the curvature datum.
            let (tangent, deriv2, speed_sq) = if he == edge.he_plus {
                let d = curve.carrier().deriv(t0);
                (d, curve.carrier().deriv2(t0), d.norm_squared())
            } else {
                let d = curve.carrier().deriv(t1);
                (-d, curve.carrier().deriv2(t1), d.norm_squared())
            };
            let chord_len = p_final.distance(p_base);
            let extent = geom_brep::edge_extent(curve.carrier(), t0, t1, chord_len);
            Ok((
                final_vertex,
                tangent.normalize() * extent,
                Some((deriv2, speed_sq)),
            ))
        }
    }
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
        let (final_vertex, dir_a, conic_jet) = chord(body, vertex, he)?;
        // Entry class — "which side does this edge lead to":
        // - LINE edges: the far vertex's cached verdict, bit-identical
        //   to M3 (a straight edge's interior is the chord).
        // - CONIC edges (M1 fix, M5 PR 5 review): the OUTGOING TANGENT
        //   side through the named trilean `split_conic_departure`
        //   (margin `t̂·n̂` metered at the edge's honest extent — dir_a
        //   is exactly t̂·extent). The far vertex misleads on conics:
        //   a belly arc between ON vertices departs into its own side
        //   while both endpoints read On. Post-root-insertion the
        //   interior is sign-constant (every interior crossing was
        //   split out), so the departure IS the interior side; a Zero
        //   margin (in-plane departure — an in-plane arc or a graze
        //   contact) classifies On for rule (b)'s adjudication;
        //   in-band escalates typed.
        let class = if let Some((deriv2, speed_sq)) = conic_jet {
            let margin = Margin::of(dir_a.dot(plane.normal));
            match decide("split_conic_departure", margin, band) {
                Ok(Sign::Negative) => PlaneSide::Below,
                Ok(Sign::Positive) => PlaneSide::Above,
                // An in-plane departure ties at first order. The C12.2
                // descent (M5 PR 9): classify by which side the arc
                // CURVES to — the named second-order trilean, margin
                // the displacement the curvature induces at the edge's
                // honest extent (D4 ¶1). A second-order tie stays On
                // for rule (b)'s adjudication (never guess); in-band
                // escalates (F6 — an osculating pair is a sliver).
                Ok(Sign::Zero) => {
                    match geom_brep::enters_material_order2(
                        deriv2,
                        speed_sq,
                        plane.normal,
                        dir_a.norm(),
                        band,
                    ) {
                        Ok(geom_brep::EntersMaterial::Enters) => PlaneSide::Below,
                        Ok(geom_brep::EntersMaterial::Exits) => PlaneSide::Above,
                        Ok(geom_brep::EntersMaterial::Tangent) => PlaneSide::On,
                        Err(diag) => {
                            let (face, _, _) = sector_face(body, vertex, he)?;
                            return Err(SplitReduceError::SliverSector { vertex, face, diag });
                        }
                    }
                }
                Err(diag) => {
                    let (face, _, _) = sector_face(body, vertex, he)?;
                    return Err(SplitReduceError::SliverSector { vertex, face, diag });
                }
            }
        } else {
            *sides.get(final_vertex).ok_or_else(corrupt)?
        };
        entries.push(SectorEntry {
            he,
            kind: SectorEntryKind::Edge,
            class,
        });

        // Wideness of the sector CW-after `he` (bounded by `he` and the
        // next orbit edge), and the subdivision direction it implies:
        // the three rungs are [`crate::sector_shape`] — ONE
        // implementation, called from here and from the boolean lane's
        // sector walk under each lane's own K names (smell scan S5).
        // This is a call, not a copy. The derivation the
        // module docs above carry — why convex subdivision, why the
        // wideness trilean has no escalation cliff — is what that
        // shared body implements.
        let next_he = orbit[(i + 1) % orbit.len()];
        let (_, dir_b, _) = chord(body, vertex, next_he)?;
        let (face, n_face, _) = sector_face(body, vertex, he)?;
        let sliver = |diag| SplitReduceError::SliverSector { vertex, face, diag };
        let SectorShape {
            arm,
            bisector: wide,
            ..
        } = sector_shape(
            dir_a,
            dir_b,
            n_face,
            he == next_he,
            SPLIT_SECTOR_PREDICATES,
            band,
        )
        .map_err(sliver)?;
        if let Some(bisector) = wide {
            let margin = Margin::levered(bisector.dot(plane.normal), arm);
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
