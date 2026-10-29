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
//! A face with a tangent plane at the vertex is planar there to first
//! order, so a sector's interior is exactly
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
//! called from here and from the boolean lane's sector walk under one
//! pooled set of K names. The derivation stays here; the code lives
//! there.
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
//!
//! # At a cone's apex
//!
//! A cone face has no tangent plane at its apex
//! ([`crate::sector_face::VertexFace::Apex`]). Its sector's interior
//! directions are the rulings between its bounds, round the axis, not
//! positive combinations of them, so a plane through the apex can cross
//! such a sector twice at any width and the convex argument above does
//! not hold. The plane's side along the rulings is monotone between its
//! two extremal rulings, so the sector is subdivided at each extremal
//! ruling strictly inside it instead ([`crate::sector_shape::apex_shape`]):
//! between consecutive entries the side then changes at most once, which
//! is what the run scan reads. Those entries are the walk's
//! `WideBisector` duplicates, up to two in one sector (a window over a
//! half-turn holds both extrema), where a tangent plane's wide sector
//! holds one and a slit tip's full turn three, at its quarters
//! ([`crate::sector_shape::FullTurn`]). Every reader of the array takes
//! several in a sector (rule (b)'s duplicate rows); the plane's side
//! changes sign at most twice in a sector (a plane through the vertex
//! meets a full turn along one line, two rays), so at most one stretch of
//! duplicates alone lies apart from its bounds (`insert`'s dangling
//! strut).

use geom_core::{Band, Decide, Indeterminate, Margin, Sign, Vec3};
use slotmap::SecondaryMap;

use super::rules;
use super::{PlaneSide, SectorEntry, SectorEntryKind, SectorReading, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::live::{Proven, linked, proven};
use crate::sector_face::{SectorCarrier, SectorFaceError, VertexFace};
use crate::sector_shape::{
    ApexShape, CornerBounds, CornerShape, SectorFault, SectorShape, SlitTips, TurnOrder,
    apex_shape, corner_shape,
};
use crate::validate::decide;

/// Resolves the sector face for the sector CW-after `he` (module docs:
/// `face(loop(mate(he)))`) together with its outward normal **at the
/// base vertex** and whether the surface is a plane.
///
/// The walk and the normals are [`crate::sector_face`] — ONE
/// implementation, called from here and from the boolean lane's sector
/// walk. What stays here is this lane's adaptation of
/// it, and only that: the split lane's error type, the planar flag
/// `rules::apply_rule_a` branches on, and the `Sphere`, `Torus` and
/// `Spline` refusals.
///
/// **`Sphere` refuses here rather than there**, as `Torus` and `Spline`
/// do. The shared walk has a wired arm for each — the boolean lane
/// executes it — and this lane does not, so the refusal is typed here
/// (C12.1, per arm) rather than left to the gate.
///
/// It is worth being exact about WHY the arm is unreachable, because
/// the obvious answer is wrong: it is not that the F5 carrier gate
/// ([`super::classify`]) runs first. [`super::classify_neighborhood`]
/// is public, deliberately, so tests and the joining step can inspect
/// classification on their own, and on that path no gate runs at all.
/// What makes a sphere-carried sector unreachable through the public
/// door is that the operand cannot have one: `split_reduce` gates, and
/// the only other way in hands the caller a body it built through the
/// Euler ops, where the surface a face carries comes from
/// `set_face_surface`. A sphere-carried face therefore reaches this
/// arm only from INSIDE the crate — which is exactly where its test
/// row lives.
///
/// The normal arrives as an [`OutwardNormal`](geom_brep::enters::OutwardNormal) with the face's `sense`
/// folded in (S10), minted at the shared chokepoint. Everything
/// downstream of it here — `rules::apply_rule_a`'s `enters_material`
/// call, the sector-shape rungs, the departure trileans — is
/// sense-invariant GIVEN that value and must not multiply again: those
/// sites pair it with the STORED orbit order, which `revert` reverses
/// together with the sense bit, so a second sense fold would
/// cancel this one.
pub(super) fn sector_face<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
    band: Band,
) -> Result<(FaceKey, VertexFace<T>, bool), SplitReduceError> {
    let resolved = crate::sector_face::resolve(body, vertex, he, band).map_err(|e| match e {
        SectorFaceError::Unsupported { face, kind }
        | SectorFaceError::Crease { face, kind }
        | SectorFaceError::Unplaced { face, kind } => {
            SplitReduceError::CurvedBooleanUnsupported { face, kind }
        }
        SectorFaceError::ApexUndecided { face, diag } => {
            SplitReduceError::ApexUndecided { vertex, face, diag }
        }
    })?;
    match resolved.carrier {
        SectorCarrier::Plane => Ok((resolved.face, resolved.at_vertex, true)),
        SectorCarrier::Cylinder | SectorCarrier::Cone => {
            Ok((resolved.face, resolved.at_vertex, false))
        }
        SectorCarrier::Sphere => Err(SplitReduceError::CurvedBooleanUnsupported {
            face: resolved.face,
            kind: geom::SurfaceKind::Sphere,
        }),
        SectorCarrier::Torus => Err(SplitReduceError::CurvedBooleanUnsupported {
            face: resolved.face,
            kind: geom::SurfaceKind::Torus,
        }),
        SectorCarrier::Spline(kind) => Err(SplitReduceError::CurvedBooleanUnsupported {
            face: resolved.face,
            kind,
        }),
    }
}

/// The outgoing direction of orbit half-edge `he` out of the base
/// vertex, scaled to the edge's extent from below (its `.norm()` is the
/// sector predicates' lever arm, its `.normalize()` the direction):
///
/// - **Line** carriers: the chord `p(final) − p(base)` — the M3 path,
///   bit-identical (the chord IS the direction, and its length the
///   extent).
/// - **Curved** carriers (a conic, a spiric or a spline): the departure
///   [`geom_brep::ConicDeparture`] reads, the outgoing unit tangent
///   scaled by [`geom_brep::edge_extent_inf`] (the chord collapses on
///   near-closed arcs, and is not a spline's direction), with the
///   edge's [`geom_brep::ConicBend`] there.
#[allow(clippy::type_complexity)] // (far vertex, scaled dir, conic bend) — one internal tuple
pub(super) fn chord<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<(VertexKey, Vec3<T>, Option<geom_brep::ConicBend<T>>), SplitReduceError> {
    let final_vertex = body.proven_half_edge_end(he);
    let p_base = body.resolve_vertex_point(vertex, Proven);
    let p_final = body.resolve_vertex_point(final_vertex, Proven);
    let edge_key = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
    let edge = linked(
        &body.edges,
        edge_key,
        EntityId::Edge,
        EntityId::HalfEdge(he),
        "edge",
    );
    let curve = body.edge_curve_linked(edge_key, edge).certified().ok_or(
        SplitReduceError::NullEdgeAtVertex {
            vertex,
            edge: edge_key,
        },
    )?;
    match curve.carrier() {
        geom::Curve3::Line { .. } => Ok((final_vertex, p_final - p_base, None)),
        geom::Curve3::Circle { .. }
        | geom::Curve3::Ellipse { .. }
        | geom::Curve3::Spiric { .. }
        | geom::Curve3::Nurbs(_) => {
            let departure =
                geom_brep::ConicDeparture::of(curve, he == edge.he_plus, p_final.distance(p_base));
            Ok((final_vertex, departure.dir, Some(departure.bend)))
        }
    }
}

/// Why [`entry_side`] read no side: the walk's own refusal, the far
/// vertex's (`far`'s), or an escalated reading of a conic's departure,
/// which the caller names its sector for.
#[derive(Debug)]
pub(crate) enum EntryFault<E> {
    /// [`chord`]'s refusal.
    Walk(SplitReduceError),
    /// The far vertex's side did not read.
    Far(E),
    /// A conic's departure or bend lies in the band.
    Undecided(Indeterminate),
}

/// **Which side of the plane orbit half-edge `he` leaves `vertex`
/// into**: the walk's entry class before rules (a) and (b), the one
/// reading the walk below and [`crate::vertex_section`] both take.
/// Returns the edge's ray ([`chord`]) with the side.
///
/// - LINE edges: the far vertex's side (`far`); a straight edge's
///   interior is the chord.
/// - CONIC edges: the OUTGOING TANGENT side through the named trilean
///   `split_conic_departure` (margin `t̂·n̂` metered at the edge's
///   extent, the ray being exactly `t̂·extent`). The far vertex
///   misleads on conics: a belly arc between ON vertices departs into
///   its own side while both endpoints read On. Post-root-insertion the
///   interior is sign-constant (every interior crossing was split out),
///   so the departure IS the interior side. An in-plane departure ties
///   at first order, and the arc then reads by which side it BENDS to
///   against `reference`, certified as the boolean's vertex-on-face
///   pass reads it ([`geom_brep::conic_bend_side`]). A whole edge
///   within the band of the plane reads On; a bend the band does not
///   separate from the first-order residue escalates (F6).
pub(crate) fn entry_side<T: Decide, E>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
    (normal, reference): (Vec3<T>, geom_brep::ReferenceNormal<T>),
    far: impl FnOnce(VertexKey) -> Result<PlaneSide, E>,
    band: Band,
) -> Result<(Vec3<T>, PlaneSide), EntryFault<E>> {
    let (final_vertex, dir, conic_bend) = chord(body, vertex, he).map_err(EntryFault::Walk)?;
    let Some(bend) = conic_bend else {
        return far(final_vertex)
            .map(|side| (dir, side))
            .map_err(EntryFault::Far);
    };
    let side = match decide("split_conic_departure", Margin::of(dir.dot(normal)), band) {
        Ok(Sign::Negative) => PlaneSide::Below,
        Ok(Sign::Positive) => PlaneSide::Above,
        Ok(Sign::Zero) => {
            match geom_brep::conic_bend_side(dir, dir.norm(), bend, reference, band) {
                Ok(geom_brep::EntersMaterial::Enters) => PlaneSide::Below,
                Ok(geom_brep::EntersMaterial::Exits) => PlaneSide::Above,
                Ok(geom_brep::EntersMaterial::Tangent) => PlaneSide::On,
                Err(diag) => return Err(EntryFault::Undecided(diag)),
            }
        }
        Err(diag) => return Err(EntryFault::Undecided(diag)),
    };
    Ok((dir, side))
}

/// **A sector's subdivision entries**: the sector after an orbit
/// half-edge, bounded by the rays `(dir_a, dir_b)` in `face`, which is
/// `at_vertex` there, its bounds as `bounds` says. With a tangent plane,
/// [`corner_shape`] reads it: a wide sector gets one entry at its
/// subdivision direction, and a slit tip's full turn three, at its
/// quarters. At a cone's apex, [`apex_shape`] reads it against the
/// plane, and it gets one at each extremal ruling inside it (module
/// docs). Each entry is its direction, the sector's arm, and the side of
/// the plane along `normal` that direction points to, levered at the arm
/// (`split_bisector_side`), in orbit order; none for a convex sector or
/// an apex sector the plane's side is monotone across. The walk's
/// duplicate entries, and [`crate::vertex_section`]'s reading of the
/// same sector. Faults and escalations are the split's refusals at
/// `(vertex, face)`.
pub(crate) fn wide_entries<T: Decide>(
    (vertex, face): (VertexKey, FaceKey),
    (dir_a, dir_b): (Vec3<T>, Vec3<T>),
    at_vertex: VertexFace<T>,
    bounds: CornerBounds,
    normal: Vec3<T>,
    band: Band,
) -> Result<Vec<(Vec3<T>, T, PlaneSide)>, SplitReduceError> {
    let sliver = |diag| SplitReduceError::SliverSector {
        vertex,
        face,
        reading: SectorReading::Escalated(diag),
    };
    let fault = |fault| match fault {
        SectorFault::NonFiniteChord => SplitReduceError::NonFiniteSectorChord { vertex, face },
        SectorFault::UnderflowedChord => SplitReduceError::UnderflowedSectorChord { vertex, face },
        SectorFault::Rung { diag, .. } => sliver(diag),
    };
    let (arm, directions) = match at_vertex {
        VertexFace::Tangent(n_face) | VertexFace::NearAxis(n_face) => {
            match corner_shape(dir_a, dir_b, n_face, bounds, band).map_err(fault)? {
                CornerShape::Sector(SectorShape { arm, bisector, .. }) => {
                    (arm, bisector.into_iter().collect())
                }
                CornerShape::FullTurn(turn) => (turn.arm, turn.quarters(TurnOrder::FromOwn).into()),
            }
        }
        VertexFace::Apex(apex) => {
            let ApexShape { arm, rulings } = apex_shape(
                dir_a,
                dir_b,
                &apex,
                bounds == CornerBounds::OneEdge,
                normal,
                band,
            )
            .map_err(fault)?;
            (arm, rulings)
        }
    };
    directions
        .into_iter()
        .map(|d: Vec3<T>| {
            let side = match decide(
                "split_bisector_side",
                Margin::levered(d.dot(normal), arm),
                band,
            )
            .map_err(sliver)?
            {
                Sign::Negative => PlaneSide::Below,
                Sign::Positive => PlaneSide::Above,
                Sign::Zero => PlaneSide::On,
            };
            Ok((d, arm, side))
        })
        .collect()
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
/// invariant, a `vertex` that does not resolve or is a lone vertex, a
/// far vertex `sides` holds no verdict for, or a null edge at `vertex`
/// ([`SplitReduceError::NullEdgeAtVertex`]).
pub fn classify_neighborhood<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    sides: &SecondaryMap<VertexKey, PlaneSide>,
    vertex: VertexKey,
    band: Band,
) -> Result<Vec<SectorEntry>, SplitReduceError> {
    classify_neighborhood_rays(body, plane, sides, vertex, &SlitTips::of(body), band)
        .map(|(entries, _)| entries)
}

/// One entry's ray out of the vertex: its direction scaled to its
/// reach (an edge entry's [`chord`], a duplicate's bisector at its
/// sector's arm), and whether the plane holds it (its side read On,
/// before rules (a) and (b) send it to one), and the cone apex the
/// sector after it is read at, where its face is one there.
#[derive(Clone, Copy, Debug)]
pub(super) struct EntryRay<T: geom_core::Real> {
    pub dir: Vec3<T>,
    pub in_plane: bool,
    pub apex: Option<crate::sector_face::ConeApex<T>>,
}

/// [`classify_neighborhood`], with each entry's [`EntryRay`] beside it,
/// `tips` the body's slit tips.
pub(super) fn classify_neighborhood_rays<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    sides: &SecondaryMap<VertexKey, PlaneSide>,
    vertex: VertexKey,
    tips: &SlitTips,
    band: Band,
) -> Result<(Vec<SectorEntry>, Vec<EntryRay<T>>), SplitReduceError> {
    if body.get_vertex(vertex).is_none() {
        return Err(SplitReduceError::StaleVertex { vertex });
    }
    let orbit = body.vertex_orbit_linked(vertex);
    if orbit.is_empty() {
        return Err(SplitReduceError::LoneVertex { vertex });
    }

    let mut entries = Vec::with_capacity(orbit.len());
    let mut dirs = Vec::with_capacity(orbit.len());
    let mut apexes = Vec::with_capacity(orbit.len());
    for (i, &he) in orbit.iter().enumerate() {
        let (dir_a, class) = entry_side(
            body,
            vertex,
            he,
            (
                plane.normal.get(),
                geom_brep::ReferenceNormal::of_split_plane(plane.normal.get()),
            ),
            |far| {
                sides
                    .get(far)
                    .copied()
                    .ok_or(SplitReduceError::UnrecordedSide { vertex: far })
            },
            band,
        )
        .map_err(|fault| match fault {
            EntryFault::Walk(e) | EntryFault::Far(e) => e,
            EntryFault::Undecided(diag) => match sector_face(body, vertex, he, band) {
                Ok((face, _, _)) => SplitReduceError::SliverSector {
                    vertex,
                    face,
                    reading: SectorReading::Escalated(diag),
                },
                Err(e) => e,
            },
        })?;
        entries.push(SectorEntry {
            he,
            kind: SectorEntryKind::Edge,
            class,
        });
        dirs.push(dir_a);

        // Wideness of the sector CW-after `he` (bounded by `he` and the
        // next orbit edge), and the subdivision direction it implies:
        // the three rungs and a slit tip's full turn are
        // [`corner_shape`] — ONE implementation, called from here and from the boolean lane's
        // sector walk, under the one pooled set of K names (pooled in
        // #652). This is a call, not a copy. The derivation the
        // module docs above carry — why convex subdivision, why the
        // wideness trilean has no escalation cliff — is what that
        // shared body implements.
        let next_he = orbit[(i + 1) % orbit.len()];
        let (_, dir_b, _) = chord(body, vertex, next_he)?;
        let (face, at_vertex, _) = sector_face(body, vertex, he, band)?;
        let apex = match at_vertex {
            VertexFace::Tangent(_) | VertexFace::NearAxis(_) => None,
            VertexFace::Apex(apex) => Some(apex),
        };
        apexes.push(apex);
        let wide = wide_entries(
            (vertex, face),
            (dir_a, dir_b),
            at_vertex,
            tips.bounds(vertex, face, he == next_he),
            plane.normal.get(),
            band,
        )?;
        for (bisector, arm, class) in wide {
            entries.push(SectorEntry {
                he,
                kind: SectorEntryKind::WideBisector,
                class,
            });
            dirs.push(bisector * arm);
            apexes.push(apex);
        }
    }

    let rays: Vec<EntryRay<T>> = dirs
        .iter()
        .zip(&entries)
        .zip(apexes)
        .map(|((&dir, e), apex)| EntryRay {
            dir,
            in_plane: e.class == PlaneSide::On,
            apex,
        })
        .collect();
    let readings = rules::apply_rule_a(body, plane, vertex, &mut entries, &dirs, band)?;
    rules::apply_rule_b(body, vertex, &mut entries, &readings, band)?;
    Ok((entries, rays))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::fixtures::raw_prism;
    use geom_core::Tol;

    /// The split lane has no sphere arm and says so BY NAME, on the
    /// shared walk's report rather than by re-matching the surface —
    /// an arm the shared walk created and nothing covered until this
    /// row.
    #[test]
    fn a_sphere_carried_sector_refuses_by_name() {
        let p = raw_prism(3, Tol::witness());
        let face = p.face_side[0];
        let mut body = p.body;
        body.set_face_surface(
            face,
            crate::construction::FaceSurface::New {
                surface: geom::Surface::Sphere {
                    center: geom_core::Point3::new(0.0, 0.0, 0.0),
                    radius: 2.0,
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
        let outer = body.get_face(face).unwrap().outer;
        let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
        else {
            panic!("the side face's outer loop is a cycle");
        };
        let orbit_he = body.mate(first).unwrap();
        let vertex = body.get_half_edge(orbit_he).unwrap().start;
        match sector_face(
            &body,
            vertex,
            orbit_he,
            Band::linear(Tol::witness()).unwrap(),
        ) {
            Err(SplitReduceError::CurvedBooleanUnsupported { face: f, kind }) => {
                assert_eq!(f, face);
                assert_eq!(kind, geom::SurfaceKind::Sphere);
            }
            other => panic!("expected the typed sphere refusal, got {other:?}"),
        }
    }
}

/// **A spline leaves a vertex along its tangent, not its chord**: the
/// boolean sweep rows' Bézier sheet, whose arc leaves `(0, 0, 0)` along
/// `(1, 2, 0)` while its chord runs along `x̂`
/// (`boolean::sectors::spline_departure_rows`, the boolean walk's twin).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod spline_departure_rows {
    use crate::boolean::reduce::planar_lane_carrier_rows::arc_sheet;
    use geom_core::Vec3;

    #[test]
    fn the_split_walk_reads_a_splines_tangent() {
        let (body, edge) = arc_sheet(1.0, 1.0);
        let he = body.get_edge(edge).unwrap().he_plus;
        let vertex = body.get_half_edge(he).unwrap().start;
        let (_, dir, bend) = super::chord(&body, vertex, he).unwrap();
        let tangent = Vec3::new(1.0, 2.0, 0.0).normalize();
        assert!(
            (dir.normalize() - tangent).norm() < 1e-12,
            "the arc leaves along its tangent (1, 2, 0)/√5, got {dir:?}"
        );
        assert!(
            (dir.norm() - 2.0).abs() < 1e-12,
            "levered at its extent, the chord's length 2: {dir:?}"
        );
        assert!(bend.is_some(), "a curved edge carries its bend");
    }
}
