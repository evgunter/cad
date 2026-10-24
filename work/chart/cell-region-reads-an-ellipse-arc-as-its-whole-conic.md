---
id: cell-region-reads-an-ellipse-arc-as-its-whole-conic
kind: issue
title: cell_region reads an elliptic edge on its whole conic's annulus, so cells beside the conic's phantom arc stay Undecided
status: closed
opened: 2026-10-24
priority: P3
cost: M
parent: clearance-cells-are-decided-in-model-space
closed: 2026-10-24
---

## What

`topo::cell_region`'s boundary reading (`crates/topo/src/cell_region.rs`,
`carrier_distance`) reads a circular edge on its own arc (`arc_distance`:
the chordal trim decides whether the arc's nearest point is an end), but
an elliptic edge on the annulus of its plane between its semi-axes, which
holds the WHOLE conic. A cell beside the part of the ellipse the edge does
not cover is therefore never cleared of that edge: it stays `Undecided`
however small it gets, and the clearance sweep subdivides it to its depth
budget instead of dropping it.

The circle case measured why this matters: before `arc_distance`, a
bumped block's half-cylinder bump read its two end circles whole, the
phantom lower half of the cylinder stayed `Undecided` where it touches
the bottom wall, and `m10_5_r1_probes_interval::a_block_with_a_rounded_bump_certifies_strictly_positive`
refused on its depth budget (36 refused pairs) instead of holding.

It is a looseness, never a soundness defect: the annulus holds the arc.

## Shape of a fix

The same reading for an ellipse: the arc's nearest point to a point off
it is an end where the point's nearest conic point is decided past the
arc's ends. On an ellipse the nearest conic point has no closed form, so
the trim needs a different station, e.g. the eccentric-anomaly foot read
through `ConicArc`'s unit coordinates with its levered trim.

## Closed

Closed 2026-10-24 by `clearance-cells-are-decided-in-model-space`'s
review pass (PR 5673): an elliptic edge, as a circular one, is read
through its own arc's floor (`ring_path::Conic::distance_floor`, the arc
of the circle over the same span less the conic's distance from it),
beside its whole annulus, in `splitting::containment::arc_distance_floor`.
`sweep`'s `cell_region_bodies::an_oblique_cylinder_section_decides_beside_its_ellipse`
holds it to an independent oracle.

