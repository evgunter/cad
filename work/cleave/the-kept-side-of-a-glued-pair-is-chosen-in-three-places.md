---
id: the-kept-side-of-a-glued-pair-is-chosen-in-three-places
kind: issue
title: Which description a glued pair keeps is chosen in three places: the zip keeps A's vertex, the merge keeps the arena-first face except where it plugs a hole, and the glue decides nothing
status: closed
opened: 2026-10-27
closed: 2026-10-31
priority: P1
cost: M
refs: [a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop]
branch: cleave/kept-side-in-one-place
pr: 5848
---


Ruled by `[ev]` PR 5780 (2026-10-27). DESIGN.md's canonical-places
table now lists "Which description a glued pair keeps | decided once,
at the glue | [coincidence.booleans]", and `[coincidence.booleans]`
names the rule: the first-written operand's description is kept, a
fixed rule that reads no values.

## What

Today the choice is made in more than one place, and they can
disagree:

- **The zip** keeps A's vertex at a fused seam joint
  (`crates/topo/src/boolean/zip.rs`, the comment "The kept vertex is
  `ob`'s at its start, its successor's at its end: A's").
- **The merge** keeps the group's arena-first face that lies in no
  other member's hole (`crates/topo/src/merge_faces.rs`
  `outermost_survivor`, documented on `MergedGroup::kept`). In a
  Boolean's result that is ordinarily operand A's, and B's where A's
  face plugs a hole of B's. In that case the result holds B's carrier
  with A's vertices.
- **The glue door** decides the pair Zero, but records no kept side
  for the zip or the merge to read.

## Owed

The glue decides the kept side once per pair it decides Zero, the
first-written operand's, and the zip and the merge read that decision.
The merge's hole exception goes, so a plugged hole keeps A's
description like every other glued pair.

Not owed, by the same ruling: re-deriving the other face's boundary
on the kept carrier. Ev, on PR 5780: "it definitely is tidier but
strays more from the original".

## Closed

2026-10-31, PR 5848. `boolean::glue::kept_first` states the decision
once: a glued pair's sides as `(kept, other)`, the first-written
operand's first. The zip reads it for the vertex a fused joint keeps
and the edge a seam pair keeps; the boolean records each glued surface
pair kept side first (`ops::GluedSurfaces::of_result`,
`GluedSurfacePair::{kept, other}`); and the merge (`merge_faces.rs`
`kept_description`, `wear`) re-charts the group's survivor onto the
description the glued pairs record first, the arena-first member's
where nothing orders the group. The survivor is still the outermost
member, a surgery matter only: where A's face plugs a hole of B's, B's
host face survives wearing A's description. `contact8_dangling_seam`
`a_plug_folded_first_merges_into_the_face_it_plugs` pins the plug ∪
holed block caps on the plug's surface keys, green at tiers 2, 3, 3′
and its volume. Two further places the sweep found are filed:
`the-lump-keeps-a-coincident-regions-copy-by-its-own-table` and
`a-zero-vertex-pair-keeps-its-smallest-point-key`.
