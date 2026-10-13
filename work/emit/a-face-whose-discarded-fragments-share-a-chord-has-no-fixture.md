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
