//! Per-face integral properties over the **exact** B-rep (M2 PR 7):
//! face contributions to the divergence-theorem volume and the surface
//! area. Key-free like the rest of this crate — the owning body
//! flattens each face loop into [`LoopEdge`]s and injects them.
//!
//! **Two lanes, and this header describes one of them.** Everything
//! below is the CLOSED-FORM lane: the M2 analytic surfaces over every
//! loop of a face their closed forms read — a cylinder or torus face
//! over any region its rims and rulings bound, a cone face over its
//! boundary's vector area, a sphere face over any loops of circle arcs.
//! The other is
//! [`quad`], the certified-quadrature lane — NURBS patches, conic-
//! trimmed faces, an enclosure with a `pad` rather than an exact
//! number — and it is `pub`, larger than this lane, and governed by
//! its own module docs. A claim here about "every face" or "no
//! fallback" is a claim about the closed-form lane only.
//!
//! # Formulation
//!
//! The solid volume is `V = (1/3)·Σ_faces ∮_face p·n dA` (divergence
//! theorem; Mäntylä §13.3 generalized off the polyhedral case). Each
//! face's flux integral splits against a per-surface **anchor point**
//! `c_f` (plane origin, cylinder axis origin, cone apex, sphere/torus
//! center):
//!
//! ```text
//! ∮_f p·n dA  =  ∮_f (p − c_f)·n dA  +  c_f · A⃗_f
//! ```
//!
//! - `A⃗_f = ∮_f n dA` is the **vector area**, a pure boundary integral
//!   `(1/2)∮_{∂f}(p − ref)×dp` (Stokes) with exact per-carrier closed
//!   forms ([line and circular-arc][mod@self#boundary-closed-forms]) —
//!   automatically signed by the stored loop orientation (outward
//!   CCW, D1), so no orientation data is needed for this term.
//! - `∮_f (p − c_f)·n dA` has a per-surface closed form `s_f·K_f`
//!   against the chart normal: 0 for planes (points of the plane) and
//!   cones (generators through the apex), `radius·Area_f` for
//!   cylinders and spheres, and an elementary trig form for tori.
//!   `s_f = ±1` records whether the face's outward normal is the chart
//!   normal or its negation, recovered from stored boundary data
//!   (below).
//!
//! **Where the orientation lives after M5 S10.** `topo::Face` now
//! carries an explicit `sense` bit, but this module remains almost
//! entirely **winding-derived**, and that is deliberate. `A⃗` and every
//! chart Green form read the face's stored loop traversal, which the
//! interior-left rule already ties to the *outward* normal; `revert`
//! reverses loops and flips `sense` in the same step, so feeding the
//! bit into a winding-derived term would negate the volume twice. The
//! bit enters only on the sphere, whose Gauss–Bonnet area reads its
//! arcs' curvature against the outward normal and whose radial term
//! `s_f` IS the bit (`curved::sphere_face`). There the boundary's own
//! reading of the side (`curved::sphere_loop_side`) is held against
//! the bit before it is used, and a contradiction refuses; elsewhere
//! the *agreement* of the two encodings is a tier-3 obligation (check
//! 6), not this module's.
//!
//! Areas of curved faces: a cylinder's and a torus's from the chart
//! Jacobian over the region their loops bound (`−∮ v du` on the
//! cylinder, every loop included); a cone's `|axis·A⃗|/sin α`; a
//! sphere's Gauss–Bonnet's over its circle arcs, holes included; a
//! planar face's `‖A⃗_f‖` (rings subtract automatically via their
//! stored opposite orientation).
//!
//! # Boundary closed forms
//!
//! With `ref` a fixed reference point, `w = start/end` the traversal
//! endpoints on the carrier:
//!
//! - line: `(1/2)·(a − ref)×(b − ref)`;
//! - circular arc (unit axis n̂, radius R, angle `Δ = t1 − t0`):
//!   `(1/2)·[(p0 − ref)×(p1 − p0) + R²·(Δ − sin Δ)·n̂]`
//!
//! (the triangle `ref, p0, p1` plus the segment between chord and arc,
//! from `(p − C)×dp = R²·n̂·dt`; an ellipse's is the same with
//! `R² → a·b`, and [`loop_vector_area`] says why the lever is `p0`
//! rather than the centre). Reversed traversal negates the form.
//!
//! # Stored-data discipline and the iso-rectangle premise
//!
//! Every curved-face quantity is extracted from **stored** data —
//! carrier parameter intervals (angle-true for circle carriers; spans
//! minted from the segment's stored sweep or the sweep angle at
//! construction), stored circle centers/axes/radii, and carrier
//! endpoint evaluations — never from endpoint `atan2` chart inversion
//! (the wedge-unwrap trap, M2 PR 6's blocker: two endpoint inversions
//! differenced lose the winding and sit on a branch cut wherever an
//! arc is anchored at the seam or a pole). The `atan2`s the module
//! does take are angles between two stored directions — a sphere
//! loop's turning at a vertex, a projected arc's tangent–chord angle —
//! each read where its cut is decided absent. Each kind's boundary is
//! verified as its closed form needs — carrier kind, incidence on the
//! surface, closure — through the crate's [`decide`](crate::dihedral)
//! funnel, and a definite failure is a typed [`PropsError`]. **No
//! silent quadrature fallback**: the [`quad`] lane exists and is
//! `pub`, but nothing here routes to it on a refusal.
//!
//! **The iso-rectangle premise is ONE named predicate** —
//! `curved::require_rims_at_extremes` (`props_rim_level`): *every rim
//! sits at one of the face's two extreme `v`-levels*. Before S58 it
//! was re-derived per consumer to three different strengths, and the
//! rim-group span-sum rule that stood in for it admitted a
//! cross-shaped domain and certified a 19%-low volume with
//! `pad = 0.0` (#649). Its one consumer is **[`require_iso_rectangle`]**,
//! the predicate's own public door for a lane that rests on the premise
//! without wanting a volume (`mesh`'s swept-rectangle walk), on every
//! curved kind, with its sense-free companion: every rim encodes the
//! SAME side (`curved::unanimous_rim_side`).
//!
//! No side reading and no flux lane rests on it. The cylinder and the
//! torus integrate the region their loops bound by their chart Green
//! forms, the cone reads its boundary's vector area, and the sphere
//! reads any loops of circles by Gauss–Bonnet, so a notched or ringed
//! face measures and encodes its side as a rectangle does. [`boundary_material_sign_loops`] reads
//! each kind's side off the quantity its flux integrates.
//!
//! Closure: every lane on a cylinder, torus or cone chart decides it one way
//! (`closure`): each junction's chart gap, less whole periods, against a
//! stated slack, then whole-number windings held to a face boundary's
//! integer facts. The windings are read two ways by what the lane holds:
//! an interval lane sums the whole periods its junctions are nearest, a
//! closed form reads its loop's total angle at half-turn thresholds. The
//! sphere's Gauss–Bonnet decides every junction in space
//! (`props_loop_closed`). No lane reads the loop-local vertex **tags**
//! ([`LoopEdge`]).

mod bracket;
mod closure;
mod curved;
mod loop_area;
pub mod quad;
pub mod rational_rim;
pub mod spiric;
pub(crate) mod taylor;

use geom::Curve3;
use geom_core::spline::SpanLocate;
use geom_core::{Indeterminate, KERNEL_OR_FILE_DEFECT_ENDING, Point3, Real, SizedPass, Vec3};

use crate::recourse::{Reading, RefusedArm, RefusedSide, SizedDecision, Unsized};

pub use curved::{
    ClosedSurface, MaterialSign, SeamCover, SeamLoops, boundary_material_sign,
    boundary_material_sign_loops, closed_surface, cone_face_closed_form, curved_face,
    curved_face_loops, require_iso_rectangle, require_one_chart_branch, seam_loops,
    sphere_region_cover, sphere_region_material_sign, whole_surface,
};
pub(crate) use curved::{PoleLeft, sphere_pole_left};
pub use loop_area::{
    arc_segment_term, chord_bulge, is_rational, loop_vector_area, vector_area_reads,
};

/// One traversed boundary edge of a face loop: a key-free view of
/// (carrier, certified `he_plus`-forward parameter interval, traversal
/// direction within this loop, loop-local endpoint tags). The owning
/// body flattens its half-edge cycles into these (traversal order;
/// `start`/`end` are the traversal-order vertex tags — any small ints
/// injective over the loop's vertices).
///
/// # Trust boundary: vertex tags
///
/// The tags must **faithfully identify shared vertices** — no residual
/// can catch a tag lie, because a lie leaves the geometry unchanged.
/// No props lane reads them: closure is decided on the traversed
/// endpoints, and a tag lie moves no flux (`torus_flux_reads_no_tags`).
/// `topo`'s flattening satisfies the contract by construction
/// (first-seen traversal order over the half-edge cycle); callers
/// constructing `LoopEdge`s by hand own it.
#[derive(Clone, Debug)]
pub struct LoopEdge<T: Real> {
    /// The edge's carrier locus.
    pub carrier: Curve3<T>,
    /// The identity of the edge this one is a piece of, when the
    /// owning body records one ([`CarrierId`]); `None` for a loop
    /// built without a body ([`LoopEdge::hand_built`]). Two edges with
    /// equal ids are pieces of ONE edge — one carrier, one
    /// parametrisation, intervals that partition its own — which is
    /// what lets a parse fold them back into it ([`curved`]'s torus
    /// meridian fold). Equality of ids is the only identity test props
    /// runs; two edges carrying the same locus as VALUES are never
    /// inferred to be one edge. A hand-built id is the loop author's
    /// assertion of what a body would have recorded, exactly as the
    /// vertex tags are: the fold enforces what it can see — the pieces
    /// meet, and span one certified interval — and trusts the identity
    /// for the rest.
    pub carrier_id: Option<CarrierId>,
    /// Certified interval start (`he_plus`-forward, `t0 < t1`).
    pub t0: T,
    /// Certified interval end.
    pub t1: T,
    /// Whether this loop traverses the edge `t0 → t1` (`he_plus`) or
    /// reversed (`he_minus`).
    pub forward: bool,
    /// Traversal-order start vertex tag (loop-local).
    pub start: u32,
    /// Traversal-order end vertex tag (loop-local).
    pub end: u32,
}

impl<T: Real> LoopEdge<T> {
    /// A loop edge stated without a body — a test's or a consumer's
    /// hand-built loop. It carries no [`CarrierId`], so no two such
    /// edges are ever folded into one; the opt-out is said here, once.
    pub fn hand_built(
        carrier: Curve3<T>,
        t0: T,
        t1: T,
        forward: bool,
        start: u32,
        end: u32,
    ) -> Self {
        Self {
            carrier,
            carrier_id: None,
            t0,
            t1,
            forward,
            start,
            end,
        }
    }
}

impl<T: SpanLocate> LoopEdge<T> {
    /// The carrier point at the interval start `t0` (the `he_plus`
    /// start; **not** the traversal start when `forward` is false).
    pub(crate) fn p0(&self) -> Point3<T> {
        self.carrier.eval(self.t0)
    }

    /// The carrier point at the interval end `t1`.
    pub(crate) fn p1(&self) -> Point3<T> {
        self.carrier.eval(self.t1)
    }

    /// The carrier points at the edge's TRAVERSAL ends, in traversal
    /// order — `(p0, p1)` forward, swapped otherwise, the geometric
    /// twin of the `(start, end)` tag pair.
    pub(crate) fn traversal_ends(&self) -> (Point3<T>, Point3<T>) {
        if self.forward {
            (self.p0(), self.p1())
        } else {
            (self.p1(), self.p0())
        }
    }
}

/// The identity of the original edge a boundary edge is a piece of —
/// the root of its split lineage in the owning body, opaque here. A
/// body's loop flattening mints one per edge from its own keys
/// (`topo` chases each edge's split provenance to the edge that was
/// never itself minted by a split), so ids are comparable only within
/// ONE body's flattening: a graft re-keys, and two bodies' ids mean
/// nothing to each other. A split keeps the parent's carrier and
/// partitions its interval, so equal ids assert one carrier and one
/// parametrisation by construction, never by a comparison of stored
/// geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CarrierId(u64);

impl CarrierId {
    /// The one constructor. `topo`'s flattening is the minter in
    /// production; anyone else who mints one asserts, as the loop's
    /// author, what a body would have recorded ([`LoopEdge`]'s
    /// `carrier_id` states the contract).
    pub fn minted(raw: u64) -> Self {
        Self(raw)
    }
}

/// A face's closed-form contribution to the body integrals.
#[derive(Clone, Copy, Debug)]
pub struct FaceContribution<T: Real> {
    /// `∮_face p·n dA` with `n` the face's **outward** unit normal —
    /// the divergence-theorem flux; the body volume is `Σ flux / 3`.
    pub flux: T,
    /// The face's (unsigned) surface area.
    pub area: T,
}

/// Typed failure of a per-face closed form (closed enum, D4 ¶3): the
/// boundary is outside the M2 iso-rectangle inventory, a consistency
/// residual is definitely nonzero, or a structural classification
/// escalated. Never a silent fallback.
#[derive(Clone, Debug, PartialEq)]
// The variant roster `topo`'s sample-coverage row reads (this
// crate's `test-support` feature, test builds only).
#[cfg_attr(
    feature = "test-support",
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(PropsErrorKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum PropsError {
    /// A carrier or surface this closed-form inventory has no arm for
    /// and never will in this lane: the `Nurbs` placeholder, and a
    /// `Curve3::Spiric` boundary edge on a plane (the oval's area is an
    /// elliptic integral) or on a cylinder, cone or sphere (a spiric
    /// lies on none). A spiric-bounded face is measured by the
    /// quadrature lane instead ([`spiric`]), which `topo`'s face walk
    /// routes it to by carrier kind; this lane's refusal is what a walk
    /// holding no quadrature lane reads. The variant carries no `what`,
    /// so the frontier is named here and at each raising site.
    Unimplemented,
    /// The boundary shape is outside the M2 iso-rectangle inventory,
    /// a stored-data consistency residual of an inventory premise is
    /// definitely nonzero, or a
    /// stored span is outside certification's per-edge bounds
    /// `0 < Δt ≤ τ` (`props_meridian_span_forward` /
    /// `props_meridian_span_winding` on a sphere meridian arc, the
    /// `props_meridian_pieces_*` names on a reconstructed torus
    /// meridian) — an arc no closed form here may fold. The payload
    /// names the structural expectation that failed.
    ///
    /// **Name-only**, like [`Self::NotOneChartBranch`] and for the
    /// same reason. Where a margin decided the refusal, `what` is that
    /// decision's predicate name (`props_rim_level`, `props_rim_side`)
    /// and the K stream holds the margin under it; the other arms'
    /// `what` is a structural sentence no margin decided. Either way
    /// this `Decide`-generic lane does not read a margin back as
    /// `f64`.
    NotIsoRectangle {
        /// Which structural expectation failed (static description).
        what: &'static str,
    },
    /// **A boundary edge does not lie on its own face's surface** — the
    /// premise every closed form here starts from, and one no valid
    /// body violates at any tolerance, so this is a defect of the
    /// kernel or of the file the body was read from.
    ///
    /// Split from [`Self::NotIsoRectangle`] because the two readings
    /// are not one fact and no consumer could tell them apart from the
    /// variant: a face outside the inventory is valid input on an
    /// unbuilt lane, while an edge off its surface is a contradiction
    /// between a stored carrier and the surface it is stored against.
    /// The split is taken at the RAISING site, by the premise the
    /// residual checks ([`PropsCheck::Exact`]), not by a census of
    /// predicate names: the same name is one premise on one surface and
    /// the other on another — `props_rim_fit` places a circle on a
    /// cylinder, cone or sphere, while on a torus it additionally asks
    /// for an iso-v rim, which a Villarceau circle on the surface
    /// fails.
    ///
    /// **Name-only**, like [`Self::NotIsoRectangle`] and for the same
    /// reason.
    OffSurface {
        /// Which incidence failed (static description).
        what: &'static str,
    },
    /// A cone face's `v` range definitely spans both nappes — not a
    /// face any M2 construction produces.
    NappeSpanning,
    /// A sphere face (`curved::sphere_face`) fails a premise of its
    /// Gauss–Bonnet closed form: an edge not a circle on the sphere,
    /// a loop that does not close, a cusp, or a degenerate area.
    /// Name-only, as [`Self::NotIsoRectangle`]: `what` is the deciding
    /// predicate's name.
    SphereLoop {
        /// Which premise failed (the predicate's name).
        what: &'static str,
    },
    /// A cone face (`curved::cone_face_closed_form`) fails a premise of
    /// its vector-area closed form: a loop that sweeps past a turn
    /// through the apex, that meets itself at a vertex, or a ring that
    /// touches the apex. Name-only, as [`Self::SphereLoop`]: `what` is
    /// the deciding predicate's name.
    ConeLoop {
        /// Which premise failed (the predicate's name).
        what: &'static str,
    },
    /// **A face's holes cover more than its region**: every ring is
    /// wound against the outer loop (`props_ring_winding` against
    /// `props_cone_outer_side`), but their areas sum past the outer
    /// loop's — the face's side (`props_cone_area_side`) is not the
    /// outer loop's — so the face's area would come out
    /// negative. Holes that overlap each other or leave the region.
    HolesExceedFace,
    /// A one-loop sphere face's sense bit contradicts the side its
    /// boundary encodes (`curved::sphere_loop_side`), so its radial
    /// flux term — read off the bit — would measure the complement of
    /// the face.
    /// Two causes, which the boundary cannot tell apart: the bit is
    /// inverted, or the face holds in its interior the one chart pole
    /// off its loop, which import must normalise into a vertex of the
    /// face.
    SenseContradicted,
    /// A sphere face WITH HOLES whose sense bit contradicts the side
    /// its boundary encodes — the side of its loops with positive
    /// area, which no chart pole enters — so the bit is inverted.
    SenseInverted,
    /// **A sphere seam region's faces overlap**: under their sense bits,
    /// its members claim the region `times` times over (`times ≥ 2`,
    /// [`sphere_region_cover`]). Tier 3's check 6 refuses the same region
    /// (`ValidationError::SeamRegionOvercovered`).
    SeamRegionOvercovered {
        /// How many times over the members claim the region.
        times: usize,
    },
    /// **A sphere seam region's cover census is undecided**: in the band
    /// of a half-sphere threshold, or below its floor of no cover
    /// ([`SeamCover::Unresolved`]). Tier 3's check 6 refuses the same
    /// (`ValidationError::SeamRegionCoverUnresolved`).
    SeamRegionCoverUnresolved,
    /// **Two faces of one sphere meet with their material on one side of
    /// their seam**: their sense bits disagree across a shared edge, a
    /// lamina there. It is the finding tier 3's check 4 names
    /// (`ValidationError::LaminaWedge`), read here off the seam region
    /// ([`SeamCover::MixedSense`]), since the measurement door runs no
    /// check 4.
    SeamRegionSensesDisagree,
    /// **A hole is wound the same way as its face** (`props_ring_winding`):
    /// a ring's signed area has the sign of the face's whole, where a
    /// hole's boundary is traversed opposite to the region it is cut
    /// from. Its area would be ADDED to the face's. Stored data that is
    /// wrong, whichever lane reads it: the closed forms', the cylinder
    /// quadrature's and the spiric lane's one decision
    /// (`curved::require_holes_wound_against`).
    RingWoundWithFace,
    /// **A loop does not close on its periodic chart**
    /// (`props_chart_junction_open`): at one junction the edge after
    /// starts definitely farther from where the edge before ends than
    /// the junction's stated slack allows, once whole periods are taken
    /// off. Stored data that is wrong: consecutive edges of a loop meet
    /// at their shared vertex. Every lane on a cylinder or torus chart
    /// decides it the one way (`closure::require_junction_closed`).
    ChartLoopOpen,
    /// **A face's loops wind its periodic chart as no face's boundary
    /// does**: with every junction decided closed, one loop winds the
    /// chart more than once about one of its directions, or the face's
    /// loops wind it a nonzero number of times together, so they bound
    /// no region of it. Stored data that is wrong: a face's boundary is
    /// simple closed curves that bound it. A band's two rims, once each
    /// and opposite ways, are not this; a lane whose closed form cannot
    /// fold such a band refuses it as its inventory.
    LoopsWindChart,
    /// A boundary edge's traversed **arc** leaves one branch of the
    /// chart, though its CARRIER is a certified iso curve: the arc's
    /// stored parameter span contains a chart singularity in its
    /// interior, so the chart coordinate the edge is supposed to hold
    /// constant jumps by π mid-edge.
    ///
    /// Raised only by [`require_one_chart_branch`], which is a
    /// different question from [`Self::NotIsoRectangle`]'s and is
    /// asked by a different set of consumers: the flux lane's extent
    /// derivation FOLDS the singularity in and measures such a face
    /// exactly, while a lane that reads one chart coordinate per edge
    /// cannot read this edge at all. Valid input, unbuilt lane (D2
    /// addendum row 2): the recourse is to state the meridian as two
    /// edges meeting at the singularity, which every consumer reads.
    ///
    /// **`what` carries the per-kind sentence, not this variant's
    /// prose**, because the jump is not one fact: a sphere meridian's
    /// azimuth jumps by π at a pole, a cone generator's flips to the
    /// mirror nappe at the apex. A single sentence here would be a
    /// sphere sentence printed over a cone refusal.
    ///
    /// **Name-only: the measured overshoot's record is the K
    /// stream, not this payload.** The margin that decided the
    /// refusal is the `props_meridian_pole` / `props_cone_apex`
    /// decision, levered to metres, and `k_stats::decide` records it
    /// under that name — so the number that separates "re-author the
    /// part" from "kernel bug" exists, where a diagnosis reads
    /// margins. The caller's recourse (state the edge as two meeting
    /// at the singularity) does not depend on its size. Carrying it
    /// here would mean reading a definite margin back as `f64` from a
    /// `Decide`-generic lane, a compound `Bounds` bound that
    /// `scripts/gates/bounds-allowlist.sh` exists to keep off
    /// `props/curved.rs`; one payload does not earn that seam. Every
    /// arm of this enum is name-only, as [`Self::NotIsoRectangle`] is.
    NotOneChartBranch {
        /// Which boundary edge, as its index in the loop slice the
        /// caller handed in — the same order `topo::props::loop_edges`
        /// flattens the half-edge cycle into.
        ///
        /// **An index and not an `EdgeKey`, structurally.** A
        /// [`LoopEdge`] is a KEY-FREE view by construction — that is
        /// the trust boundary this module's docs draw — and
        /// `geom-brep` sits BELOW `topo` in the dependency order, so
        /// `EdgeKey` is not a type this crate can name. The index is
        /// what a caller can resolve: it indexes the same slice it
        /// passed, and `topo::props::loop_edges` returns the loop's
        /// half-edges in that order beside it, so the caller holds
        /// the key it wants without props ever handling one.
        edge: usize,
        /// The per-kind sentence: which chart singularity the span
        /// crosses and what the edge's constant coordinate does there
        /// (static description).
        what: &'static str,
    },
    /// The face's parameter extent is coincident with zero — a
    /// degenerate (zero-area) face, refused rather than integrated.
    ///
    /// Read precisely, this is "the AREA ENCLOSURE does not certify a
    /// positive extent", which is what `props_face_extent` /
    /// `props_quad_face_extent` decide and what the quadrature's
    /// convergence meter needs a lever from. A face whose true area is
    /// positive but whose enclosure straddles zero — an area pad that
    /// dwarfs the area, e.g. the extreme-weight rational patches in
    /// [`quad`]'s envelope table — lands here too: a false negative
    /// (a capability gap, recorded as such), never a wrong answer.
    DegenerateFace,
    /// A structural classification landed in the ambiguity band or was
    /// poisoned (D4 ¶3: escalate, never guess).
    Escalated {
        /// The escalation, with its predicate name attached.
        cause: Indeterminate,
        /// **What was being decided**, as the closed type its ending is
        /// an exhaustive match over (D4 ¶1 (i)). Some forty decisions
        /// raise this variant and no one lever reaches them all, so the
        /// recourse follows the check rather than the predicate's name,
        /// which stays routing a developer reads in `Debug`.
        check: PropsCheck,
    },
    /// A quadrature input is outside the lane's certified inventory
    /// (M5 PR 11): a rational pcurve channel, a chart kind without a
    /// closed-form flux algebra (every analytic chart MINTS pcurves
    /// since M6-3; only the cylinder and described-NURBS lanes carry
    /// flux), a scalar with no certification bracket, or a missing
    /// stored cache. The payload names the
    /// structural fact AND the real blocker (exact structural doors —
    /// no in-band twin exists, stated so the omission of the
    /// two-tolerance shape reads as a decision).
    QuadratureUnsupported {
        /// Which structural expectation failed, with its blocker.
        what: &'static str,
    },
}

/// **What a props decision was deciding** — the closed type a refusal's
/// ending is an exhaustive match over (D4 ¶1 (i): "the decision is a
/// closed type at its site, so its recourse is an exhaustive match,
/// never a lookup by predicate name").
///
/// Some forty decisions raise [`PropsError::Escalated`], and no one
/// lever reaches them all: a stored-boundary incidence is a defect
/// however it is refused, an inventory premise is a measurement the
/// kernel does not have, a face extent is a size the user may intend,
/// and the quadrature's convergence meter is the kernel's own
/// approximation limit. The check says which, so this `Display` and
/// `topo::validate`'s at-rest reading of it end one way per decision
/// out of [`crate::recourse`]'s one table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropsCheck {
    /// **A premise an exact construction establishes**: a boundary
    /// edge's incidence on its own face's surface, a cone face staying
    /// on one nappe, a sphere loop that closes with no cusp, one
    /// surface's representation of every rim level of a face. No valid
    /// body violates it at any tolerance, so every refused arm —
    /// definitely nonzero, or in band — is a defect of the kernel or of
    /// the file the body was read from ([`Unsized::Defect`]).
    Exact,
    /// **A premise of a lane's certified inventory**: iso-ness, a rim's
    /// level or side, a stored span inside certification's per-edge
    /// bounds, a boundary shape a closed form has an arm for, a trim
    /// piece the quadrature can decompose. A valid face can fail it — a
    /// sphere circle cut by an oblique plane, a torus Villarceau circle,
    /// a lune's meridian that is not a great circle — so a refusal is a
    /// measurement the kernel does not have yet, not a fault in the
    /// body, and it ends in [`geom_core::NOT_YET_ENDING`].
    Inventory,
    /// **The face's parameter extent**: a size the user may intend, so
    /// it ends in [`FACE_EXTENT`]'s lever and, on a band-decided arm
    /// whose margin gives one, the tolerance below which a smaller one
    /// certifies the extent positive.
    Extent,
}

/// The face-extent decision, whose refused side is a face too thin for
/// its area to certify positive: the one home both its definite arm
/// ([`PropsError::DegenerateFace`]) and its in-band arm
/// ([`PropsCheck::Extent`]) read, so the pair tells one story (D4 ¶1
/// (iv)).
pub const FACE_EXTENT: SizedDecision = SizedDecision {
    lever: "widen the face well past the tolerance",
    size: "width",
    passes: SizedPass::Positive,
    // The stored description a lever edits IS the face's own extent.
    side: RefusedSide::Lever,
    no_size: None,
};

impl PropsCheck {
    /// What this check decides, as the question a refusal names before
    /// its payload.
    #[must_use]
    pub fn subject(self) -> &'static str {
        match self {
            Self::Exact => {
                "whether the face's stored boundary is one a construction here \
                            could produce"
            }
            Self::Inventory => "whether this face's boundary is one the closed forms here fold",
            Self::Extent => "whether the face's area is positive",
        }
    }

    /// **The one ending a refusal of this check carries** on `arm`, read
    /// at `reading` — [`crate::recourse`]'s table, one row per check.
    #[must_use]
    pub fn ending(self, arm: RefusedArm<'_>, reading: Reading) -> String {
        match self {
            Self::Exact => Unsized::Defect.recourse(arm, reading),
            Self::Inventory => crate::recourse::not_yet(arm, reading),
            Self::Extent => FACE_EXTENT.recourse(arm, reading),
        }
    }
}

impl core::fmt::Display for PropsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unimplemented => f.write_str(
                "no closed form here measures a NURBS or Approx carrier. Recourse: state the \
                 face on an analytic surface, where one is",
            ),
            Self::NotIsoRectangle { what } => write!(
                f,
                "this face's boundary is not one the closed forms here fold ({what}). \
                 Recourse: re-cut the face into iso-parameter rectangles — a wall merged \
                 across iso lines splits back into rectangular sub-faces"
            ),
            Self::OffSurface { what } => write!(
                f,
                "a boundary edge does not lie on its own face's surface ({what}). \
                 {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::SphereLoop { what } => write!(
                f,
                "a sphere face's boundary does not bound a region of its sphere ({what}). \
                 Recourse: state its edges as circles ON the sphere, meeting end to end with \
                 no cusp, around an area short of the whole sphere"
            ),
            Self::ConeLoop { what } => write!(
                f,
                "a cone face's boundary does not bound a region of its cone once ({what}). \
                 Recourse: state each loop as edges ON one nappe, meeting end to end, \
                 going round the axis at most once and meeting itself nowhere"
            ),
            Self::HolesExceedFace => write!(
                f,
                "a face's holes cover more area than its outer boundary encloses, so they \
                 overlap one another or leave the face. {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::SeamRegionOvercovered { times } => write!(
                f,
                "the faces of one sphere overlap: together they cover their region {times} \
                 times. {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::SeamRegionCoverUnresolved => f.write_str(
                "whether the faces of one sphere overlap cannot be decided at this tolerance. \
                 Recourse: measure at a finer tolerance",
            ),
            Self::SeamRegionSensesDisagree => write!(
                f,
                "two faces of one sphere meet with their material on the same side of the edge \
                 between them, so the body is a sheet there. {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::SenseContradicted => write!(
                f,
                "a sphere face's orientation sense contradicts the side its boundary encodes: \
                 either the face is inside-out, or it holds a pole of its sphere in its \
                 interior, which import must make a vertex of the face. \
                 {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::SenseInverted => write!(
                f,
                "a sphere face with holes is inside-out: its orientation sense contradicts \
                 the side of its boundary that has positive area. \
                 {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::RingWoundWithFace => write!(
                f,
                "a hole in a face is traversed the same way as the face's own boundary, so \
                 its area would be added to the face's rather than taken from it. \
                 {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::ChartLoopOpen => write!(
                f,
                "a face's stored boundary does not close on its surface: one edge starts away \
                 from where the edge before it ends. {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::LoopsWindChart => write!(
                f,
                "a face's stored boundary loops wind around its surface as no face's boundary \
                 does: one winds more than once, or together they do not wind to zero, so they \
                 bound no region of the face. {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::NappeSpanning => write!(
                f,
                "a cone face's parameter range spans both nappes, and a cone face stays on \
                 one nappe. {KERNEL_OR_FILE_DEFECT_ENDING}"
            ),
            Self::NotOneChartBranch { edge, what } => write!(
                f,
                "boundary edge {edge}'s traversed arc leaves one chart branch — {what}. \
                 Recourse: state that side as two edges meeting at the singularity"
            ),
            // The definite arm of the face-extent decision, ending in
            // the same table its in-band arm does (D4 ¶1 (iv)): the
            // verdict carries no margin, so the lever stands alone.
            Self::DegenerateFace => write!(
                f,
                "this face's parameter extent is coincident with zero, so its area cannot be \
                 certified positive. {}",
                FACE_EXTENT.recourse(RefusedArm::SignCertain(None), Reading::AtRest)
            ),
            Self::Escalated { cause, check } => write!(
                f,
                "{}",
                cause.undecided(
                    check.subject(),
                    check.ending(RefusedArm::Undecided(cause), Reading::AtRest)
                )
            ),
            Self::QuadratureUnsupported { what } => write!(
                f,
                "a quadrature input is outside the certified inventory ({what}). Recourse: \
                 re-mint a missing stored cache; every other blocker named is a lane this \
                 build has not certified, so state the face inside the certified inventory"
            ),
        }
    }
}

impl std::error::Error for PropsError {}

/// The flux and area of a **planar** face from its loops (outer +
/// rings, all traversed as stored): `A⃗ = Σ_loops (1/2)∮(p−origin)×dp`,
/// `flux = origin·A⃗` (the plane through `origin` makes
/// `(p−origin)·n = 0`), `area = ‖A⃗‖` (rings subtract via their stored
/// opposite winding; the loop orientation convention points `A⃗` along
/// the face's outward normal). `origin` doubles as the translation
/// reference (Mäntylä's far-from-origin conditioning remedy).
///
/// **Sense-invariant by derivation** (M5 S10). This function takes no
/// sense at all — not the bit its curved sibling takes — and
/// deliberately must not: `A⃗` is a boundary integral
/// in the face's STORED traversal order, and the interior-left rule
/// already points it along the *outward* normal, whichever side that
/// is. A planar face's entire flux is `origin·A⃗` (the anchored term
/// vanishes on the plane), so orientation reaches this computation
/// exclusively through the winding, never through the surface's chart
/// normal. `revert` reverses loops and flips `Face::sense` together;
/// applying the sense here as well would negate the volume twice.
///
/// # Errors
///
/// [`PropsError::Unimplemented`] on a rational `Nurbs` carrier, which
/// the certified lane reads instead ([`rational_rim`]); non-rational
/// spline boundaries integrate exactly through
/// [`loop_vector_area`]'s per-span Gauss closed form (the reachable
/// at-rest case is a stage-1-promoted plane keeping its parsed spline
/// boundary carriers).
pub fn planar_face<T: SpanLocate>(
    origin: Point3<T>,
    loops: &[Vec<LoopEdge<T>>],
) -> Result<FaceContribution<T>, PropsError> {
    let mut va = Vec3::zero();
    for lp in loops {
        va = va + loop_vector_area(lp, origin)?;
    }
    let flux = (origin - Point3::origin()).dot(va);
    Ok(FaceContribution {
        flux,
        area: va.norm(),
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn line_edge(a: Point3<f64>, b: Point3<f64>) -> LoopEdge<f64> {
        let d = b - a;
        let len = d.norm();
        LoopEdge::hand_built(
            Curve3::Line {
                origin: a,
                dir: d * (1.0 / len),
            },
            0.0,
            len,
            true,
            0,
            0,
        )
    }

    /// Assemble the unit cube from six planar faces (outward CCW
    /// loops): total flux/3 = +1, total area = 6; with every loop
    /// reversed (an inside-out cube) the volume flips to −1 — the
    /// sign the tier-3 +V invariant fires on.
    #[test]
    fn hand_built_cube_volume_sign_tracks_orientation() {
        let p = Point3::new;
        // Faces as (origin-on-plane, CCW-from-outside vertex cycles).
        let faces: [[Point3<f64>; 4]; 6] = [
            // bottom (z = 0, outward −z): CW seen from +z.
            [p(0., 0., 0.), p(0., 1., 0.), p(1., 1., 0.), p(1., 0., 0.)],
            // top (z = 1, outward +z).
            [p(0., 0., 1.), p(1., 0., 1.), p(1., 1., 1.), p(0., 1., 1.)],
            // front (y = 0, outward −y).
            [p(0., 0., 0.), p(1., 0., 0.), p(1., 0., 1.), p(0., 0., 1.)],
            // back (y = 1, outward +y).
            [p(0., 1., 0.), p(0., 1., 1.), p(1., 1., 1.), p(1., 1., 0.)],
            // left (x = 0, outward −x).
            [p(0., 0., 0.), p(0., 0., 1.), p(0., 1., 1.), p(0., 1., 0.)],
            // right (x = 1, outward +x).
            [p(1., 0., 0.), p(1., 1., 0.), p(1., 1., 1.), p(1., 0., 1.)],
        ];
        let volume = |reverse: bool| -> (f64, f64) {
            let mut flux = 0.0;
            let mut area = 0.0;
            for cycle in &faces {
                let order: Vec<Point3<f64>> = if reverse {
                    cycle.iter().rev().copied().collect()
                } else {
                    cycle.to_vec()
                };
                let mut edges = Vec::new();
                for i in 0..4 {
                    edges.push(line_edge(order[i], order[(i + 1) % 4]));
                }
                let c = planar_face(order[0], &[edges]).unwrap();
                flux += c.flux;
                area += c.area;
            }
            (flux / 3.0, area)
        };
        let (v_out, a_out) = volume(false);
        assert!((v_out - 1.0).abs() < 1e-12, "outward cube volume {v_out}");
        assert!((a_out - 6.0).abs() < 1e-12, "cube area {a_out}");
        let (v_in, a_in) = volume(true);
        assert!((v_in + 1.0).abs() < 1e-12, "inside-out cube volume {v_in}");
        assert!((a_in - 6.0).abs() < 1e-12, "area is orientation-blind");
    }

    /// S6 (two-tolerance, D4 ¶1 addendum): the face-extent pair —
    /// exactly-zero extent (`DegenerateFace`) and in-band
    /// (`Escalated` under [`PropsCheck::Extent`]) — is one user
    /// situation, so both arms end in [`FACE_EXTENT`]'s lever, the one
    /// table (D4 ¶1 (iv)).
    ///
    /// They are not the same STRING, and that is the rule working: the
    /// definite arm has no size to tighten below and names the lever
    /// alone, while the in-band arm names the tolerance its own margin
    /// gives. One decision, one lever, the value where there is one.
    #[test]
    fn face_extent_pair_ends_in_one_levers_table() {
        let definite = PropsError::DegenerateFace.to_string();
        assert!(
            definite.ends_with("Recourse: widen the face well past the tolerance"),
            "{definite}"
        );
        let in_band = PropsError::Escalated {
            cause: Indeterminate {
                margin: geom_core::MarginDiag::value(5e-9),
                band: geom_core::Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("props_face_extent"),
                terminal_sliver: false,
            },
            check: PropsCheck::Extent,
        }
        .to_string();
        assert!(
            in_band.ends_with(
                "Recourse: widen the face well past the tolerance, or, if this width is \
                 intended, tighten the tolerance below 5e-10 m"
            ),
            "{in_band}"
        );
        // Neither arm offers a declaration: a face too thin to certify
        // is nobody's coincidence to declare.
        for msg in [&definite, &in_band] {
            assert!(!msg.contains("declare"), "{msg}");
        }
    }

    /// **The in-band arm's ending follows its CHECK, not its predicate
    /// name** (D4 ¶1 (i)): one escalation payload, four checks, four
    /// endings out of [`crate::recourse`]'s table — and each names a
    /// lever that check actually has.
    #[test]
    fn an_escalation_ends_by_the_check_that_raised_it() {
        let under = |check| {
            PropsError::Escalated {
                cause: Indeterminate {
                    margin: geom_core::MarginDiag::value(5e-9),
                    band: geom_core::Band::new(1e-9, 1e-8).unwrap(),
                    predicate: Some("props_rim_fit"),
                    terminal_sliver: false,
                },
                check,
            }
            .to_string()
        };
        // A premise no valid body violates is a defect however it is
        // refused, and no tolerance is offered for one.
        let on_surface = under(PropsCheck::Exact);
        assert!(
            on_surface.ends_with(geom_core::KERNEL_OR_FILE_DEFECT_ENDING),
            "{on_surface}"
        );
        assert!(!on_surface.contains("tolerance below"), "{on_surface}");
        // An inventory premise a valid face can fail is a lane not
        // built: no lever, and the sentence says so plainly.
        let inventory = under(PropsCheck::Inventory);
        assert!(
            inventory.ends_with(geom_core::NOT_YET_ENDING),
            "{inventory}"
        );
        assert!(!inventory.contains("Recourse:"), "{inventory}");
        // A size the user may intend carries the value its margin gives.
        assert!(
            under(PropsCheck::Extent).ends_with("tighten the tolerance below 5e-10 m"),
            "{}",
            under(PropsCheck::Extent)
        );
    }

    /// **`PropsError`'s recourse claim, made enforceable** — the same
    /// row `topo`'s `every_chart_region_arm_names_a_recourse` writes,
    /// for the carrier tier 3 reaches through
    /// `ValidationError::VolumeUncomputable { MassPropsError::Face }`.
    /// That arm renders this error whole and adds nothing, so an arm
    /// here that states a condition and stops is a message that stops.
    ///
    /// **Exactly one marker per arm**, counted the way every other
    /// roster in the repo counts it
    /// (`test_utils::refusal::recourse_markers`): a labelled
    /// `Recourse:`, or the words that say there is no way through. An
    /// arm that ends in a dead end is not labelled `Recourse:` and is
    /// not expected to be.
    ///
    /// **A vocabulary floor on top of that, not a proof.** A check
    /// cannot tell a recourse from a sentence containing a verb, and a
    /// new arm whose recourse uses a word not on this list fails it
    /// honestly — extend the list in the same change. The payloads
    /// below are chosen to carry no verb of their own, so what the row
    /// measures is the variant's own clause and not its `what`.
    /// Every arm of [`PropsError`], one payload each, chosen to carry
    /// no recourse verb of its own so a row reads the variant's clause.
    fn props_error_arms() -> Vec<PropsError> {
        let escalated = PropsError::Escalated {
            cause: Indeterminate {
                margin: geom_core::MarginDiag::value(5e-9),
                band: geom_core::Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("props_face_extent"),
                terminal_sliver: false,
            },
            check: PropsCheck::Extent,
        };
        vec![
            PropsError::Unimplemented,
            PropsError::NotIsoRectangle {
                what: "props_rim_level",
            },
            PropsError::OffSurface {
                what: "props_rim_fit",
            },
            PropsError::SphereLoop {
                what: "props_sphere_loop_cusp",
            },
            PropsError::ConeLoop {
                what: "props_cone_loop_vertex",
            },
            PropsError::HolesExceedFace,
            PropsError::SenseContradicted,
            PropsError::SenseInverted,
            PropsError::RingWoundWithFace,
            PropsError::ChartLoopOpen,
            PropsError::LoopsWindChart,
            PropsError::NappeSpanning,
            PropsError::NotOneChartBranch {
                edge: 0,
                what: "azimuth jumps by one half-turn at the pole",
            },
            PropsError::DegenerateFace,
            escalated,
            PropsError::QuadratureUnsupported {
                what: "a rational pcurve channel",
            },
        ]
    }

    #[test]
    fn every_props_error_arm_names_a_recourse() {
        const RECOURSE_VERBS: &[&str] = &[
            "state", "widen", "loosen", "report", "re-cut", "re-mint", "lie",
        ];
        let arms = props_error_arms();
        assert_eq!(arms.len(), 16, "an arm was added without a row here");
        for arm in &arms {
            let msg = arm.to_string();
            assert_eq!(
                test_utils::refusal::recourse_markers(&msg),
                1,
                "not exactly one recourse marker: {msg}"
            );
            let lower = msg.to_lowercase();
            assert!(
                RECOURSE_VERBS.iter().any(|v| lower.contains(v)),
                "no recourse in: {msg}"
            );
        }
    }

    /// **No arm opens with a stage prefix and none runs past the word
    /// budget** — the shape the viewer's own guard reads
    /// (`test_utils::refusal`), on the sentence a caller holding a
    /// `PropsError` reads directly.
    #[test]
    fn every_props_error_arm_fits_where_it_is_shown() {
        for arm in props_error_arms() {
            let msg = arm.to_string();
            assert_eq!(
                test_utils::refusal::stage_prefixes(&msg, &[]),
                Vec::<String>::new(),
                "{msg}"
            );
            let words = msg.split_whitespace().count();
            assert!(
                words <= test_utils::refusal::BUDGET,
                "{words} words, over the budget: {msg}"
            );
        }
    }
}
