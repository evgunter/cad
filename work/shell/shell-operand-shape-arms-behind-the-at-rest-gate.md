---
id: shell-operand-shape-arms-behind-the-at-rest-gate
kind: issue
title: shell's piece sort, Pieces, OperandOuterShells and ChartSenseMixed arms sit behind a gate that refuses what they read
status: open
opened: 2026-10-06
priority: P3
cost: E
---


`topo::shell_open` (`crates/topo/src/shell.rs`) takes `&AtRestBody`
since PR 4112, and tier 3 now runs before the verb reads anything. Four
arms read operand shapes that gate already refuses at a certifying
scalar — and the door is bounded on the certification right, so no
other scalar reaches it:

- **The piece sort** (`shell_open`, "Decide: one piece of material per
  solid", `crate::pieces::sort_into_pieces`) and
  **`ShellError::Pieces`**: check 10 refuses two `Outer` shells under one
  solid (`SolidOuterShells`) and a misplaced void (`ShellWinding`), so a
  finished operand is one piece per solid already. Every row that
  reached the sort built its operand with the test-only
  `with_solids_merged_for_tests`; those rows now read the gate's refusal
  (`shell5_r2_probes`, `shell8_r2_probes`, `hollow_island`).
- **`ShellError::OperandOuterShells`**: its `outer: 0` arm (a solid of
  only cavities) is check 7's `NegativeVolume`; its `outer ≥ 2` arm is
  check 10's `SolidOuterShells`, except where the two shell-role readers
  part in band (`work/fuse/one-home-for-where-a-shell-stands.md`), which
  is the one way it may still be reachable.
- **`ShellError::ChartSenseMixed`**: the only row that reached it built a
  body tier 3 refuses (`verbs_shell::a_mixed_sense_chart_built_from_outside_is_refused_at_the_gate`,
  `DescriptionNotAdjacent`). Whether a finished body can wear one chart
  with both senses within one solid (`step-import`'s shared keys) is
  unmeasured.

Owed: for each, find a finished operand that reaches it or retire it.
Retiring a variant crosses `editor-core/src/verbs/shell.rs`'s mapping,
`pncad-py/src/tags.rs` and `editor-core/tests/refusal_concision_chains.rs`.
