//! Topology entities and their typed arena keys — the half-edge B-rep
//! (M1, replacing M0's placeholder adjacency).
//!
//! The seven entity kinds of the manifold B-rep (D1 in `docs/DESIGN.md`):
//! [`Solid`], [`Shell`], [`Face`], [`Loop`], [`HalfEdge`], [`Edge`],
//! [`Vertex`]. Each kind has its own key type minted by slotmap's
//! `new_key_type!`, so a key into one arena is a *different type* from a
//! key into another — handing a `VertexKey` to something expecting an
//! `EdgeKey` fails to typecheck. Cross-arena confusion is a compile
//! error, not a runtime surprise:
//!
//! ```compile_fail,E0308
//! use topo::{HalfEdge, HalfEdgeKey, LoopKey, VertexKey};
//!
//! // A `VertexKey` is not an `EdgeKey`: a half-edge's `edge` field will
//! // not accept a key into the vertex arena. This example must NOT
//! // compile.
//! let _ = HalfEdge {
//!     edge: VertexKey::default(),
//!     start: VertexKey::default(),
//!     parent_loop: LoopKey::default(),
//!     next: HalfEdgeKey::default(),
//!     prev: HalfEdgeKey::default(),
//! };
//! ```
//!
//! And the other way around — a `HalfEdgeKey` is not a key into any other
//! arena either:
//!
//! ```compile_fail,E0308
//! use topo::{Edge, HalfEdgeKey};
//!
//! // A `HalfEdgeKey` is not a `CurveKey`: an edge's `curve` field will
//! // not accept a key into the half-edge arena. This example must NOT
//! // compile.
//! let _ = Edge {
//!     he_plus: HalfEdgeKey::default(),
//!     he_minus: HalfEdgeKey::default(),
//!     curve: HalfEdgeKey::default(),
//! };
//! ```
//!
//! # Scalar-free by construction (Q1)
//!
//! Topology entities contain **no scalar values and no comparisons** —
//! only typed keys referencing other arenas. All the scalar-valued data
//! lives in the geometry arenas (see [`crate::geometry`]), which are the
//! only `T`-carrying storage in a [`Body`](crate::Body). Every
//! topology-determining decision was made at construction time through
//! named predicates (Q1); an entity is the *record* of those decisions,
//! so it needs no numbers of its own.
//!
//! # The half-edge structure in one paragraph
//!
//! An [`Edge`] is an undirected piece of curve; each of its two sides is
//! a directed [`HalfEdge`]. A half-edge belongs to exactly one [`Loop`]
//! (one connected boundary curve of a [`Face`]), where `next`/`prev`
//! links chain the loop's half-edges into a circular cycle. The edge's
//! two half-edges traverse it in **opposite directions** (antiparallel),
//! one per adjacent face — which is exactly what makes neighboring faces'
//! boundary orientations mutually consistent. A half-edge stores only its
//! *start* vertex; its end vertex is **derived**, never stored:
//! `end(he) = start(next(he))`. The mate of a half-edge (the other half
//! of its edge) is likewise **computed** ([`crate::Body::mate`]), never
//! stored. This is Mäntylä's GWB structure (ch. 10) adapted to arenas —
//! with two deliberate deviations documented below (typed empty loops,
//! outer loop excluded from `rings`) and one global mirror (orientation).
//!
//! # Orientation conventions (ratified at M1 — the single source of truth)
//!
//! This section discharges the DESIGN.md deferred item
//! *"orientation/sense conventions (M1 — classic bug-farm territory,
//! document as conventions once)"*. This is that once. There is **one**
//! rule; everything else is a corollary, never an independent choice:
//!
//! > **The interior-left rule.** Walking a loop in `next` order, with
//! > the face's outward normal pointing toward the viewer, the face
//! > interior lies to the **left** of every half-edge.
//!
//! Corollaries:
//!
//! - **Outer loops run counterclockwise** viewed from outside (i.e. from
//!   the tip of the outward normal); **rings (interior loops) run
//!   clockwise** viewed the same way. Both follow from interior-left —
//!   they are not separate conventions.
//! - **The two half-edges of an edge are antiparallel**: the two faces
//!   adjacent to an edge lie on opposite sides of it, so interior-left
//!   forces their loops to traverse the edge in opposite directions —
//!   `end(he) = start(mate(he))` and vice versa.
//! - **Vertex orbits.** The half-edges *starting* at a vertex `v` are
//!   visited **clockwise** (viewed from outside, outward normal toward
//!   the viewer) by the step `next(mate(he))`, and **counterclockwise**
//!   by its inverse `mate(prev(he))`. Derivation from the rule (do not
//!   trust transcription — rederive on paper if in doubt): stand at the
//!   normal's tip looking down at `v`. For a half-edge `he` leaving `v`,
//!   interior-left places `he`'s face in the angular sector
//!   *counterclockwise* of `he`. Its mate ends at `v` inside the
//!   neighboring face across `he`'s edge — the sector *clockwise* of
//!   `he` — and `next(mate(he))` is that face's half-edge leaving `v`,
//!   which runs along the clockwise-most edge of that sector. So each
//!   `next ∘ mate` step advances one face clockwise around `v`. The two
//!   steps are mutual inverses (`mate(prev(next(mate(he)))) = he` since
//!   `prev ∘ next = id` and `mate ∘ mate = id`); both are exercised and
//!   direction-tested in the validator's fixtures.
//! - **`Edge::he_plus` defines the edge's intrinsic direction**: the
//!   edge "points" the way its plus half traverses it. **Forward
//!   contract for M2:** when real curve geometry attaches, the curve's
//!   parameter direction MUST agree with `he_plus` — increasing curve
//!   parameter runs from `start(he_plus)` to `end(he_plus)`. Pcurves and
//!   any per-face traversal senses are then derived, never stored as
//!   peer conventions.
//!
//! ## Warning: GWB/Mäntylä diagrams are MIRRORED
//!
//! Mäntylä's GWB — our primary source for ch. 9–11 — orients face
//! boundaries **clockwise** viewed from outside (stated at §11.5). We
//! ratified counterclockwise. Every figure, argument order, and traversal
//! idiom transcribed from the book must be consciously mirrored; in
//! particular GWB's vertex-orbit idiom `adj = mate(adj)->nxt` is stated
//! for *his* convention. **Never transcribe a figure's orientation
//! directly** — rederive from the interior-left rule and test by
//! construction.
//!
//! # Deviations from GWB, deliberate
//!
//! - **Typed empty loops.** GWB represents a loop with no edges (the
//!   `mvfs` state) as a placeholder half-edge whose edge pointer is NIL,
//!   which makes every half-edge consumer handle a null case and makes
//!   `mate` partial. We put the emptiness where it belongs — in the loop,
//!   as [`LoopBoundary::Empty`] — so a [`HalfEdge`] always describes a
//!   real edge segment and **all its fields are non-optional**.
//! - **`Face::outer` is excluded from `Face::rings`** (GWB's `flout` is
//!   a member of `floops`); exactly-once loop ownership becomes a plain
//!   partition check.
//! - **No per-solid edge/vertex lists.** GWB's solids carry direct
//!   doubly-linked lists of their edges and vertices; ch. 10 §10.5 names
//!   them redundant conveniences. Arenas plus traversal replace them.
//! - **Upward back-pointers on the spine** (`Loop::face`, `Face::shell`,
//!   `Shell::solid`) are kept — §10.5 marks the loop back-pointer
//!   load-bearing for the Euler operators — and the validator checks
//!   ownership and back-pointers in both directions.

use slotmap::new_key_type;

use crate::geometry::{CurveKey, PointKey, SurfaceKey};

new_key_type! {
    /// Typed key into a body's solid arena.
    pub struct SolidKey;

    /// Typed key into a body's shell arena.
    pub struct ShellKey;

    /// Typed key into a body's face arena.
    pub struct FaceKey;

    /// Typed key into a body's loop arena.
    pub struct LoopKey;

    /// Typed key into a body's half-edge arena.
    pub struct HalfEdgeKey;

    /// Typed key into a body's edge arena.
    pub struct EdgeKey;

    /// Typed key into a body's vertex arena.
    pub struct VertexKey;
}

/// A solid: a connected volume bounded by one or more shells.
///
/// Multi-shell solids are constructible (a boolean's `A ∖ B` with `B`
/// strictly inside `A` yields an outer shell plus a reverted void
/// shell). The outer-shell/cavity-shell distinction — which shell
/// bounds material from outside versus which bound internal voids — is
/// **not stored**: the shell list carries no designation and this
/// structure does not enforce one. It is DERIVED from orientation
/// wherever it is needed, by the sign of a shell's signed volume (the
/// loops wind about the outward normal, so a cavity wall integrates
/// negative — `step_export::volume`).
///
/// A `SolidKey` is not a `ShellKey` — a solid's shell list rejects keys
/// of any other kind at compile time:
///
/// ```compile_fail,E0308
/// use topo::{Solid, SolidKey};
///
/// // This example must NOT compile: `shells` holds `ShellKey`s only.
/// let _ = Solid {
///     shells: vec![SolidKey::default()],
/// };
/// ```
#[derive(Clone, Debug)]
pub struct Solid {
    /// The shells bounding this solid, each owned by exactly this solid
    /// (validated, together with the shells' `solid` back-pointers).
    pub shells: Vec<ShellKey>,
}

/// A shell: one connected, closed boundary surface of a solid.
///
/// Closedness/watertightness in the half-edge representation is
/// *structural*: every edge has exactly two antiparallel half-edges and
/// every vertex orbit is a single cycle — both are tier-1 validator
/// checks, as are the component-aware per-shell Euler–Poincaré count
/// and the same-shell rule for an edge's two faces. Tier 2
/// ([`crate::validate_closed`]) additionally requires the shell's
/// incidence complex connected (c = 1) on finished solids.
#[derive(Clone, Debug)]
pub struct Shell {
    /// The faces of this shell, each owned by exactly this shell
    /// (validated, together with the faces' `shell` back-pointers).
    pub faces: Vec<FaceKey>,
    /// Back-pointer to the owning solid (validated against
    /// [`Solid::shells`]).
    pub solid: SolidKey,
}

/// A face: a bounded region of a surface, delimited by one outer loop
/// and zero or more interior loops (rings).
///
/// `outer` being non-optional bakes in "every face has at least one
/// loop": the face `mvfs` creates is born with an [`LoopBoundary::Empty`]
/// outer loop, never with no loop at all.
///
/// **Which loop is outer is a designation, not a derivable fact.** On a
/// closed surface the boundary curve alone does not distinguish "outside"
/// from "inside" — the designation is maintained by the operators (`mvfs`
/// designates the loop it creates; `lmef` designates the new face's
/// loop; ring reclassification is a deliberately separate helper, PR 3)
/// and the validator can only check its *consistency* (ownership
/// partition, `outer ∉ rings`), never recompute it.
///
/// Deviation from GWB, deliberate: `outer` is **excluded** from `rings`
/// (GWB keeps `flout` as a member of `floops`), so "every loop owned
/// exactly once across all faces, counting outer and rings" is a plain
/// partition check with no self-overlap special case.
///
/// **Orientation sense** (M5 S10, ratified as option (a) over
/// `Surface::Reversed` / NURBS conversion / negative-radius spheres):
/// the face's *outward normal* is its surface's chart normal times the
/// sign of [`Face::sense`]. Before S10 the outward normal simply WAS
/// the chart normal, which left `revert` with nothing to write on a
/// curved face — the analytic chart normals are outward for either sign
/// of the radius (cyl/cone/torus: odd in r) or outward exactly under
/// the ratified `radius > 0` convention (sphere: even in r). The bit
/// moves the reversal into topology, where it is exact structure and
/// never a numeric decide.
#[derive(Clone, Debug)]
pub struct Face {
    /// The surface this face is a region of (D2: faces reference
    /// surfaces).
    pub surface: SurfaceKey,
    /// Orientation sense: `true` iff the material side agrees with the
    /// surface's chart normal (i.e. the face's outward normal is `+n`);
    /// `false` iff it is reversed (outward normal `-n`).
    ///
    /// **Writers (M5 S11).** The Euler operators mint `sense: true`
    /// (the material side is not op-level knowledge — `mef` sees two
    /// chords, not the profile); constructors attach the honest bit
    /// through [`crate::Body::set_face_sense`] wherever the chart
    /// normal points into material, decided from the profile's stored
    /// winding/turn structure, never numerically: extrude's concave
    /// arc walls, and a revolve's inward walls (bore cylinder, inward
    /// cone, under-side plane annulus, concave sphere/torus band). The
    /// remaining writer-to-be is curved [`crate::Body::revert`] (the
    /// follow-on unit), which flips every face of a body at once.
    /// Consumers are audited and threaded as of S10; the test-only
    /// door [`crate::Body::flipped_face_sense_for_tests`] exercises
    /// the *incoherent* single-face flip.
    ///
    /// STEP alignment: this is exactly `advanced_face.same_sense`
    /// (ISO 10303-42 `face_surface`), which PR 13's exporter consumes
    /// directly rather than deriving.
    pub sense: bool,
    /// The outer boundary loop (see the type docs: a maintained
    /// designation). Not a member of `rings`.
    pub outer: LoopKey,
    /// The interior loops (rings), each owned by exactly this face
    /// (validated). Rings run clockwise viewed from outside, per the
    /// interior-left rule (module docs).
    pub rings: Vec<LoopKey>,
    /// Back-pointer to the owning shell (validated against
    /// [`Shell::faces`]).
    pub shell: ShellKey,
}

impl Face {
    /// The face's **outward-normal sign**: `+1` when [`Face::sense`] is
    /// `true` (material side agrees with the surface's chart normal),
    /// `-1` when it is `false`. The face's outward normal at a point is
    /// `sense_sign() * chart_normal(u, v)`.
    ///
    /// Exact structure, not a numeric decision: the sign is *selected*
    /// by a `bool`, so no comparison, no tolerance, and nothing for the
    /// k-lint to flag. `Interval` instantiations get an exact `±1`
    /// (`Real::one()` is exact in every backend), so a sense flip is a
    /// bitwise sign change, never a widening.
    #[must_use]
    pub fn sense_sign<T: geom_core::Real>(&self) -> T {
        if self.sense { T::one() } else { -T::one() }
    }
}

/// The boundary state of a loop: a genuine cycle of half-edges, or the
/// *empty loop* degenerate state — a lone vertex, no edges at all.
///
/// The empty loop is Mäntylä's load-bearing degenerate state: it is what
/// `mvfs` creates (the skeletal body's one face bounds nothing but a
/// vertex) and what `kemr` can strand (an empty ring). GWB encodes it as
/// a placeholder half-edge with a NIL edge pointer; we ratified the
/// **typed** representation instead — the emptiness lives here in the
/// loop, so [`HalfEdge`] needs no nullable fields and
/// [`crate::Body::mate`] is total on live half-edges. Illegal states
/// ("a half-edge that is not half of any edge") are unrepresentable.
///
/// Named `Cycle`, not `Ring`: "ring" already means *interior loop of a
/// face* in this codebase (and in Mäntylä), and a cycle bounds outer
/// loops and rings alike.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopBoundary {
    /// The empty loop: a lone vertex, no edges. The vertex's `emanating`
    /// must be `None` (validated) — an empty loop's vertex has no
    /// half-edges.
    Empty {
        /// The lone vertex this loop consists of.
        vertex: VertexKey,
    },
    /// A circular cycle of half-edges, entered at an arbitrary
    /// representative. The full cycle is reached by following
    /// [`HalfEdge::next`]; the validator checks that the walk closes and
    /// that every member points back via
    /// [`HalfEdge::parent_loop`].
    Cycle {
        /// A representative half-edge of the cycle (which one is
        /// arbitrary and carries no meaning).
        first: HalfEdgeKey,
    },
}

/// A loop: one connected boundary of a face — either a cycle of
/// half-edges or an empty loop (see [`LoopBoundary`]).
#[derive(Clone, Debug)]
pub struct Loop {
    /// The boundary state (cycle or lone vertex).
    pub boundary: LoopBoundary,
    /// Back-pointer to the owning face (validated against
    /// [`Face::outer`]/[`Face::rings`]).
    pub face: FaceKey,
}

/// A half-edge: one directed side of an edge, belonging to exactly one
/// loop.
///
/// All fields are non-optional — the typed empty-loop state
/// ([`LoopBoundary::Empty`]) is what makes GWB's nullable placeholder
/// unnecessary (see the [module docs](self)). The end vertex is derived,
/// never stored: `end(he) = start(next(he))`
/// ([`crate::Body::half_edge_end`]). The mate (other half of `edge`) is
/// computed, never stored ([`crate::Body::mate`]).
#[derive(Clone, Debug)]
pub struct HalfEdge {
    /// The edge this half-edge is one side of. The edge points back via
    /// exactly one of its two slots (validated bijection).
    pub edge: EdgeKey,
    /// The vertex this half-edge starts at (walking the loop in `next`
    /// order).
    pub start: VertexKey,
    /// Back-pointer to the loop whose cycle this half-edge belongs to
    /// (validated against the cycle reached from
    /// [`LoopBoundary::Cycle`]).
    pub parent_loop: LoopKey,
    /// The next half-edge of the loop's circular cycle (`next`/`prev`
    /// are mutual inverses, validated).
    pub next: HalfEdgeKey,
    /// The previous half-edge of the loop's circular cycle.
    pub prev: HalfEdgeKey,
}

/// An edge: a bounded, undirected piece of a curve, realized as two
/// antiparallel half-edges.
///
/// Endpoint vertices are no longer stored (M0's `start`/`end` are gone):
/// they are derived from the half-edges —
/// `start(he_plus)`/`end(he_plus)` — so there is exactly one place the
/// incidence lives.
///
/// **`he_plus` defines the edge's intrinsic direction**, and the M2
/// forward contract fixes the geometry to it: the referenced curve's
/// parameter direction must agree with `he_plus` (increasing parameter
/// runs `start(he_plus) → end(he_plus)`). See the orientation section of
/// the [module docs](self).
#[derive(Clone, Debug)]
pub struct Edge {
    /// The positively-oriented half: defines the edge's intrinsic
    /// direction.
    pub he_plus: HalfEdgeKey,
    /// The negatively-oriented half: traverses the edge opposite to
    /// `he_plus` (antiparallel, validated).
    pub he_minus: HalfEdgeKey,
    /// The curve this edge is a piece of (D2: edges reference curves).
    pub curve: CurveKey,
}

/// A vertex: a point of the model where edges end (or a lone vertex of
/// an empty loop).
///
/// `emanating` is the **only `Option` in the topology structure**, and
/// its two states are a meaningful dichotomy, not a nullable link:
/// `Some(he)` names one half-edge starting here (which one is arbitrary;
/// the full set is the vertex orbit, [`crate::Body::vertex_orbit`]);
/// `None` means this is a *lone vertex* — the vertex of exactly one
/// [`LoopBoundary::Empty`] loop, with no half-edges at all. The validator
/// enforces the equivalence in both directions.
#[derive(Clone, Debug)]
pub struct Vertex {
    /// The point this vertex sits at (D2: vertices reference points).
    pub point: PointKey,
    /// One half-edge starting at this vertex, or `None` for a lone
    /// vertex (see the type docs).
    pub emanating: Option<HalfEdgeKey>,
}

/// A topology entity reference with its kind — the type-erased form of
/// the seven typed keys, for reporting and provenance lookup.
///
/// Typed keys keep arenas confusion-free, but error reporting and
/// D5-provenance lookup want "any entity" as one value; this closed enum
/// (all seven kinds, no more — D3 style) is that sum. It never grants
/// arena access — resolving one still requires matching back to the
/// typed key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityId {
    /// A solid.
    Solid(SolidKey),
    /// A shell.
    Shell(ShellKey),
    /// A face.
    Face(FaceKey),
    /// A loop.
    Loop(LoopKey),
    /// A half-edge.
    HalfEdge(HalfEdgeKey),
    /// An edge.
    Edge(EdgeKey),
    /// A vertex.
    Vertex(VertexKey),
}

impl core::fmt::Display for EntityId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Solid(k) => write!(f, "solid {k:?}"),
            Self::Shell(k) => write!(f, "shell {k:?}"),
            Self::Face(k) => write!(f, "face {k:?}"),
            Self::Loop(k) => write!(f, "loop {k:?}"),
            Self::HalfEdge(k) => write!(f, "half-edge {k:?}"),
            Self::Edge(k) => write!(f, "edge {k:?}"),
            Self::Vertex(k) => write!(f, "vertex {k:?}"),
        }
    }
}

/// A geometry-arena reference with its kind — the counterpart of
/// [`EntityId`] for the three geometry arenas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomRef {
    /// A point (vertex geometry).
    Point(PointKey),
    /// A curve (edge geometry).
    Curve(CurveKey),
    /// A surface (face geometry).
    Surface(SurfaceKey),
}

impl core::fmt::Display for GeomRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Point(k) => write!(f, "point {k:?}"),
            Self::Curve(k) => write!(f, "curve {k:?}"),
            Self::Surface(k) => write!(f, "surface {k:?}"),
        }
    }
}
