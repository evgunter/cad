---
id: a-plane-near-square-through-a-cone-apex-at-delta-near-eps-refuses-a-full-circle-corner
kind: issue
title: A plane through a cone's apex tilted delta near eps off square refuses Escalated Corner Straight full_circle or FallbackExtentUnsupported
status: open
opened: 2026-11-01
priority: P3
cost: M
---

Filed by SPLITDOOR on `splitdoor/cone-apex-boolean` (PR 6266), from its
review's measurement, not reproduced on the branch.

## What

A plane through a tipped cone's apex, tilted `δ` off square to its axis
with `δ` near the tolerance, meets the cone at its apex alone to the band.
At ε 1e-9 it refuses `Escalated { decision: Corner(Straight {
full_circle }) }` or `FallbackExtentUnsupported`, at half-angle 30° for
`δ = 1e-8` and `δ = 1e-9`, and at 60° for `δ = 1e-9`. Square to the axis
(`δ = 0`) and tilted well off it, the same op builds
(`a_plane_meeting_a_tipped_cone_at_its_apex_alone_keeps_it_whole`).

## Where to look

Whether the escalation is the honest answer: a plane `δ ≈ ε` off square
grazes the cone's full circle of rulings at first order within the band,
so an in-band reading is expected. `FallbackExtentUnsupported` is not:
the extent fallback reached a pose it has no arm for. Measure which op
reads which, and whether the fallback's refusal should be the
escalation's.
