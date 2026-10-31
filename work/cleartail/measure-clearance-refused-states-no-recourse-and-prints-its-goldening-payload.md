---
id: measure-clearance-refused-states-no-recourse-and-prints-its-goldening-payload
kind: issue
title: NodeErrorKind::MeasureClearanceRefused renders a class name and the goldening payload, and none of its eleven arms states a recourse
status: open
opened: 2026-10-20
priority: P4
cost: M
---


Found by `wire-census2`, whose PR gives each of
`clearance::ClearanceRefusal`'s eleven arms a row in the feature-tree
census (`every_forwarded_arm_has_a_row`, `crates/editor-core/tests/refusal_concision_chains.rs:1389`; before it, one
arm had a row).

`NodeErrorKind::MeasureClearanceRefused`'s `Display`
(`crates/editor-core/src/eval/mod.rs:2718`) renders "the clearance engine
refused `{name}` ({payload})": the class name `ClearanceRefusal::name`
gives the goldening form, then `said_payload`, which is
`ClearanceRefusal::payload`'s machine channel for every arm but
`Selection` (`crates/editor-core/src/clearance.rs:577`, `payload` at `:674`). So:

- **No arm states a recourse.** All eleven rows are on
  `FILED_NO_RECOURSE` (`crates/editor-core/tests/refusal_concision_chains.rs:258`) under a comment citing this file: `Sliver`,
  `Budget`, `Unsupported`, `Selection`, `PoisonEnclosure`,
  `NothingCertified`, `NotADistance`, `EmptyScope`,
  `ToleranceHasNoBand`, `NoAdmittedPair`, `WitnessUnverified`.
- **The payload is the goldening form.** `Unsupported` and
  `PoisonEnclosure` print faces as `FaceKey` `Debug`, `NotADistance`
  prints the bound's bits (`bff0000000000000` for −1), and `Budget`
  prints `CellBudget`'s `Debug` (`Depth { max_cell_depth: 24 }`), which
  the shape guard reads as a `Debug` struct and a stage label. Each
  span is on `ADMISSIONS` (`crates/editor-core/tests/refusal_concision_chains.rs:200`) citing this file. (The `Unsupported` span
  was filed on `work/props/props-refusal-prose-outgrows-the-viewer.md`,
  which no longer exists.)

## Repair shape

A sentence per arm in words, from the facts `ClearanceRefusal` carries,
each ending in its recourse — a bigger budget names its dial, a
`NotADistance` asks for a non-negative bound, `Sliver` sends the reader
to the geometry — or in the kernel-defect ending for
`WitnessUnverified`, which the enum's own docs call unreachable. Keep
`name` and `payload` for the goldening form. Then drop the rows'
admissions.

## Narrowed (SYMCLEAR, 2026-10-28)

`NodeErrorKind::MeasureClearanceRefused` now carries
`topo::clearance::MinSeparationRefusal`, the arms its door can raise:
`Unsupported`, `PoisonEnclosure`, `EmptyScope` (SELFX retired the
fourth, `NoAdmittedPair`: scopes that meet read 0). The other seven rows and the `NotADistance` and
`Budget` admissions are gone, and the census rows are
`MeasureClearanceRefused/<arm>`.
