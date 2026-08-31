//! The N1 role vocabulary: [`StableName`], [`RolePath`], the closed
//! per-op [`RoleSeg`] enum, and the N2 [`Qualifier`] verdicts.
//!
//! # Organization (spec D2, reported choice)
//!
//! ONE closed enum with op-grouped variants, not per-op enums
//! composed: role arguments are themselves names (N1's composition),
//! so a heterogeneous `Vec<RoleSeg>` is the natural spine and a
//! single enum keeps it flat. Variants are grouped and documented by
//! op; each group is versioned with its op's contract.
//!
//! # Kind tagging (spec D1, reported choice)
//!
//! A RUNTIME kind tag ([`EntityKind`] field), not phantom typing:
//! `Declare` pairs, table keys, and (PR 4) hit-test returns all need
//! kind-heterogeneous collections, and the F3 serialization story
//! wants one concrete type. Kind agreement is enforced at emission
//! (the table refuses a name whose kind disagrees with its entity).
//!
//! # Locators (spec D2, cited)
//!
//! [`ProfileEdgeRef`]/[`ProfileVertexRef`] carry the profile's OWN
//! canonical combinatorial identity — `profile::ValidatedProfile`'s
//! loop order (outer first, then holes in the DESCRIPTION's order —
//! recipe data) and each loop's canonical chain indices, whose
//! canonical start is selected through the exact-order band
//! (`canonical_order_x`/`_y`, `crates/profile/src/validate.rs`):
//! total, rotation-invariant, and a function of recipe structure plus
//! recorded verdicts — NOT bare enumeration indices. The sweep
//! emitters (`Extruded`, `Revolved`) index their output maps by
//! exactly these identities, which is what makes sweep naming a
//! mechanical zip.

use crate::node::RecipeNodeId;

/// Entity kinds a [`StableName`] can denote (N1: bodies are
/// first-class alongside faces/edges/vertices, Q-h).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum EntityKind {
    /// A whole body.
    Body,
    /// A face.
    Face,
    /// An edge.
    Edge,
    /// A vertex.
    Vertex,
}

impl EntityKind {
    /// The kind as a prose noun — the one spelling every user-facing
    /// message uses, so a rendered kind never leans on `Debug`.
    pub(crate) fn noun(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::Face => "face",
            Self::Edge => "edge",
            Self::Vertex => "vertex",
        }
    }
}

/// N1's stable name: a derivation path — the minting node plus an
/// op-typed role path. Float-free and arena-key-free by construction;
/// serialization is structural (F3, PR 6).
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct StableName {
    /// The entity kind this name denotes (N1's `K`, runtime-tagged —
    /// module docs).
    pub kind: EntityKind,
    /// The recipe node whose operation minted the entity (for
    /// pass-through ops — Transform, split-intact entities — the
    /// ORIGINAL minting node: those ops contribute no segment).
    pub node: RecipeNodeId,
    /// The role path within that operation.
    pub path: RolePath,
}

/// A sequence of role segments (N1). Usually length 1; composition
/// (`[FromA(..), Fragment(..)]`) grows it.
pub type RolePath = Vec<RoleSeg>;

/// An extrude/revolve cap end.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum CapEnd {
    /// On the sketch plane translated by the extrusion vector.
    Top,
    /// On the sketch plane.
    Bottom,
}

/// A profile edge (segment) by canonical combinatorial identity
/// (module docs: the profile crate's canonical form, cited — not a
/// bare index).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct ProfileEdgeRef {
    /// Canonical loop: 0 = outer, then holes in description order.
    pub loop_index: u32,
    /// Canonical segment index within the loop's chain.
    pub segment: u32,
}

/// A profile vertex by canonical combinatorial identity: vertex `v`
/// starts segment `v` of its loop's canonical chain.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct ProfileVertexRef {
    /// Canonical loop index.
    pub loop_index: u32,
    /// Canonical vertex index (the start vertex of segment `vertex`).
    pub vertex: u32,
}

/// Which meridian of a revolve (the M2 band/pole/seam taxonomy).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum MeridianEnd {
    /// Partial: the start-cap side (on the sketch plane).
    Start,
    /// Partial: the end-cap side (sketch plane rotated by θ).
    End,
    /// Full: the `u = 0` seam chain (F13 seam role).
    Seam,
    /// Full, wire case only: the angle-π copies (not a seam).
    Pi,
}

/// A split output half, by the tool plane's orientation (the plane's
/// normal side is `Above` — recipe-covariant: the tool is the split
/// node's own input).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum SplitHalf {
    /// Material on the tool plane's normal side.
    Above,
    /// Material on the opposite side.
    Below,
}

/// A recorded side-of verdict (N2: a margined predicate's SIGN, never
/// a value). `Mixed` aggregates a fragment whose probe vertices sit
/// definitely on both sides (wrap-around fragments) — still a
/// verdict vector entry, still flip-localized (it changes only when
/// a vertex's side verdict flips).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum SideVerdict {
    /// Every off-plane probe decided positive.
    Positive,
    /// Every off-plane probe decided negative.
    Negative,
    /// Definite probes on both sides.
    Mixed,
    /// Every probe coincident with the carrier (the fragment lies in
    /// it).
    On,
}

/// An N2 fragment discriminator: covariant margined predicate
/// verdicts against recipe-covariant references. NO values, NO bare
/// indices — `OrderAlong.rank` is an ordinal under the named
/// order-along comparison (N2's sanctioned order-along(oriented
/// parent carrier)), which changes only at a recorded flip.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum Qualifier {
    /// Sign vector of side-of(partner's oriented carrier plane), one
    /// entry per partner, sorted by partner name (`name_frag_side_of`
    /// through `k_stats`). Partners are the cutting entities' names —
    /// recipe-covariant by construction.
    ///
    /// "Oriented" means **outward**-oriented: a partner face's
    /// reference plane takes its normal from the face's material side
    /// — `topo::Face::sense_sign()` times the stored chart normal (M5
    /// S10). The verdicts are signs against that plane, so the
    /// orientation sense is part of the geometry these names are
    /// covariant with; see `emit_topo::face_plane`.
    SideOf(Vec<(StableName, SideVerdict)>),
    /// Ordinal position under order-along(oriented parent carrier)
    /// (`name_frag_order_along` through `k_stats`): rank `rank` of
    /// `of` fragments, ordered along the parent's own oriented
    /// carrier (edge direction; for face fragments of a split, the
    /// section line oriented by n_face × n_tool).
    ///
    /// The carrier's orientation is load-bearing — reversing it
    /// reverses every rank — so where it is built from face normals
    /// those are **outward** normals (M5 S10, `emit_topo::face_plane`),
    /// never raw chart normals.
    OrderAlong {
        /// This fragment's rank (0-based) along the carrier.
        rank: u32,
        /// How many sibling fragments the ordering ranked.
        of: u32,
    },
}

/// Which support of a rim blend an entity lies on (M6-5). A pair of
/// structural ROLES, not a geometric classification: the surgery knows
/// which support is which from the chain it resolved, and a rim
/// between two supports of the SAME kind — two cones meeting at a
/// latitude circle — has no kind that tells them apart.
///
/// [`RimSupport::Host`] is the PLANAR support wherever the rim has
/// one, so a caller selecting the flat side of a plane–sphere rim
/// selects the host; a rim whose supports both curve takes the
/// resolved link's own first side.
///
/// **The planarity boundary is where this vocabulary is NOT
/// covariant, and it is load-bearing enough to state here.** Because
/// the host is defined by planarity, an edit that carries a support
/// ACROSS planarity re-decides which arc each role addresses, while
/// the rim's own name and the selection naming it stay word for word
/// the same. Every edit that does not cross that boundary leaves both
/// roles fixed. The instability is INHERITED, not introduced — the
/// retired kind vocabulary moved on exactly the same edits — but it is
/// worse in one respect and the record should say so: a renamed KIND
/// makes a stored selection stop resolving, loudly, where a swapped
/// ROLE silently retargets it to the other arc of the same rim.
/// Pinned by `blend5_r1_probes` and `blend5_r2_probes`.
///
/// # The kernel twin, and why this is not it
///
/// `sweep::fillet::naming::RimSide` is the same two roles, recorded by
/// the surgery as it carves; `names::emit_fillet` maps one onto the
/// other by an identity match. The duplication is deliberate and the
/// emitter's match is the SEAM.
///
/// This side is what a file remembers: persisted and VERSIONED, so its
/// spelling is file data and cannot move without a schema break (v18
/// is that break). The kernel side is a birth record of arena keys
/// with no serde and no version, free to be re-spelled with the
/// surgery. Collapsing them would either drag serde and a schema
/// number down into the kernel — which G1 layering forbids the other
/// way round too, since the kernel must not depend on `editor-core` —
/// or let a surgery refactor silently re-spell every saved document.
/// With the seam, a rename on the kernel side that the emitter still
/// maps costs no version bump at all. The fuller statement lives at
/// `RimSide`'s own declaration.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
// INERT as it stands, and kept deliberately: an externally-tagged enum
// of unit-only variants already rejects an unknown variant name
// unconditionally, so this attribute guards only a FUTURE variant that
// carries fields. Its siblings above carry it for the same reason;
// issue #1308 owns the workspace-wide disposition.
#[serde(deny_unknown_fields)]
pub enum RimSupport {
    /// The HOST support: the planar one wherever the rim has one (on a
    /// ladder rim, the face carrying the rim as a ring), otherwise the
    /// resolved link's own first side.
    Host,
    /// The MATE support: the other side of the same rim (on a ladder
    /// rim, the cap the rim bounds).
    Mate,
}

/// One op-typed role segment (N1; closed enum, spec D2). Grouped by
/// op; each group versions with its op's contract.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum RoleSeg {
    // ---- Shared ----
    /// The single output body of a body-producing op (extrude,
    /// revolve, boolean). Q-h: a body's name is the node + the role
    /// that minted it.
    OutputBody,

    // ---- Extrude ----
    /// A cap face.
    Cap(CapEnd),
    /// A side-wall face swept from a profile segment.
    Lateral(ProfileEdgeRef),
    /// A cap–wall rim edge (cap end × profile segment).
    RimEdge(CapEnd, ProfileEdgeRef),
    /// A strut (join) edge swept from a profile vertex.
    LateralEdge(ProfileVertexRef),
    /// A cap vertex over a profile vertex.
    CapVertex(CapEnd, ProfileVertexRef),

    // ---- Revolve (M2 band/pole/seam taxonomy) ----
    /// A wall (band) face swept from a profile segment.
    Band(ProfileEdgeRef),
    /// A latitude (rim) edge at a profile vertex (partial: wedge
    /// arcs; full: full-period rims).
    BandRim(ProfileVertexRef),
    /// Full, wire case: the π…2π band's latitude half-rim.
    BandRimPi(ProfileVertexRef),
    /// Full, wire case: the π…2π band's wall face.
    BandPi(ProfileEdgeRef),
    /// A meridian edge (per profile segment, per meridian).
    Meridian(MeridianEnd, ProfileEdgeRef),
    /// A meridian vertex: the copy of a profile vertex on a wedge
    /// cap plane (partial) or the surviving meridian vertex (full).
    MeridianVertex(MeridianEnd, ProfileVertexRef),
    /// A wedge cap face (partial revolve; `Start`/`End` only).
    RevolveCap(MeridianEnd),
    /// An on-axis (pole) vertex at a profile vertex on the axis.
    Pole(ProfileVertexRef),
    /// The shared axis edge of an on-axis profile segment (partial).
    AxisEdge(ProfileEdgeRef),

    // ---- Booleans ----
    /// An entity surviving from operand A (argument: its name in the
    /// A operand's table).
    FromA(Box<StableName>),
    /// An entity surviving from operand B.
    FromB(Box<StableName>),
    /// A zip-minted seam entity: the crossing of an A-operand entity
    /// and a B-operand entity (edges: face × face; vertices:
    /// edge × face / face × edge), by their operand names.
    Seam {
        /// The A-side crossing entity's name.
        a: Box<StableName>,
        /// The B-side crossing entity's name.
        b: Box<StableName>,
    },
    /// An F7 merged face: the sorted set of constituent names retires
    /// into this name (N3; canonical order = name order).
    Merged(Vec<StableName>),
    /// A fragment discriminator, composed AFTER the parent-bearing
    /// segment: `[FromA(f), Fragment(q)]` reads "the q-qualified
    /// fragment of A's face f" (N2).
    Fragment(Qualifier),

    // ---- Split ----
    /// A split output body (the half is the tool plane's own
    /// orientation — covariant).
    SplitBody(SplitHalf),
    /// A section face: the split-minted face on the tool plane,
    /// bounding the `side` half; `section` is the position in section
    /// completion order — a function of the recipe + the recorded
    /// `split_join_order_*` verdicts (the exact-order sort), so
    /// covariant in N4's sense (reorderings are recorded flips).
    SectionFace {
        /// Which output half this section face bounds.
        side: SplitHalf,
        /// Position in section completion order (covariance: module
        /// docs of `topo::splitting::order` — the sort runs through
        /// named exact-order predicates).
        section: u32,
    },
    /// A section-boundary edge: where the section meets an operand
    /// face (argument: the operand face's name), on the `side` half
    /// (each half owns its own coincident copy).
    SectionEdge {
        /// Which output half.
        side: SplitHalf,
        /// The operand face the section boundary runs across.
        face: Box<StableName>,
    },
    /// A fragment of an operand entity carved by the split: faces cut
    /// by the section, edges crossing the plane. The side IS the N2
    /// discriminator (the kernel's own decided classification against
    /// the tool plane — the split node's recipe-covariant reference);
    /// same-side face multiplicity appends `Fragment(OrderAlong)`.
    SplitFragment {
        /// Which output half holds this fragment.
        side: SplitHalf,
        /// The operand entity's name.
        parent: Box<StableName>,
    },
    /// A crossing vertex minted where the tool plane crossed an
    /// operand edge (argument: the operand edge's name; each half
    /// keeps its own coincident copy — `side` names which).
    CrossingVertex {
        /// Which output half holds this copy.
        side: SplitHalf,
        /// The operand edge the plane crossed.
        edge: Box<StableName>,
    },
    /// A per-half copy of an operand vertex the tool plane passed
    /// THROUGH (review R2): both halves keep a coincident copy, so
    /// the operand name alone would alias — the side tag (the
    /// kernel's own recorded side assignment, a verdict) is the
    /// discriminator. Fully birth-derived: the operand identity via
    /// the null-pair copy row, the side via which half owns the copy.
    OnToolVertex {
        /// Which output half holds this copy.
        side: SplitHalf,
        /// The operand vertex the plane passed through.
        of: Box<StableName>,
    },

    // ---- Fillet (M6-5: the composition surgery's vocabulary) ----
    //
    // Every segment carries the SOURCE entity's OWN stable name, so a
    // fillet name composes covariantly under an upstream bump exactly
    // as the boolean emitter's `FromA`/`Seam` do: the target's names
    // move, and these move with them, without this emitter deciding
    // anything about geometry.
    /// An entity carried through from the fillet's target (argument:
    /// its name in the target's table) — a shrunk support face, an
    /// untouched edge, a far vertex. The single-operand analogue of
    /// [`RoleSeg::FromA`].
    FromTarget(Box<StableName>),
    /// The blend face rounding a source edge.
    BlendFace(Box<StableName>),
    /// The octant (sphere patch) rounding a source vertex.
    CornerFace(Box<StableName>),
    /// A trimline: where a blend meets ONE of its two supports. Both
    /// arguments are needed — one source edge yields two trimlines,
    /// discriminated by which support they lie in.
    TrimEdge {
        /// The source edge being blended.
        edge: Box<StableName>,
        /// The support face the trimline lies in.
        support: Box<StableName>,
    },
    /// A blend foot: where a support's two trimlines meet, retracted
    /// from a source corner vertex. One source vertex yields one foot
    /// per incident support.
    FootVertex {
        /// The source corner vertex.
        vertex: Box<StableName>,
        /// The support face the foot lies in.
        support: Box<StableName>,
    },
    /// A corner arc: where an octant meets one of its three incident
    /// blends.
    CornerArc {
        /// The source corner vertex the octant rounds.
        vertex: Box<StableName>,
        /// The source edge whose blend the arc bounds.
        edge: Box<StableName>,
    },
    /// The torus band face rounding a CLOSED chain (argument: the
    /// chain's source edges as a sorted set — a rim is a cycle with no
    /// distinguished first edge, so the SET is the covariant identity;
    /// the N3 [`RoleSeg::Merged`] precedent, same canonical order).
    BandFace(Vec<StableName>),
    /// A band trimline on one support (a rim edge yields one per
    /// side).
    BandTrim {
        /// The source rim edge this arc replaces.
        edge: Box<StableName>,
        /// Which support the arc lies on.
        support: RimSupport,
    },
    /// A band foot: the HOST-support vertex retracted from a source
    /// rim vertex. (Named by role, not kind: a rim between two curved
    /// walls has no planar support and still mints one — pinned by
    /// `blend5_r1_probes`.)
    BandFoot(Box<StableName>),
    /// The vertex where the band's MATE-side trimline crossed a source
    /// edge running off the rim (on a ladder rim, a cap meridian).
    BandCross(Box<StableName>),
    /// The surviving piece of a source edge the band's trimline cut
    /// (the shortened meridian).
    BandCut(Box<StableName>),
    /// A band's SLIT: the double-traversed torus meridian that keeps
    /// the annular band RING-FREE (`sweep::fillet::surgery`'s donut
    /// representation). Argument: the source edge whose severed piece
    /// became it.
    BandSlit(Box<StableName>),

    // ---- Instantiate part (ASM-2A D-4: the name bridge) ----
    /// An entity of an instantiated part's product, named under the
    /// instance (argument: its PART-LOCAL stable name).
    ///
    /// The instance's identity rides in the enclosing
    /// [`StableName::node`] — the instantiate node — and the referenced
    /// DOCUMENT's identity rides in that node's `doc_ref`, so neither is
    /// repeated here: one qualifier per instance is all the
    /// distinctness the part-local names need, exactly as
    /// [`RoleSeg::Instance`] carries a pattern's.
    InPart {
        /// The entity's name inside the referenced document's product.
        of: Box<StableName>,
    },

    // ---- Pattern ----
    /// Instance `i` of the pattern's master (i is the D8-structural
    /// index — A8/N1; `of` is the master entity's name).
    Instance {
        /// The structural instance index.
        i: u32,
        /// The master entity this instance copy corresponds to.
        of: Box<StableName>,
    },
}

/// The [`RoleSeg`] variants that embed no [`StableName`], as a
/// PATTERN rather than a predicate.
///
/// Four matches classify segments by this partition and each does
/// something different with the other half — the name walk visits,
/// the rewrite rebuilds, the selector collects, and the attribution
/// walk ([`fn@super::attribute`]) stops. Only the negative answer is
/// common, so only the negative answer is shared, and it is shared as
/// an or-pattern so that none of them loses its exhaustiveness: a
/// variant added to [`RoleSeg`] and not added here breaks every one of
/// those builds, exactly as spelling the list out at each site did.
/// What changes is that classifying it name-free is ONE decision at
/// one site instead of four that can be made differently.
///
/// A `fn` returning `bool` would not do: a caller may forget to call
/// a predicate, and the property this list carries is the one the
/// compiler holds.
macro_rules! name_free_seg {
    () => {
        $crate::names::RoleSeg::OutputBody
            | $crate::names::RoleSeg::Cap(_)
            | $crate::names::RoleSeg::Lateral(_)
            | $crate::names::RoleSeg::RimEdge(..)
            | $crate::names::RoleSeg::LateralEdge(_)
            | $crate::names::RoleSeg::CapVertex(..)
            | $crate::names::RoleSeg::Band(_)
            | $crate::names::RoleSeg::BandRim(_)
            | $crate::names::RoleSeg::BandRimPi(_)
            | $crate::names::RoleSeg::BandPi(_)
            | $crate::names::RoleSeg::Meridian(..)
            | $crate::names::RoleSeg::MeridianVertex(..)
            | $crate::names::RoleSeg::RevolveCap(_)
            | $crate::names::RoleSeg::Pole(_)
            | $crate::names::RoleSeg::AxisEdge(_)
            | $crate::names::RoleSeg::SplitBody(_)
            | $crate::names::RoleSeg::SectionFace { .. }
    };
}

pub(crate) use name_free_seg;
