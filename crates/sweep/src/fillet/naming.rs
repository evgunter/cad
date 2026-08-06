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
//! # The two doors, and their two provenance channels
//!
//! The **composition surgery** mutates a CLONE of the source body in
//! place, so an output entity's provenance is one of exactly two
//! things:
//!
//! - it was **minted** by the surgery — one of the rows below names
//!   the source entity it was minted FOR; or
//! - it is a **survivor**, keeping the arena key it had in the source
//!   (a shrunk support face, an untouched edge, a far vertex).
//!
//! The **whole-body rebuild** (M6-5 PR-2) has no survivors: it mints
//! every face of the result fresh into a new arena, so nothing carries
//! a source key. Its shrunk support faces are therefore recorded
//! explicitly, in [`FilletNaming::supports`] — the ONE row the surgery
//! never writes, because there the same fact is carried by key
//! identity. Both doors then name a shrunk support the same way, which
//! is the point: a name must not depend on which door built the body.
//!
//! [`FilletNaming::dead`] closes the loop: it lists the source keys
//! the fillet RETIRED, so a consumer can check
//! `output = (source − dead) ⊎ minted` rather than assume it — in BOTH
//! directions (`sweep/tests/m6_5_fillet_naming.rs` executes both). On
//! the surgery door a survivor is therefore a birth fact too — "this
//! key was not minted and not retired" — not an inference from
//! geometry. On the whole-body door every source entity is retired,
//! and `supports` carries what would otherwise be lost.
//!
//! # What consumes these rows
//!
//! `editor-core`'s `names::emit_fillet` is the one production
//! consumer. It reads every field EXCEPT [`Retired`], which exists for
//! the totality identity the test suite executes: the emitter does not
//! need it, because an output key that is neither minted nor present
//! upstream already refuses `MissingUpstream` when it is looked up.
//! `Retired` is what makes that refusal a checked consequence of the
//! construction rather than a hope, and it is the only thing that can
//! catch a source entity destroyed WITHOUT a record — a case the
//! emitter cannot see, since a destroyed entity leaves no output key
//! to ask about.

use topo::{EdgeKey, FaceKey, VertexKey};

/// Which support a rim band's trim arc lies on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RimSide {
    /// The planar support (the face carrying the rim as a ring).
    Plane,
    /// The curved support (the cap the rim bounds).
    Sphere,
}

/// The source keys the fillet retired.
#[derive(Clone, Debug, Default)]
pub struct Retired {
    /// Source edges that no longer exist. Surgery: the requested chain
    /// edges (excised across their strips) and, on the rim path, the
    /// meridian remnants killed with their rim vertex. Whole-body:
    /// EVERY source edge — the door admits only the every-edge request
    /// and rebuilds into a fresh arena.
    pub edges: Vec<EdgeKey>,
    /// Source vertices that no longer exist. Surgery: the sharp
    /// corners fused under their octants, and the rim vertices.
    /// Whole-body: every source vertex, each replaced by its octant.
    pub vertices: Vec<VertexKey>,
}

/// **Per-entity birth records for one fillet.** Every field is
/// `(minted key, the source entity it was minted for, …)`, in the
/// deterministic order the constructor visited them (D9).
///
/// The whole-body door fills [`FilletNaming::supports`], `blends`,
/// `corners`, `trims`, `feet`, `arcs` and `dead`; it admits no closed
/// chains, so every rim field stays empty. The surgery door fills
/// everything except `supports`.
#[derive(Clone, Debug, Default)]
pub struct FilletNaming {
    /// Shrunk support face ← the source face it is the shrunk copy of.
    /// **Whole-body door only**: the surgery's shrunk supports keep
    /// their source key and are survivors, so they need no row (module
    /// docs).
    pub supports: Vec<(FaceKey, FaceKey)>,

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

    /// What the fillet retired from the source.
    pub dead: Retired,
}
