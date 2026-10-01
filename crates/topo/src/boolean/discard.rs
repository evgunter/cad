//! **The discarded faces of a boolean** ([`DiscardRow`],
//! `BooleanNaming::discards`): every face an operand loses to the
//! result, with the kept faces it bordered.
//!
//! The naming layer names a face the result holds as several pieces by
//! the walls between them (N2's `Borders`), and those walls stand where
//! a part of the operand face was discarded. That part is gone from the
//! result, and the carve that drops it drops its edges' split lineage
//! with it, so the fact is recorded here, while the operand still holds
//! it. A row reads nothing but topology: which faces a discarded face
//! meets across which edges.
//!
//! Every path that splits an operand face records its discards: the
//! section path (`finish`) and the declared-REST union (`rest`). The
//! paths that keep or drop whole operands split no face, so no face of
//! theirs has pieces for a discard to lie between, and they record
//! none.

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, VertexKey};
use crate::provenance::Provenance;

use super::{BooleanError, Operand};

/// One face a boolean discarded (`BooleanNaming::discards`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardRow {
    /// The operand the face belonged to.
    pub operand: Operand,
    /// The face, in its operand's CLONE keys: chase
    /// `face_fragments_a`/`face_fragments_b` for the operand face it is
    /// a fragment of.
    pub face: FaceKey,
    /// Each stretch along which it bordered a face the result KEPT:
    /// the kept side's two end vertices, in result keys before the
    /// zip's fusions (settle them through `vertex_merges`). The result
    /// edge between the two settled ends is where the kept face meets
    /// the region this face held; a stretch whose ends fuse, or that no
    /// live edge joins, merged away with the faces beside it.
    pub bordered: Vec<(VertexKey, VertexKey)>,
    /// Each other boundary edge with its split ancestry (the edge,
    /// then each edge it was split from), in the same clone keys as
    /// `face`. The discard deletes these edges' lineage from the
    /// result, so a later reader that meets an ancestor here can still
    /// tell that the edge bounded this face.
    pub boundary_chains: Vec<Vec<EdgeKey>>,
}

/// The row for discarded face `face` of `body`, an operand clone.
///
/// `kept_across(mate)` says whether the face across an edge is one the
/// result keeps beside it (the stretch is then `bordered`); `kept_ends`
/// maps that stretch's two ends, in `body`'s keys, to the kept side's
/// ends in result keys.
pub(super) fn discard_row<T: geom_core::Real>(
    body: &Body<T>,
    face: FaceKey,
    operand: Operand,
    kept_across: &dyn Fn(FaceKey) -> bool,
    kept_ends: &dyn Fn(VertexKey, VertexKey) -> Result<(VertexKey, VertexKey), BooleanError>,
) -> Result<DiscardRow, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let fd = body
        .get_face(face)
        .ok_or_else(|| desync("a discarded face no longer resolves"))?;
    let mut row = DiscardRow {
        operand,
        face,
        bordered: Vec::new(),
        boundary_chains: Vec::new(),
    };
    for l in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body
            .get_loop(l)
            .ok_or_else(|| desync("a discarded face's loop no longer resolves"))?
            .boundary
        else {
            continue;
        };
        for he in body
            .loop_cycle(first)
            .ok_or_else(|| desync("a discarded face's loop is not walkable"))?
        {
            let h = body
                .get_half_edge(he)
                .ok_or_else(|| desync("a discarded face's half-edge no longer resolves"))?;
            let mate_face = body
                .mate(he)
                .and_then(|m| body.face_of_half_edge(m))
                .ok_or_else(|| desync("a discarded face's edge has no face across it"))?;
            if kept_across(mate_face) {
                let end = body
                    .half_edge_end(he)
                    .ok_or_else(|| desync("a discarded face's half-edge has no end"))?;
                row.bordered.push(kept_ends(h.start, end)?);
            } else {
                let mut chain = vec![h.edge];
                let mut at = h.edge;
                while let Some(Provenance::SplitEdge { edge: up }) = body.edge_provenance_of(at) {
                    if chain.contains(up) {
                        return Err(desync("a discarded edge's split lineage is cyclic"));
                    }
                    at = *up;
                    chain.push(at);
                }
                row.boundary_chains.push(chain);
            }
        }
    }
    Ok(row)
}
