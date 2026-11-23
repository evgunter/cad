---
id: the-sphere-cell-aspect-cap-was-chosen-for-a-raw-uv-cdt
kind: issue
title: sphere_grid_steps caps the cell aspect at sqrt 2 for a raw-UV Delaunay lemma; the curved lane's CDT now runs in its lattice's metric, where the premise holds at any raw aspect
status: open
opened: 2026-11-23
priority: P4
cost: M
---


Found by the review of `tesslane/one-slit-band` (PR 7498).

## What

`sizing::sphere_grid_steps` chooses its steps at UV aspect at most `SPHERE_CELL_ASPECT = √2`, so that after the counts' `ceil` (which can halve `h_v`) the lattice stays under the `2√2` past which its off-lattice lemma fails: a Delaunay triangle beside a rim or meridian chord point stays within one cell only while the aspect, in the plane the CDT runs in, is under `2√2` (measured past it: 1.5 δ on a polar cap at latitude 1.2, in raw UV).

The curved lane's CDT now runs in its lattice's metric (`curved::cdt_metric`): there the aspect is within `√2` of one whatever the raw aspect. So on the curved lane the cap no longer guards the lemma, and past 45° of latitude, where the cap binds (`h_u = √2·h_v`), the sphere's cells are smaller than the cell bound alone asks.

## What closing it takes

Decide whether the cap lifts on the curved lane (the trimmed lane's sphere arm triangulates in the chart's own metric at mid latitude, not the lattice's, and keeps its own reason). If it lifts, `sphere_grid_steps` solves the cell bound alone and its pin (`sphere_grid_steps_land_the_worst_triangle_at_delta`) re-derives; the coarser mesh is the gain. Measure the worst certificate on polar caps before and after.
