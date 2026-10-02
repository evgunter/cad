//! **What an uncut cell complex is to the other operand** — a set of
//! faces of one operand that the other operand's boundary does not
//! cut. Two answers come from two sources, and they are asked in order:
//!
//! 1. **a side**, In or Out, from the complex's own points
//!    (the cell-dimension witness ladder, [`complex_side`]);
//! 2. for a whole shell the ladder leaves undecided with every witness
//!    ON the other boundary, **`On`** — the shell lies on a shell of
//!    the other operand — from the coincidences the reduction settled
//!    ([`on_verdict`], [`check_mutual`]).
//!
//! Two questions read it:
//!
//! - **the uncut-shell verdict** ([`shell_verdict`]): the containment
//!   fallback's per-shell verdict and the uncut-component probe of
//!   `setopfinish`. Both answers apply. The containment fallback runs
//!   only when the operands have no crossings, and for a curved
//!   boundary the extent certificates that run before it
//!   (`ops::sphere_extent_scan`, `ops::section_extent_pass`) certify
//!   that none was missed; `setopfinish` classifies a component that
//!   carries no section face, so its ground is the join's: every
//!   crossing of the two boundaries was found and cut, and this
//!   component met none.
//! - **section-loop role resolution** (`join::resolve_roles_geometric`):
//!   the region faces flanking each loop of a completed section
//!   polygon, read once every polygon is cut, so no crossing runs
//!   through them. Only the side applies: a region is not a shell.
//!
//! # The side: the witness ladder
//!
//! An uncut complex meets the other operand's boundary only where it
//! lies ON it (a seam, a declared flush face, a vertex at rest on a
//! face), so every point of it OFF that boundary is on one side, and
//! [`point_in_solid`] at any such point names the complex's side. One
//! decisive witness therefore names the side; a second could only
//! agree.
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
//! The ladder consults no record of contacts, at any dimension. A
//! vertex the reduction recorded ON the other boundary is there by
//! geometry, within the band's zero, and reads `OnBoundary`; a declared
//! pair whose carriers sit in the band's sliver is refused by the
//! reduction before any witness runs. So a recorded contact never reads
//! decisively, which [`debug_assert_contacts_undecisive`] checks on
//! every boolean that reaches the ladder.
//!
//! The first decisive witness decides. A block inside another, flush on
//! four walls, reaches the third tier: its vertices and edges all lie
//! on the other boundary, and the interior of each end face does not.
//! When no witness decides, the reading says how many witnesses read
//! the other boundary and how many read too near it to say. An in-band
//! reading is about one point, possibly near a face far from the
//! complex, and rides along as evidence only: a shell with one refuses
//! [`BooleanError::ShellWitnessExhausted`].
//!
//! # `On`: the settled coincidences
//!
//! A shell every witness of which reads `OnBoundary` may lie wholly ON
//! the other operand's boundary: one body twice, or one operand's shell
//! carried unchanged into the other. No point of such a shell is off
//! that boundary, so the ladder cannot name it; the coincidence ladder
//! can. This answer reads reduction records — the face pairs the
//! reduction SETTLED one carrier (`BooleanReduction`'s `coincident`:
//! shared recipe source, or a verified declaration) — and never values,
//! nor how many witnesses read `OnBoundary`. The shell is `On` a shell
//! of the other operand when every face of each is in a settled pair
//! with a face of the other, the pairs agree on orientation
//! ([`on_verdict`]), and the partner reads `On` back ([`check_mutual`]).
//! The two then hold one surface, and [`on_kept`] keeps or drops each
//! copy by Eq. 15.3, the rule for a coincident face pair. Where the
//! pairs certify less — a face with no settled pair, mixed
//! orientations, no partner covered back — the boolean refuses
//! [`BooleanError::CoincidentShell`].
//!
//! The same shell is also what reaches `On` when its faces off the
//! other boundary are all curved, since tier 3 reads planar faces only.
//! Such a face has no settled pair unless the recipe or a declaration
//! made it one carrier with a face of the other, so it refuses typed.

use geom_core::{Band, Decide, Point3, Tol, Vec3};
use slotmap::SecondaryMap;
use std::collections::BTreeSet;

use super::solid_contain::{
    PointInSolidError, SolidContainment, face_plane, point_in_face, point_in_solid,
};
use super::{
    BooleanError, BooleanOp, CarrierRelation, ContactRecords, Operand, SettledPair, SideCode,
};
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

/// What an uncut shell of one operand is to the other operand: on one
/// side of its boundary, or `On` it (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ShellVerdict {
    /// The ladder's first decisive witness: `In` or `Out`.
    Side(SideCode),
    /// The shell lies on `partner`, a shell of the other operand, and
    /// `partner` lies on it.
    On {
        /// The other operand's shell, in the other body's keys.
        partner: ShellKey,
        /// `SameOriented` or `SameOpposite`, read off the settled pairs
        /// between the two shells.
        relation: CarrierRelation,
    },
}

/// What a shell's settled coincidence pairs say about its orientation
/// against the other operand, where they do not certify it `On`
/// ([`BooleanError::CoincidentShell`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellOrientation {
    /// Every pair is aligned, but the faces paired are not one shell
    /// of the other operand lying back on this one.
    Same,
    /// Every pair is opposed, with the same shortfall.
    Opposite,
    /// Some pairs are aligned and some opposed.
    Mixed,
    /// `face` has no settled pair, so no orientation is read.
    Unpaired {
        /// The shell's first face, in arena order of the shell's face
        /// list, that no settled pair names.
        face: FaceKey,
    },
}

/// The verdict on the uncut `shell` of `body` against `other`: the
/// ladder (module docs) over the shell's faces, then, where every
/// witness read `OnBoundary`, the `On` question ([`on_verdict`]).
///
/// # Errors
///
/// [`BooleanError::Containment`] when a probe refuses other than
/// in-band; [`BooleanError::ShellWitnessExhausted`] when no witness
/// decides and one read in-band; [`BooleanError::CoincidentShell`]
/// when every witness lies on the other boundary and the settled pairs
/// do not certify `On`; [`BooleanError::JoinDesync`] when the shell
/// does not walk.
pub(super) fn shell_verdict<T: Decide>(
    (body, shell, operand): (&Body<T>, ShellKey, Operand),
    other: &Body<T>,
    coincident: &[SettledPair],
    band: Band,
    tol: Tol,
) -> Result<ShellVerdict, BooleanError> {
    let faces = &body
        .get_shell(shell)
        .ok_or(BooleanError::JoinDesync {
            what: "uncut shell no longer resolves",
        })?
        .faces;
    match complex_side(body, faces, other, band, tol)? {
        Reading::Side(s) => Ok(ShellVerdict::Side(s)),
        Reading::Undecided(t) if t.in_band == 0 => {
            on_verdict((shell, operand), faces, other, coincident)
        }
        Reading::Undecided(t) => Err(BooleanError::ShellWitnessExhausted {
            operand,
            shell,
            on_boundary: t.on_boundary,
            in_band: t.in_band,
            first_in_band: t.first_in_band,
        }),
    }
}

/// **The `On` question, this shell's half** (module docs): is every
/// face of `shell` in a settled coincidence pair with a face of exactly
/// one shell of `other`, at one orientation? The other half — that
/// shell answering the same of this one — is [`check_mutual`]'s.
///
/// Coverage, not counts: a face may pair with several faces of the
/// partner, and the partner's faces with several of this shell's.
fn on_verdict<T: Decide>(
    (shell, operand): (ShellKey, Operand),
    faces: &[FaceKey],
    other: &Body<T>,
    coincident: &[SettledPair],
) -> Result<ShellVerdict, BooleanError> {
    let refuse = |orientation| BooleanError::CoincidentShell {
        operand,
        shell,
        orientation,
    };
    let mine: BTreeSet<FaceKey> = faces.iter().copied().collect();
    // (this shell's face, the paired face's shell, the relation)
    let mut pairs: Vec<(FaceKey, ShellKey, CarrierRelation)> = Vec::new();
    for p in coincident {
        let (f, g) = match operand {
            Operand::A => (p.a, p.b),
            Operand::B => (p.b, p.a),
        };
        if mine.contains(&f) {
            let theirs = other
                .get_face(g)
                .map(|f| f.shell)
                .ok_or(BooleanError::JoinDesync {
                    what: "a settled pair names a face the other operand lacks",
                })?;
            pairs.push((f, theirs, p.relation));
        }
    }
    if let Some(&face) = faces.iter().find(|&&f| !pairs.iter().any(|q| q.0 == f)) {
        return Err(refuse(ShellOrientation::Unpaired { face }));
    }
    let orientation = |within: Option<ShellKey>| {
        let mut read = pairs
            .iter()
            .filter(|q| within.is_none_or(|s| q.1 == s))
            .map(|q| q.2);
        match read.next() {
            Some(CarrierRelation::SameOpposite)
                if read.all(|x| x == CarrierRelation::SameOpposite) =>
            {
                ShellOrientation::Opposite
            }
            Some(CarrierRelation::SameOriented)
                if read.all(|x| x == CarrierRelation::SameOriented) =>
            {
                ShellOrientation::Same
            }
            _ => ShellOrientation::Mixed,
        }
    };
    let mut partners: Vec<ShellKey> = pairs.iter().map(|q| q.1).collect();
    partners.sort();
    partners.dedup();
    partners.retain(|&s| {
        mine.iter()
            .all(|&f| pairs.iter().any(|q| q.0 == f && q.1 == s))
    });
    let [partner] = partners[..] else {
        return Err(refuse(orientation(None)));
    };
    match orientation(Some(partner)) {
        ShellOrientation::Same => Ok(ShellVerdict::On {
            partner,
            relation: CarrierRelation::SameOriented,
        }),
        ShellOrientation::Opposite => Ok(ShellVerdict::On {
            partner,
            relation: CarrierRelation::SameOpposite,
        }),
        o => Err(refuse(o)),
    }
}

/// **The keep rule for an `On` shell**, per operand: Eq. 15.3's lump
/// for a coincident face pair, kept where it is the side `op` keeps
/// ([`super::tables::kept_copy`]'s rule, cell for cell). Aligned, ∪
/// and ∩ keep A's copy and ∖ neither; opposed, ∪ and ∩ keep neither
/// and ∖ keeps A's.
pub(super) fn on_kept(op: BooleanOp, operand: Operand, relation: CarrierRelation) -> bool {
    super::tables::eq15_3_lump(op, operand, relation) == super::finish::kept_side(op, operand)
}

/// The shells of `verdicts` that `op` keeps for `operand`, in order.
pub(super) fn kept_shells(op: BooleanOp, operand: Operand, verdicts: &[Verdict]) -> Vec<ShellKey> {
    verdicts
        .iter()
        .filter(|(_, v)| match *v {
            ShellVerdict::Side(s) => s == super::finish::kept_side(op, operand),
            ShellVerdict::On { relation, .. } => on_kept(op, operand, relation),
        })
        .map(|(k, _)| *k)
        .collect()
}

/// One shell with its verdict.
pub(super) type Verdict = (ShellKey, ShellVerdict);

/// **`On` is mutual**: each `On` shell's partner reads `On` back, naming
/// it, at the same orientation. `verdicts` holds each operand's working
/// body and its shells' verdicts, A then B; `pristine` the operands at
/// rest, whose keys a partner is named in. A working shell's faces are
/// its pristine faces (it is uncut), so a face crosses between the two.
///
/// # Errors
///
/// [`BooleanError::CoincidentShell`] for the first `On` shell whose
/// partner does not read it back; [`BooleanError::JoinDesync`] for a
/// face that resolves in one key space and not the other.
pub(super) fn check_mutual<T: Decide>(
    verdicts: [(&Body<T>, &[Verdict]); 2],
    pristine: [&Body<T>; 2],
) -> Result<(), BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let first_face = |body: &Body<T>, s: ShellKey| {
        body.get_shell(s)
            .and_then(|sh| sh.faces.first().copied())
            .ok_or(desync("an On shell has no face"))
    };
    for (i, operand) in [(0, Operand::A), (1, Operand::B)] {
        let j = 1 - i;
        let (working, mine) = verdicts[i];
        let (their_working, theirs) = verdicts[j];
        for &(shell, verdict) in mine {
            let ShellVerdict::On { partner, relation } = verdict else {
                continue;
            };
            let at_rest = pristine[i]
                .get_face(first_face(working, shell)?)
                .map(|f| f.shell)
                .ok_or(desync("an On shell's face is not its operand's at rest"))?;
            let t = their_working
                .get_face(first_face(pristine[j], partner)?)
                .map(|f| f.shell)
                .ok_or(desync("an On partner's face left its working copy"))?;
            let back = theirs.iter().find(|(s, _)| *s == t).map(|(_, v)| *v);
            if back
                != Some(ShellVerdict::On {
                    partner: at_rest,
                    relation,
                })
            {
                return Err(BooleanError::CoincidentShell {
                    operand,
                    shell,
                    orientation: match relation {
                        CarrierRelation::SameOpposite => ShellOrientation::Opposite,
                        _ => ShellOrientation::Same,
                    },
                });
            }
        }
    }
    Ok(())
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
                    body.half_edge_start_point(he)
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{ShellOrientation, ShellVerdict, on_verdict};
    use crate::boolean::{BooleanError, BooleanResult, CarrierRelation, Operand, SettledPair};
    use crate::entity::{FaceKey, ShellKey};
    use crate::test_support_fixtures::brick;
    use crate::{Body, union};
    use geom_core::Tol;

    /// **A partner covers the whole shell, not part of it.** A unit
    /// block S against a body of two disjoint blocks P1 and P2, with a
    /// settled-pair list handed in: five of S's faces pair into P1, the
    /// sixth only into P2, and every face of P1 pairs back into S. P1
    /// would read `On` back, so only the coverage filter in
    /// [`on_verdict`] keeps S from being called `On` P1 with a face
    /// lying on P2. The reduction is not known to produce such a list
    /// (an unsettled face lying on P1's twin refuses first), so the list
    /// is built here rather than reached.
    #[test]
    fn a_shell_split_across_two_partners_is_not_on_either() {
        let tol = Tol::witness();
        let unit = (0.0, 1.0);
        let s: Body<f64> = brick(unit, unit, unit, tol);
        let p1: Body<f64> = brick(unit, unit, unit, tol);
        let p2: Body<f64> = brick((3.0, 4.0), unit, unit, tol);
        let BooleanResult::Body(other) = union(&p1, &p2, tol).unwrap() else {
            panic!("two blocks are not empty");
        };
        let other = other.body;
        let shells: Vec<(ShellKey, Vec<FaceKey>)> = other
            .shells()
            .map(|(k, sh)| (k, sh.faces.clone()))
            .collect();
        assert_eq!(shells.len(), 2, "the other body holds P1 and P2");
        let (shell, s_faces) = s
            .shells()
            .map(|(k, sh)| (k, sh.faces.clone()))
            .next()
            .unwrap();
        let (one, two) = (&shells[0].1, &shells[1].1);
        let same = CarrierRelation::SameOriented;
        let mut coincident: Vec<SettledPair> = (0..5)
            .map(|i| SettledPair {
                a: s_faces[i],
                b: one[i],
                relation: same,
            })
            .collect();
        coincident.push(SettledPair {
            a: s_faces[5],
            b: two[0],
            relation: same,
        });
        coincident.push(SettledPair {
            a: s_faces[0],
            b: one[5],
            relation: same,
        });
        match on_verdict((shell, Operand::A), &s_faces, &other, &coincident) {
            Err(BooleanError::CoincidentShell {
                orientation: ShellOrientation::Same,
                ..
            }) => {}
            Ok(ShellVerdict::On { partner, .. }) => {
                panic!("S read On {partner:?} with a face paired only into the other shell")
            }
            other => panic!("expected CoincidentShell(Same), got {other:?}"),
        }
    }
}
