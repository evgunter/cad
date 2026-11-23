---
id: main-red-the-kite-moats-digest-moved-in-value-at-pr-7411
kind: issue
title: Main red: the kite moat's evaluation digest moved in value at PR 7411's re-posed plane offset, after PR 7131 respelled it
status: open
opened: 2026-11-23
priority: P0
cost: E
---


`editor-core` `split_pinch_keeps::the_kite_moats_subtract_names_its_edges_where_its_result_stands`
(slow set) is red on main at every ε at its digest assert
(`split_pinch_keeps.rs`, "the kite moat's evaluation, bodies and name
tables"): main pins `0xa4d9_ab2a_eaf0_bd50`, and the evaluation now
reads `0xfe07_0805_06b3_9e32` (`18304608028150963762`). Two merges moved
it.

1. **PR 7131 (`onehome/carrier-frames`, `9bc9d6517b`): spelling only.**
   The digest read `0x5f8d_d6d3_ab59_87de` after it. PR 7400's STYLE
   review dumped every chunk `fixture::digest` feeds on both sides; the
   dumps differ only in `Plane`'s `Debug` spelling and are
   byte-identical once rewritten into the old one. Filed as a class in
   `a-fixture-digest-pin-moves-on-a-debug-respelling` (GREEN, P3).
2. **PR 7411 (`germ/plane-pose-far-frame`, `3547fe5764`): values
   moved.** It re-poses `geom::plane_offset` as `Real::difference_dot`
   (exact differences, `Dot2`), so every caller of the one spelling of
   `(q − o)·n` now reads it to its own digits.

**Bisect, on hosted CI** (GREEN U14, 2026-11-23). The same chunk-dump
probe (a copy of `fixture::digest`'s walk that prints each chunk) ran on
throwaway branches at six main commits, all deleted since:

| main commit | feed digest | dump |
|---|---|---|
| `ccabcfb760^1`, `ccabcfb760` (PR 7434), `1883da3554` (7411's first parent) | `0x5f8dd6d3ab5987de` | byte-identical to the review's post-7131 dump |
| `3547fe5764` (PR 7411), `bb90c60104`, `c76443d5cc` | `0xfe07080506b39e32` | byte-identical to the dump on PR 7400's head (main `9eb483e0c7`) |

Runs 45200638946, 45200654296, 45200668604 (before) and 45209781073,
45209930200, 45210232315 (after); the `test (app)` slow-set step read
the row's left at `6885395609920571358` before and
`18304608028150963762` after, at all three ε. Each tree feeds 2061
chunks.

**What moved** (102 of 2061 chunks: 50 curve and 50 edge-curve chunks,
each an `EdgeCurve`'s `Debug` text, and two point-bit chunks; no name
table, census or count moved):

- 96 certificate residuals, e.g. `max_residual: 2.7755575615628914e-16`
  to `2.807118009980003e-16` on the subtract's moat edges.
- Last-ulp moves in the split's plane–plane intersection lines, each
  toward the rounded-exact value: `param_end 1.4999999999999996` to
  `1.5`; a witness and line origin `z: 2.0000000000000004` to `2.0`
  (two point-bit chunks, `0x4000000000000001` to `0x4000000000000000`)
  and `1.4000000000000004` to `1.4`; directions and `param_end`s at
  the last ulp (`1.920937271229855` to `1.9209372712298547`).

So this is not a respelling and was not re-blessed. The moves are of
the size and direction 7411 sets out to make, which argues they are
right; the re-pin is for whoever confirms that the kite's edges are
read better, not merely differently, at `0xfe07_0805_06b3_9e32`.
