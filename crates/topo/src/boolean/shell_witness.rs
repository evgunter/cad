//! **The cell-dimension witness ladder**: which side of the other
//! operand a cell complex — a set of faces of one operand — lies on,
//! where the other operand's boundary does not cut it. Two questions
//! read it:
//!
//! - **the uncut-shell witness** ([`shell_side`]): the containment
//!   fallback's per-shell verdict and the uncut-component probe of
//!   `setopfinish`. The containment fallback runs only when the
//!   operands have no crossings, and for a curved boundary the extent
//!   certificates that run before it (`ops::sphere_extent_scan`,
//!   `ops::section_extent_pass`) certify that none was missed;
//!   `setopfinish` classifies a component that carries no section face,
//!   so its ground is the join's: every crossing of the two boundaries
//!   was found and cut, and this component met none.
//! - **section-loop role resolution** (`join::resolve_roles_geometric`):
//!   the region faces flanking each loop of a completed section
//!   polygon, read once every polygon is cut, so no crossing runs
//!   through them.
//!
//! An uncut complex meets the other operand's boundary only where it
//! lies ON it (a seam, a declared flush face, a vertex at rest on a
//! face), so every point of it OFF that boundary is on one side, and
//! [`point_in_solid`] at any such point names the complex's side. One
//! decisive witness is therefore the answer; a second could only agree.
//!
//! The witnesses are the complex's own points, one per cell, in
//! increasing dimension:
//!
//! 1. each vertex;
//! 2. each edge's carrier at its parameter midpoint
//!    ([`geom_brep::EdgeCurve::mid_point`]), a point ON the edge
//!    whatever its kind;
//! 3. one point of each planar face's relative interior: the first
//!    candidate — a consecutive vertex triple's centroid, then the
//!    midpoint of two of the face's vertices — that
//!    [`point_in_face`] certifies strictly inside the face.
//!
//! A witness is **inconclusive** when it reads `OnBoundary`, or when
//! its reading is in-band (an escalated margin, a ray schedule that
//! only grazed): the point is on the other boundary or too near it to
//! say, and the next witness is read. A declared contact is one or the
//! other — its carriers differ by less than the band, and the reduction
//! took them as one — so no record of contacts is consulted, at any
//! dimension. Any other refusal is about the other operand rather than
//! the point, and propagates.
//!
//! The first decisive witness decides. A block inside another, flush on
//! four walls, reaches the third tier: its vertices and edges all lie
//! on the other boundary, and the interior of each end face does not.
//! When no witness decides, the complex's side is undecided: the
//! reading names the first in-band witness's refusal if there was one.
//! A complex whose vertices and edges all lie on the other boundary and
//! whose faces off it are all curved reaches this (tier 3 reads planar
//! faces only).

use geom_core::{Band, Decide, Point3, Tol, Vec3};
use slotmap::SecondaryMap;

use super::solid_contain::{
    PointInSolidError, SolidContainment, face_plane, point_in_face, point_in_solid,
};
use super::{BooleanError, Operand, SideCode};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, ShellKey, VertexKey};
use crate::splitting::PointInLoopError;

/// What the ladder read off a complex (module docs).
#[derive(Debug)]
pub(super) enum Reading {
    /// The first decisive witness's side: `In` or `Out`.
    Side(SideCode),
    /// No witness decided; the first in-band refusal met, if any.
    Undecided(Option<PointInSolidError>),
}

/// The side of `other` the cell complex `faces` of `body` lies on,
/// read off its first decisive witness (module docs).
///
/// # Errors
///
/// [`BooleanError::Containment`] when a probe refuses other than
/// in-band; [`BooleanError::JoinDesync`] when the complex does not walk.
pub(super) fn complex_side<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    other: &Body<T>,
    band: Band,
    tol: Tol,
) -> Result<Reading, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut in_band: Option<PointInSolidError> = None;
    let mut side = |q: Point3<T>| -> Result<Option<SideCode>, BooleanError> {
        Ok(match point_in_solid(other, q, band, tol) {
            Ok(SolidContainment::In) => Some(SideCode::In),
            Ok(SolidContainment::Out) => Some(SideCode::Out),
            Ok(SolidContainment::OnBoundary) => None,
            Err(e) if is_in_band(&e) => {
                in_band.get_or_insert(e);
                None
            }
            Err(e) => return Err(BooleanError::Containment(e)),
        })
    };
    let mut halves: Vec<HalfEdgeKey> = Vec::new();
    for &face in faces {
        halves.extend(face_loops(body, face)?.into_iter().flatten());
    }
    let half = |he: HalfEdgeKey| {
        body.get_half_edge(he)
            .ok_or(desync("witnessed half-edge no longer resolves"))
    };

    let mut seen_vertex: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    for &he in &halves {
        let v = half(he)?.start;
        if seen_vertex.insert(v, ()).is_some() {
            continue;
        }
        let p = body
            .get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or(desync("witnessed vertex has no point"))?;
        if let Some(s) = side(p)? {
            return Ok(Reading::Side(s));
        }
    }

    let mut seen_edge: SecondaryMap<EdgeKey, ()> = SecondaryMap::new();
    for &he in &halves {
        let e = half(he)?.edge;
        if seen_edge.insert(e, ()).is_some() {
            continue;
        }
        let curve = body
            .get_edge(e)
            .and_then(|ed| body.get_curve_geom(ed.curve))
            .ok_or(desync("witnessed edge has no curve"))?;
        let Some(curve) = curve.certified() else {
            continue;
        };
        if let Some(s) = side(curve.mid_point())? {
            return Ok(Reading::Side(s));
        }
    }

    for &face in faces {
        let normal = match face_plane(body, face) {
            Ok((_, normal)) => normal,
            Err(PointInSolidError::KindUnsupported { .. }) => continue,
            Err(e) => return Err(BooleanError::Containment(e)),
        };
        if let Some(q) = face_interior_point(body, face, normal, band)?
            && let Some(s) = side(q)?
        {
            return Ok(Reading::Side(s));
        }
    }

    Ok(Reading::Undecided(in_band))
}

/// Is `e` an in-band reading of the probed point (module docs)?
fn is_in_band(e: &PointInSolidError) -> bool {
    matches!(
        e,
        PointInSolidError::Escalated { .. }
            | PointInSolidError::RayExhausted
            | PointInSolidError::Loop(
                PointInLoopError::Escalated { .. } | PointInLoopError::RayExhausted { .. },
            )
    )
}

/// The side of `other` the uncut `shell` of `body` lies on: the
/// ladder (module docs) over the shell's faces.
///
/// # Errors
///
/// [`BooleanError::Containment`] when a probe refuses, or when no
/// witness decides and one read in-band;
/// [`BooleanError::ShellWitnessExhausted`] when every witness lies on
/// `other`'s boundary; [`BooleanError::JoinDesync`] when the shell does
/// not walk.
pub(super) fn shell_side<T: Decide>(
    body: &Body<T>,
    shell: ShellKey,
    other: &Body<T>,
    operand: Operand,
    band: Band,
    tol: Tol,
) -> Result<SideCode, BooleanError> {
    let faces = &body
        .get_shell(shell)
        .ok_or(BooleanError::JoinDesync {
            what: "uncut shell no longer resolves",
        })?
        .faces;
    match complex_side(body, faces, other, band, tol)? {
        Reading::Side(s) => Ok(s),
        Reading::Undecided(Some(e)) => Err(BooleanError::Containment(e)),
        Reading::Undecided(None) => Err(BooleanError::ShellWitnessExhausted { operand, shell }),
    }
}

/// The first candidate strictly inside planar `face` (module docs,
/// tier 3): each consecutive vertex triple's centroid, then the
/// midpoint of each pair of the face's vertices, over its outer loop
/// and every ring. `None` when no candidate certifies.
fn face_interior_point<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    band: Band,
) -> Result<Option<Point3<T>>, BooleanError> {
    let loops = face_loop_points(body, face)?;
    let triples = loops.iter().filter(|p| p.len() >= 3).flat_map(|p| {
        let n = p.len();
        (0..n).map(move |i| triple_centroid(p[i], p[(i + 1) % n], p[(i + 2) % n]))
    });
    let vertices: Vec<Point3<T>> = loops.concat();
    let chords = vertices
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| vertices[i + 1..].iter().map(move |&b| chord_midpoint(a, b)));
    for q in triples.chain(chords) {
        if certified_in_face(body, face, normal, q, band)? {
            return Ok(Some(q));
        }
    }
    Ok(None)
}

/// Each walkable loop of `face` — its outer loop, then every ring — as
/// its half-edge cycle. A loop with no cycle (a lone vertex) has no
/// edge and bounds no area, so it is left out.
pub(super) fn face_loops<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<Vec<HalfEdgeKey>>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let f = body
        .get_face(face)
        .ok_or(desync("face no longer resolves"))?;
    let mut out = Vec::new();
    for l in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body
            .get_loop(l)
            .ok_or(desync("face loop no longer resolves"))?
            .boundary
        else {
            continue;
        };
        out.push(
            body.loop_cycle(first)
                .ok_or(desync("face loop not walkable"))?,
        );
    }
    Ok(out)
}

/// [`face_loops`] as each half-edge's start point.
fn face_loop_points<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<Vec<Point3<T>>>, BooleanError> {
    face_loops(body, face)?
        .into_iter()
        .map(|cycle| {
            cycle
                .into_iter()
                .map(|he| {
                    body.get_half_edge(he)
                        .and_then(|h| body.get_vertex(h.start))
                        .and_then(|vd| body.get_point(vd.point).copied())
                        .ok_or(BooleanError::JoinDesync {
                            what: "face vertex has no point",
                        })
                })
                .collect()
        })
        .collect()
}

/// The centroid of three consecutive loop vertices: a face-interior
/// candidate, inside the face only where the corner at `b` is convex.
fn triple_centroid<T: Decide>(a: Point3<T>, b: Point3<T>, c: Point3<T>) -> Point3<T> {
    a + ((b - a) + (c - a)) * T::from_f64(1.0 / 3.0)
}

/// The midpoint of two of a face's vertices: a face-interior candidate
/// wherever the chord between them is a diagonal of the face.
fn chord_midpoint<T: Decide>(a: Point3<T>, b: Point3<T>) -> Point3<T> {
    a.lerp(b, T::from_f64(0.5))
}

/// Does [`point_in_face`] certify `p` strictly inside planar `face`?
/// An inconclusive answer — outside, on a loop, an in-band margin, an
/// exhausted schedule, an outline the walk cannot cross — is `false`:
/// the candidate is discarded, never probed. A face the walk cannot
/// read at all is an error.
pub(super) fn certified_in_face<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    p: Point3<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    match point_in_face(body, face, normal, p, band) {
        Ok(verdict) => Ok(verdict == Some(true)),
        Err(
            PointInSolidError::Escalated { .. }
            | PointInSolidError::Loop(
                PointInLoopError::Escalated { .. } | PointInLoopError::RayExhausted { .. },
            )
            | PointInSolidError::EdgeCarrierUnsupported { .. },
        ) => Ok(false),
        Err(e) => Err(BooleanError::Containment(e)),
    }
}
