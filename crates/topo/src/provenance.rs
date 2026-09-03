//! D5 provenance: persistent topological identity from birth.
//!
//! Every topology entity carries a provenance record from the moment it is
//! created — which operation created it, from which inputs. This does not
//! solve the topological naming problem, but recording identity at birth
//! is cheap and retrofitting it onto anonymous entities is nearly
//! impossible (D5 in `docs/DESIGN.md`).
//!
//! Since M1 PR 2 the record is **typed per operator**: each Euler
//! operator stamps everything it mints with its own variant carrying the
//! site (argument keys) it was applied at — [`Provenance::Mvfs`],
//! [`Provenance::Mev`], [`Provenance::Mef`], (PR 3)
//! [`Provenance::Kemr`], [`Provenance::Mekr`], and (PR 4)
//! [`Provenance::Mfkrh`]. Operators that mint nothing (`kfmrh`,
//! `ring_move`, and PR 4's `kvfs`/`kev`/`kef`) have no variant: a
//! provenance record is a **birth record**, written once when the entity
//! is created and never rewritten — a loop that `kfmrh` demotes to a
//! ring, that `mfkrh` promotes back to an outer loop, or that
//! `ring_move` reparents, keeps the record of the operation that
//! *created* it. Symmetrically, kill-direction operators **remove** the
//! records of the entities they kill (a `SecondaryMap` entry outliving
//! its entity would be a leak; the validator's bidirectional provenance
//! pass makes such leaks loud in both directions). Together with deterministic
//! minting (D9), the provenance records are the derivation's fingerprints
//! in the materialized body (D1: a `Body` is never authoritative — it is
//! the evaluation of its construction, and provenance points back at that
//! construction). Later PRs extend the variant set operator by operator;
//! M4's persistent naming builds its stable references on top. Extending
//! this enum is a compiler-guided change across every match site, per the
//! workspace's closed-enum style (D3).
//!
//! Storage shape: a `slotmap::SecondaryMap<Key, Provenance>` per entity
//! kind inside [`Body`](crate::Body), not an inline field on each entity.
//! This keeps the entity structs pure containment data (what a shape *is*)
//! with identity bookkeeping parallel to it (where it *came from*), and it
//! lets M4's naming machinery attach richer records without touching
//! entity layout. Both the raw builder API and the Euler operators record
//! a `Provenance` at every topology insertion, so an entity without
//! provenance is unrepresentable.

use crate::entity::{HalfEdgeKey, LoopKey};
use crate::euler::{MefSite, MevSite};
use crate::euler_ring::MekrSite;

/// Why a topology entity exists (D5).
///
/// The site payloads record the operator's argument keys — keys are
/// body-lineage-scoped (see [`Body`](crate::Body)), so a provenance
/// record is meaningful exactly where its entity is. The recorded keys
/// are the arguments **as they were at call time**: a kill-direction
/// record (e.g. [`Provenance::Kemr`], which kills the very half-edges it
/// was addressed by) may hold keys that no longer resolve — the record
/// is historical, not a live reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Provenance {
    /// Created directly — by test scaffolding through the crate-internal
    /// raw builder (`pub(crate)` since M1 PR 5; the Euler operators are
    /// the only public construction path) — rather than by an operator.
    Primordial {
        /// A static label naming the creating context (e.g.
        /// `"test:tiny-body"`).
        op: &'static str,
    },
    /// Created by [`Body::mvfs`](crate::Body::mvfs) (which consumes
    /// nothing, so there are no argument keys to record): the skeletal
    /// body's solid, shell, face, empty loop, and lone vertex.
    Mvfs,
    /// Created by [`Body::mev`](crate::Body::mev): the new vertex, edge,
    /// and both half-edges.
    Mev {
        /// The site the operator was applied at (its argument keys).
        site: MevSite,
    },
    /// Created by [`Body::mef`](crate::Body::mef): the new face, loop,
    /// edge, and both half-edges.
    Mef {
        /// The site the operator was applied at (its argument keys).
        site: MefSite,
    },
    /// Created by [`Body::kemr`](crate::Body::kemr): the new ring loop
    /// (the only entity `kemr` mints). The recorded half-edges are the
    /// killed edge's two halves and no longer resolve (see the type
    /// docs: records are historical).
    Kemr {
        /// `kemr`'s first argument — the half whose side became this
        /// ring.
        he1: HalfEdgeKey,
        /// `kemr`'s second argument — the half whose side kept the old
        /// loop.
        he2: HalfEdgeKey,
    },
    /// Created by [`Body::mekr`](crate::Body::mekr): the new edge and
    /// both half-edges.
    Mekr {
        /// The site the operator was applied at (its argument keys).
        site: MekrSite,
    },
    /// Created by [`Body::mfkrh`](crate::Body::mfkrh): the new face (the
    /// only topology entity `mfkrh` mints — the promoted loop survives
    /// with its own birth record).
    Mfkrh {
        /// `mfkrh`'s argument — the ring promoted to the new face's
        /// outer loop. A **surviving** key, unlike [`Provenance::Kemr`]'s
        /// (the loop outlives the call), though later kills may of
        /// course retire it; records are historical either way.
        ring: LoopKey,
    },
    /// Created by [`Body::mev_null`](crate::Body::mev_null) (M3 PR 1):
    /// the new coincident vertex, the null edge, and both half-edges.
    MevNull {
        /// The site the operator was applied at (its argument keys).
        site: MevSite,
        /// The declared F9 side of the new vertex.
        new_side: crate::null::NewVertexSide,
    },
    /// Created by [`Body::split_edge`](crate::Body::split_edge) (M3
    /// PR 1): the new interior vertex, the new (second-child) edge, and
    /// both its half-edges.
    SplitEdge {
        /// The edge that was split (a surviving key: it becomes the
        /// first child).
        edge: crate::entity::EdgeKey,
    },
    /// Created by [`Body::movefac`](crate::Body::movefac) (M3 PR 1):
    /// a new shell minted to re-home one connected component.
    Movefac {
        /// The shell that was partitioned.
        shell: crate::entity::ShellKey,
    },
}

/// A split lineage that never reaches a root: chasing `SplitEdge`
/// birth records parent to parent revisited an edge. A lineage is a
/// chain of strictly older edges, so this is a corrupt provenance
/// record — a kernel bug surfaced typed, never a silently chosen
/// root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitLineageCycle {
    /// The edge whose lineage cycles.
    pub edge: crate::entity::EdgeKey,
}

impl core::fmt::Display for SplitLineageCycle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "split lineage of edge {:?} cycles: its SplitEdge records never reach a root",
            self.edge
        )
    }
}

impl std::error::Error for SplitLineageCycle {}

impl<T: geom_core::Real> crate::Body<T> {
    /// The root of `edge`'s split lineage: its `SplitEdge` birth
    /// records chased parent to parent until `stop` says the caller's
    /// identity boundary is reached (a name table's first named key,
    /// for the names lane) or the edge carries no split record — that
    /// edge is the root. `split_edge` keeps the parent's key for the
    /// first child and records the parent on the second, so every
    /// piece of one original edge reaches one root, and a body's loop
    /// flattening mints each edge's carrier identity from it.
    ///
    /// Bounded by the body's edge count: a walk that has not ended
    /// after every edge could have been visited once has revisited
    /// one, and returns [`SplitLineageCycle`] rather than a root.
    ///
    /// # Errors
    ///
    /// [`SplitLineageCycle`] on a cycling lineage.
    pub fn split_root(
        &self,
        edge: crate::entity::EdgeKey,
        mut stop: impl FnMut(crate::entity::EdgeKey) -> bool,
    ) -> Result<crate::entity::EdgeKey, SplitLineageCycle> {
        let mut root = edge;
        for _ in 0..=self.edges.len() {
            if stop(root) {
                return Ok(root);
            }
            match self.edge_provenance_of(root) {
                Some(Provenance::SplitEdge { edge: parent }) => root = *parent,
                _ => return Ok(root),
            }
        }
        Err(SplitLineageCycle { edge })
    }
}
