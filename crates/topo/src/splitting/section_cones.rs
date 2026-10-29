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
//! On each side, after the join and before the side's merges and joins,
//! every section loop that passes a point at corners of two vertices is
//! read through check 9's own reading
//! ([`crate::validate::loop_point_groups`],
//! [`crate::validate::point_corners`], the wedge about
//! [`crate::validate::corner_normal`]). Where corner `i` holds another
//! side of the loop at the point, and the wedge from `i`'s leaving side
//! round to the arriving side of corner `j`, at another vertex, holds
//! none, the two vertices fuse by a zero-length chord across the face
//! from `i` to `j` ([`crate::fuse::fuse`]). The chord swaps the two
//! corners' arriving sides, so `i` then runs to `j`'s and `j` to `i`'s;
//! it divides the face, and the face it makes is a section face of the
//! side. Each fusion leaves one vertex fewer at the point, so the
//! reading ends.
//!
//! A corner fused this way is the one corner its leaving side has in
//! the section (the wedge to the next arriving side round the point,
//! holding nothing), so the corners a point ends with, and the vertices
//! they gather, do not depend on the order the fusions run in. Which of
//! two fused keys survives does: the one standing for the smaller
//! operand vertex ([`SplitNaming::original_of`]). Nothing reads that key alone: each fusion is a row of
//! [`SplitNaming::vertex_fusions`], and the split's lineage reads a
//! dead vertex as the vertex that survives it, so a record or a name
//! citing either cone lands on the one vertex.

use geom_core::{Band, Decide, Tol};

use super::PlaneSide;
use super::finish::{SplitFinishError, SplitNaming};
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, VertexKey};
use crate::fuse::{Joint, fuse};
use crate::validate::{
    CornerSide, PointCorner, WedgeNames, corner_normal, loop_point_groups, point_corners,
    wedge_holds_named,
};

/// The split's cone-corner names ([`wedge_holds_named`]).
const CONE_CORNER: WedgeNames = WedgeNames {
    after_leaving: "split_cone_corner_after_leaving",
    before_arriving: "split_cone_corner_before_arriving",
    turn: "split_cone_corner_turn",
    straight: "split_cone_corner_straight",
    straddle: "split_cone_corner_straddle",
};

/// A fusion read: the leaving half-edges of two corners, at two
/// vertices of one point, of a loop of `face`.
struct Fusion {
    face: FaceKey,
    from: HalfEdgeKey,
    to: HalfEdgeKey,
}

/// Fuses `side`'s cones whose section corners cross (module docs).
///
/// # Errors
///
/// - [`SplitFinishError::ConeCornerEscalated`] where a reading the
///   fusion needed lands in the band;
/// - [`SplitFinishError::ConeCornerUnread`] where a corner at a point of
///   two vertices has a side with no departure read, or the face no
///   normal there;
/// - [`SplitFinishError::ConeFusionDesync`] or
///   [`SplitFinishError::Euler`] where the fusion refuses;
/// - [`SplitFinishError::Corrupt`] where a key the reading just walked no
///   longer resolves.
pub(super) fn fuse_section_cones<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    side: PlaneSide,
    naming: &mut SplitNaming,
    tol: Tol,
) -> Result<(), SplitFinishError> {
    let band = Band::linear(tol).map_err(SplitFinishError::Band)?;
    while let Some(Fusion { face, from, to }) = next_fusion(body, side, naming, band)? {
        let start = |he| body.get_half_edge(he).map(|h| h.start);
        let (a, b) = (
            start(from).ok_or(SplitFinishError::Corrupt)?,
            start(to).ok_or(SplitFinishError::Corrupt)?,
        );
        let keep = if naming.original_of(a) <= naming.original_of(b) {
            a
        } else {
            b
        };
        let p = body
            .get_vertex(keep)
            .and_then(|v| body.points.get(v.point).copied())
            .ok_or(SplitFinishError::Corrupt)?;
        let fused = fuse(
            body,
            Joint::Chord { he1: from, he2: to },
            (p, keep),
            |what| SplitFinishError::ConeFusionDesync { what },
            tol,
        )?;
        naming
            .vertex_fusions
            .push((fused.dead, fused.kept))
            .map_err(|_| SplitFinishError::Corrupt)?;
        if let Some(made) = fused.divided {
            naming.sections.push((made, side));
            naming
                .face_fragments
                .push((made, face))
                .map_err(|_| SplitFinishError::Corrupt)?;
        }
    }
    Ok(())
}

/// The first fusion `side`'s section faces call for, in section order
/// and check 9's order.
fn next_fusion<T: Decide>(
    body: &Body<T>,
    side: PlaneSide,
    naming: &SplitNaming,
    band: Band,
) -> Result<Option<Fusion>, SplitFinishError> {
    for &(face, s) in &naming.sections {
        if s != side {
            continue;
        }
        let Some(f) = body.get_face(face) else {
            continue;
        };
        for (cycle, at) in loop_point_groups(body, f) {
            let vertex = |i: usize| body.get_half_edge(cycle[i]).map(|h| h.start);
            let vertices = at
                .iter()
                .map(|&i| vertex(i).ok_or(SplitFinishError::Corrupt))
                .collect::<Result<Vec<_>, _>>()?;
            if vertices.iter().all(|&v| v == vertices[0]) {
                continue;
            }
            let corners = point_corners(body, &cycle, &at);
            if corners.len() < at.len() {
                return Err(SplitFinishError::ConeCornerUnread {
                    vertex: vertices[0],
                });
            }
            if let Some((i, j)) = fusion_at(body, face, &corners, band)? {
                return Ok(Some(Fusion {
                    face,
                    from: corners[i].1[0].0,
                    to: corners[j].1[0].0,
                }));
            }
        }
    }
    Ok(None)
}

/// The first `(i, j)`, corners at two vertices, where `i`'s wedge holds
/// another side and the wedge from `i`'s leaving side to `j`'s arriving
/// side holds none.
fn fusion_at<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    corners: &[PointCorner<T>],
    band: Band,
) -> Result<Option<(usize, usize)>, SplitFinishError> {
    let first = corners[0].0;
    let n = corner_normal(body, face, first, band)
        .map_err(|source| SplitFinishError::ConeCornerEscalated {
            vertex: first,
            source,
        })?
        .ok_or(SplitFinishError::ConeCornerUnread { vertex: first })?;
    // Whether the wedge from `leaving` to `arriving` holds a side of the
    // point other than those two.
    let holds = |vertex: VertexKey,
                 leaving: &CornerSide<T>,
                 arriving: &CornerSide<T>|
     -> Result<bool, SplitFinishError> {
        let unread = || SplitFinishError::ConeCornerUnread { vertex };
        let (a, b) = (
            leaving.2.ok_or_else(unread)?,
            arriving.2.ok_or_else(unread)?,
        );
        for (_, sides) in corners {
            for d in sides {
                if d.0 == leaving.0 || d.0 == arriving.0 {
                    continue;
                }
                let dep = d.2.ok_or_else(unread)?;
                if wedge_holds_named(&CONE_CORNER, n, a, b, dep, band)
                    .map_err(|source| SplitFinishError::ConeCornerEscalated { vertex, source })?
                {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    };
    for (i, (vi, [leaving, arriving])) in corners.iter().enumerate() {
        if !holds(*vi, leaving, arriving)? {
            continue;
        }
        for (j, (vj, [_, other])) in corners.iter().enumerate() {
            if vj != vi && !holds(*vi, leaving, other)? {
                return Ok(Some((i, j)));
            }
        }
    }
    Ok(None)
}
