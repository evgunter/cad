---
id: germ-orders-about-a-normal-have-two-homes
kind: issue
title: The split's germ order and the boolean's pierce/strut orders read one question in two homes
status: open
opened: 2026-10-10
priority: P3
cost: M
---

Found by PR 4565's review (Q1).

## What

Three places order directions in a plane about its normal, each with
its own names and tie handling:

- `splitting/insert.rs` `section_chords`: the split's germs at an ON
  vertex, counterclockwise from the first germ, by `split_germ_side`,
  `split_germ_sense` and `split_germ_order`. Two germs a reading puts on
  one line refuse `SectionGermsUnordered { Coincident }`; the two germs
  either side of one in-plane entry are grouped by the topology and
  never read.
- `boolean/vtxfac.rs` `germ_order`: a pierce's germs, clockwise about the
  pierced face's normal, by `bool_strut_side` and `strut_order`, with a
  germ along the first refused as a `Coincide::Sectors` coincidence.
- `boolean/insert.rs` `strut_order` and `boolean/sectors.rs`
  `direction_sense` (`bool_strut_order`, `bool_dir_same`): the readings
  `germ_order` is built from.

They read one question, the cyclic order of unit directions in a plane,
levered at the shortest reach a reading uses. They differ in sense (CW
against CCW), in the error domain (`BooleanError` against
`SplitReduceError`), in their K names, and in what a tie means to the
caller.

## Why not in PR 4565

Sharing means one door whose ties are the caller's, beside
`null::ring_tree` in `null.rs`, and moving the boolean's two readings
onto it renames K names the nightly's k-lint baselines read
(`bool_strut_side`, `bool_strut_order`, `bool_dir_same`). That is a
re-baseline of its own, on the boolean's ground.

## Done when

One cyclic-order door serves both lanes, ties reported to the caller,
with one set of K names, and the predicate audit's rows for both point
at it.

## A third rule for one tie (2026-10-28, SPLITDOOR `splitdoor/hole-rim-tangent-germs`)

Two germs along one direction now meet three different rules:

- `splitting/insert.rs` `section_chords` orders them one order down
  (`curving`): each face's section curvature off the shared tangent,
  `geom_brep::implicit_graph_acceleration`, differenced and read by
  `geom_brep::enters_material_order2`; an exact zero refuses
  `Coincident`, a singular face `Singular`.
- `boolean/vtxfac.rs` `germ_order` breaks the tie by the partners'
  nesting (`tie_breaks_after`: the germ whose run's other germ lies
  nearer clockwise comes after, so the chords nest).
- `boolean/insert.rs` `strut_order` refuses it (`bool_strut_order`
  through `decide_nonzero`), which a boolean tangent to a hole's rim at
  a vertex reaches
  (`work/splitdoor/a-boolean-whose-face-is-tangent-to-a-holes-rim-at-a-vertex-refuses-its-strut-order.md`).

One door would also have to say which of these a tie means.

## One home for the tie's second order (2026-10-28, SPLITDOOR `splitdoor/boolean-strut-order`)

`geom_brep::tied_germ_turn` now orders two germs along one tangent by
their bends (each section's acceleration off the tangent toward
`n × t`, `implicit_graph_acceleration` in a plane,
`implicit_section_acceleration` in a curved face's tangent plane),
returning the tie (`Zero`) and a singular bend to the caller. The
split's `section_chords` (`curving`) and the pierce ring's
`germ_order` both read it; `insert::strut_order` takes the caller's
tie as a closure, so the rules that remain are the caller's, stated
where they apply:

- one pierce's two germs along one direction (the ring) and two
  sectors' germs (the split): one order down, then refused;
- two pierces' germs along one ray (the ring): the chords' nesting
  (`tie_breaks_after`), unchanged: cones touching along a ray in the
  face have sections tangent to it, which read no bend;
- the orbit walk's two germs in one entry (`walk_faces_first`,
  `walks_after`): still first order alone
  (`work/splitdoor/the-booleans-orbit-walk-orders-two-germs-along-one-direction-at-first-order-alone.md`).

The first-order readings, their K names and the sense difference are
as before.

## The orbit walk and the ring's ties read second order now (2026-11-01, CLEAVE `cleave/two-lumps-pinch-rays`)

The last bullet above is stale since `119e857190` (PR 5812): the orbit
walk's tie of two germs along one direction in one entry, each along
one of the other solid's coincident edges, is read one order further by
`insert::beside_order` (`walks_before_beside`): each germ lies on the
side of the line where the other solid's face flush with the sector
borders its edge (`bool_beside_flush`), read by `walks_after`. That is a
second home for "which cone's copy lies on which side of the ray",
beside `vtxfac::tie_breaks_after` (the ring's two pierces' germs along
one ray, by their chords' nesting). Since PR 6170 the join reads the
ring's tie too: `PierceRingRecord::ties` keeps each tie's two strut
halves in the ring's order, and `join::tie_winding` winds a ring run of
no area by it, so one door would also serve that reader.
