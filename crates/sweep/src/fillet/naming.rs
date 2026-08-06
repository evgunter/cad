//! **Fillet birth records** (M6-5): what the composition surgery
//! MINTED, and from which entity of the source body — recorded by the
//! construction itself, as it constructs.
//!
//! The kernel never sees a stable name (D1/G1): this module carries
//! arena keys and nothing else. `editor-core`'s emitter turns these
//! rows into names. What matters is that every row is written at the
//! moment the entity is born, from the plan that decided to mint it —
//! never recovered afterwards by matching geometry, which is exactly
//! what N4 forbids (the `BooleanNaming` / `SplitNaming` discipline).
//!
//! # The two provenance channels
//!
//! The surgery mutates a CLONE of the source body in place, so an
//! output entity's provenance is one of exactly two things:
//!
//! - it was **minted** by the surgery — one of the rows below names
//!   the source entity it was minted FOR; or
//! - it is a **survivor**, keeping the arena key it had in the source
//!   (a shrunk support face, an untouched edge, a far vertex).
//!
//! [`FilletNaming::dead`] closes the loop: it lists the source keys
//! the surgery RETIRED, so a consumer can check
//! `output = (source − dead) ⊎ minted` rather than assume it. A
//! survivor is therefore a birth fact too — "this key was not minted
//! and not retired" — not an inference from geometry.

use topo::{EdgeKey, FaceKey, VertexKey};

/// Which support a rim band's trim arc lies on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RimSide {
    /// The planar support (the face carrying the rim as a ring).
    Plane,
    /// The curved support (the cap the rim bounds).
    Sphere,
}

/// The source keys the surgery retired.
#[derive(Clone, Debug, Default)]
pub struct Retired {
    /// Source edges that no longer exist: the requested chain edges
    /// (excised across their strips) and, on the rim path, the
    /// meridian remnants killed with their rim vertex.
    pub edges: Vec<EdgeKey>,
    /// Source vertices that no longer exist: the sharp corners fused
    /// under their octants, and the rim vertices.
    pub vertices: Vec<VertexKey>,
}

/// **Per-entity birth records for one surgery.** Every field is
/// `(minted key, the source entity it was minted for, …)`, in the
/// deterministic order the surgery visited them (D9).
#[derive(Clone, Debug, Default)]
pub struct FilletNaming {
    // ---- The blank phase (open plane–plane chains). ----
    /// Blend face ← the source edge it rounds.
    pub blends: Vec<(FaceKey, EdgeKey)>,
    /// Octant face ← the source (trivalent, sharp) vertex it rounds.
    pub corners: Vec<(FaceKey, VertexKey)>,
    /// Trimline edge ← (the source edge it parallels, the support
    /// face it lies in).
    pub trims: Vec<(EdgeKey, EdgeKey, FaceKey)>,
    /// Foot vertex ← (the source corner vertex it retracts from, the
    /// support face it lies in).
    pub feet: Vec<(VertexKey, VertexKey, FaceKey)>,
    /// Corner arc ← (the source corner vertex, the source edge whose
    /// blend the arc bounds).
    pub arcs: Vec<(EdgeKey, VertexKey, EdgeKey)>,

    // ---- The rim phase (closed plane–sphere chains). ----
    /// Torus band face ← the closed chain's source edges, sorted.
    pub bands: Vec<(FaceKey, Vec<EdgeKey>)>,
    /// Rim trim arc ← (the source rim edge it replaces, which support
    /// it lies on).
    pub rim_trims: Vec<(EdgeKey, EdgeKey, RimSide)>,
    /// Rim foot vertex ← the source rim vertex it retracts from, on
    /// the planar support.
    pub rim_feet: Vec<(VertexKey, VertexKey)>,
    /// Meridian split vertex ← the source meridian edge it split.
    pub meridian_splits: Vec<(VertexKey, EdgeKey)>,
    /// The SURVIVING piece of a split meridian ← the source meridian.
    /// (Present even when the surviving piece kept the source key —
    /// the piece is a fragment, so it is named as one.)
    pub meridian_remnants: Vec<(EdgeKey, EdgeKey)>,
    /// A band's SLIT ← the source meridian whose upper piece became
    /// it (the double-traversed torus meridian; one per band).
    pub slits: Vec<(EdgeKey, EdgeKey)>,

    /// What the surgery retired from the source.
    pub dead: Retired,
}
