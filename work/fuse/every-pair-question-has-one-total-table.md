---
id: every-pair-question-has-one-total-table
kind: unit
title: Audit every geometric pair question (point/curve/surface against each other) for one total kind-pair table with one method per pair and no runtime fallback, and propose the general principle for DESIGN.md
status: open
opened: 2026-10-14
priority: P2
cost: M
refs: [two-curve-curve-meeting-finders, 5027]
---


## Ev's question

On PR 5027 (2026-10-14), after accepting the C5 curve-pair paragraph, Ev asked whether the principle should generalize further, and whether anything else is out of compliance with it. "That could be a future unit."

The principle in question: every geometric pair question has exactly one total kind-pair table, one method per pair, and no runtime fallback. The pairs are point, curve and surface against each other.

## Known non-compliance (orchestrator's first list, 2026-10-14)

- **Curve × surface has no table.**
  - `boolean::carrier_touch` is a bisection with conic-only speed bounds; it refuses spiric and spline carriers.
  - `circle_roots` and `conic_quadric` each have their own door.
  - Conic × plane is solved twice: `carrier_cross::circle_meets_plane`/`ellipse_meets_plane` and `splitting::conic_plane_candidates`.
- **The census edge-face cut** reads only chords and conics; anything else is `Uncrossable` (restread row).
- **Line × line is spelled three times.** `the-curve-pair-reader` absorbs this.
- **Point against curve has four readings.** `the-curve-pair-reader` absorbs this.
- **Point against face has two routines.** They now share one core, and each keeps its own boundary decision.
- **geom-brep SSI and props** run six subdivision walks of their own (`issues/geom-brep-keeps-six-subdivision-walks-of-its-own`).

## What it owes

1. **Sweep every pair question** in the kernel. For each, record:
   - the table it should have;
   - its arms today;
   - which pairs are off-table, duplicated, or reached through a runtime fallback.
2. **File each gap** on its owner's slate.
3. **Propose the general principle as a `docs/DESIGN.md` revision** in its own `[ev]` PR, weighed by a designer pair if the shape of the principle is itself a fork.
