---
id: the-ring-gap-reads-a-lower-estimate-as-zero
kind: issue
title: step-import: ring containment reads Zero on a lower estimate of the ring gap (chart distance × the chart's least metric), which is no [verdicts] Zero
status: open
opened: 2026-11-18
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
priority: P3
cost: E
---


Found by the confirmation re-review of `importgate/interp-layer` (PR 7091).

## What

`crates/step-import/src/chart.rs:252` (`infer_outer`) reads a ring's
containment gap as `boundary_distance(&candidate.poly, probe) * floor`:
the chart distance times the chart's least local metric, a lower
estimate of the 3-D distance between the two bounds. It sends that to
`FileBand::read(STEP_RING_CONTAINMENT, ..)`, and a Zero answer becomes
`OuternessRefusal::RingsTouch`, ended as `Refusal::Touching` ("reads as
none at the band the file is read at").

A `[verdicts]` Zero says the two are one within the band. A lower bound
reading Zero says only that the true gap *may* be that small, so the
Touching refusal states more than was read. The outcome is a refusal
either way (in band refuses too, as `ApartInBand`), so no body is
admitted wrongly; only the refusal's words overclaim.

## Fix

Read the gap as an enclosure (the chart distance under the chart's least
and greatest metric over the probe's neighbourhood) and decide Zero only
on its upper end, or name the refusal by what the lower bound says (not
apart) rather than by what it would say were it exact.
