---
id: a-fillet-run-just-past-the-band-builds-and-then-escalates-a-span
kind: issue
title: a fillet run the fit decides Positive just past K·ε builds, then validation's arc_span / line_span on the non-adjacent pair escalates: the span checks lose the slack the fit margin kept
status: open
opened: 2026-11-10
priority: P2
cost: M
---


## Finding

Found by PR 6682's re-review (ROUND, 2026-11-10), by execution
(`/root/.local/share/cad-work/round-review-6682b/cad/crates/profile/tests/rr6682b_probes.rs`,
`rrb_grid`, `rrb_grid2`, `rrb_ext_detail`; lane-local, copy what you need).

`fillet_leg_fit` reads a fillet's run against the run's linear band
(`crates/profile/src/sugar.rs` `line_line_fillet_trims`, the
`decide_passing(Gate::FilletLegFit, …)` call, ~282; `arc_fillet_trims`,
~678–681), so a run below ε is no run and one in (ε, Kε) escalates at
the fit. Just past Kε the fit decides Positive and the run is built,
but validation's span checks on the NON-adjacent pair that meets
across that short run — `line_span` (`crates/profile/src/seg.rs`,
~1027) and `arc_span` (~1067), reached from `line_line` / `line_arc` /
`arc_arc` (~1625, ~1732, ~1984) — read the run's far end within the
band of the other segment and escalate. The build succeeds and the
loop it built does not validate.

Windows measured:

- **New in PR 6682** (the merges used to absorb these runs): the ray
  extension `line(2).fillet(0.5)`, bare and `.tangent()`, arriving at
  x = 2.5 + d for d in (Kε, ≈(K + 0.9)ε), at every ε row (`arc_span`
  on segments 0 and 2); and the arc-tip extension
  (`fillet_run_fit.rs`'s `arc_extension_at_the_tip(0.3, 1 − kε)`) for
  k ≈ 10–14 at the default row. The predicate varies by row and by k
  (`line_span` at 1e-9 and 1e-6, `carrier_line_circle` at 1e-12).
- **Already on main** for authored runs: a line × arc corner, an arc ×
  line corner (`la2`), the lens's arc × arc (`lens2`), and the
  straight-tip extension at k 15–20.

Pinned red on this row: `fillet_run_fit::a_run_just_past_the_band_builds_and_then_escalates_a_span`.

## Candidate fixes

1. Read the fit margin with the slack the span checks lose: decide the
   run against a band widened by what a span check on the neighbours of
   a run of that length can lose, so a run the span checks cannot read
   escalates at the fit instead.
2. Skip the far end of a run the fit already decided Positive: the
   pair pass need not re-ask the span of an end the construction placed
   past the band.

## Coupling

`sweep::swept::cosurface` (`crates/sweep/src/swept.rs`, ~1109) restates
the same identity margin the profile's `seg::arc_arc` reads, and PR
6682 relies on the two agreeing: validation's `carrier_circles_identity`
escalation at an arc extension's tip pair stands for the question
`cosurface` would ask of the run's walls. A fix here should not move
one without the other.
