---
id: the-cylinder-rim-axial-decision-refuses-every-leaf-of-the-two-parameter-drive
kind: issue
title: The cylinder rim's axial decision (pcurve_cylinder_chart_axial) refuses every leaf of m10_3's two-parameter drive at Interval, since PR 6105
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Filed by a GREEN lane (`green/main-red-editor-core`, 2026-11-02) from
`work/green/main-red-six-editor-core-interval-rows-red-behind-the-cancelled-full-suite`.
The code is `geom_brep::pcurve_cache::cylinder_rim_axial`, which CLEAR's
CAVITYDIAG (PR 6105) added on this program's ground to close
`a-cylinder-rims-axial-channel-holds-zero-only-to-an-enclosure-at-interval`.

## What

Two rows of `crates/editor-core/tests/m10_3_driver_interval.rs` stop
certifying anything. At every ε row (default, 1e-6, 1e-12) the
two-parameter plate's ±0.1 drive refuses all four leaves:

- `the_two_parameter_drive_certifies_after_bisection_and_accounts_for_all_of_it`:
  "the box should certify SOMETHING: Receipt { certified: 0, refused: 4, splits: 3 }";
- `a_band_parameter_certifies_normally_and_prices_nothing`: "a band must
  not stop certification" (the same drive, with a `Band` radius).

## Where it came from

The first bad merge is PR 6105 (`5acea3cb35`, `clear/cavitydiag`). Its
first parent `2a6323786e` passes both rows, and so does `3592dfc075`
before it. On main `a285cdbf` both rows pass again with one change to
`cylinder_rim_axial`: the rim's terms are zeroed wherever both hold zero,
and the `decide("pcurve_cylinder_chart_axial", …)` call is dropped. So the
refusals come from the decision itself, not from the zeroed terms. The
likely reading, not yet confirmed: on a leaf where the plate's radius varies,
the sup of the rim's axial terms is not decided `Zero` symbolically, and the
drive refuses a leaf with an undecided decision. A rim stays ⊥ its axis at
every parameter value, so the decision should hold over the box.

The same experiment also shows that #6105's other moves are this
decision's own symbolic count, and those are re-blessed:
`sym11_exact_channel_rows` (annulus 654 -> 666, bracket 1542 -> 1556),
`sym_9_retry_interval`'s ladder row (annulus 654 -> 666), and both
`m10_9_pins_interval` rows (annulus 654 -> 666, bracket 1537 -> 1551,
door shut (1474, 1218) -> (1488, 1218)).

Reproduce (opt-level 2, as the nightly runs it):

    CARGO_PROFILE_DEV_OPT_LEVEL=2 CARGO_PROFILE_TEST_OPT_LEVEL=2 \
    cargo nextest run -p editor-core --profile ci --ignore-default-filter \
      -E 'test(/^m10_3_driver_interval::(a_band_parameter_certifies_normally|the_two_parameter_drive_certifies)/)'

## Pinned red

Both rows are pinned at every ε to stop on their current panic
(`test_utils::pinned_red::row`), citing this row.

## Done when

The drive certifies its leaves again, with the rim decision kept, and
the two pins are dropped.
