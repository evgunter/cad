---
id: k-lints-definite-floor-is-read-off-a-retired-predicate
kind: issue
title: k-lint's definite-side floor and its threshold provenance are read off volume_backstop, a predicate the kernel no longer decides
status: open
opened: 2026-10-31
priority: P3
cost: M
---

Found by CLEAVE's PR 5842, which took the boolean's volume backstop off
the operation path under `[ev]` PR 5780. The kernel no longer decides
`volume_backstop`, `volume_backstop_operand` or
`volume_backstop_violation`, so no new k-report sample carries them.

## What

`tools/k-lint`'s definite-side floor, `BASELINE_FLOOR_MARGIN = 4.0e-5`
(`tools/k-lint/src/lib.rs`, the module docs' "definite cluster" line,
the percentile rationale and the rule (2) cap note), is P0 of the M7
baseline rounded down, and that P0 is a `volume_backstop` sample
(4.79652e-5 m on corpus/die_pips and corpus/die_composed).
`tools/k-lint/tests/threshold_provenance.rs` re-derives the floor from
that witness (its table row and the two `("volume_backstop", …)` rows),
and `docs/K-REPORT.md` and `docs/k-report-data/*` carry the samples.

As a record of the M7 baseline that is still true. As a floor for the
next baseline its witness family is gone: the next distribution's
definite minimum is some other predicate's, and nothing re-derives it.

## Owed

At the next K-REPORT re-baseline, re-derive the definite-side floor
from a family the kernel still decides, re-point the provenance rows
and the module docs at it, and say in K-REPORT that the M7 floor's
family left with the backstop.
