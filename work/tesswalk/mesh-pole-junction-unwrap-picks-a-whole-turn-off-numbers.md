---
id: mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers
kind: issue
title: the mesh walk's pole-junction unwrap picks a branch off numbers, unswept against the same-edge rule
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [the-period-jump-has-no-ruling]
---


Found by PR 5080's review (both reviewers). It was outside PCERT's sweep
of strict-branch picks at singular points.

**The site.** `mesh::walk::unwrap_tie` (`crates/mesh/src/walk.rs`, about
:378-391; called from the loop walk's pole-junction arm, about :1247).
It is the f64 mesh twin of `topo::chord_join`'s pole pick:
- It reads `k = (prev − raw)/τ`.
- It takes `k.round()` when `k` is within ¼ of an integer.
- Otherwise it takes the floor/ceil candidate nearer an `anchor`.

So at a pole junction it chooses between `prev` and `prev ± τ` off the
numbers alone.

**The ruling.** Ev's ruling on `the-period-jump-has-no-ruling`
(PR 4975) says a whole turn is never read off numbers. At a pole:
- the walk entering and leaving along one edge is a whole turn by
  orientation, a literal;
- distinct edges are decided by sign decisions at whole-period marks,
  metered by the parallelism lever.

`topo::chord_join::pole_branch` implements this (PR 5080).

**What is unchecked.** Whether `unwrap_tie` agrees with that structure:
- on a slit, the same edge in and out at the pole, where `k` is an
  integer and `round` keeps the previous branch rather than turning a
  whole turn;
- on two distinct edges within ε of one direction.

A row on the slit dome of `crates/sweep/tests/pole_slit_window.rs`,
meshed, would show it.

## Moved to TESSWALK (what the walk and the lanes decide: structural premises, and readings taken off undecided numbers) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
