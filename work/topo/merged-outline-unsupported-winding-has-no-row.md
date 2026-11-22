---
id: merged-outline-unsupported-winding-has-no-row
kind: issue
title: merged_outline_ring's UnsupportedWinding verdict has no row: no body at f64 holds a loop the planar sum does not wind
status: open
opened: 2026-11-13
priority: P4
cost: C
---


Filed by the `sectarms/planar-spline-winding` lane.

`Body::merged_outline_ring` (`crates/topo/src/merge_faces.rs`) answers
`OutlineVerdict::UnsupportedWinding` where no loop of the merged face
winds positively or zero and one has no winding
(`crate::loop_winding::Unwound`), and names a zero winding before it.
Both were held by rows on a triangle with a rational quarter circle for
an edge (`a_nurbs_carrying_cycle_stays_undecidable`,
`a_zero_winding_is_named_before_a_loop_the_kernel_does_not_wind`),
which the planar sum did not wind.

The sum now winds that loop, and no fixture took its place. What is
left unwound at `f64` is an edge read by its enclosure alone with no
enclosure: a spiric with a piece whose `f` is not certified positive at
the finest count, or a spline whose speed bound is not certified, and
the mint refuses the one tried (a rational spline of degree 29:
`set_edge_curve` escalates `nurbs_span_meter` on its poisoned speed
bound). A scalar with no certified lane (a dual) reaches the verdict,
and the fixtures there are `f64`'s. The first row is re-pinned to the
decided winding
(`a_spline_carrying_cycle_winds_alike_for_every_reader`); the second is
deleted.

A row needs either a body at a scalar that does not certify, or a
spiric arc the mint takes and the enclosure does not read.
