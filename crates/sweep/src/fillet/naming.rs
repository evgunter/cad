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
//! A shrunk support is therefore a survivor and needs no row of its
//! own: the fact that it is the same face is carried by key identity.
//!
//! [`FilletNaming::dead`] closes the loop: it lists the source keys
//! the fillet RETIRED, so a consumer can check
//! `output = (source − dead) ⊎ minted` rather than assume it — in BOTH
//! directions (`sweep/tests/m6_5_fillet_naming.rs` executes both). A
//! survivor is thus a birth fact too — "this key was not minted and
//! not retired" — not an inference from geometry.
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
    /// Source edges that no longer exist: the requested chain edges
    /// (excised across their strips) and, on the rim path, the
    /// meridian remnants killed with their rim vertex.
    pub edges: Vec<EdgeKey>,
    /// Source vertices that no longer exist: the sharp corners fused
    /// under their octants, and the rim vertices.
    pub vertices: Vec<VertexKey>,
}

/// **Per-entity birth records for one fillet.** Every field is
/// `(minted key, the source entity it was minted for, …)`, in the
/// deterministic order the constructor visited them (D9).
///
/// A request whose chains are all open fills `blends`, `corners`,
/// `trims`, `feet`, `arcs` and `dead`, leaving every rim field empty;
/// a closed (rim) chain fills the rim phase as well.
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

    /// What the fillet retired from the source.
    pub dead: Retired,
}
