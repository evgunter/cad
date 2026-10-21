---
id: a-split-merging-a-two-piece-wall-reads-its-seam-vertex-as-a-pinch-corner
kind: issue
title: A split that merges a two-piece wall's halves across the seam leaves its loop through the seam vertex twice, and a near-tangent cut there refuses at rest as a pinch corner, ending as a kernel defect
status: open
opened: 2026-10-19
priority: P3
cost: M
---


## Finding

Measured by the `wire-splitmax` lane on PR 5334 (2026-10-19), against
`origin/main` at `4cd4835c4a`, with
`crates/sweep/tests/split_seam_vanishing_tilt.rs`
`a_split_through_a_seam_vertex_returns_no_side_tier_three_refuses`
logging every pose's outcome (80 poses: the two-piece unit cylinder
turned `φ`, cut through its seam vertex `v` at tilt `ψ` off the wall).

| ε | main: answered / at rest / other | PR 5334 | poses that moved |
|---|---|---|---|
| 1e-9 | 20 / 26 / 34 | 20 / 26 / 34 | none |
| 1e-12 | 20 / 28 / 32 | 20 / 28 / 32 | none |

No pose that answered on main refuses on the branch. What changed is
the finding some at-rest refusals carry. Each half now ends with
maximal faces (`topo::splitting::finish` `end_maximal`), so where the
cut leaves both half-walls' pieces in one half, they merge across the
seam strut, and the merged wall's loop passes `v` twice: once along
the rim, once where the near-tangent section ellipse leaves it at a
turn of about `ψ`. Check 9's pinch corner reads that turn:

- ε = 1e-9: 2 poses (`φ 0.9 ψ 1e-7`, `L 0.02` and `L 0.18`, sign −1)
  now refuse `PinchCornerEscalated` (margin 1.1e-9 in the band
  `[1e-9, 1e-8]`) where main refused `VolumeUncomputable`.
- ε = 1e-12: 7 poses (`ψ 1e-7` and `1.1e-7`, sign −1) now refuse
  `PinchCornerCrossed`.

**So it is a refusal of a body that was already marginal, not a
regression a user meets:** every such pose refused at rest on main
(ψ ≤ 1.1·10⁻⁷, the ellipse's semi-axis 10⁷). But the new finding's
ending is the kernel-defect one (`validate.rs`, `PinchCornerCrossed`
and `PinchCornerEscalated`'s `own_close(.., DEFECT)`), so the refusal
now blames the kernel for what is the carrier's rounding amplified by
the semi-axis, where main's `VolumeUncomputable` ended "no way through
yet". The test admits both findings (`carrier_resolution`).

## What it needs

Either the pinch-corner check reads the corner's turn through the
same carrier-resolution class the other findings do (no lever, not a
defect), or the merge declines to glue across a vertex its group's
loop would pass twice within the band of a crossing. The first is
`topo::validate` (TOPO / RESTFRONT); the second the merge door.

Signed (wire-splitmax lane).

## Parked (WIRE orchestrator, 2026-10-19)

Its question, which ending a carrier-rounding pinch corner gets, is
the class [ev] PR 5411 decides: a check over a result no proof covers
(row 6), or a row-1 carrier-resolution finding. Parked until 5411
answers. The merge-door alternative is WIRE's (`end_maximal`); the
validate alternative is TOPO / RESTFRONT's.

## Unparked (WIRE orchestrator, 2026-10-20)

PR 5411's ratified D9 answers its question: a site whose firing a
correct kernel can reach is not a defect but a precision limit the model
controls (row 1, a lever and `KERNEL_LIMIT_RECOURSE`). The near-tangent
pinch corner here comes from the carrier's rounding, amplified by the
semi-axis, so it takes the limit ending rather than `DEFECT`.
- If validate's `PinchCornerCrossed` / `PinchCornerEscalated` change
  their ending, that is the `validate.rs` fix (RESTFRONT's ground; see
  `work/scalar/kernel-defect-ending-sites-convert-to-panics-or-their-true-ending.md`).
- Alternatively, WIRE's merge door declines the glue.

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).
