---
id: main-red-three-m10-interval-pins-moved-at-5728
kind: issue
title: Main red: three editor-core M10 interval pins moved at every eps since PR 5728 (certification half-width, stepped-shaft leaves, two_hole_plate symbolic count)
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found by nightly triage 2026-10-28, run
[40637814390](https://github.com/evgunter/cad/actions/runs/40637814390)
(main `a1cfd1cfdb`), job `full suite (every eps row, slow set included)`.
Red at every ε row (default, 1e-6, 1e-12); all three are slow-set rows.

## What

Three of editor-core's interval-driver pins moved:

- `m10_3_driver_interval::the_certification_width_is_no_longer_bounded_by_epsilon`
  (`crates/editor-core/tests/m10_3_driver_interval.rs:479`): the
  whole-certifying half-width settles at 0.0948 where the row's band is
  0.488 ± 2% (default ε). The row's own message: a move DOWN says the
  tier stopped discharging an identity, a move UP that the numeric
  channel's widening changed.
- `m10_4_r1_probes_interval::r1_a_real_tolerance_study_on_the_stepped_shaft`
  (`crates/editor-core/tests/m10_4_r1_probes_interval.rs:952`): the ±0.1
  box certifies in 4 leaves after 3 splits,
  `Receipt { certified: 4, refused: 0, splits: 3 }`, where the datum
  (M10-8) is one leaf.
- `m10_9_pins_interval::m10_9_no_registrant_lies_on_any_measured_document`
  (`crates/editor-core/tests/m10_9_pins_interval.rs:569`):
  `two_hole_plate` at 1e-9 proves 1305 decisions symbolically where the
  measurement says 1249.

## Where it came from

Bisected on a clean checkout: all three pass at `b8974faabc^1` and fail
at `b8974faabc`, the merge of PR 5728 (`offset/limb2-for-every-carrier`).
PR 5728 re-pinned the M10 rows it ran ("Counts that moved (limb-2
identities now counted symbolic)", and `m10_8`'s re-pin, where limb 2's
composite has no net at the root box and the drive bisects). These three
are slow-set rows outside its gate and were not re-read.

## What closing it takes

Each row's message asks for the mechanism before a re-baseline. The
likely reading is the one PR 5728 gave for `m10_8` and `m10_bulge`
(limb 2's decisions now counted symbolic; a box whose tilted faces'
chart frames do not certify at the root bisects), but the half-width
moving DOWN by 5× is the opposite direction from "more identities
discharged", so it needs measuring, not assuming. Then re-pin with the
reason, or fix the driver.

Reproduce: `cargo nextest run -p editor-core --ignore-default-filter -E
'test(/^m10_3_driver_interval::the_certification_width/) |
test(/^m10_4_r1_probes_interval::r1_a_real_tolerance_study/) |
test(/^m10_9_pins_interval::m10_9_no_registrant_lies/)'`.

## Band

P2, not P0 (2026-10-28): these are slow-set rows at interval and
tight-ε ground, which `work/README.md`'s bands put at P2; a main red is
not P0 by itself. The rows are pinned at what main produces, so main
is green on them while this row stays open for the mechanism.

## Pinned (2026-10-28)

Re-measured on main `974f160a`, identical at all three ε rows, and
each row pinned at that number with a comment naming this row:

- `m10_3`: the whole-certifying half-width is 0.094756 at every row
  (was 0.438988 / 0.488315 / 0.497892 at 1e-6 / 1e-9 / 1e-12).
- `m10_4`: `Receipt { certified: 4, refused: 0, splits: 3 }`; the
  study's chamber is the leaf `[-0.1, 0]` on both axes, not the whole
  box.
- `m10_9`: `two_hole_plate` proves 1149 decisions symbolically. The
  1305 / 1249 above predate PR 5811, which merged after 5728 but was
  measured on a main without it and moved the pin 1249 -> 1101. So
  PR 5728's move is 1101 -> 1149, +48.

Closing this row still takes the mechanism of each move; the pins hold
main green until then.
