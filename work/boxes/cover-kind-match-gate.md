---
id: cover-kind-match-gate
kind: unit
title: A tree gate: no per-kind Surface or Curve3 match that builds a box, support, reach or ball outside the cover modules
status: parked
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [cover-levers]
---


Last unit of the cover build (`cover-geom-readings` has the order and the ratified text).

A `scripts/gates/` check that fails when a file outside the cover modules matches on `Surface::` or `Curve3::` kinds to build a box, support, reach or ball, or writes a literal torus or sphere ball. Per Ev's ask on this design: make the code lead every future instance to the one place. The survey found that name greps miss inline matches, so the gate should key on the shape, not on function names. Its allowlist only shrinks. The door census in `boxes.rs` (`every_door_that_reads_a_box_is_inventoried`) is retargeted to cover readers or retired into the gate (`box-door-census-misses-a-point-free-read`).

## Scheduled here (2026-11-09, PR 6596's review)

`cover-levers` handed these on in its PR body, and they belong to this unit's sweep:

- **`geom_brep::Reach::foot_on` / `reading_point`** still match on kind to choose a pivot. Where the pivot feeds a lever (`foot_lever`'s floor), it is a region read. `cover-levers`' fix pass corrects the floor; this unit makes the pivot choice read the cover, or shows it is not a region question.
- **Folded edge sites that still read the whole carrier rather than the span:** props, arcs, trace, `chord_join::SectionCurve::lever`, `RimSide`.
- **`containment::arc_distance_floor`'s conic annulus floor**, which `cover-levers` left: a per-kind tightening of a nearest distance. It belongs in the cover's arc near reading (`geom::cover` `Arc::point_near`), and moves the clearance and touch gates' verdicts, which need re-measuring. (`cover-levers` fixed `pcurve_cache`'s `R + r` angular arm. It showed that `rho_sum`, a Lipschitz factor, and `chart_region`'s `R + r·cos v`, a chart metric, are not region bounds.)

The gate's job is to make every such site red. Its shape needs to see inline arms, `speed_lo()` annuli and literal `R + r` sums. Name-keyed greps missed each of these. (BOXES orchestrator)
