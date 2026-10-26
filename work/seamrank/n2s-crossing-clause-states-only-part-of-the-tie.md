---
id: n2s-crossing-clause-states-only-part-of-the-tie
kind: issue
title: N2's crossing clause says only an equal pair ties, where rank_crossings also ties an unequal pair whose order it cannot certify
status: closed
opened: 2026-10-07
priority: P3
cost: E
closed: 2026-10-26
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

## Closed (2026-10-26, tracker sweep)

Superseded. The clause this row asked to amend is gone: FORK-ORIENT
(fork-log row 197; Ev, 2026-10-26, "cool, seems good!"; merged as PR
5705, `964083e5`) rewrote N2's crossing paragraph so that nothing is
ranked. `crates/editor-core/src/names/README.md` now reads "Nothing is
ranked: crossings, and vertices of one run, that their names do not
tell apart tie (N4)", so a pair whose order cannot be certified ties by
the rule itself and there is no partial statement left to widen. The
code still ranks (`emit_topo::rank_crossings`); retiring it is the
build of `work/wire/a-unions-line-of-several-edges-is-oriented-by-its-least-edge.md`.
