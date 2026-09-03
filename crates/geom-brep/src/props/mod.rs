//! Per-face integral properties over the **exact** B-rep (M2 PR 7):
//! face contributions to the divergence-theorem volume and the surface
//! area. Key-free like the rest of this crate — the owning body
//! flattens each face loop into [`LoopEdge`]s and injects them.
//!
//! **Two lanes, and this header describes one of them.** Everything
//! below is the CLOSED-FORM lane: the M2 analytic surfaces over
//! structurally verified iso-parameter rectangles. The other is
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
//! twice. The bit enters at exactly one site — the **rimless** sphere
//! band, whose boundary has no rim to read `s_f` off and which
//! previously hardcoded `+1`. Everything else here is sense-invariant
//! by derivation, and the *agreement* of the two encodings is a tier-3
//! obligation (the validator's loop-role winding check), not this
//! module's.
//!
//! Areas of curved faces come from the chart Jacobians over the face's
//! iso-parameter rectangle `[u0,u1]×[v0,v1]`; planar face area is
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
//! minted from `θ = 4·atan|bulge|` or the sweep angle at construction,
//! the sanctioned re-inspection), stored circle centers/axes/radii,
//! and carrier endpoint evaluations — never from endpoint `atan2`
//! chart inversion (the wedge-unwrap trap, M2 PR 6's blocker). The
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
//! sits at one of the face's two extreme `v`-levels*. The total
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
//! * Every **flux/area closed form** runs it before integrating —
//!   cylinder, cone, rim-bearing sphere, torus — with **one
//!   exemption**, so "every curved kind" is not the claim: the
//!   **rimless sphere band**, which carries no rim, so the predicate
//!   is vacuous on it rather than satisfied by it. What that arm does
//!   establish (its meridians all lie on ONE great circle, which is
//!   where `Δu = π` comes from; its `v`-extent, from the fold that
//!   carries each arc's span-derived pole extremes) is stated at
//!   `curved::sphere`, at the arm.
//! * **[`boundary_material_sign`] runs it too, on ALL FOUR arms**,
//!   because every one of them reaches a side derivation that rests
//!   on this premise. It was listed here as a second exemption, on the
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
//!   walk cites it before walking a face). It ADMITS the rimless
//!   sphere band the flux lane refuses on `props_band_coplanar`:
//!   `Δu = π` is the closed form's premise, not the shape's, and the
//!   door says so at its definition.
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
//! trusted as declared (the [`LoopEdge`] trust boundary), and the
//! residuals certify carriers, not that the traversed arcs jointly
//! close a loop.

mod curved;
mod loop_area;
pub mod quad;

use geom::Curve3;
use geom_core::spline::SpanLocate;
use geom_core::{Indeterminate, Point3, Real, Vec3};

pub use curved::{
    MaterialSign, boundary_material_sign, curved_face, require_iso_rectangle,
    require_one_chart_branch,
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
pub enum PropsError {
    /// A carrier or surface is the unimplemented `Nurbs` placeholder.
    Unimplemented,
    /// The boundary shape is outside the M2 iso-rectangle inventory,
    /// or a stored-data consistency residual is definitely nonzero.
    /// The payload names the structural expectation that failed.
    NotIsoRectangle {
        /// Which structural expectation failed (static description).
        what: &'static str,
    },
    /// A cone face's `v` range definitely spans both nappes — not a
    /// face any M2 construction produces.
    NappeSpanning,
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
    /// **No measured overshoot in the payload, and that is a
    /// scheduled gap, not a choice** (issue 1602). The margin IS
    /// measured — it is the same `props_meridian_pole` /
    /// `props_cone_apex` quantity the funnel records, levered to
    /// metres — but reading a DEFINITE margin back as `f64` from a
    /// `Decide`-generic lane needs a compound `Bounds`/`Enclosure`
    /// bound, which `scripts/gates/bounds-allowlist.sh` does not
    /// ratify for `props/curved.rs`. Every arm of this enum that
    /// carries a measured `f64` gets it from a concrete scalar
    /// ([`Self::QuadratureBudget`], from a `RingInterval`); the
    /// generic arms are name-only, exactly as
    /// [`Self::NotIsoRectangle`] is. Issue 1602 is the ratification
    /// that would let this arm carry the number.
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

impl core::fmt::Display for PropsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unimplemented => {
                f.write_str("integral properties: Nurbs carrier/surface is unimplemented (D3)")
            }
            Self::NotIsoRectangle { what } => write!(
                f,
                "integral properties: face boundary outside the iso-rectangle inventory ({what})"
            ),
            Self::NappeSpanning => f.write_str("integral properties: cone face spans both nappes"),
            Self::NotOneChartBranch { edge, what } => write!(
                f,
                "integral properties: boundary edge {edge}'s traversed arc leaves one chart \
                 branch — {what}; state the side as two edges meeting at the singularity"
            ),
            Self::DegenerateFace => write!(
                f,
                "integral properties: face parameter extent is degenerate (zero area) — {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::Escalated { cause } => {
                write!(f, "integral properties: classification escalated: {cause}")
            }
            Self::QuadratureBudget {
                width_len,
                target_len,
            } => write!(
                f,
                "integral properties: the certified quadrature enclosure stalled at a mean \
                 boundary displacement of {width_len:.3e} m against the {target_len:.3e} m \
                 target (which scales with the run's tolerance) — certified bounds or typed \
                 refusal, never a silently wide answer; loosen the tolerance or simplify \
                 the trim"
            ),
            Self::QuadratureUnsupported { what } => write!(
                f,
                "integral properties: quadrature input outside the certified inventory: {what}"
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
/// `sense_sign` and deliberately must not: `A⃗` is a boundary integral
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
    /// (`Escalated`) — is one user situation; both arms carry the
    /// shared recourse fragment.
    #[test]
    fn face_extent_pair_carries_the_shared_recourse() {
        let msg = PropsError::DegenerateFace.to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );

        let msg = PropsError::Escalated {
            cause: Indeterminate {
                margin: geom_core::MarginDiag::Value(5e-9),
                band: geom_core::Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("props_face_extent"),
            },
        }
        .to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );
    }
}
