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
