---
id: the-interior-witness-rung-reads-a-ring-where-it-is-stored-not-in-its-outer-loops-sheet
kind: issue
title: chart_region's interior-witness rung arranges a face's ring segments where each ring's lift is stored, a period from its outer loop's sheet for a displaced ring
status: open
opened: 2026-11-14
priority: P3
---


Found by the sweep of `work/tesslane/a-rings-lift-is-placed-against-its-outer-loop-in-one-place.md` (branch `tesslane/ring-placement`).

`chart_region::extract_face_uv` reads each loop of a face through its own lift (`loop_uv_polygon`, `ChartRead::Minted` through `pcurves::lifted_images`). Each loop's lift starts on one of its own rows. So on a closed chart a ring's polygon can sit whole periods from its outer loop's: ball ∖ tube's ring is one `u` period below (`crates/mesh/tests/lift_probes.rs`, `a_ring_sits_in_its_outer_loops_polygon_but_on_the_ball_less_the_tube`).

- The area deductions (`overlap_of_regions`, the cylinder band's area margin) are translation-invariant, so a displaced ring deducts what a placed one does.
- `boundary_segments`, for the interior-witness rung, arranges the rings' segments where they are stored. A displaced ring's segments lie outside the outer polygon, so the arrangement does not cut the region where the hole is. A cell centre can then fall in the hole; `contfp` answers it `Out`, and the rung loses that witness. It is not unsound, only weaker.

`topo::Body::face_lift` places each ring in the outer loop's sheet on an analytic chart. It serves the minted read here; the derived read (`ChartRead::Derived`) has no stored rows and would need the same rule over derived images.

Not measured: whether any pair the interior-witness rung is asked about holds a displaced ring.
