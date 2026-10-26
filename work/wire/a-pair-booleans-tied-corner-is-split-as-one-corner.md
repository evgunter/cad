---
id: a-pair-booleans-tied-corner-is-split-as-one-corner
kind: issue
title: The pair boolean groups an operand corner's copies by name, so a tied corner's candidates are split as one corner
status: open
opened: 2026-10-25
priority: P2
cost: M
---


Filed by WIRE's `wire-bridges` lane (PR 5535) from reading the code. It
has not been measured: no fixture with a tied operand corner held as
several vertices has been built yet.

## What

`emit_topo::name_boolean_vertices` keys `corners` by the corner's
NAME (`corners.entry(name)`). A tied operand vertex's candidates
share one name, so every result vertex at either candidate lands in
one entry, and the corner pass hands them all to
`Obstacles::split_corner` as copies of one corner. Two cases follow:

- **Both candidates survive whole.** No obstacle borders two of the
  copies, so `Obstacles::walls` refuses ("a copy of a corner held as
  several meets no recorded discard between them"). N4 wants the bare
  tied name instead.
- **One candidate is divided.** The undivided candidate's copy has
  empty walls, and the same refusal fires. N4 says: "the divided
  candidate's pieces are named by their `Borders`, the undivided
  candidates stay under the bare name".

The union's twin, `emit_union::name_corner_copies`, keys a corner by the
member corners it sits at, so each candidate is its own corner.

## What it needs

- A fixture: an operand whose table ties a vertex, met by a boolean
  that holds one candidate as several vertices, and one that leaves
  both whole.
- Then key the pair's corner pass by the operand vertex (the candidate)
  rather than the name, as the union does, sharing the body with
  `name_corner_copies` where that is possible.
- The union side has no witness either: the same fixture, met through a
  union, would be one.
