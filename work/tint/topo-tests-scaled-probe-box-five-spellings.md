---
id: topo-tests-scaled-probe-box-five-spellings
kind: issue
title: The scaled probe box is spelled five times across four topo suites
status: open
opened: 2026-09-16
---

## Finding

- **Where**: `crates/topo/tests/` — `probe_census.rs`'s `bx`,
  `probe_s5_sectors.rs`'s `bx`, `rim_dim_boolean_twins.rs`'s `box_at`,
  and two `bx` closures in `rim_dim_review_probes.rs`
  (`which_fixed_predicates_fire_in_the_twin_configs` and
  `silent_fixed_predicates_scale_linearly`).
- **Importance**: low
- **Confidence**: sure — the five are one function
- **Raised by**: the `dup-brick` lane (S-DUP), 2026-09-16

All five build **the axis-aligned box with every one of its six
coordinates through one scale factor**, over `Probe`. After the
`dup-brick` unit they no longer re-spell the construction — each is now
a one-line delegation to `common::brick` — but the *wrapper* is still
written out five times, and the five disagree only in how the scale
arrives: `bx` takes `s: f64` and closes over it, `box_at` takes the
scale as a `&F: Fn(f64) -> f64`, and the two closures capture an `s`
from their enclosing test. `probe_census.rs`'s `bx` and
`probe_s5_sectors.rs`'s `bx` are byte-identical.

The home is obvious and already reachable: a `scaled_brick` (or
`brick` with a scale argument) beside `brick` in
`crates/topo/tests/common/mod.rs`. What stops this being a one-line
unit is that it is only worth doing once someone decides which of the
two scale conventions the shared door takes — the `f64` factor, which
is what four of the five want, or the arbitrary map, which
`rim_dim_boolean_twins.rs` uses to run two configurations from one
body builder.

**Residue disclosed, not scheduled.** The `dup-brick` PR fixed the
construction copies; this is what its sweep left standing, and it is
filed here rather than left in that PR body.
