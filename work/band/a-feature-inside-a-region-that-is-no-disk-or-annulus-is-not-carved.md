---
id: a-feature-inside-a-region-that-is-no-disk-or-annulus-is-not-carved
kind: issue
title: blend: a feature wholly inside a straight band's region that is neither a disk on one cap nor an annulus between two (a cup with a handle, a U-tube) refuses WhollyInside, though S − R / S ∪ F takes it
status: open
opened: 2026-10-14
priority: P3
cost: M
---


## Finding

Ev's band ruling (PR 4963) makes a blend `S − R` (convex) or `S ∪ F`
(concave), so any feature wholly inside a straight band's region goes
with it. The carve builds two shapes only: `consume::plan` admits a
feature that is a disk on one cap (`V − E + F − R = 1`) or an annulus
between the two caps of one band (`= 0`), and refuses every other
`WhollyInside` before any mutation. Rows in
`crates/sweep/tests/band_one_cycle_members.rs`,
`a_cup_with_a_handle_and_a_u_tube_refuse_wholly_inside`, pin three that
the rule takes:

- a square blind bore with a square cross bar spanning it (one mouth,
  genus 1, count `−1`), under both verbs;
- a square pin with a square hole across it, under the chamfer (under
  the fillet its mouth crosses the arc, a measured `RingClearance`);
- two square bores from one end face meeting inside (a U-tube: two
  mouths on one cap), under the chamfer.

Each is read wholly inside (every face certified, every mouth inside
its sliver); only `plan`'s count refuses. The P8 mutant in PR 5161's
review (the count checked for the annulus alone) shows what the count
guards: the cup with a handle then reaches `consume::spur` and fails as
a `SurgeryInvariant`, a cycle of edges left inside the merged face.

## What a fix needs

A walk that takes a feature of any genus and any number of mouths: cut
each handle (`mekr`/`kef` on a chord across it, as the ring bridge does
for a ring) until the merge leaves a tree, and plug all but one mouth.
The soundness argument (`plan`'s doc) holds unchanged for any closed
surface inside `M`, since `M`'s complement is connected; only the
surgery is missing. Cost M.
