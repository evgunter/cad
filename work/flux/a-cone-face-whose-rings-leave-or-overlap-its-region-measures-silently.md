---
id: a-cone-face-whose-rings-leave-or-overlap-its-region-measures-silently
kind: issue
title: A cone face whose rings leave, straddle or overlap its region, or whose loop crosses itself between vertices, measures a wrong area silently
status: open
priority: P2
cost: H
refs: [4696, validate-tier3-curved-boundary-containment]
opened: 2026-10-11
---


## What

`geom_brep::props::curved::cone_face_closed_form` reads a cone face's
area off the vector area of all its loops. It decides that each loop
closes, goes round the axis at most once and meets itself at no vertex,
and that each ring is contractible, wound against the outer loop and,
together, no bigger than it (`HolesExceedFace`). It does **not** decide
that a ring lies inside the outer loop, that rings are disjoint, or that
a loop does not cross itself between vertices. The lane is entered by
any caller of `curved_face_loops`, validated or not, and tier 3's own
containment check is the deferred
`work/restfront/validate-tier3-curved-boundary-containment.md`.

Measured by the PR 4696 dual review's R2 probe
(`review/4696-015BzWT2`, `crates/geom-brep/tests/review_4696_probes.rs`
`probe_rings`), re-run on the fix-pass head. The cone has apex
`(0.2, −0.1, −1)`, axis `(0.3, −0.4, 1)` and α = 0.5, and the outer loop
is the chart rectangle `[0, 3] × [1, 2]` (area 2.15741):

| rings (chart rectangles, each wound against) | true face area | measured |
|---|---|---|
| `[0.2, 2.0]×[1.1, 1.9]` ∪ `[1.0, 2.8]×[1.1, 1.9]` (overlap, sum > the union) | 0.66161 | 0.08630 |
| `[0.2, 1.4]×[1.3, 1.7]` ∪ `[1.0, 2.2]×[1.3, 1.7]` (overlap, sum < region) | 1.58210 | 1.46704 |
| `[4.0, 5.0]×[1.3, 1.7]` (outside the outer loop) | 2.15741 | 1.86976 |
| `[2.5, 3.5]×[1.3, 1.7]` (straddling the outer loop) | — (no face) | 1.86976 |
| `[0.1, 2.0]×[1.05, 1.95]` ∪ `[1.0, 2.9]×[1.05, 1.95]` (sum > outer) | — | refused, `HolesExceedFace` |

A loop that crosses itself at interior points of two edges (no shared
vertex) is the same class: its lobes carry opposite signs and the
vector area is their difference. The sphere's Gauss–Bonnet shares it.

R1 of the same review (`review/4696-GdHo7F`, `overlapping_holes`) found
the same overlap case.

## The question

Whether the closed-form lane should decide containment itself (a
point-in-loop test of each ring against the outer loop on the chart,
pairwise ring disjointness, edge-edge crossing), or leave it to tier 3's
deferred check and say so at the door. (FLUX cone-vector-area lane)
