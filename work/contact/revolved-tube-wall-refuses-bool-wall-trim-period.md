---
id: revolved-tube-wall-refuses-bool-wall-trim-period
kind: issue
title: point_in_solid refuses most probes of a fully revolved tube: bool_wall_trim_period escalates Invalid on a wall face
status: open
opened: 2026-09-25
priority: P1
cost: D
refs: [ATREST-9]
---

Measured by an ATREST-9 dual reviewer (PR #3204) and re-measured in
the lane. It is pre-existing: the reviewer found the result identical
with the planar arm's old walk switched back in. A refusal, not a
wrong answer, but it takes `point_in_solid` away from a whole common
body class.

## Repro

A tube: the profile `(0.25, 0) → (0.5, 0) → (0.5, 1) → (0.25, 1)`
revolved fully about the sketch `y` axis (`sweep::revolve`,
`Revolution::Full`). `point_in_solid` at `Band::linear(Tol::witness())`
over a 9³ grid in `[−0.6, 0.6] × [−0.1, 1.1] × [−0.6, 0.6]`:
**567 of 729 probes refuse**. The first payload:

```text
(−0.5507, −0.0453, −0.4280): Escalated { face: FaceKey(5v1),
  diag: Indeterminate { margin: Invalid, band: Band { zero: 1e-9, escalate: 1e-8 },
  predicate: Some("bool_wall_trim_period") } }
```

The reviewer's grid gave 3,458 of 3,926 (88%), at every pose including
identity.

## Not diagnosed here

`bool_wall_trim_period` is the cylinder arm's period guard (the
cosine-window construction: `point_on_wall_in_face` escalates `Invalid`
on a window that is not definitely narrower than a period). A tube's
walls come from a FULL revolve. The guess is that one wall face (or a
closed group of wall faces) spans the whole azimuth and has no
surface-group representative to stand for it, as the cone and sphere
arms have, so the ray lane escalates wherever the wall is crossed.
That the solid cylinder does not show this suggests the bore wall
(reversed sense, no axis vertex) is the difference. It is not
confirmed: read the face at `FaceKey(5v1)` and its azimuth window
first.

