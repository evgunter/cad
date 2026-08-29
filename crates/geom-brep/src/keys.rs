//! Typed keys into a body's geometry arenas.
//!
//! These are slotmap key *types*, not arenas: the arenas themselves live
//! in `topo`'s `Body<T>` (points, curves, surfaces), which re-exports
//! these types unchanged. They are defined **here**, one layer below
//! `topo`, because D2's intensional edge descriptions reference surfaces
//! *by arena key* ([`crate::EdgeDescription::Intersection`] holds two
//! `SurfaceKey`s) — the description layer has to be able to name arena
//! slots without depending on the arena store above it.
//!
//! # Lineage scoping (Q1)
//!
//! A key is meaningful only within the lineage of the body that minted
//! it. The full stale-vs-foreign story is documented on `topo::Body`;
//! the one-line consequence for this crate: certification never resolves
//! keys itself — resolution is injected by the caller (a lookup closure
//! over the owning body's arenas), so a key never silently crosses
//! lineages inside this layer.

use slotmap::new_key_type;

new_key_type! {
    /// Typed key into a body's point arena (vertex geometry).
    pub struct PointKey;

    /// Typed key into a body's curve arena (edge geometry).
    pub struct CurveKey;

    /// Typed key into a body's surface arena (face geometry).
    pub struct SurfaceKey;
}
