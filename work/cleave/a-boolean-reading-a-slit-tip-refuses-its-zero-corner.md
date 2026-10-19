---
id: a-boolean-reading-a-slit-tip-refuses-its-zero-corner
kind: issue
title: A Boolean reading an operand's slit or ring tip at a contact refuses SlitTipUnread: the corner between its two coincident edges is not read
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by the Boolean slit-form lane (`cleave/boolean-builds-the-slit-form`,
the parent row `a-boolean-builds-a-reflex-edge-lying-inside-a-face`).

## What

The Boolean now builds D1's slit form, so an operand may hold a slit
or a zero-area ring: two coincident distinct line edges in one face,
whose tip is a vertex both leave along one ray. Where a later Boolean
reads that tip's sectors (a contact there), `build_sectors`
(`crates/topo/src/boolean/sectors.rs`) meets the corner between the two
edges: its sine is in the zero band and its cosine definitely positive,
which `sector_shape` (`crates/topo/src/sector_shape.rs`, the
`SECTOR_STRAIGHT` rung) rejects between two distinct edges ("a spike
has no interior direction to guess"). That rejection is typed
`BooleanError::SlitTipUnread { operand, vertex, face }`, a construction
gap with no lever, where it would otherwise escalate `Corner(Straight)`
with a lever on the kernel's own slit.

Reached by `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`:
the plate less a void lying under the top along its ray holds the ray as
a ring of the top, and every op of it with each pyramid whose apex is at
the ring's tip (`along_void_ray`, five scenes) refuses, at every pose
(`REFUSING`). On main that body held the ray as one edge whose wedge
claimed the top's material on both sides, and every op built.

## Fix

Read the tip's corner as what it is in a sound body: the face's corner
round the tip, a full turn from one edge to its twin, subdivided into
sectors under a half-turn each, as the lone-edge full-circle reading
does. Then lift `SlitTipUnread` and the `REFUSING` rows.
