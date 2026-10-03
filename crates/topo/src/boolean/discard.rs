//! **The discarded faces of a boolean** ([`DiscardRow`],
//! `BooleanNaming::discards`): every face an operand loses to the
//! result, with the kept faces it bordered.
//!
//! The naming layer names a face the result holds as several pieces by
//! the walls between them (N2's `Borders`), and those walls stand where
//! a part of the operand face was discarded. That part is gone from the
//! result, and the carve that drops it drops its edges' split lineage
//! with it, so the fact is recorded here, while the operand still holds
//! it. A row reads topology — which faces a discarded face meets across
//! which edges — and one fact the classification decided: where a kept
//! face holding part of its region through a coincident copy runs into
//! it ([`HeldEdge`]).
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
    /// Each stretch where a kept face holding part of this face's region
    /// through a coincident copy (`BooleanNaming::covered`) runs into
    /// it, so that the region this face lost meets the kept face holding
    /// the rest: that face's edge, by its two ends in result keys before
    /// the zip's fusions, as `bordered` ([`HeldEdge`]).
    pub held: Vec<(VertexKey, VertexKey)>,
}

/// An edge of the kept face of a covered pair that runs into the
/// dropped face (`BooleanReduction::held`). It is read at a vertex both
/// operands hold, where the classification finds the two faces'
/// sectors coincident and keeps one copy: the edge bounds the kept
/// copy's sector and lies inside the dropped copy's, along neither of
/// its edges, and the face across it does not cover the dropped face
/// too.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeldEdge {
    /// The operand of the kept copy, whose clone holds `edge`.
    pub holder: Operand,
    /// The edge, in `holder`'s clone keys.
    pub edge: EdgeKey,
    /// The dropped face it runs into, in the other operand's clone keys
    /// as the classification read them.
    pub face: FaceKey,
    /// The other operand's vertex where the edge enters `face`.
    pub at: VertexKey,
}

/// The end of the face lineage that starts at `face` and steps through
/// `up`, the boolean's one bounded lineage walk: `up` names the next
/// face of a row, `None` where none does, and `rows` counts the rows
/// it reads. Its readers: fragment rows back to the face they were
/// divided from (`BooleanNaming::face_fragments_a`/`_b`,
/// `SplitNaming::face_fragments`), and a merge's absorption rows
/// forward to the surviving face.
///
/// `None` when the rows cycle: a walk that outlasts `rows` steps has
/// revisited a face, so the record is corrupt, and a reader refuses it
/// rather than treating the face it stopped on as the end.
pub fn lineage_root(
    face: FaceKey,
    rows: usize,
    up: impl Fn(FaceKey) -> Option<FaceKey>,
) -> Option<FaceKey> {
    let mut at = face;
    for _ in 0..=rows {
        match up(at) {
            Some(next) => at = next,
            None => return Some(at),
        }
    }
    None
}

/// The held edges one operand's discarded faces may border
/// ([`DiscardRow::held`]).
pub(super) struct HeldInto<'a, T: geom_core::Real> {
    /// Every held edge of the boolean.
    pub entries: &'a [HeldEdge],
    /// The discarded operand's chord-split rows, `(new, divided-from)`.
    pub fragments: &'a [(FaceKey, FaceKey)],
    /// The other operand, whose clone holds the edges.
    pub holder_op: Operand,
    /// Its clone.
    pub holder: &'a Body<T>,
    /// A vertex of `holder` in result keys.
    pub to_result: &'a dyn Fn(VertexKey) -> Option<VertexKey>,
    /// The discarded operand's null-edge copies of a vertex
    /// (`BooleanReduction::null_edges`), which the split may leave on
    /// a fragment in place of the vertex a held edge entered at.
    pub copies: &'a super::NullCopies,
}

impl<T: geom_core::Real> HeldInto<'_, T> {
    /// The held stretches of discarded `face`, whose boundary holds
    /// `boundary`: each held edge into a face it is a fragment of,
    /// entering at one of its vertices or a null-edge copy of one.
    fn stretches(
        &self,
        face: FaceKey,
        boundary: &std::collections::BTreeSet<VertexKey>,
    ) -> Result<Vec<(VertexKey, VertexKey)>, BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        let root = |f| {
            lineage_root(f, self.fragments.len(), |k| {
                self.fragments
                    .iter()
                    .find(|(new, _)| *new == k)
                    .map(|&(_, up)| up)
            })
            .ok_or_else(|| desync("a face's fragment lineage is cyclic"))
        };
        let own = root(face)?;
        let mut out = Vec::new();
        let enters = |at: VertexKey| self.copies.of(at).iter().any(|v| boundary.contains(v));
        for h in self.entries {
            if h.holder != self.holder_op || !enters(h.at) || root(h.face)? != own {
                continue;
            }
            let edge = self
                .holder
                .get_edge(h.edge)
                .ok_or_else(|| desync("a held edge no longer resolves"))?;
            let end = |he| {
                self.holder
                    .get_half_edge(he)
                    .and_then(|h| (self.to_result)(h.start))
                    .ok_or_else(|| desync("a held edge's end is missing from the result"))
            };
            out.push((end(edge.he_plus)?, end(edge.he_minus)?));
        }
        out.sort();
        out.dedup();
        Ok(out)
    }
}

/// The row for discarded face `face` of `body`, an operand clone.
///
/// `kept_across(mate)` says whether the face across an edge is one the
/// result keeps beside it (the stretch is then `bordered`); `kept_ends`
/// maps that stretch's two ends, in `body`'s keys, to the kept side's
/// ends in result keys, given the face across it. `held` gives the row
/// its held stretches; a path that holds no covered pair passes `None`.
pub(super) fn discard_row<T: geom_core::Real>(
    body: &Body<T>,
    face: FaceKey,
    operand: Operand,
    kept_across: &dyn Fn(FaceKey) -> bool,
    kept_ends: &dyn Fn(
        FaceKey,
        VertexKey,
        VertexKey,
    ) -> Result<(VertexKey, VertexKey), BooleanError>,
    held: Option<&HeldInto<'_, T>>,
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
        held: Vec::new(),
    };
    let mut boundary = std::collections::BTreeSet::new();
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
            boundary.insert(h.start);
            let mate_face = body
                .mate(he)
                .and_then(|m| body.face_of_half_edge(m))
                .ok_or_else(|| desync("a discarded face's edge has no face across it"))?;
            if kept_across(mate_face) {
                let end = body
                    .half_edge_end(he)
                    .ok_or_else(|| desync("a discarded face's half-edge has no end"))?;
                row.bordered.push(kept_ends(mate_face, h.start, end)?);
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
    if let Some(held) = held {
        row.held = held.stretches(face, &boundary)?;
    }
    Ok(row)
}
