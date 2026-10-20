---
id: shell-and-replace-face-kernel-defect-endings-split-by-class
kind: issue
title: topo: eight shell and replace-face refusals end in KERNEL_DEFECT_ENDING; six are bug-only and the two pcurve arms may be a fit's limit
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---

(ENCL implementer, from the census in
`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`,
on main 5df6e6c854.)

`[failure]` (D2) splits every state in two: one input reaches, which
is a typed refusal whose ending must name the decision's lever, and one
only a kernel bug reaches, which MUST panic ("downgrading one … to a
typed error launders a bug into a supported outcome"). A typed refusal
ending in `geom_core::KERNEL_DEFECT_ENDING` is therefore wrong in either
class. The one exception is a kernel approximation limit, which takes
`[tolerance.recourse]`'s last resort (`KERNEL_LIMIT_RECOURSE` /
`KERNEL_LIMIT_LAST_RESORT`), not this ending.

## Sites

Door for the first six: `topo::shell`. Door for the last two: the
offset/replace-face doors.

- `crates/topo/src/shell.rs` `ShellError` `Display`, `Insert` (~736).
  **Class 2, likely**: the shell op certifies its own cavities before
  the insert door reads them.
- `shell.rs` `ShellError` `Display`, `Pcurve` (~756). **Class 3,
  unsure**: no gate before the closing mint certifies that a fitted
  offset carrier admits pcurves, so the pcurve fit's own limit is not
  excluded. If one is shown to exclude it, this is class 2.
- `shell.rs` `ShellError` `Display`, `NotValid` (~767). **Class 2,
  likely**: a postcondition on the op's own result. `[failure]`: "a
  check that fails on the kernel's own result reports a bug and
  panics".
- `shell.rs` `AsShelled` `Display`, the six "asked" arms (`StaleFace`,
  `EmptyGroup`, `GroupChartsDiffer`/`TogetherChartMixed`,
  `SharedSurfaceKey`, `TogetherFaceRepeated`, `TogetherPartialSet`)
  (~808). **Class 2, sure**: the shell op composes every offset-door
  call itself.
- `shell.rs` `AsShelled` `Display`, `Fit { InvalidRequest }` (~894).
  **Class 2, sure**: the thickness gate passes only a finite, positive
  wall, and the tolerance is the run's witness.
- `shell.rs` `AsShelled` `Display`, `Pcurve` (~899). **Class 3,
  unsure**, as `ShellError::Pcurve`.
- `crates/topo/src/replace_face.rs` `ReplaceFaceError` `Display`,
  `Structure` (~843). **Class 2, likely**: moving a valid spline's
  control points keeps its knots and degree.
- `replace_face.rs` `ReplaceFaceError` `Display`, `ResultNotClosed`
  (~983). **Class 2, likely**: a postcondition on the door's own
  result.

## Repair shape

- **Class 2** (only a kernel bug reaches it): `unreachable!` or
  `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0
  first (a type that cannot hold the state is preferred). The tests
  that pin the refusal's text move to `#[should_panic]`, or are deleted
  where they only built the error value by hand.
- **Class 1** (input reaches it): the decision's own lever, written in
  its own words (`[tolerance.recourse]` (i)), and a test that reaches it
  through the door.
- **Class 3** (an approximation limit): the last-resort recourse.

Re-check each `likely` or `unsure` argument before acting on it: one
that fails moves the site to class 1.
