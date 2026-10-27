---
id: main-red-a-lofted-operand-and-an-approx-cap-refuse-differently
kind: issue
title: Main red (sweep slow set): m5_s11's lofted operand and operand_gate_pose's Approx cap no longer refuse at their pinned doors, at every eps
status: open
opened: 2026-10-27
priority: P1
cost: M
---


**Main red.** Measured by a restread triage lane on `origin/main` at
`9d97abd704` (2026-10-27), `sweep`'s `all` binary built with
`--release`, each row run alone under `--profile ci
--ignore-default-filter`. Both rows are in the **slow set**, and both are
red at the default ε and at 1e-12.

- `m5_s11_concave_sense::a_lofted_operand_refuses_the_union_check_typed`
  (`crates/sweep/tests/m5_s11_concave_sense.rs:853`). The union of the
  lofted notch with the pellet still refuses, but at the face-pair door
  (`BooleanError::CurvedBooleanUnsupported`, "…where one solid's spline
  (NURBS) face meets the face of the other solid…: that pairing of faces
  is not supported yet") rather than the edge-carrier door ("…is a
  spiric or spline (NURBS) curve") that the row pins. The two solids
  are disjoint (the pellet sits inside the notch).
- `operand_gate_pose::an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose`
  (`crates/sweep/tests/operand_gate_pose.rs:197`):
  `ε 1e-9, identity, ∪: the cap clears the bar, so the op reaches the
  classification's Approx door: Ok(true)`. The union of a box with an
  `Approx` cap and a bar clear of it now **builds a body**, where the row
  pins `Containment(KindUnsupported { kind: Approx })`. `Approx` is off
  the boolean roster by decision, so a built body is either a new door
  or a wrong answer.

**Culprit: PR 5789** (`splice/converse-arm`, merge `a006eb49ba`), by a
first-parent bisect on both rows at the default ε. `a006eb49ba` fails
both rows, and its first parent `92f1fd60ea` passes both. Other points
measured: `7eb29ffbe0` green, `ab2b6108e8` red. Not PR 5723, the
operand-gate removal: its merge `d6eea3efa5` and that merge's first
parent both pass.

**Done when** both rows are green on main. The Approx cap must refuse at
its kind's door again, or the row must be re-pinned with a reason the
merge's PR states. The lofted operand's refusal must be the typed one the
row names, or the row must be re-pinned to the door that now answers
first.

## Evidence from `splice/interior-arm` (PR 5826), 2026-10-27

- With the analytic span × spline face arm in, `m5_s11`'s union reads
  every pellet edge clear of the loft's walls and refuses at the extent
  fallback, `NurbsExtentUnsupported { operand: A, face }`, naming
  `side_faces[0][2]`, the concave top wall. PR 5826 re-pins the row
  there. It is green at ε 1e-9, 1e-6 and 1e-12.
- `operand_gate_pose::an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose`
  passes at the default ε on main after PR 5814. It is still red at
  **ε 1e-6** on `origin/main` `1f851964` (measured alone in a clean
  worktree), at a different assertion,
  `crates/sweep/tests/operand_gate_pose.rs:239`:
  `ε 1e-6, identity, just over the cap: past the box, got
  Err(KindUnsupported { face: FaceKey(1v1), kind: Approx })`. That is
  the point-in-solid probe over the cap, which reads no side where the
  row expects `Out`. PR 5826 reds identically there.
