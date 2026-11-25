---
id: a-carried-frames-renormalization-is-logged-at-f64-and-not-run-at-the-box-scalar
kind: issue
title: A mate chart's carried frame is renormalized, and the decision logged, at f64 only: every leaf of a driven mate document parts from its witness
status: open
opened: 2026-11-24
priority: P2
cost: M
---


Found by M13b (2026-11-24; `probe/M13b-REPORT.md` section 3 on
`boxfinish/probe-build-record`), tracing why the `f64` lane's decision
log and the box lane's differ at one point.

`Chart::charted` (`crates/editor-core/src/mate/value.rs`)
renormalizes the carried frame under `POSE_UNIT_NORM` unless the chart
is the identity or `T::spins_by_frames()`, which is `false` at `f64`
and `true` at `Interval` and `Sym`. So the `f64` witness logs three
`pose_unit_norm` decisions that no box-scalar replay makes: 1,991 of
the suite's builds have this shape (1,348 parting at
`session_box_reach`, 625 at `mate_chart_slides_span`, 18 at the end of
the log).

**None is driven today.** The first drive of a document with a mate
refuses every leaf: under today's leaf comparison as a flip naming
nothing, under the text put to Ev on PR 7700 as `Parted`, one replay
each. It is a prerequisite for driving a mated assembly, and the first
row the lane-parity test of that PR's unit will turn red on once a
mate document joins the driven set.

**The fix is at the site:** the renormalization's decision is made in
both lanes or logged in neither (an unlogged door with its ledger row,
as F48 made the props lifted pass). Which is right depends on what the
decision is for at `f64` and why the box scalars skip it; read that
first. Ground: `editor-core`'s mate values (MSOLVE's and COINCIDE's
rows live there); filed here because the box drive found it.
(BOXFINISH orchestrator)
