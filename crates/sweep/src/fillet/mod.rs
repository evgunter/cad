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
//! Beside them sits one **routing** decision, which is not a validity
//! predicate and is named apart for that reason:
//! `fillet3_support_coaxiality`, the departure of a CURVED support pair
//! from the shared axis (or shared ruling) its arm's spine is derived
//! from, in meters at the link's own lever arm. Every other fact a
//! curved arm needs — the surface kinds, the nappe, the material side —
//! is read structurally off stored data; "these two stored axes are the
//! same axis" is the one that is placed geometry and cannot be. A
//! definite miss refuses [`FilletError::SpineUnsupported`], because a
//! pair that is not coaxial has a spine that is neither line nor circle
//! and belongs to the canal unit.
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
//! corner CONFIGURATION ([`FilletError::FilletCornerUnsupported`],
//! carrying a [`CornerConfig`] — the battery's classifier and the
//! assembly's valence and convexity doors both), and every link whose
//! support pair is outside the analytic-arm table
//! ([`FilletError::SpineUnsupported`] — the canal-surface
//! approximating-blend lane, banked as its own reviewed unit). A corner whose configuration is the supported one
//! but whose edges are not all requested is a **run-out**, which is
//! about the request rather than the configuration and refuses as
//! [`FilletError::UnsupportedRunOut`].

mod admit;
pub mod battery;
pub mod blend;
pub mod build;
pub mod naming;
pub mod surgery;

use core::fmt;

use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Sign};
use topo::{EdgeKey, EntityId, FaceKey, VertexKey};

pub use battery::{
    BatteryVerdict, ChainClosure, Convexity, FilletRequest, Link, run_battery, run_battery_for,
};
pub use blend::{BlendArm, CornerBall, EdgeBlend, RimBlend};
pub use build::{Filleted, fillet_edges};
pub use naming::{FilletNaming, RimSide};

/// **Which band a request grafts onto its edges.** The battery, the
/// admission doors and the composition surgery are shared by both
/// verbs; this is the one bit that says which one is running, and it
/// rides on the [`BatteryVerdict`] so no assembly step has to be told
/// twice.
///
/// The two differ in exactly three places, each named on the arm that
/// takes it: the analytic arm a link resolves to
/// ([`battery::run_battery_for`]), which of C8's predicates are facts
/// about the request at all (the rolling ball's radius-vs-curvature
/// headroom and spine regularity are ball facts, and a chamfer meters
/// neither), and the corner geometry the surgery grafts
/// ([`surgery`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendKind {
    /// The constant-radius rolling ball: cylinder/torus bands and
    /// sphere-octant corners. `radius` is the ball's.
    Fillet,
    /// The flat strip at equal setback along both supports, with a
    /// planar corner patch. `radius` is the SETBACK.
    Chamfer,
}

impl fmt::Display for BlendKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fillet => write!(f, "fillet"),
            Self::Chamfer => write!(f, "chamfer"),
        }
    }
}

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
/// the front door that does not exist yet (the standing frontier
/// error-text pattern), and so the post-M5 unit that implements run-outs
/// inherits a vocabulary Evan already owns rather than inventing one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunOutPolicy {
    /// The blend runs at full radius all the way to the vertex and a
    /// corner patch fills the junction. The three-convex-edge case of
    /// this policy — the sphere octant — is the ONE configuration that
    /// ships.
    ///
    /// It is the policy MOST out-of-scope corners name, but not all of
    /// them, and the exceptions are the interesting ones
    /// ([`CornerConfig::policy`] is the map): a MIXED-CONVEXITY vertex
    /// names [`Self::RunOutFeather`] instead, because a corner patch
    /// cannot help where the ball changes sides; and a
    /// [`CornerConfig::SeamVertex`] names NO policy at all, because it
    /// is not a corner — the surface is smooth through it, so there is
    /// nothing for a run-out to run out into.
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
        /// The vertex's valence as found — the edge orbit at the
        /// battery and the assembly door, the face orbit at the octant
        /// derivation, which agree on a manifold body.
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
    /// A vertex where a CHART SEAM crosses an otherwise smooth rim: the
    /// two edges continuing the rim carry the same support pair, and the
    /// other two are co-surface seam meridians (one surface on both
    /// sides, so the dihedral there is zero by construction).
    ///
    /// **Not a corner**, and that is the whole content of the tag. The
    /// surface is smooth through the point — the seam is where a chart
    /// was cut, not where material turns — so there is no wedge, no
    /// ball-rest configuration distinct from the neighbouring rim
    /// points, and no run-out policy that would help. What helps is
    /// asking for the rim whole, which is a door that already exists,
    /// and that is what this tag's recourse says.
    SeamVertex,
    /// A vertex reached with an in-band or poisoned configuration
    /// margin — the configuration could not be classified at all.
    Indeterminate,
}

impl CornerConfig {
    /// **The run-out policy that would handle this configuration** —
    /// and `None` where no run-out policy would, because the
    /// configuration is not a corner at all
    /// ([`CornerConfig::SeamVertex`]).
    ///
    /// The one place the tag → policy map lives, so a refusal's payload
    /// cannot disagree with its own tag.
    #[must_use]
    pub fn policy(self) -> Option<RunOutPolicy> {
        match self {
            // A corner patch cannot help where the ball changes sides
            // mid-corner; feathering is the policy that addresses it.
            Self::MixedConvexity { .. } => Some(RunOutPolicy::RunOutFeather),
            // Not a corner: the surface is smooth through the point, so
            // there is nothing for a run-out to run out INTO.
            Self::SeamVertex => None,
            _ => Some(RunOutPolicy::RunOutStopAtVertex),
        }
    }

    /// The recourse sentence that is TRUE of this configuration. A
    /// seam vertex names the closed-rim door that exists; every other
    /// tag names the run-out door that does not.
    #[must_use]
    pub fn recourse(self) -> &'static str {
        match self {
            Self::SeamVertex => FILLET3_SEAM_VERTEX_RECOURSE,
            _ => FILLET3_CORNER_RECOURSE,
        }
    }
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
            Self::SeamVertex => write!(
                f,
                "a chart-seam vertex on a smooth rim, which is not a corner at all"
            ),
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
/// The recourse for an edge whose dihedral sign decided Zero — no
/// definite wedge side at the metered lever, at any radius.
pub const FILLET3_TANGENTIAL_RECOURSE: &str = "blend an edge whose supports meet at a definite angle; a dihedral with no definite \
     wedge side gives a rolling ball no side to sit in, at any radius";
/// The recourse for a spine the rolling ball's own envelope folds on.
pub const FILLET3_SPINE_RECOURSE: &str =
    "reduce the fillet radius below the spine's own curvature radius";
/// The recourse for a chain that is not G1 (closed) / not classified
/// (open).
pub const FILLET3_CHAIN_RECOURSE: &str = "supply a connected, tangent-continuous chain. Splitting the request at the break \
     helps only where the break is a genuine tangent break between two blendable runs; \
     where it is a CORNER, splitting leaves that corner partly requested and refuses \
     again as a run-out — request every edge of the corner instead";
/// The recourse for a convexity sign flip along a chain.
pub const FILLET3_CONVEXITY_RECOURSE: &str =
    "split the chain at the convexity flip and fillet each run separately";
/// The recourse for a corner the octant patch does not cover — it
/// names the run-out front door that does not exist yet.
pub const FILLET3_CORNER_RECOURSE: &str = "fillet a chain that terminates in a three-convex-edge vertex; general run-outs \
     are not implemented";
/// The recourse for a chain that stops at a CHART SEAM on an otherwise
/// smooth rim.
///
/// It names the REQUEST that describes what the caller wants — the rim
/// entire, which is a closed chain — rather than a run-out policy,
/// because a run-out at a smooth point is not what is missing. Whether
/// that closed chain's band is then carved is the closed-rim surgery's
/// own question and it answers it in its own words: today the ring-free
/// annulus band is a ONE-EDGE rim's, so a rim a seam has split still
/// meets that door's frontier. This sentence deliberately stops short of
/// promising the carve.
pub const FILLET3_SEAM_VERTEX_RECOURSE: &str = "request the rim whole — every arc the chart seam split it into — rather than a \
     chain that stops at the seam, which is a chart artifact the surface is smooth \
     through";
/// The recourse for a CHAIN whose shape is outside the front door of
/// the in-place composition surgery. True of exactly the chain-shape
/// refusals: what remains outside is junction carry-through, concave
/// (material-adding) blends, and rims that are not whole circular
/// plane\u{2013}sphere rings.
pub const FILLET3_ASSEMBLY_RECOURSE: &str = "fillet a set of edges whose open chains are single convex plane\u{2013}plane links \
     ending at fully-requested trivalent corners and whose closed chains are \
     circular plane\u{2013}sphere rims; junction carry-through, run-outs and concave \
     blends are not implemented";
/// The recourse for a BODY the surgery has not been built for. The
/// surgery operates in place on one solid; multi-solid and shell-less
/// bodies are a separate door.
pub const FILLET3_BODY_RECOURSE: &str = "fillet a body that is a single solid with a single shell; filleting across \
     several solids at once is not implemented";
/// The recourse for a stored geometry the surgery's closed forms do
/// not cover. Everything this unit decides is exact and stored — never
/// sampled — so a carrier outside the covered shapes refuses rather
/// than approximating.
pub const FILLET3_GEOMETRY_RECOURSE: &str = "fillet edges whose supports are planes (and, for a rim, a sphere cap) and whose \
     stored carriers are lines and circles; the surgery's exact forms cover no other \
     stored shape, and approximating one is not implemented";
/// The recourse for a ring the blend's trimline would consume (the
/// surgery's ring carry-through check).
pub const FILLET3_RING_RECOURSE: &str =
    "reduce the fillet radius, or move the feature whose ring sits inside the blend's setback";
/// The recourse for a support pair outside the analytic-arm table —
/// it names the banked unit.
pub const FILLET3_SPINE_KIND_RECOURSE: &str = "use a chain whose support pairs have analytic blend arms (plane–plane or \
     plane–sphere); other pairs need the canal-surface approximating blend, which is \
     not implemented";
/// The recourse for a CHAMFER over a support pair its one arm does not
/// cover. Its own sentence rather than the fillet's: the chamfer's
/// missing door is the curved-support strip, not the canal surface,
/// and the plane–sphere pair the fillet offers is not an alternative
/// here.
pub const CHAMFER_ARM_RECOURSE: &str = "chamfer edges whose two supports are both planes; the chamfer over a curved \
     support is not implemented";

/// A fillet refusal. Closed enum, D3 style. Every variant is one of
/// three things, and the D2 addendum row it belongs to is stated on
/// it: a battery verdict (refused BEFORE construction — the whole
/// point), a frontier naming a front door that does not exist yet
/// (row 2), or a statement that the input was invalid (row 1). A state
/// the surgery can prove impossible is not in this enum at all — it is
/// an `unreachable!` at the branch that would observe it (row 4).
///
/// # Which frontier variant a site takes
///
/// The rule is **what the branch READS**, never which noun the frame
/// happens to hold — otherwise one fact refuses two ways and ships two
/// different recourse sentences for it:
///
/// | The branch reads | Variant |
/// |---|---|
/// | the body's solid/shell inventory | [`FilletError::UnsupportedBody`] |
/// | a stored `Surface`, carrier or trimline | [`FilletError::UnsupportedGeometry`] |
/// | a corner's own valence or convexity mix | [`FilletError::FilletCornerUnsupported`] |
/// | which edges the REQUEST covers at a termination | [`FilletError::UnsupportedRunOut`] |
/// | any other property of the chain or how it sits on its supports | [`FilletError::UnsupportedChain`] |
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
    /// **Predicate 5, the undecided wedge**: the dihedral's signed
    /// margin decided Zero, so there is no definite wedge side for a
    /// rolling ball at the metered lever. Genuine tangency — the two
    /// supports sharing a tangent plane along the edge — is one cause
    /// (a co-surface seam produces it at a margin of exactly zero),
    /// not a fact this refusal establishes. Distinct from
    /// [`FilletError::ConvexitySignFlip`]: a `Zero` edge does not
    /// disagree with the chain's convexity — none was decided.
    TangentialEdge {
        /// The edge whose dihedral decided Zero.
        edge: EdgeKey,
        /// `((n_a × n_b)·τ̂)·arm`, meters — decided Zero at the
        /// metered lever.
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
    /// along the chain. (An edge whose sign decided Zero is not a
    /// flip — it refuses as [`FilletError::TangentialEdge`].)
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
        /// The run-out policy that WOULD handle it — [`None`] where
        /// none would, because the configuration is not a corner
        /// ([`CornerConfig::policy`], the tag's own map).
        policy: Option<RunOutPolicy>,
    },
    /// The link's support pair has no analytic blend arm in the
    /// battery's table (whose rows the payload's own roster names).
    /// The refusal is minted from the PAIR, not from
    /// the spine the pair would trace — a coaxial curved pair's spine
    /// can be a perfectly good circle and still land here. The
    /// general lane is the canal-surface approximating blend, banked
    /// as its own reviewed unit.
    SpineUnsupported {
        /// The link whose support pair has no analytic arm.
        edge: EdgeKey,
        /// The support pair, as text (the honest blocker).
        supports: &'static str,
    },
    /// **The CHAMFER's arm table**: the link's support pair is not
    /// plane–plane, which is the one pair the ruled strip is built
    /// over. Its own variant rather than
    /// [`FilletError::SpineUnsupported`] because the two name
    /// different missing doors — a chamfer over a curved support is
    /// VERBS-ARMS' machinery, not the canal-surface approximating
    /// blend — and one recourse per user situation is the rule
    /// (D4 ¶1 addendum).
    ChamferArmUnsupported {
        /// The link whose support pair the strip does not cover.
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
    /// The request names one edge twice, so the chain walk would
    /// double a link.
    RepeatedEdge {
        /// The edge the request repeats.
        edge: EdgeKey,
    },
    /// **The band's size is not definitely positive** (D2 addendum row
    /// 1: invalid input, checked at the door before anything resolves).
    ///
    /// A zero or negative setback is not a small chamfer, and neither
    /// is one whose bracket straddles zero: there is no band to build
    /// and no margin to meter. It is refused at the door because a
    /// nonpositive size silently LEVERS the margins that quote it —
    /// `fillet3_corner_independence`'s `|det(n₁,n₂,n₃)|·d` collapses
    /// to zero at `d = 0`, so the consumer would read "a trihedron
    /// with dependent support normals" about a cube corner whose
    /// normals are exactly orthonormal. A false fact about the BODY is
    /// worse than no diagnosis.
    ///
    /// **Not a metered predicate, deliberately.** Whether the caller
    /// handed in a positive number is a fact about the REQUEST, not a
    /// geometric quantity of the body, so it takes no `k_stats` name
    /// and no band — a K-corpus row here would meter the caller.
    NonpositiveSize {
        /// The size as handed in, meters: its bracket's low end, so a
        /// straddling or poisoned enclosure reports the end that fails.
        size: f64,
    },
    /// **Frontier** (D2 addendum row 2): the body is a shape the
    /// in-place surgery has not been built for. Valid input, unbuilt
    /// door.
    UnsupportedBody {
        /// How many solids the body holds.
        solids: usize,
        /// How many shells the body holds.
        shells: usize,
    },
    /// **Frontier** (D2 addendum row 2): a property of the requested
    /// CHAIN puts it outside the built door.
    ///
    /// Two families, and the second is not a shape of the chain in
    /// isolation: (a) the chain's own form — multi-link open chains
    /// (junction carry-through), support pairs no arm covers, concave
    /// chains, one-edge chains; and (b) how the chain sits on its
    /// supports — a rim that is not a whole ring of its plane, a
    /// sphere support carrying rings of its own or more than its own
    /// arc, a rim vertex that does not drop exactly one meridian, a
    /// half-cap arc not flanked by split points, a trimline that does
    /// not cross a meridian inside its span.
    UnsupportedChain {
        /// An edge of the chain that names the site.
        edge: EdgeKey,
        /// Which chain shape is not built.
        detail: &'static str,
    },
    /// **Frontier** (D2 addendum row 2): the REQUEST does not cover a
    /// chain termination the way the octant assembly needs — a
    /// run-out.
    ///
    /// This is deliberately *not* [`FilletError::FilletCornerUnsupported`],
    /// which is the OQ6 vocabulary for what a corner's own
    /// CONFIGURATION is (valence, convexity mix) and which every such
    /// refusal here does use. A corner whose shape is exactly the
    /// supported one, with only some of its edges requested, has no
    /// [`CornerConfig`] arm: the only one that fits is
    /// [`CornerConfig::ThreeConvexEdges`], which renders as the
    /// configuration that IS built. Minting an arm for it would extend
    /// a vocabulary decided at #85, which is a design change rather
    /// than an execution.
    UnsupportedRunOut {
        /// Where the request's coverage ran out — the terminating
        /// vertex, or the boundary edge that is not requested.
        at: EntityId,
        /// What the request does not cover.
        detail: &'static str,
    },
    /// **Frontier** (D2 addendum row 2): a stored carrier, trimline or
    /// surface is not one of the shapes the surgery's closed forms
    /// cover (a circle rim carrier, a straight open trimline, a planar
    /// support, a torus band).
    UnsupportedGeometry {
        /// The entity whose stored geometry is not covered.
        at: EntityId,
        /// Which stored shape was found instead.
        detail: &'static str,
    },
    /// **The body handed to the surgery does not hold together where
    /// the plan read it** (D2 addendum row 1): a stored reference that
    /// did not resolve, a cycle that did not close, or a verdict whose
    /// keys disagree with the body's own structure. This is not a
    /// fillet frontier and carries no fillet recourse — the input is
    /// invalid, and the surgery refuses rather than building on it.
    BodyNotIntact {
        /// The entity the plan was reading.
        at: EntityId,
        /// What the plan was reading when the reference failed.
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
    /// **The result's pcurve caches could not be re-minted** after the
    /// surgery — a chart image outside a derivation route, a loop that
    /// does not close in the chart, or a cache that fails its
    /// certification.
    ///
    /// The pass's own typed refusal is nested whole: it names the
    /// half-edge or face at fault and its own reason, which no
    /// rendering of this error has to reconstruct.
    Certify {
        /// The surgery step that ran the pass.
        site: &'static str,
        /// The pcurve pass's typed refusal.
        source: topo::PcurveMintError,
    },
    /// **An Euler operator refused during assembly.**
    ///
    /// The operator's own refusal is nested whole — `StaleKey`,
    /// `Certification`, and the rest of its vocabulary reach the caller
    /// typed rather than as prose. `site` names the surgery step that
    /// ran the operator.
    Op {
        /// The surgery step that ran the operator.
        site: &'static str,
        /// The operator's typed refusal.
        source: topo::EulerOpError,
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
                "fillet: edge {edge:?}'s dihedral has no definite wedge side — its sign \
                 decided Zero at the metered lever (margin {margin} m), as a tangential \
                 join does; {FILLET3_TANGENTIAL_RECOURSE}"
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
            Self::FilletCornerUnsupported { vertex, corner, .. } => {
                // Both halves of this sentence come from the TAG — the
                // policy it names and the recourse that is true of it —
                // so the message cannot contradict itself, and the
                // payload's `policy` cannot make it lie.
                let recourse = corner.recourse();
                match corner.policy() {
                    Some(policy) => write!(
                        f,
                        "fillet corner: {vertex:?} is {corner}, which only a run-out policy \
                         would handle ({policy}) — {recourse}"
                    ),
                    None => write!(f, "fillet corner: {vertex:?} is {corner} — {recourse}"),
                }
            }
            Self::SpineUnsupported { edge, supports } => write!(
                f,
                "fillet: the {supports} support pair at edge {edge:?} has no analytic \
                 blend arm — {FILLET3_SPINE_KIND_RECOURSE}"
            ),
            Self::ChamferArmUnsupported { edge, supports } => write!(
                f,
                "chamfer: the {supports} support pair at edge {edge:?} has no ruled \
                 strip — {CHAMFER_ARM_RECOURSE}"
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
            Self::RepeatedEdge { edge } => write!(
                f,
                "fillet: the request repeats edge {edge:?} — request each edge once; a \
                 repeated edge would double a link in the chain walk"
            ),
            Self::NonpositiveSize { size } => write!(
                f,
                "edge blend: the band size {size} m is not definitely positive — supply a \
                 positive radius or setback. A nonpositive size has no band to build, and \
                 it also levers the corner and clearance margins that quote it, so it is \
                 refused as the invalid input it is rather than reported as a fact about \
                 the body"
            ),
            Self::UnsupportedBody { solids, shells } => write!(
                f,
                "fillet assembly: the body is {solids} solid(s) and {shells} shell(s), not a \
                 single solid with a single shell — {FILLET3_BODY_RECOURSE}"
            ),
            Self::UnsupportedChain { edge, detail } => write!(
                f,
                "fillet assembly: {detail} (chain at edge {edge:?}) — \
                 {FILLET3_ASSEMBLY_RECOURSE}"
            ),
            Self::UnsupportedRunOut { at, detail } => write!(
                f,
                "fillet assembly: {detail} (at {at}) — {FILLET3_CORNER_RECOURSE}"
            ),
            Self::UnsupportedGeometry { at, detail } => write!(
                f,
                "fillet assembly: {detail} (at {at}) — {FILLET3_GEOMETRY_RECOURSE}"
            ),
            Self::BodyNotIntact { at, detail } => write!(
                f,
                "fillet surgery: {detail} — {at} did not resolve. The body handed to the \
                 surgery does not hold together there; this is invalid input, not a fillet \
                 frontier, and no fillet recourse applies"
            ),
            Self::RingClearance { face, margin } => write!(
                f,
                "fillet surgery: a ring of support face {face:?} sits within a blend's \
                 trimline — margin {margin} m; {FILLET3_RING_RECOURSE}"
            ),
            Self::Certify { site, source } => {
                write!(f, "fillet: {site} — {source}")
            }
            Self::Op { site, source } => {
                write!(f, "fillet: assembly refused at {site} — {source}")
            }
        }
    }
}

impl core::error::Error for FilletError {}

#[cfg(test)]
#[allow(clippy::panic)]
#[allow(clippy::expect_used)]
mod recourse_tests {
    use core::mem::discriminant;

    use geom_core::{Band, Indeterminate, MarginDiag};
    use topo::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};

    use super::CornerConfig;
    use super::{
        CHAMFER_ARM_RECOURSE, FILLET3_ASSEMBLY_RECOURSE, FILLET3_BODY_RECOURSE,
        FILLET3_CHAIN_RECOURSE, FILLET3_CLEARANCE_RECOURSE, FILLET3_CONVEXITY_RECOURSE,
        FILLET3_CORNER_RECOURSE, FILLET3_GEOMETRY_RECOURSE, FILLET3_RADIUS_RECOURSE,
        FILLET3_RING_RECOURSE, FILLET3_SEAM_VERTEX_RECOURSE, FILLET3_SPINE_KIND_RECOURSE,
        FILLET3_SPINE_RECOURSE, FILLET3_TANGENTIAL_RECOURSE, FilletError, FilletSite,
    };

    /// Every recourse sentence this module can append.
    const ALL: [&str; 14] = [
        CHAMFER_ARM_RECOURSE,
        FILLET3_RADIUS_RECOURSE,
        FILLET3_CLEARANCE_RECOURSE,
        FILLET3_TANGENTIAL_RECOURSE,
        FILLET3_SPINE_RECOURSE,
        FILLET3_CHAIN_RECOURSE,
        FILLET3_CONVEXITY_RECOURSE,
        FILLET3_CORNER_RECOURSE,
        FILLET3_ASSEMBLY_RECOURSE,
        FILLET3_BODY_RECOURSE,
        FILLET3_GEOMETRY_RECOURSE,
        FILLET3_RING_RECOURSE,
        FILLET3_SPINE_KIND_RECOURSE,
        FILLET3_SEAM_VERTEX_RECOURSE,
    ];

    /// What a variant's `Display` is allowed to append.
    enum Recourse {
        /// This sentence, and no other.
        Exactly(&'static str),
        /// One sentence, chosen at render time by the escalation's own
        /// predicate name — so the contract is "at most one", not
        /// "which one". (`Escalated` routes to six different
        /// constants; a table row naming one of them would be false.)
        RoutedByPredicate,
        /// None at all: the variant reports invalid input, or forwards
        /// another error's own text.
        None,
    }

    /// **The recourse contract, as one exhaustive table.**
    ///
    /// A recourse is advice, so it must be TRUE of the variant that
    /// appends it, and a variant reporting invalid input has no fillet
    /// advice to give. Each arm carries BOTH halves — the decision and
    /// a witness value of its own variant — so the two cannot drift
    /// apart by omission, which is exactly how a hand-kept witness list
    /// lets a wrong decision ship green.
    ///
    /// **What this still cannot do:** Rust cannot enumerate a type's
    /// variants, so the *seeds* below are hand-written. Adding a
    /// variant is a compile error here (the match is exhaustive, and
    /// the arm must produce a witness of that variant), but nothing
    /// in-file forces the new witness to be *rendered* until it is
    /// seeded. `tests/review_d2_recourse_at_the_site.rs` closes that at
    /// the place it matters, by reaching refusals through
    /// `fillet_edges`.
    fn contract(err: &FilletError) -> (FilletError, Recourse) {
        match err {
            FilletError::Band(_) => (err.clone(), Recourse::None),
            FilletError::ChainNotConnected { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_CHAIN_RECOURSE))
            }
            FilletError::RadiusHeadroom { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_RADIUS_RECOURSE))
            }
            FilletError::FaceClearanceUncertified { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_CLEARANCE_RECOURSE))
            }
            FilletError::TangentialEdge { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_TANGENTIAL_RECOURSE))
            }
            FilletError::SpineIrregular { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_SPINE_RECOURSE))
            }
            FilletError::ChainNotG1 { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_CHAIN_RECOURSE))
            }
            FilletError::ConvexitySignFlip { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_CONVEXITY_RECOURSE))
            }
            FilletError::FilletCornerUnsupported { corner, .. } => {
                (err.clone(), Recourse::Exactly(corner.recourse()))
            }
            FilletError::SpineUnsupported { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_SPINE_KIND_RECOURSE))
            }
            FilletError::ChamferArmUnsupported { .. } => {
                (err.clone(), Recourse::Exactly(CHAMFER_ARM_RECOURSE))
            }
            FilletError::Escalated { .. } => (err.clone(), Recourse::RoutedByPredicate),
            // The surgery's own frontiers (D2 addendum row 2).
            FilletError::UnsupportedBody { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_BODY_RECOURSE))
            }
            FilletError::UnsupportedChain { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_ASSEMBLY_RECOURSE))
            }
            FilletError::UnsupportedRunOut { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_CORNER_RECOURSE))
            }
            FilletError::UnsupportedGeometry { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_GEOMETRY_RECOURSE))
            }
            FilletError::RingClearance { .. } => {
                (err.clone(), Recourse::Exactly(FILLET3_RING_RECOURSE))
            }
            // Invalid input (row 1), and the two forwarding variants.
            FilletError::RepeatedEdge { .. } => (err.clone(), Recourse::None),
            FilletError::NonpositiveSize { .. } => (err.clone(), Recourse::None),
            FilletError::BodyNotIntact { .. } => (err.clone(), Recourse::None),
            FilletError::Certify { .. } => (err.clone(), Recourse::None),
            FilletError::Op { .. } => (err.clone(), Recourse::None),
        }
    }

    /// The seeds this row renders: the surgery's own variants, plus
    /// `Escalated`, whose `Display` routes to six different constants
    /// and is the one arm a single-sentence table gets wrong.
    fn seeds() -> Vec<FilletError> {
        let band = Band::new(1e-9, 1e-6).expect("a band");
        vec![
            FilletError::UnsupportedBody {
                solids: 2,
                shells: 2,
            },
            FilletError::UnsupportedChain {
                edge: EdgeKey::default(),
                detail: "a chain shape that is not built",
            },
            FilletError::UnsupportedRunOut {
                at: EntityId::Vertex(VertexKey::default()),
                detail: "a termination the request does not cover",
            },
            FilletError::UnsupportedGeometry {
                at: EntityId::Face(FaceKey::default()),
                detail: "a stored shape the closed forms do not cover",
            },
            FilletError::RepeatedEdge {
                edge: EdgeKey::default(),
            },
            FilletError::NonpositiveSize { size: 0.0 },
            FilletError::ChamferArmUnsupported {
                edge: EdgeKey::default(),
                supports: "non-(plane–plane)",
            },
            FilletError::BodyNotIntact {
                at: EntityId::HalfEdge(HalfEdgeKey::default()),
                detail: "a reference the plan followed",
            },
            // BOTH corner routes are seeded: the recourse this variant
            // appends is chosen by its TAG, so one witness would leave
            // the other route unrendered and unchecked.
            FilletError::FilletCornerUnsupported {
                vertex: VertexKey::default(),
                corner: CornerConfig::NEdgeVertex { valence: 4 },
                policy: CornerConfig::NEdgeVertex { valence: 4 }.policy(),
            },
            FilletError::FilletCornerUnsupported {
                vertex: VertexKey::default(),
                corner: CornerConfig::SeamVertex,
                policy: CornerConfig::SeamVertex.policy(),
            },
            FilletError::Escalated {
                site: FilletSite::Chain,
                source: Indeterminate {
                    margin: MarginDiag::Value(0.0),
                    band,
                    predicate: Some("fillet3_ring_clearance"),
                },
            },
        ]
    }

    /// How many of `ALL` appear in `text`.
    fn recourses_in(text: &str) -> Vec<&'static str> {
        ALL.into_iter().filter(|r| text.contains(r)).collect()
    }

    /// **The tag's two maps agree.** A corner tag names a run-out policy
    /// EXACTLY when its recourse is the run-out one — the only pairing
    /// that renders a coherent sentence, since `Display` takes both
    /// halves from the tag. A tag that named a policy and a recourse
    /// pointing somewhere else would say "only a run-out policy would
    /// handle this" and then advise something that is not one.
    #[test]
    fn a_corner_tag_names_a_policy_exactly_when_its_recourse_is_the_run_out_one() {
        for corner in [
            CornerConfig::ThreeConvexEdges,
            CornerConfig::NEdgeVertex { valence: 4 },
            CornerConfig::MixedConvexity { convex: 1 },
            CornerConfig::DependentNormals,
            CornerConfig::SeamVertex,
            CornerConfig::Indeterminate,
        ] {
            assert_eq!(
                corner.policy().is_some(),
                corner.recourse() == FILLET3_CORNER_RECOURSE,
                "{corner} names policy {:?} but recourse {:?} — the two maps have drifted",
                corner.policy(),
                corner.recourse()
            );
        }
    }

    #[test]
    fn a_recourse_is_appended_only_where_the_table_allows_it() {
        for seed in seeds() {
            let (witness, expected) = contract(&seed);
            assert_eq!(
                discriminant(&witness),
                discriminant(&seed),
                "the table's arm must witness its OWN variant: {seed:?}"
            );
            let text = witness.to_string();
            let found = recourses_in(&text);
            match expected {
                Recourse::Exactly(one) => assert!(
                    found == [one],
                    "{witness:?} must carry exactly its own recourse, found {} — {text}",
                    found.len()
                ),
                Recourse::RoutedByPredicate => assert!(
                    found.len() == 1,
                    "{witness:?} must route to exactly one recourse, found {} — {text}",
                    found.len()
                ),
                Recourse::None => assert!(
                    found.is_empty(),
                    "{witness:?} reports invalid input and must give no fillet recourse: \
                     {text}"
                ),
            }
        }
    }
}
