---
id: a-fillet-run-the-fit-gate-passes-validation-calls-degenerate
kind: issue
title: the fillet fit gate emits a run validation then refuses as DegenerateSegment: the build and the validator decide a short run on different bands
status: open
opened: 2026-11-06
priority: P2
cost: M
---


**Found by PR 6545's review (ROUND, 2026-11-06), by execution.** A class
finding, not this PR's defect.

## Finding

Near an exact fit, `fillet_leg_fit` (`crates/profile/src/sugar.rs`)
classifies the fillet's run-in as Positive and the build emits it, but
`validate` at the same tolerance refuses the loop `DegenerateSegment`.
Reproduces on main without `.cusp()`: a forward `fillet(0.3)` off an
arc leg with the arrival at y = 1.3000000001 (a ~1e-10 run), and even
at y = 1.3, where a 1-ulp run is emitted. With `.cusp()` (PR 6545), a
corner at x = 1.4999999999 shows the same. The reviewer's probe:
`/root/.local/share/cad-work/round-review-cusp/scratch/rv_cusp_probe.rs`
(lane-local; copy it into the unit).

The fit gate classifies on the exact-order band (`Band::least()`),
validation's segment-length gate on the run's linear band, so a run in
`(exact-order, linear band)` is built and then refused. Either the fit
gate's Positive arm must not emit a run the validator will call
degenerate (the zero-fit arm absorbs it), or the two must read one
decision. Which, and whether this is one more instance of the
`fillet_leg_fit`/`fillet_leg_reach` exact-band question in
`fillet-inband-recourse-drops-the-tolerance-lever`, is the unit's to
decide.
