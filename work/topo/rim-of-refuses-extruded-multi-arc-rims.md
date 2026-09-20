---
id: rim-of-refuses-extruded-multi-arc-rims
kind: issue
title: topo::query::rim_of refuses every multi-arc rim extrude mints: the arcs' carrier circles are not bit-identical
status: open
opened: 2026-09-13
---


## Finding (BLEND unit 7, Phase 1)

`topo::query::rim_of` (`crates/topo/src/query.rs`) matches a seed
arc's siblings by `CircleId::same_circle` — centre, radius and axis
each **bit-equal** — and its doc says every producer a consumer holds
a body from, `extrude` named, "stores one rim's arcs on bit-identical
centres, radii and axes". Measured false for `extrude`: a circle
authored as `n ≥ 2` bulged arcs and extruded stores each arc's
carrier with its own centre and radius — on the pristine two-arc
cylinder the centres are `(3.4e-33, 8.6e-17, 1)` and
`(-3.4e-33, -2.5e-17, 1)`, on the three-arc one the radii are
`0.5000000000000001`, `0.5`, `0.5` — so the door sees one arc "on
this arc's own circle" and refuses `NotOneRim` on every extruded
multi-arc rim, the shape every `disc_of_arcs` fixture is. No seed
finder routed through the door can serve those fixtures;
`test_support::circle_arcs_at_z` scans by carrier station instead.
In-tree reproduction:
`review_closed_chain_junctions_r1_probes::r1_rim_of_refuses_the_extruded_two_and_three_arc_rims_on_carrier_bits`.

`review_blend1_r1_probes::r1_a_three_arc_rim_carves_where_a_two_arc_rim_does`
attributes its
`rim_of` refusal to the re-keyed cap; the pristine extrusion refuses
the same way, so that comment names a second cause, not the first.

## Fix shape

Either `extrude` derives every arc's centre and radius of one loop
circle from one computation (where each arc's stored centre comes
from today is the producer's side to measure), or `rim_of`'s identity
admits a tolerance it states — the door's doc argues against the
second. Which arm belongs to the producer or the door is the
orchestrator's call; the measurement is the r1 row above, on
`test_support::disc_of_arcs(2, 0.5, 1.0, tol)` and `(3, …)`.
