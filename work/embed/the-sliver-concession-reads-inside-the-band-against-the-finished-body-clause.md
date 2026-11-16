---
id: the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause
kind: issue
title: The injectivity check's sliver concession reads a strip inside the band as holding nothing, where DESIGN's finished-body clause says a reading inside the band refuses as ambiguous
status: open
opened: 2026-11-13
priority: P1
cost: M
design: true
---



## Finding

From CLEAR's lane clear-vacuous, while it traced
`a-half-sphere-reads-vacuous-in-its-own-window-so-a-departure-pair-certifies-on-a-zero-width-rectangle`
(PR 6901).

`topo::injective`'s module docs list the sliver among the engine's
concessions:
- a presentation rectangle whose width across a shared line decides
  `Zero` at `SELF_INTERSECTION_SLIVER` reads as holding nothing off the
  line;
- the cell's face is then `Presentation::Vacuous`;
- the departure arm certifies a pair outright when either face is
  vacuous.

Three readers apply it: `present_chart`, `present_line_corner`, and,
since PR 6894, `present_plane_corner`. 6894's lane measured that
`chute` needs it.

DESIGN.md's finished-body clause (`[topology.tiers]`) says: "A reading
inside the band refuses as ambiguous, never as Holds or Violated." The
sliver is a reading inside the band whose consequence is a certified
pair, so on its face it contradicts that sentence.

There may be a reading under which both hold. The dropped points lie
within the band of a shared edge, so a crossing they could hide is
within K·ε of the boundary, and the same clause holds that apart only
"by more than the band". Nobody has argued that, or checked it against
the clause's provenance. The clause came from Ev's embedding ruling,
`31091b6747`.

## What is needed

Decide whether the sliver is consistent with the clause.
- **If it is,** argue it in the module docs and cite the clause. This
  is a faithful elaboration.
- **If it is not,** this is a design fork. The cost is the bodies that
  hold only through it: measure them, starting with `chute` (PR 6894),
  `present_line_corner`'s pins, and the tour walk.

Until this is decided, no new reader adopts the sliver.
