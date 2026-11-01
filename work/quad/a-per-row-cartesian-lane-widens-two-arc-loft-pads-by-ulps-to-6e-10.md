---
id: a-per-row-cartesian-lane-widens-two-arc-loft-pads-by-ulps-to-6e-10
kind: issue
title: PR 5629's per-row Cartesian lane widens two arc-loft pads in the thread-count digest: vpad +6.5e-10 relative at 1e-6, apad by ulps at 1e-6 and 1e-12
status: open
opened: 2026-10-24
priority: P3
cost: E
---


## What

PR 5629 (`67006526`, ISO's `iso/loft-seam-per-row-lane`) sends a skin row
of constant non-1 weight to the Cartesian lane. That moved the
thread-count digest
(`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`,
`thread-count-digest/eps-*.txt`) on every ε row. QUAD's fix lane
re-cut the digest on PR 5640, under that file's rule for a move in
the wrong direction that is the named consequence of a correct change.
This item is the looseness that rule says to file.

Most pads got narrower. Three got wider:

| ε | body | pad | before → after | relative |
|---|---|---|---|---|
| 1e-6 | `arc_loft_1e9eps` | vpad | 2.2105706956e3 → 2.2105706970e3 | +6.47e-10 |
| 1e-6 | `arc_loft_1e9eps` | apad | ulps | +9.98e-15 |
| 1e-12 | `arc_loft_1e11eps` | apad | ulps | +7.44e-15 |

The narrower moves, for scale:

| ε | body | pad | relative |
|---|---|---|---|
| 1e-6 | `arc_loft_1e11eps` | vpad | −5.50e-7 |
| 1e-9 | `arc_loft_1e11eps` | vpad | −6.29e-7 |
| 1e-12 | `arc_loft_1e11eps` | vpad | −4.18e-8 |

Every reading still holds its closed form. The arc loft is a straight
stack of identical sections, so it is a prism. Its section is a square
of side 2s with one side bulged out by a quarter arc, so
`V = 2s³(3 + π/2)` and `A = 2s(6s + s·π√2/2) + 2s²(3 + π/2)`. On every
row V sits at 0.022 of its pad and A at 0.143.

A probe of the same fixture reproduces the old digest lines exactly at
`72a76e0a` (the parent of 5629) and the new ones at `67006526`.

The `sym-session` lines moved in the same re-cut: `sym_arc_loft` and
`sym_thin_strip` `validate_geometric` took 4 and 8 more symbolic
decisions. That move was not bisected separately.

## Done when

The vpad widening at 1e-6 is explained, which the ulp-level apad moves
do not need, and is either fixed or shown to be the price of the
Cartesian lane's exact arithmetic on that row.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds. PRs 5589, 5628, 5741 and 5815 moved no reading on this
line: `crates/sweep/tests/thread-count-digest/eps-1e-6.txt:7` still reads
`arc_loft_1e9eps vpad=40a1452432655556` (2.2105706969896955e3), the
widened value. The digest re-cuts since changed verdict counts and
hashes only. The widening is still unexplained.
