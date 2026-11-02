---
id: cover-edge-doors
kind: unit
title: Every per-Curve3 edge-extent door reads geom's arc cover: topo's edge box rule, axial carrier, carrier and arc balls, the shell and maximal-faces balls, geom-brep's edge reaches, sweep and step-import
status: parked
opened: 2026-11-02
priority: P1
cost: H
blocked_on: [cover-geom-readings]
---


Second unit of the cover build (`cover-geom-readings` has the order and the ratified text).

Every door that bounds an edge's region by matching on `Curve3` reads `geom`'s arc cover instead, and keeps no kind match:
- topo: `boolean::boxes::{edge_box_rule, edge_rule_box, axial_carrier, edge_axial_span}`, `census::{edge_reach_in, edge_reach_of}` and `Candidates::build`'s `curved_box` (its NURBS comment is stale);
- `splitting::containment::{carrier_ball, conic_arc_ball}` and `step_ball`;
- `shell::{carrier_box, moved_walls}`;
- `reduce::gate_maximal_faces`'s inline ball;
- `geom_brep::extent::{edge_reach_from, edge_extent_sup, edge_reach_bounds}` (upper side) and `Reach::{lever_from, span_reach_from, range_along}`'s per-kind arms;
- `sweep::blend::surgery::boxed_reach`'s edge match;
- step-import `knot_split::piece_box`.

The survey (log.md, 2026-11-02) has the full list and where the doors disagree. Lower-bound doors (`edge_extent_inf`, `certify::edge_extent`) are a different question and stay.

Where an edge door moves a verdict (a ball or box tightens), re-measure; do not hold the old refusal.

Closes `an-edges-extent-has-four-dispatchers`.

- **`geom::spiric_period_box` becomes internal** to the cover reading once `boxes::edge_rule_box` reads the arc cover (PR 6383's review: it was left `pub` for this unit).
