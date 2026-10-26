//! **The discarded faces of a boolean** ([`DiscardRow`],
//! `BooleanNaming::discards`): every face an operand loses to the
//! result, with the kept faces it bordered, except a dropped coincident
//! face a containment fallback could not divide
//! (`BooleanNaming::uncut`).
//!
//! The naming layer names a face the result holds as several pieces by
//! the walls between them (N2's `Borders`), and those walls stand where
//! a part of the operand face was discarded. That part is gone from the
//! result, and the carve that drops it drops its edges' split lineage
//! with it, so the fact is recorded here, while the operand still holds
//! it. A row reads topology: which faces a discarded face meets across
//! which edges, a one-sided chord along a kept edge among them
//! ([`ChordsAcross`]).
//!
//! Both paths record them: the section path (`finish`), and the
//! containment fallback, which keeps or drops whole shells and records
//! every face of a dropped one, except those it leaves uncut, once it
//! has cut its dropped coincident faces (`ops::fallback_fates`). Such a
//! face borders no kept face of its own operand, so its stretches are
//! read through substituted fragments alone.

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, VertexKey};
use crate::provenance::Provenance;

use super::{BooleanError, Operand};

/// One face a boolean discarded (`BooleanNaming::discards`), or a face
/// it divided along an edge lying in it, whose lost region is that line
/// (`boolean::slit`).
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
    /// zip's splits per cone, its fusions, the output stage's join and
    /// merge, and the partings' joined ends (settle them through
    /// `BooleanNaming::settler`; an end
    /// the zip split per cone lies at one of its copies,
    /// `Settler::settle_cones`). The result edge between the two
    /// settled ends, or the joined edge holding one of them
    /// (`BooleanNaming::stretch_through_joins`), is where the kept face
    /// meets the region this face held; a stretch whose ends fuse, or
    /// lie inside a merged face, or that no live edge joins, merged away
    /// with the faces beside it.
    pub bordered: Vec<(VertexKey, VertexKey)>,
    /// Each stretch along which it bordered a kept face THROUGH a
    /// substituted fragment ([`super::SubstitutedFace`]): across a
    /// one-sided chord the boolean cut it along, or across its own edge
    /// where the fragment beside it is substituted. The kept face holding
    /// the fragment meets this face's region there, along its own edge,
    /// which is no seam: the stretch is the kept edge's two ends, or this
    /// face's edge's, in result keys, read as [`Self::bordered`]'s are.
    pub bordered_through: Vec<(VertexKey, VertexKey)>,
    /// Each other boundary edge with its split ancestry (the edge,
    /// then each edge it was split from), in the same clone keys as
    /// `face`. The discard deletes these edges' lineage from the
    /// result, so a later reader that meets an ancestor here can still
    /// tell that the edge bounded this face.
    ///
    /// Each chain is a copy of the clone's `SplitEdge` records read up
    /// to its root, so two discards that share an ancestor each carry
    /// its hop, with the one parent the record names
    /// ([`crate::provenance::Provenance::SplitEdge`] says why a copy is
    /// as sound as the records).
    pub boundary_chains: Vec<Vec<EdgeKey>>,
}

/// The substituted fragments a dropped coincident face was cut into
/// ([`super::fate`]): where a discarded face meets one across a one-sided
/// chord, it borders the kept face holding that fragment along the kept
/// edge the chord runs on; where it meets one across its own edge, it
/// borders that kept face along the edge.
pub(super) struct ChordsAcross<'a> {
    /// The cuts.
    pub cuts: &'a super::fate::FateCuts,
    /// The kept operand's vertex in result keys.
    pub to_result: &'a dyn Fn(VertexKey) -> Option<VertexKey>,
    /// The discarded face's operand's vertex in result keys: its own
    /// kept copy, else the kept operand's vertex it meets.
    pub own_to_result: &'a dyn Fn(VertexKey) -> Option<VertexKey>,
}

impl ChordsAcross<'_> {
    /// The kept edge's ends in result keys where `he` of `operand`'s
    /// clone is a chord with a substituted fragment across it, in the
    /// order of `he`'s start and end.
    fn bordered(
        &self,
        operand: Operand,
        edge: EdgeKey,
        start: VertexKey,
        across: FaceKey,
    ) -> Result<Option<(VertexKey, VertexKey)>, BooleanError> {
        let Some(c) = self.cuts.chord(operand, edge) else {
            return Ok(None);
        };
        if !self.cuts.is_substituted(operand, across) {
            return Ok(None);
        }
        let end = |v| {
            (self.to_result)(v).ok_or(BooleanError::JoinDesync {
                what: "a chord's kept edge end is missing from the result",
            })
        };
        let [k0, k1] = c.kept_ends;
        Ok(Some(if c.ends[0] == start {
            (end(k0)?, end(k1)?)
        } else {
            (end(k1)?, end(k0)?)
        }))
    }

    /// The edge's own ends `(start, end)` in result keys where `across`,
    /// the face beside it, is a substituted fragment of `operand`'s
    /// clone; `None` where it is not, or where an end meets no vertex of
    /// the result (it lies inside the kept face, which holds no vertex
    /// there to border along).
    fn substituted(
        &self,
        operand: Operand,
        (start, end): (VertexKey, VertexKey),
        across: FaceKey,
    ) -> Option<(VertexKey, VertexKey)> {
        if !self.cuts.is_substituted(operand, across) {
            return None;
        }
        (self.own_to_result)(start).zip((self.own_to_result)(end))
    }
}

/// An operand's vertex in result keys, where the result holds one.
pub(super) type InResult<'a> = &'a dyn Fn(VertexKey) -> Option<VertexKey>;

/// Vertex `v` of `operand`'s clone in result keys: the one copy of it
/// the result keeps (`kept`, read on each of its null-edge copies), or
/// where it keeps none, the one vertex of the other operand it meets
/// there (a vertex–vertex contact, read through `theirs`).
pub(super) fn own_in_result<T: geom_core::Real>(
    red: &super::BooleanReduction<T>,
    operand: Operand,
    (kept, theirs): (InResult<'_>, InResult<'_>),
    v: VertexKey,
) -> Option<VertexKey> {
    let one = |mut rs: Vec<VertexKey>| {
        rs.sort();
        rs.dedup();
        match rs.as_slice() {
            [r] => Some(*r),
            _ => None,
        }
    };
    let copies = super::NullCopies::of_operand(&red.null_edges, operand).of(v);
    let own: Vec<VertexKey> = copies.iter().filter_map(|&k| kept(k)).collect();
    if !own.is_empty() {
        return one(own);
    }
    let other = super::NullCopies::of_operand(&red.null_edges, operand.other());
    one(red
        .contacts
        .vv
        .iter()
        .filter_map(|c| {
            let (mine, o) = match operand {
                Operand::A => (c.a, c.b),
                Operand::B => (c.b, c.a),
            };
            copies.contains(&mine).then_some(o)
        })
        .flat_map(|o| other.of(o))
        .filter_map(theirs)
        .collect())
}

/// The row for discarded face `face` of `body`, an operand clone.
///
/// `kept_across(mate)` says whether the face across an edge is one the
/// result keeps beside it (the stretch is then `bordered`); `kept_ends`
/// maps that stretch's two ends, in `body`'s keys, to the kept side's
/// ends in result keys, given the half-edge across it. `chords` borders
/// the row along the kept edges its one-sided chords run on; a path
/// that cuts none passes `None`.
pub(super) fn discard_row<T: geom_core::Real>(
    body: &Body<T>,
    face: FaceKey,
    operand: Operand,
    kept_across: &dyn Fn(FaceKey) -> bool,
    kept_ends: &dyn Fn(
        HalfEdgeKey,
        VertexKey,
        VertexKey,
    ) -> Result<(VertexKey, VertexKey), BooleanError>,
    chords: Option<&ChordsAcross<'_>>,
) -> Result<DiscardRow, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let fd = body
        .get_face(face)
        .ok_or_else(|| desync("a discarded face no longer resolves"))?;
    let mut row = DiscardRow {
        operand,
        face,
        bordered: Vec::new(),
        bordered_through: Vec::new(),
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
            let across = || desync("a discarded face's edge has no face across it");
            let mate = body.mate(he).ok_or_else(across)?;
            let mate_face = body.face_of_half_edge(mate).ok_or_else(across)?;
            let end = body
                .half_edge_end(he)
                .ok_or_else(|| desync("a discarded face's half-edge has no end"))?;
            let through = match chords {
                Some(c) => match c.bordered(operand, h.edge, h.start, mate_face)? {
                    Some(stretch) => Some(stretch),
                    None => c.substituted(operand, (h.start, end), mate_face),
                },
                None => None,
            };
            if let Some(stretch) = through {
                row.bordered_through.push(stretch);
            } else if kept_across(mate_face) {
                row.bordered.push(kept_ends(mate, h.start, end)?);
            } else {
                let mut chain = vec![h.edge];
                let mut at = h.edge;
                while let Some(Provenance::SplitEdge { edge: up }) = body.edge_provenance_of(at) {
                    if chain.contains(up) {
                        return Err(BooleanError::SplitLineage(crate::SplitLineageCycle {
                            edge: h.edge,
                        }));
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

/// **The split records an operand's carve takes with it**: each edge of
/// `src`, the operand clone, that `carved` no longer holds and that held
/// a `SplitEdge` record, with the edge it was split from, in the clone's
/// keys (`BooleanNaming::carved_lineage`). The carve removes a dropped
/// edge's birth record with the edge, so a kept piece split off it
/// leads to its parent only through this copy. A row's
/// [`DiscardRow::boundary_chains`] hold the edges of a discarded face
/// whose face across is dropped too; an edge the carve kills beside a
/// face the row counts as kept is in no chain, and is here.
pub(super) fn carved_lineage<T: geom_core::Real>(
    src: &Body<T>,
    carved: &Body<T>,
) -> std::collections::BTreeMap<EdgeKey, EdgeKey> {
    let mut lineage = std::collections::BTreeMap::new();
    super::edge_join::record_killed_lineage(
        &mut lineage,
        &super::edge_join::split_records(src),
        src.edges()
            .map(|(e, _)| e)
            .filter(|&e| carved.get_edge(e).is_none()),
    );
    lineage
}
