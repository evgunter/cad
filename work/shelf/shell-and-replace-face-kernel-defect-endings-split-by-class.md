---
id: shell-and-replace-face-kernel-defect-endings-split-by-class
kind: issue
title: topo: ten shell, replace-face and transform refusals end in KERNEL_DEFECT_ENDING; seven are bug-only, a caller's scaffold reaches the transform's, and the two pcurve arms may be a fit's limit
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
blocked_on: [replace-face-transports-a-nurbs-edge-as-a-ruling]
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

Door for the first six: `topo::shell`. Door for the last two: the
offset/replace-face doors.

- `crates/topo/src/shell.rs` `ShellError` `Display`, `Insert` (~736).
  **Class 2, likely**: the shell op certifies its own cavities before
  the insert door reads them.
- `shell.rs` `ShellError` `Display`, `Pcurve` (~756). **Class 3,
  unsure**: no gate before the closing mint certifies that a fitted
  offset carrier admits pcurves, so the pcurve fit's own limit is not
  excluded. If one is shown to exclude it, this is class 2. Also,
  `PcurveMintError` wraps `UnsupportedChart` and `IsoUnsupported`
  (`pcurve_cache.rs`). Those parts may be D2 row 2 (not yet built,
  `NOT_YET_ENDING`) rather than approximation limits, so decide per
  arm.
- `shell.rs` `ShellError` `Display`, `NotValid` (~767). **Class 2,
  likely**: a postcondition on the op's own result. `[failure]`: "a
  check that fails on the kernel's own result reports a bug and
  panics". It is probably reached today through the bug that
  `ResultNotClosed` (below) is.
- `shell.rs` `AsShelled` `Display`, the six "asked" arms (`StaleFace`,
  `EmptyGroup`, `GroupChartsDiffer`/`TogetherChartMixed`,
  `SharedSurfaceKey`, `TogetherFaceRepeated`, `TogetherPartialSet`)
  (~808). **Class 2, sure**: the shell op composes every offset-door
  call itself.
- `shell.rs` `AsShelled` `Display`, `Fit { InvalidRequest }` (~894).
  **Class 2, sure**: the thickness gate passes only a finite, positive
  wall, and the tolerance is the run's witness.
- `shell.rs` `AsShelled` `Display`, `Pcurve` (~899). **Class 3,
  unsure**, as `ShellError::Pcurve`, including the per-arm row-2
  question.
- `crates/topo/src/replace_face.rs` `ReplaceFaceError` `Display`,
  `Structure` (~843). **Class 2, likely**: moving a valid spline's
  control points keeps its knots and degree.
- `replace_face.rs` `ReplaceFaceError` `Display`, `ResultNotClosed`
  (~983). **Class 2, likely**: a postcondition on the door's own
  result.

**Order: after `replace-face-transports-a-nurbs-edge-as-a-ruling`**
(`blocked_on`). That open row is a known input trigger of
`ResultNotClosed` and, probably, of `ShellError::NotValid`: input
reaches the postcondition through that kernel bug. Converting either
site to a panic before the transport fix lands ships a panic that input
reaches.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.
- **Class 3**: the last-resort recourse (`KERNEL_LIMIT_RECOURSE` / `KERNEL_LIMIT_LAST_RESORT`).

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.

## Second-pass sites

(Census second pass, as of main 69ab780194: `## Census, second pass` in the parent row.)

- `crates/topo/src/transform.rs` `TransformError::render`, `NullScaffold` (~202), through `recourse::defect_ending(at)` at a build. Door: `topo::transform_rigid`. **Class 1, likely**: the public `mvfs` → `set_face_surface(plane)` → `Body::mev_null(MevSite::Lone)` leaves a null-scaffold edge on a body a caller then moves. The tree treats scaffolding as the caller's unfinished edit elsewhere (`Unfinished::SCAFFOLDING_REFUSAL`, blend's breach arm, mesh `NullScaffoldEdge`). Repair: that lever, "finish the edit first". PR 5464 made its build-side `Display` end in `KERNEL_DEFECT_ENDING`. **Fixed by PR 5528**: at a build it ends in `Unfinished::SCAFFOLDING_RECOURSE`; at rest and at import it keeps the door's defect ending.
- `transform.rs` `TransformError::render`, `Corrupt` (~227). **Class 2, likely**: a dangling key is a tier-1 break, and tier 1 holds at every public door (`[failure]`'s closure property). Class 2's repair above applies.
