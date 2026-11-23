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

## Verdict: the new value is right, re-pinned (GREEN U15, 2026-11-23)

Read from the two dumps (`dump-d`, before 7411; `dump-e`, after) by
re-deriving every moved number in Python, with exact `fractions`
arithmetic as the reference and IEEE `f64` replaying the kernel's
formulas. No cargo was run. The 102 chunks fall into three classes.

**1. The split's vertex at the pocket wall (2 point-bit chunks).** The
kite's corner `(2.0, 3.5)` in the moat's sketch is placed at
`(3.5, ·, 2.0)`. It is the vertex `(3.5, 1.0, 2.0)` where the split
plane `y = 1` meets that corner. Its `z` was `0x4000000000000001`
(`2.0000000000000004`) and is now `2.0` exactly. Every input that
fixes it (`3.5`, `1.0`, `2.0`) is a dyadic, so the exact value is
`2.0`.

**2. Plane–plane lines through that vertex and the kite's sides
(12 edge-curve chunks; on 8 of them the residual moved too).** Each
new `f64` is closer to the exact value
than the old one. The exact value is taken both ways: from the decimal
construction, and from the `f64` literals as stored (`0.8` and `3.2`
are not dyadic). Error against the decimal construction:

| quantity | exact | old (ulp off RN) | new (ulp off RN) |
|---|---|---|---|
| `param_end`/`param_start` on `x = 3.5` (`17v9`, `41v5`, …) | `3.5 − 2 = 1.5` | `1.4999999999999996` (−2) | `1.5` (0) |
| side `(x 2, z 0.8)→(3.5, 2)`: witness `z` | `1.4` | `1.4000000000000004` (+2) | `1.4` (0) |
| same side: `param_end` `√3.69` | `1.92093727122985460…` | `1.920937271229855` (+1) | `1.9209372712298547` (0) |
| same side: `dir.x` `1.5/√3.69` | `0.78086880944303033…` | `…302` (−1) | `…303` (0) |
| same side: `dir.z` `1.2/√3.69` | `0.62469504755442426…` | `…244` (+1) | `…242` (−1), \|err\| 8.6e-17 vs 1.4e-16 |
| side `(3.5, 2)→(2, 3.2)`: witness `z`, origin `z` | `2.6`, `2.0` | `2.6000000000000005`, `2.0000000000000004` (+1, +1) | `2.6`, `2.0` (0, 0) |
| same side: `param_end`, `dir.x`, `dir.z` | `√3.69`, `∓1.5/√3.69`, `1.2/√3.69` | each 1 ulp off | each the RN of the exact value |

Taken from the stored `f64` literals instead, the second side's new
values are exactly the correctly rounded ones. On the first side every
new value is closer than the old one: the length errs by 1.1e-16
against 3.4e-16, `dir.x` by 6.3e-17 against 1.7e-16, and `dir.z` by
7.1e-17 against 1.5e-16. The `−1` against the decimal value is a
direction normalised from a stored plane's rounded normal, which an
exact offset does not undo. Nothing moved away from the exact value.

**3. Certificate residuals (96 chunks; 88 on carriers that did not
move).** A `Certificate`'s `max_residual` at `f64` is the largest
sampled value of the checks it folds (`certify.rs`, `check_residual`):
here the two `implicit_residual`s, which are `geom::plane_offset`
(`implicit.rs:186`), at the 9 schedule stations, plus the two endpoint
distances. Replaying those formulas over each chunk's own carrier and
stored planes reproduces all 100 changed certificates bit for bit:
the old values with the plain `(q − o)·n` and the new ones with
7411's `Dot2`. Four of the 100 were 0 on both sides, which is why only
96 chunks moved. Every new `max_residual` equals the correctly rounded
exact residual of its sampled `f64` points against the stored planes
(100/100). The old values were the plain formula's cancellation noise,
quantised to multiples of `2⁻⁵⁴` (`2.7755575615628914e-16` is `5·2⁻⁵⁴`).
That noise read the true residual too high on 48 chunks and too low on
44. One case is the side `(2, 0.8)→(3.5, 2)`, which reported
`4.996e-16` against a true `5.105e-16`.

The 40 that grew are therefore not a tighter pose sampling a larger
residual elsewhere. On 88 of the 96 chunks the carrier and stations
are bit-identical before and after. Only the reading changed, and the
new reading is the residual those points actually have; the old one
under-read it there. Where the carrier itself moved (class 2), the
true residual fell: from `5.105e-16` to `1.221e-16` and from
`3.468e-16` to `2.455e-16`. The endpoint residual of `59v1`/`53v1` fell
from `4.44e-16` to 0 with the class-1 vertex. The largest residual
on either side is `5.0e-16`, eleven orders below the tightest
acceptance band (ε = 1e-12). The test passed every certificate at all
three ε on both sides, and only the digest assert was red. At `f64`
the number is a sampled measurement, not an enclosure, so "sound"
here means it is the true sampled maximum to within half an ulp. The
old one was not that.

So `0xfe07_0805_06b3_9e32` is the kite's evaluation read better, not
merely differently. The pin is moved to it.
