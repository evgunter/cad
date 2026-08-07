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
//! own. Every item below is either a re-export or a constructor that
//! calls exactly one kernel constructor.
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

/// The document/edit layer: `Doc`, `DocEdit`, expressions, evaluation,
/// stable names. The layer the GUI and the language bindings speak.
pub use editor_core;
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
pub mod prelude;
