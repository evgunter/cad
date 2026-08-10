//! The authoring façade: one crate to depend on.
//!
//! # The contract
//!
//! 1. **One dependency.** Everything an authoring consumer needs is
//!    reachable from `pncad`. The kernel crates are re-exported as
//!    modules (`pncad::profile`, `pncad::topo`, …), so a consumer's
//!    manifest names `pncad` and nothing else — including for the
//!    *payload types of error enums*, which are otherwise the leak
//!    that forces a second `path` dependency (see [`closure`]).
//! 2. **A prelude.** [`prelude`] is the curated common surface,
//!    derived from what the demo corpus actually imports rather than
//!    from taste: the profile vocabulary, the four body operations,
//!    the validation ladder, mass properties, tessellation, the
//!    STL/STEP export doors, and the document layer's entry points.
//!    Everything else stays one module hop away.
//! 3. **f64-first seams.** The kernel is generic over
//!    [`geom_core::Real`] so it can run at `f64`, at certified
//!    intervals, and at dual numbers. Authoring code should not pay
//!    for that: [`authoring`] holds the thin constructors that take
//!    plain `f64` and embed it, so a scene writes `p3(0.0, 0.0, 1.0)`
//!    instead of `Point3::new(S::from_f64(0.0), …)`. Generic
//!    instantiation stays the kernel's interior; the seam pays the
//!    conversion once, exactly (`from_f64` is an exact embedding for
//!    every implementor).
//! 4. **Fail-loud, typed errors.** The façade adds no `unwrap`, no
//!    silent default, and no softening: every wrapper here returns
//!    the kernel's own `Result` with the kernel's own error type. A
//!    façade that panicked where the kernel refused would be a worse
//!    library than no façade.
//!
//! The façade contains no geometry and no numeric behavior of its
//! own. Every item below is either a re-export or a thin wrapper that
//! does nothing but call into the kernel: six of the seven seam
//! functions are a single kernel constructor call, and [`validated`]
//! is the one two-call form (`Profile::new` then `Profile::validate`)
//! — the exact pair the demo corpus wrote by hand at every scene.
//!
//! [`validated`]: authoring::validated
//!
//! # Examples
//!
//! The example corpus is `demos/tour` — eighteen scenes that depend
//! on this crate and nothing else, each running the same ladder:
//! author a profile, build a body, validate it at tiers 1→2→3,
//! measure its mass properties, tessellate, cross-check, export.
//! Read the tour for how any of this is meant to be used; real
//! narrative documentation is a later unit of this program.
//!
//! ```
//! use pncad::prelude::*;
//!
//! let square = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
//! let profile = validated(SketchPlane::<f64>::xy(), vec![square])?;
//! let body = extrude(&profile, Extrusion::Distance(real(1.0)))?;
//! let props = mass_properties(&body.body)?;
//! assert!((props.volume - 1.0).abs() < 1e-12);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// ---------------------------------------------------------------
// Module re-exports: the whole authoring surface, one hop away.
// ---------------------------------------------------------------

// NOT re-exported as a module (LB13): the document layer is exposed
// through the curated `document` + `select` surfaces instead. Its
// arena keys are body-lineage-scoped and must not leave editor-core
// (G1), and a whole-crate re-export handed them out one hop past the
// LIB-U5 seal. Kernel crates keep their module re-exports below —
// they carry geometry, not keys into a particular evaluation.
/// B-rep geometry primitives: surface/curve kinds, pcurves, section
/// classification. Mostly interior, but it owns error payload types
/// that surface through `topo` and `sweep` refusals.
pub use geom_brep;
/// The numeric foundation: points, vectors, transforms, the [`Real`]
/// scalar trait, and the tolerance environment.
///
/// [`Real`]: geom_core::Real
pub use geom_core;
/// Curves: NURBS curves in 2-D and 3-D, fitting, projection.
pub use geom_curves;
/// Surfaces: the analytic `Surface` kinds and NURBS surfaces.
pub use geom_surfaces;
/// Certified tessellation and the mesh validation cross-checks.
pub use mesh;
/// 2-D profile authoring: loops, vertices, sketch planes, the
/// `LoopBuilder` sugar, and the validation tiers.
pub use profile;
/// The D6 API-boundary quantity layer (LIB-U8a): `Length`/`Angle`/
/// `Count` newtypes, the unit table + constants (`25.0 * MM`), and
/// the display formatter. NOTE: `quantity::Length` is the public
/// quantity type; `geom_core::predicate::Margin<T>` is the
/// kernel-internal classify-seam margin type (renamed from
/// `Length<T>`, LB9) — different things, and only the former is
/// prelude surface.
pub use quantity;
/// STEP AP242 export.
pub use step_export;
/// STEP import and its adoption diagnostics.
pub use step_import;
/// STL export, binary and ASCII.
pub use stl;
/// The four body operations: extrude, revolve, loft/sweep, fillet.
pub use sweep;
/// Topology: bodies, Booleans, splitting, transforms, the validation
/// ladder, and mass properties.
pub use topo;

// `bvh` is deliberately NOT re-exported: it is an interior
// acceleration structure. No demo scene, no export corpus, and no
// document-layer path names it — the measurement that decides this
// per the U1 spec. Re-export it the day a consumer needs it.

pub mod authoring;
pub mod closure;
pub mod document;
pub mod export;
pub mod prelude;
pub mod select;
