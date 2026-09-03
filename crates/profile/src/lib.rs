//! 2-D sketch profiles as data: the bulge-chain [`Profile`] and its
//! trilean validation into [`ValidatedProfile`] (M2 PR 2).
//!
//! A profile is the *input* to sweeps (M2 PR 4/5): closed 2-D loops on a
//! [`SketchPlane`], stored as plain data with **zero
//! representation-consistency conditions** (the D2 peer-representation
//! lesson applied at the input boundary, ratified in the PR #24
//! conversation, PR #24). Sweeps accept only a
//! [`ValidatedProfile`], the canonicalized output of
//! [`Profile::validate`]; arcs lower to `geom` circle carriers at
//! sweep time — this crate stays 2-D and depends on `geom-core` only.
//!
//! # Profile conventions (normative, stated once)
//!
//! - **Units (D6):** coordinates in meters in the sketch plane's (x, y)
//!   chart; angles in radians.
//! - **The chain.** A [`ProfileLoop`] is a vertex chain
//!   `[{pos, bulge}, …]`, closed by construction: vertex k's `bulge`
//!   describes the segment from vertex k to vertex k+1, and the **last
//!   vertex's bulge describes the implicit closing segment back to the
//!   first** — there is no open/closed flag and no way to write an open
//!   chain.
//! - **Bulge semantics (DXF-compatible, ratified).** For the segment
//!   from vertex A to vertex B, `bulge` b = tan(θ/4) where θ is the
//!   arc's signed included angle; b = 0 is a straight line segment.
//!   **Positive b sweeps counterclockwise** about the arc's center
//!   (θ > 0): the path *turns left* in chain order, and the arc's apex
//!   bows to the **right** of the chord A→B — true for every θ (the
//!   sagitta formula below is exact for all b). The center's side
//!   depends on the arc class: for a **minor** arc (|b| < 1, |θ| < π)
//!   the center lies to the **left** of the chord (a minor arc bows
//!   away from its center); at |b| = 1 (a semicircle) the center is
//!   *on* the chord; for a **major** arc (|b| > 1, |θ| > π) the
//!   apothem L·(1 − b²)/(4b) changes sign and the center crosses to
//!   the **same side as the apex** (the arc wraps more than half the
//!   circle around it). Negative b is the mirror image of all of the
//!   above (clockwise sweep, apex left). The sign is *geometry, not
//!   winding*: it says which way this one segment curves, independent
//!   of the loop's traversal role.
//!   Closed forms used throughout, with L = |B − A|, û = (B − A)/L, and
//!   n̂ = û rotated +90° (the left normal):
//!   - signed sagitta (apex offset): apex = midpoint − n̂·(L·b/2);
//!   - signed apothem: center = midpoint + n̂·(L·(1 − b²)/(4b));
//!   - radius r = L·(1 + b²)/(4|b|); included angle θ = 4·atan(b).
//! - **Reversal is an involution on bulges.** Reversing a chain maps
//!   each segment's b ↦ −b (same locus, opposite traversal);
//!   [`ProfileLoop::reversed`] implements the reindexing and
//!   `reversed ∘ reversed` is the identity, bit-exactly (negation is
//!   exact). Under test.
//! - **θ ∈ (−2π, 2π) exclusive by construction**: b = tan(θ/4) is
//!   finite, so no single segment can close a full period. Closed
//!   carriers therefore need **≥ 2 vertices** (ratified): the minimal
//!   circle is two arcs — representation and topology agree with the
//!   vertices-derive-bounds rule (`geom`'s `curves` module docs).
//! - **Winding is invisible.** There is no direction concept in the
//!   API: users write loops in either traversal; [`Profile::validate`]
//!   derives nesting from containment and canonicalizes traversal
//!   internally (outers counterclockwise, holes clockwise, in sketch
//!   coordinates). No winding errors exist.
//! - **Tangency is declared intent, verified — never inferred (the
//!   #101 discipline).** A *joint* (the junction of two adjacent
//!   segments at their shared vertex) whose two **distinct** carriers
//!   meet tangentially must be declared in
//!   [`ProfileLoop::tangent_joints`]; validation refuses undeclared
//!   definite tangency ([`ProfileError::UndeclaredTangency`]) and
//!   contradicted declarations ([`ProfileError::TangencyContradicted`])
//!   alike. Same-carrier continuation (collinear lines, cocircular
//!   arcs — e.g. the minimal two-arc circle's joints) is carrier
//!   identity, not tangency: legal undeclared. Free arcs whose joints
//!   are definitely transversal (secant carriers) remain legal
//!   undeclared — declaration marks *tangency*, not arc-ness. The
//!   authoring path is the PATHS lattice's `.fillet(r)` ([`path`]),
//!   which computes tangent geometry exactly and declares by
//!   construction; [`ProfileLoop::tangent_joints`] is the explicit flag
//!   for raw hand-authored chains.
//! - **The sketch plane is conventional data.** [`SketchPlane`] is a
//!   rigid placement: profile (x, y) ↦ plane origin + x·u + y·v, with
//!   u/v/normal the columns of the placement's linear part. Rigidity
//!   (u, v orthonormal, normal = u × v) is a convention carried by the
//!   data, unchecked here — exactly like PR 1's `u_ref` — and validation
//!   is purely 2-D (the plane is passed through untouched).
//!
//! # Validation and canonical form
//!
//! [`Profile::validate`] is the single gate: arity, degeneracy,
//! simplicity, the containment forest, and canonicalization, every
//! decision through named trilean predicates (geom-core's
//! [`geom_core::Decide`] door — see `crate::validate` docs for the
//! predicate inventory and `geom_core::k_stats` for the margin-statistics
//! hook). Its canonical-form rules (deterministic starting vertex, loop
//! order, traversal senses) are documented on [`ValidatedProfile`]; the
//! canonical form is invariant under input traversal order and
//! starting-vertex rotation of every loop (under test — D9-load-bearing,
//! since recipes replay this). It is **not** invariant under
//! reordering the input's loop list: the outer is hoisted first, but
//! holes keep their discovery (input) order — a D9 recipe replays the
//! loop order it recorded, so reordering loops is a *different*
//! recipe, deliberately.
//!
//! **Deferred (named): session-box enforcement (D4 ¶4).** Nothing in
//! this crate rejects geometry outside the documented model size range
//! — construction sugar can mint absurd carriers from near-degenerate
//! input (e.g. [`bulge_from_via`] with a collinear-ish through-point
//! yields a ~1e15 m radius arc that validates if simple). D4 ¶4 wants
//! out-of-range geometry rejected at construction; that check is a
//! kernel-wide boundary (shared with `topo`/`geom`), not a
//! per-crate ad-hoc test, and lands as its own design item — this is
//! the first site where innocent input reaches the gap.
//!
//! # Totality
//!
//! Construction and sugar are evaluation code: total, comparison-free,
//! no panics; garbage in (NaN coordinates, a through-point coincident
//! with a chord endpoint) produces well-defined poison or degenerate
//! data *values*, which validation catches with typed errors — never a
//! guess (the poison policy of `geom_core::real`).

mod fillet_select;
pub mod lift;
pub mod path;
mod seg;
pub mod structure;
mod sugar;
mod validate;

use geom_core::{Affine3, Mat3, Point2, Point3, Real, Vec3};

pub use lift::{Fidelity, LiftOutcome, LiftRefusal, lift, lift_checked};
pub use path::program::{
    ArcData, ArcMode, ClosedLoop, ReplayError, ReplayErrorKind, Step, Target, TipState, Verb,
    replay, replay_guided, replay_recording,
};
pub use path::{
    ArcCarrierScalar, ArcLen, ArcSide, ArrivesTangent, Bulge, Center, ContinueTarget, LineTarget,
    Open, PartialPath, PathError, PathErrorKind, PathNoCornerReason, PointLeg, Radius, Start,
    Sweep, TangentArcTarget, Via, circle, circle_split,
};
pub use structure::{
    CanonicalStructure, CornerGate, Decision, DecisionValue, FilletDecision, LoopCanonical,
    ProfileStructure, ReplayStructure, SegmentShape, StructureRefusal, StructureRefusalKind,
};
pub use sugar::{ArcSweep, FilletLegShape, bulge_from_center, bulge_from_via};
pub use validate::{
    BlendArc, ContactKind, EscalationSite, FilletLeg, FilletLegCarrier, LoopRole, NoCornerReason,
    ProfileError, SegmentKind, SegmentRef, ValidatedLoop, ValidatedProfile, ValidatedSegment,
};

/// One vertex of a profile loop: a position plus the bulge of the
/// segment *leaving* it toward the next vertex (see the crate docs'
/// bulge semantics).
#[derive(Clone, Copy, Debug)]
pub struct ProfileVertex<T: Real> {
    pos: Point2<T>,
    bulge: T,
}

impl<T: Real> ProfileVertex<T> {
    /// A vertex: a position plus the bulge of the segment leaving it.
    ///
    /// **What the privacy on this type does and does not claim.**
    /// Vertex *values* stay mintable wherever the type is nameable —
    /// this door is unconditional and every field reads back, exactly
    /// as for [`Point2`]. Privacy here buys representation freedom, not
    /// mint-prevention. The funnel claim is about LOOPS: outside this
    /// crate a [`ProfileLoop`] cannot be spelled from a vertex table,
    /// because the only raw loop door is [`RawLoop`] and that trait is
    /// off the presented surface. A caller holding a bag of vertices
    /// has nothing to put them in.
    pub fn new(pos: Point2<T>, bulge: T) -> Self {
        Self { pos, bulge }
    }

    /// The vertex position in sketch-plane coordinates (meters).
    pub fn pos(&self) -> Point2<T> {
        self.pos
    }

    /// The bulge b = tan(θ/4) of the segment from this vertex to the
    /// next (0 ⇒ line; sign per the crate docs — positive sweeps
    /// counterclockwise). The last vertex's bulge belongs to the
    /// implicit closing segment.
    pub fn bulge(&self) -> T {
        self.bulge
    }
}

/// A closed loop: a vertex chain, closed by construction (the last
/// vertex's segment returns to the first — see the crate docs).
///
/// Plain data — conventions are carried by data (D2), and nothing is
/// checked at construction: [`Profile::validate`] is the gate.
///
/// **Sealed at the crate boundary.** The fields are private and read
/// back through [`vertices`](Self::vertices) /
/// [`tangent_joints`](Self::tangent_joints), so outside this crate the
/// only compilable route to a loop is a door: the [`path`] lattice (the
/// presented channel) or [`RawLoop`] (kernel vocabulary, omitted from
/// the façade). Naming the type, reading it, and matching on error
/// payloads all still work; spelling one from a vertex table does not.
/// The seal is a CRATE boundary, not a module one — `crates/profile`'s
/// own internals construct loops directly and stay on the sealed-verbs
/// discipline instead.
///
/// A doctest is a separate crate, so these two blocks are the seal
/// executed at the boundary it claims. A struct literal is a PRIVACY
/// error, not a missing-import one:
///
/// ```compile_fail,E0451
/// use profile::{ProfileLoop, ProfileVertex};
/// let _: ProfileLoop<f64> = ProfileLoop {
///     vertices: Vec::<ProfileVertex<f64>>::new(),
///     tangent_joints: Vec::new(),
/// };
/// ```
///
/// The doors compile, in the same position:
///
/// ```
/// use geom_core::Point2;
/// use profile::{ProfileLoop, RawLoop};
///
/// let square: ProfileLoop<f64> = RawLoop::polygon([
///     Point2::new(0.0, 0.0),
///     Point2::new(1.0, 0.0),
///     Point2::new(1.0, 1.0),
///     Point2::new(0.0, 1.0),
/// ]);
/// assert_eq!(square.vertices().len(), 4);
/// assert!(square.tangent_joints().is_empty());
/// ```
#[derive(Clone, Debug)]
pub struct ProfileLoop<T: Real> {
    /// The vertex chain, in traversal order (either winding — winding
    /// is invisible, see the crate docs).
    vertices: Vec<ProfileVertex<T>>,
    /// Declared-tangent joints, as vertex indices — see
    /// [`ProfileLoop::tangent_joints`] for the normative semantics.
    tangent_joints: Vec<usize>,
}

/// The raw loop-minting doors — **kernel vocabulary, off the presented
/// surface** (Ev's ruling on #413, executed by LIB-RETTAIL).
///
/// The invariant this trait exists to hold: a caller who can NAME
/// [`ProfileLoop`] cannot thereby MINT one from a vertex table. Inherent
/// methods travel with their type, so as long as `new`/`polygon` were
/// inherent, every surface that made the type nameable — and the type
/// must stay nameable, since read-back, error payloads and
/// [`ValidatedLoop`] all hand one back — re-exported the authoring tier
/// with it. Trait methods travel with the TRAIT instead: the façade's
/// curated `pncad::profile` module omits `RawLoop`, and a downstream
/// caller who never imports it has no `ProfileLoop::polygon` in scope.
///
/// Authoring goes through [`path`] (the PATHS lattice), which classifies
/// junctions at authoring time and declares tangency by construction.
/// This trait is what the kernel's own crates and their fixtures use to
/// spell a vertex table directly.
///
/// The seal is what makes that invariant total: [`ProfileLoop`]'s
/// fields are private, so outside this crate there is no struct-literal
/// route around the trait — a downstream `ProfileLoop { .. }` does not
/// compile (E0451, under test).
pub trait RawLoop<T: Real>: Sized {
    /// Builds a loop from a vertex chain, with no declared-tangent
    /// joints — add them with [`with_tangent_joints`](Self::with_tangent_joints),
    /// or author the loop through [`path`], which declares by
    /// construction.
    fn new(vertices: Vec<ProfileVertex<T>>) -> Self;

    /// Builds a loop of straight segments through the given points (all
    /// bulges zero) — polygon sugar.
    fn polygon(points: impl IntoIterator<Item = Point2<T>>) -> Self;

    /// The same loop with its **declared-tangent joints** set to the
    /// given vertex indices (see
    /// [`ProfileLoop::tangent_joints`](ProfileLoop::tangent_joints) for
    /// what a declaration means and how validation verifies it).
    ///
    /// Declaring by hand is the explicit hand-authoring/persistence
    /// form; `.fillet(r)` on the [`path`] lattice declares by
    /// construction and is the authoring path.
    fn with_tangent_joints(self, tangent_joints: Vec<usize>) -> Self;
}

impl<T: Real> RawLoop<T> for ProfileLoop<T> {
    fn new(vertices: Vec<ProfileVertex<T>>) -> Self {
        Self {
            vertices,
            tangent_joints: Vec::new(),
        }
    }

    fn polygon(points: impl IntoIterator<Item = Point2<T>>) -> Self {
        <Self as RawLoop<T>>::new(
            points
                .into_iter()
                .map(|pos| ProfileVertex {
                    pos,
                    bulge: T::zero(),
                })
                .collect(),
        )
    }

    fn with_tangent_joints(mut self, tangent_joints: Vec<usize>) -> Self {
        self.tangent_joints = tangent_joints;
        self
    }
}

impl<T: Real> ProfileLoop<T> {
    /// The vertex chain, in traversal order (either winding —
    /// winding is invisible, see the crate docs).
    pub fn vertices(&self) -> &[ProfileVertex<T>] {
        &self.vertices
    }

    /// **Declared-tangent joints** (the #101 discipline): vertex
    /// indices whose *joint* — the junction between the segment
    /// arriving at that vertex and the segment leaving it — is declared
    /// tangent (first-order carrier contact between two *distinct*
    /// carriers).
    ///
    /// Semantics, normative:
    /// - A declaration is **intent, verified — never trusted**:
    ///   validation checks the joint's carrier-tangency margin is
    ///   definite Zero and refuses
    ///   [`ProfileError::TangencyContradicted`] otherwise.
    /// - Conversely, a joint whose carriers *are* definitely tangent
    ///   **must** be declared: undeclared exact tangency is refused as
    ///   [`ProfileError::UndeclaredTangency`] (relying on tangency that
    ///   numerically happens-to-hold is the pattern the boolean door's
    ///   UndeclaredCoincidence retired; lifted here to the profile
    ///   door).
    /// - **Same-carrier continuation is not tangency**: collinear
    ///   line/line joints and cocircular arc/arc joints (the two-arc
    ///   circle's joints) are carrier *identity*, legal undeclared, and
    ///   a declaration there is contradicted.
    /// - Duplicate indices are harmless (set semantics); an
    ///   out-of-range index is a typed validation error. Order is not
    ///   significant.
    ///
    /// The PATHS lattice's `.fillet(r)` ([`path`]) is the authoring
    /// path (it computes tangent geometry exactly and declares by
    /// construction); [`RawLoop::with_tangent_joints`] is the
    /// explicit hand-authoring/persistence form.
    ///
    /// [`ProfileError::TangencyContradicted`]: validate::ProfileError::TangencyContradicted
    /// [`ProfileError::UndeclaredTangency`]: validate::ProfileError::UndeclaredTangency
    pub fn tangent_joints(&self) -> &[usize] {
        &self.tangent_joints
    }

    /// The reversed chain: the same locus traversed the other way.
    ///
    /// Reindexing: the reversed chain visits `v0, v(n−1), v(n−2), …,
    /// v1`, and each segment's bulge is the **negation** of the original
    /// segment it retraces (b ↦ −b, the reversal involution of the crate
    /// docs): `reversed[k] = { pos: v[(n−k) mod n].pos,
    /// bulge: −v[(n−k−1) mod n].bulge }`.
    ///
    /// `reversed ∘ reversed` is the identity bit-exactly (the
    /// reindexing round-trips and IEEE negation is exact) — under test.
    ///
    /// Declared-tangent joints travel with their vertex: joint j maps
    /// to (n − j) mod n, the reversed chain's index of the same
    /// geometric junction (an involution, so the round-trip is exact
    /// elementwise).
    pub fn reversed(&self) -> Self {
        let n = self.vertices.len();
        if n == 0 {
            return self.clone();
        }
        Self {
            vertices: (0..n)
                .map(|k| ProfileVertex {
                    pos: self.vertices[(n - k) % n].pos,
                    bulge: -self.vertices[(n - k - 1) % n].bulge,
                })
                .collect(),
            // Out-of-range indices (garbage data) pass through
            // untouched — total code; validation refuses them typed.
            tangent_joints: self
                .tangent_joints
                .iter()
                .map(|&j| if j < n { (n - j) % n } else { j })
                .collect(),
        }
    }
}

/// The rigid placement of a sketch plane in 3-space: profile (x, y) ↦
/// `placement`·(x, y, 0) = plane origin + x·u + y·v, where u, v, and the
/// plane normal are the columns of the placement's linear part
/// (`linear.c0`, `linear.c1`, `linear.c2`).
///
/// Rigidity — u, v, normal orthonormal and right-handed
/// (normal = u × v) — is **conventional data, unchecked** (the crate
/// docs' plane convention; the same posture as `geom`'s unit
/// `dir`/`u_ref` fields). Tier-3 geometric validation certifies it at
/// rest; a non-rigid placement yields a well-defined skewed sketch, not
/// poison.
#[derive(Clone, Copy, Debug)]
pub struct SketchPlane<T: Real> {
    /// The placement map (rigid by convention).
    pub placement: Affine3<T>,
}

impl<T: Real> SketchPlane<T> {
    /// Wraps a placement map.
    pub fn new(placement: Affine3<T>) -> Self {
        Self { placement }
    }

    /// The world xy-plane: identity placement (u = x̂, v = ŷ,
    /// normal = ẑ, origin at the world origin).
    pub fn xy() -> Self {
        Self::new(Affine3::identity())
    }

    /// The world yz-plane: u = ŷ, v = ẑ, normal = ŷ × ẑ = x̂, origin at
    /// the world origin.
    ///
    /// The cyclic convention (x→y→z→x) is what the tour's letterforms
    /// captions mean by "a yz sketch extruded +x": sketch (x, y) maps
    /// to world (0, x, y), and the extrusion normal — the third
    /// placement column — is +x̂.
    pub fn yz() -> Self {
        Self::from_frame(Point3::origin(), Vec3::unit_y(), Vec3::unit_z())
    }

    /// The world zx-plane: u = ẑ, v = x̂, normal = ẑ × x̂ = ŷ, origin at
    /// the world origin.
    ///
    /// Same cyclic convention as [`Self::yz`] one step further round,
    /// so the captions' "a zx sketch extruded +y" is literal: sketch
    /// (x, y) maps to world (y, 0, x), and the extrusion normal is +ŷ.
    pub fn zx() -> Self {
        Self::from_frame(Point3::origin(), Vec3::unit_z(), Vec3::unit_x())
    }

    /// The plane through `origin` spanned by `u` and `v`, with
    /// normal = u × v (computed, keeping the frame right-handed by
    /// construction when u ⊥ v are unit — which is the caller's
    /// conventional obligation, unchecked).
    pub fn from_frame(origin: Point3<T>, u: Vec3<T>, v: Vec3<T>) -> Self {
        Self::new(Affine3::from_parts(
            Mat3::from_cols(u, v, u.cross(v)),
            origin - Point3::origin(),
        ))
    }

    /// Maps a sketch point to world space: `placement`·(x, y, 0),
    /// exactly [`Affine3::transform_point`] on the embedded point (fixed
    /// order, D9).
    pub fn to_world(&self, p: Point2<T>) -> Point3<T> {
        self.placement
            .transform_point(Point3::new(p.x, p.y, T::zero()))
    }

    /// The plane's origin — sketch (0, 0) in world space.
    ///
    /// The four accessors below READ the frame [`from_frame`] wrote:
    /// they are projections of `placement`, never a recomputation, so
    /// `from_frame(o, u, v)` round-trips through them BITWISE. That is
    /// why the translation is transcribed component by component
    /// rather than added to the coordinate origin — `0 + (-0) = 0`
    /// would quietly launder a signed zero the stored frame kept.
    ///
    /// [`from_frame`]: Self::from_frame
    pub fn origin(&self) -> Point3<T> {
        let t = self.placement.translation;
        Point3::new(t.x, t.y, t.z)
    }

    /// The plane's u direction — the placement's first linear column,
    /// the world direction sketch +x runs.
    pub fn u(&self) -> Vec3<T> {
        self.placement.linear.c0
    }

    /// The plane's v direction — the placement's second linear column,
    /// the world direction sketch +y runs.
    pub fn v(&self) -> Vec3<T> {
        self.placement.linear.c1
    }

    /// The plane's normal — the placement's third linear column, which
    /// [`from_frame`] filled with u × v. This is the direction an
    /// extrusion of a profile on this plane runs.
    ///
    /// [`from_frame`]: Self::from_frame
    pub fn normal(&self) -> Vec3<T> {
        self.placement.linear.c2
    }
}

impl SketchPlane<f64> {
    /// Bit-exact frame equality (the `Doc::bit_eq` precedent, spec
    /// D7): every one of the twelve stored components compared by
    /// BITS, so `0.0` and `-0.0` are different planes here.
    ///
    /// Bit comparison, not tolerance comparison, is the only equality
    /// this type can honestly offer: a sketch plane carries no ε, and
    /// "the same plane" up to tolerance is a geometric question the
    /// kernel answers about BODIES, at tier 3. Two planes equal here
    /// place every sketch point identically, by construction.
    pub fn bit_eq(&self, other: &Self) -> bool {
        let bits = |p: &Self| {
            let (o, l) = (p.origin(), p.placement.linear);
            [
                o.x, o.y, o.z, l.c0.x, l.c0.y, l.c0.z, l.c1.x, l.c1.y, l.c1.z, l.c2.x, l.c2.y,
                l.c2.z,
            ]
            .map(f64::to_bits)
        };
        bits(self) == bits(other)
    }
}

/// A sketch profile: closed loops on a sketch plane — the raw input
/// data. [`Profile::validate`] is the only way to make it consumable by
/// sweeps.
#[derive(Clone, Debug)]
pub struct Profile<T: Real> {
    /// The plane the profile lives on (passed through validation
    /// untouched — validation is 2-D).
    pub plane: SketchPlane<T>,
    /// The loops, in any order and either winding (winding is
    /// invisible; nesting is derived by validation).
    pub loops: Vec<ProfileLoop<T>>,
}

impl<T: Real> Profile<T> {
    /// Builds a profile from a plane and loops.
    pub fn new(plane: SketchPlane<T>, loops: Vec<ProfileLoop<T>>) -> Self {
        Self { plane, loops }
    }
}
