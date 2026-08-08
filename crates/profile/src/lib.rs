//! 2-D sketch profiles as data: the bulge-chain [`Profile`] and its
//! trilean validation into [`ValidatedProfile`] (M2 PR 2).
//!
//! A profile is the *input* to sweeps (M2 PR 4/5): closed 2-D loops on a
//! [`SketchPlane`], stored as plain data with **zero
//! representation-consistency conditions** (the D2 peer-representation
//! lesson applied at the input boundary, ratified in the PR #24
//! conversation — see `docs/M2-PLAN.md` PR 2). Sweeps accept only a
//! [`ValidatedProfile`], the canonicalized output of
//! [`Profile::validate`]; arcs lower to `geom-curves` circle carriers at
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
//!   vertices-derive-bounds rule (`geom-curves` crate docs).
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
//!   primary authoring path is [`LoopBuilder::fillet`], which computes
//!   tangent geometry exactly and declares by construction;
//!   [`LoopBuilder::declare_tangent`] is the explicit flag for
//!   hand-authored chains.
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
//! predicate inventory and `crate::k_stats` for the margin-statistics
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
//! kernel-wide boundary (shared with `topo`/`geom-curves`), not a
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

pub mod k_stats;
pub mod path;
mod seg;
mod sugar;
mod validate;

use geom_core::{Affine3, Mat3, Point2, Point3, Real, Vec3};

pub use path::{
    ArcCenterTarget, ArcTarget, ArcViaTarget, LineTarget, Open, PartialPath, PathError, Start,
    TangentArcTarget, circle,
};
pub use sugar::{ArcSweep, FilletLegShape, LoopBuilder, bulge_from_center, bulge_from_via};
pub use validate::{
    ContactKind, EscalationSite, FilletLeg, FilletLegCarrier, LoopRole, NoCornerReason,
    ProfileError, SegmentKind, SegmentRef, ValidatedLoop, ValidatedProfile, ValidatedSegment,
};

/// One vertex of a profile loop: a position plus the bulge of the
/// segment *leaving* it toward the next vertex (see the crate docs'
/// bulge semantics).
#[derive(Clone, Copy, Debug)]
pub struct ProfileVertex<T: Real> {
    /// The vertex position in sketch-plane coordinates (meters).
    pub pos: Point2<T>,
    /// The bulge b = tan(θ/4) of the segment from this vertex to the
    /// next (0 ⇒ line; sign per the crate docs — positive sweeps
    /// counterclockwise). The last vertex's bulge belongs to the
    /// implicit closing segment.
    pub bulge: T,
}

/// A closed loop: a vertex chain, closed by construction (the last
/// vertex's segment returns to the first — see the crate docs).
///
/// Plain data with public fields (conventions are carried by data, D2);
/// nothing is checked at construction — [`Profile::validate`] is the
/// gate.
#[derive(Clone, Debug)]
pub struct ProfileLoop<T: Real> {
    /// The vertex chain, in traversal order (either winding — winding
    /// is invisible, see the crate docs).
    pub vertices: Vec<ProfileVertex<T>>,
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
    /// [`LoopBuilder::fillet`] is the primary authoring path (it
    /// computes tangent geometry exactly and declares by construction);
    /// [`LoopBuilder::declare_tangent`] and this field are the
    /// explicit hand-authoring/persistence form.
    ///
    /// [`ProfileError::TangencyContradicted`]: validate::ProfileError::TangencyContradicted
    /// [`ProfileError::UndeclaredTangency`]: validate::ProfileError::UndeclaredTangency
    pub tangent_joints: Vec<usize>,
}

impl<T: Real> ProfileLoop<T> {
    /// Builds a loop from a vertex chain (no declared-tangent joints;
    /// set [`ProfileLoop::tangent_joints`] or use the builder's
    /// declaration paths for those).
    pub fn new(vertices: Vec<ProfileVertex<T>>) -> Self {
        Self {
            vertices,
            tangent_joints: Vec::new(),
        }
    }

    /// Builds a loop of straight segments through the given points (all
    /// bulges zero) — polygon sugar.
    pub fn polygon(points: impl IntoIterator<Item = Point2<T>>) -> Self {
        Self::new(
            points
                .into_iter()
                .map(|pos| ProfileVertex {
                    pos,
                    bulge: T::zero(),
                })
                .collect(),
        )
    }

    /// Starts a [`LoopBuilder`] at `start` — the human-friendly
    /// constructor for mixed line/arc chains (see [`LoopBuilder`]).
    pub fn builder(start: Point2<T>) -> LoopBuilder<T> {
        LoopBuilder::start(start)
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
/// docs' plane convention; the same posture as `geom-curves`' unit
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
