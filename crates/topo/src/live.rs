//! [`Live`] — a half-edge key that has been observed to resolve.
//!
//! The Euler surgery modules splice by writing `next`/`prev` through
//! [`Body::link_half_edges`], whose two writes must both land: a splice
//! that half-happens leaves a torn loop cycle. Under the D2 addendum a
//! lookup that fails there is a kernel bug announced with
//! `unreachable!`, so the obligation to know the key resolves belongs to
//! whoever supplies it — and a shared helper cannot discharge it,
//! because it knows none of its callers.
//!
//! `Live` is that obligation as a type. It wraps a [`HalfEdgeKey`] and
//! its field is private to this module, so **the only way to obtain one
//! is a door that performs the lookup** — [`Live::certify`], or
//! [`Body::resolve_half_edge_live`], which keeps the certificate the
//! same `Some` arm already earned. There is no trusted mint, no `From`,
//! no public field: a key nothing has looked up cannot reach a splice.
//!
//! # What a `Live` claims, exactly
//!
//! *This key resolved in this body when the token was made.* Not that it
//! resolves now — Rust has no way to expire a plain value when a `&mut`
//! call mutates the arena it describes, short of branding [`Body`] with
//! a generative lifetime, which would infect every signature that names
//! a body. So the residual obligation the token does **not** carry is
//! named here rather than left to be re-derived: *do not remove
//! half-edges between certifying a key and splicing it.* Every mutation
//! phase in this crate satisfies that by shape — half-edge removal is
//! the last thing an operator does, after every splice.
//!
//! That residue is not silent. The arenas are slotmaps, so a key removed
//! from this body never resolves in it again: a token that outlived its
//! key fails the lookup at the splice and announces, exactly as an
//! unproven raw key does today. The type never converts a loud failure
//! into a quiet one; what it adds is that the check cannot be skipped.
//!
//! The one claim outside its reach is a token certified against one body
//! and spliced into another: slotmap keys are per-arena, so the second
//! lookup may resolve to an unrelated half-edge. That is the
//! live-but-wrong shape, not liveness — no lookup detects it, the
//! validator does — and every splice in this crate certifies and writes
//! through one `&mut self`.

use crate::body::Body;
use crate::entity::{EntityId, HalfEdge, HalfEdgeKey};
use crate::euler::EulerOpError;
use geom_core::Real;

/// A [`HalfEdgeKey`] that was observed to resolve — see the [module
/// docs](self) for the precise claim and its one residue.
///
/// `Copy`, because a proof used twice is the same proof: a splice
/// through a one-half-edge loop links a key to itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Live(HalfEdgeKey);

impl Live {
    /// Certification from a bare key: the lookup, and nothing else.
    /// `None` when the key does not resolve.
    pub(crate) fn certify<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> Option<Self> {
        body.half_edges.contains_key(he).then_some(Self(he))
    }

    /// The key back out. Certification is one-way by design: this
    /// direction discards a proof, which is always sound.
    pub(crate) fn key(self) -> HalfEdgeKey {
        self.0
    }
}

impl<T: Real> Body<T> {
    /// Certifies a half-edge key for splicing, refusing a stale one as
    /// the plan phase's typed error.
    ///
    /// This is the plan-phase door: an operator that will splice through
    /// a key it read out of the arena certifies it here, **before any
    /// mutation**, so the mutation phase below cannot fail midway
    /// (atomicity).
    pub(crate) fn certify_half_edge(&self, he: HalfEdgeKey) -> Result<Live, EulerOpError> {
        Live::certify(self, he).ok_or(EulerOpError::StaleKey {
            key: EntityId::HalfEdge(he),
        })
    }

    /// [`Body::resolve_half_edge`] keeping the certificate its lookup
    /// earns, for an operator that both reads a half-edge's fields and
    /// splices through the key itself.
    ///
    /// The certificate comes out of the same `Some` arm the fields do,
    /// so this door checks exactly once — an operator that certified
    /// separately would carry a refusal path its own `resolve` had
    /// already made unreachable.
    pub(crate) fn resolve_half_edge_live(
        &self,
        he: HalfEdgeKey,
    ) -> Result<(Live, HalfEdge), EulerOpError> {
        match self.half_edges.get(he) {
            Some(data) => Ok((Live(he), data.clone())),
            None => Err(EulerOpError::StaleKey {
                key: EntityId::HalfEdge(he),
            }),
        }
    }

    /// [`Body::loop_cycle`] with its members certified.
    ///
    /// The walk resolves every member it returns, so this costs one
    /// redundant lookup per member and adds no failure mode: `None` here
    /// means the walk itself failed.
    pub(crate) fn loop_cycle_live(&self, he: HalfEdgeKey) -> Option<Vec<Live>> {
        self.loop_cycle(he)?
            .into_iter()
            .map(|member| Live::certify(self, member))
            .collect()
    }
}
