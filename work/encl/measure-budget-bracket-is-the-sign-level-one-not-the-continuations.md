---
id: measure-budget-bracket-is-the-sign-level-one-not-the-continuations
kind: issue
title: SignCertificate::measure's budget-refusal bracket is the sign-level one, thousands of times wider than what the continuation computed before refusing
status: open
opened: 2026-09-26
priority: P3
cost: D
---


## The finding

DEMO-FOUND (`memories/demo-purpose.md`), by the tour's teapot spout
probe (`demos/tour/src/teapot.rs`) once it measured through
`validate_geometric_certificate(..).measure()` and printed the bracket
a budget refusal carries.

At `CAD_TOLERANCE_EPS=1e-12` the spout canal refuses the reporting
target, and `topo::TargetUnreached::bracket` hands back

```
V in [0.000049972, 0.000060168] m^3
```

— a half-width of about `5.1e-6 m³` on a `5.5e-5 m³` body, ±9 %. The
refusal it rides with says the refused face's mean boundary
displacement was `2.801e-8 m` after one round; metered back through
`width(flux)/(3·area_mid)` with the body's area of `≈ 0.025 m²`, that
face's volume enclosure is of order `2e-9 m³`
(`work/perf/tier3-plus-v-needs-a-sign-and-pays-for-a-precision.md`
does the same arithmetic on an earlier reading). So the continuation
held an enclosure about three orders of magnitude narrower than the
one the caller gets, and dropped it.

This is by design as written: `SignCertificate::measure`
(`crates/topo/src/props.rs`) reads `self.enclosure()` BEFORE
`refine_to_target` runs, and `TargetUnreached::bracket`'s doc says it
is "the bracket the certificate held before the continuation ran".
`refine_to_target` answers `Err(MassPropsError)`, which carries a
width meter per face and no enclosure, so `measure` has nothing
narrower to return.

## Why it matters to a consumer

The bracket is sound, so nothing is wrong. But a caller reporting it
— the tour's ribbon, `pncad-py`'s `validate_geometric_measured`,
`step-import`'s `StepImport::Solid::enclosure` — reports the coarsest
bracket the kernel ever held for that body, on exactly the bodies
where the bracket is ALL it has to report. The teapot narration now
prints the ±9 % figure beside prose that says the enclosure "excludes
zero by about five orders of magnitude at the very round the chase
stops on"; both are true, of two different enclosures, and only the
wider one is reachable.

## What a fix is

Not decided here. The shape the finding suggests: the continuation's
refusal carries the whole-body enclosure at the round it stopped on
(every face's enclosure at its last completed round, folded as the
sign walk folds), and `measure` returns that where it is narrower
than the sign level. Whether the fold is sound across faces left at
different rounds is the question to settle first.
