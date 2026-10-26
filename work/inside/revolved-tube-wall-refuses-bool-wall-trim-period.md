---
id: revolved-tube-wall-refuses-bool-wall-trim-period
kind: issue
title: point_in_solid refuses most probes of a fully revolved tube: bool_wall_trim_period escalates Invalid on a wall face
status: closed
opened: 2026-09-25
priority: P1
cost: D
refs: [3204]
closed: 2026-10-24
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


## CONTACT close-out triage (2026-10-08)

It absorbs the closed duplicate `a-notched-full-turn-wall-has-no-ray-trim`: the same full-turn-wall window guard (`narrower_than_period`, `solid_contain.rs`). Its witness is in `germ_coplanar_conic.rs`, and the fix carries it as a row.

## Met again (CLEAVE `cleave/missed-ray-side`, 2026-10-11)

On that branch a ray that meets nothing has its side read by rays
aimed at planar faces, wherever the volume does not decide it. The
one-segment cylinder of `crates/sweep/tests/a_plane_across_a_one_face_wall.rs`
was cut by the tilted plane `z = 1 + y·tan 20°`, and by the plane
`z = c + m(1 − x)` for `c ∈ {0.02, 1e-3}`.

- Its `∪` results and `B ∖ A` now unite with a far brick.
- The wall pieces, `A ∩ B` (both orders) and `A ∖ B`, still do not, at
  ε 1e-9, 1e-6 and 1e-12.
- Instrumented on the tilted `A ∩ B` at 1e-9, from
  `point_in_solid((50, 50, 50))`: every aimed ray, at the bottom disc
  and at the ellipse cap, is set aside on this row's
  `bool_wall_trim_period` (margin `Invalid`) at the wall face. The
  refusal the caller sees is still the volume's `VolumeUncertified`.

`a_plane_across_a_one_face_wall` pins those three runs as
`Known::Operand` and cites this row.

## Moved on `restread/at-infinity-probe` (RESTREAD, 2026-10-18)

The missed ray's side now falls back to the volume's sign measured
through the quadrature once the closed form and the aimed rays leave it
open (`solid_contain.rs` `missed_side`). The three wall pieces above unite
with the far brick at ε 1e-9, and `a_plane_across_a_one_face_wall` no
longer pins them as `Known::Operand`. The aimed rays still meet this
row's `bool_wall_trim_period` refusal at the wall face; the quadrature
reads the side before that matters. A query whose ray crosses the wall
is not affected and still meets the guard.

## Closed by CHART (2026-10-24)

On branch `chart/ringed-curved-doors`
(`curved-face-containment-answers-none-on-a-ringed-face`), a cylinder wall
is read from its boundary edges (`topo::boolean::ruled_region`), with no
window and no period guard. The repro is now a row,
`sweep/tests/a_plane_across_a_one_face_wall.rs`'s
`a_full_revolved_tube_answers_every_grid_point`: all 729 grid points
answer their truth at ε `1e-9`, `1e-6` and `1e-12`. `germ_coplanar_conic`'s
notched full-turn wall no longer refuses, and its row is strict.
