//! **Constant-radius rolling-ball fillets** (CURVED-DESIGN C8, OQ6;
//! M5 PR 12): the validity-predicate battery, the analytic blend
//! arms, and the typed refusal vocabulary for everything else.
//!
//! # The banked principle IS the API
//!
//! C8's ratified order is binding and it is the whole design: fillet
//! validity is a set of **named margined predicates over the
//! INPUTS**, evaluated BEFORE any construction. Not a post-hoc check
//! on a blend that was already built, not an assertion inside the
//! constructor — a battery that runs first, on the body and the
//! radius the caller handed in, and refuses typed with the offending
//! margin as payload. That ordering is what lets M6 certify fillet
//! validity over a whole parameter box (the banked payoff): the
//! predicates are Q1 trileans through `k_stats` from birth, so an
//! interval replay answers "valid for every radius in this box"
//! without ever constructing a surface.
//!
//! The six, in C8's order — see [`battery`] for each one's margin,
//! its units, and its lever arm:
//!
//! 1. [`battery::radius_headroom`] — `fillet3_radius_headroom`
//! 2. [`battery::face_clearance`] — `fillet3_face_clearance`
//! 3. [`battery::spine_regularity`] — `fillet3_spine_regularity`
//! 4. [`battery::chain_g1`] — `fillet3_chain_g1`
//! 5. [`battery::convexity_at`] — `fillet3_convexity_sign`
//! 6. [`battery::corner_config`] — `fillet3_corner_independence`
//!
//! # Why `fillet3_*` and not S2's `fillet_*`
//!
//! S2's seven `fillet_*` predicates are a **profile-plane**
//! corner-fitting family: their margins are 2-D leg setbacks and
//! offset-carrier intersections over a profile loop, and
//! `ProfileError`'s escalated arm dispatches its recourse sentence by
//! matching `Indeterminate::predicate` against those exact names.
//! This unit's predicates measure different quantities (3-D normal
//! curvature headroom, face extents, spine curvature, dihedral
//! convexity sign, trihedral independence) over a different input
//! (a `Body`'s edges). Sharing the prefix would (a) route a 3-D
//! escalation into a 2-D recourse sentence at the first name
//! collision, and (b) fuse two unrelated corpora in the K-report's
//! per-family buckets, which is precisely the analysis the K funnel
//! exists to keep separable. So: a NEW family, `fillet3_*`, the `3`
//! reading "the three-dimensional battery".
//!
//! # Scope (OQ6, decided at #85)
//!
//! In: closed smooth chains, and open chains terminating in the
//! three-convex-edge vertex whose corner patch is a sphere octant.
//! Out, refused typed with the OQ6 payload vocabulary: every other
//! corner configuration ([`FilletError::FilletCornerUnsupported`]), and every
//! chain whose spine is not a line or a circle
//! ([`FilletError::SpineUnsupported`] — the canal-surface
//! approximating-blend lane, banked as its own reviewed unit).

pub mod battery;
pub mod blend;
pub mod build;
pub mod naming;
pub mod surgery;

use core::fmt;

use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Sign};
use topo::{EdgeKey, FaceKey, VertexKey};

pub use battery::{BatteryVerdict, ChainClosure, Convexity, FilletRequest, Link, run_battery};
pub use blend::{BlendArm, CornerBall, EdgeBlend, RimBlend};
pub use build::{Filleted, fillet_edges};
pub use naming::{FilletNaming, RimSide};

/// The one classification funnel of this module (the crate pattern):
/// delegates to [`geom_core::k_stats::decide`], which names the
/// predicate for the margin-telemetry recorder, classifies through
/// the sanctioned [`Decide`] door, and tags any escalation.
pub(crate) fn decide<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    geom_core::k_stats::decide(name, margin, band)
}

/// Where a fillet escalation happened — the payload half of the
/// two-tolerance shape (D4 ¶1 addendum): one message and one recourse
/// per user situation, margins riding along as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilletSite {
    /// At one link of the chain (its edge names the site).
    Link {
        /// The link's edge.
        edge: EdgeKey,
    },
    /// At one junction between consecutive links.
    Joint {
        /// The junction vertex.
        vertex: VertexKey,
    },
    /// Over the chain as a whole (closure, spine).
    Chain,
}

/// The **run-out policy vocabulary** (OQ6, decided by Evan at #85) —
/// refusal-payload names ONLY. Neither variant has a constructor
/// surface anywhere in the kernel: they exist so a refusal can name
/// the front door that does not exist yet (the `FullRevolveHoles`
/// precedent), and so the post-M5 unit that implements run-outs
/// inherits a vocabulary Evan already owns rather than inventing one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunOutPolicy {
    /// The blend runs at full radius all the way to the vertex and a
    /// corner patch fills the junction. The three-convex-edge case of
    /// this policy — the sphere octant — is the ONE configuration M5
    /// ships; every other vertex valence or convexity mix names this
    /// policy as what would handle it.
    RunOutStopAtVertex,
    /// The radius decays to zero before the vertex and the blend
    /// fades back into the sharp edge. No variable-radius machinery
    /// exists at M5, so this policy is named and never taken.
    RunOutFeather,
}

impl fmt::Display for RunOutPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RunOutStopAtVertex => write!(f, "stop-at-vertex with a corner patch"),
            Self::RunOutFeather => write!(f, "feather the radius out before the vertex"),
        }
    }
}

/// The corner-configuration tags C8's scope box enumerates. Exactly
/// one — [`CornerConfig::ThreeConvexEdges`] with independent support
/// normals — is constructible at M5; the rest are the refusal
/// taxonomy, each pinned by a fixture that reaches it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CornerConfig {
    /// Three edges, all definitely convex, support normals definitely
    /// independent: the sphere-octant corner patch. IN SCOPE.
    ThreeConvexEdges,
    /// A vertex of valence other than three. The rolling ball's
    /// contact set is not a spherical triangle there, so the corner
    /// patch is not one sphere octant.
    NEdgeVertex {
        /// The vertex's edge valence as found.
        valence: usize,
    },
    /// Three edges, but their convexity signs do not agree: the
    /// rolling ball would have to change sides mid-corner.
    MixedConvexity {
        /// How many of the three edges classified convex.
        convex: usize,
    },
    /// Three edges, all convex, but the support normals are
    /// definitely dependent (a flat or degenerate trihedron): the
    /// ball centre is not determined by the three distance
    /// conditions.
    DependentNormals,
    /// A vertex reached with an in-band or poisoned configuration
    /// margin — the configuration could not be classified at all.
    Indeterminate,
}

impl fmt::Display for CornerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThreeConvexEdges => write!(f, "three convex edges (the sphere-octant corner)"),
            Self::NEdgeVertex { valence } => write!(f, "a valence-{valence} vertex"),
            Self::MixedConvexity { convex } => {
                write!(f, "a mixed-convexity vertex ({convex} of 3 edges convex)")
            }
            Self::DependentNormals => write!(f, "a trihedron with dependent support normals"),
            Self::Indeterminate => write!(f, "a vertex whose configuration did not classify"),
        }
    }
}

/// The recourse sentence for every radius/curvature situation (D4 ¶1
/// addendum: one recourse per user situation, shared by the definite
/// and escalated arms so the text can never drift apart).
pub const FILLET3_RADIUS_RECOURSE: &str =
    "reduce the fillet radius, or blend a support with more curvature headroom";
/// The recourse for a support face whose survival the clearance screen
/// cannot certify.
pub const FILLET3_CLEARANCE_RECOURSE: &str =
    "reduce the fillet radius, or enlarge the support face whose clearance is uncertified";
/// The recourse for an edge whose supports are tangent — there is no
/// wedge to blend, at any radius.
pub const FILLET3_TANGENTIAL_RECOURSE: &str = "blend an edge whose supports meet at a definite angle; a tangential join has no wedge \
     for a rolling ball at any radius";
/// The recourse for a spine the rolling ball's own envelope folds on.
pub const FILLET3_SPINE_RECOURSE: &str =
    "reduce the fillet radius below the spine's own curvature radius";
/// The recourse for a chain that is not G1 (closed) / not classified
/// (open).
pub const FILLET3_CHAIN_RECOURSE: &str =
    "supply a tangent-continuous chain, or split the request at the tangent break";
/// The recourse for a convexity sign flip along a chain.
pub const FILLET3_CONVEXITY_RECOURSE: &str =
    "split the chain at the convexity flip and fillet each run separately";
/// The recourse for a corner the octant patch does not cover — it
/// names the run-out front door that does not exist yet.
pub const FILLET3_CORNER_RECOURSE: &str = "fillet a chain that terminates in a three-convex-edge vertex; general run-outs \
     are not implemented";
/// The recourse for an assembly request outside the front door — the
/// in-place composition surgery. What remains outside is named per
/// refusal; the remainder is junction carry-through, run-outs at
/// partially-requested corners, and concave (material-adding)
/// blends, which are not implemented.
pub const FILLET3_ASSEMBLY_RECOURSE: &str = "fillet a set of edges whose open chains are single convex plane\u{2013}plane links \
     ending at fully-requested trivalent corners and whose closed chains are \
     circular plane\u{2013}sphere rims; junction carry-through, run-outs and concave \
     blends are not implemented";
/// The recourse for a ring the blend's trimline would consume (the
/// surgery's ring carry-through check).
pub const FILLET3_RING_RECOURSE: &str =
    "reduce the fillet radius, or move the feature whose ring sits inside the blend's setback";
/// The recourse for a general spine — it names the banked unit.
pub const FILLET3_SPINE_KIND_RECOURSE: &str = "use a chain whose rolling-ball spine is a line or a circle; general spines need \
     the canal-surface approximating blend, which is not implemented";

/// A fillet refusal. Closed enum, D3 style. Every variant is either a
/// battery verdict (refused BEFORE construction — the whole point) or
/// a frontier that names the front door that does not exist yet.
#[derive(Clone, Debug)]
pub enum FilletError {
    /// The run's tolerance did not yield a valid band.
    Band(BandError),
    /// The chain's edges do not form a connected path, or an edge is
    /// not in the body at all — a structural precondition, checked
    /// before any margin.
    ChainNotConnected {
        /// The edge at which the walk broke.
        edge: EdgeKey,
    },
    /// **Predicate 1**: the rolling ball is definitely too big for a
    /// support's normal curvature along the chain — the blend would
    /// interfere with the very surface it blends.
    RadiusHeadroom {
        /// The support face whose curvature ran out.
        face: FaceKey,
        /// `(1 − r·κ_max)·r`, meters (the headroom at lever arm `r`).
        margin: f64,
        /// The blend radius, meters — the lever arm.
        radius: f64,
    },
    /// **Predicate 2**: two boundary features of a support face are
    /// closer than their two blends' setbacks, so this screen cannot
    /// certify that the face survives.
    ///
    /// **This is a screen, not a verdict** (fix pass F1). It is
    /// conservative BY DIRECTION: it compares a straight-line gap
    /// against two setbacks that in general eat along different
    /// directions, so a face whose boundary edges meet at an angle can
    /// be refused here while a direction-aware test would admit it.
    /// The reviewer's witness is a unit hexagonal prism, which this
    /// refuses from `r = 0.5` although its cap survives to the apothem
    /// `0.866` (pinned in `review_pr12_probes.rs`). The screen is kept
    /// because it never goes the other way — it cannot pass a request
    /// whose face really is consumed, which is the direction the
    /// ordering claim depends on — and the error says what it tests
    /// rather than asserting the stronger fact.
    FaceClearanceUncertified {
        /// The face whose survival is uncertified.
        face: FaceKey,
        /// `gap − setback_here − setback_there`, meters.
        margin: f64,
        /// The straight-line gap between the two boundary features,
        /// meters.
        gap: f64,
    },
    /// **Predicate 5, the zero arm**: the two supports share a tangent
    /// plane along the edge, so the dihedral has no wedge and there is
    /// no side for a rolling ball to sit in. Distinct from
    /// [`FilletError::ConvexitySignFlip`] (fix pass F6): a tangential
    /// edge does not disagree with the chain's convexity, it HAS none.
    TangentialEdge {
        /// The edge with no wedge.
        edge: EdgeKey,
        /// `((n_a × n_b)·τ̂)·arm`, meters — definitely zero here.
        margin: f64,
    },
    /// **Predicate 3**: the spine (the rolling-ball centre locus, an
    /// offset locus) folds on itself at this radius.
    SpineIrregular {
        /// `(1 − r·κ_spine)·r`, meters.
        margin: f64,
        /// The blend radius, meters — the lever arm.
        radius: f64,
    },
    /// **Predicate 4**: consecutive links do not meet tangentially, so
    /// no constant-radius spine runs through the junction.
    ChainNotG1 {
        /// The junction vertex.
        vertex: VertexKey,
        /// `sin θ · arm`, meters.
        margin: f64,
        /// The folded lever arm, meters.
        arm: f64,
    },
    /// **Predicate 5**: the dihedral's convexity sign is not constant
    /// along the chain (or an edge is tangential, with no side to
    /// roll on).
    ConvexitySignFlip {
        /// The edge whose sign disagrees with the chain's.
        edge: EdgeKey,
        /// `((n₁ × n₂)·τ̂)·arm`, meters; positive = convex.
        margin: f64,
        /// The chain's own convexity, as established by its first
        /// definitely-classified link.
        chain: Convexity,
    },
    /// **Predicate 6** and the OQ6 refusal vocabulary: the corner
    /// configuration at a chain termination is not the sphere-octant
    /// case.
    FilletCornerUnsupported {
        /// The vertex whose configuration is out of scope.
        vertex: VertexKey,
        /// Which configuration was found.
        corner: CornerConfig,
        /// The run-out policy that WOULD handle it.
        policy: RunOutPolicy,
    },
    /// The chain's rolling-ball spine is neither a line nor a circle:
    /// the blend is a canal surface, the kernel's first approximating
    /// SURFACE, banked as its own reviewed unit.
    SpineUnsupported {
        /// The link whose supports force a general spine.
        edge: EdgeKey,
        /// The support pair, as text (the honest blocker).
        supports: &'static str,
    },
    /// A margin landed in the band, or was poisoned: the same user
    /// situation as the definite arm above it, reported with the
    /// margin as data (two-tolerance, D4 ¶1 addendum).
    Escalated {
        /// Where.
        site: FilletSite,
        /// The margin diagnosis and the predicate that produced it.
        source: Indeterminate,
    },
    /// The battery passed, but the request is outside the assembly
    /// front door: the in-place composition surgery ([`surgery`] —
    /// edge sets whose open chains end at fully-requested trivalent
    /// corners, plus circular plane–sphere rim chains). The `detail`
    /// names exactly which remaining gap was hit (junction
    /// carry-through, run-outs, concave blends, non-circle rims —
    /// each a front door that does not exist yet, the
    /// `FullRevolveHoles` precedent).
    AssemblyUnsupported {
        /// What about the request put it outside the front door.
        detail: &'static str,
    },
    /// **The surgery's ring carry-through check**
    /// (`fillet3_ring_clearance`): a ring of a support face sits
    /// within (or in band of) a blend trimline, so splitting the face
    /// along that trimline would consume the ring's feature instead
    /// of carrying it through. Exact closed form (circle-vs-line /
    /// circle-vs-circle), never sampled.
    RingClearance {
        /// The support face whose ring is too close.
        face: FaceKey,
        /// The clearance margin, meters (negative or zero here).
        margin: f64,
    },
    /// The blend geometry could not be certified as stored — a
    /// carrier/surface pair outside the jet certificate's lane, or a
    /// residual over ε.
    Certify {
        /// The underlying certification refusal, as text.
        detail: String,
    },
    /// An Euler operator refused during assembly.
    Op {
        /// The underlying operator refusal, as text.
        detail: String,
    },
}

impl From<BandError> for FilletError {
    fn from(source: BandError) -> Self {
        Self::Band(source)
    }
}

impl fmt::Display for FilletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band(e) => write!(f, "fillet: {e}"),
            Self::ChainNotConnected { edge } => write!(
                f,
                "fillet chain: the edge sequence is not a connected path at {edge:?} — \
                 {FILLET3_CHAIN_RECOURSE}"
            ),
            Self::RadiusHeadroom {
                face,
                margin,
                radius,
            } => write!(
                f,
                "fillet: radius {radius} m exceeds the curvature headroom of support \
                 {face:?} — margin {margin} m at lever arm {radius} m; \
                 {FILLET3_RADIUS_RECOURSE}"
            ),
            Self::FaceClearanceUncertified { face, margin, gap } => write!(
                f,
                "fillet: the clearance screen cannot certify that support face {face:?} \
                 survives — two of its boundary features are {gap} m apart and their \
                 blends set back further than that, margin {margin} m. The screen is \
                 conservative by direction and does not assert the face IS consumed; \
                 {FILLET3_CLEARANCE_RECOURSE}"
            ),
            Self::TangentialEdge { edge, margin } => write!(
                f,
                "fillet: edge {edge:?} joins its supports tangentially — the dihedral has \
                 no wedge (margin {margin} m); {FILLET3_TANGENTIAL_RECOURSE}"
            ),
            Self::SpineIrregular { margin, radius } => write!(
                f,
                "fillet: the rolling-ball spine folds at radius {radius} m — margin \
                 {margin} m at lever arm {radius} m; {FILLET3_SPINE_RECOURSE}"
            ),
            Self::ChainNotG1 {
                vertex,
                margin,
                arm,
            } => write!(
                f,
                "fillet chain: the links at {vertex:?} are not tangent-continuous — \
                 margin {margin} m at lever arm {arm} m; {FILLET3_CHAIN_RECOURSE}"
            ),
            Self::ConvexitySignFlip {
                edge,
                margin,
                chain,
            } => write!(
                f,
                "fillet chain: edge {edge:?} is not {chain} like the rest of the chain \
                 — margin {margin} m; {FILLET3_CONVEXITY_RECOURSE}"
            ),
            Self::FilletCornerUnsupported {
                vertex,
                corner,
                policy,
            } => write!(
                f,
                "fillet corner: {vertex:?} is {corner}, which only a run-out policy \
                 would handle ({policy}) — {FILLET3_CORNER_RECOURSE}"
            ),
            Self::SpineUnsupported { edge, supports } => write!(
                f,
                "fillet: the {supports} support pair at edge {edge:?} gives a general \
                 rolling-ball spine — {FILLET3_SPINE_KIND_RECOURSE}"
            ),
            Self::Escalated { site, source } => {
                let recourse = match source.predicate {
                    Some("fillet3_radius_headroom") => FILLET3_RADIUS_RECOURSE,
                    Some("fillet3_face_clearance") => FILLET3_CLEARANCE_RECOURSE,
                    Some("fillet3_spine_regularity") => FILLET3_SPINE_RECOURSE,
                    Some("fillet3_chain_g1" | "fillet3_chain_arm") => FILLET3_CHAIN_RECOURSE,
                    Some("fillet3_convexity_sign") => FILLET3_CONVEXITY_RECOURSE,
                    Some("fillet3_ring_clearance") => FILLET3_RING_RECOURSE,
                    Some("fillet3_corner_independence") => FILLET3_CORNER_RECOURSE,
                    // Fix pass F6: an escalation from a predicate this
                    // match does not know is a MISSING recourse, and
                    // saying so is the honest answer — emitting the
                    // radius sentence would hand the user an action
                    // that has nothing to do with what escalated.
                    other => {
                        return write!(
                            f,
                            "fillet at {site:?}: {source} — no recourse is recorded for \
                             predicate {other:?}; this is a gap in the error table, not \
                             advice to act on"
                        );
                    }
                };
                write!(f, "fillet at {site:?}: {source} — {recourse}")
            }
            Self::AssemblyUnsupported { detail } => {
                write!(f, "fillet assembly: {detail} — {FILLET3_ASSEMBLY_RECOURSE}")
            }
            Self::RingClearance { face, margin } => write!(
                f,
                "fillet surgery: a ring of support face {face:?} sits within a blend's \
                 trimline — margin {margin} m; {FILLET3_RING_RECOURSE}"
            ),
            Self::Certify { detail } => write!(f, "fillet: blend geometry uncertified — {detail}"),
            Self::Op { detail } => write!(f, "fillet: assembly refused — {detail}"),
        }
    }
}

impl core::error::Error for FilletError {}
