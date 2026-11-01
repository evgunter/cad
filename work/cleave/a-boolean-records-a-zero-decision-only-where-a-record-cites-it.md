---
id: a-boolean-records-a-zero-decision-only-where-a-record-cites-it
kind: issue
title: A boolean emits a Zero decision as a Coincidence only where a contact record cites it or it pinches, so a decision that shapes a regular cell (a third face through a seam) is invisible to the coincidence door
status: closed
opened: 2026-10-19
priority: P1
cost: M
design: true
closed: 2026-10-19
---


Filed by the WIRE orchestrator from FORK-ROWPRESENCE (2026-10-19). The
designers measured it on a private build of `d089305c32`; their probe
material is in WIRE's session scratchpad.

## What

`crates/topo/src/boolean/ops.rs` `cite_rows` emits a boolean's decisions
as `Coincidence` rows only where `records.decided()` cites them or they
pinch. `DecisionSite::VertexFusion`'s doc states the policy: a vertex
identity is a row where its "touch survives into the result, as a record
or as a pinch".

A decision that survives as the geometry of a regular cell leaves no
row, and so the `unproven-coincidence` lint never sees it. D10
`[coincidence.door]` says every coincidence the kernel infers from
values is recorded.

**The pencil.** Every member order builds, and `topo` records no
coincidence for the prism's face through the boxes' seam line. Tilt
that face slightly into the empty quadrant and a sliver appears.
- a = x[-1,1] y[-1,0] z[0,2]
- b = x[0,1] y[-1,1] z[0.3,1.7]
- c = a triangular prism over (-.5,-.5), (.5,-.5), (.5,.5), z[0.6,1.4]

The same filter applies to a pair boolean.

## Second gap, same layer

In `union_pinch_member_order`'s `crossing_one_wedge`, the (a, c) pair
judgement records no row, although two wedges that overlap in material
have apex edges on one line. A two-member `a ∪ c` probably publishes
nothing for that coincidence either; this is unchecked. The pair verb
states an edge-line coincidence as vertex rows only, and not as an
edge–edge carrier identity.

## What it needs

Every Zero decision a boolean takes is a `Coincidence`, cited or not.
`design: true` because the "touch survives" policy is stated text;
check its provenance before changing it. This is the prerequisite unit
for WIRE's `[ev]` PR 5329 (DM4's decision log), whichever way Ev rules.

Signed (WIRE orchestrator).

## Resolution (2026-10-19)

Closed by WIRE's PR 5352. Every margined decision a boolean takes is now a
`Coincidence` row, in pair booleans and fold steps alike, through one
spelling door (`boolean/ops.rs` `spell`). A decision whose input-cell
spelling would widen a cell is held as a typed `UnspelledDecision` on
`BooleanNaming::unspelled`; nothing is dropped. The pencil's step holds its
two crossings, and the (a, c) wedges publish their apex rows.

Follow-ups filed on CLEAVE:
- `a-cited-decision-whose-input-cells-misstate-it-is-still-published`
- `identical-twin-coincidence-rows-arise-on-coincident-lines`
- `a-boolean-zero-decided-and-consumed-in-place-reaches-no-row`
- `a-boolean-records-a-face-pair-decision-only-where-the-two-faces-meet`
- `a-split-records-an-on-vertex-only-where-it-leaves-two-runs`
- `a-re-charted-operands-rows-may-name-cells-the-re-chart-minted`

WIRE's unit 2 spells the held decisions.
