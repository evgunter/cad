//! **The kernel query seat** (`docs/VERB-SEAT-DESIGN.md` §1) — the
//! geometric half of the selection vocabulary as pure functions of a
//! [`Body`], at the layer whose types they serve.
//!
//! The document layer's `select_where` speaks stable names and
//! delegates its per-entity geometric tests HERE; a caller holding a
//! body and arena keys — a kernel-direct consumer, a demo, a test —
//! asks the same questions through the same one implementation. Names
//! at the document door, keys at the body door, one predicate under
//! both (the `ContactClass` layering precedent: defined lowest,
//! re-exported upward).
//!
//! # The load-bearing split: EXACT vs DECIDED
//!
//! - **EXACT** — [`edge_carrier_matches`], [`face_surface_matches`],
//!   [`edge_adjacent_matches`] (and the kind reads under them) read a
//!   carrier's enum TAG. Post-#256 (always-promote; "exact analytic
//!   geometry has exactly one native representation") the tag IS the
//!   semantic kind. These predicates are total, deterministic,
//!   trivially equivariant (kinds are motion-invariant) and
//!   scalar-independent. They go through NO funnel and carry NO
//!   margin, deliberately: minting a fake margin for a tag match would
//!   be dimension-laundering in the other direction. A missing
//!   carrier, a dangling key or an unreadable adjacency is an honest
//!   NO, never a panic — which is what makes a purely-exact filter
//!   total.
//!
//!   **[`rim_of`] is a fourth EXACT door and it does NOT answer NO.**
//!   It reads stored data the same way the three predicates do — the
//!   carrier's tag, then its `center`, `radius` and `axis` compared
//!   BIT for bit, and side surface KEYS — with no funnel, no margin
//!   and nothing decided. What it does differently is its answer
//!   shape: a predicate returns a `bool`, so "the key dangles" and
//!   "the kind is wrong" can both honestly be NO; a door that returns
//!   a SET has no such spelling, because an empty set and a partial
//!   set are both answers a caller would act on. So it refuses typed
//!   ([`RimError`]) at every point a predicate would answer NO, and
//!   the totality is in the refusals rather than in the `false`.
//! - **DECIDED** — [`datum_distance_sign`] is a real numeric
//!   comparison and therefore a `k_stats::decide` site with a named
//!   `sel_*` predicate ([`SEL_DATUM_DISTANCE`]), an honest
//!   [`Margin`] door, and a typed indeterminate on an in-band
//!   comparand. It participates in the K census exactly like any
//!   kernel site (SELECT-DESIGN GS-Q1: the naming convention does the
//!   separating, not a second funnel).
//!
//!   **The `sel_*` convention covers the SELECTOR sites, and this
//!   module has one that is not a selector site.** The datum a
//!   selection is measured against carries a decision of its own, one
//!   layer earlier: [`UnitVec3::new`] ([`DATUM_UNIT_NORM`], no `sel_`
//!   prefix) decides that a direction has a finite, nonzero length
//!   before normalizing it. It is deliberately outside the convention
//!   — it decides nothing about a candidate and answers no selection
//!   question; it is a constructor refusing a value the type cannot
//!   hold, and a `sel_` name on it would tell a census reader it
//!   belongs to a selector margin population it is not part of. What
//!   it buys the door above is that [`datum_distance`] is arithmetic
//!   all the way down.
//!
//!   **[`is_finite_length`] is a second thing here that is not a
//!   selection question**, and unlike the one above it is public. It
//!   takes a bare scalar, reads no [`Body`] and reaches no funnel: it
//!   is the value-channel question `UnitVec3::new` asks before
//!   deciding a length, shared so that the evaluation layer's own
//!   direction door asks it in the same words rather than a second
//!   spelling. Whether it belongs here at all — `geom-core` holds
//!   `Real`, `is_poison` and `Vec3::normalize`'s own overflow note —
//!   is an open question for this seat's owner, filed as
//!   `is-finite-length-homed-in-the-query-seat`.
//!
//! # Where an entity IS, for the decided door
//!
//! The decided door measures a POINT against a [`DatumValue`]. The
//! entity → point convention is [`crate::readback`]'s, not a second
//! one minted here: a vertex's stored position
//! ([`readback::vertex_point`](crate::readback::vertex_point)), an
//! edge's certified carrier frame origin
//! ([`readback::edge_pose`](crate::readback::edge_pose)), a face's
//! carrier frame origin
//! ([`readback::face_pose`](crate::readback::face_pose)) — and the
//! read-back refusals travel with those doors (a NURBS face has no
//! canonical frame, so it refuses rather than being silently dropped).
//! Datum-node RESOLUTION — a recipe reference becoming a
//! [`DatumValue`] — stays in the document layer; this seat takes the
//! resolved value. One half of what used to be up there came down with
//! the type: the document layer no longer normalizes a datum's
//! direction by hand, because [`UnitVec3`] admits no unnormalized
//! spelling, so the normalization and its typed refusal are HERE and
//! the document layer maps that refusal onto its own node error.
//! Stable names themselves never appear below the G1 line, which is
//! the point.

use geom::Curve3;
use geom_brep::{SurfaceKey, SurfaceKind};
use geom_core::k_stats::decide;
use geom_core::{
    Band, Bounds, Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Vec2, Vec3,
};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::null::CurveGeom;

/// Which [`Curve3`] variant a carrier is: the fieldless mirror of the
/// curve enum, and the edge-side twin of [`SurfaceKind`].
///
/// The mirror is hand-written and [`CurveKind::of`]'s match is
/// EXHAUSTIVE with no wildcard arm, so adding a `Curve3` variant fails
/// to compile here rather than silently classifying as something else
/// — the same fail-loud tripwire the role-segment mirrors use.
///
/// (Placement: the mirror lives where it is used — [`SurfaceKind`]
/// beside the certify machinery in `geom-brep`, this one beside the
/// query predicates that read it. `SurfaceKind` stays the workspace's
/// ONE fieldless surface mirror; no second is minted here.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CurveKind {
    /// [`Curve3::Line`].
    Line,
    /// [`Curve3::Circle`].
    Circle,
    /// [`Curve3::Ellipse`].
    Ellipse,
    /// [`Curve3::Nurbs`].
    Nurbs,
}

impl CurveKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::Line, Self::Circle, Self::Ellipse, Self::Nurbs];

    /// The kind of a carrier (exhaustive by construction — type docs).
    #[must_use]
    pub fn of<T: Real>(c: &Curve3<T>) -> Self {
        match c {
            Curve3::Line { .. } => Self::Line,
            Curve3::Circle { .. } => Self::Circle,
            Curve3::Ellipse { .. } => Self::Ellipse,
            Curve3::Nurbs(_) => Self::Nurbs,
        }
    }

    /// This kind's bit position in a [`CurveKindSet`].
    const fn bit(self) -> u8 {
        match self {
            Self::Line => 0,
            Self::Circle => 1,
            Self::Ellipse => 2,
            Self::Nurbs => 3,
        }
    }
}

/// A SET of [`CurveKind`]s — the predicate's comparand, so "a line or
/// an arc" is one predicate rather than a union of two selections.
///
/// A bitset, not a `Vec`: the mirror is closed and tiny, so the value
/// is `Copy`, `Ord` and canonical (no ordering or duplicate freedom to
/// disagree about between two equal sets).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CurveKindSet(u8);

impl CurveKindSet {
    /// The set of exactly these kinds. An EMPTY set matches nothing
    /// (the same posture as an empty document-layer `Selector`).
    #[must_use]
    pub fn of(kinds: impl IntoIterator<Item = CurveKind>) -> Self {
        Self(kinds.into_iter().fold(0, |acc, k| acc | (1 << k.bit())))
    }

    /// The singleton set — the common case.
    #[must_use]
    pub fn just(kind: CurveKind) -> Self {
        Self::of([kind])
    }

    /// Whether `kind` is a member.
    #[must_use]
    pub fn contains(self, kind: CurveKind) -> bool {
        self.0 & (1 << kind.bit()) != 0
    }

    /// Whether the set is empty (matches nothing).
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The members, in [`CurveKind::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = CurveKind> {
        CurveKind::ALL
            .into_iter()
            .filter(move |k| self.contains(*k))
    }
}

/// The [`SurfaceKind`] bit position in a [`SurfaceKindSet`].
///
/// EXHAUSTIVE with no wildcard arm: a new `SurfaceKind` variant fails
/// to compile here (and `ALL_SURFACE_KINDS` below is pinned against
/// this function by a unit test, so the pair cannot drift apart).
const fn surface_bit(kind: SurfaceKind) -> u8 {
    match kind {
        SurfaceKind::Plane => 0,
        SurfaceKind::Cylinder => 1,
        SurfaceKind::Cone => 2,
        SurfaceKind::Sphere => 3,
        SurfaceKind::Torus => 4,
        SurfaceKind::Nurbs => 5,
        SurfaceKind::Approx => 6,
    }
}

/// Every [`SurfaceKind`], in declaration order — the iteration order of
/// a [`SurfaceKindSet`].
pub const ALL_SURFACE_KINDS: [SurfaceKind; 7] = [
    SurfaceKind::Plane,
    SurfaceKind::Cylinder,
    SurfaceKind::Cone,
    SurfaceKind::Sphere,
    SurfaceKind::Torus,
    SurfaceKind::Nurbs,
    SurfaceKind::Approx,
];

/// A SET of [`SurfaceKind`]s — [`CurveKindSet`]'s face-side twin, and
/// the comparand of both [`face_surface_matches`] and each side of
/// [`edge_adjacent_matches`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SurfaceKindSet(u8);

impl SurfaceKindSet {
    /// The set of exactly these kinds. An EMPTY set matches nothing.
    #[must_use]
    pub fn of(kinds: impl IntoIterator<Item = SurfaceKind>) -> Self {
        Self(
            kinds
                .into_iter()
                .fold(0, |acc, k| acc | (1 << surface_bit(k))),
        )
    }

    /// The singleton set — the common case.
    #[must_use]
    pub fn just(kind: SurfaceKind) -> Self {
        Self::of([kind])
    }

    /// Whether `kind` is a member.
    #[must_use]
    pub fn contains(self, kind: SurfaceKind) -> bool {
        self.0 & (1 << surface_bit(kind)) != 0
    }

    /// Whether the set is empty (matches nothing).
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The members, in [`ALL_SURFACE_KINDS`] order.
    pub fn iter(self) -> impl Iterator<Item = SurfaceKind> {
        ALL_SURFACE_KINDS
            .into_iter()
            .filter(move |k| self.contains(*k))
    }
}

// ---------------------------------------------------------------
// Materializers.
// ---------------------------------------------------------------

/// Every edge key of a body, in slot-index order (deterministic per
/// D9 — see [`Body::edges`]). "All of them", as one door: the
/// whole-body selection a caller hands to a key-taking verb.
#[must_use]
pub fn all_edges<T: Real>(body: &Body<T>) -> Vec<EdgeKey> {
    body.edges().map(|(k, _)| k).collect()
}

/// Every face key of a body, in slot-index order (deterministic per
/// D9) — [`all_edges`]'s face-side sibling.
#[must_use]
pub fn all_faces<T: Real>(body: &Body<T>) -> Vec<FaceKey> {
    body.faces().map(|(k, _)| k).collect()
}

// ---------------------------------------------------------------
// The EXACT predicates: total tag reads, no funnel, no margin.
// ---------------------------------------------------------------

/// The certified carrier kind of an edge, or `None` for a dangling key
/// or an uncertified (null-scaffold) carrier — for the EXACT
/// predicates, "no carrier" is an honest no, not a refusal.
#[must_use]
pub fn edge_carrier_kind<T: Real>(body: &Body<T>, e: EdgeKey) -> Option<CurveKind> {
    let curve = body.get_edge(e)?.curve;
    body.get_curve_geom(curve)
        .and_then(CurveGeom::certified)
        .map(|c| CurveKind::of(c.carrier()))
}

/// The surface kind of a face, or `None` for a dangling key or an
/// unreadable surface reference.
#[must_use]
pub fn face_surface_kind<T: Real>(body: &Body<T>, f: FaceKey) -> Option<SurfaceKind> {
    body.get_face(f)
        .and_then(|face| body.get_surface(face.surface))
        .map(SurfaceKind::of)
}

/// The surface kind on one side of an edge, or `None` where the
/// adjacency or its geometry is not there to read.
fn face_kind_across<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> Option<SurfaceKind> {
    let h = body.get_half_edge(he)?;
    let f = body.get_loop(h.parent_loop)?.face;
    body.get_surface(body.get_face(f)?.surface)
        .map(SurfaceKind::of)
}

/// EXACT: whether the edge's certified carrier kind is a member of
/// `kinds`. Total — a missing edge or carrier is an honest NO.
#[must_use]
pub fn edge_carrier_matches<T: Real>(body: &Body<T>, e: EdgeKey, kinds: CurveKindSet) -> bool {
    edge_carrier_kind(body, e).is_some_and(|k| kinds.contains(k))
}

/// EXACT: whether the face's surface kind is a member of `kinds`.
/// Total — a missing face or surface is an honest NO.
#[must_use]
pub fn face_surface_matches<T: Real>(body: &Body<T>, f: FaceKey, kinds: SurfaceKindSet) -> bool {
    face_surface_kind(body, f).is_some_and(|k| kinds.contains(k))
}

/// EXACT: whether the two faces across an edge have kinds drawn one
/// from each set — UNORDERED, so `(Plane, Sphere)` matches a rim
/// whichever half-edge carries which. The unordered reading is what
/// makes the predicate equivariant under a reflection that swaps the
/// sides. Total — a side whose face kind cannot be read is an honest
/// NO.
#[must_use]
pub fn edge_adjacent_matches<T: Real>(
    body: &Body<T>,
    e: EdgeKey,
    a: SurfaceKindSet,
    b: SurfaceKindSet,
) -> bool {
    body.get_edge(e).is_some_and(|edge| {
        match (
            face_kind_across(body, edge.he_plus),
            face_kind_across(body, edge.he_minus),
        ) {
            (Some(p), Some(m)) => {
                (a.contains(p) && b.contains(m)) || (a.contains(m) && b.contains(p))
            }
            (None, _) | (_, None) => false,
        }
    })
}

// ---------------------------------------------------------------
// The DECIDED door and the type it measures against: two funnel
// sites — the datum's own unit-direction constructor, and the
// distance-sign door — each with an honest Margin and a typed
// indeterminate.
// ---------------------------------------------------------------

/// **A direction that cannot be unnormalized.** [`UnitVec3::new`] is
/// the only way to spell one; it normalizes a vector whose length is a
/// finite, definitely-nonzero number and refuses every other input
/// typed — so a plane normal or an axis direction held here is unit as
/// a property of the TYPE, not of the caller's diligence, and it stays
/// unit after it is copied back out of whatever structure holds it.
///
/// The signed distance to a plane is a length only against a unit
/// normal, so an unnormalized one silently scales a DECIDED
/// predicate's comparand — a wrong [`Sign`] with no refusal. That
/// failure is unrepresentable rather than asserted.
///
/// **"Every other input" includes the ones a length comparison alone
/// cannot see.** A vector whose components overflow the norm
/// (`|v| ≳ 1e154` at `f64`) has an INFINITE length, which the scalar's
/// own [`Decide`] machinery calls maximally definite — a `Positive`
/// answer, followed by a division that collapses the direction to
/// zero. The constructor therefore asks whether the length is a finite
/// number BEFORE asking which side of zero it lies on
/// ([`UnitVec3Error::NonFiniteLength`]); without that order the type's
/// guarantee would be false exactly where it is least visible.
#[derive(Debug, Clone, Copy)]
pub struct UnitVec3<T: Real>(Vec3<T>);

/// **The funnel site name** of the unit-direction constructor's length
/// decision. Its comparand is a genuine length (the vector's norm), so
/// it goes through the plain [`Margin::norm3`] door and owes NO
/// `docs/predicate-dimension-audit.md` row.
///
/// A K row name reaching the funnel through a const, not a literal at
/// the decide site, so it is a roster carrier (`docs/K-REPORT.md`,
/// "The inventory method, restated").
pub const DATUM_UNIT_NORM: &str = "datum_unit_norm";

/// Why a vector could not become a [`UnitVec3`] — a closed enum (D4
/// ¶3): every arm is a fact about the input, never a lane to swallow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitVec3Error {
    /// The vector's length decided to zero: it names no direction, and
    /// picking one for it would be invention (spec D3).
    Degenerate,
    /// The vector's length is not a finite number — the components
    /// overflow the norm (`|v| ≳ 1e154` at `f64`), or one of them is
    /// the scalar's poison. Refused BEFORE the length is decided,
    /// because an infinite margin is maximally definite to
    /// [`Decide`] and would be normalized into a zero direction; a
    /// poisoned one has no direction either.
    NonFiniteLength,
    /// The length decision landed in the ambiguity band — at the
    /// interval scalar, an enclosure that straddles "has a direction"
    /// and "does not". Escalated unaltered.
    Escalated(Indeterminate),
}

impl core::fmt::Display for UnitVec3Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Degenerate => f.write_str(
                "a direction vector decided to zero length, so it names no \
                 direction to normalize",
            ),
            Self::NonFiniteLength => f.write_str(
                "a direction vector's length is not a finite number — its \
                 components overflow the norm, or one of them is not a \
                 number; scale the geometry into the session's range",
            ),
            Self::Escalated(source) => {
                write!(f, "a direction vector's length is indeterminate: {source}")
            }
        }
    }
}

impl std::error::Error for UnitVec3Error {}

/// **Is `x` a finite number at this scalar?** — asked through the
/// value channel every [`Real`] has, with no bracket read and no
/// threshold invented.
///
/// A finite value less itself is exactly zero; `∞ − ∞` and `NaN − NaN`
/// are the scalar's poison. So the self-difference IS the question,
/// which is why the equal-operands lint is allowed here and nowhere
/// near it. An enclosure answers YES however wide it is (a finite
/// interval's self-difference is a finite interval around zero, and an
/// enclosure whose upper end overflowed still contains its truth) —
/// the honest scope: this catches the point scalars, which is where an
/// infinite length turns into a definite wrong answer.
///
/// One rule, one spelling, and now one CALLER: every 3-D direction
/// door in the workspace asks this question through
/// [`unit_direction`], which is the only place it is asked before a
/// length is classified — the datum constructor and `editor-core`'s
/// own direction door are two calls to that one body under two funnel
/// names.
///
/// It is still not asked everywhere a direction is normalized, and the
/// honest scope is the remaining list: `editor-core`'s
/// `Frame::rotate_then_translate` asks nothing and is refused
/// downstream on the non-finite frame it builds; its
/// `clearance::chart_frame` asks a different question (a bracket read
/// of the normalized OUTPUT); and `profile`'s 2-D director doors
/// decide a length they never asked to be finite and cannot reach
/// this predicate at all, because `profile` depends on `geom-core`
/// alone and this crate sits above it
/// (`work/seat/two-d-director-doors-skip-the-finiteness-question`).
pub fn is_finite_length<T: Real>(x: T) -> bool {
    #[allow(clippy::eq_op)]
    let residual = x - x;
    !residual.is_poison()
}

/// **The direction-length decision, and the workspace's only spelling
/// of it**: is the length a finite number, which side of zero is it
/// on, and — only then — the normalized ray or a typed refusal.
///
/// Two questions in this order, and the order is the point.
///
/// 1. **Is the length a finite number?** Asked through the value
///    channel every scalar has ([`is_finite_length`]): a finite value
///    less itself is exactly zero, while `∞ − ∞` and `NaN − NaN` are
///    the scalar's poison. No bracket is read and no threshold is
///    invented, so an enclosure of any width passes — the arm bites at
///    the point scalars, which is where the failure is (an interval
///    whose norm overflowed still ENCLOSES the truth, so it stays
///    sound and simply refuses later, where a `f64` would answer a
///    definite wrong sign).
/// 2. **Which side of zero is it on?** Through the scalar's own
///    decision machinery ([`Margin::norm3`] on the caller's band) —
///    [`Real`] deliberately has no comparison surface, and a
///    hand-rolled `> 0` would be wrong at the interval scalar. Only a
///    DEFINITELY zero length refuses, so a wide enclosure that
///    contains a real direction never refuses spuriously; one that
///    straddles zero escalates instead of guessing.
///
/// **`site` is the funnel name, and it is a parameter because the NAME
/// belongs to the layer that owns the value while the DECISION belongs
/// here.** A datum's normal or axis direction is decided under
/// [`DATUM_UNIT_NORM`], through [`UnitVec3::new`] — the type that
/// holds it has no unnormalized spelling, so the invariant is the
/// type's. A transform's rotation axis and a pattern's direction are
/// decided under `eval_direction_norm`, at `editor-core`'s own
/// direction door, because the evaluation layer owns those values and
/// its K telemetry is read per layer. Two names, one body: the
/// alternative — one name — would erase which layer a length decision
/// came from, and the alternative to one body was the six lines living
/// twice, which is how the two copies came to differ in the first
/// place.
///
/// The site is a `&'static str` and never a stored field or an enum:
/// nothing here dispatches on it, and a kernel that had to name its
/// callers would grow a variant per caller.
///
/// # Errors
///
/// [`UnitVec3Error::NonFiniteLength`] on an overflowed or poisoned
/// length, [`UnitVec3Error::Degenerate`] on a decided-zero one,
/// [`UnitVec3Error::Escalated`] on an in-band one.
pub fn unit_direction<T: Decide>(
    v: Vec3<T>,
    site: &'static str,
    band: Band,
) -> Result<Vec3<T>, UnitVec3Error> {
    // `norm3` below recomputes this same value (`Vec3::norm` is
    // deterministic), so the gate and the margin are the one length;
    // it is spelled twice rather than reached into.
    if !is_finite_length(v.norm()) {
        return Err(UnitVec3Error::NonFiniteLength);
    }
    match decide(site, Margin::norm3(v), band) {
        Ok(Sign::Positive) => Ok(v.normalize()),
        Ok(_) => Err(UnitVec3Error::Degenerate),
        Err(source) => Err(UnitVec3Error::Escalated(source)),
    }
}

impl<T: Real> UnitVec3<T> {
    /// The direction itself, unit.
    #[must_use]
    pub fn get(self) -> Vec3<T> {
        self.0
    }
}

impl<T: Decide> UnitVec3<T> {
    /// **The only constructor**: `v` normalized, or a typed refusal.
    ///
    /// The decision itself — finiteness first, then which side of zero
    /// the length lies on, then normalize or refuse — is
    /// [`unit_direction`], which the evaluation layer's own direction
    /// door calls too; the two questions and the reason for their
    /// order are documented there. What this constructor adds is the
    /// TYPE: a direction that reaches it comes out unit as a property
    /// of the type rather than of the caller's diligence, and the
    /// funnel name it decides under is [`DATUM_UNIT_NORM`], because a
    /// datum's normal or axis direction is a value this layer owns.
    ///
    /// # Errors
    ///
    /// [`UnitVec3Error::NonFiniteLength`] on an overflowed or poisoned
    /// length, [`UnitVec3Error::Degenerate`] on a decided-zero one,
    /// [`UnitVec3Error::Escalated`] on an in-band one.
    pub fn new(v: Vec3<T>, band: Band) -> Result<Self, UnitVec3Error> {
        unit_direction(v, DATUM_UNIT_NORM, band).map(Self)
    }
}

/// A resolved datum: geometry VALUES, not kernel entities and not
/// recipe references. Normals and axis directions are [`UnitVec3`],
/// which is unit by construction — nothing here re-normalizes (not
/// bit-preserving) and nothing needs to: an unnormalized datum has no
/// spelling.
#[derive(Debug, Clone)]
pub enum DatumValue<T: Real> {
    /// A plane through `origin` with `normal`.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The normal.
        normal: UnitVec3<T>,
    },
    /// An axis through `origin` along `dir`.
    Axis {
        /// A point on the axis.
        origin: Point3<T>,
        /// The direction.
        dir: UnitVec3<T>,
    },
    /// A point.
    Point {
        /// Its position.
        position: Point3<T>,
    },
    /// **An oriented plane** — origin plus a right-handed pair of
    /// in-plane directions, so the surface AND the spin about its
    /// normal are pinned.
    ///
    /// A [`DatumValue::Plane`] fixes five of a placement's six rigid
    /// degrees of freedom; the sixth, the rotation about the normal,
    /// is exactly what a sketch's `u` and `v` axes are. Anything that
    /// only measures against the SURFACE (a section cut,
    /// [`datum_distance`]) wants the plane and would have to ignore
    /// the spin; anything that reads or writes 2D coordinates on the
    /// plane needs the frame, because there is nothing else to hang an
    /// `(x, y)` pair on. The two are separate variants for that
    /// reason, not as a naming accident.
    ///
    /// `u` and `v` are unit by their type and ORTHOGONAL by the
    /// contract of whoever built the value — the evaluation layer
    /// orthonormalizes and refuses a degenerate pair loudly, so a
    /// frame reaching a consumer spans a plane. The normal is `u × v`,
    /// computed rather than stored: storing it would be a second
    /// opinion that could come to disagree with the pair.
    Frame {
        /// Sketch (0, 0) in world space.
        origin: Point3<T>,
        /// The first in-plane direction — sketch +x.
        u: UnitVec3<T>,
        /// The second in-plane direction — sketch +y, perpendicular to
        /// `u`.
        v: UnitVec3<T>,
    },
    /// **An axis that lives in a sketch frame**, carried in BOTH
    /// spellings — the frame's own 2-D coordinates, and the world
    /// line those coordinates name.
    ///
    /// Neither is derivable from this value alone (the frame is not in
    /// it), and the two have different readers: a revolve consumes the
    /// sketch pair, because a `RevolveAxis` IS sketch-plane metres and
    /// a round trip out to world and back would round the numbers a
    /// person typed; everything that measures or draws in 3-D consumes
    /// the world line. Carrying one and deriving the other at each
    /// reader would put the lift in two places.
    AxisInPlane {
        /// A point on the axis in the frame's 2-D coordinates, as
        /// authored.
        plane_origin: Point2<T>,
        /// The axis direction in the frame's 2-D coordinates, as
        /// authored and NOT normalized: `RevolveAxis` takes "any
        /// definitely nonzero vector" and refuses a sliver at its own
        /// door, so normalizing here would be a second opinion about
        /// the same vector. The lift below is unit because a 3-D
        /// direction in this vocabulary always is, and because the
        /// frame's axes are orthonormal the two refusals coincide
        /// exactly: `|lift(d)| = |d|`.
        plane_dir: Vec2<T>,
        /// The same axis lifted through its frame — a point on it in
        /// world space.
        origin: Point3<T>,
        /// The same axis lifted through its frame — its world
        /// direction, unit.
        dir: UnitVec3<T>,
    },
}

impl<T: Real> DatumValue<T> {
    /// A frame's normal, `u × v` — unit because a unit orthogonal pair
    /// crosses to a unit vector, so this is a projection of the frame
    /// and not a renormalization.
    ///
    /// Spelled here rather than at each reader for the reason the
    /// variant's own doc gives: the normal is DERIVED, and a consumer
    /// that recomputed it locally would be the place the two spellings
    /// drift apart.
    #[must_use]
    pub fn frame_normal(u: UnitVec3<T>, v: UnitVec3<T>) -> Vec3<T> {
        u.get().cross(v.get())
    }
}

/// **The funnel site name** of the decided position predicate — the
/// `sel_*` prefix SELECT-DESIGN §1 proposes, so any K-census consumer
/// can tell selector margins from kernel ones by name alone (GS-Q1's
/// separation mechanism). Its comparand is a genuine length (the
/// signed/unsigned distance minus the stated value), so it goes
/// through the plain [`Margin::of`] door and owes NO
/// `docs/predicate-dimension-audit.md` row — the flagged lane is for
/// comparands that cannot honestly be lengths.
///
/// A K row name reaching the funnel through a const, not a literal at
/// the decide site, so it is a roster carrier (`docs/K-REPORT.md`,
/// "The inventory method, restated").
pub const SEL_DATUM_DISTANCE: &str = "sel_datum_distance";

/// The distance of `p` from a datum: SIGNED along a plane's or a
/// frame's normal (which is unit by construction, so the dot product
/// is already a length), UNSIGNED to an axis or a point.
///
/// A frame answers as the plane it lies in — its spin about the normal
/// is exactly the datum this measurement does not read.
///
/// Arithmetic only, so [`Real`] is the whole bound: deciding what the
/// distance MEANS is [`datum_distance_sign`]'s job, and that is where
/// [`Decide`] enters.
#[must_use]
pub fn datum_distance<T: Real>(datum: &DatumValue<T>, p: Point3<T>) -> T {
    match datum {
        DatumValue::Plane { origin, normal } => (p - *origin).dot(normal.get()),
        DatumValue::Axis { origin, dir } => {
            let d = dir.get();
            let v = p - *origin;
            (v - d * v.dot(d)).norm()
        }
        DatumValue::Point { position } => (p - *position).norm(),
        DatumValue::Frame { origin, u, v } => (p - *origin).dot(DatumValue::frame_normal(*u, *v)),
        // The world lift, by the same arithmetic the 3-D axis uses —
        // an axis is an axis to a measurement, whichever coordinates
        // it was written in.
        DatumValue::AxisInPlane { origin, dir, .. } => {
            let d = dir.get();
            let v = p - *origin;
            (v - d * v.dot(d)).norm()
        }
    }
}

/// DECIDED: which side of the stated `value` the point's
/// [`datum_distance`] lands on, through the [`SEL_DATUM_DISTANCE`]
/// funnel. The comparand is the distance MINUS the stated value — a
/// length minus a length, so [`Margin::of`] is the honest door.
///
/// # Errors
///
/// The funnel's [`Indeterminate`] when the margin lands strictly
/// inside the ambiguity band: neither side of the comparison is
/// certified, and a caller must neither include nor drop the
/// candidate silently.
pub fn datum_distance_sign<T: Decide>(
    datum: &DatumValue<T>,
    p: Point3<T>,
    value: T,
    band: Band,
) -> Result<Sign, Indeterminate> {
    decide(
        SEL_DATUM_DISTANCE,
        Margin::of(datum_distance(datum, p) - value),
        band,
    )
}

// ---------------------------------------------------------------
// The rim door: the whole closed rim one arc belongs to. EXACT —
// stored tags and stored carriers, read bitwise; no funnel, no
// margin, no sampled geometry.
// ---------------------------------------------------------------

/// Why [`rim_of`] could not name a rim — a closed enum (D4 ¶3): every
/// arm is a fact about the seed or about the body, and none of them is
/// a lane that hands back part of a rim.
#[derive(Debug, Clone, PartialEq)]
pub enum RimError {
    /// The seed's certified carrier is not a circle, so it names no
    /// rim. `kind` is the carrier's kind, or `None` where the edge
    /// carries no certified carrier at all (a null scaffold) — the two
    /// are different facts and the payload says which.
    NotAnArc {
        /// The seed.
        edge: EdgeKey,
        /// The seed's carrier kind, `None` when it has no certified
        /// carrier.
        kind: Option<CurveKind>,
    },
    /// The seed's two sides lie on ONE surface: a chart-seam meridian,
    /// not a rim edge. A rim's two sides are two surfaces, so no seam
    /// meridian can ever be a rim's arc — which is the exclusion every
    /// hand-rolled radius scan had to remember.
    CoSurface {
        /// The seed.
        edge: EdgeKey,
        /// The one surface both its sides rest on.
        surface: SurfaceKey,
    },
    /// **The arcs that matched the seed do not form one closed chain
    /// on shared vertices**: the walk dangles at a vertex (a partial
    /// revolve's open rim is the honest instance), it branches there,
    /// or it closes leaving matched arcs unused. A partial set is
    /// never returned.
    ///
    /// **What was tested, stated so the payload can be read.** The
    /// arcs in `arcs` are the ones whose stored `center`, `radius` and
    /// `axis` are bit-equal to the seed's and whose two sides rest on
    /// the seed's two surfaces; the chain is walked over THOSE. So a
    /// refusal has two quite different causes and the payload
    /// distinguishes them: there is really a hole in the rim, or an
    /// arc of the rim is stored on a carrier this door does not call
    /// the same circle (a different `u_ref` is fine — it is not read —
    /// but a fresh `center`, `radius` or `axis`, a negated axis
    /// included, is not). A caller seeing `gap` at a parameter its
    /// body has an edge across should look at carrier identity, not
    /// for a missing edge.
    NotOneRim {
        /// Every arc that matched the seed, in arena order. The chain
        /// was walked over exactly these.
        arcs: Vec<EdgeKey>,
        /// The seed carrier's parameter at the vertex the walk stopped
        /// at — the lower end of the bracket at an enclosing scalar. A
        /// report, not a comparand: nothing in this door branches on
        /// it.
        gap: f64,
    },
    /// A dangling key or an unreadable reference on the way — the
    /// `sweep::blend` `not_intact` shape.
    NotIntact(EntityId),
}

impl core::fmt::Display for RimError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAnArc { edge, kind } => {
                // The kind is NAMED, not `Debug`-rendered: a payload
                // reaching a message through `Debug` is what the prose
                // census hunts, and words read better in a refusal. The
                // match is exhaustive with no wildcard arm, so a new
                // `CurveKind` fails to compile here; the circle arm is
                // unreachable through the door and is stated rather
                // than folded into a catch-all.
                let carries = match kind {
                    None => "no certified carrier",
                    Some(CurveKind::Line) => "a line",
                    Some(CurveKind::Circle) => "a circle",
                    Some(CurveKind::Ellipse) => "an ellipse",
                    Some(CurveKind::Nurbs) => "a NURBS curve",
                };
                write!(
                    f,
                    "edge {edge:?} carries {carries}, and a rim is named by an \
                     arc of a circle"
                )
            }
            Self::CoSurface { edge, surface } => write!(
                f,
                "edge {edge:?} has surface {surface:?} on both sides: a chart-seam \
                 meridian, and a rim's two sides are two surfaces"
            ),
            Self::NotOneRim { arcs, gap } => write!(
                f,
                "the {} arcs stored on this arc's own circle, between its two \
                 surfaces, do not form one closed chain: the walk stops at \
                 carrier parameter {gap}. Either the rim really is open there, \
                 or an arc of it is stored on a carrier this door does not call \
                 the same circle",
                arcs.len()
            ),
            Self::NotIntact(at) => write!(f, "the body is not intact at {at}"),
        }
    }
}

impl std::error::Error for RimError {}

/// A circle carrier's IDENTITY as a set of points: centre, axis and
/// radius. `u_ref` is deliberately NOT part of it — it carries the
/// seam (D2, conventional data), and one rim's arcs are minted one per
/// chart with a seam each, so their `u_ref`s differ on every
/// seam-split body in the corpus.
struct CircleId<T: Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
}

/// Are the two scalars the SAME STORED VALUE, bit for bit? At an
/// enclosing scalar both bracket ends must agree, so two enclosures
/// that merely overlap are different values. This is the whole of the
/// door's numeric comparison: no subtraction, no threshold, no funnel.
fn same_bits<T: Bounds>(a: T, b: T) -> bool {
    a.lo().to_bits() == b.lo().to_bits() && a.hi().to_bits() == b.hi().to_bits()
}

/// [`same_bits`] over a point.
fn same_point_bits<T: Bounds>(a: Point3<T>, b: Point3<T>) -> bool {
    same_bits(a.x, b.x) && same_bits(a.y, b.y) && same_bits(a.z, b.z)
}

/// [`same_bits`] over a vector.
fn same_vec_bits<T: Bounds>(a: Vec3<T>, b: Vec3<T>) -> bool {
    same_bits(a.x, b.x) && same_bits(a.y, b.y) && same_bits(a.z, b.z)
}

impl<T: Bounds> CircleId<T> {
    /// The same circle: `center`, `radius` and `axis`, each bit-equal.
    ///
    /// **The axis is compared bit-for-bit and its NEGATION is not
    /// admitted**, though `-axis` names the same point set. Admitting
    /// it would cost the order contract: an arc stored on `-axis` runs
    /// its `he_plus` the other way round the circle, so a rim carrying
    /// one answers `[a, b, c]` from one seed and `[c, b, a]` from
    /// another — a reversal, not the rotation
    /// [`rim_of`] promises. Measurement is what makes the narrower rule
    /// free: no producer in the corpus stores a rim's arcs on opposed
    /// axes, so nothing that exists is refused by this. An arc that IS
    /// stored opposed does not match, and the rim it belongs to refuses
    /// [`RimError::NotOneRim`] — an honest refusal rather than an order
    /// nobody can rely on.
    fn same_circle(&self, other: &Self) -> bool {
        same_point_bits(self.center, other.center)
            && same_bits(self.radius, other.radius)
            && same_vec_bits(self.axis, other.axis)
    }
}

/// The circle a carrier is, or `None` for any other kind.
fn circle_id<T: Real>(carrier: &Curve3<T>) -> Option<CircleId<T>> {
    match carrier {
        Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => Some(CircleId {
            center: *center,
            axis: *axis,
            radius: *radius,
        }),
        _ => None,
    }
}

/// **Where on a circle a point sits**, as the carrier's own parameter:
/// the four-quadrant angle of `p − center` in the stored frame,
/// measured against the same two vectors [`Curve3::param_near`] takes
/// at `near = 0` — the position `u_ref·radius` and the tangent
/// `(axis × u_ref)·radius`.
///
/// **The radius factor is carried, not cancelled**, and that is the
/// whole of what this had to get right. `atan2(y·r, x·r)` is
/// `atan2(y, x)` for `r > 0` and `atan2(y, x) ± π` for `r < 0`, so
/// dropping `r` — which a first cut did, calling it a folded constant
/// — silently disagrees with the door above by half a turn on a
/// carrier whose stored radius is negative. A negative radius is
/// degenerate data the constructors reject and tier 3 refuses, but a
/// refusal PAYLOAD is exactly where such a body still reaches a
/// reader, and a payload that disagrees with the tree's own parameter
/// door is worse than one that is merely surprising.
///
/// It differs from `param_near`'s circle arm only in reaching those
/// two vectors directly instead of through `eval`/`deriv`: at `θ = 0`
/// those evaluate `u_ref·cos 0 + v_ref·sin 0` and its derivative,
/// whose `v_ref` terms are exact zeros. What that leaves is a
/// signed-zero difference in a summand, which changes an answer only
/// where the whole dot product is zero and `atan2` then reads the
/// sign of a zero.
///
/// Spelled here rather than reached through that door because the door
/// is generic over every carrier kind and so carries the NURBS arm's
/// span-locate bound; taking it would make this door's bound compound,
/// which is the shape `Bounds`' scope rule exists to catch. This one
/// is arithmetic on a circle, and it feeds nothing but a refusal's
/// report.
fn circle_param<T: Real>(carrier: &Curve3<T>, p: Point3<T>) -> Option<T> {
    let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = carrier
    else {
        return None;
    };
    let w = p - *center;
    let r_near = *u_ref * *radius;
    let tau_near = axis.cross(*u_ref) * *radius;
    Some(w.dot(tau_near).atan2(w.dot(r_near)))
}

/// The surface the face across `he` rests on, or the reference that
/// could not be read.
fn surface_across<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> Result<SurfaceKey, EntityId> {
    let h = body.get_half_edge(he).ok_or(EntityId::HalfEdge(he))?;
    let l = body
        .get_loop(h.parent_loop)
        .ok_or(EntityId::Loop(h.parent_loop))?;
    let face = body.get_face(l.face).ok_or(EntityId::Face(l.face))?;
    Ok(face.surface)
}

/// An edge's two side surfaces, `he_plus` first.
fn edge_sides<T: Real>(body: &Body<T>, e: EdgeKey) -> Result<(SurfaceKey, SurfaceKey), EntityId> {
    let edge = body.get_edge(e).ok_or(EntityId::Edge(e))?;
    Ok((
        surface_across(body, edge.he_plus)?,
        surface_across(body, edge.he_minus)?,
    ))
}

/// An edge's two end vertices, in `he_plus`-forward order — start
/// first, so the carrier's parameter increases from the first to the
/// second (the `he_plus` forward contract).
fn edge_ends<T: Real>(body: &Body<T>, e: EdgeKey) -> Result<(VertexKey, VertexKey), EntityId> {
    let edge = body.get_edge(e).ok_or(EntityId::Edge(e))?;
    let h = body
        .get_half_edge(edge.he_plus)
        .ok_or(EntityId::HalfEdge(edge.he_plus))?;
    let end = body
        .half_edge_end(edge.he_plus)
        .ok_or(EntityId::HalfEdge(edge.he_plus))?;
    Ok((h.start, end))
}

/// Whether two unordered surface pairs are the same pair.
fn same_pair(a: (SurfaceKey, SurfaceKey), b: (SurfaceKey, SurfaceKey)) -> bool {
    (a.0 == b.0 && a.1 == b.1) || (a.0 == b.1 && a.1 == b.0)
}

/// **The rim an arc belongs to, whole.**
///
/// A rim is named by any ONE of its arcs. `rim_of` returns every edge
/// of `body` whose certified carrier is the SAME circle as `edge`'s and
/// whose two sides lie on the SAME TWO SURFACES — surface keys, so
/// several faces of one surface across chart seams count as one side —
/// in carrier order starting at `edge` and running in the direction
/// `edge`'s carrier parameter increases. The result is what a fillet
/// verb's `&[EdgeKey]` wants: the rim entire, no more (a co-surface
/// seam meridian can never match, because its two sides are one
/// surface and a rim's are two) and no less (a strict subset is never
/// returned — a matched set that does not close refuses).
///
/// Same circle means the stored carriers' `center`, `radius` and
/// `axis` are each bit-equal; `u_ref` is not read, because it carries
/// the per-chart seam and a rim's arcs disagree on it. An axis stored
/// NEGATED is a different circle to this door although it is the same
/// point set — see [`CircleId::same_circle`] for why the narrower rule
/// is the one that keeps the order contract. Same surfaces means equal
/// [`SurfaceKey`]s. That is a total read of stored data — the EXACT
/// class this module's header names, no funnel and no margin — and the
/// corpus is what makes it honest: every producer a consumer holds a
/// body from (revolve, `merge_coplanar_faces`, the boolean, extrude)
/// stores one rim's arcs on bit-identical centres, radii and axes.
///
/// **What "closes" means, exactly**: the matched arcs form one closed
/// chain on SHARED VERTICES, walked from `edge` and returning to it
/// having used every matched arc. It is not a covering test. Arcs that
/// cover part of the circle twice and another part not at all still
/// chain, and this door answers them as a rim — the instance is issue
/// `rim-door-admits-a-double-cover`, and what refuses such a body is
/// tier 3's conventional specs, not this door.
///
/// The order is deterministic (D9) and `rim_of(b)` is a rotation of
/// `rim_of(a)` for any two arcs `a`, `b` of one rim — unconditionally,
/// because every arc that matches shares the seed's stored `axis` and
/// therefore winds the same way round the circle, so which arc a walk
/// starts at is the only freedom left.
///
/// # Errors
///
/// [`RimError::NotAnArc`] when the seed carries no circle,
/// [`RimError::CoSurface`] when its two sides are one surface,
/// [`RimError::NotOneRim`] when the matched arcs do not form one closed
/// chain, [`RimError::NotIntact`] on a dangling key or an unreadable
/// reference.
pub fn rim_of<T: Bounds>(body: &Body<T>, edge: EdgeKey) -> Result<Vec<EdgeKey>, RimError> {
    let seed_edge = body
        .get_edge(edge)
        .ok_or(RimError::NotIntact(EntityId::Edge(edge)))?;
    let seed_carrier = match body
        .get_curve_geom(seed_edge.curve)
        .and_then(CurveGeom::certified)
    {
        Some(c) => c.carrier().clone(),
        None => return Err(RimError::NotAnArc { edge, kind: None }),
    };
    let Some(seed_circle) = circle_id(&seed_carrier) else {
        return Err(RimError::NotAnArc {
            edge,
            kind: Some(CurveKind::of(&seed_carrier)),
        });
    };
    let seed_sides = edge_sides(body, edge).map_err(RimError::NotIntact)?;
    if seed_sides.0 == seed_sides.1 {
        return Err(RimError::CoSurface {
            edge,
            surface: seed_sides.0,
        });
    }

    // The matched set, in arena order (D9).
    let mut matched: Vec<EdgeKey> = Vec::new();
    for (k, e) in body.edges() {
        let Some(circle) = body
            .get_curve_geom(e.curve)
            .and_then(CurveGeom::certified)
            .and_then(|c| circle_id(c.carrier()))
        else {
            continue;
        };
        if !seed_circle.same_circle(&circle) {
            continue;
        }
        // Only a carrier match reaches the adjacency, so an edge with
        // no readable sides is a fault of this rim's neighbourhood and
        // not of every unrelated edge in the arena.
        let sides = edge_sides(body, k).map_err(RimError::NotIntact)?;
        if same_pair(sides, seed_sides) {
            matched.push(k);
        }
    }

    order_rim(body, edge, &seed_carrier, matched)
}

/// The matched set as ONE closed chain starting at `edge`, or the
/// typed refusal that names the vertex the walk stopped at.
///
/// **The test is a CLOSED CHAIN ON SHARED VERTICES, and that is all it
/// is.** Consecutive arcs share a vertex — key equality, which is what
/// makes the test exact — the walk starts at `edge` and it must return
/// to `edge`'s start having consumed every matched arc. It is not a
/// covering test: arcs that between them cover one part of the circle
/// TWICE and another not at all still form a closed chain, and this
/// door answers them as a rim (issue `rim-door-admits-a-double-cover`;
/// only tier 3's conventional specs refuse such a body). The
/// alternative is a parametric test, and the arcs of one rim are
/// minted one per chart with a seam each, so their stored parameter
/// intervals are each stated in their own frame — comparing them
/// across arcs needs a decided comparison this door does not have.
fn order_rim<T: Bounds>(
    body: &Body<T>,
    edge: EdgeKey,
    seed_carrier: &Curve3<T>,
    matched: Vec<EdgeKey>,
) -> Result<Vec<EdgeKey>, RimError> {
    // The parameter the refusal reports, computed only when it refuses:
    // where the walk stopped, in the seed's own frame. A vertex whose
    // point cannot be read is an intactness fault and is refused as
    // one — a NaN in the payload would be this door reporting a
    // parameter it never computed.
    let fail = |at: VertexKey, arcs: &[EdgeKey]| -> RimError {
        let Ok(point) = crate::readback::vertex_point(body, at) else {
            return RimError::NotIntact(EntityId::Vertex(at));
        };
        let Some(gap) = circle_param(seed_carrier, point) else {
            // Unreachable: the seed's carrier is a circle by the time
            // the walk runs. Stated rather than unwrapped.
            return RimError::NotIntact(EntityId::Edge(edge));
        };
        RimError::NotOneRim {
            arcs: arcs.to_vec(),
            gap: gap.lo(),
        }
    };

    let (start, mut frontier) = edge_ends(body, edge).map_err(RimError::NotIntact)?;
    let mut ordered = vec![edge];
    loop {
        if frontier == start {
            // Closed. It is one rim exactly when the walk consumed
            // every matched arc; anything left over is a second
            // component on the same circle and support pair.
            return if ordered.len() == matched.len() {
                Ok(ordered)
            } else {
                Err(fail(frontier, &matched))
            };
        }
        let mut next = None;
        for k in &matched {
            if ordered.contains(k) {
                continue;
            }
            let (a, b) = edge_ends(body, *k).map_err(RimError::NotIntact)?;
            if a == frontier || b == frontier {
                if next.is_some() {
                    // A branch: three arcs of one circle meeting at one
                    // vertex is not a chain, and picking one would be
                    // the guess this door refuses to make.
                    return Err(fail(frontier, &matched));
                }
                next = Some((*k, if a == frontier { b } else { a }));
            }
        }
        let Some((k, beyond)) = next else {
            // A dangling end — the partial revolve's open rim.
            return Err(fail(frontier, &matched));
        };
        ordered.push(k);
        frontier = beyond;
    }
}

// The door-only contracts: totality on dangling keys, materializer
// determinism, empty-set and unordered-pair semantics, the unit
// constructor's two refusals, and the decided door's band partition.
// The delegation's agreement with the document layer is pinned
// upstairs (`editor-core`'s selector suites drive the same arms
// through `select_where`). Boundary rows adapted from a review probe.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;

    use super::*;
    use crate::fixtures::{plane_surface, prism};

    /// A prism fixture with one wall re-surfaced as a PLANE, so the
    /// body carries two surface kinds (the fixture's placeholder
    /// Nurbs everywhere else) and circle-certified carriers.
    fn mixed() -> Body<f64> {
        let mut p = prism(4, Tol::witness()).body;
        let face = all_faces(&p)[0];
        let plane = p.add_surface(plane_surface(
            Point3::origin(),
            geom_core::Vec3::unit_z(),
            geom_core::Vec3::unit_x(),
        ));
        p.get_face_mut(face)
            .expect("a face the materializer just listed")
            .surface = plane;
        p
    }

    #[test]
    fn materializers_are_the_arena_fold_and_deterministic() {
        let body = mixed();
        let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        assert_eq!(all_edges(&body), edges, "slot-index order, nothing else");
        assert_eq!(all_faces(&body), faces);
        assert_eq!(all_edges(&body), all_edges(&body), "same body, same answer");
        assert!(!edges.is_empty() && !faces.is_empty(), "not vacuous");
    }

    #[test]
    fn dangling_keys_are_an_honest_no_never_a_panic() {
        let body = mixed();
        let (e, f) = (EdgeKey::default(), FaceKey::default());
        assert_eq!(edge_carrier_kind(&body, e), None);
        assert_eq!(face_surface_kind(&body, f), None);
        assert!(!edge_carrier_matches(
            &body,
            e,
            CurveKindSet::of(CurveKind::ALL)
        ));
        assert!(!face_surface_matches(
            &body,
            f,
            SurfaceKindSet::of(ALL_SURFACE_KINDS)
        ));
        assert!(!edge_adjacent_matches(
            &body,
            e,
            SurfaceKindSet::of(ALL_SURFACE_KINDS),
            SurfaceKindSet::of(ALL_SURFACE_KINDS)
        ));
    }

    /// The rim door's OTHER `NotAnArc` payload: an edge whose curve
    /// entry is null scaffolding carries no kind at all, and the arm
    /// says so rather than guessing one. Rowed here because minting a
    /// null curve is crate-internal; every other refusal is reachable
    /// from outside and rowed there (`topo/tests/rim_of.rs`,
    /// `sweep/tests/rim_of_rows.rs`).
    #[test]
    fn a_seed_with_no_certified_carrier_is_not_an_arc_and_names_no_kind() {
        let mut body = mixed();
        let e = all_edges(&body)[0];
        let null = body.add_null_curve(crate::null::NullEdge {
            below_end: VertexKey::default(),
            above_end: VertexKey::default(),
        });
        body.get_edge_mut(e).expect("an edge just listed").curve = null;
        assert_eq!(
            rim_of(&body, e),
            Err(RimError::NotAnArc {
                edge: e,
                kind: None
            })
        );
    }

    /// **[`circle_param`] agrees with [`Curve3::param_near`], including
    /// on the carrier whose stored radius is NEGATIVE** — the case the
    /// first cut of this function got wrong by half a turn, because it
    /// cancelled the radius out of both `atan2` arguments and `atan2`
    /// only ignores a POSITIVE common factor.
    ///
    /// Both doors are callable here: `param_near` needs `SpanLocate`
    /// for its NURBS arm, and `f64` has it — it is `rim_of`'s own
    /// signature that may not take that bound, not a test's.
    #[test]
    fn the_gap_parameter_agrees_with_the_curve_doors_own_reading() {
        use core::f64::consts::{FRAC_PI_3, FRAC_PI_4, PI};
        let frame = |radius: f64| Curve3::Circle {
            center: Point3::new(0.25, -1.5, 0.75),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        for radius in [2.0_f64, -2.0] {
            let c = frame(radius);
            for theta in [0.0, FRAC_PI_4, FRAC_PI_3, 2.0, PI - 0.5, -1.25] {
                let p = c.eval(theta);
                let want = c.param_near(p, 0.0).expect("a circle locates");
                let got = circle_param(&c, p).expect("and so does this one");
                assert!(
                    (got - want).abs() < 1e-12,
                    "radius {radius}, theta {theta}: {got} vs the curve door's {want}"
                );
            }
        }
        // The claim is not vacuous: cancelling the radius would flip
        // the negative-radius readings by exactly pi.
        let c = frame(-2.0);
        let p = c.eval(FRAC_PI_3);
        let w = p - Point3::new(0.25, -1.5, 0.75);
        let cancelled = w
            .dot(Vec3::new(0.0, 0.0, 1.0).cross(Vec3::new(1.0, 0.0, 0.0)))
            .atan2(w.dot(Vec3::new(1.0, 0.0, 0.0)));
        let got = circle_param(&c, p).expect("locates");
        assert!(
            (got - cancelled).abs() > 1.0,
            "the radius-cancelling reading really is the wrong one here: \
             {cancelled} against {got}"
        );
    }

    #[test]
    fn an_empty_set_matches_nothing() {
        let body = mixed();
        for e in all_edges(&body) {
            assert!(!edge_carrier_matches(&body, e, CurveKindSet::default()));
            assert!(!edge_adjacent_matches(
                &body,
                e,
                SurfaceKindSet::default(),
                SurfaceKindSet::of(ALL_SURFACE_KINDS)
            ));
        }
        for f in all_faces(&body) {
            assert!(!face_surface_matches(&body, f, SurfaceKindSet::default()));
        }
    }

    #[test]
    fn the_adjacent_pair_is_unordered() {
        let body = mixed();
        let mut mixed_pair_hit = false;
        for e in all_edges(&body) {
            for a in ALL_SURFACE_KINDS {
                for b in ALL_SURFACE_KINDS {
                    let (sa, sb) = (SurfaceKindSet::just(a), SurfaceKindSet::just(b));
                    assert_eq!(
                        edge_adjacent_matches(&body, e, sa, sb),
                        edge_adjacent_matches(&body, e, sb, sa),
                        "swapping the sets cannot change the answer"
                    );
                }
            }
            let (pl, nu) = (
                SurfaceKindSet::just(SurfaceKind::Plane),
                SurfaceKindSet::just(SurfaceKind::Nurbs),
            );
            mixed_pair_hit |= edge_adjacent_matches(&body, e, pl, nu);
        }
        // The re-surfaced wall really produces a mixed pair, so the
        // symmetry loop above is exercised on a TRUE answer with two
        // DIFFERENT sets, not only on false ones.
        assert!(mixed_pair_hit, "the fixture carries a Plane x Nurbs rim");
    }

    /// **The constructor is the enforcement**: a vector with no
    /// decided length refuses typed, and one with a length normalizes
    /// however far from unit it started. The wobble rows are the ones
    /// that matter for the decided door downstream — the datum they
    /// end up in measures a length whatever the input's scale was.
    ///
    /// Every fixture here is off unit by MORE than the assertion's own
    /// tolerance, deliberately: a wobble the f64 grid swallows
    /// (`1.0 + 1e-30` IS `1.0`; `(1e-30, 1e-30, 1.0)` has
    /// `norm_squared` exactly 1) would pass this row without the
    /// normalization ever running.
    #[test]
    fn the_unit_constructor_refuses_the_lengthless_and_normalizes_the_rest() {
        let band = Band::new(1e-6, 1e-3).expect("a well-ordered band");
        assert!(matches!(
            UnitVec3::new(Vec3::new(0.0, 0.0, 0.0), band),
            Err(UnitVec3Error::Degenerate)
        ));
        // Scale is irrelevant to what comes out: a 1e-12 wobble on a
        // unit input and a 1e6 blow-up both leave unit length.
        for v in [
            Vec3::new(0.0, 0.0, 1.0 + 1e-12),
            Vec3::new(1e-6, 1e-6, 1.0),
            Vec3::new(3e6, 4e6, 0.0),
            Vec3::new(3.0, 4.0, 12.0),
        ] {
            assert!(
                (v.norm() - 1.0).abs() > 1e-15,
                "the fixture {v:?} is already unit, so it would not \
                 exercise the normalization"
            );
            let u = UnitVec3::new(v, band)
                .expect("a vector with a length")
                .get();
            assert!(
                (u.norm() - 1.0).abs() <= 1e-15,
                "norm {} for {v:?}",
                u.norm()
            );
        }
        // The overflow class: a length that is not a finite NUMBER
        // refuses BEFORE the sign of the length is asked for. An
        // infinite margin is maximally definite to `sign_within`, so
        // deciding first would answer Positive and normalize the
        // direction into the zero vector — a datum that then answers a
        // definite WRONG sign with no refusal.
        for v in [
            Vec3::new(1e200, 0.0, 0.0),
            Vec3::new(0.0, 1e200, 1e200),
            Vec3::new(f64::INFINITY, 0.0, 0.0),
            Vec3::new(f64::NEG_INFINITY, 0.0, 0.0),
            Vec3::new(f64::NAN, 0.0, 1.0),
        ] {
            assert!(
                matches!(UnitVec3::new(v, band), Err(UnitVec3Error::NonFiniteLength)),
                "{v:?} must refuse, not normalize"
            );
        }
        // What the refusal prevents, executed rather than asserted.
        // The normalization these components go through collapses the
        // direction to the ZERO vector:
        let collapsed = Vec3::new(1e200, 0.0, 0.0).normalize();
        assert_eq!((collapsed.x, collapsed.y, collapsed.z), (0.0, 0.0, 0.0));
        // — and a zero normal is DEFINITELY non-unit, the condition
        // the retired tripwire fired on. So this input was loud before
        // the type existed, and the finiteness arm is what keeps it
        // loud now that the tripwire is gone: without it the door
        // would take a zero normal and measure every point as exactly
        // on the plane.
        let tripwire = Band::new(1e-9, 2e-9).expect("a well-ordered band");
        assert_eq!(
            (collapsed.dot(collapsed) - 1.0).sign_within(tripwire),
            Ok(Sign::Negative)
        );
        // And the datum built from one measures a LENGTH: the same
        // plane spelled at scale 1e6 answers the same distance.
        let p = Point3::new(3.0, 4.0, -2.0);
        let at = |v| DatumValue::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: UnitVec3::new(v, band).expect("a vector with a length"),
        };
        assert_eq!(
            datum_distance(&at(Vec3::new(0.0, 0.0, 1e6)), p),
            datum_distance(&at(Vec3::new(0.0, 0.0, 1.0)), p)
        );
    }

    #[test]
    fn the_decided_door_partitions_on_the_band() {
        let band = Band::new(1e-6, 1e-3).expect("a well-ordered band");
        let up = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), band).expect("a unit z");
        let plane = DatumValue::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: up,
        };
        let axis = DatumValue::Axis {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: up,
        };
        let point = DatumValue::Point {
            position: Point3::new(0.0, 0.0, 0.0),
        };
        let p = Point3::new(3.0, 4.0, -2.0);
        // SIGNED along a plane's normal, UNSIGNED to an axis or point.
        assert_eq!(datum_distance(&plane, p), -2.0);
        assert_eq!(datum_distance(&axis, p), 5.0);
        assert!((datum_distance(&point, p) - 29.0_f64.sqrt()).abs() < 1e-12);
        // The stated value is `d - dv`, so the margin the funnel sees
        // is `d - (d - dv)` — dv up to a rounding ulp, which is why
        // each row sits comfortably inside its region rather than on
        // the band's exact boundary (the boundary-inclusive semantics
        // are `sign_within`'s own pinned contract in geom-core).
        for datum in [&plane, &axis, &point] {
            let d = datum_distance(datum, p);
            // |margin| <= zero: definite Zero.
            for dv in [0.0, 1e-7, -1e-7] {
                assert_eq!(
                    datum_distance_sign(datum, p, d - dv, band),
                    Ok(Sign::Zero),
                    "dv={dv}"
                );
            }
            // Strictly inside the gray zone: refuses, either side.
            for dv in [5e-4, -5e-4, 2e-6, -2e-6] {
                assert!(
                    datum_distance_sign(datum, p, d - dv, band).is_err(),
                    "dv={dv}"
                );
            }
            // |margin| >= escalate: the definite sign of (distance - value).
            for dv in [2e-3, 1.0] {
                assert_eq!(
                    datum_distance_sign(datum, p, d - dv, band),
                    Ok(Sign::Positive),
                    "dv={dv}"
                );
                assert_eq!(
                    datum_distance_sign(datum, p, d + dv, band),
                    Ok(Sign::Negative),
                    "dv={dv}"
                );
            }
        }
    }
}

// The interval-safety half of the constructor's contract, at the
// scalar that has enclosures to be safe about: refusal is reserved for
// a length the enclosure DECIDES is zero, and an enclosure that merely
// cannot tell escalates rather than refusing.
#[cfg(test)]
#[cfg(feature = "interval")]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod interval_tests {
    use geom_core::{Decide, Interval, Real, Sign};

    use super::{Band, UnitVec3, UnitVec3Error, Vec3};

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// An enclosure that CONTAINS unit length is a direction — the
    /// widths interval arithmetic carries are not a reason to refuse —
    /// and what comes back DECIDES unit at the band.
    ///
    /// The assertion direction is deliberate. "Not definitely off
    /// unit" would be satisfied by every degradation, including a
    /// normalization that never ran on a wide enough input: a claim
    /// that gets easier as the enclosure gets worse is not evidence.
    /// `Ok(Sign::Zero)` on `‖u‖ − 1` is the opposite — it holds only
    /// while the whole enclosure sits inside the band, so it fails
    /// loudly if the normalization is skipped, if the result is
    /// collapsed, or if the enclosure blows up. Both fixtures are
    /// therefore tight; the price is that this row says nothing about
    /// SLOPPY enclosures, which the escalation row below covers.
    #[test]
    fn an_enclosure_containing_unit_length_passes() {
        // Straddles unit length; a scale away from it.
        let wobbled = Vec3::new(
            Interval::from_bounds(-1e-13, 1e-13),
            Interval::from_bounds(-1e-13, 1e-13),
            Interval::from_bounds(1.0 - 1e-13, 1.0 + 1e-13),
        );
        let scaled = Vec3::new(
            Interval::from_f64(3e4),
            Interval::from_f64(4e4),
            Interval::from_bounds(-1e-9, 1e-9),
        );
        for v in [wobbled, scaled] {
            let u = UnitVec3::new(v, band())
                .expect("an enclosure with a length")
                .get();
            let off = u.norm() - Interval::from_f64(1.0);
            assert_eq!(
                off.sign_within(band()),
                Ok(Sign::Zero),
                "the normalized enclosure must DECIDE unit length: {off:?}"
            );
        }
    }

    /// The overflow class at the enclosure scalar, stated honestly: an
    /// interval whose norm overflows still ENCLOSES the true length,
    /// so it is not unsound and the constructor does not refuse it —
    /// what it loses is precision, and the loss surfaces downstream as
    /// an escalation rather than as a definite wrong sign. Poison is
    /// the arm that does bite here: an empty/NaI component has no
    /// length at all.
    #[test]
    fn an_overflowed_enclosure_stays_sound_and_poison_refuses() {
        let huge = Vec3::new(
            Interval::from_f64(1e200),
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
        );
        let u = UnitVec3::new(huge, band())
            .expect("an overflowing enclosure still encloses its direction")
            .get();
        // Containment, the interval contract: unit length is inside
        // what comes back, so nothing downstream can certify a wrong
        // side from it.
        let off = u.norm() - Interval::from_f64(1.0);
        assert!(
            !matches!(off.sign_within(band()), Ok(Sign::Positive | Sign::Negative)),
            "an overflowed enclosure must not DECIDE off-unit: {off:?}"
        );
        let poisoned = Vec3::new(
            Interval::from_f64(f64::NAN),
            Interval::from_f64(0.0),
            Interval::from_f64(1.0),
        );
        assert!(
            matches!(
                UnitVec3::new(poisoned, band()),
                Err(UnitVec3Error::NonFiniteLength)
            ),
            "a poisoned component names no direction"
        );
    }

    /// A DECIDED zero length refuses; an enclosure that straddles the
    /// band escalates instead of picking an arm.
    #[test]
    fn a_decided_zero_refuses_and_a_straddling_enclosure_escalates() {
        let zero = Vec3::new(
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
        );
        assert!(
            matches!(UnitVec3::new(zero, band()), Err(UnitVec3Error::Degenerate)),
            "a length the enclosure decides is zero"
        );
        let straddling = Vec3::new(
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
            Interval::from_bounds(0.0, 1e-7),
        );
        assert!(
            matches!(
                UnitVec3::new(straddling, band()),
                Err(UnitVec3Error::Escalated(_))
            ),
            "an enclosure that cannot tell escalates"
        );
    }
}
