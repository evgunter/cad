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
//! [`Provenance::Mev`], [`Provenance::Mef`]. Together with deterministic
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

use crate::euler::{MefSite, MevSite};

/// Why a topology entity exists (D5).
///
/// The site payloads record the operator's argument keys — keys are
/// body-lineage-scoped (see [`Body`](crate::Body)), so a provenance
/// record is meaningful exactly where its entity is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Provenance {
    /// Created directly — by hand, by test scaffolding, or by the raw
    /// builder placeholder (which retreats behind the Euler operators at
    /// M1 PR 5) — rather than by an operator.
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
}
