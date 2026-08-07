//! The closure property, and the audit that establishes it.
//!
//! # The property
//!
//! *Every type reachable through the public API of the re-exported
//! surface — including every error-enum payload — is nameable from
//! `pncad` without naming a second crate.*
//!
//! This is the property that makes "one dependency" true rather than
//! merely convenient. A façade that re-exports the operations but not
//! the types their refusals carry is a façade only until the first
//! consumer tries to `match` on a failure — at which point they add a
//! second `path` dependency and the promise is gone.
//!
//! # The leak that motivated it
//!
//! `topo::BooleanError::CurvedBooleanUnsupported` carries a
//! `geom_brep::SurfaceKind` — which surface kind refused. `topo` does
//! not re-export `SurfaceKind`, so a `topo`-only consumer can receive
//! that error but cannot spell its payload. The demo tour's manifest
//! documented exactly this, and depended on `geom-brep` for no other
//! reason.
//!
//! The fix is structural rather than case-by-case: because `pncad`
//! re-exports every crate that owns a payload type, the payload is
//! always nameable through *some* `pncad::` path. The leak was a gap
//! in one crate's re-export list; the façade closes the whole class.
//!
//! # The audit
//!
//! Every public error enum of the re-exported crates was read and its
//! variant payloads classified. The findings, in summary:
//!
//! - **Same-crate payloads** (the large majority) are trivially
//!   reachable.
//! - **Cross-crate payloads exported at their owner's root** —
//!   `geom_brep::SurfaceKind`, `geom_brep::PropsError`,
//!   `geom_brep::SectionError`, `geom_brep::NewellError`,
//!   `geom_brep::CertifyError`, `geom_core::BandError`,
//!   `geom_core::Indeterminate`, `geom_core::SplineError`,
//!   `geom_curves::FitError`, `geom_curves::EllipseInvalid`,
//!   `profile::ProfileError`, `topo::EulerOpError`,
//!   `topo::PcurveMintError`, `topo::TransformError`,
//!   `topo::BooleanError`, `topo::SplitError`, and the entity keys —
//!   are reachable as `pncad::<crate>::<Type>`.
//! - **Cross-crate payloads buried below their owner's root** — three
//!   of them: `geom_core::spline::KnotAlgebraError`,
//!   `geom_core::linalg::lsq::LsqError`, and `sweep::fillet::FilletError`.
//!   Each lives in a *public* module, so each is still nameable
//!   through `pncad` (`pncad::geom_core::spline::KnotAlgebraError`,
//!   and so on) — a longer path, but never a second crate. No kernel
//!   edit was required to close the property.
//!
//! The audited enums, by crate:
//!
//! - `topo`: `BooleanError`, `TransformError`, `PcurveMintError`,
//!   `ValidationError`, `MassPropsError`, `EulerOpError`,
//!   `SplitReduceError`, `SplitJoinError`, `SplitFinishError`,
//!   `SplitError`, `PointInSolidError`, `PointInLoopError`,
//!   `PlaneEqError`, `MergeCoplanarError`, `RevertError`,
//!   `SourceAttachError`
//! - `sweep`: `ExtrudeError`, `RevolveError`, `LoftError`,
//!   `SkinError`, `TubeError`, `fillet::FilletError`
//! - `geom_brep`: `CertifyError`, `PcurveCertifyError`, `PropsError`,
//!   `NewellError`, `SsiError`, `SectionError`, `PcurveError`
//! - `geom_core`: `BandError`, `SplineError`, `KnotAlgebraError`,
//!   `ComposeError`, `LsqError`, `ToleranceError`,
//!   `ToleranceEnvError`
//! - `geom_curves`: `FitError`, `ProjectionInconclusive`
//! - `geom_surfaces`: (defines no error type)
//! - `profile`: `ProfileError`
//! - `mesh`: `TessellateError`
//! - `stl`: `StlError`
//! - `step_export`: `StepExportError`
//! - `step_import`: `StepImportError`
//! - `editor_core`: `NodeError`/`NodeErrorKind`, `EditError`,
//!   `NamingError`, `ResolveError`, `HitTestError`, `EvalError`,
//!   `DimensionError`, `PersistError`, `SnapshotError`, `MetaError`,
//!   `MetaVersionError`
//!
//! # The pin
//!
//! `tests/all.rs` matches on the cross-crate payloads named above
//! using only `pncad::` paths. The test crate has **no
//! dev-dependencies at all**, so it is physically incapable of
//! reaching a kernel crate directly: if the property regresses, that
//! test stops compiling. Documentation of a closure property is a
//! claim; a test that cannot name a second crate is a proof.
