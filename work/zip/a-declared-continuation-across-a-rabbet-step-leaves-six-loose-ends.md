---
id: a-declared-continuation-across-a-rabbet-step-leaves-six-loose-ends
kind: issue
title: A union across a rabbet's step with every finding declared refuses Join(UnpairedLooseEnds { count: 6 })
status: closed
opened: 2026-10-01
closed: 2026-10-02
---


Found by REACH's fix pass on PR 3657 (`reach/cosurface-continuation`),
measured on that branch after main was merged in (`3aff2e6a07`).

## Repro

`crates/sweep/tests/reach_continuation.rs`, the rabbeted plate
(`rabbeted`: 6 × 4 × 1 with a 1 × 0.5 step cut along its east edge,
volume 22) and a block on its east edge, every flush-detector finding
declared (one `Rest` pair and the continuations), the union:

| block | union | subtract | intersect |
|---|---|---|---|
| `x 5..6, z 0..1` (fills the rabbet and overlaps the step below it) | `Join(UnpairedLooseEnds { count: 6 })` | 20, builds | 2, builds |
| `x 4..6, z 0.5..1` (fills the rabbet and overlaps the plate's top) | `Join(UnpairedLooseEnds { count: 6 })` | 20, builds | 2, builds |
| `x 5..6, z 0.5..1` (fills the rabbet only) | 24, builds | — | — |
| `x 4..6, z 0..1` | 24, builds | 16, builds | 6, builds |

The volume oracle is box arithmetic: each refused union is the full
6 × 4 × 1 box, 24.

The refusal is raised by `boolean/join.rs` `connect`'s leftover count
(the `UnpairedLooseEnds` return after the completed polygons are
resolved). Undeclared, each pose refuses `UndeclaredCoincidence` at the
reduction, as it should. What the two failing unions share is a
continuation pair beside a `Rest` pair where the block's face OVERLAPS
the plate's (the east wall over the step's wall, or the top over the
plate's top) rather than only abutting it; which section ends go
unpaired has not been measured.

The row pinning the refusal is
`a_declared_continuation_across_a_rabbet_step_refuses_its_union` in that
file; it moves to the build when this is fixed.

## Closed by JOIN-1 (PR 3790)

I merged main, with this item, into JOIN-1's branch. On that branch both refused unions now build. Each gives the 6 × 4 × 1 box: volume 24, six faces, sound at tiers 3 and 3′, and a legal operand against a disjoint brick.

The section segments along the step are edges of both solids, and JOIN-1's locus matching pairs their ends.

The row is now `a_declared_continuation_across_a_rabbet_step_builds_every_op`.
