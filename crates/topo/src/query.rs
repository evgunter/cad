//! **The kernel query seat** (VERB-SEAT-DESIGN §1, `crates/verbs/README.md`) — the
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
//!   seed carrier's tag, then side surface KEYS and shared vertex
//!   KEYS, never a carrier's values — with no funnel, no margin and
//!   nothing decided. What it does differently is its answer
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
//!   layer earlier: [`UnitVec3::new`] under this module's
//!   [`DATUM_UNIT_NORM`] (no `sel_` prefix) decides that a direction
//!   has a finite, nonzero length before normalizing it. It is
//!   deliberately outside the convention — it decides nothing about a
//!   candidate and answers no selection question; it is a constructor
//!   refusing a value the type cannot hold, and a `sel_` name on it
//!   would tell a census reader it belongs to a selector margin
//!   population it is not part of. What it buys the door above is that
//!   [`datum_distance`] is arithmetic all the way down.
//!
//!   **The body is `geom-core`'s, and the NAME is this module's.** The
//!   witness type and its decision
//!   ([`geom_core::decide_unit_direction`], the workspace's one
//!   `Margin::norm3` decide-then-normalize body) live in `geom-core`;
//!   what this seat owns is the funnel name a datum direction is
//!   decided under, because the datum is a value this layer owns. The
//!   passer is the evaluation layer (`editor-core`'s `datum_unit`,
//!   which builds every [`DatumValue`]); its own direction door passes
//!   its own name to the same body: two ratified funnel names, one
//!   body.
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
//! spelling, so the normalization and its typed refusal are the
//! kernel type's, decided under this seat's name, and the document
//! layer maps that refusal onto its own node error.
//! Stable names themselves never appear below the G1 line, which is
//! the point.

use geom::Curve3;
use geom_brep::SurfaceKey;
use geom_core::k_stats::decide;
use geom_core::{
    Band, Decide, Indeterminate, Margin, OrthoFrame, Point2, Point3, Real, Sign, UnitVec3, Vec2,
};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::readback::{CarrierAbsence, DanglingRef};

/// The kinds, re-exported where their sets and predicates live.
///
/// [`CurveKind`] and [`SurfaceKind`] are `geom`'s, beside the enums
/// they mirror ([`Curve3::kind`], [`geom::Surface::kind`]), and every
/// crate above reuses them. What this seat owns is the SETS over them
/// — [`CurveKindSet`], [`SurfaceKindSet`] and their bit numbering — and
/// the EXACT predicates that read them.
pub use geom::{CurveKind, SurfaceKind};

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
        Self(kinds.into_iter().fold(0, |acc, k| acc | curve_bit(k)))
    }

    /// The singleton set — the common case.
    #[must_use]
    pub fn just(kind: CurveKind) -> Self {
        Self::of([kind])
    }

    /// Whether `kind` is a member.
    #[must_use]
    pub fn contains(self, kind: CurveKind) -> bool {
        self.0 & curve_bit(kind) != 0
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

/// A SET of [`SurfaceKind`]s — [`CurveKindSet`]'s face-side twin, and
/// the comparand of both [`face_surface_matches`] and each side of
/// [`edge_adjacent_matches`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SurfaceKindSet(u8);

impl SurfaceKindSet {
    /// The set of exactly these kinds. An EMPTY set matches nothing.
    #[must_use]
    pub fn of(kinds: impl IntoIterator<Item = SurfaceKind>) -> Self {
        Self(kinds.into_iter().fold(0, |acc, k| acc | surface_bit(k)))
    }

    /// The singleton set — the common case.
    #[must_use]
    pub fn just(kind: SurfaceKind) -> Self {
        Self::of([kind])
    }

    /// Whether `kind` is a member.
    #[must_use]
    pub fn contains(self, kind: SurfaceKind) -> bool {
        self.0 & surface_bit(kind) != 0
    }

    /// Whether the set is empty (matches nothing).
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The members, in [`SurfaceKind::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = SurfaceKind> {
        SurfaceKind::ALL
            .into_iter()
            .filter(move |k| self.contains(*k))
    }
}

/// A kind's bit in a [`CurveKindSet`]: its place in [`CurveKind::ALL`],
/// which is declaration order.
const fn curve_bit(kind: CurveKind) -> u8 {
    const { assert!(CurveKind::ALL.len() <= u8::BITS as usize) };
    1 << kind as u8
}

/// A kind's bit in a [`SurfaceKindSet`]: its place in
/// [`SurfaceKind::ALL`], which is declaration order.
const fn surface_bit(kind: SurfaceKind) -> u8 {
    const { assert!(SurfaceKind::ALL.len() <= u8::BITS as usize) };
    1 << kind as u8
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
///
/// The flattening of the typed readback door
/// [`crate::readback::edge_carrier_kind`], which is the one reading of
/// an edge's carrier tag: the three lookups run once, there, and what
/// is dropped here is only WHICH of them came back empty.
#[must_use]
pub fn edge_carrier_kind<T: Real>(body: &Body<T>, e: EdgeKey) -> Option<CurveKind> {
    crate::readback::edge_carrier_kind(body, e).ok()
}

/// The surface kind of a face, or `None` for a dangling key or an
/// unreadable surface reference: the flattening of the typed readback
/// door [`crate::readback::face_carrier_kind`], which is the one
/// reading of a face's carrier tag.
#[must_use]
pub fn face_surface_kind<T: Real>(body: &Body<T>, f: FaceKey) -> Option<SurfaceKind> {
    crate::readback::face_carrier_kind(body, f).ok()
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
    let Ok(sides) = crate::readback::edge_sides(body, e) else {
        return false;
    };
    match (
        face_surface_kind(body, sides.plus.face),
        face_surface_kind(body, sides.minus.face),
    ) {
        (Some(p), Some(m)) => (a.contains(p) && b.contains(m)) || (a.contains(m) && b.contains(p)),
        (None, _) | (_, None) => false,
    }
}

// ---------------------------------------------------------------
// The DECIDED door and the funnel name of the type it measures
// against: the datum's own unit-direction constructor is
// `geom_core::UnitVec3::new` under this module's name, and the
// distance-sign door has an honest Margin and a typed
// indeterminate.
// ---------------------------------------------------------------

/// **The funnel site name** of a datum direction's length decision —
/// the name this crate passes to [`UnitVec3::new`] when a datum's
/// normal or axis direction is decided, because a datum is a value
/// this layer owns. Its comparand is a genuine length (the vector's
/// norm), so it goes through the plain [`Margin::norm3`] door and
/// owes NO `docs/predicate-dimension-audit.md` row.
///
/// A K row name reaching the funnel through a const, not a literal at
/// the decide site, so it is a roster carrier (`docs/K-REPORT.md`,
/// "The inventory method, restated").
pub const DATUM_UNIT_NORM: &str = "datum_unit_norm";

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
    /// The payload is the frame WITNESS: `u` (sketch +x) and `v`
    /// (sketch +y) are unit and orthogonal as a property of the type,
    /// decided where the frame was minted, and `w = u × v` is the
    /// normal — carried by the witness rather than recomputed at each
    /// reader, which is where two spellings would drift apart.
    Frame(OrthoFrame<T>),
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
        DatumValue::Frame(f) => (p - f.origin()).dot(f.w().get()),
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
// surface keys and vertex keys, read off the topology; no carrier is
// compared, no funnel, no margin, no sampled geometry.
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
    /// **The edges between the seed's two surfaces do not close into
    /// one chain through the seed**: at vertex `at` the chain dangles
    /// (a partial revolve's open rim is the honest instance) or
    /// branches. A partial set is never returned.
    NotOneRim {
        /// The chain as walked from the seed up to `at`, in walk order.
        walked: Vec<EdgeKey>,
        /// The vertex the walk stopped at.
        at: VertexKey,
        /// Whether the chain dangles or branches there.
        how: RimBreak,
    },
    /// A dangling reference on the way: a key the body does not hold,
    /// or a geometry key a live entity names and the arena does not.
    NotIntact(DanglingRef),
}

/// How a rim's chain fails to close at the vertex
/// [`RimError::NotOneRim`] names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RimBreak {
    /// One end of an edge between the seed's two surfaces is at the
    /// vertex — the end the walk arrived on: the chain ends there.
    Dangles,
    /// Three or more ends of edges between the seed's two surfaces are
    /// at the vertex (a closed edge's two ends both count), so it is no
    /// single chain.
    Branches,
}

impl core::fmt::Display for RimError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAnArc { edge, kind } => {
                // The kind is NAMED, not `Debug`-rendered: a payload
                // reaching a message through `Debug` is what the prose
                // census hunts, and words read better in a refusal.
                let carries = match kind {
                    None => "no certified carrier".to_owned(),
                    Some(kind) => {
                        crate::validate::with_article(&format!("{} curve", kind.adjective()))
                    }
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
            Self::NotOneRim {
                how: RimBreak::Dangles,
                ..
            } => f.write_str(
                "this edge names no closed rim: the edges between its two \
                 surfaces stop at a vertex where none continues them. Select \
                 the edges one by one instead",
            ),
            Self::NotOneRim {
                how: RimBreak::Branches,
                ..
            } => f.write_str(
                "this edge names no single rim: the edges between its two \
                 surfaces end at one vertex more than twice. Select the edges \
                 one by one instead",
            ),
            Self::NotIntact(DanglingRef::Entity(at)) => {
                write!(f, "the body is not intact at {at}")
            }
            Self::NotIntact(DanglingRef::Geometry(at)) => write!(
                f,
                "the body is not intact: a live entity names {at}, which does \
                 not resolve"
            ),
        }
    }
}

impl std::error::Error for RimError {}

/// The refusal for a topological key that did not resolve.
fn torn(id: EntityId) -> RimError {
    RimError::NotIntact(DanglingRef::Entity(id))
}

/// An edge's two side surfaces, `he_plus` first.
fn side_surfaces<T: Real>(
    body: &Body<T>,
    e: EdgeKey,
) -> Result<(SurfaceKey, SurfaceKey), RimError> {
    let sides = crate::readback::edge_sides(body, e).map_err(RimError::NotIntact)?;
    Ok((sides.plus.surface, sides.minus.surface))
}

/// A half-edge's start and end vertices.
fn half_edge_ends<T: Real>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> Result<(VertexKey, VertexKey), EntityId> {
    let h = body.get_half_edge(he).ok_or(EntityId::HalfEdge(he))?;
    let end = body.half_edge_end(he).ok_or(EntityId::HalfEdge(he))?;
    Ok((h.start, end))
}

/// An edge's two end vertices, in `he_plus`-forward order.
fn edge_ends<T: Real>(body: &Body<T>, e: EdgeKey) -> Result<(VertexKey, VertexKey), EntityId> {
    let edge = body.get_edge(e).ok_or(EntityId::Edge(e))?;
    half_edge_ends(body, edge.he_plus)
}

/// The seed's two surfaces as an unordered pair, stored lower key
/// first — the arena order the walk's direction is fixed by.
type SurfacePair = (SurfaceKey, SurfaceKey);

/// Whether `sides` is `pair`, in either order.
fn on_pair(sides: (SurfaceKey, SurfaceKey), pair: SurfacePair) -> bool {
    (sides.0.min(sides.1), sides.0.max(sides.1)) == pair
}

/// **The seed gate: a rim is named by an arc of a circle.** The one
/// carrier read the door makes, and only of the seed — the walk reads
/// no edge's carrier and is kind-agnostic.
///
/// This precondition reflects today's consumers, not the door's
/// shape: the fillet's closed-rim band reads a circle frame. It is
/// expected to be lifted, so the door names any closed chain between
/// two surfaces (an ellipse rim from a tilted plane through a
/// cylinder), when a consumer needs non-circle rims.
fn seed_is_an_arc<T: Real>(body: &Body<T>, edge: EdgeKey) -> Result<(), RimError> {
    match crate::readback::edge_carrier_ref(body, edge) {
        Ok(Curve3::Circle { .. }) => Ok(()),
        Ok(other) => Err(RimError::NotAnArc {
            edge,
            kind: Some(other.kind()),
        }),
        Err(CarrierAbsence::Dangling(at)) => Err(RimError::NotIntact(at)),
        Err(CarrierAbsence::NoCarrier) => Err(RimError::NotAnArc { edge, kind: None }),
    }
}

/// **The rim an arc belongs to, whole.**
///
/// A rim is named by any ONE of its arcs. `rim_of` returns the closed
/// chain through `edge`, on shared vertices, of the edges whose two
/// sides lie on `edge`'s two surfaces — surface KEYS, so several faces
/// of one surface across chart seams count as one side. The result is
/// what a fillet verb's `&[EdgeKey]` wants: the rim entire, no more (a
/// co-surface seam meridian is never on the pair, because its two
/// sides are one surface and a rim's are two) and no less (a chain
/// that does not close refuses).
///
/// **Membership is read off the topology; no carrier is compared.** A
/// shared surface key is the producer's recorded decision that those
/// faces lie on one surface. Two circles of one surface pair's
/// intersection can cross (a bitangent plane cuts a torus in two), but
/// at a crossing with all four arcs present the walk meets more than
/// two edge ends and refuses [`RimBreak::Branches`]; a crossing where
/// a third surface has removed two of the four arcs is reached by no
/// public door known to this door. Edges on
/// the same pair in another chain are another rim (a plane through a
/// torus has two) and are not part of this answer.
///
/// **What "closes" means, exactly**: every vertex the walk reaches,
/// the seed's start included, meets exactly two ends of edges on the
/// pair, and the walk returns to the seed. It is not a covering test.
/// Arcs that cover part of the circle twice and another part not at
/// all still chain, and this door answers them as a rim — the instance
/// is issue `rim-door-admits-a-double-cover`, and what refuses such a
/// body is tier 3's conventional specs, not this door.
///
/// **The seed must be a circle arc today.** The walk is kind-agnostic;
/// the precondition ([`RimError::NotAnArc`]) reflects today's
/// consumers, whose closed-rim band reads a circle frame, and is
/// expected to be lifted to any closed chain between two surfaces when
/// a consumer needs non-circle rims.
///
/// **Order.** The answer starts at `edge` and runs the way the
/// half-edges on the pair's lower surface key (arena order) run: each
/// face's loop has its face on one side, so every rim edge's half-edge
/// on that surface winds the rim the same way. So the order is
/// deterministic (D9) and `rim_of(b)` is a rotation of `rim_of(a)` for
/// any two edges `a`, `b` of one rim, whatever winding each arc's
/// carrier was stored with.
///
/// # Errors
///
/// [`RimError::NotAnArc`] when the seed carries no circle,
/// [`RimError::CoSurface`] when its two sides are one surface,
/// [`RimError::NotOneRim`] when the chain through it dangles or
/// branches, [`RimError::NotIntact`] on a dangling reference.
pub fn rim_of<T: Real>(body: &Body<T>, edge: EdgeKey) -> Result<Vec<EdgeKey>, RimError> {
    seed_is_an_arc(body, edge)?;
    let sides = side_surfaces(body, edge)?;
    if sides.0 == sides.1 {
        return Err(RimError::CoSurface {
            edge,
            surface: sides.0,
        });
    }
    let pair = (sides.0.min(sides.1), sides.0.max(sides.1));

    let seed = body
        .get_edge(edge)
        .ok_or_else(|| torn(EntityId::Edge(edge)))?;
    let lower_side = if sides.0 == pair.0 {
        seed.he_plus
    } else {
        seed.he_minus
    };
    let (start, mut frontier) = half_edge_ends(body, lower_side).map_err(torn)?;
    let mut walked = vec![edge];
    loop {
        let arrived = *walked.last().unwrap_or(&edge);
        let next = continuation(body, pair, frontier, arrived, &walked)?;
        if frontier == start {
            return Ok(walked);
        }
        walked.push(next);
        let (a, b) = edge_ends(body, next).map_err(torn)?;
        frontier = if a == frontier { b } else { a };
    }
}

/// The edge on `pair` that continues the chain at `at` from `arrived`,
/// or the refusal naming `at`: exactly two ends of edges on the pair
/// meet a chain vertex — a closed edge's two at its one vertex — so
/// one end is a dangle and three or more a branch.
fn continuation<T: Real>(
    body: &Body<T>,
    pair: SurfacePair,
    at: VertexKey,
    arrived: EdgeKey,
    walked: &[EdgeKey],
) -> Result<EdgeKey, RimError> {
    let refuse = |how| RimError::NotOneRim {
        walked: walked.to_vec(),
        at,
        how,
    };
    let mut ends = 0usize;
    let mut next = None;
    for k in body
        .edges_of_vertex(at)
        .ok_or_else(|| torn(EntityId::Vertex(at)))?
    {
        if !on_pair(side_surfaces(body, k)?, pair) {
            continue;
        }
        let (a, b) = edge_ends(body, k).map_err(torn)?;
        ends += usize::from(a == at) + usize::from(b == at);
        if k != arrived || a == b {
            next = Some(k);
        }
    }
    match (ends, next) {
        (2, Some(k)) if k == walked[0] || !walked.contains(&k) => Ok(k),
        (0..=1, _) => Err(refuse(RimBreak::Dangles)),
        _ => Err(refuse(RimBreak::Branches)),
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
    use geom_core::{Tol, UnitVec3Error, Vec3};

    use super::*;
    use crate::fixtures::{plane_surface, raw_prism};

    /// A prism fixture with one wall re-surfaced as a PLANE, so the
    /// body carries two surface kinds (the fixture's placeholder
    /// Nurbs everywhere else) and circle-certified carriers.
    fn mixed() -> Body<f64> {
        let mut p = raw_prism(4, Tol::witness()).body;
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

    /// The datum door is the same body, so it answers the same way —
    /// executed rather than argued, because "shares the predicate"
    /// has been wrong before.
    ///
    /// The sentence is pinned too: this is the text a user reads, and
    /// the defect being fixed was that it named zero length for a
    /// direction that is not zero.
    #[test]
    fn the_datum_constructor_refuses_an_underflowed_direction_by_its_own_name() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let refused = UnitVec3::new(Vec3::new(1e-180, 0.0, 0.0), DATUM_UNIT_NORM, band)
            .expect_err("a direction with no measurable length is refused");
        assert_eq!(refused, UnitVec3Error::UnderflowedLength);
        let said = refused.to_string();
        assert!(
            said.starts_with("a direction vector's length underflowed to zero"),
            "the refusal says what happened to the LENGTH: {said}"
        );
        assert!(
            said.contains(geom_core::RANGE_RECOURSE),
            "and it names the recourse that works: {said}"
        );
        // The recourse it must NOT name, because it does not work:
        // the direction is fine, and no smaller ε recovers it.
        assert!(
            !said.contains("names no direction"),
            "an underflowed direction is not the no-direction refusal: {said}"
        );
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
            SurfaceKindSet::of(SurfaceKind::ALL)
        ));
        assert!(!edge_adjacent_matches(
            &body,
            e,
            SurfaceKindSet::of(SurfaceKind::ALL),
            SurfaceKindSet::of(SurfaceKind::ALL)
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

    /// [`RimError::NotAnArc`]'s text, both payloads: the kind reads in
    /// the adjective register with its article (`an elliptical` is the
    /// vowel case), and a missing carrier says so.
    #[test]
    fn not_an_arc_names_the_curve_in_words() {
        let edge = all_edges(&mixed())[0];
        let text = |kind| RimError::NotAnArc { edge, kind }.to_string();
        assert_eq!(
            text(Some(CurveKind::Line)),
            format!(
                "edge {edge:?} carries a straight curve, and a rim is named by an arc of a circle"
            )
        );
        assert_eq!(
            text(Some(CurveKind::Ellipse)),
            format!(
                "edge {edge:?} carries an elliptical curve, and a rim is named by an arc of a \
                 circle"
            )
        );
        assert_eq!(
            text(None),
            format!(
                "edge {edge:?} carries no certified carrier, and a rim is named by an arc of a \
                 circle"
            )
        );
    }

    /// **`NotOneRim` reads within the viewer's budget, and names only
    /// what the walk saw** — under 50 words per arm, each arm saying
    /// which of the two causes it is and the recourse.
    #[test]
    fn the_not_one_rim_text_is_short_and_says_which_break() {
        for (how, says) in [
            (RimBreak::Dangles, "stop at a vertex"),
            (RimBreak::Branches, "more than twice"),
        ] {
            let text = RimError::NotOneRim {
                walked: vec![EdgeKey::default()],
                at: VertexKey::default(),
                how,
            }
            .to_string();
            let words = text.split_whitespace().count();
            assert!(words < 50, "{how:?}: {words} words: {text}");
            assert!(text.contains(says), "{how:?} names its own cause: {text}");
            assert!(
                text.contains("one by one"),
                "{how:?} names the recourse: {text}"
            );
        }
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
                SurfaceKindSet::of(SurfaceKind::ALL)
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
            for a in SurfaceKind::ALL {
                for b in SurfaceKind::ALL {
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

    /// **A datum built from any scale measures a LENGTH.** The
    /// constructor normalizes however far from unit the input started
    /// (its own rows, in `geom-core`), and the decided door downstream
    /// is what that buys: the same plane spelled at scale 1e6 answers
    /// the same distance.
    #[test]
    fn a_datum_built_at_any_scale_measures_a_length() {
        let band = Band::new(1e-6, 1e-3).expect("a well-ordered band");
        let p = Point3::new(3.0, 4.0, -2.0);
        let at = |v| DatumValue::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: UnitVec3::new(v, DATUM_UNIT_NORM, band).expect("a vector with a length"),
        };
        assert_eq!(
            datum_distance(&at(Vec3::new(0.0, 0.0, 1e6)), p),
            datum_distance(&at(Vec3::new(0.0, 0.0, 1.0)), p)
        );
    }

    #[test]
    fn the_decided_door_partitions_on_the_band() {
        let band = Band::new(1e-6, 1e-3).expect("a well-ordered band");
        let up = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), DATUM_UNIT_NORM, band).expect("a unit z");
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
