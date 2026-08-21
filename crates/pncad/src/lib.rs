//! The authoring façade: one crate to depend on.
//!
//! # The contract
//!
//! 1. **One dependency, closed over error payloads.** Everything an
//!    authoring consumer needs is reachable from `pncad`. The kernel
//!    crates are re-exported as modules (`pncad::profile`,
//!    `pncad::topo`, …), so a consumer's manifest names `pncad` and
//!    nothing else — including the *payload types of error enums*,
//!    which are otherwise the leak that forces a second `path`
//!    dependency: `topo::BooleanError::CurvedBooleanUnsupported`
//!    carries a `geom_brep::SurfaceKind` that `topo` does not
//!    re-export, so a `topo`-only consumer can receive the error and
//!    not spell its payload. Re-exporting the owning crates closes
//!    that whole class rather than one case, at the cost of a longer
//!    path for the few payloads that sit below their owner's root
//!    (`geom_core::spline::KnotAlgebraError`,
//!    `sweep::fillet::FilletError`, `topo::boolean::ContainError`,
//!    `mesh::validate::MeshError`) — a longer path, never a second
//!    crate — and it required **zero kernel edits**, which is the
//!    ruling other crates cite when they need a payload type and find
//!    its owner does not re-export it: the answer is a direct edge on
//!    the owning crate, never a new re-export added to somebody
//!    else's root. The one stated exception is `MigrationStep`, whose
//!    signature speaks `serde_json::Value`; [`document`] records why
//!    it stays out. `tests/all.rs` is the pin: it matches on the
//!    cross-crate payloads using only `pncad::` paths, and a guard
//!    test there reads its own source and fails if any kernel crate
//!    is named outside one.
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
//! # What the façade itself contains
//!
//! No geometry and no numeric behavior. The **authoring** surface is
//! re-exports and thin wrappers that do nothing but call into the
//! kernel: every [`authoring`] seam but one is a single kernel
//! constructor call, and [`validated`] is that one — the two-call
//! form (`Profile::new` then `Profile::validate`) the demo corpus
//! wrote by hand at every scene.
//!
//! (Stated as a shape, not a count, on purpose. The previous wording
//! said "six of the seven", which had been wrong since the `polygon`
//! door was removed — a stale count in the sentence whose job is to
//! say what is true. The shape is guarded:
//! `the_authoring_seam_roster_is_what_the_crate_doc_claims` in
//! `tests/all.rs` reads `authoring.rs` and fails if a seam is added
//! or removed, or if a second one chains a follow-up kernel call, so
//! this sentence cannot rot the same way twice.)
//!
//! **[`workspace`] is not that, deliberately.** It is a real
//! subsystem: it scans a directory of save files, reads each one's
//! `id:` header, refuses a duplicate id naming both claimants,
//! resolves a `DocRef` through the full load door and checks the
//! content pin it recomputes, writes new and rewritten files, mints
//! random document ids from OS entropy, and implements
//! [`document::PartResolver`] so an evaluation can cross the
//! document seam. It lives here rather than in `editor-core` because
//! the kernel is deterministic by construction: ambient randomness
//! and the filesystem are exactly what must stay out of it, so the
//! layer allowed to hold them is this one. What it adds is I/O and
//! identity — still no geometry and still no numerics.
//!
//! **[`tolerance`] is not that either**, and it is the third thing this
//! section has to name. It re-exports `geom_core`'s ε vocabulary and
//! adds three doors that *report* the run's committed ε and where it
//! came from. Two of them — [`tolerance::report`] and
//! [`tolerance::eps_source`] — **commit the ambient bootstrap as a side
//! effect of being asked**, exactly as `Tolerance::get` does, so a
//! program that later loads a document turns that load into a
//! `ToleranceConflict` by having asked. That is the one place in this
//! façade where calling a wrapper changes the run, and
//! [`tolerance::committed_report`] is the door that does not; the
//! module says so at each of the three. Still no geometry and still no
//! numerics.
//!
//! [`validated`]: authoring::validated
//!
//! # Start here
//!
//! **[`guide::journey`] is the guide** — quickstart, then the
//! canonical journey from authoring to export, in Rust and Python
//! side by side. Every code block in it is a doctest, so it cannot
//! drift from this crate. The rest of the written documentation:
//!
//! - [`guide::examples`] — the corpus as the example set: every tour
//!   scene and corpus document, and what each demonstrates.
//! - [`guide::fail_loud`] — the refusal vocabulary, layer by layer.
//!   If something refused and you want to know why, start there.
//! - [`guide::selecting`] — naming and selecting entities: the
//!   materializers, the pattern language, the geometric filters, and
//!   the detect/declare protocol. The worked examples for
//!   [`select`].
//! - [`guide::north_star_audit`] — what the Python bindings can
//!   author today, and the named gaps.
//!
//! The example corpus proper is `demos/tour`: 34 stops across 15
//! scene modules that depend on this crate and nothing else, each
//! running the same ladder — author a profile, build a body, validate
//! it at tiers 1→2→3/3′, measure its mass properties, tessellate,
//! cross-check the mesh against the exact measure, export. That
//! ladder is what the guide teaches.
//!
//! # A fifteen-line example
//!
//! ```
//! use pncad::prelude::*;
//!
//! let square: ClosedLoop<f64> = Open
//!     .at(p2(0.0, 0.0))
//!     .line_to(p2(1.0, 0.0))?
//!     .line_to(p2(1.0, 1.0))?
//!     .line_to(p2(0.0, 1.0))?
//!     .line_to(Start)?;
//! let profile = validated(SketchPlane::<f64>::xy(), vec![square.into()])?;
//! let body = extrude(&profile, Extrusion::Distance(real(1.0)))?;
//! let props = mass_properties(&body.body)?;
//! assert!((props.volume - 1.0).abs() < 1e-12);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// ---------------------------------------------------------------
// Module re-exports: the whole authoring surface, one hop away.
// ---------------------------------------------------------------

// NOT re-exported as a module: the document layer is exposed
// through the curated `document` + `select` surfaces instead. Its
// arena keys are body-lineage-scoped and must not leave editor-core,
// and a whole-crate re-export would hand them out one hop past that
// seal. Kernel crates keep their module re-exports below —
// they carry geometry, not keys into a particular evaluation.
/// Geometry: the analytic `Curve3`/`Surface` kinds, NURBS curves in
/// 2-D and 3-D and NURBS surfaces, fitting, projection.
pub use geom;
/// B-rep geometry primitives: surface/curve kinds, pcurves, section
/// classification. Mostly interior, but it owns error payload types
/// that surface through `topo` and `sweep` refusals.
pub use geom_brep;
/// The numeric foundation: points, vectors, transforms, the [`Real`]
/// scalar trait, and the tolerance environment.
///
/// [`Real`]: geom_core::Real
pub use geom_core;
/// Certified tessellation and the mesh validation cross-checks.
pub use mesh;
// `profile` is NOT re-exported whole:
// `pncad::profile::ProfileLoop::polygon` was the measured leak of the
// raw authoring tier past the curated surface. `pub mod profile`
// below is the narrowed replacement — a curated module in place of a
// whole-crate re-export, applied to one nameability.
/// The D6 API-boundary quantity layer: `Length`/`Angle`/
/// `Count` newtypes, the unit table + constants (`25.0 * MM`), and
/// the display formatter. NOTE: `quantity::Length` is the public
/// quantity type; `geom_core::predicate::Margin<T>` is the
/// kernel-internal classify-seam margin type — different things, and
/// only the former is prelude surface.
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
// document-layer path names it, and that measurement is what decides
// the re-export. Re-export it the day a consumer needs it.

pub mod authoring;
pub mod document;
pub mod export;
pub mod guide;
pub mod prelude;
pub mod profile;
pub mod select;
pub mod tolerance;
pub mod workspace;
