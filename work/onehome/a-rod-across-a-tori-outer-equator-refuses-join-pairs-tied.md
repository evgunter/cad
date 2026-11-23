---
id: a-rod-across-a-tori-outer-equator-refuses-join-pairs-tied
kind: issue
title: torus_doors_join's rod across the outer equator refuses JoinPairsTied in the slow set
status: open
opened: 2026-11-18
---


`sweep::all torus_doors_join::a_rod_across_the_outer_equator_builds_at_its_volume`
(`crates/sweep/tests/torus_doors_join.rs:153`) subtracts a rod along `x`
through `y = 2.45`, radius 0.1, from the donut. It refuses at the row's
`unwrap_or_else` (`torus_doors_join.rs:83`) with `refused: JoinPairsTied`.

Seen in the nightly slow set (`--ignore-default-filter -E 'not default()'`)
on PR 7088's heads `aafba8b28` and `0e4a88b3b`, in local runs, 4.6 s each.
Those heads change no Boolean or join code, but this isn't checked against
a clean main. `work/embed/log.md:55` saw it on EMBED's instrumented build
and named it on no row. GREEN's
`main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps.md`
counts "`torus_doors_join` ×3", and its per-row list doesn't name this
row.

Closing it means reproducing it on main. Where it is red there, bisect it
and decide whether the tie the join refuses is a real tie at this pose.
