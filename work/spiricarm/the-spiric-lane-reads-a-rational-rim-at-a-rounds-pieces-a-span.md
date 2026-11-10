---
id: the-spiric-lane-reads-a-rational-rim-at-a-rounds-pieces-a-span
kind: issue
title: The spiric lane reads each rational rim at a round's full piece count per span, as the rotational lane did
status: open
opened: 2026-11-10
priority: P3
cost: E
refs: [a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses]
---

## What

Filed by the cylinder-pair cost lane (branch
`spiricarm/cylinder-pair-cost`, 2026-11-10), from its sweep for a lane
that reads a round's whole piece count on every span of a net.

`geom_brep::props::spiric` reads each of a face's rational rims through
`rational_rim::rational_vector_area_split(e, pieces)` at the round's
`EDGE_INIT_PIECES << round` pieces per span, and `spiric::round_cost`
prices it so (`w_rational · span_count` per piece). The rotational lane
did the same until its rounds spread the count over the spans
(`rotational::span_pieces`): a 625-span chord read 10,000 pieces at round
0 and floored with a removable part ten orders below its width. No
fixture here puts a many-span rim on a plane cap or a torus wall, so the
cost is unmeasured on this lane; a loft or sweep edge on a cap would be
the case to measure before changing the schedule.
