//! D5 provenance: persistent topological identity from birth.
//!
//! Every topology entity carries a provenance record from the moment it is
//! created — which operation created it, from which inputs. This does not
//! solve the topological naming problem, but recording identity at birth
//! is cheap and retrofitting it onto anonymous entities is nearly
//! impossible (D5 in `docs/DESIGN.md`) — so the *slot* exists from day
//! one, even though M0 has nothing interesting to put in it.
//!
//! Storage shape: a `slotmap::SecondaryMap<Key, Provenance>` per entity
//! kind inside [`Body`](crate::Body), not an inline field on each entity.
//! This keeps the entity structs pure containment data (what a shape *is*)
//! with identity bookkeeping parallel to it (where it *came from*), and it
//! lets M4's naming machinery attach richer records without touching
//! entity layout. The builder API takes a `Provenance` at every topology
//! insertion, so an entity without provenance is unrepresentable.

/// Why a topology entity exists (D5). M0 placeholder shape.
///
/// M1's Euler operators add operator variants (which operation, from which
/// input entities), and M4's persistent naming builds its stable
/// references on top; extending this enum is a compiler-guided change
/// across every match site, per the workspace's closed-enum style (D3).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Provenance {
    /// Created directly — by hand, by test scaffolding, or by primitive
    /// construction predating the Euler operators — rather than derived
    /// from other entities by an operator.
    Primordial {
        /// A static label naming the creating operation or context (e.g.
        /// `"test:tiny-body"`). A `&'static str` placeholder: M1 replaces
        /// free-form labels with typed operator variants.
        op: &'static str,
    },
}
