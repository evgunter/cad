---
id: the-split-rule-b-reads-an-in-plane-edges-material-by-convexity
kind: issue
title: splitting::rules::apply_rule_b reads which side an in-plane edge's material lies on by its wedge's convexity, a composition beside the line cone's parity
status: open
opened: 2026-11-01
priority: P3
cost: M
---


Filed by `recl-edge-edge-composes-the-other-solids-material-beside-the-cone`'s
sweep (branch `oneread/recl-line-cone`), whose shape grep (`reflex`,
`convex`, half-space and/or composition) met it outside `boolean/`.

## What

`crates/topo/src/splitting/rules.rs` `apply_rule_b` reclassifies an ON
entry (an edge of the split solid lying in the split plane) by its two
cyclic neighbours' sides and, where both lie on one side `S`, by the
edge's wedge (`edge_wedge`): convex goes with `S`, reflex with the
opposite (module docs' table). That is the solid's material about the
edge's line against the split plane, read by convexity, where the
boolean's edge-edge site now reads a solid's material about a line by
the line cone's parity (`boolean::sectors::line_fans`). Not confirmed
as the same class: the split plane is not a solid, and rule (b) reads
one solid's own wedge, which `edge_wedge` decides through one
classifier (`geom_brep::enters_material`), not a composition of two
half-spaces. Filed for the weighing, not as a defect.

## Owed

Say whether rule (b)'s convexity reading is the line cone's question
(the split plane as a whole plane through the edge's line, the
neighbours' sides its given side) or a different one.
