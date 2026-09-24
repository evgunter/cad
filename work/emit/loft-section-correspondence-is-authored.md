---
id: loft-section-correspondence-is-authored
kind: unit
title: A loft's section correspondence is authored: each loop's orientation canonicalized, its start vertex and the hole order as authored
status: dispatched
opened: 2026-09-23
priority: P0
cost: H
branch: emit/loft-correspondence
refs: [loft-anchors-every-section-with-section-zeros-map, loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body, 3102]
---


## What Ev ratified

This was ratified by Ev on PR 3102's thread (2026-09-23), in three
comments; the reasoning is there.

A loft's correspondence between sections is information that no
per-section canonicalization can recover. The witness is a square
rotated 90°: it is the same point set, but the 0° and 90° twisted lofts
are different solids. The pieces of the correspondence split as
follows:

- **Orientation** is forced by validity: an orientation-reversing
  correspondence sweeps the walls through each other. So each loop's
  traversal is canonicalized, outer counterclockwise and holes
  clockwise.
- **The cyclic offset** comes from each loop's AUTHORED start vertex,
  never from a geometric canonical form. A lex-min start jumps when a
  vertex moves, which would re-twist the loft silently.
- **Hole order** is already the authored input order
  (`crates/profile/src/validate.rs`: "outer first, then holes in
  input order").

Ev ruled out an explicit per-section offset on the loft node as
redundant.

## What changes

`loft_geometry` (`crates/sweep/src/skin.rs`) skins segment k of every
section, where k is counted in canonical traversal from the loop's
authored start. Today it counts from the lex-min start. Consequences:

- Each loft wall's index is the same program step in every section,
  up to a fixed, derivable reflection for a loop authored against its
  canonical sense. So the loft's published names are every section's
  own program numbering, and DM8's ratified wording holds without a
  loft exception.
- The per-section translation machinery built on PR 3102
  (`SectionAnchors`/`Anchoring`) is not needed for the loft. PR 3102
  closes unmerged. Its red rows carry over as this unit's spec: each
  section's step names the wall it swept, including holes, a
  non-identity section 0, and memo re-derivation.
- The twisted-loft finding filed on 3102's branch is absorbed here and
  never needs to reach main as its own row. Its fixture: section 0 is
  the centred square `(-1,-1),(1,-1),(1,1),(-1,1)` at z=0. Section 1
  is the same square rotated +30°, authored as the images in order,
  `(-0.366,-1.366),(1.366,-0.366),(0.366,1.366),(-1.366,0.366)`, at
  z=1. The author means +30°; lex-min pairing builds −60°.
- NOT absorbed: carve's
  `loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body`.
  The first strip becomes the author's choice, but the v-parameters
  are still taken from it alone.

## Done when

- The twisted fixture builds the +30° solid, checked by strut
  endpoints and volume.
- Every section's program step names the wall its own step swept,
  through the ordinary door with the profile's own naming.
- A loft whose sections are authored in the same corresponding order
  as today is geometrically unchanged. Whole-body diffs over the loft
  corpus show this: only lofts whose lex-min starts disagreed move.
- DM8 (`crates/editor-core/REFERENCES.md`) and `loft_geometry`'s
  contract doc state the authored correspondence.
- It lands as an ordinary PR. Ev (3102's thread): an `[ev]` PR is owed
  only if the change meaningfully deviates from what was ratified above.
  Moving published names for non-loft verbs, or a name-bit migration,
  would be such a deviation.

## Ev's second ruling (3102's thread, 2026-09-23)

Measuring before the first commit turned up two things the first
ratification did not settle. Ev ruled on both:

- **The canonical start becomes the authored start globally, not
  only in the loft.** `profile::validate` canonicalizes each loop's
  orientation and keeps its authored start. The lex-min start in V3
  (`crates/profile/README.md`) is retired. Measured cost: no published
  name moves for any verb, and body point sets are identical. Arena
  order moves in 4 corpus documents, one extrude volume moves by
  1 ulp, and about 30 goldens and verdict counts shift. Ev: "same rule
  about never skipping a good change to avoid rebaselining applies".
  Re-baseline all of them and say in the PR what moved.
- **Every verb publishes its profile refs in canonical numbering**
  (orientation-normalized, authored start). The door therefore reads
  each profile's own anchor, with no loft special case, and a section
  authored against section 0's sense needs no extra data. The cost is
  a name migration: extrudes and revolves of clockwise-authored
  profiles renumber their walls s → n−1−s. Ev accepted that.

Neither needs a further `[ev]` round unless the implementation
deviates from these two rulings.
