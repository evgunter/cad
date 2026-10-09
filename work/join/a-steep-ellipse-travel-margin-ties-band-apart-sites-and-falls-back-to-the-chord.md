---
id: a-steep-ellipse-travel-margin-ties-band-apart-sites-and-falls-back-to-the-chord
kind: issue
title: On a steep ellipse the join's travel margin can tie two band-apart sites and fall back to the chord order the margin replaced
status: closed
opened: 2026-10-04
priority: P3
cost: E
refs: [a-pocket-crossing-a-side-face-refuses-at-ring-rehoming-on-a-curved-face, join-ranks-conic-facing-germs-by-chord]
closed: 2026-10-09
branch: join/three-small-join-rows
pr: 4396
---


Found by the fix-pass review on PR 4008 (MINOR 1, inspection, likely).
Unmeasured: no committed battery reaches it.

## What

`crates/topo/src/boolean/join.rs` `nearer_along` ranks one germ's
candidates within a half-turn by `bool_join_arc_travel`, the margin
`turned_past` computes: a candidate's signed distance from the plane
through the conic's axis and the incumbent's site. On a steep ellipse
that distance is not the distance between the two sites. Between the
ellipse's vertices it can be about 5× smaller than the gap between two
sites at aspect k = 10. Two sites a few bands apart can then decide
`Zero`, and `nearer_along` falls through to `nearer`, which is the
chord order, the order that is not monotone on that ellipse.

So the claim that the margin "resolves two sites as finely as the
distance between them does" holds on a circle, not on a steep ellipse.
The defect is at band scale only.

## Done when

A pose with two sites a few bands apart on a k ≥ 6 ellipse is a row.
It either builds sound, or `nearer_along` refuses typed rather than
falling back to the chord. One way to get there: scale the margin by
the conic's local radius so it reads arc length, or make an in-band
travel tie escalate.

## Built

`join.rs` `turned_past` reads the arc: `n·(p − site) / |n·t̂|`, with
`n = axis × radial` and `t̂` the conic's unit tangent at the site (the
germ direction recorded there, carried on `Turn`). The plane distance
over `sin ψ` is the arc to first order, `s (1 + ½ s κ cot ψ)`, and
reading from the site leaves nothing to cancel near it. `|n·t̂|` is the
site's rotational sense, which `germs_face_each_other` has already
decided nonzero. `nearer_along` (`bool_join_arc_travel`) and `germ_arm`
(`bool_join_arc_ahead`) both read it.

The row is
`crates/sweep/tests/a_steep_ellipse_orders_band_apart_sites_along_its_arc.rs`:
a cylinder leaned to `k = 10` and `k = 60` through a block whose top
face has a notch and a finger `12ε` wide on the ellipse's flank, every
op in both orders.

## Measured

At ε 1e-9, 1e-6 and 1e-12 (the pose is sized in bands, so the readings
are the same in bands at each):

- `k = 10`: on main every run escalates at `bool_join_arc_travel`,
  margin −4.78 bands (`24 bands × sin ψ`, 0.198). With the fix the
  travel decides, and every run stops at the next reading,
  `bool_join_nearest`. That is the order between two pairs on the
  notch's two parallel walls, whose chords differ by 1.65 bands. The
  evidence is added to
  `a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale`.
- `k = 60`: the old margin tied `Zero` (0.8 bands). On main and with
  the fix alike, every run refuses at the certifier's span check,
  `IntervalNotForward`. That check meters the finger's `12ε` section
  edge at the ellipse's minor semi-axis, `12ε·√2/60`. Filed as
  `work/issues/an-ellipse-span-is-metered-at-its-minor-axis-and-refuses-a-flank-edge-k-times-longer-than-the-band.md`.

So the chord fallback never shipped a body: while that meter holds, an
in-face edge between two tied partners cannot certify. The fix removes
that dependence. The row asserts every run builds `SOUND` or stops at
one of those two filed doors, and never at `bool_join_arc_travel`; on
main the `k = 10` runs fail it.
