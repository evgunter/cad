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
//! its reading is in-band ([`inconclusive`]): the point is on the other
//! boundary or too near it to say, and the next witness is read. Any
//! other refusal is about the other operand rather than the point, and
//! propagates.
//!
//! No record of contacts is consulted, at any dimension. A vertex the
//! reduction recorded ON the other boundary is there by geometry, within
//! the band's zero, and reads `OnBoundary`; a declared pair whose
//! carriers sit in the band's sliver is refused by the reduction before
//! any witness runs. So a recorded contact never reads decisively, which
//! [`debug_assert_contacts_undecisive`] checks on every boolean that
//! reaches the ladder.
//!
//! The first decisive witness decides. A block inside another, flush on
//! four walls, reaches the third tier: its vertices and edges all lie
//! on the other boundary, and the interior of each end face does not.
//! When no witness decides, the complex's side is undecided, and the
//! reading says how many witnesses read the other boundary and how many
//! read too near it to say. That is the cause; an in-band reading is
//! about one point, possibly near a face far from the complex, and rides
//! along as evidence only. A complex whose vertices and edges all lie on
//! the other boundary and whose faces off it are all curved reaches this
//! (tier 3 reads planar faces only).

use geom_core::{Band, Decide, Point3, Tol, Vec3};
use slotmap::SecondaryMap;

use super::solid_contain::{
    PointInSolidError, SolidContainment, face_plane, point_in_face, point_in_solid,
};
use super::{BooleanError, ContactRecords, Operand, SideCode};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, ShellKey, VertexKey};
use crate::splitting::PointInLoopError;

/// What the ladder read off a complex (module docs).
#[derive(Debug)]
pub(super) enum Reading {
    /// The first decisive witness's side: `In` or `Out`.
    Side(SideCode),
    /// No witness decided.
    Undecided(Tally),
}

/// The witnesses of a complex none of which decided.
#[derive(Debug, Default)]
pub(super) struct Tally {
    /// Witnesses that read `OnBoundary`.
    pub(super) on_boundary: usize,
    /// Witnesses that read in-band ([`inconclusive`]).
    pub(super) in_band: usize,
    /// The first in-band reading, as evidence.
    pub(super) first_in_band: Option<PointInSolidError>,
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
    let mut tally = Tally::default();
    let mut side = |q: Point3<T>| -> Result<Option<SideCode>, BooleanError> {
        Ok(match point_in_solid(other, q, band, tol) {
            Ok(SolidContainment::In) => Some(SideCode::In),
            Ok(SolidContainment::Out) => Some(SideCode::Out),
            Ok(SolidContainment::OnBoundary) => {
                tally.on_boundary += 1;
                None
            }
            Err(e) if inconclusive(&e) => {
                tally.in_band += 1;
                tally.first_in_band.get_or_insert(e);
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

    Ok(Reading::Undecided(tally))
}

/// Is `e` a reading too near a boundary to say, about the one point
/// asked — the ladder's inconclusive refusal (module docs), and the
/// certificate's ([`certified_in_face`])? Another point of the same
/// complex can still decide.
///
/// - **In**: `Escalated` (a margin in the band's sliver),
///   `RayExhausted` (every schedule ray grazed), and the in-plane loop
///   walk's own two (`Loop(Escalated)`, `Loop(RayExhausted)`).
/// - **Out**: every refusal about a body rather than a point — a face
///   kind or edge carrier the door has no arm for, a corrupt face, a
///   zero or uncertified volume, a sphere chart it cannot read. Each
///   would answer the same at any point, so passing over it would only
///   defer it.
fn inconclusive(e: &PointInSolidError) -> bool {
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
/// [`BooleanError::Containment`] when a probe refuses other than
/// in-band; [`BooleanError::ShellWitnessExhausted`] when no witness
/// decides; [`BooleanError::JoinDesync`] when the shell does not walk.
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
        Reading::Undecided(t) => Err(BooleanError::ShellWitnessExhausted {
            operand,
            shell,
            on_boundary: t.on_boundary,
            in_band: t.in_band,
            first_in_band: t.first_in_band,
        }),
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
/// `false` discards the candidate unprobed: outside, on a loop, an
/// [`inconclusive`] reading, or an edge of `face` whose carrier the
/// walk cannot cross — that face then offers no candidate, as a curved
/// face offers none. Any other refusal is an error.
pub(super) fn certified_in_face<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    p: Point3<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    match point_in_face(body, face, normal, p, band) {
        Ok(verdict) => Ok(verdict == Some(true)),
        Err(e) if inconclusive(&e) => Ok(false),
        Err(PointInSolidError::EdgeCarrierUnsupported { .. }) => Ok(false),
        Err(e) => Err(BooleanError::Containment(e)),
    }
}

/// Every vertex the reduction recorded ON the other operand
/// (`ContactRecords::vv`, `a_on_b`, `b_on_a`) reads `OnBoundary` or
/// in-band against it, never `In` or `Out` (module docs: the reason the
/// ladder reads no contact record). Debug builds only; a refusal of the
/// probe is not this check's question and is passed over.
pub(super) fn debug_assert_contacts_undecisive<T: Decide>(
    contacts: &ContactRecords,
    a: (&Body<T>, &Body<T>),
    b: (&Body<T>, &Body<T>),
    band: Band,
    tol: Tol,
) {
    if !cfg!(debug_assertions) {
        return;
    }
    let check = |(body, other): (&Body<T>, &Body<T>), v: VertexKey, operand: Operand| {
        let Some(p) = body
            .get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
        else {
            return;
        };
        let read = point_in_solid(other, p, band, tol);
        debug_assert!(
            !matches!(read, Ok(SolidContainment::In | SolidContainment::Out)),
            "a contact vertex of operand {operand:?} recorded ON the other operand reads \
             decisively: {read:?}"
        );
    };
    for c in &contacts.vv {
        check(a, c.a, Operand::A);
        check(b, c.b, Operand::B);
    }
    for c in &contacts.a_on_b {
        check(a, c.vertex, Operand::A);
    }
    for c in &contacts.b_on_a {
        check(b, c.vertex, Operand::B);
    }
}
