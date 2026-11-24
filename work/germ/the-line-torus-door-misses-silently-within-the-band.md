---
id: the-line-torus-door-misses-silently-within-the-band
kind: issue
title: The line × torus root door answers a silent Miss on a line passing within the zero band of the torus: its count-0 Miss is decided on the quartic's discriminant, not on a clearance
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [a-torus-touch-at-a-definite-non-elliptic-point-refuses-r-tan, line-torus-roots-may-certify-noise-when-the-line-origin-is-far]
---


Found by ORBIT's torus-touch build (branch `orbit/torus-touch-build`),
auditing the root doors the section certificate's touch leans on.

## What the touch needs of a door

The touch (`boolean::section_cert`, "A touch") clears a tangency on a
silent pair when one point of its locus places `Out` of a face. Its
argument needs the sweep to record or refuse every edge that comes
within about `2m` of the other carrier over the other face (`m` the
touch's decided margin, `2m ≤ 4ε`): premise S′. A door that answers
`Miss` (no event) on an edge that close breaks it. A door is safe when
its silent answer is a clearance decided past the band's escalation
`Kε`.

## Measured (ε = 1e-9, K = 10; donut `R = 2`, `r = 0.5` about y)

Lines passing a gap `g` above the outer equator's top `(0, 0, 2.5)`,
read by `geom_brep::curve_face::line_quadric::line_quadric_roots`:

| direction | g = 0 | 1e-11 | 5e-10 | 2e-9 | 4e-9 | 9e-9 | 2.4e-8 |
|---|---|---|---|---|---|---|---|
| along the axis (y) | Tangency | **Miss** | **Miss** | **Miss** | **Miss** | Miss | Miss |
| in the midplane (x) | Tangency | Tangency | escalates | escalates | **Miss** | Miss | Miss |
| generic (1, 0.6, 0) | Tangency | Tangency | **Miss** | **Miss** | **Miss** | Miss | Miss |

The count-0 `Miss` is read off `depressed_quartic_roots`'s sign algebra:
`Δ` decided `Positive` and `p` or `D` positive. `Δ/lever¹¹` scales as
the gap times the product of the other root separations, which for an
axis-parallel line is about `2300·g` here, so it is decided at a gap of
`ε/100`. It is a decision about the count, never a length of clearance.

A short segment 4ε to 9ε inside the solid below the top answers two
certified roots past its span, which `reduce.rs`'s both-ends-inside
torus arm reads as no event (`NoInterior`).

Every other door the touch leans on holds: circle × torus, circle ×
sphere, circle × wall and circle × plane answer a tangency within ε,
escalate in the sliver and miss only past `Kε` (2.4e-8 here); line ×
wall and line × sphere (the belly branch's doors) likewise, their `Miss`
a discriminant decided as a depth in metres.

## Why it matters

The merged elliptic torus touch already leans on S′ along the line ×
torus door (a brick's edge beside an off-face plane touch), and the
near-band touch leans on it more widely: at a saddle the carriers cross
along the touch's lobes, so the edge that parts a section component
from the touch's point may stand just outside the solid, where this door
goes silent.

## The fix it wants

The line × torus door's silent answers decided as a clearance in
metres: a `Miss` or a `NoInterior` that stands as a decided gap
between the segment and the tube past `Kε` (for example the residual's
extreme over the span, read as a length), with the count-0 sign algebra
kept for counting only.
