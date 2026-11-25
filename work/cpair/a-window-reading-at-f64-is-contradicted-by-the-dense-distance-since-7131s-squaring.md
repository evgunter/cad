---
id: a-window-reading-at-f64-is-contradicted-by-the-dense-distance-since-7131s-squaring
kind: issue
title: curve_pair's window oracle: a reading at f64 is contradicted by the dense distance at eps 1e-12 on nightly seed 0x4071461b4c688a45, since PR 7131 squares frames at f64
status: open
opened: 2026-11-22
priority: P2
cost: M
refs: [a-touch-window-fuzz-seed-contradicts-the-dense-distance-at-1e-12, 7402, 7131]
---


Found by GREEN (U12) in nightly 44989234943 (main `4d3402ecb1`'s kernel), at ε 1e-12. It is green in the 11-21 nightly 44870651337 (main `f5ef8f2b`), on that run's own seed.

`geom-brep` `curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance` (`oracle.rs`, the assertion at about line 349) fails:

    ε 0.000000000001: 1 readings contradicted (reproduce with CAD_FUZZ_SEED=0x4071461b4c688a45 CAD_FUZZ_EFFORT=1): [
      "draw 7 f64: Recedes on (3.2272599256367685e-10, 3.228751041377335e-10) ring None: a Circle(Circle { frame: Frame {
        origin: (-293.9481167401991, 168.2692198219998, 1.1866774119181317e-7), axis: (1.9599143542866378e-10, -1.819360637945598e-10, 1.0),
        u_ref: (0.8761920509866891, -0.4819621248477306, -2.5941242967692446e-10) }, radius: 457.44658154992277 }) b (120.366928440912, 113.43223485221837)" ]

**This is a contradicted reading, not a refusal.** The reader answers `Recedes` at `f64` on a piece where the oracle's dense distance says otherwise. Until shown otherwise, a contradiction against the dense distance is a soundness question. It may still be the oracle's: its sampling at a piece 1.5e-13 long, 290 m out.

**The merge is PR 7131** (`9bc9d6517b`, `onehome/carrier-frames`). Each result below is the seed above replayed in a child at 1e-12 on hosted CI:
- green at `e4d679d37e` (run 45083192761);
- red at `9bc9d6517b` (run 45083200534);
- under 7131's mutant patch on `4d3402ecb1` (run 45083184695):
  - **F1F alone turns it green.** F1F stores an `f64` frame raw, as main did before 7131.
  - F1X alone and A alone leave it red.
  - All three together turn it green.

So the draw's circle now carries a frame squared at `f64` (7131's `RigidMotion::squared`), and the reader's `f64` reading of that pose is what the oracle contradicts. Which side is wrong is not measured. The sibling row `a-touch-window-fuzz-seed-contradicts-the-dense-distance-at-1e-12` is the `_at_a_touch` row of the same oracle, on another seed.

**Reproduce:**

    CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x4071461b4c688a45 CAD_FUZZ_EFFORT=1 \
      cargo nextest run -p geom-brep -E 'test(=curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance)'

The row is counted in ONEHOME's `pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted` (on PR 7402's branch until that merges) as one of the frame cost's two `f64`-squaring rows. Unlike the rest of that count, it is a finding and not a cost. It is not pinned; GREEN greens nothing until Ev rules on the count.
(GREEN, U12)

## A second seed, from `oneround/unsourced-unions` (PR 7294), 2026-11-23

The row's hosted run on PR 7294 (run 45098183324, the slow-set step for the tests the diff edits) reds the same way at ε 1e-12 on seed `0xfe838f340344caed` (`CAD_FUZZ_EFFORT=1`): `draw 10 f64: Recedes on (-6.599691873753483e-8, -6.592501423312038e-8) ring None`. The draw is a circle of radius 734 whose centre is 721 m out, with a frame tilted by about 3·10⁻¹² against a point pair. Replayed locally, it reds on the same draw with PR 7294's head and with `origin/main`'s `geom-brep` files (`d7963dd38f`). So it is this row's, not that diff's.

## Evidence (germ/co-surface-table, 2026-11-23)

`geom-brep`'s `curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance` is red on `origin/main` at default ε on seed `0x22eb45feecea31d9` (re-run on a main worktree, 2026-11-23, from PR 7431's review).

## Evidence (germ/spiric-near-miss-off-q, 2026-11-23)

The same row is red on main `61a3d77a0b`'s sources on seed `0x856e9c81e4b404d1` at ε unset, 1e-6 and 1e-12: three `Recedes` readings contradicted at ε 1e-12's column, draw 18, a circle of radius 485.93 whose frame's axis is `1.3e-9` off `z`.

**Pinned red (GREEN U13, 2026-11-23, PR 7558).** Ev ruled on PR 7505 that PR 7131's frame cost stands. This row is a wrong answer, not that cost, and is pinned as one. `a_window_reading_the_dense_distance_contradicts_is_pinned_red` (`oracle.rs`) replays the sweep on `CONTRADICTED_SEED` with a fixed replay text and pins the whole failure text: the count and draw 7's reading. The oracle's bands are its own, so the seed stops at **every** ε row (default, 1e-6 and 1e-12, measured), not only at 1e-12. The pin goes red when the reading holds (mutant F1F) or a different one is contradicted.

**The fuzzer is red on main at random seeds often (GREEN U13, 2026-11-23).** PR 7558's gate (run 45195294885, default ε) drew seed `0xc960ef101b854219`, and the sweep failed the same way: draw 58, `f64` `Recedes` on a circle of radius 523, 405 m out, contradicted at the needle's band. That is three seeds seen red in one day: this one, `0x22eb45feecea31d9` above, and the nightly's `0x4071461b4c688a45`. The row is in the slow set, so every nightly leg and every PR that edits `oracle.rs` draws it. Since its bands do not read ε, a red seed is red at every leg.
- 2026-11-23 (GERM, PR 7553's fix pass): another seed. At `CAD_FUZZ_SEED=0xabba8ad2f01a1859`, ε 1e-12, `each_window_reading_holds_on_a_dense_distance` reads draw 34 at `f64` as `Recedes` on `(1.68e-8, 2.66e-8)` where the dense distance contradicts it. It does the same on main `9eb483e0c7`.
- 2026-11-23 (GERM, PR 7553): seed `0x1` reds the row too, at ε 1e-12. Draw 58 at `f64` is read `Recedes` against the dense distance, the same on main `64aeba04f9`.
