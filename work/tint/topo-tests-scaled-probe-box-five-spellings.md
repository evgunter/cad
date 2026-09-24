---
id: topo-tests-scaled-probe-box-five-spellings
kind: issue
title: topo's suites spell the box wrapper n ways, of which the scaled probe box is five
status: open
opened: 2026-09-16
priority: P3
cost: E
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

## Widened, 2026-09-16 — the class is not five

Filed first as five instances. The style review of the same PR showed
the class is *"topo's suites spell the box wrapper n ways"*, and that it
is much larger than the scaled-probe family:

- 49 spellings of the unit cube across 19 suites, and three coexisting
  conventions for naming a nullary wrapper over `brick` —
  `topo-tests-unit-cube-has-fifty-spellings`.
- Named wrappers whose bodies are one `brick` call under a domain name
  (`corner_table.rs`'s `top`/`leg`, `review_f7_pole_r1_probes.rs`'s
  `distant_brick`), converted by that PR but left as wrappers.
- Hand-built seats that never reach the shared builder at all —
  `topo-tests-straddle-seat-hand-copies`.

So the decision this row owes is narrower than the class but should be
taken with those in view: **one scale convention for the box wrapper**
(`f64` factor, which four of the five want, or an arbitrary
`Fn(f64) -> f64` map, which `rim_dim_boolean_twins.rs` uses to run two
configurations off one builder), homed in `common`. Whoever takes it
should read the unit-cube row first, because both answers land in the
same door.
