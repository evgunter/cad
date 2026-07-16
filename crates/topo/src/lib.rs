//! Topology arenas, [`Body<T>`], and the structural validation harness —
//! the `topo` layer's M0 skeleton (D1 in `docs/DESIGN.md`).
//!
//! A B-rep body is a plain value: typed generational arenas
//! (slotmap-style, D1) for the six manifold topology kinds — solid,
//! shell, face, loop, edge, vertex — plus geometry arenas, plus a D5
//! provenance record per entity from birth. This crate is the M0
//! *skeleton* of that layer: the arena store, the containment spine, the
//! raw-insertion builder placeholder, and the structural validator. Euler
//! operators, the half-edge structure, and the real invariant checklist
//! (Euler–Poincaré, watertightness, residual ≤ ε) are M1, deliberately —
//! see the scope fences in [`entity`] and [`validate`].
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
//!
//! # Example
//!
//! Build the smallest well-formed body by raw insertion (the M1
//! placeholder path — Euler operators replace it) and validate it:
//!
//! ```
//! use geom_core::Point3;
//! use topo::{Body, CurveGeom, Edge, Face, Loop, Provenance, Shell, Solid, SurfaceGeom, Vertex};
//!
//! let prov = || Provenance::Primordial { op: "example" };
//! let origin = || Point3::new(0.0, 0.0, 0.0);
//!
//! let mut body = Body::<f64>::new();
//! let p0 = body.add_point(origin());
//! let p1 = body.add_point(Point3::new(1.0, 0.0, 0.0));
//! let v0 = body.add_vertex(Vertex { point: p0 }, prov());
//! let v1 = body.add_vertex(Vertex { point: p1 }, prov());
//! let c0 = body.add_curve(CurveGeom::Placeholder { anchor: origin() });
//! let c1 = body.add_curve(CurveGeom::Placeholder { anchor: origin() });
//! let e0 = body.add_edge(Edge { start: v0, end: v1, curve: c0 }, prov());
//! let e1 = body.add_edge(Edge { start: v1, end: v0, curve: c1 }, prov());
//! let lp = body.add_loop(Loop { edges: vec![e0, e1] }, prov());
//! let srf = body.add_surface(SurfaceGeom::Placeholder { anchor: origin() });
//! let f = body.add_face(Face { surface: srf, loops: vec![lp] }, prov());
//! let sh = body.add_shell(Shell { faces: vec![f] }, prov());
//! body.add_solid(Solid { shells: vec![sh] }, prov());
//!
//! assert_eq!(topo::validate(&body), Ok(()));
//!
//! // A malformed body reports EVERY defect, typed:
//! body.add_point(origin()); // orphan geometry — nothing should leak
//! let errors = topo::validate(&body).unwrap_err();
//! assert_eq!(errors.len(), 1);
//! ```

pub mod body;
pub mod entity;
pub mod geometry;
pub mod provenance;
pub mod validate;

pub use body::Body;
pub use entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, GeomRef, Loop, LoopKey, Shell, ShellKey, Solid,
    SolidKey, Vertex, VertexKey,
};
pub use geometry::{CurveGeom, CurveKey, PointKey, SurfaceGeom, SurfaceKey};
pub use provenance::Provenance;
pub use validate::{ValidationError, validate};
