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
//! Every publicly-reachable error type of the re-exported crates was
//! read and its variant payloads classified. This list has been
//! through two passes: a first, `pub use`-driven sweep, and a second
//! after review found the first one incomplete. The types the first
//! pass missed are marked **\[2\]**, and they are instructive — each was
//! missed for a *structural* reason worth naming, because the same
//! reason will hide the next one:
//!
//! - `topo::boolean::ContainError` **\[2\]** — re-exported by its own
//!   crate's `boolean` module but NOT lifted to the crate root, while
//!   its neighbours in the same `pub use` line are. A root-level scan
//!   walks straight past it. It carries a cross-crate
//!   `geom_core::Indeterminate`: exactly the audited class.
//! - `geom_curves::EllipseInvalid` **\[2\]** — defined directly in its
//!   crate's root module with no `pub use` line at all, so a scan
//!   driven by re-export statements never sees it. Also carries
//!   `geom_core::Indeterminate`.
//! - `mesh::validate::MeshError` **\[2\]** — lives below the crate root;
//!   `mesh::MeshError` does not resolve.
//! - `geom_surfaces::SurfaceProjectionInconclusive` **\[2\]** — the
//!   first pass recorded "geom_surfaces defines no error type", which
//!   was simply wrong.
//! - `geom_core::Indeterminate`, `geom_core::ToleranceEnvErrorKind`,
//!   `step_import::{AdoptionAttempt, AdoptionCandidate}`,
//!   `editor_core::ResolutionFailure`,
//!   `editor_core::persist::MigrationError` **\[2\]** — public error or
//!   error-carrying types the first pass did not enumerate.
//!
//! The audited types, by crate (module path given where the type is
//! NOT at its crate's root — that distinction is the whole point):
//!
//! - `topo`: `BooleanError`, `TransformError`, `PcurveMintError`,
//!   `ValidationError`, `MassPropsError`, `EulerOpError`,
//!   `SplitReduceError`, `SplitJoinError`, `SplitFinishError`,
//!   `SplitError`, `PointInSolidError`, `PointInLoopError`,
//!   `PlaneEqError`, `MergeCoplanarError`, `RevertError`,
//!   `SourceAttachError`, `boolean::ContainError` **\[2\]**
//! - `sweep`: `ExtrudeError`, `RevolveError`, `LoftError`,
//!   `SkinError`, `TubeError`, `fillet::FilletError`
//! - `geom_brep`: `CertifyError`, `PcurveCertifyError`, `PropsError`,
//!   `NewellError`, `SsiError`, `SectionError`, `PcurveError`
//! - `geom_core`: `BandError`, `SplineError`, `ToleranceError`,
//!   `ToleranceEnvError`, `ToleranceEnvErrorKind` **\[2\]**,
//!   `Indeterminate` **\[2\]**, `spline::KnotAlgebraError`,
//!   `spline::ComposeError`, `linalg::lsq::LsqError`
//! - `geom_curves`: `FitError`, `ProjectionInconclusive`,
//!   `EllipseInvalid` **\[2\]**
//! - `geom_surfaces`: `SurfaceProjectionInconclusive` **\[2\]**
//! - `profile`: `ProfileError`
//! - `mesh`: `TessellateError`, `validate::MeshError` **\[2\]**
//! - `stl`: `StlError`
//! - `step_export`: `StepExportError`
//! - `step_import`: `StepImportError`, `AdoptionAttempt` **\[2\]**,
//!   `AdoptionCandidate` **\[2\]**
//! - `editor_core`: `NodeError`/`NodeErrorKind`, `EditError`,
//!   `NamingError`, `DuplicateName`, `ResolveError`, `HitTestError`, `EvalError`,
//!   `DimensionError`, `PersistError`, `SnapshotError`, `MetaError`,
//!   `MetaVersionError`, `ResolutionFailure` **\[2\]**,
//!   `persist::MigrationError` **\[2\]**
//!
//! **Result: the property holds, with one stated exception (below),
//! and it required zero kernel edits.** The `SurfaceKind` leak was
//! never a missing type — it was a gap in one crate's re-export list,
//! and a façade that re-exports the owning crates closes the class
//! structurally. Several payloads sit below their owner's crate root
//! and are reached by module path rather than a root re-export
//! (`geom_core::spline::KnotAlgebraError`,
//! `geom_core::linalg::lsq::LsqError`, `sweep::fillet::FilletError`,
//! `topo::boolean::ContainError`, `mesh::validate::MeshError`). Each
//! lives in a *public* module, so each is nameable through `pncad` —
//! a longer path, never a second crate.
//!
//! # The exception, stated
//!
//! Honesty is worth more here than a clean claim:
//!
//! 1. **`editor_core::MigrationStep`** is
//!    `fn(serde_json::Value) -> Result<serde_json::Value, MigrationError>`.
//!    Consuming it therefore requires naming `serde_json::Value`, a
//!    third-party type that `pncad` does not re-export. This is
//!    persistence-schema machinery rather than authoring surface, so
//!    the façade does not grow a `serde_json` dependency for it
//!    today; a consumer who writes schema migrations needs that crate
//!    directly. If the bindings unit ever exposes migrations, this is
//!    the decision to revisit.
//!
//! **Retired (#234):** `editor_core`'s `DuplicateName` — the error of
//! `NameTable::insert`/`insert_tied` — used to be a second exception:
//! a `pub` type in a *private* module, obtainable and matchable but
//! with no writable path from anywhere, its own crate included. The
//! U1 audit read that as a kernel wart no façade could fix, and it was
//! right that no FAÇADE could: the missing line was in `editor_core`,
//! which now re-exports the type at its root. `pncad::select` carries
//! it beside `NameTable`, so the façade route exists too.
//!
//! The remaining exception does not involve `bvh` (nothing in any
//! public error payload references it, which is what justifies leaving
//! it interior) or any other unreachable kernel type.
//!
//! # The pin
//!
//! `tests/all.rs` matches on the cross-crate payloads named above
//! using only `pncad::` paths, and guards itself.
//!
//! A previous version of this section claimed the pin was enforced by
//! the absence of dev-dependencies — that the test binary was
//! "physically incapable" of naming a kernel crate. **That was false.**
//! Cargo passes `--extern` for a crate's ordinary dependencies to its
//! test targets as well as its dev-dependencies, so all twelve are in
//! scope there no matter what the manifest says; review falsified the
//! claim by adding `use topo as _;` and watching it compile.
//!
//! The enforcement is instead a guard test that reads the test file's
//! own source and fails if any kernel crate is named outside a
//! `pncad::` path — catching both a `use` statement and an inline
//! qualified path, comments excluded so that documentation may still
//! discuss the leak by name. It is a source-level check executed as a
//! test, not a link-level impossibility. That is a weaker mechanism
//! than the one first advertised, and it is the one actually in place:
//! a guard that overstates itself is worse than no guard, because it
//! stops anyone from looking.
