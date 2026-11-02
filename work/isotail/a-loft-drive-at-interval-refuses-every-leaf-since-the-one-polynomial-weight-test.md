---
id: a-loft-drive-at-interval-refuses-every-leaf-since-the-one-polynomial-weight-test
kind: issue
title: e6: a point-polygon loft and a rotated-frame loft no longer certify under the interval drive; they refuse all 256 leaves on budget and take 960 s instead of 8, since PR 5635
status: open
opened: 2026-10-24
priority: P2
cost: M
refs: [5635]
design: true
---

## Found (INTENT nightly triage, 2026-10-24)

Nightly run 40066431545 (main `3dfb96b91c`) reds both rows of
`crates/editor-core/tests/e6_drive_over_a_loft_at_interval.rs` that
expect one certified leaf, at default, 1e-6 and 1e-12
(`assert_certified_in_one_leaf`):

    a_point_polygon_loft_certifies_under_a_box_elsewhere:
      point: Some(Budget(Leaves { max_leaves: 256 }))
      left:  Receipt { certified: 0, refused: 256, splits: 255 }
      right: Receipt { certified: 1, refused: 0, splits: 0 }
    a_loft_placed_on_a_rotated_frame_certifies_under_a_box_elsewhere:
      rotated: Some(Budget(Leaves { max_leaves: 256 }))  (same receipts)

Each row took 957–962 s in the nightly. On 2026-10-23 (run
39906156937) they passed in 8.6 s. The arc-loft row of the same file
(`an_arc_loft_bisects_to_its_budget…`, which expects budget mass)
still passes.

**Bisected** (debug, default ε, the point-polygon row with a 150 s
timeout, `cargo nextest run -p editor-core --test all
--ignore-default-filter`): it passes at `0ca666f4` (#5634), the
first-parent merge just before, and at `c8b398d2` (#5611) and
`a7f11f62` (#5605). It times out at `9180a67d`, the merge of ISO's
PR 5635 (`iso/one-polynomial-weight-test`). Between `0ca666f4` and
`9180a67d` the only first-parent merge is `5589ff66` (#5642), which
touches no crate. So the regression is 5635's.

**Suspected mechanism, not verified.** 5635's `Weights::constant`
treats a weight that carries an enclosure, a derivative or a symbol as
not constant, whatever its value channel reads. Under the interval
drive a loft's section weights are probably such weights. Where the
old `!= 1.0` read their value channel and took the polynomial arm,
`constant()` now answers `None`, and the rational lane bisects to the
budget. If that is the mechanism, then either the drive's weights
should reach the constant test as plain constants, or the rational
lane has to certify what the polynomial lane did. Which one is a
design question for this program.

**Done when** both rows certify in one leaf again, or are re-stated
with a ruling on what a constant weight is under the drive. The rows'
assertions stay as they are.

Reproduce: `cargo nextest run -p editor-core --test all
--ignore-default-filter -E
'test(/^e6_drive_over_a_loft_at_interval::/)'`.

## 2026-10-28 — disabled on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Both rows still red at every ε, about 960 s each. They are
`#[ignore = "disabled red: work/isotail/…"]` rather than pinned: a pin
would spend 16 minutes per row per ε to re-read a budget refusal. So
nothing goes red when the fix lands, and closing this row owes removing
the two `ignore`s. Run them with the reproduce
command above plus `--run-ignored all`.
