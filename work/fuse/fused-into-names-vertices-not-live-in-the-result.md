---
id: fused-into-names-vertices-not-live-in-the-result
kind: issue
title: BooleanNaming::fused_into maps a fused vertex to a key that is not a live vertex of the result in about 1 in 8 lattice-brick results that build a body
status: open
opened: 2026-10-06
priority: P1
cost: M
refs: [emit-topo-reads-a-fusion-chain-one-hop, 4116]
---

Found by the review of PR 4116 (ZIP, `zip/survivor-and-recourse`).
Measured by that review; cause unfound.

## The finding

`BooleanNaming::fused_into` (`boolean/ops.rs`) documents each fused
result vertex → "the vertex it finally fused into", read off
`vertex_merges` through `Fusions::survivor`. Over the lattice-brick
corpus, in 1,221 of the 10,288 results that build a body (12,638 Ok,
Empty included) some value of
`fused_into()` is not a live vertex of the result body. The count is
identical on PR 4116's base and head, so it predates the `Fusions`
type; that PR makes every list fold well within itself, which shows
the dead survivor is not a list-order corruption.

`Fusions` sees only its own rows (its doc says so): a survivor can be
dead in the body if something outside the list killed it. Candidates,
none measured: a `kev`/`kef` after the fusions that retires the
survivor without a row (`merge_coplanar_faces` removing a collinear
valence-two vertex, `sweep_and_close`, the pcurve mint), or a fusion
some site makes without appending its row.

## What is owed

Measure: take one failing lattice-brick case, record which vertex the
survivor is and which operator removed it. Then either the remover
appends a row (and `fused_into` is total over live keys, pinned by an
assertion over the corpus) or the doc says what a dead value means.
`discards`' `bordered` ends are read through this map, so a dead
survivor can mis-name a discard's border.

Sibling: `emit-topo-reads-a-fusion-chain-one-hop` (WIRE), the naming
layer's own read of the same rows.
