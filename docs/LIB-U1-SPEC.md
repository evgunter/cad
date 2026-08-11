# LIB-U1 spec — the façade crate + prelude (binding)

Mandate (LIBRARY-DESIGN.md §L5 U1, authorized §L8): ONE crate to
depend on for authoring — re-exports, a curated prelude, f64-first
entry points — killing the P8 boilerplate class and closing the
`SurfaceKind` error-payload leak. This spec is binding: deviations
are REPORTED (numbered, with the executed blocker), never
improvised silently.

## 0. Output discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
reports ≤150 lines. The 64k output limit kills agents that draft
whole files in one Write. Run every build/battery row as a
synchronous FOREGROUND Bash call, one at a time, reading each
result before the next; NEVER arm waiters, monitors, or background
chains for your own builds/tests.

## 1. The fence (two orchestrators share this repo)

- **New code lives in `crates/pncad`** (the Q9 placeholder name —
  do not bikeshed it) plus the ONE member line in root
  `Cargo.toml`, plus the `demos/tour` rework (§4).
- **Permitted micro-edits elsewhere**: individual `pub use`
  re-export lines in kernel crates ONLY where the closure
  property (§3) is otherwise unreachable — each one reported.
  No other kernel-crate edits; no semantic changes anywhere.
- **No CI edits** (the hosted matrix picks up workspace members
  automatically). No edits to `docs/M6-*`, `docs/M7-*`,
  `crates/step-import`, `crates/step-export`, `scripts/`.
- **No montage/render regeneration** — tour SOURCE edits are in
  scope, committed render outputs are not.
- **Do not touch `SectionSegments`** or any loft/sweep section
  plumbing — that is unit U3, not this unit.
- `publish = false` stays workspace-wide (nothing publishable
  before Q9).

## 2. The crate

`crates/pncad`, workspace member. Structure:

- **Module re-exports**: `pub use` each authoring-surface crate as
  a module (`pncad::profile`, `pncad::sweep`, `pncad::topo`,
  `pncad::geom_core`, `pncad::geom_curves`, `pncad::geom_surfaces`,
  `pncad::geom_brep`, `pncad::mesh`, `pncad::stl`,
  `pncad::step_export`, `pncad::step_import`, `pncad::editor_core`)
  — the tour's eleven path-deps plus `step-import`. `bvh` is
  interior unless you find a demo/test consumer that needs it
  (measured call, reported).
- **`pncad::prelude`**: the curated common surface. Derive the
  inventory from measurement, not taste: what the 18 tour scenes
  and the step-export/editor-core corpora actually import (the
  LIBRARY-DESIGN §L2 survey is the map). Expect: `LoopBuilder`,
  profile/loop types, the four body ops' entry points, validation
  tiers, mass properties, tessellation, STL/STEP export doors,
  the `Doc`/`DocEdit`/evaluate surface. Report the final
  inventory as a list in the PR body.
- **f64-first seams**: type aliases and thin wrappers so an
  authoring user never writes `S::from_f64` or a turbofish:
  promote the demos' six near-identical `p2` helpers and five
  `validated` wrappers (`demos/tour/src/bodies.rs:20-35` et al.)
  into ONE façade-provided form each. Generic instantiation
  remains the kernel's interior; the façade seam pays the
  conversion once. No numeric behavior change — these are
  aliases/wrappers, not reimplementations.
- Crate-level rustdoc: a short statement of the façade contract
  (one dependency, prelude, fail-loud typed errors) + a pointer
  to the tour as the example corpus. Real docs are U10; do not
  write a book.

## 3. The closure property (the SurfaceKind leak, generalized)

Requirement: **every type reachable through the public API of the
re-exported surface — including every error-enum payload — is
importable from `pncad` without naming a second crate.** The known
instance: `topo::BooleanError::CurvedBooleanUnsupported` carries
`geom_brep::SurfaceKind`, which `topo` does not re-export
(documented in `demos/tour/Cargo.toml`). Fix the CLASS, not the
instance:

- Audit the public error enums of the re-exported crates for
  payload types not otherwise exported; make each reachable from
  the façade (module re-exports usually suffice; where a payload
  type is buried, a reported `pub use` at its home crate is the
  permitted micro-edit).
- Pin it with a compile-level test in `pncad`: a test module that
  `match`es on each cross-crate error payload (the SurfaceKind
  case verbatim from the tour's comment) using ONLY `pncad::...`
  imports. List the audited enums in the PR body.
- Delete the leak comment from `demos/tour/Cargo.toml` when the
  rework (§4) makes it moot.

## 4. Acceptance: the tour reworked onto the façade

`demos/tour/Cargo.toml` ends with exactly ONE kernel path
dependency: `pncad`. All 18 scenes compile and the tour battery
passes unchanged — same pins, same ε rows, zero geometry diffs
(this unit changes imports and deletes boilerplate; any changed
number is a defect). The per-scene `p2`/`validated` definitions
are deleted in favor of the façade forms. Scene code otherwise
untouched — profile-authoring rework is U2 PR-2, not this unit.

## 5. Verification ladder (foreground, one row at a time)

1. `cargo build -p pncad`, then clippy (workspace lint config;
   `clippy::panic` denied in production code).
2. `pncad` closure tests (§3).
3. `cargo test -p tour` (or the tour's battery entry — discover
   it from `demos/tour`; the ε rows are part of it).
4. Workspace battery per `local-scripts/test-fast.sh` locally for
   iteration; **hosted CI is the only gate**.

## 6. PR discipline

- Commit AND push after every coherent unit of work.
- **Merge `origin/main` immediately before opening the PR, and
  re-merge whenever main moves while it is open** (a CONFLICTING
  PR runs NO checks — it looks like CI absence, not failure).
  After any push, confirm checks actually STARTED
  (`gh pr checks` shows rows).
- PR body: the sanitized logical documentation — façade contract,
  prelude inventory, audited-enum list, reported deviations
  (numbered), micro-edits made under the §1 permission.
- **NO Co-Authored-By trailer in lane commits** (A/B blinding
  overrides the harness convention); if a model mention lands in
  a PUSHED commit, STOP and report to the orchestrator — never
  rewrite history yourself.
- Report completion to the orchestrator (report file per your
  dispatch message); the orchestrator runs the review pass and
  merges. Do not self-merge.
