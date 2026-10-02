---
id: split-whole-orbit-run-mints-an-unlabelled-strut
kind: issue
title: The splitter's null-edge insertion turns an Above run holding every real edge of the orbit into a strut and records it as a fan split
status: open
opened: 2026-10-02
---

## What

Found by the sweep of the boolean pierce-insertion fix for
`join-desync-on-the-star-fixture` (branch `join/star-desync`).

`insert_null_edges` (`crates/topo/src/splitting/insert.rs`) computes
the fan end of a run with real edges as `he2 = next(mate(last.he))`.
When the Above run holds every real edge of the vertex orbit (the
Below side is only a bisector entry inside one physical sector), `he2`
comes back round to `first.he`. `MevSite::Fan { he1, he2 }` with
`he1 == he2` is an empty run (`crates/topo/src/euler.rs`, `MevSite`
docs), so `mev_null` builds a strut spliced before `first.he` instead
of moving the run. The record still says `dangling: false` and
`below_end: vertex`, but `vertex` holds every Above edge and the new
vertex holds none.

The boolean's two insertion sites handle this case. The vertex–vertex
path (`boolean/insert.rs`, `mint_run`) refuses it typed ("null-edge
run spans the entire vertex orbit") after `run_degenerates` has tried
the other direction. The pierce path (`boolean/vtxfac.rs`,
`classify_vertex_on_face`) records it as the dangling strut it is,
after `join/star-desync`.

## Reachability

Unmeasured. For a split, the run needs a Below bisector between Above
real bounds, which takes a reflex sector whose bisector dips below
the plane, or On edges that resolve Above around a flat sector. No
fixture has been built for either.

## The shape to give

Measure first: build a split whose vertex has a reflex or flat sector
of that kind. If the case is reachable, either refuse it typed as the
VV boolean path does, or record the strut with the side attribute
swapped (the strut tip is the Below copy).
