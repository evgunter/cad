---
id: pick-refuses-a-crossing-within-rounding-of-a-plane
kind: issue
title: the certified determinant refuses a crossing whose ray lies within rounding of the triangle's plane
status: open
opened: 2026-09-16
priority: P0
cost: H
---


## The finding

`ray_triangle` (`crates/editor-core/src/resolve/pick.rs`) refuses a
candidate whose Möller–Trumbore determinant does not exceed the
forward rounding-error bound of its own evaluation
(`certified_determinant`, `3 · EPSILON · Σ|e1_i|·S_i`). That is the
class the mechanism exists to refuse — a ray in or within rounding of
the triangle's plane, where the barycentrics would be rounding noise
over rounding noise — and it is also, by construction, a refusal of
any GENUINE crossing whose ray/plane angle is below the bound: the
crossing exists in exact arithmetic over the mesh's rounded corners,
and the test cannot tell where on the triangle it is.

Measured at the fix pass of
`pick-grazing-ray-answer-depends-on-candidate-order`
(`crates/viewer/tests/review_pick_r2.rs`, the wide aim over the corpus
at open and after the first edit: 441 126 rays): 20 016 candidates on
10 536 rays refused at the determinant, the best-conditioned of them
at `|det| / (|e1|·|e2|·|d|) = 4.8e-16` — about two units of roundoff
of angle. No candidate at or above `1e-12` is refused (the row
asserts it). On the aimed rays the refused candidates are axis rays
in, or within rounding of, the plane of a planar face's triangle
through a vertex or edge the neighbouring, non-coplanar triangles
answer; the answered-at-the-vertex count (141 094 of the 441 126) is
pinned by the row.

The residue: a ray whose angle to a triangle's plane is genuinely
below ~2u and which crosses that triangle in its open interior, where
no neighbour shares the point, answers the next face behind or a
miss. No corpus ray of either aim is in that class; a synthetic one
is a ray at angle 1e-17 through the middle of an isolated triangle.
The shape of a fix, if the class is ever worth its cost, is an exact
or extended-precision fallback for the uncertified candidates only —
never a widened bound, which moves the tie behaviour the
`index_memo` rows pin.
