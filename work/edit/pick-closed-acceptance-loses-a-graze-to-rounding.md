---
id: pick-closed-acceptance-loses-a-graze-to-rounding
kind: issue
title: the exact ray/triangle test loses a vertex or edge graze to rounding in u and v on near-tangent hits
status: open
opened: 2026-09-16
---


## The finding

`ray_triangle` (`crates/editor-core/src/resolve/pick.rs`) documents
its closed boundaries — `u ∈ [0, 1]`, `v ≥ 0`, `u + v ≤ 1` — as the
reason "watertight meshes never lose a graze to an open boundary".
The comparisons are on rounded values, and a graze whose test is
ill-conditioned (a near-tangent hit, determinant ~1e-5 on a
unit-scale mesh) rounds its `u` or `v` a few 1e-14 across the
boundary on EVERY triangle sharing the point, so the graze is lost
to all of them and the ray falls through to whatever is behind, or
to a miss.

Measured at the guard unit (`pick-grazing-ray-answer-depends-on-candidate-order`,
its PR), over every landing of `viewer`'s `index_memo` corpus and the
gallery ring: of the tie-break row's aimed rays (`tie_rays_for`, each
aimed at a mesh vertex or an edge midpoint at distance `reach` along
an axis), 273 answer BEYOND the aimed point or miss — identically
before and after the box-entry guard, so this is the closed
acceptance's own class, not the guard's. One diagnosed: the gallery
ring at open, tie rays 32 and 33 (a +y/−y pair through the midpoint
of a boundary segment shared by patch 2 tri 72 and patch 3 tri 88):
both incident triangles compute `det ≈ ±2.66e-5`, `u = 0.5`,
`v = −1.03e-14`, `t = 1.48` — the aimed point exactly, refused by
`v ≥ 0` on both — and the pick answers the typed miss through a ring
the ray touches.

The tie-break row does not see this: it counts ties where they occur
and asserts nothing about a ray that misses its aimed point. A row
that would (every aimed ray answers at or before `reach`) is red on
`main` for these 273, which is why the guard unit did not add it.

The shape of a fix is a question for the row's owner: a rounding-aware
acceptance (a derived bound on the error of `u` and `v` in terms of
the operands and machine epsilon, never a tuned constant), or a
vertex/edge-aware fallback in the loop. Either changes the answers
the `index_memo` differential pins, and its tie floor.

## Dropped from EDIT-PICK2's carry (2026-09-16)

This row does NOT close with
`pick-accepts-uncertified-barycentrics-on-a-certified-determinant`,
and its `rides_with` is cleared. EDIT-PICK2 ruled, built and measured
the acceptance that would have closed it — MEET, admitting a
barycentric whose interval reaches back into `[0, 1]` — and MEET
trades order independence: it admits a value outside the range, the
hit point `a + u·e1 + v·e2` then leaves the closed triangle, and the
caller's early-out stops before a candidate that would have won
(`work/edit/pick-hit-point-from-an-out-of-range-barycentric-leaves-the-triangle`,
measured at 2 rays of 19 296 on the tie-break aim; MEET's gain was 20
of 149 aimed rays recovered). The unit landed the half that does not
trade it — the closed comparison ∧ INFORM — and this row waits on the
same ruling as
`pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`:
what the door answers for `t`, and with what width, once a candidate
is admitted at all.
