//! **The uncut-shell witness**: which side of the other operand a
//! shell lies on when the other operand's boundary does not cut it —
//! the containment fallback's per-shell verdict and the uncut-component
//! probe of `setopfinish`, one rule for both.
//!
//! An uncut shell meets the other operand's boundary only where it
//! lies ON it (a declared flush face, a vertex at rest on a face), so
//! every point of the shell OFF that boundary is on one side of it,
//! and [`point_in_solid`] at any such point names the shell's side.
//! Where the operands have no crossings, a curved boundary has this
//! premise from the extent certificates that run before the fallback
//! (`ops::sphere_extent_scan`, `ops::section_extent_pass`).
//!
//! The witnesses are the shell's own points, one per cell, in
//! increasing dimension:
//!
//! 1. each vertex, except the declared-contact vertices of
//!    [`contact_skip_set`];
//! 2. each edge's carrier at its parameter midpoint
//!    ([`geom_brep::EdgeCurve::mid_point`]), a point ON the edge
//!    whatever its kind;
//! 3. one point of each planar face's relative interior: the first
//!    candidate — a consecutive vertex triple's centroid, then the
//!    midpoint of two of the face's vertices — that
//!    [`point_in_face`] certifies strictly inside the face.
//!
//! The first witness off the other boundary decides. A block inside
//! another, flush on four walls, reaches the third tier: its vertices
//! and edges all lie on the other boundary, and the interior of each
//! end face does not.
//!
//! When every witness lies on the other boundary the shell's side is
//! undecided, and the probe refuses
//! [`BooleanError::ShellWitnessExhausted`] naming the shell. Two
//! operands that are one body reach this, as does a shell whose
//! vertices and edges all lie on the other boundary and whose faces off
//! it are all curved (tier 3 reads planar faces only).

use geom_core::{Band, Decide, Point3, Tol, Vec3};
use slotmap::SecondaryMap;

use super::solid_contain::{
    PointInSolidError, SolidContainment, face_plane, point_in_face, point_in_solid,
};
use super::{BooleanError, ContactRecords, Operand, SideCode};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, ShellKey, VertexKey};
use crate::splitting::PointInLoopError;

/// The side of `other` the uncut `shell` of `body` lies on (module
/// docs), read off its first witness off `other`'s boundary.
///
/// # Errors
///
/// [`BooleanError::ShellWitnessExhausted`] when every witness lies on
/// `other`'s boundary; [`BooleanError::Containment`] when a probe
/// refuses; [`BooleanError::JoinDesync`] when the shell does not walk.
pub(super) fn shell_side<T: Decide>(
    body: &Body<T>,
    shell: ShellKey,
    other: &Body<T>,
    skip: &SecondaryMap<VertexKey, ()>,
    operand: Operand,
    band: Band,
    tol: Tol,
) -> Result<SideCode, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let faces = &body
        .get_shell(shell)
        .ok_or(desync("uncut shell no longer resolves"))?
        .faces;
    let side = |q: Point3<T>| -> Result<Option<SideCode>, BooleanError> {
        Ok(
            match point_in_solid(other, q, band, tol).map_err(BooleanError::Containment)? {
                SolidContainment::In => Some(SideCode::In),
                SolidContainment::Out => Some(SideCode::Out),
                SolidContainment::OnBoundary => None,
            },
        )
    };
    let mut halves: Vec<(FaceKey, HalfEdgeKey)> = Vec::new();
    for &face in faces {
        let f = body
            .get_face(face)
            .ok_or(desync("uncut shell face no longer resolves"))?;
        for l in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
            let LoopBoundary::Cycle { first } = body
                .get_loop(l)
                .ok_or(desync("uncut shell loop no longer resolves"))?
                .boundary
            else {
                continue;
            };
            for he in body
                .loop_cycle(first)
                .ok_or(desync("uncut shell loop not walkable"))?
            {
                halves.push((face, he));
            }
        }
    }
    let point = |v: VertexKey| {
        body.get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or(desync("uncut shell vertex has no point"))
    };

    let mut seen_vertex: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    for &(_, he) in &halves {
        let v = body
            .get_half_edge(he)
            .ok_or(desync("uncut shell half-edge no longer resolves"))?
            .start;
        if skip.contains_key(v) || seen_vertex.insert(v, ()).is_some() {
            continue;
        }
        if let Some(s) = side(point(v)?)? {
            return Ok(s);
        }
    }

    let mut seen_edge: SecondaryMap<EdgeKey, ()> = SecondaryMap::new();
    for &(_, he) in &halves {
        let e = body
            .get_half_edge(he)
            .ok_or(desync("uncut shell half-edge no longer resolves"))?
            .edge;
        if seen_edge.insert(e, ()).is_some() {
            continue;
        }
        let curve = body
            .get_edge(e)
            .and_then(|ed| body.get_curve_geom(ed.curve))
            .ok_or(desync("uncut shell edge has no curve"))?;
        let Some(curve) = curve.certified() else {
            continue;
        };
        if let Some(s) = side(curve.mid_point())? {
            return Ok(s);
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
            return Ok(s);
        }
    }

    Err(BooleanError::ShellWitnessExhausted { operand, shell })
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
    let desync = |what| BooleanError::JoinDesync { what };
    let f = body
        .get_face(face)
        .ok_or(desync("uncut shell face no longer resolves"))?;
    let third = T::from_f64(1.0 / 3.0);
    let half = T::from_f64(0.5);
    let mut loops = Vec::new();
    for l in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body
            .get_loop(l)
            .ok_or(desync("uncut shell loop no longer resolves"))?
            .boundary
        else {
            continue;
        };
        let points = body
            .loop_cycle(first)
            .ok_or(desync("uncut shell loop not walkable"))?
            .into_iter()
            .map(|he| {
                body.get_half_edge(he)
                    .and_then(|h| body.get_vertex(h.start))
                    .and_then(|vd| body.get_point(vd.point).copied())
                    .ok_or(desync("uncut shell vertex has no point"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        loops.push(points);
    }
    let triples = loops.iter().filter(|p| p.len() >= 3).flat_map(|p| {
        let n = p.len();
        (0..n).map(move |i| {
            let (a, b, c) = (p[i], p[(i + 1) % n], p[(i + 2) % n]);
            a + ((b - a) + (c - a)) * third
        })
    });
    let vertices: Vec<Point3<T>> = loops.concat();
    let chords = vertices
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| vertices[i + 1..].iter().map(move |&b| a.lerp(b, half)));
    for q in triples.chain(chords) {
        if certified_in_face(body, face, normal, q, band)? {
            return Ok(Some(q));
        }
    }
    Ok(None)
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

/// The declared-contact vertex skip set of one operand.
pub(super) fn contact_skip_set(
    contacts: &ContactRecords,
    operand: Operand,
) -> SecondaryMap<VertexKey, ()> {
    let mut skip = SecondaryMap::new();
    for c in &contacts.vv {
        skip.insert(
            match operand {
                Operand::A => c.a,
                Operand::B => c.b,
            },
            (),
        );
    }
    let list = match operand {
        Operand::A => &contacts.a_on_b,
        Operand::B => &contacts.b_on_a,
    };
    for c in list {
        skip.insert(c.vertex, ());
    }
    skip
}
