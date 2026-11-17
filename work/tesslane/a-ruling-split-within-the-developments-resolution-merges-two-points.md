---
id: a-ruling-split-within-the-developments-resolution-merges-two-points
kind: issue
title: a cone face with two points of one ruling closer than its development resolves refuses DevelopedMerge
status: open
opened: 2026-11-17
priority: P1
cost: M
---

Found by PR 7049's review (`a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it`); ruled a residual of that PR (TESSLANE, 2026-11-17), then raised to P1 when the review showed it at the default ε.

## Finding

A cone face on the chart lane that holds two points of one ruling closer
together than its development resolves refuses
`SphereTrimShape { DevelopedMerge }` at every δ. The class's faces reach
it whenever a ruling is split close to one of its ends, or split twice
close together anywhere along it, and at the default ε on a cone of small
half-angle:

| half-angle α | `2⁻²⁷·r / sin α` (r = 2) | merges at a gap of | meshes at a gap of |
|---|---|---|---|
| 0.01° | 8.5·10⁻⁵ | 3·10⁻⁵ | 10⁻⁴ |
| 0.1° | 8.5·10⁻⁶ | 10⁻⁵ | 3·10⁻⁵ |
| 1° | 8.5·10⁻⁷ | 3·10⁻⁷ | 10⁻⁶ |
| 3° | 2.8·10⁻⁷ | 10⁻⁷ | 3·10⁻⁷ |
| 26.6° | 3.3·10⁻⁸ | 1.1·10⁻⁸ (two splits mid-ruling) | 3.4·10⁻⁸ |

Measured 2026-11-17 on the wall `(1, 0), (2 + tan α, 0), (2, 1), (1, 1)`
revolved a full turn about `y` and placed obliquely, its cone face's seam
split at the gap from its start or twice the gap apart at its middle (the
two answer alike), δ 0.01, default ε; gaps on a 1–3 grid, so the bounds
are to that grid. The largest merging gap is 0.34 to 1.2 of
`2⁻²⁷·r / sin α`, as the cut's power-of-two bins put it. The review
measured 408 such cells at the default ε that mesh on the merge base,
over the wall and the solid `(0, 0), (2 + tan α, 0), (2, 1), (0, 1)`,
full and half turns, δ 0.1, 0.01 and 0.001; and 168 more at ε `10⁻¹²`
with a split `10⁻⁹` from an end on frustums, frustum walls and diamonds.
Every one of them meshed on the base, where the rectangle lane took the
face.

## Mechanism

The face is the chart lane's (`walk::splits_a_straight_side`), whose cone
arm triangulates in the development (`trimmed::developed`). Its slant
radius `r / sin α` is cut to 27 significant bits and its direction to 26,
so every product is exact and a ruling's points lie on one exact line
through the apex, which is the rule that keeps a triangle off one ruling.
Two points of one ruling whose slant radii fall in one 27-bit bin develop
to one point, and `trimmed::developed_merge` refuses the face, since the
CDT would hold two mesh vertices as one. The bin is a length along the
ruling of up to `2⁻²⁷·r / sin α`, which grows as the cone flattens to a
cylinder: about `8.5·10⁻⁵` at α = 0.01° and radius 2.

Pinned: `split_rulings::a_draft_cone_ruling_split_within_the_developments_resolution_refuses`
(default ε) and
`split_rulings::a_ruling_split_within_the_developments_resolution_refuses`
(ε `10⁻¹²`).

## The cure

Its cure is specified in
`work/tesslane/a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation.md`.
The strip unit (`a-ruled-face-is-meshed-as-a-strip`, parked on Ev's
answer) would take these faces out of the development altogether.
