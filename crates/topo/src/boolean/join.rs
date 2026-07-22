//! Boolean joining (ch. 15 §15.7, Programs 15.13/15.14): the ch. 14
//! join skeleton (PR 3's [`ChordJoiner`]) driven **in lockstep across
//! both solids**, with `scanjoin`'s dual requirement — a candidate
//! joins only where a topological neighbor exists in BOTH solids at
//! the SAME loose end — realized through PR 4's explicit
//! [`NullEdgePairRecord`](super::NullEdgePairRecord) correspondence
//! keys (F9: correspondence as data, never correlated array order;
//! `ssortnulledges` stays engineered out — pairs are processed in
//! record order, which is the deterministic discovery order).
//!
//! # Sense and roles are side attributes, never slots (the below-copy
//! # audit)
//!
//! Boolean runs mint copies of BOTH parities (In-runs mint
//! `NewVertexSide::Below` copies — the PR 4 interface fact), so
//! nothing here may assume "the minted copy is the above copy" as the
//! split lane could. Every decision reads the F9 attributes:
//!
//! - the **up half** of a null edge is the half *starting at
//!   `attr.below_end`* (the IN end) — resolved per edge from the
//!   recorded [`NullEdge`] attribute, for original vertices and minted
//!   copies alike;
//! - polygon-completion **roles** are resolved by membership of the
//!   IN-end/OUT-end vertex sets built from the same attributes
//!   (`in_copy` = the loop through IN ends), recorded as
//!   [`NullFacePair::Boolean`];
//! - the cross-solid **offer pairing** is the book's antiparallel
//!   crossover (A-he1 ↔ B-he2) as data: offers are
//!   `(a_up, b_down)` then `(a_down, b_up)` — the two solids'
//!   polygon copies run in opposite senses. Pinned by the two-brick
//!   trace; a wrong crossover fails loudly (`UnpairedLooseEnds`).
//!
//! # Lockstep discipline
//!
//! Joins, retirements, and completions must occur in BOTH solids
//! together (the correspondence invariant). Any divergence — one side
//! loose where the other is not, one polygon completing where the
//! other merges — is the typed
//! [`BooleanError::JoinDesync`] refusal, never a silent mis-join.
//! There is no geometric sort and no section-area certification here:
//! boolean intersection polygons are in general non-planar (§15.7);
//! degenerate results are netted at the component stage instead.

use geom_core::{Band, Decide};
use slotmap::SecondaryMap;

use super::{BooleanError, BooleanReduction, Operand};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopKey, VertexKey};
use crate::null::{NullEdge, NullFacePair};
use crate::splitting::join::{ChordJoiner, CutOutcome, SplitJoinError, loop_starts};

/// One completed section-polygon **pair**: the 2-loop null face in
/// each solid, with the loop roles as F9 data (IN copy = the loop
/// through the IN-side ends).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompletedPolygonPair {
    /// The A-clone null face.
    pub a_face: FaceKey,
    /// A's IN-copy loop.
    pub a_in_loop: LoopKey,
    /// A's OUT-copy loop.
    pub a_out_loop: LoopKey,
    /// The B-clone null face.
    pub b_face: FaceKey,
    /// B's IN-copy loop.
    pub b_in_loop: LoopKey,
    /// B's OUT-copy loop.
    pub b_out_loop: LoopKey,
}

/// Per-solid joining state: the shared chord core plus the F9 side
/// data every decision reads.
struct SolidJoin {
    joiner: ChordJoiner,
    /// IN-side end vertices (below ends), from the attributes.
    in_set: SecondaryMap<VertexKey, ()>,
    /// OUT-side end vertices (above ends).
    out_set: SecondaryMap<VertexKey, ()>,
    /// Null-edge attribute lookup.
    attrs: SecondaryMap<EdgeKey, NullEdge>,
}

impl SolidJoin {
    fn new<T: Decide>(red: &BooleanReduction<T>, operand: Operand, band: Band) -> Self {
        let mut in_set = SecondaryMap::new();
        let mut out_set = SecondaryMap::new();
        let mut attrs = SecondaryMap::new();
        for r in red.null_edges_of(operand) {
            in_set.insert(r.attr.below_end, ());
            out_set.insert(r.attr.above_end, ());
            attrs.insert(r.edge, r.attr);
        }
        Self {
            joiner: ChordJoiner::new(band),
            in_set,
            out_set,
            attrs,
        }
    }

    /// The two halves of a null edge in F9 data order: **up half**
    /// (starting at the IN end, `attr.below_end`) first.
    fn halves<T: Decide>(
        &self,
        body: &Body<T>,
        edge: EdgeKey,
    ) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        let attr = self
            .attrs
            .get(edge)
            .ok_or(desync("null edge without a recorded F9 attribute"))?;
        let edge_data = body
            .get_edge(edge)
            .ok_or(desync("null-pair edge no longer resolves"))?;
        let plus_start = body
            .get_half_edge(edge_data.he_plus)
            .ok_or(desync("null-edge half no longer resolves"))?
            .start;
        if plus_start == attr.below_end {
            Ok((edge_data.he_plus, edge_data.he_minus))
        } else {
            Ok((edge_data.he_minus, edge_data.he_plus))
        }
    }

    /// The up/down sense of a null-edge half — `start ∈ in_set` ⇒ up.
    /// Reads side from the attribute-derived sets (the below-copy
    /// audit: minted copies of either parity resolve here).
    fn is_up<T: Decide>(&self, body: &Body<T>, he: HalfEdgeKey) -> Result<bool, BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        let start = body
            .get_half_edge(he)
            .ok_or(desync("loose half no longer resolves"))?
            .start;
        if self.in_set.contains_key(start) {
            Ok(true)
        } else if self.out_set.contains_key(start) {
            Ok(false)
        } else {
            Err(desync("loose half starts at a vertex of neither side set"))
        }
    }

    /// The face owning a half-edge's loop.
    fn face_of<T: Decide>(&self, body: &Body<T>, he: HalfEdgeKey) -> Result<FaceKey, BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        let l = body
            .get_half_edge(he)
            .ok_or(desync("half-edge no longer resolves"))?
            .parent_loop;
        Ok(body
            .get_loop(l)
            .ok_or(desync("loop no longer resolves"))?
            .face)
    }

    /// Resolve a completed null face's loop roles by IN/OUT-set
    /// membership, uniform across each loop (mixed ⇒ typed error).
    fn resolve_roles<T: Decide>(
        &self,
        body: &Body<T>,
        face: FaceKey,
        outer: LoopKey,
        ring: LoopKey,
    ) -> Result<(LoopKey, LoopKey), BooleanError> {
        let classify = |l: LoopKey| -> Result<bool, BooleanError> {
            let starts =
                loop_starts(body, l).map_err(|_| BooleanError::JoinDesync {
                    what: "completed section loop is not walkable",
                })?;
            let in_count = starts
                .iter()
                .filter(|v| self.in_set.contains_key(**v))
                .count();
            if in_count == starts.len() {
                Ok(true)
            } else if in_count == 0 {
                Ok(false)
            } else {
                Err(BooleanError::Join(SplitJoinError::SectionLoopMixed {
                    face,
                }))
            }
        };
        match (classify(outer)?, classify(ring)?) {
            (true, false) => Ok((outer, ring)),
            (false, true) => Ok((ring, outer)),
            _ => Err(BooleanError::Join(SplitJoinError::SectionLoopMixed {
                face,
            })),
        }
    }
}

/// The lockstep joining sweep (module docs). Mutates both annotated
/// clones in `red` in place; returns the completed polygon pairs in
/// completion order, with [`NullFacePair::Boolean`] records set.
pub(super) fn bool_connect<T: Decide>(
    red: &mut BooleanReduction<T>,
    band: Band,
) -> Result<Vec<CompletedPolygonPair>, BooleanError> {
    let mut sa = SolidJoin::new(red, Operand::A, band);
    let mut sb = SolidJoin::new(red, Operand::B, band);
    let mut ends: Vec<(HalfEdgeKey, HalfEdgeKey)> = Vec::new();
    let mut completed = Vec::new();
    let pairs: Vec<(EdgeKey, EdgeKey)> = red
        .null_pairs
        .iter()
        .map(|p| (p.a_edge, p.b_edge))
        .collect();

    for (a_edge, b_edge) in pairs {
        let (a_up, a_down) = sa.halves(&red.a, a_edge)?;
        let (b_up, b_down) = sb.halves(&red.b, b_edge)?;
        // The antiparallel crossover as data (module docs).
        let offers = [(a_up, b_down), (a_down, b_up)];
        let mut joined = [false, false];
        for (slot, (a_half, b_half)) in offers.into_iter().enumerate() {
            let matched = take_neighbor(&mut ends, &red.a, &red.b, &sa, &sb, a_half, b_half)?;
            if let Some((a_end, b_end)) = matched {
                sa.joiner
                    .join(&mut red.a, a_end, a_half)
                    .map_err(BooleanError::Join)?;
                sb.joiner
                    .join(&mut red.b, b_end, b_half)
                    .map_err(BooleanError::Join)?;
                joined[slot] = true;
                retire_consumed(
                    red, &mut sa, &mut sb, &mut ends, &mut completed, a_end, b_end,
                )?;
            }
        }
        if joined[0] && joined[1] {
            cut_pair(red, &mut sa, &mut sb, &mut completed, a_edge, b_edge)?;
        }
    }

    if !ends.is_empty() {
        return Err(BooleanError::Join(SplitJoinError::UnpairedLooseEnds {
            count: ends.len(),
        }));
    }
    Ok(completed)
}

/// `scanjoin`: find a loose-end pair that neighbors the candidate in
/// BOTH solids (same face, opposite sense — per solid); consume it, or
/// register the candidate pair.
fn take_neighbor<T: Decide>(
    ends: &mut Vec<(HalfEdgeKey, HalfEdgeKey)>,
    a: &Body<T>,
    b: &Body<T>,
    sa: &SolidJoin,
    sb: &SolidJoin,
    a_half: HalfEdgeKey,
    b_half: HalfEdgeKey,
) -> Result<Option<(HalfEdgeKey, HalfEdgeKey)>, BooleanError> {
    let a_face = sa.face_of(a, a_half)?;
    let a_up = sa.is_up(a, a_half)?;
    let b_face = sb.face_of(b, b_half)?;
    let b_up = sb.is_up(b, b_half)?;
    for i in 0..ends.len() {
        let (a_end, b_end) = ends[i];
        let a_hit = sa.face_of(a, a_end)? == a_face && sa.is_up(a, a_end)? != a_up;
        let b_hit = sb.face_of(b, b_end)? == b_face && sb.is_up(b, b_end)? != b_up;
        if a_hit && b_hit {
            ends.remove(i);
            return Ok(Some((a_end, b_end)));
        }
    }
    ends.push((a_half, b_half));
    Ok(None)
}

/// Retire the consumed ends' edges once their other halves stop being
/// loose — in both solids together, or not at all (lockstep).
fn retire_consumed<T: Decide>(
    red: &mut BooleanReduction<T>,
    sa: &mut SolidJoin,
    sb: &mut SolidJoin,
    ends: &mut [(HalfEdgeKey, HalfEdgeKey)],
    completed: &mut Vec<CompletedPolygonPair>,
    a_end: HalfEdgeKey,
    b_end: HalfEdgeKey,
) -> Result<(), BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let a_edge = red
        .a
        .get_half_edge(a_end)
        .ok_or(desync("consumed A end no longer resolves"))?
        .edge;
    let b_edge = red
        .b
        .get_half_edge(b_end)
        .ok_or(desync("consumed B end no longer resolves"))?
        .edge;
    let a_mate = red.a.mate(a_end).ok_or(desync("A end has no mate"))?;
    let b_mate = red.b.mate(b_end).ok_or(desync("B end has no mate"))?;
    let a_loose = ends.iter().any(|(ae, _)| *ae == a_mate);
    let b_loose = ends.iter().any(|(_, be)| *be == b_mate);
    if a_loose != b_loose {
        return Err(desync("solids disagree on loose-end retirement"));
    }
    if !a_loose {
        cut_pair(red, sa, sb, completed, a_edge, b_edge)?;
    }
    Ok(())
}

/// Cut the corresponding null edges in both solids; completions must
/// coincide (lockstep), and roles are resolved from the F9 side sets.
fn cut_pair<T: Decide>(
    red: &mut BooleanReduction<T>,
    sa: &mut SolidJoin,
    sb: &mut SolidJoin,
    completed: &mut Vec<CompletedPolygonPair>,
    a_edge: EdgeKey,
    b_edge: EdgeKey,
) -> Result<(), BooleanError> {
    let a_out = sa
        .joiner
        .cut_core(&mut red.a, a_edge)
        .map_err(BooleanError::Join)?;
    let b_out = sb
        .joiner
        .cut_core(&mut red.b, b_edge)
        .map_err(BooleanError::Join)?;
    match (a_out, b_out) {
        (CutOutcome::Merged, CutOutcome::Merged) => Ok(()),
        (
            CutOutcome::Completed {
                face: a_face,
                ring: a_ring,
            },
            CutOutcome::Completed {
                face: b_face,
                ring: b_ring,
            },
        ) => {
            let desync = |what| BooleanError::JoinDesync { what };
            let a_outer = red
                .a
                .get_face(a_face)
                .ok_or(desync("completed A face no longer resolves"))?
                .outer;
            let b_outer = red
                .b
                .get_face(b_face)
                .ok_or(desync("completed B face no longer resolves"))?
                .outer;
            let (a_in_loop, a_out_loop) = sa.resolve_roles(&red.a, a_face, a_outer, a_ring)?;
            let (b_in_loop, b_out_loop) = sb.resolve_roles(&red.b, b_face, b_outer, b_ring)?;
            red.a.set_null_face_pair(
                a_face,
                NullFacePair::Boolean {
                    in_copy: a_in_loop,
                    out_copy: a_out_loop,
                },
            )?;
            red.b.set_null_face_pair(
                b_face,
                NullFacePair::Boolean {
                    in_copy: b_in_loop,
                    out_copy: b_out_loop,
                },
            )?;
            completed.push(CompletedPolygonPair {
                a_face,
                a_in_loop,
                a_out_loop,
                b_face,
                b_in_loop,
                b_out_loop,
            });
            Ok(())
        }
        _ => Err(BooleanError::JoinDesync {
            what: "one solid completed a polygon where the other merged slivers",
        }),
    }
}
