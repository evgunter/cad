---
id: main-red-cylinder-pair-the-region-cuts-unmarched-at-1e-6
kind: issue
title: Main red: section_cert's a_cylinder_pair_the_region_cuts_marches_two_unbounded_arcs reads Unmarched at ε 1e-6
status: open
opened: 2026-11-02
priority: P3
cost: E
---


Found by SPLITDOOR (`splitdoor/cone-apex-vertex-pairs`, unit B stage B1)
running its edited rows at three ε. Red identically on main `39b833003`.

## What

`CAD_TOLERANCE_EPS=1e-6 cargo nextest run --profile ci
--ignore-default-filter -p topo -E
'test(a_cylinder_pair_the_region_cuts_marches_two_unbounded_arcs)'`
panics at `crates/topo/src/boolean/section_cert_rows.rs:129` (`shape`):
"not classified: Unmarched". The row asks the marched lane for the two
walls of radius 0.5, `0.05` rad apart and `0.1` apart, inside a region
of radius 1.5 (`section_cert_rows.rs`, the row's fixture). At the
default ε and at 1e-12 it passes. The row is in the slow set, so the
per-PR gate does not run it; the nightly runs it at every ε row.

## Where to look

What the march door answers at 1e-6 for this pair: whether the fixture's
near-parallel axes put the march's seed or step inside the band at that
ε, so that the lane gives up rather than tracing the two arcs.
