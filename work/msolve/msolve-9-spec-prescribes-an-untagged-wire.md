---
id: msolve-9-spec-prescribes-an-untagged-wire
kind: issue
title: MSOLVE-9's spec prescribes an untagged MateFrame wire, citing a LoggedEdit shape that no longer exists
status: open
priority: P1
cost: E
opened: 2026-09-23
rides_with: MSOLVE-9
---


`docs/MSOLVE-9-SPEC.md:70-74` prescribes the new `MateFrame` wire as
*"untagged over the two arms, each inner struct `deny_unknown_fields`
(the shape `LoggedEdit` established), so every existing document reads
as `Authored` unchanged"*. Two things under that sentence moved:

- **The precedent is gone.** PR 3123 (PORT) retired `LoggedEdit`'s
  two-shape untagged wire: an entry is now one struct,
  `{ "edit": …, "maintenance": […] }`, with no fallback, and
  the pre-rows bare shape refuses `Unreadable`. Ev's ruling there:
  backward compatibility with older logs is a red flag, and churn or a
  format change never outweighs a better final state.
- **The attribute is refused.** PR 2702 (PORT-DIMS-1) adds
  `scripts/gates/persist-no-backtracking.sh`, which fails CI on any
  `#[serde(untagged)]` under `crates/editor-core/src`. The load door's
  structured `DimensionError` rides a first-refusal-wins premise
  (`crates/editor-core/src/persist/refusal.rs`), and an untagged enum
  records a refusal on each failed arm and then discards it.

So the unit as specified would go red at the gate. The design it
wants — an existing file reading as `Authored` without a re-save — is
the compatibility PR 3123 just removed. An externally tagged
`MateFrame` (`{"Authored": {…}}` / `{"FromFace": {…}}`) with the
checked-in corpus regenerated is the shape that fits both, and C5's
row would move by the wrapping, as the LoggedEdit change moved
`golden.cad` and `die_composed_tour.pncad`. Whoever dispatches
MSOLVE-9 should amend the spec's wire paragraph before the lane reads
it; it is MSOLVE's spec, so this row does not edit it.
