---
id: a-steep-ellipse-travel-margin-ties-band-apart-sites-and-falls-back-to-the-chord
kind: issue
title: On a steep ellipse the join's travel margin can tie two band-apart sites and fall back to the chord order the margin replaced
status: dispatched
opened: 2026-10-04
priority: P3
cost: E
refs: [a-pocket-crossing-a-side-face-refuses-at-ring-rehoming-on-a-curved-face, join-ranks-conic-facing-germs-by-chord]
branch: join/three-small-join-rows
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
