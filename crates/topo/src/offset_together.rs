//! `offset_planes_together` — the SIMULTANEOUS offset door, planar
//! corners.
//!
//! # Why this exists, and why the per-face door could not be widened
//!
//! [`crate::replace_faces_offset`] moves ONE chart and re-describes its
//! boundary against neighbours that did not move. Composing it over
//! every chart of a body — which is what `shell` did — cannot produce
//! an offset body at an OBLIQUE junction, and the reason is arithmetic
//! rather than a posture:
//!
//! A corner where planes `n₁, n₂, n₃` meet is visited once per chart,
//! and each visit transports it rigidly by that chart's own `d·nᵢ`, so
//! it accumulates `Σ dᵢ·nᵢ`. The corner an offset body needs is the
//! point satisfying `nᵢ·x = nᵢ·oᵢ + dᵢ` for every `i` at once. Those
//! agree exactly when the normals are mutually perpendicular — which is
//! why a box has always been right and is bit-identical here — and
//! diverge otherwise: on a regular hexagonal prism at `t = 0.02` the
//! accumulation lands 11.5 mm from the true corner and leaves 30 mm of
//! wall where 20 mm was asked for.
//!
//! `ReanchorOffCarrier` is the gate that has been PREVENTING that body,
//! and it stays load-bearing for everything this door does not cover.
//! What this door does instead is solve the corner ONCE, against every
//! moved plane meeting it, and re-derive each edge as the intersection
//! of its TWO MOVED planes.
//!
//! # Scope, stated as a gate rather than as a hope
//!
//! **Every face of the body must be planar and must be in the moving
//! set.** Both are checked and refused typed
//! ([`ReplaceFaceError::TogetherNonPlanar`],
//! [`ReplaceFaceError::TogetherPartialSet`]) — a body with one curved
//! face has corners this door cannot solve, and a partially moving set
//! has corners whose answer depends on faces it was not told about.
//! Curved corners are the C5-table work that follows this unit; until
//! it lands they refuse where they always did.
//!
//! # What every step is, exactly
//!
//! Two planes translate, so their intersection line keeps its direction
//! and translates too — every motion here is a rigid translation, and
//! that is what keeps the door closed-form:
//!
//! 1. **the corner** — `nᵢ·x = cᵢ` over the distinct moved planes at
//!    the vertex, solved by Cramer on the first well-conditioned
//!    triple in arena order; any further plane is VERIFIED against the
//!    solution rather than assumed
//!    ([`ReplaceFaceError::TogetherCorner`] when it disagrees, which is
//!    the valence-past-3 shape);
//! 2. **the edge** — the intersection line of its two moved planes,
//!    oriented so the stored parameter order survives; a seam between
//!    two faces of ONE chart is not an intersection at all and
//!    translates with its own plane;
//! 3. **the description** — an intrinsic one re-points at the new
//!    surface keys and re-states its witness at the new mid-parameter;
//!    a mapped one translates by the edge's own displacement, which is
//!    the same rigid vector the carrier moved by.
//!
//! The conditioning of step 1 is metered, not assumed. A corner's
//! solve amplifies each plane's position error by `1/|det|`, and
//! `|det|` — a triple product of UNIT normals — is a pure number, which
//! no band in meters can classify. It is therefore metered by the total
//! offset the call is asking the body to absorb: that is the quantity
//! the ill conditioning would be amplifying, it is exact, and it needs
//! no comparison to compute. A corner that fails it is singular
//! (coplanar-adjacent faces, or three planes sharing a line) and
//! refuses typed.

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeGeometry};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Tol, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::euler::FaceSurface;
use crate::geometry::SurfaceKey;
use crate::props::PropsQuadLane;
use crate::replace_face::ReplaceFaceError;

/// One chart's move: the faces wearing it, and the signed distance
/// along its stored normal.
pub struct ChartMove<T: Real> {
    /// The faces wearing the chart. They must share one surface key.
    pub faces: Vec<FaceKey>,
    /// The signed offset along the chart's stored normal.
    pub distance: T,
}

/// A moved plane, resolved once and read many times.
struct MovedPlane<T: Real> {
    old_key: SurfaceKey,
    normal: Vec3<T>,
    /// `n · x = c` for the MOVED plane.
    c: T,
    /// The rigid displacement the plane itself underwent.
    delta: Vec3<T>,
}

/// **Offset every chart of `body` at once** (module docs).
///
/// `moves` names each chart and its signed distance; every face of the
/// body must appear exactly once across them.
///
/// # Errors
///
/// [`ReplaceFaceError`], the body untouched on every one: the whole
/// plan is decided before anything is written, and the writes go to a
/// clone that replaces `body` only on success.
pub fn offset_planes_together<T: Decide + PropsQuadLane>(
    body: &mut Body<T>,
    moves: &[ChartMove<T>],
    band: Band,
    tol: Tol,
) -> Result<(), ReplaceFaceError<T>> {
    // ---- Decide: the scope gate. ----
    let mut planes: Vec<(FaceKey, MovedPlane<T>)> = Vec::new();
    for m in moves {
        for &face in &m.faces {
            let data = body
                .get_face(face)
                .ok_or(ReplaceFaceError::StaleFace { face })?;
            let surface = body
                .get_surface(data.surface)
                .ok_or(ReplaceFaceError::Corrupt)?;
            let Surface::Plane { origin, normal, .. } = surface else {
                return Err(ReplaceFaceError::TogetherNonPlanar {
                    face,
                    kind: geom_brep::SurfaceKind::of(surface),
                });
            };
            let delta = *normal * m.distance;
            planes.push((
                face,
                MovedPlane {
                    old_key: data.surface,
                    normal: *normal,
                    c: normal.dot(radius(*origin + delta)),
                    delta,
                },
            ));
        }
    }
    for (face, _) in body.faces() {
        if !planes.iter().any(|(k, _)| *k == face) {
            return Err(ReplaceFaceError::TogetherPartialSet { face });
        }
    }
    let plane_of = |face: FaceKey| planes.iter().find(|(k, _)| *k == face).map(|(_, p)| p);

    // ---- The scale every dimensionless margin is metered by. ----
    //
    // A corner's solve and an edge's plane pair both turn on quantities
    // that are pure numbers — a triple product of unit normals, the
    // sine between two of them — and a pure number cannot be classified
    // against a band in meters. The lever is the total offset the
    // operation is asking the body to absorb: it is what the ill
    // conditioning would be amplifying, it is available exactly, and it
    // needs no comparison to compute.
    let scale = moves
        .iter()
        .fold(T::zero(), |acc, m| acc + m.distance.abs());

    // ---- Decide: every corner, before anything is written. ----
    let mut moved: Vec<(VertexKey, Point3<T>)> = Vec::new();
    for (vertex, _) in body.vertices() {
        let mut at: Vec<&MovedPlane<T>> = Vec::new();
        for face in faces_at_vertex(body, vertex)? {
            let p = plane_of(face).ok_or(ReplaceFaceError::Corrupt)?;
            if !at.iter().any(|q| q.old_key == p.old_key) {
                at.push(p);
            }
        }
        moved.push((vertex, solve_corner(vertex, &at, scale, band)?));
    }
    let point_at = |v: VertexKey| moved.iter().find(|(k, _)| *k == v).map(|(_, p)| *p);

    // ---- Decide: every edge's carrier and description. ----
    let mut specs: Vec<(EdgeKey, EdgeCurveSpec<T>)> = Vec::new();
    for (edge, edge_data) in body.edges() {
        let (fa, fb) =
            crate::replace_face::edge_faces(body, edge).ok_or(ReplaceFaceError::Corrupt)?;
        let (pa, pb) = (
            plane_of(fa).ok_or(ReplaceFaceError::Corrupt)?,
            plane_of(fb).ok_or(ReplaceFaceError::Corrupt)?,
        );
        let start = body
            .get_half_edge(edge_data.he_plus)
            .ok_or(ReplaceFaceError::Corrupt)?
            .start;
        let end = body
            .half_edge_end(edge_data.he_plus)
            .ok_or(ReplaceFaceError::Corrupt)?;
        let (p_start, p_end) = (
            point_at(start).ok_or(ReplaceFaceError::Corrupt)?,
            point_at(end).ok_or(ReplaceFaceError::Corrupt)?,
        );
        let curve = body
            .get_curve_geom(edge_data.curve)
            .and_then(crate::null::CurveGeom::certified)
            .ok_or(ReplaceFaceError::Corrupt)?;
        let old_carrier = curve.carrier().clone();
        let (t0_old, t1_old) = curve.params();
        let description = *curve.description();
        let old_start = old_carrier.eval(t0_old);

        // A SEAM between two faces of one chart is not an intersection
        // of two planes — the two are the same plane — so it moves with
        // that plane and keeps its parameters. Every other edge is the
        // line the two moved planes meet in.
        let (carrier, t0, t1) = if pa.old_key == pb.old_key {
            (
                crate::replace_face::translate_curve(&old_carrier, pa.delta)
                    .map_err(|error| ReplaceFaceError::Structure { edge, error })?,
                t0_old,
                t1_old,
            )
        } else {
            let line = intersect_planes(edge, pa, pb, scale, band)?;
            let Curve3::Line { origin, dir } = line else {
                return Err(ReplaceFaceError::Corrupt);
            };
            let (mut t0, mut t1) = ((p_start - origin).dot(dir), (p_end - origin).dot(dir));
            // Keep the stored traversal forward: the attach layer's
            // span gate reads the parameter order, and flipping the
            // direction is the only free choice this construction has.
            let (origin, dir) = if matches!(
                decide("offset_together_span", Margin::of(t1 - t0), band),
                Ok(Sign::Negative)
            ) {
                let dir = -dir;
                t0 = (p_start - origin).dot(dir);
                t1 = (p_end - origin).dot(dir);
                (origin, dir)
            } else {
                (origin, dir)
            };
            (Curve3::Line { origin, dir }, t0, t1)
        };

        let mid = carrier.eval((t0 + t1) * T::from_f64(0.5));
        let displacement = p_start - old_start;
        specs.push((
            edge,
            EdgeCurveSpec {
                description: restate(description, mid, displacement, edge)?,
                carrier,
                param_start: t0,
                param_end: t1,
            },
        ));
    }

    // ---- Mutation, on a clone (every decision is done). ----
    let mut work = body.clone();
    let mut minted: Vec<(SurfaceKey, SurfaceKey)> = Vec::new();
    for m in moves {
        let Some(&first) = m.faces.first() else {
            return Err(ReplaceFaceError::EmptyGroup);
        };
        let p = plane_of(first).ok_or(ReplaceFaceError::Corrupt)?;
        let Some(Surface::Plane { origin, u_ref, .. }) = work
            .get_face(first)
            .and_then(|f| work.get_surface(f.surface))
            .cloned()
        else {
            return Err(ReplaceFaceError::Corrupt);
        };
        let new_key = work
            .set_face_surface(
                first,
                FaceSurface::New(Surface::Plane {
                    origin: origin + p.delta,
                    normal: p.normal,
                    u_ref,
                }),
            )
            .map_err(|error| ReplaceFaceError::Op { edge: None, error })?;
        for &member in &m.faces[1..] {
            work.set_face_surface(member, FaceSurface::Shared(new_key))
                .map_err(|error| ReplaceFaceError::Op { edge: None, error })?;
        }
        minted.push((p.old_key, new_key));
    }
    for (vertex, point) in &moved {
        let old_point = work
            .get_vertex(*vertex)
            .ok_or(ReplaceFaceError::Corrupt)?
            .point;
        let new_point = work.add_point(*point);
        work.get_vertex_mut(*vertex)
            .ok_or(ReplaceFaceError::Corrupt)?
            .point = new_point;
        work.remove_point_if_orphaned(old_point);
    }
    for (edge, mut spec) in specs {
        for (old, new) in &minted {
            spec.description = crate::replace_face::remap_description(spec.description, *old, *new);
        }
        work.set_edge_curve(edge, spec, tol)
            .map_err(|error| ReplaceFaceError::Op {
                edge: Some(edge),
                error,
            })?;
    }
    if let Err(errors) = crate::validate::validate_closed(&work) {
        return Err(ReplaceFaceError::ResultNotClosed { errors });
    }
    *body = work;
    Ok(())
}

/// `description` re-stated for the moved edge: an intrinsic one keeps
/// its (about to be remapped) surfaces with the witness at the new
/// mid-parameter; a mapped one translates by the edge's own rigid
/// displacement.
fn restate<T: Real>(
    description: EdgeGeometry<T>,
    mid: Point3<T>,
    displacement: Vec3<T>,
    edge: EdgeKey,
) -> Result<EdgeGeometry<T>, ReplaceFaceError<T>> {
    Ok(match description {
        EdgeGeometry::Intersection { s1, s2, .. } => EdgeGeometry::Intersection {
            s1,
            s2,
            witness: mid,
        },
        EdgeGeometry::TangentIntersection { s1, s2, .. } => EdgeGeometry::TangentIntersection {
            s1,
            s2,
            witness: mid,
        },
        EdgeGeometry::MappedCurve(m) => EdgeGeometry::MappedCurve(
            crate::replace_face::translate_mapped(m, displacement).ok_or(
                ReplaceFaceError::CarrierLaneUnsupported {
                    edge,
                    what: "a rotation-family mapped description (its trajectory does not \
                           translate)",
                },
            )?,
        ),
        other => other,
    })
}

/// The corner: `nᵢ·x = cᵢ` over the distinct moved planes at a vertex.
fn solve_corner<T: Decide>(
    vertex: VertexKey,
    at: &[&MovedPlane<T>],
    scale: T,
    band: Band,
) -> Result<Point3<T>, ReplaceFaceError<T>> {
    let non_simple = |what: &'static str| ReplaceFaceError::TogetherCorner {
        vertex,
        planes: at.len(),
        what,
    };
    if at.len() < 3 {
        return Err(non_simple(
            "fewer than three distinct planes meet here, so no point is determined",
        ));
    }
    // The first well-conditioned triple in arena order (D9).
    let mut solved = None;
    for i in 0..at.len() {
        for j in i + 1..at.len() {
            for k in j + 1..at.len() {
                let (a, b, c) = (at[i], at[j], at[k]);
                let det = a.normal.dot(b.normal.cross(c.normal));
                match decide(
                    "offset_together_corner",
                    Margin::of(det.abs() * scale),
                    band,
                ) {
                    Ok(Sign::Positive) => {}
                    Ok(_) => continue,
                    Err(source) => return Err(ReplaceFaceError::Escalated { source }),
                }
                solved = Some(cramer(a, b, c, det));
                break;
            }
            if solved.is_some() {
                break;
            }
        }
        if solved.is_some() {
            break;
        }
    }
    let point = solved.ok_or_else(|| {
        non_simple("every triple of planes here is singular (they share a line or a plane)")
    })?;
    // Any further plane is VERIFIED, never assumed: a valence-past-3
    // corner whose planes do not concur has no offset point at all,
    // and guessing one is how a wrong body gets built.
    for p in at {
        let residual = p.normal.dot(radius(point)) - p.c;
        match decide("offset_together_concurrence", Margin::of(residual), band) {
            Ok(Sign::Zero) => {}
            Ok(_) => {
                return Err(non_simple(
                    "the planes meeting here do not concur after the offset, so this corner has \
                     no offset point",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
    }
    Ok(point)
}

/// Cramer's rule on three plane equations with a known determinant.
fn cramer<T: Real>(a: &MovedPlane<T>, b: &MovedPlane<T>, c: &MovedPlane<T>, det: T) -> Point3<T> {
    let v = (b.normal.cross(c.normal) * a.c
        + c.normal.cross(a.normal) * b.c
        + a.normal.cross(b.normal) * c.c)
        / det;
    Point3::new(v.x, v.y, v.z)
}

/// The line two moved planes meet in.
fn intersect_planes<T: Decide>(
    edge: EdgeKey,
    a: &MovedPlane<T>,
    b: &MovedPlane<T>,
    scale: T,
    band: Band,
) -> Result<Curve3<T>, ReplaceFaceError<T>> {
    let d = a.normal.cross(b.normal);
    let n2 = d.dot(d);
    match decide(
        "offset_together_edge_pair",
        Margin::of(d.norm() * scale),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(_) => {
            return Err(ReplaceFaceError::TogetherCorner {
                vertex: VertexKey::default(),
                planes: 2,
                what: "an edge whose two faces' planes are parallel after the offset, so they \
                       meet in no line",
            });
        }
        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
    }
    let _ = edge;
    let p = (b.normal.cross(d) * a.c + d.cross(a.normal) * b.c) / n2;
    Ok(Curve3::Line {
        origin: Point3::new(p.x, p.y, p.z),
        dir: d.normalize(),
    })
}

/// A point read as the vector from the origin — the form a plane
/// equation's `n · x` needs.
fn radius<T: Real>(p: Point3<T>) -> Vec3<T> {
    Vec3::new(p.x, p.y, p.z)
}

/// Every face incident to a vertex, in orbit order.
fn faces_at_vertex<T: Real>(
    body: &Body<T>,
    vertex: VertexKey,
) -> Result<Vec<FaceKey>, ReplaceFaceError<T>> {
    let Some(emanating) = body
        .get_vertex(vertex)
        .ok_or(ReplaceFaceError::Corrupt)?
        .emanating
    else {
        return Ok(Vec::new());
    };
    let orbit = body
        .vertex_orbit(emanating)
        .ok_or(ReplaceFaceError::Corrupt)?;
    let mut out = Vec::new();
    for he in orbit {
        let face = body
            .get_loop(
                body.get_half_edge(he)
                    .ok_or(ReplaceFaceError::Corrupt)?
                    .parent_loop,
            )
            .ok_or(ReplaceFaceError::Corrupt)?
            .face;
        if !out.contains(&face) {
            out.push(face);
        }
    }
    Ok(out)
}
