//! **Edge-blend birth records** (M6-5): what the composition surgery
//! MINTED, and from which entity of the source body — recorded by the
//! construction itself, as it constructs. Both verbs write these rows
//! through the one shared surgery: a fillet's row carries a curved
//! mint (a torus arc, a sphere octant) where a chamfer's carries its
//! flat twin (a straight chord, a plane patch), and the ROWS are the
//! same because the carve is.
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
//! [`BlendNaming::dead`] closes the loop: it lists the source keys
//! the blend RETIRED, so a consumer can check
//! `output = (source − dead) ⊎ minted` rather than assume it — in BOTH
//! directions (`sweep/tests/m6_5_fillet_naming.rs` executes both). A
//! survivor is thus a birth fact too — "this key was not minted and
//! not retired" — not an inference from geometry.
//!
//! # What consumes these rows
//!
//! `editor-core`'s `names::emit_blend` is the one production
//! consumer (one IMPLEMENTATION, reached through both verbs' thin
//! emitter doors). It reads every field EXCEPT [`Retired`], which exists for
//! the totality identity the test suite executes: the emitter does not
//! need it, because an output key that is neither minted nor present
//! upstream already refuses `MissingUpstream` when it is looked up.
//! `Retired` is what makes that refusal a checked consequence of the
//! construction rather than a hope, and it is the only thing that can
//! catch a source entity destroyed WITHOUT a record — a case the
//! emitter cannot see, since a destroyed entity leaves no output key
//! to ask about.

use topo::{EdgeKey, FaceKey, VertexKey};

/// Which of a rim band's two supports a trim arc lies on.
///
/// The two variants are the carve's two ROLES, which is what the
/// surgery decides and therefore all it can record: the supports of a
/// rim are told apart by the link that resolved them, never by their
/// surface kinds — a rim between two cones has two supports of the
/// same kind, and a kind would not tell them apart at all.
///
/// [`RimSide::Host`] is the PLANAR support wherever the rim has one
/// (see [`second_support_is_host`], the one place that rule lives), so a
/// ladder rim's two arcs keep the sides a caller means by "the flat
/// one" and "the cap"; a curved-on-curved rim takes the link's own
/// `face_a` as host.
///
/// **The roles are fixed under every edit that does not cross the
/// PLANARITY boundary, and re-decide across it** — the host is defined
/// by planarity, so carrying a support from curved to flat swaps which
/// arc each role addresses while the rim's own name does not change.
/// Stated because a consumer stores these: the boundary is where a
/// stored reference silently retargets rather than failing.
///
/// # Why this exists beside `editor_core::names::RimSupport`
///
/// The two are deliberate twins, and the emitter's identity match
/// between them is the SEAM, not redundancy. This one is a kernel
/// birth record: arena keys, no serde, free to change with the
/// surgery. Its twin is a persisted, VERSIONED name alphabet whose
/// spelling is file data and cannot move without a schema break. G1
/// layering forbids the kernel depending on editor-core, and merging
/// them would either drag serde and a schema version into the kernel
/// or let a surgery refactor silently re-spell every saved document.
/// The seam is where a break is absorbed: a rename on this side that
/// the emitter still maps needs no version bump.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RimSide {
    /// The HOST support: the planar one wherever the rim has one, and
    /// otherwise the link's own `face_a` side.
    Host,
    /// The MATE support: the other side of the same rim.
    Mate,
}

/// **The host rule, in the one place it lives**: of a rim's two
/// supports the host is the PLANAR one when exactly one of them is
/// planar, and otherwise the link's own FIRST slot. Answers `true`
/// when the second support is the host.
///
/// It takes the two planarity verdicts rather than the supports
/// themselves, because the surgery asks this question holding
/// different things at each site — the one-link arm holds `FaceKey`s
/// and the seam-split arm holds `SurfaceKey`s — and the rule is about
/// neither. Both call here, so the rule has one reading.
///
/// This decides a PERSISTED name ([`RimSide`] is what the emitter
/// turns into `editor_core::names::RimSupport`), which is why it is
/// worth a home: changing it re-points every stored reference to a
/// band trimline, silently, at the planarity boundary.
pub fn second_support_is_host(first_planar: bool, second_planar: bool) -> bool {
    second_planar && !first_planar
}

/// The source keys the blend retired.
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

/// **Per-entity birth records for one edge blend** — a fillet's or a
/// chamfer's, whichever verb ran the surgery. Every field is
/// `(minted key, the source entity it was minted for, …)`, in the
/// deterministic order the constructor visited them (D9).
///
/// A request whose chains are all open fills `blends`, `corners`,
/// `trims`, `feet`, `arcs` and `dead`, leaving every rim field empty;
/// a closed (rim) chain fills the rim phase as well.
#[derive(Clone, Debug, Default)]
pub struct BlendNaming {
    // ---- The blank phase (open plane–plane chains). ----
    /// Blend face ← the source edge it replaces (the fillet's rolling
    /// band, or the chamfer's ruled strip).
    pub blends: Vec<(FaceKey, EdgeKey)>,
    /// Corner face ← the source (trivalent, sharp) vertex it
    /// replaces: the fillet's sphere octant, or the chamfer's flat
    /// triangular patch.
    pub corners: Vec<(FaceKey, VertexKey)>,
    /// Trimline edge ← (the source edge it parallels, the support
    /// face it lies in).
    pub trims: Vec<(EdgeKey, EdgeKey, FaceKey)>,
    /// Foot vertex ← (the source corner vertex it retracts from, the
    /// support face it lies in).
    pub feet: Vec<(VertexKey, VertexKey, FaceKey)>,
    /// Corner boundary edge ← (the source corner vertex, the source
    /// edge whose blend it bounds): the fillet's corner ARC, or the
    /// chamfer's straight chord — the row names the role, not the
    /// carrier shape.
    pub arcs: Vec<(EdgeKey, VertexKey, EdgeKey)>,

    // ---- The rim phase (closed chains). ----
    /// Torus band face ← the closed chain's source edges, sorted.
    pub bands: Vec<(FaceKey, Vec<EdgeKey>)>,
    /// Rim trim arc ← (the source rim edge it replaces, which support
    /// it lies on).
    pub rim_trims: Vec<(EdgeKey, EdgeKey, RimSide)>,
    /// Rim foot vertex ← the source rim vertex it retracts from, on
    /// the HOST support (planar wherever the rim has one, and a rim
    /// between two curved walls still mints these).
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

    /// What the blend retired from the source.
    pub dead: Retired,
}
