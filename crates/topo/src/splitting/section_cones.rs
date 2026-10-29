//! **A pinch's cones fuse on a side whose section pairs their germs.**
//!
//! The reduction reads each ON vertex alone ([`super::insert`]), so each
//! section corner it closes pairs two germs of one vertex. A pinch is
//! several vertices on one point key, one per cone (`zip`'s module
//! docs), and the plane can pass between the cones: a side's material
//! then joins them there, and each of the section's corners at the point
//! runs from a germ of one cone to a germ of another. Closed per cone,
//! each copy's corner is the complement of its own cone's gap and holds
//! the other's sides, a crossed corner (tier 3, check 9).
//!
//! On each side, a section loop that passes a point at corners of two
//! vertices is read as tier 3 reads it ([`crate::validate::side`],
//! [`crate::validate::wedge_holds_named`], about the section face's
//! outward normal). Where corner `i` holds another side of the loop at
//! the point, and the wedge from `i`'s leaving side round to the
//! arriving side of corner `j`, at another vertex, holds none, the two
//! vertices fuse by a zero-length chord across the face from `i` to
//! `j`, collapsed by a `kev` ([`crate::boolean::zip::fuse_dividing`],
//! the boolean's fusion). The chord swaps the two corners' arriving
//! sides: `i` then runs to `j`'s and `j` to `i`'s. The chord divides the
//! face, and the face it makes is a section face of the side. Each
//! fusion leaves one vertex fewer at the point, so the reading ends.
//!
//! The vertex kept is the one standing for the smaller operand vertex
//! (itself, or the original its null-pair copy was minted at), which
//! either orientation of the plane reads alike.

use std::collections::BTreeMap;

use geom_core::{Band, Decide, Tol, Vec3};

use super::PlaneSide;
use super::finish::{SplitFinishError, SplitNaming};
use super::section_loops::section_normal;
use crate::body::Body;
use crate::boolean::zip::{Joint, fuse_dividing};
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, VertexKey};
use crate::validate::{CornerSide, WedgeNames, mate, side as corner_side, wedge_holds_named};

/// The split's cone-corner names ([`wedge_holds_named`]).
const CONE_CORNER: WedgeNames = WedgeNames {
    after_leaving: "split_cone_corner_after_leaving",
    before_arriving: "split_cone_corner_before_arriving",
    turn: "split_cone_corner_turn",
    straight: "split_cone_corner_straight",
    straddle: "split_cone_corner_straddle",
};

/// One corner of a loop at a point: its vertex, and its leaving and
/// arriving sides as departures from the point.
struct Corner<T: geom_core::Real> {
    vertex: VertexKey,
    leaving: CornerSide<T>,
    arriving: CornerSide<T>,
}

/// A fusion read: the leaving half-edges of corners `i` and `j`, at two
/// vertices of one point.
struct Fusion {
    face: FaceKey,
    from: HalfEdgeKey,
    to: HalfEdgeKey,
}

/// Fuses `side`'s cones whose section corners cross (module docs).
///
/// # Errors
///
/// [`SplitFinishError::ConeCornerEscalated`] where a wedge reading the
/// fusion needed lands in the band; [`SplitFinishError::Euler`] where
/// the fusion's `mef` or `kev` refuses; [`SplitFinishError::Corrupt`]
/// where a key the reading just walked no longer resolves.
pub(super) fn fuse_section_cones<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    side: PlaneSide,
    naming: &mut SplitNaming,
    normal: Vec3<T>,
    tol: Tol,
) -> Result<(), SplitFinishError> {
    let band = Band::linear(tol).map_err(SplitFinishError::Band)?;
    let n = section_normal(normal, side);
    let originals: BTreeMap<VertexKey, VertexKey> = naming.vertex_pairs.iter().copied().collect();
    let stands_for = |v: VertexKey| originals.get(&v).copied().unwrap_or(v);
    while let Some(Fusion { face, from, to }) = next_fusion(body, side, naming, n, band)? {
        let start = |he| body.get_half_edge(he).map(|h| h.start);
        let (a, b) = (
            start(from).ok_or(SplitFinishError::Corrupt)?,
            start(to).ok_or(SplitFinishError::Corrupt)?,
        );
        let keep = if stands_for(a) <= stands_for(b) { a } else { b };
        let p = body
            .get_vertex(keep)
            .and_then(|v| body.points.get(v.point).copied())
            .ok_or(SplitFinishError::Corrupt)?;
        let (_, _, made) = fuse_dividing(
            body,
            Joint::Chord { he1: from, he2: to },
            (p, keep),
            |_| SplitFinishError::Corrupt,
            tol,
        )?;
        if let Some(made) = made {
            naming.sections.push((made, side));
            naming
                .face_fragments
                .push((made, face))
                .map_err(|_| SplitFinishError::Corrupt)?;
        }
    }
    Ok(())
}

/// The first fusion `side`'s section faces call for, in section order,
/// loop order and corner order.
fn next_fusion<T: Decide>(
    body: &Body<T>,
    side: PlaneSide,
    naming: &SplitNaming,
    n: Vec3<T>,
    band: Band,
) -> Result<Option<Fusion>, SplitFinishError> {
    for &(face, s) in &naming.sections {
        if s != side {
            continue;
        }
        let Some(f) = body.get_face(face) else {
            continue;
        };
        for l in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
            let Some(LoopBoundary::Cycle { first }) = body.get_loop(l).map(|l| l.boundary) else {
                continue;
            };
            let cycle = body.loop_cycle(first).ok_or(SplitFinishError::Corrupt)?;
            let mut keyed: Vec<_> = cycle
                .iter()
                .enumerate()
                .map(|(i, &he)| {
                    let v = body.get_half_edge(he).ok_or(SplitFinishError::Corrupt)?.start;
                    Ok((body.get_vertex(v).ok_or(SplitFinishError::Corrupt)?.point, i))
                })
                .collect::<Result<_, SplitFinishError>>()?;
            keyed.sort_unstable();
            for group in keyed.chunk_by(|x, y| x.0 == y.0) {
                if group.len() < 2 {
                    continue;
                }
                let Some(corners) = corners_at(body, &cycle, group) else {
                    continue;
                };
                if let Some((i, j)) = fusion_at(&corners, n, band)? {
                    return Ok(Some(Fusion {
                        face,
                        from: corners[i].leaving.0,
                        to: corners[j].leaving.0,
                    }));
                }
            }
        }
    }
    Ok(None)
}

/// The corners of `cycle` at the indices `group` carries, or `None`
/// where they sit at one vertex (nothing to fuse) or a side has no
/// departure read (a NURBS edge, tier 3's residue too).
fn corners_at<T: Decide>(
    body: &Body<T>,
    cycle: &[HalfEdgeKey],
    group: &[(crate::geometry::PointKey, usize)],
) -> Option<Vec<Corner<T>>> {
    let m = cycle.len();
    let corners = group
        .iter()
        .map(|&(_, i)| {
            let he = cycle[i];
            let leaving = corner_side(body, he)?;
            let arriving = corner_side(body, mate(body, cycle[(i + m - 1) % m])?)?;
            leaving.2?;
            arriving.2?;
            Some(Corner {
                vertex: body.get_half_edge(he)?.start,
                leaving,
                arriving,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    corners
        .iter()
        .any(|c| c.vertex != corners[0].vertex)
        .then_some(corners)
}

/// The first `(i, j)`, corners at two vertices, where `i`'s wedge holds
/// another side and the wedge from `i`'s leaving side to `j`'s arriving
/// side holds none.
fn fusion_at<T: Decide>(
    corners: &[Corner<T>],
    n: Vec3<T>,
    band: Band,
) -> Result<Option<(usize, usize)>, SplitFinishError> {
    // Whether the wedge from `leaving` to `arriving` holds a side of the
    // point other than those two.
    let holds = |vertex: VertexKey,
                 leaving: &CornerSide<T>,
                 arriving: &CornerSide<T>|
     -> Result<bool, SplitFinishError> {
        let (Some(a), Some(b)) = (leaving.2, arriving.2) else {
            return Ok(false);
        };
        for c in corners {
            for d in [&c.leaving, &c.arriving] {
                if d.0 == leaving.0 || d.0 == arriving.0 {
                    continue;
                }
                let Some(dep) = d.2 else { continue };
                if wedge_holds_named(&CONE_CORNER, n, a, b, dep, band).map_err(|source| {
                    SplitFinishError::ConeCornerEscalated { vertex, source }
                })? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    };
    for (i, ci) in corners.iter().enumerate() {
        if !holds(ci.vertex, &ci.leaving, &ci.arriving)? {
            continue;
        }
        for (j, cj) in corners.iter().enumerate() {
            if cj.vertex != ci.vertex && !holds(ci.vertex, &ci.leaving, &cj.arriving)? {
                return Ok(Some((i, j)));
            }
        }
    }
    Ok(None)
}
