---
id: a-sweep-row-signs-a-chart-area-by-the-sense-bit-by-hand
kind: issue
title: sweep's a_loops_lift_runs_through_its_poles signs a chart area by the sense bit by hand; the rule's home is geom_brep::ChartWinding
status: open
opened: 2026-11-14
---


## Finding

Found by TESSLANE's winding-rule unit (`tesslane/winding-rule`), 2026-11-14,
sweeping the test directories topo's fold census does not enter.

`crates/sweep/tests/a_loops_lift_runs_through_its_poles.rs` (the helper that
answers a lifted polygon's equal-area and whether it is polar) ends
`Some((if record.sense { area } else { -area }, polar))`: a chart polygon's
signed area turned into the face's area by the bit, by hand. The same spelling
in `mesh::trimmed`'s test `the_tilted_pairs_faces_keep_their_poles_as_sides`
now reads `ChartWinding::of_signed_area(area) == Some(ChartWinding::of_face(sense))`.

## What closing it takes

Assert the polygon's winding against `geom_brep::ChartWinding::of_face` and the
area's magnitude separately, as the mesh row does.
