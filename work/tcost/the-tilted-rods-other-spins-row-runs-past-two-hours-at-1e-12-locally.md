---
id: the-tilted-rods-other-spins-row-runs-past-two-hours-at-1e-12-locally
kind: issue
title: germ_tilted_rod's other-spins row ran over 2h18m at 1e-12 on a session box, against 2096 s at 1e-9 and 486 s at 1e-6
status: open
opened: 2026-11-04
priority: P2
cost: C
---


Filed by SPLICE's `splice/spiric-on-cap-plane` lane, from its Phase 0
measurement on main at `7072c784`.

## Finding

`crates/sweep/tests/germ_tilted_rod.rs`
`other_spins_and_steeper_tilts_refuse_at_their_germs` (in the slow set,
`.config/nextest.toml`'s `ci` filter) was run alone, on one cloud
session box, with `cargo nextest run -p sweep --test all -E
'test(other_spins_and_steeper)'`, one ε row after another. A probe
`eprintln!` printed at the slit pass's unread rider; it was not
committed.

| ε | wall time | outcome |
|---|---|---|
| 1e-9 (default) | 2096.6 s | PASS |
| 1e-6 | 486.0 s | PASS |
| 1e-12 | > 2h18m (8280 s), single-threaded at 100% CPU | killed unfinished; it had not yet reached the β 0.6 poses, where the 1e-9 and 1e-6 runs first reach the slit pass's unread rider |

The nightly's `full suite (eps 1E-12, slow set included)` job does run
it: run 41779026337, job 126438971955, at `63cbd7c3`. It is not among
that job's 33 failures, and the job's nextest summary is 3982.7 s for
17798 tests run in parallel. So on hosted hardware the row completes at
1e-12 in under about 66 minutes. The captured log tail does not reach
its own PASS line, so its own time there is not read.

What to settle:

- whether the 1e-12 row's cost is the test's own, for example a
  bisection or march that deepens with the band and runs at least 4×
  the 1e-9 time, or the box's;
- why 1e-9 costs four times 1e-6 here, when the poses and the stops are
  the same at each row.

A single row near an hour at one ε puts the nightly's 120-minute
`full-suite` timeout within reach.
