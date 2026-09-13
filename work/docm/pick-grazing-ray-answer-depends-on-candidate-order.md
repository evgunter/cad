---
id: pick-grazing-ray-answer-depends-on-candidate-order
kind: issue
title: a grazing ray's pick answer is decided by the candidate order, not the geometry
status: open
opened: 2026-09-12
---



## The finding

`pick_face`'s exact test (`ray_triangle`, `crates/editor-core/src/resolve/pick.rs`)
can answer a `t` OUTSIDE the triangle's own box when the ray lies in
the triangle's plane: Möller–Trumbore's determinant is then ~1e-19
and `u`, `v`, `t` are rounding noise that can land inside the closed
acceptance. Seen on the gallery ring at PERF-9's tie-break row (a ray
along +y through a chord point of the tube): the triangle
`patch 0 tri 1` (det 2.1e-19) answers `t = 1.476` for a box whose
conservative entry is `t_enter = 1.480 − 4 ulp`; the true graze is at
the corner, `t = 1.480`.

What decides the pick today is the early-out, by ulp luck: a
neighbouring triangle's hit rounds to `1.480 − 6 ulp`, which is
strictly below the grazed box's `t_enter`, so the loop breaks before
the garbage `t` is tested and the answer is the corner. A reference
over the same tree with no early-out answers the garbage `t` (a point
0.004 below the corner, on a face the ray does not cross there). So
for a grazing ray the answer is a function of the candidate ORDER and
of which neighbour rounds which way, not of the geometry.

PERF-9's two-level index reproduces the single-tree candidate
sequence exactly (`MeshPick::candidates`), so it answers the same as
today, garbage-luck included, and `viewer`'s `index_memo` reference
loop keeps the early-out for that reason. The defect is upstream of
the index: the exact test needs a guard consistent with the pruning —
refuse a hit whose `t` is below its box's `t_enter` (a true hit never
is: the entry is a certified lower bound), or refuse near-parallel
determinants — so the answer becomes a function of per-triangle tests
alone. Either changes today's answers on grazing rays; the pin to
re-baseline is `index_memo`'s reference loop, whose early-out then
becomes redundant rather than load-bearing.
