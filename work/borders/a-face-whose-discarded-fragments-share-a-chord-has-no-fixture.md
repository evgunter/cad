---
id: a-face-whose-discarded-fragments-share-a-chord-has-no-fixture
kind: issue
title: No fixture pins the Borders obstacle join of two discarded fragments of one face across a chord
status: open
opened: 2026-10-12
priority: P3
cost: M
---

## What

`borders::Obstacles::walls` joins two discards of one parent into one
obstacle where both bounded an edge that died with them
(`Discard::bounds`, read in `Obstacles::record`). It was added for a
corner held as several vertices, where a corner's discarded region
crosses the edges between its faces
(`a-corner-held-as-several-result-vertices-ties.md`). The same join
applies to a face parent: `by_parent` maps every discarded fragment of
one operand face to that face, so two discarded fragments of one face
across a chord that cut them are now one obstacle. That is N2's
obstacle ("a connected part of the parent's region that no piece
holds"), and before the join they were two.

Measured on PR 4916's review: no join on the face path changed an
obstacle root anywhere in the `editor-core` suite (all 41 that did were
on the corner path). So the face behaviour is untested: no fixture has a
face whose discarded region a chord divides, with pieces on both sides
of that region.

## Fix shape

Build such a fixture (a face cut by a chord inside the region the
boolean discards, for example where an earlier cut's chord crosses a
region a later member discards) and pin the pieces' `Borders` sets.

## Tried, no fixture yet (2026-10-14)

The face path's join needs two discards of one parent that share a
`bounds` key `(record, operand, clone edge)`. That means one operand
face of one boolean holding two discarded fragments across an operand
edge that bounded both. An operand is maximal, so an edge has two
different faces on its sides. That leaves two shapes:
- a chart cut, where the edge divides one operand face into two of its
  pieces (`BooleanNaming::chart_cuts`, a sphere's cut-in meridians);
- two faces in one `Merged` parent across the edge between them.

Tried:
- **A stepped bar across the plate.** Its left part stands in the plate
  and its right part sits flush on the top, so the top region inside the
  bar and the region under its coincident underside meet along the
  step's edge. The kernel discards them as **one** fragment of the top:
  one `DiscardRow`, sharing its seams with the bar's underside. Turning
  the `bounds` join off moved nothing. A boolean does not cut a face
  along a line that has discarded region on both sides.
- **A sunk cylinder boss and slots.** The boss wall's two semicircular
  faces join one parent only where the output stage merges them. A slot
  over the half-seam line leaves each half as one piece, so nothing
  under one parent is divided there.

Left to try: a sphere whose discarded region crosses a cut-in meridian
and divides it, with pieces on both sides of that region. The ball
fixtures (`emit_ball_cut_by_brick`) refuse at a turned pole in the
union today. Avoid that by keeping the poles out of the discarded
region.
