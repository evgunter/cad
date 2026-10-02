---
id: a-declared-continuation-across-a-rabbet-step-leaves-six-loose-ends
kind: issue
title: A union across a rabbet's step with every finding declared refuses Join(UnpairedLooseEnds { count: 6 })
status: open
opened: 2026-10-01
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

## A second shape, from a fold order (FUSE designer probe, 2026-10-02)

Measured at the kernel level (`test_support::brick`, `flush_declarations`
at every step) by a FUSE designer lane weighing
`work/fuse/a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made.md`:
`a` = x∈(0,1), `c` = x∈(0.5,1.5), y∈(−1,0), `b` = x∈(0.5,1.5),
y∈(0,1), all z∈(0,1). Folded `[a, c, b]` or `[c, a, b]`, the step
`(a ∪ c) ∪ b` refuses `Join(UnpairedLooseEnds { count: 6 })`: `a ∪ c`
is an L-shaped block and `b` sits flush against both of its arms, the
same reflex-step shape as the rabbet. The other four orders fuse. Not
pinned by a test; the probe was scratch and was removed.

## Re-opened at the merge (JOIN orchestrator, 2026-10-02)

The rabbet itself builds on JOIN-1, as above. The fold-order shape above
(`(a ∪ c) ∪ b` with `b` flush against both arms of an L) arrived on main
while JOIN-1 was in flight and has not been measured on JOIN-1. The row
stays open for it: if it builds now, pin it and close the row; if not, the
row carries it.
