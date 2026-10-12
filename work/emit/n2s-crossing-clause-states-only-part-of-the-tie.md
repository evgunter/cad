---
id: n2s-crossing-clause-states-only-part-of-the-tie
kind: issue
title: N2's crossing clause says only an equal pair ties, where rank_crossings also ties an unequal pair whose order it cannot certify
status: open
opened: 2026-10-07
priority: P3
cost: E
refs: [a-crossing-of-a-nurbs-edge-ties-for-want-of-its-parameter]
---


## What

`crates/editor-core/src/names/README.md` (N2, *Vertices*, line 254)
says crossings of one line by one face with one sense "are ranked along
the line by its carrier's own parameter, in the edge's stored
orientation; an equal pair ties". `emit_topo::rank_crossings` also ties
a pair whose parameters differ but whose order it cannot certify:
- readings the band cannot part;
- pieces whose orientations disagree or cannot be read;
- two crossings on one NURBS piece that `emit_topo::chord_along` does
  not certify (a piece that folds back along its chord, or a closed
  one).

Before PR 4278 every pair on one NURBS piece tied, so the gap predates
that PR. The clause states the rule as if the order were always
available. The README is ratified text, so the wording goes to Ev:
"an equal pair, or one whose order cannot be certified, ties", or
whatever form the ruling takes.
