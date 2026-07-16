//! Topology arenas, [`Body<T>`], and the structural validation harness —
//! the `topo` layer with M1's half-edge representation (D1 in
//! `docs/DESIGN.md`).
//!
//! A B-rep body is a plain value: typed generational arenas
//! (slotmap-style, D1) for the seven manifold topology kinds — solid,
//! shell, face, loop, half-edge, edge, vertex — plus geometry arenas,
//! plus a D5 provenance record per entity from birth. M1 PR 1 lands the
//! half-edge structure itself (adjacency, orientation conventions, the
//! tier-1 structural validator); the Euler operators that make it the
//! exclusive construction path are the following PRs, so the raw
//! builder placeholder is still how bodies are assembled here.
//!
//! # Orientation conventions
//!
//! Documented **once**, in the [`entity`] module docs: the interior-left
//! rule (counterclockwise outer loops viewed from outside), antiparallel
//! half-edge pairs, derived end vertices, the vertex-orbit step, and the
//! GWB-is-mirrored transcription warning. Start there before touching
//! topology code.
//!
//! # The genericity boundary (Q1, settled by this crate's shape)
//!
//! `Body<T>` is **scalar-free topology plus geometry arenas over `T`**.
//! Topology entities contain no scalar values and no comparisons — only
//! typed keys — while the geometry arenas (points, curves, surfaces) are
//! the only `T`-carrying storage. Topology never branches on `T`: every
//! topology-determining decision was made at construction time through
//! named predicates (Q1 in `docs/DESIGN.md`), and a body is the *result*
//! of those decisions. An interval replay therefore materializes a
//! `Body<Interval>` with identical topology keys and interval-valued
//! geometry.
//!
//! # Determinism (D9)
//!
//! Arena iteration is slot-index order — deterministic given identical
//! construction history, which the pure-replay model guarantees. The
//! standing invariant (documented at [`body`]): no arena iteration may
//! feed a geometry decision unless the construction sequence is itself
//! deterministic; accordingly this crate contains no `HashMap`/`HashSet`.
//! Half-edge traversal is **bounded** — the walks cap at the arena
//! length and fail loud instead of spinning on corrupt input (the kernel
//! never hangs, D9).
//!
//! # Example
//!
//! Build the skeletal `mvfs` state — one solid, one shell, one face
//! whose outer loop is an *empty loop* holding a lone vertex — by raw
//! insertion, and validate it. (This is the state PR 2's `mvfs` operator
//! mints in one call; the raw builder inserts with provisional null keys
//! and patches, because the structure's references are cyclic.)
//!
//! ```
//! use geom_core::Point3;
//! use topo::{Body, Face, FaceKey, Loop, LoopBoundary, Provenance, Shell, Solid, SurfaceGeom, Vertex};
//!
//! let prov = || Provenance::Primordial { op: "example" };
//!
//! let mut body = Body::<f64>::new();
//! let p = body.add_point(Point3::new(0.0, 0.0, 0.0));
//! let v = body.add_vertex(Vertex { point: p, emanating: None }, prov());
//! let solid = body.add_solid(Solid { shells: vec![] }, prov());
//! let shell = body.add_shell(Shell { faces: vec![], solid }, prov());
//! body.get_solid_mut(solid).unwrap().shells.push(shell);
//! let srf = body.add_surface(SurfaceGeom::Placeholder { anchor: Point3::new(0.0, 0.0, 0.0) });
//! // The loop's face does not exist yet: insert with a provisional null
//! // key (it never resolves), then patch once the face is minted.
//! let lp = body.add_loop(
//!     Loop { boundary: LoopBoundary::Empty { vertex: v }, face: FaceKey::default() },
//!     prov(),
//! );
//! let f = body.add_face(Face { surface: srf, outer: lp, rings: vec![], shell }, prov());
//! body.get_loop_mut(lp).unwrap().face = f;
//! body.get_shell_mut(shell).unwrap().faces.push(f);
//!
//! assert_eq!(topo::validate(&body), Ok(()));
//!
//! // A malformed body reports EVERY defect, typed:
//! body.add_point(Point3::new(1.0, 0.0, 0.0)); // orphan geometry — nothing should leak
//! let errors = topo::validate(&body).unwrap_err();
//! assert_eq!(errors.len(), 1);
//! ```

pub mod body;
pub mod entity;
#[cfg(test)]
pub(crate) mod fixtures;
pub mod geometry;
pub mod provenance;
pub mod validate;

pub use body::Body;
pub use entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, GeomRef, HalfEdge, HalfEdgeKey, Loop, LoopBoundary,
    LoopKey, Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey,
};
pub use geometry::{CurveGeom, CurveKey, PointKey, SurfaceGeom, SurfaceKey};
pub use provenance::Provenance;
pub use validate::{ValidationError, validate};
