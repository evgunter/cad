---
id: cone-side-is-a-second-reader-of-a-directions-side-of-a-polygon-cone
kind: issue
title: boolean::sectors::cone_side reads a direction's side of a polygon cone beside the census's link reader, levered where the census decides in metres
status: open
opened: 2026-10-19
priority: P3
cost: M
---

Filed by RESTREAD's link-reading lane (branch `restread/link-touch-reading`),
as the decision on
`work/restread/a-pinch-point-the-census-reads-as-mixed-touch-where-the-body-holds-no-overlap.md`
asks ("one reader, through the census's door").

## What

Two readers answer "is this direction inside this polygon cone's
material" by the parity of a great arc's crossings of the cone's face
sectors:

- `crates/topo/src/link.rs` `Link::reads` (the census's touch
  analysis): a cone given as faces with real points (each face's piece
  at the vertex, a fan of wedges), every sign a point's distance from a
  plane in metres through `link::metric::Distance`
  (`link::tests::the_link_decides_only_through_its_doors`).
  The reference is a wedge's centroid; an arc whose reading is undecided
  is passed over for the next reference.
- `crates/topo/src/boolean/sectors.rs` `cone_side`: a cone given as
  `BoolSector`s (unit bounds with reaches), every sign a levered
  deviation (`least_lever`), with its own `apart` escape.

`cone_side` was not lifted into the census as it stands: its readings
are levers of directions, which the census's door rule forbids, and its
input is the boolean's sectors, not faces. Moving the boolean's side
onto the census reader did not fit the unit that made it.

## Evidence that one can serve both

`crates/topo/src/boolean/sectors/cone_fuzz.rs` now reads the census
reader (`link::bare_cone_reads`, test-only) beside `cone_side` and
`wedge_classes` on every cone and probe of its families (stars, thin
fins and slivers, turned stars, graze, bound, long-probe darts), under
the same exact oracle and the same rule that a class decided in band is
wrong. Its counts print per ε in the fuzz's `[fuzz]` line.

## Owed

Retire one reader. Either the boolean's sectors call `census::Link`'s
reader (its sectors carry their far points: `Reach::Chord`), or the
census's reader moves to a home both call. Related:
`work/twice/great-arc-parity-is-read-twice-with-two-degeneracy-rules.md`
(the same parity read a third time, by `ring_path::path_parity`).
