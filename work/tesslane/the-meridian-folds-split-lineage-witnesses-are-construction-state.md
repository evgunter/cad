---
id: the-meridian-folds-split-lineage-witnesses-are-construction-state
kind: issue
title: The torus meridian fold's split-lineage witnesses are all construction state, so no at-rest body is known to reach the fold
status: open
priority: P3
cost: M
opened: 2026-11-23
---

Found by the LATHE wrap-edge-station lane (2026-11-23, branch
`lathe/wrap-edge-station`).

## What

Props folds the pieces of a split edge back into the meridian they
carry, keyed on split lineage (`crates/geom-brep/src/props/curved.rs`,
`props_meridian_pieces_meet`; MESH-10, issue 1562). Every row that
reaches the fold builds its witness by `split_edge` on the donut's
seam meridian, a wrap edge of its one wall:
- `mesh` `iso_rectangle_door::a_split_seam_donut_meshes_and_measures_as_the_unsplit_donut`;
- `iso_rectangle_door::split_children_partition_the_parent_edges_own_parametrisation`;
- `mesh10r1_probes::r1_a_reparametrised_split_child_refuses_at_the_fold`;
- `mesh10r2_probes::m10r2_set_edge_curve_on_a_split_child`;
- `mesh10r2_probes::m10r2_split_lineage_after_graft`.

The join now takes a station on a wrap edge, and check 11 refuses one
at rest
(`work/lathe/a-station-on-a-wrap-edge-is-not-joinable-and-passes-check-11.md`).
So each of those bodies is construction state: no finished-body door
(`mass_properties`, a boolean's operands) takes one. The rows now read
the fold only through the readers that take a plain `Body`
(`topo::props::loop_edges` with the shape door, the flux lane and
`boundary_material_sign`, and `mesh::tessellate`). They assert
construction state where they used to measure the split body's volume.
`m10r2_split_lineage_after_graft` lost its union half outright: issue
1597's two operand orders of a disjoint union of the grafted split
donut have no witness now, since no boolean takes a construction-state
operand.

## An at-rest witness

A seam meridian is carried by more than one edge at rest only where
another edge meets it, at a vertex that is no station (valence 3 or
more). The review of PR 7532 found one door that builds that:
`donut ∪ bar`, the tour donut and the brick `x ∈ [1, 3]`,
`y, z ∈ [−0.1, 0.1]`. The union is at rest, and its minor circles
carry `SplitEdge` lineage where the bar's edges end on them (six
split-lineage edges, measured on `lathe/wrap-edge-station`).

## Done when

The rows above read the fold on that body, or one like it, at rest:
`mass_properties` and the meridian fold on its split pieces, and the
union half of issue 1597 re-pinned with a far box in both operand
orders. That body's own 1-ulp order asymmetry is filed apart, on the
boolean's slate
(`work/onedoor/a-disjoint-union-volume-differs-by-an-ulp-between-operand-orders.md`).
