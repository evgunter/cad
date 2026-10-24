---
id: cell-region-reads-an-ellipse-arc-as-its-whole-conic
kind: issue
title: cell_region reads an elliptic edge on its whole conic's annulus, so cells beside the conic's phantom arc stay Undecided
status: open
opened: 2026-10-24
priority: P3
cost: M
parent: clearance-cells-are-decided-in-model-space
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

