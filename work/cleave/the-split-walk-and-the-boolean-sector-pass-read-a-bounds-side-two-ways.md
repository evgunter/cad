---
id: the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways
kind: issue
title: entry_side and sectors::side_code read one bound's side of a plane two ways, and disagree at every first-order tie
status: open
opened: 2026-10-13
priority: P3
cost: M
---


Found by the review of PR 4858 (branch `cleave/vertex-against-section`).

## What

Two readers decide which side of a plane one bound of a vertex's
sector leaves on:

- `splitting::entry_side`: the split's neighbourhood walk, which
  `vertex_section` also reads at both doors. A conic reads
  `split_conic_departure` and, on a first-order tie, falls through to
  `geom_brep::conic_bend_side` (second order).
- `boolean::sectors::side_code`: the Boolean's own sector pass. It
  levers the bound at its `Reach` with a sagitta charge, and a tie stays
  On or refuses.

So at every first-order tie they disagree by construction. On the poses
PR 4858 measured, the disagreement changed no outcome. But the Boolean
decides which faces a section enters at a vertex
(`vertex_section`) by one reader, and builds its sectors by the other.

## Owed

One reading of a bound's side, read by both: either `side_code` takes
`entry_side`'s second-order fall-through, or the reverse, decided on
which one certifies what it claims.
