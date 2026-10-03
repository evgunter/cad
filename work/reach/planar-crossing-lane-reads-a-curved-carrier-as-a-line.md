---
id: planar-crossing-lane-reads-a-curved-carrier-as-a-line
kind: unit
title: the sweep's planar crossing lane reads a spiric or NURBS carrier as a line
status: review
opened: 2026-10-03
priority: P3
cost: M
branch: reach/planar-lane-curved-carrier
refs: [boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule]
pr: 3984
---


Found by both designers of the NURBS/spiric operand fork
(`analysis/design-fork/nurbs-spiric-operands-d1` and `-d2`, §premise).

## Finding

`splitting::classify`'s conic root door answered `Err(())` alike for a
line, a spiric and a NURBS carrier, and the sweep's planar arm
(`boolean::reduce::sweep_direction`) read `Err` as "a line: the M3 lane
below owns it": same-side endpoints meant no crossing, and opposite
ones a crossing interpolated linearly in the parameter. Only the
body-scoped operand gate (`gate_operand_edges`) kept a curved carrier
out of that arm.

Measured on `origin/main` (0770bfaa3) with the gate bypassed (a planar
sheet bounded by a quadratic Bézier arc, `A`'s sweep direction run
directly), identical at ε 1e-9, 1e-6 and 1e-12:

| fixture | oracle (closed form) | main |
|---|---|---|
| arc dipping through a brick face `y = ½` | two crossings in the face, `t = (1 ± √½)/2` | `Ok`, no contact |
| arc crossing a brick face `x = 1.5` once | one crossing, `t = 5/6`, `(1.5, 0.556, 0)` | `Ok`, a vertex at `(1.275, 0.75, 0)` recorded ON the face, 0.225 off its plane |

The full two-direction sweep returned `Ok(())` on the second.

## Fix

`plane_crossing_lane` answers `Line`, `Conic(meet)` or `Unlaned`. Every
lane that reads an operand edge's carrier refuses a spiric or NURBS one
at its own site as `BooleanError::EdgeCarrierUnsupported`, naming the
edge, the face and the lane (`EdgeCarrierSite`): the planar and the
curved crossing lanes, the join's germ frame and ring run, the vertex
sector walk, and the continuation scan's face extent. The split lane
keeps its `edge_clears` pass or `CurvedEdgeUnsupported`.

**Scope widened (REACH orchestrator, adopting the design fork's
reconciled reports):** with every site typed, `gate_operand_edges` and
`BooleanError::CurvedEdgeUnsupported` are deleted. Each site has a row
that reds without its fix (`boolean/planar_lane_carrier_rows.rs`,
`boolean/join.rs` `spline_edge_rows`,
`topo/tests/review_cleave_nurbs_lane.rs`), and
`sweep/tests/spline_spiric_operands.rs` runs the loft prism and the
spiric-rimmed cavity through every op against five brick placements in
both orders: every outcome is a typed refusal, and any body built is
held to its closed-form volume.

Class residue filed: `a-nurbs-edges-sector-departure-is-its-chord`
(now the split half only), `section-area-skips-a-spiric-or-nurbs-section-edge`,
`shell/replace-face-transports-a-nurbs-edge-as-a-ruling`; decisions
filed: `nurbs-edge-crossing-rung-is-the-ring-composite`,
`spiric-operand-edges-reopen-with-their-first-producer`.
