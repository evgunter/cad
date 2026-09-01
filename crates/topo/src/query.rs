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
//! - **DECIDED** — [`datum_distance_sign`] is a real numeric
//!   comparison and therefore a `k_stats::decide` site with a named
//!   `sel_*` predicate ([`SEL_DATUM_DISTANCE`]), an honest
//!   [`Margin`] door, and a typed indeterminate on an in-band
//!   comparand. It participates in the K census exactly like any
//!   kernel site (SELECT-DESIGN GS-Q1: the naming convention does the
//!   separating, not a second funnel).
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
//! Datum-node resolution — a recipe reference becoming a
//! [`DatumValue`] — stays in the document layer; this seat takes the
//! resolved value. Stable names themselves never appear below the G1
//! line, which is the point.

use geom::{Curve3, Surface};
use geom_brep::SurfaceKind;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey};
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

    /// The lowercase name, for refusal rendering.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Circle => "circle",
            Self::Ellipse => "ellipse",
            Self::Nurbs => "nurbs",
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

    /// The kind of a carrier — the tag read the EXACT predicates are.
    #[must_use]
    pub fn kind_of<T: Real>(s: &Surface<T>) -> SurfaceKind {
        SurfaceKind::of(s)
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
// The DECIDED door: one funnel site, an honest Margin, a typed
// indeterminate.
// ---------------------------------------------------------------

/// A resolved datum: geometry VALUES, not kernel entities and not
/// recipe references. Normals/directions are UNIT — the document
/// layer normalizes at evaluation (a degenerate vector is a typed
/// refusal there), and a kernel-direct caller owes the same
/// invariant.
#[derive(Debug, Clone)]
pub enum DatumValue<T: Decide> {
    /// A plane through `origin` with UNIT `normal`.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The unit normal.
        normal: Vec3<T>,
    },
    /// An axis through `origin` along UNIT `dir`.
    Axis {
        /// A point on the axis.
        origin: Point3<T>,
        /// The unit direction.
        dir: Vec3<T>,
    },
    /// A point.
    Point {
        /// Its position.
        position: Point3<T>,
    },
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

/// The distance of `p` from a datum: SIGNED along a plane's normal
/// (which is stored unit, so the dot product is already a length),
/// UNSIGNED to an axis or a point.
#[must_use]
pub fn datum_distance<T: Decide>(datum: &DatumValue<T>, p: Point3<T>) -> T {
    match datum {
        DatumValue::Plane { origin, normal } => (p - *origin).dot(*normal),
        DatumValue::Axis { origin, dir } => {
            let v = p - *origin;
            (v - *dir * v.dot(*dir)).norm()
        }
        DatumValue::Point { position } => (p - *position).norm(),
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
