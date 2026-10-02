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

Reachable, measured in the PR 3770 review (2026-10-02). The run needs
a Below bisector between Above real bounds. A reflex sector supplies
one: the 315° prism of `review_m3_pr55::a_reflex_315_corner_tilted_cap`
(`prism_z` over `(0,0) (2,2) (-2,2) (-2,-2) (2,-2) (2,0)`, z∈[0,1]),
split by a plane through its reflex top corner `(0, 0, 1)` with normal
along `(1, 0.2, -1)`, `(1, 0.5, -0.5)` or `(1, 0, -1)`. All three edges
of the corner read Above and the top cap's reflex bisector Below, so
the run holds the whole orbit. Each split refuses
`Join(UnpairedLooseEnds { count: 2 })`. That is typed, not silent. The
mirrored normal `(-1, -0.2, 1)` puts the run on the other side and
splits cleanly (6 + 8).

With the strut recorded the other way round as a throwaway patch
(`mev_null(site, Below)`, `below_end: copy`, `above_end: vertex`),
all three splits succeed. Their parts pass tiers 2 and 3 and their
volumes sum to 14: 8 + 6, 7 + 7, and 8 + 6. The `(1, 0, -1)` below
part, 6, is checked by hand: ∫₀¹ 4(z + 1) dz. The rest of the topo
suite stays green under that patch, and no existing row reaches this
arm.

## The shape to give

Record the strut with the side attribute swapped (the strut tip is the
Below copy), and pin the three splits above as rows.
