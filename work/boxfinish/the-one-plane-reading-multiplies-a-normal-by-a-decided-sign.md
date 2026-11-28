---
id: the-one-plane-reading-multiplies-a-normal-by-a-decided-sign
kind: unit
title: The reading that decides two planes are one plane multiplies the second normal by the decided orientation sign, so two walls that are one plane prove identical only when both normals happen to carry the same sign spelling
status: review
opened: 2026-11-24
priority: P1
cost: M
branch: boxfinish/one-plane
pr: 7710
---


The reader half of the line-directions weighing
(`a-door-reading-the-minted-axis-loses-identities-with-quotient-minted-partners`,
"Weighed (2026-11-24)"). It lands first, alone.

**What is there.** `geom_brep::plane_eq::plane_ladder` decides whether
two planes are one plane in rungs: parallel (`‖n₁ × n₂‖`), orientation
(a numeric sign, σ = ±1), then the offset `d₁ − σ·d₂`, where
`dᵢ = nᵢ·(oᵢ − c)` is each plane's distance from the centre `c` of the
region the verdict is used on. `carrier_eq::reading`'s plane arm has
the same `σ·n₂` in its offset and in its swing (`|n₁ − σ·n₂|`).

The offset is zero as an expression only if `n₁ ≡ σ·n₂`, which needs
the sign of whatever factor sits in one normal and not the other. Two
quotient-minted walls carried the same factor and it cancelled; a wall
whose normal is a literal axis beside one whose normal is a chord's
quotient needs `(ro − ri)/|ro − ri| ≡ 1`, a sign the symbolic tier does
not hold. Main is exposed today wherever a verb's literal axis meets a
quotient normal (a rectangle verb's wall beside a table polygon's wall
on one plane), whatever the doors read; that pair is reasoned from the
code, not run.

**The agreed final state** (two designers, two rounds): after the
parallel rung, ONE offset `d₁ − (n₁·n₂)·d₂`, and the swing
`‖n₁ × n₂‖`, in the ladder and in `carrier_eq::reading`'s plane arm.
Each normal enters twice, so the offset is zero as an expression
whichever way each normal is spelled (`S·S = 1`); it differs from
today's by `(σ − n₁·n₂)·d₂`, second order in a tilt the parallel rung
has bounded at the region's arm. The orientation rung stays the numeric
decision it is and only labels the verdict. It is read at the region in
use, never at a plane's stored origin: measured, a form reading the
stored origins gives `Undecided` and `Apart` for two planes tilted
`2e-10` that are one plane over the region, once an origin is stated
tens of arms away.

**Measured by a designer's probe** (main `36cc57d5ea`, with the doors
reading the record): all seven `g_ring_closure_measure` rows pass (the
control fails with five unproven seats); two already-unproven rungs of
`two_blocks_resting_on_one_floor…` change their reason from
`OneConstruction` to `Budget`; no proof lost in what was run.

**Done when.**
- The ladder and `carrier_eq::reading`'s plane arm read the one offset
  and the cross-product swing; `reading`'s doc says which displacement
  its bound is of, since at `Interval` it is read as a certified bound.
- Rows: the far-origin pose (an in-band tilt with one plane's origin
  stated far along it: one plane at every distance, at `f64` and
  `Interval`); a rectangle verb's wall beside a table polygon's wall
  over a box at `Sym<Interval>` (red on main if the exposure is real);
  the operand order (the ring rows and the floor row with the ladder's
  operands swapped give the same proven set and rung labels).
- If a rung main proves goes unproven under the larger form, the
  fallback is in scope: the caller hands the ladder a point of each
  plane that lies in the region and is on the plane as a formula. Not
  before a lost proof shows it is needed.
- Every other reader with a decided sign times a normal or a direction
  is found and dispositioned (the designers read `certify.rs`'s
  `r1 − sigma·r2`, `curve_pair::pose::one_line`,
  `topo::injective::one_plane` and `one_cylinder` and found none of the
  shape; the stated-axis arm of the coincidence door was not read).
- `docs/predicate-dimension-audit.md`'s `plane_eq_offset` row says what
  the margin is.

**Letter M; tier dual (sequential at M):** a core predicate every plane
coincidence goes through; a wrong verdict there is wrong topology, and
its conditioning argument is second order. (BOXFINISH orchestrator)
