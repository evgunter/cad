---
id: the-edge-edge-and-point-record-confirms-read-no-material-sides
kind: issue
title: The edge–edge and point record confirms read only that the cells meet, never which sides their materials lie on
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by both designers of the pinch-crescent fork (`[ev]` branch
`contacthold/ev-pinch-crescent`), off its question.

## What

D1 tier 3's principle is one check at every contact: the touching
cells' materials lie on opposite sides of each other (PR 4795). The
face–face confirm reads the sides (the `Tangent` table through
`tangent_locus_relation`, "material sides included",
`crates/topo/src/census.rs:7443`–`7457`). Two other record confirms
read only that the cells meet:

- `confirm_edge_edge` (`census.rs:7735`) decides that two edges'
  interiors cross or overlap (`interiors_meet`, `ee_meeting`, or
  `curved::curved_interiors_meet`) and nothing about the wedges'
  material sectors at the meeting.
- the point lane (`census/point.rs:313`, `point::face_face`, reached
  from `census.rs:7511`) confirms a sphere–plane or sphere–sphere touch
  as a point coincidence, with no reading of which side either
  material lies on.

So a recorded edge–edge or point meeting whose materials overlap
confirms as a contact where it is interference.

## Owed

Each record confirm reads the sides at its meeting: coincident edges'
material sectors disjoint, a point touch's two materials on opposite
sides of the common tangent plane, a shared region interference (C6).
A probe per lane: a recorded meeting of overlapping materials refuses.
