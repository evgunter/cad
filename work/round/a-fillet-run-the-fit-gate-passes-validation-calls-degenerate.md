---
id: a-fillet-run-the-fit-gate-passes-validation-calls-degenerate
kind: issue
title: the fillet fit gate emits a run validation then refuses as DegenerateSegment: the build and the validator decide a short run on different bands
status: closed
opened: 2026-11-06
priority: P2
cost: M
closed: 2026-11-09
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

## Closed (PR 6682, 2026-11-09)

The build and validation now read one decision on the run.
`fillet_leg_fit` reads a leg's run (length less setback) against the
run's linear band, the band validation's `vertex_separation` reads a
segment's length on (`sugar::line_line_fillet_trims` takes the band;
`arc_fillet_trims` decides on the band it already held), on both
sides and for straight and arc runs alike. A run validation would call
degenerate is a Zero fit and is not built; a run in the band escalates
at the fit, its recourse the gate's own (`FILLET_FIT_RECOURSE`, now a
`Sized` gate, so the ending adds the tolerance arm). Pinned by
`crates/profile/tests/fillet_run_fit.rs`: the reviewer's straight,
decimal, rounded-corner grid (360 cases) and arc-extension repros, and
the outgoing side.

Not the same question as `fillet-inband-recourse-drops-the-tolerance-lever`
(closed): that row is about which recourse sentences name the tolerance.
Its table's "`FILLET_FIT_RECOURSE` — the tolerance is no lever (exact-order
band)" no longer holds now that the gate reads the linear band; the
`Sized` kind is what names that lever now. `fillet_leg_reach` stays on
the exact-order band.
