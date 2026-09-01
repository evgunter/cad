//! [`Live`] — a half-edge key a lookup has returned.
//!
//! [`Body::link_half_edges`] splices by writing `next`/`prev`, and under
//! the D2 addendum a failed lookup there is a kernel bug announced with
//! `unreachable!`. The obligation to know the key resolves therefore
//! belongs to whoever supplies it — a shared helper knows none of its
//! callers. `Live` is that obligation as a type: private field, own
//! module, and **every door that hands one out performs the lookup**.
//!
//! # What a `Live` claims
//!
//! *This key resolved in this body when the token was made* — not that
//! it resolves now, which no plain value can promise across a `&mut`
//! call. Hence the rule, stated here rather than left to be re-derived:
//! **do not remove half-edges between proving a key and splicing one.**
//! Every mutation phase in this crate obeys it by shape — half-edge
//! removal is the last thing an operator does.
//!
//! Breaking it is loud: the arenas are slotmaps, so a removed key never
//! resolves again and a stale token fails at the splice. What no lookup
//! catches is a token proven against one body and spliced into another,
//! where the key may resolve to an unrelated half-edge — live-but-wrong,
//! which is the validator's business and not liveness.
//!
//! # Guarding
//!
//! That no `Live` exists without a lookup rests on this file's privacy,
//! and **the crate's usual instrument cannot check it**: a
//! `compile_fail` doctest cannot name a `pub(crate)` type. A
//! source-level guard can, and is owed; until one lands the claim rests
//! on review of this file.
//!
//! # The other arenas
//!
//! A plan phase that only needs to REFUSE a stale key — no proof to
//! carry into a splice — calls [`require_key`], which lives here for
//! the same reason `Live` does: one statement of what a liveness check
//! is and which [`EntityId`] a failed one names.
use crate::body::Body;
use crate::entity::{EntityId, HalfEdge, HalfEdgeKey};
use crate::euler::EulerOpError;
use geom_core::Real;

/// A [`HalfEdgeKey`] a lookup has returned — see the [module
/// docs](self) for the precise claim and its one residue.
///
/// `Copy`, because a proof used twice is the same proof: a splice
/// through a one-half-edge loop links a key to itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Live(HalfEdgeKey);

impl Live {
    /// **The one place a `Live` is built from a bare key.** Private, so
    /// every caller of it is in this file and is a door that has just
    /// completed a successful lookup.
    const fn new(he: HalfEdgeKey) -> Self {
        Self(he)
    }

    /// Proof from a bare key: the lookup, and nothing else. `None` when
    /// the key does not resolve.
    pub(crate) fn of<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> Option<Self> {
        body.half_edges.contains_key(he).then(|| Self::new(he))
    }

    /// The key back out. The proof is one-way by design: this direction
    /// discards it, which is always sound.
    pub(crate) fn key(self) -> HalfEdgeKey {
        self.0
    }
}

/// Requires a key to be live in its arena, refusing a stale one as the
/// plan phase's typed error — the obligation [`Body::require_live`]
/// carries for half-edges, for the arenas that have no proof token.
///
/// Only half-edges are spliced through a key the caller holds across a
/// `&mut` call, so only they need a [`Live`] to carry the lookup
/// forward; every other arena's plan phase wants the refusal and
/// nothing else. One body for all of them, so the check and the
/// [`EntityId`] it names cannot drift arena by arena.
pub(crate) fn require_key<K: slotmap::Key, V>(
    arena: &slotmap::SlotMap<K, V>,
    key: K,
    id: fn(K) -> EntityId,
) -> Result<(), EulerOpError> {
    if arena.contains_key(key) {
        Ok(())
    } else {
        Err(EulerOpError::StaleKey { key: id(key) })
    }
}

impl<T: Real> Body<T> {
    /// Requires a half-edge key to be live, refusing a stale one as the
    /// plan phase's typed error.
    ///
    /// This is the plan-phase door: an operator that will splice through
    /// a key it read out of the arena proves it here, **before any
    /// mutation**, so the mutation phase below cannot fail midway
    /// (atomicity).
    pub(crate) fn require_live(&self, he: HalfEdgeKey) -> Result<Live, EulerOpError> {
        Live::of(self, he).ok_or(EulerOpError::StaleKey {
            key: EntityId::HalfEdge(he),
        })
    }

    /// [`Body::resolve_half_edge`] keeping the proof its lookup earns,
    /// for an operator that both reads a half-edge's fields and splices
    /// through the key itself.
    ///
    /// The proof comes out of the same `Some` arm the fields do, so this
    /// door looks up exactly once — an operator that resolved and then
    /// required separately would carry a refusal its own resolve had
    /// already made unreachable.
    pub(crate) fn resolve_half_edge_live(
        &self,
        he: HalfEdgeKey,
    ) -> Result<(Live, HalfEdge), EulerOpError> {
        match self.half_edges.get(he) {
            Some(data) => Ok((Live::new(he), data.clone())),
            None => Err(EulerOpError::StaleKey {
                key: EntityId::HalfEdge(he),
            }),
        }
    }

    /// [`Body::loop_cycle`] with its members proven.
    ///
    /// The walk resolves every member it returns, so this costs one
    /// redundant lookup per member and adds no failure mode: `None` here
    /// means the walk itself failed.
    pub(crate) fn loop_cycle_live(&self, he: HalfEdgeKey) -> Option<Vec<Live>> {
        self.loop_cycle(he)?
            .into_iter()
            .map(|member| Live::of(self, member))
            .collect()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::Live;
    use crate::body::Body;
    use crate::entity::{EdgeKey, HalfEdge, HalfEdgeKey, LoopKey, VertexKey};
    use crate::fixtures::pillow;
    use geom_core::Tol;

    fn scaffold() -> HalfEdge {
        HalfEdge {
            edge: EdgeKey::default(),
            start: VertexKey::default(),
            parent_loop: LoopKey::default(),
            next: HalfEdgeKey::default(),
            prev: HalfEdgeKey::default(),
        }
    }

    /// The null key is what an unspliced half-edge's `next`/`prev` hold
    /// — [`Body::mint_halves`] leaves both provisional — so it is the
    /// key likeliest to reach a door by accident. It resolves in no
    /// body, populated or not.
    #[test]
    fn the_null_key_is_never_live() {
        let empty = Body::<f64>::new();
        assert!(Live::of(&empty, HalfEdgeKey::default()).is_none());
        assert!(empty.require_live(HalfEdgeKey::default()).is_err());
        let body = pillow(Tol::witness()).body;
        assert!(!body.half_edges.is_empty(), "the arena must be populated");
        assert!(Live::of(&body, HalfEdgeKey::default()).is_none());
        assert!(body.require_live(HalfEdgeKey::default()).is_err());
    }

    /// A removed key never becomes live again, however hard the slot it
    /// vacated is reused. This is what makes the module's residue rule
    /// *loud*: a proof that outlived its key fails at the splice rather
    /// than resolving to whatever now occupies the slot.
    #[test]
    fn a_removed_key_stays_dead_across_slot_reuse() {
        let mut body = Body::<f64>::new();
        let dead = body.half_edges.insert(scaffold());
        body.half_edges.remove(dead);
        assert!(Live::of(&body, dead).is_none());
        for _ in 0..200_000 {
            let fresh = body.half_edges.insert(scaffold());
            assert_ne!(fresh, dead, "a reused slot must mint a NEW key");
            assert!(
                Live::of(&body, dead).is_none(),
                "the dead key resolved once its slot was reused"
            );
            body.half_edges.remove(fresh);
        }
        assert!(body.require_live(dead).is_err());
    }
}
