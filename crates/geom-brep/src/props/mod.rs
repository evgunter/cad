//! Per-face integral properties over the **exact** B-rep (M2 PR 7):
//! face contributions to the divergence-theorem volume and the surface
//! area. Key-free like the rest of this crate — the owning body
//! flattens each face loop into [`LoopEdge`]s and injects them.
//!
//! **Two lanes, and this header describes one of them.** Everything
//! below is the CLOSED-FORM lane: the M2 analytic surfaces over
//! structurally verified iso-parameter rectangles, and a cylinder face
//! over any region its rims and rulings bound. The other is
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
//! entirely **winding-derived**, and that is deliberate. Both `A⃗` and
//! the rim-recovered `s_f` read the face's stored loop traversal,
//! which the interior-left rule already ties to the *outward* normal;
//! `revert` reverses loops and flips `sense` in the same step, so
//! feeding the bit into a winding-derived term would negate the volume
//! twice. The bit enters only on the sphere: the **rimless** band,
//! whose boundary has no rim to read `s_f` off and which previously
//! hardcoded `+1`, and the face bounded by a tilted circle, whose
//! Gauss–Bonnet area reads its arcs' curvature against the outward
//! normal (`curved::sphere_circle_loop`). Everything else here is sense-invariant
//! by derivation, and the *agreement* of the two encodings is a tier-3
//! obligation (the validator's loop-role winding check), not this
//! module's.
//!
//! Areas of curved faces come from the chart Jacobians over the face's
//! iso-parameter rectangle `[u0,u1]×[v0,v1]` — on a cylinder, over its
//! chart region `−∮ v du`, every loop included; save a sphere face with
//! a circle tilted against its chart on the boundary, which has no
//! rectangle and whose area is Gauss–Bonnet's over its circle arcs
//! (`curved::sphere_circle_loop`); planar face area is
//! `‖A⃗_f‖` (rings subtract automatically via their stored opposite
//! orientation).
//!
//! # Boundary closed forms
//!
//! With `ref` a fixed reference point, `w = start/end` the traversal
//! endpoints on the carrier:
//!
//! - line: `(1/2)·(a − ref)×(b − ref)`;
//! - circular arc (center C, unit axis n̂, radius R, angle t0→t1):
//!   `(1/2)·[(C − ref)×(p1 − p0) + R²·(t1 − t0)·n̂]`
//!
//! (derived by splitting `p − ref = (C − ref) + (p − C)` and using
//! `(p−C)×d(p−C) = R²·n̂·dt`). Reversed traversal negates the form.
//!
//! # Iso-rectangle verification and stored-data discipline
//!
//! Every curved-face quantity is extracted from **stored** data —
//! carrier parameter intervals (angle-true for circle carriers; spans
//! minted from the segment's stored sweep or the sweep angle at
//! construction), stored circle centers/axes/radii,
//! and carrier endpoint evaluations — never from endpoint `atan2`
//! chart inversion (the wedge-unwrap trap, M2 PR 6's blocker: two
//! endpoint inversions differenced lose the winding and sit on a
//! branch cut wherever an arc is anchored at the seam or a pole). The
//! one `atan2` in the module is not an inversion and is the stated
//! exception: `curved::sphere_wedge_azimuth` takes the polar angle of
//! one meridian's departure direction in the frame the other meridian
//! and the face's interior direction span, and its cut — the two
//! meridians on one great circle — is the pair `props_band_coplanar`
//! has just decided definitely not present; its doc argues the
//! interval case and the interval twin pins it. The
//! boundary is structurally verified to be the M2 iso-parameter
//! inventory: each carrier's kind, its rim/meridian role, and its
//! **incidence on the surface** (rim centers on the axis with parallel
//! carrier axes and fitted radii; meridians axial/through the apex/in
//! their meridian plane at the surface's radii) are certified as
//! consistency residuals through the crate's
//! [`decide`](crate::dihedral) funnel, and a definite failure of any
//! of them is a typed [`PropsError`] — scope-boxed fail-loud. **No
//! silent quadrature fallback**: the [`quad`] lane exists and is
//! `pub`, but nothing here routes to it on a refusal. A caller that
//! wants it asks for it, so a refusal from this lane is a refusal the
//! caller sees.
//!
//! **The rectangle itself is ONE named predicate** —
//! `curved::require_rims_at_extremes` (`props_rim_level`): *every rim
//! sits at one of the face's two extreme `v`-levels*.
//!
//! On the sphere it is one of TWO, and the second is about the same
//! `[lo, hi]` from the other side: *every rim's interior side points
//! INTO the extent*, `curved::require_rim_interior_sides`
//! (`props_rim_interior_side`). A level says a latitude the boundary
//! touches and never says which side of it the material is on, so the
//! two faces a rim separates fold the same levels; what tells them
//! apart is the rim's own traversal under the face's sense bit
//! (`curved::rim_interior_side`). Where the levels are silent
//! altogether — a rim-only polar cap — that traversal supplies the
//! missing extreme instead of refusing the face
//! (`curved::sphere_rim_only_pole_level`). Both live on the sphere arm
//! because that is where the extent can be silent AND where the sense
//! bit is what settles it: a rim of a ball bounds the cap under one
//! bit and the ball minus the cap under the other.
//!
//! **The cone's rim-only face has a missing extreme too, and it needs
//! no bit** (`curved::cone_apex_level`): a cone is bounded on the apex
//! side only, so `0` is the single candidate, and the fold sits in the
//! shared parse where all three doors read it. What it does require is
//! the sphere's premise transposed — every rim's traversal agreeing,
//! which is unanimity of σ without σ, since σ is `d_u_sign` under one
//! bit. A cylinder is unbounded along its axis both ways and has no
//! candidate at all, so its rim-only face stays extent-less.
//!
//! **The SHAPE DOOR cannot ask the second, and asks its sense-free
//! residue instead.** σ reads the face's sense bit and
//! [`require_iso_rectangle`] is handed a surface and a loop, with no
//! face, so that it answers a question about the boundary alone. What
//! needs no bit is that every rim encodes the SAME side
//! (`curved::unanimous_rim_side`), and the door takes that. The
//! divergence it leaves is one rim: issue 1598's L-shaped complement
//! has nothing for unanimity to compare — `Ok(())` from the door,
//! `NotIsoRectangle { what: "props_rim_interior_side" }`
//! from the flux lane, on one face. The door's own docs say what a
//! consumer that needs the stronger premise reads instead. The total
//! `u`-measure `w(v)` changes only where a rim is (between rim levels
//! the boundary is meridians, which move no `u`-endpoint), so the rule
//! establishes `w ≡ Δu`. Before S58 the property was re-derived per
//! consumer to three different strengths; the rim-group span-sum rule
//! that stood in for it on three of the four kinds admitted a
//! cross-shaped domain and certified a 19%-low volume with `pad = 0.0`
//! (#649).
//!
//! **What the predicate is and is not, stated exactly**:
//!
//! * Every **iso flux/area closed form** runs it before integrating —
//!   cone, rim-bearing sphere, torus — with **one exemption**, so
//!   "every curved kind" is not the claim: the
//!   **rimless sphere band**, which carries no rim, so the predicate
//!   is vacuous on it rather than satisfied by it. What that arm does
//!   establish (its meridians all lie on ONE great circle that the loop
//!   runs once, which is where `Δu = π` comes from — or on two, the
//!   wedge, whose `Δu` is the azimuth between them on the face's side;
//!   its `v`-extent, from the fold that carries each arc's span-derived
//!   pole extremes) is stated at `curved::sphere`, at the arm.
//!   The **cylinder** does not run it: its flux is the chart Green
//!   form over every loop (`curved::cylinder_chart`), which integrates
//!   the region the boundary actually bounds and needs no rectangle.
//! * **[`boundary_material_sign`] runs it too, on the three iso
//!   arms**, because each reaches a side derivation that rests on this
//!   premise; the cylinder arm reads the sign of its chart area, as
//!   its flux does. It was listed here as a second exemption, on the
//!   argument that *"running the predicate there could only convert an
//!   answer into an exemption"* — which covers the ERROR direction
//!   only. The three linearly-leveled arms derive a side from
//!   `lo + hi − 2v`, *which extreme is this rim at*, and on a domain
//!   that is not a rectangle that returns a definite ±1 depending on
//!   where the owning body's loop flattening started rather than on
//!   the face: two rotations of one edge cycle, two opposite signs.
//!   Tier 3's curved check 6 turned the wrong one into a
//!   `CurvedSenseInverted`, and check 7 being gated on
//!   `errors.is_empty()`, the wrong diagnosis SUPPRESSED the honest
//!   `NotIsoRectangle` the flux lane raises on the same face. The
//!   premise and the side now travel together
//!   (`curved::linear_rim_side`), so what its callers must treat as
//!   exempt is what such a face now produces.
//!
//!   **The torus arm is not exempt either, and the argument that it
//!   was is retired here rather than restated.** That argument said
//!   the arm reads only the anchor meridian's chart orientation and
//!   the rim sharing that meridian's `t0` vertex — *two facts about
//!   one CORNER* — so no global inference is on the path. It is
//!   false. The anchor-end choice cancels against `dv/dt` only when
//!   the two rims FLANKING that meridian carry opposite `d_u`. Every
//!   corner of a rectangle gives that; a **reflex** corner does not,
//!   and on an L-shaped domain the six rotations of one cycle answer
//!   `+ + − − + +` while the flux lane refuses all six. One corner is
//!   true and not sufficient — the PAIR is what the premise buys, and
//!   only a rectangle guarantees it. The arm runs
//!   `require_rims_at_extremes` on the same `torus_ends` extremes the
//!   flux lane uses.
//! * **[`require_iso_rectangle`] is the predicate's own public door**:
//!   the per-kind boundary classification and `props_rim_level`, and
//!   nothing integrated on top — for a consumer whose lane rests on
//!   the premise without wanting a volume (`mesh`'s swept-rectangle
//!   walk cites it before walking a face). It ADMITS every rimless
//!   sphere band, and the flux lane measures the two it has a lune
//!   for — the coplanar two-band face (`props_band_coplanar`,
//!   `Δu = π`) and the wedge (`props_wedge_azimuth`, the azimuth
//!   between the two meridian half-planes on the face's side) — while
//!   refusing a rimless boundary that states neither; the premises
//!   are the closed form's, not the shape's, and the door says so at
//!   its definition.
//! * `w ≡ Δu` is **one** of the two premises `area = r·Δu·(hi − lo)`
//!   needs. The other is that `(lo, hi)` is the face's true
//!   `v`-extent, and **this predicate does not establish it** — each
//!   kind's own derivation does. The torus's ends are the anchor
//!   meridian's stored span, the pieces of a split edge folded into
//!   that meridian first. The cylinder's and cone's are `min_max`
//!   over edge ENDPOINT levels, exact because their meridians are
//!   lines, monotone in `v`. The sphere's meridians are great-circle
//!   arcs whose latitude peaks at a pole the arc may contain in its
//!   interior, so its fold also carries each arc's span-derived pole
//!   extremes (`curved::sphere_meridian_span_levels`, decided through
//!   `props_meridian_pole`) — the stored-span derivation in fold
//!   form.
//!
//! Outside that verification: the loop-local vertex **tags** are
//! trusted as declared (the [`LoopEdge`] trust boundary), and on the
//! cone, sphere and torus the residuals certify carriers, not that the
//! traversed arcs jointly close a loop. The cylinder's Green form
//! checks closure (`props_loop_closed`, `props_chart_loops_closed`),
//! because its anchor-freedom rests on it.

mod curved;
mod loop_area;
pub mod quad;

use geom::Curve3;
use geom_core::spline::SpanLocate;
use geom_core::{Indeterminate, KERNEL_OR_FILE_DEFECT_ENDING, Point3, Real, SizedPass, Vec3};

use crate::recourse::{Reading, RefusedArm, SizedDecision, StoredDefinite, Unsized};

pub use curved::{
    MaterialSign, boundary_material_sign, boundary_material_sign_loops, cone_face_closed_form,
    curved_face, curved_face_loops, require_iso_rectangle, require_one_chart_branch,
};
pub use loop_area::loop_vector_area;

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
/// They are load-bearing: the torus `s_f` inference locates the rim
/// topologically adjacent to a meridian's anchor endpoint through
/// them, and lying tags silently flip the anchored flux term's sign
/// (pinned by `torus_tag_contract_is_load_bearing`). `topo`'s
/// flattening satisfies the contract by construction (first-seen
/// traversal order over the half-edge cycle); callers constructing
/// `LoopEdge`s by hand own it.
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

    /// The vertex tag at the interval start `t0` (`he_plus` start).
    pub(crate) fn tag_at_t0(&self) -> u32 {
        if self.forward { self.start } else { self.end }
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
    /// lies on none). The spiric's frontier is the props quadrature
    /// lane for a spiric-bounded face — the spiric unit's props PR;
    /// the variant carries no `what`, so the frontier is named here
    /// and at each raising site.
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
    /// A sphere face bounded by circles tilted against its chart
    /// (`curved::sphere_circle_loop`) fails a premise of its
    /// Gauss–Bonnet closed form: an edge not a circle on the sphere,
    /// a loop that does not close, a cusp, or a degenerate area.
    /// Name-only, as [`Self::NotIsoRectangle`]: `what` is the deciding
    /// predicate's name.
    SphereLoop {
        /// Which premise failed (the predicate's name).
        what: &'static str,
    },
    /// The face's sense bit contradicts the side its boundary encodes
    /// (`curved::sphere_circle_loop_side`): the face is inside-out, and
    /// its radial flux term — read off the bit — would measure the
    /// complement of the face.
    SenseContradicted,
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
    /// arm of this enum that carries a measured `f64` gets it from a
    /// concrete scalar ([`Self::QuadratureBudget`], from an
    /// `Interval`), and the generic arms are name-only, as
    /// [`Self::NotIsoRectangle`] is.
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
    /// The certified quadrature's enclosure would not tighten to its
    /// target within the refinement budget (M5 PR 11; the
    /// [`quad`] module docs give the rule and the target's metering).
    /// Certified bounds or typed refusal — never a silently wide
    /// answer. Both payloads are LENGTHS (mean boundary displacement),
    /// the same metering as tier 3's volume check.
    ///
    /// Two-tolerance shape, stated: the in-band twin of this refusal is
    /// [`PropsError::Escalated`] on `props_quad_converged` (a margin
    /// close to the target escalates through the funnel before the
    /// budget can run out); this arm is the *definite* "the enclosure
    /// floor sits above the target" outcome. The recourse is the ε
    /// knob: the target scales with the run's ε.
    QuadratureBudget {
        /// The enclosure width the schedule reached, as a length (m):
        /// the last round's own when the schedule ran out, or — when a
        /// round proved that the last round could not certify either
        /// and the loop refused without running it — the lower bound
        /// every remaining round's width was proven to exceed, which
        /// is the last round's width to within the midpoint sum's own
        /// rounding width ([`quad`]'s `last_round_width_lo` says what
        /// the bound omits and why it is a bound). Either way a width
        /// that really missed: strictly above `target_len`.
        width_len: f64,
        /// The convergence target, as a length (m).
        target_len: f64,
        /// The refinement rounds the loop ran before refusing: the
        /// schedule's full count when it ran out; `1` when the
        /// last-round bound refused the face after round 0; `0` on the
        /// exact arm, which has no composite round. A receipt for what
        /// the refusal cost, and the witness that the early exit fired
        /// — a width alone cannot tell the two apart.
        rounds: usize,
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
    /// **The certified quadrature's convergence meter** against its
    /// target: the enclosure's own width, nothing of the model's, so
    /// there is no coincidence to declare and no geometry to move. A
    /// kernel approximation limit ([`Unsized::LastResort`]), which is
    /// the one shape D4 ¶1 (i) lets name loosening.
    Converged,
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
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// **The one recourse a spent quadrature budget names** (D4 ¶1 (i)'s
/// last resort), with the value its payload gives: the convergence
/// target is `QUAD_TARGET_LEN_FACTOR·ε`, linear in the run's ε, so the
/// width the enclosure actually reached names the tolerance whose
/// target that width would meet.
///
/// **"Simplify the trim" is not a lever this refusal has**, and it is
/// gone. Nothing in the payload is about the trim; the refusal is
/// raised on the exact arm and on the last round's proven lower bound,
/// where no trim complexity is implicated; and D4 ¶1 (i) names a
/// quadrature budget spent as its own example of a refusal that would
/// otherwise name no recourse at all, which is what licenses naming
/// loosening here and nowhere a geometry lever exists.
///
/// `topo::validate`'s checks-window mirror composes this same sentence,
/// so the two surfaces cannot disagree.
#[must_use]
pub fn quadrature_budget_recourse(width_len: f64) -> String {
    format!(
        "Recourse: loosen the tolerance to {:.3e} m or more, {}",
        width_len / quad::QUAD_TARGET_LEN_FACTOR,
        geom_core::KERNEL_LIMIT_LAST_RESORT
    )
}

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
            Self::Converged => "whether the quadrature's enclosure has converged",
        }
    }

    /// **The one ending a refusal of this check carries** on `arm`, read
    /// at `reading` — [`crate::recourse`]'s table, one row per check.
    #[must_use]
    pub fn ending(self, arm: RefusedArm<'_>, reading: Reading) -> String {
        match self {
            Self::Exact => Unsized::Defect.recourse(arm, reading),
            Self::Inventory => crate::recourse::not_yet(arm),
            Self::Extent => FACE_EXTENT.recourse(arm, reading),
            Self::Converged => Unsized::LastResort.recourse(arm, reading),
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
            Self::SenseContradicted => write!(
                f,
                "a sphere face's orientation sense contradicts the side its boundary encodes, \
                 so the face is inside-out. {KERNEL_OR_FILE_DEFECT_ENDING}"
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
                FACE_EXTENT.recourse(RefusedArm::SignCertain, Reading::AtRest)
            ),
            Self::Escalated { cause, check } => write!(
                f,
                "{} is too close to call at this tolerance: {}. {}",
                check.subject(),
                cause.payload(),
                check.ending(RefusedArm::Undecided(cause), Reading::AtRest)
            ),
            Self::QuadratureBudget {
                width_len,
                target_len,
                rounds,
            } => write!(
                f,
                "a face's certified contribution stayed {width_len:.3e} m wide after {rounds} \
                 refinement round(s), above the {target_len:.3e} m this tolerance targets. {}",
                quadrature_budget_recourse(*width_len)
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
/// [`PropsError::Unimplemented`] on a rational `Nurbs` carrier
/// (non-rational spline boundaries integrate exactly through
/// [`loop_vector_area`]'s per-span Gauss closed form — the reachable
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
        // The kernel's own approximation limit, and the one shape D4
        // lets name loosening.
        let converged = under(PropsCheck::Converged);
        assert_eq!(
            converged,
            format!(
                "whether the quadrature's enclosure has converged is too close to call at \
                 this tolerance: margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8). {}",
                geom_core::KERNEL_LIMIT_RECOURSE
            )
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
            PropsError::SenseContradicted,
            PropsError::NappeSpanning,
            PropsError::NotOneChartBranch {
                edge: 0,
                what: "azimuth jumps by one half-turn at the pole",
            },
            PropsError::DegenerateFace,
            escalated,
            PropsError::QuadratureBudget {
                width_len: 2e-9,
                target_len: 1e-9,
                rounds: 3,
            },
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
        assert_eq!(arms.len(), 11, "an arm was added without a row here");
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
