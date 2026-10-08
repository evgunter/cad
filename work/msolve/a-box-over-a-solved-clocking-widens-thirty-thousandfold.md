---
id: a-box-over-a-solved-clocking-widens-thirty-thousandfold
kind: issue
title: An Interval box over a solved clocking widens the membership margin about 3e4 times the box, so no practical box decides it
status: parked
priority: P3
cost: M
opened: 2026-10-03
blocked_on: [intent-stage3-is-built]
refs: [MSOLVE-14, an-identically-zero-margin-escalates-at-a-fine-eps]
---

## Finding

Found by MSOLVE-14's review (PR 3986) and measured in its fix pass. The
corpus's two-pin document (two coaxial pins on parallel axes and a
planar rest), with the second slab pin's frame turned about its own axis
by a parameter `θ` (`MateFrame::on_part` of a rigid step, angle `θ`,
nominal 0). Turning an unclocked coaxial frame about its own axis moves
nothing, so the true pose does not depend on `θ` at all. An `Interval`
solve over a box `±w` on `θ` (`mate::solve_document_at`):

| `w` | First undecided predicate | Margin `hi` | `hi / w` |
|---|---|---|---|
| 1e-1 | `mate_axis_point_offset` | 5.66 | 57 |
| 1e-3 | `mate_member_axis_fixed` | 27.4 | 2.7e4 |
| 1e-5 | `mate_member_axis_fixed` | 0.310 | 3.1e4 |
| 1e-7 | `mate_member_axis_fixed` | 3.10e-3 | 3.1e4 |
| 1e-9 | `mate_member_axis_fixed` | 3.10e-5 | 3.1e4 |
| 1e-11 | `mate_member_axis_fixed` | 3.10e-7 | 3.1e4 |

At ε = 1e-9 the margin decides only once `w < 1e-9 / 3.1e4 ≈ 3e-14`.

## Cause (measured)

Not a wrapping defect: the angle never crosses `atan2`'s cut here. It is
interval dependency, compounded:

1. **The adjugate inverse.** `mate_coset`'s `target * fb.inverse()`,
   `invert`'s `representative.inverse()`, and `clocking_about`'s
   `held * added.inverse()` all invert through `Mat3::inverse` (the
   adjugate over the determinant, `geom-core`). For a rotation that is
   the transpose, but an enclosure loses the correlation between the
   entries: an entry exactly 1 (`c2.z`) came out 7.7e-5 wide at
   `w = 1e-5`, and two inversions deep the clocking's moved axis point
   `w` was 5e-3 wide (about 500 `w`). Replacing every solve-side inverse
   by the rigid one (transpose, then `−Rᵀt`) cut `hi / w` from 3.1e4 to
   5.8e3, measured.
2. **Re-measuring a residual met by construction.** What is left is the
   general limit in `an-identically-zero-margin-escalates-at-a-fine-eps`:
   the solved angle carries `θ`'s width, the membership check composes
   it with `θ`'s frame again, and the residual that is zero for every
   `θ` encloses at the box's width times the lever.

## Why it is not fixed in MSOLVE-14

A rigid inverse at `Interval` is a `geom-core` linalg door (an
`Affine3::rigid_inverse` that certifies the orthogonality defect of a
literal frame, so the enclosure stays sound for inputs orthonormal only
to rounding), outside that unit's fence; and it buys a factor of five,
not convergence. The rest is the §5 (b) design question.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage3-is-built`, not on the whole program: the widening comes from member_of re-measuring a residual met by construction; stage 3's "a mate places and never checks", overconstraint by subgroup algebra without measuring, retires that re-measure. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)
