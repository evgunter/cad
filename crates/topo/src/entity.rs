//! Topology entities and their typed arena keys.
//!
//! The six entity kinds of the manifold B-rep (D1 in `docs/DESIGN.md`):
//! [`Solid`], [`Shell`], [`Face`], [`Loop`], [`Edge`], [`Vertex`]. Each
//! kind has its own key type minted by slotmap's `new_key_type!`, so a key
//! into one arena is a *different type* from a key into another — handing
//! a `VertexKey` to something expecting an `EdgeKey` fails to typecheck.
//! Cross-arena confusion is a compile error, not a runtime surprise:
//!
//! ```compile_fail
//! use geom_core::Point3;
//! use topo::{Body, Edge, Provenance, Vertex};
//!
//! let mut body = Body::<f64>::new();
//! let p = body.add_point(Point3::new(0.0, 0.0, 0.0));
//! let v = body.add_vertex(Vertex { point: p }, Provenance::Primordial { op: "doc" });
//! // A `PointKey` is not a `CurveKey`: an edge's `curve` field will not
//! // accept a key into the point arena. This example must NOT compile.
//! let _ = body.add_edge(
//!     Edge { start: v, end: v, curve: p },
//!     Provenance::Primordial { op: "doc" },
//! );
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
//! # Deliberately thin skeletons (M0 scope fence)
//!
//! Each entity carries only the uncontroversial **containment spine**:
//! Solid → Shells → Faces → Loops, plus Edge/Vertex existence and their
//! geometry references. The half-edge structure — orientation bits,
//! traversal direction, radial links — is *deliberately absent*: M1
//! settles it against the still-sought Mäntylä Euler-operator chapters
//! (and Hoffmann's boundary-rep chapter), and pre-deciding it now would be
//! design-by-guess. Each entity's docs name what M1 adds.

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

    /// Typed key into a body's edge arena.
    pub struct EdgeKey;

    /// Typed key into a body's vertex arena.
    pub struct VertexKey;
}

/// A solid: a connected volume bounded by one or more shells.
///
/// M0 carries only the shell list. M1 adds the outer-shell/cavity-shell
/// distinction (which shell bounds the material from outside versus which
/// bound internal voids) once orientation conventions are settled against
/// the Euler-operator references — that distinction is an orientation
/// question, and orientation is exactly what the M0 skeleton defers.
///
/// A `SolidKey` is not a `ShellKey` — a solid's shell list rejects keys of
/// any other kind at compile time:
///
/// ```compile_fail
/// use topo::{Body, Provenance, Solid};
///
/// let mut body = Body::<f64>::new();
/// let solid = body.add_solid(Solid { shells: vec![] }, Provenance::Primordial { op: "doc" });
/// // This example must NOT compile: `shells` holds `ShellKey`s only.
/// let _ = body.add_solid(
///     Solid { shells: vec![solid] },
///     Provenance::Primordial { op: "doc" },
/// );
/// ```
#[derive(Clone, Debug)]
pub struct Solid {
    /// The shells bounding this solid, each owned by exactly this solid
    /// (validated).
    pub shells: Vec<ShellKey>,
}

/// A shell: one connected, closed boundary surface of a solid.
///
/// M0 carries only the face list. M1 adds orientation coherence — the
/// consistent outward (or void-inward) sense of the member faces — and
/// the closedness/watertightness checks that give "shell" its geometric
/// meaning; both need the half-edge structure the M0 skeleton defers.
#[derive(Clone, Debug)]
pub struct Shell {
    /// The faces of this shell, each owned by exactly this shell
    /// (validated).
    pub faces: Vec<FaceKey>,
}

/// A face: a bounded region of a surface, delimited by loops.
///
/// M0 carries the surface reference and the loop list. M1 adds the
/// orientation bit (whether the face normal agrees with the surface
/// normal — `same_sense` in kernel argot) and the outer-loop/inner-loop
/// (hole) distinction; both are orientation conventions deferred with the
/// half-edge structure. Per-loop pcurve caches (D2) arrive with real
/// surface geometry at M2.
#[derive(Clone, Debug)]
pub struct Face {
    /// The surface this face is a region of (D2: faces reference
    /// surfaces).
    pub surface: SurfaceKey,
    /// The boundary loops, each owned by exactly this face (validated).
    /// Which loop is the outer boundary is an M1 orientation question.
    pub loops: Vec<LoopKey>,
}

/// A loop: one closed boundary curve of a face, as a cycle of edges.
///
/// M0 stores the edges as an **ordered `Vec` placeholder** — deliberately
/// *without* per-edge traversal direction. A real loop must know which way
/// it walks each edge (the same edge is walked in opposite directions by
/// its two adjacent faces' loops); that datum is the heart of the
/// half-edge structure M1 settles against the Euler-operator references,
/// and faking it now with a bare bool would pre-decide the representation.
/// M1 replaces this with the half-edge ring (orientation bits, radial
/// links to the mate loop across each edge).
#[derive(Clone, Debug)]
pub struct Loop {
    /// The edges of the cycle, in walk order (direction of walk per edge
    /// is M1's half-edge data). Each edge must be referenced by at least
    /// one loop (validated); the manifold "exactly two loops per interior
    /// edge" pairing is M1 watertightness.
    pub edges: Vec<EdgeKey>,
}

/// An edge: a bounded piece of a curve between two vertices.
///
/// M0 carries the endpoint vertices and the curve reference. M1 adds the
/// half-edge pair (one directed half per adjacent loop, with radial links)
/// once the representation is settled; per-face pcurve caches with
/// certified residuals (D2/D4 ¶2) arrive with real curve geometry at M2.
/// `start`/`end` name the curve's own parameter direction; which way a
/// loop walks the edge is the loop's (M1) business.
#[derive(Clone, Debug)]
pub struct Edge {
    /// The vertex at the curve-parameter start of the edge. A closed edge
    /// (full circle) has `start == end`.
    pub start: VertexKey,
    /// The vertex at the curve-parameter end of the edge.
    pub end: VertexKey,
    /// The curve this edge is a piece of (D2: edges reference curves).
    pub curve: CurveKey,
}

/// A vertex: a point of the model where edges end.
///
/// M0 carries only the point reference. M1 may add edge back-references
/// (the vertex→edge star) if the Euler operators want them; D2's
/// vertex-geometry taxonomy (a vertex as the intersection of three
/// surfaces, with a witness) is deferred with the rest of the intensional
/// descriptions.
#[derive(Clone, Debug)]
pub struct Vertex {
    /// The point this vertex sits at (D2: vertices reference points).
    pub point: PointKey,
}

/// A topology entity reference with its kind — the type-erased form of the
/// six typed keys, for reporting and provenance lookup.
///
/// Typed keys keep arenas confusion-free, but error reporting and
/// D5-provenance lookup want "any entity" as one value; this closed enum
/// (all six kinds, no more — D3 style) is that sum. It never grants arena
/// access — resolving one still requires matching back to the typed key.
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
