---
id: a-large-right-cone-does-not-revolve-at-fine-eps-and-its-volume-is-unread-at-the-default
kind: issue
title: The cone (0,0),(1000,0),(0,1000) does not revolve at eps 1e-12, a 4000 one at no eps, and the 1000 one's mass properties are not read at the default eps
status: open
opened: 2026-11-17
priority: P3
---

Found by PR 7054's review and reproduced by its implementer (`a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation`); neither looked into the cause.

## Finding

Bodies built with `crates/mesh/tests/split_rulings.rs`' `turned` (a profile revolved about `y`):

- The right cone `(0, 0), (1000, 0), (0, 1000)`, a full turn, does not revolve at ε 10⁻¹² (`revolve` answers an error; payload not read). At the default ε and 10⁻⁶ it builds.
- The same cone at height 4000 does not revolve at any of the three ε.
- On the 1000 cone at the default ε, `reach_probes::geometry_enclosing` panics at `pinned_volume` (`crates/mesh/tests/reach_probes.rs`, near :232) on every cell that tessellates, at every δ from 1000 to 1. The cones of height 1 and 100 (half-angles 2°, 10°, 45°) and of height 1000 at 2° do not. It is the unsplit body's mass properties that `pinned_volume` reads, so the split and the mesh are not involved; the panic's payload was not read.
- Related: at ε 10⁻⁶ a wedge of 0.05 of the wall `(1, 0), (2 + tan 0.01°, 0), (2, 1), (1, 1)` does not revolve (`ChartSingularityUndecided { point: Apex }` on its cone face).

## What it costs today

`split_rulings::a_run_anchored_on_a_needle_triangle_refuses_unresolved` reads `check_mesh` and `check_winding` and not the volume where the 1000 cone meshes, and uses the unit cone at ε 10⁻¹².

Owed: the cause of each (which decision escalates or refuses, at what margin), and whether a part of size 1000 failing to revolve or to be measured is a band scaling with the part's size.
