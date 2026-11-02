---
id: joining-a-spline-carrier-is-unbuilt
kind: issue
title: A curved join on a spline carrier refuses JoinCarrierUnsupported: the kept edge cannot be run on along a carrier with no period or parameter inverse
status: open
opened: 2026-10-07
priority: P0
cost: H
---

## The finding

The curved join restates the kept edge over both edges' span on its own
carrier (`topo::boolean::edge_join::joined_spec`): a closed join over
the carrier's period, which only `Circle`, `Ellipse` and `Spiric`
carriers have, and an open join through the killed edge's far end,
recovered by `Curve3::param_near`, which has no inverse on a `Nurbs`
carrier. Where either is missing the join refuses
`BooleanError::JoinCarrierUnsupported` (pncad-py tag
`join_carrier_unsupported`) and the boolean fails, where before the
curved join it left the vertex standing. The ruling allows the refusal
(a vertex the build cannot join refuses typed); no `ci` row reaches
it, since no operand carries a spline edge between two faces past the
boolean's operand gate (`BooleanError::CurvedEdgeUnsupported`).

## What it needs

A spline carrier's run-on: its knot span extended (or the two edges'
spline images concatenated) and a parameter foot on it, so the joined
edge is one spline with a certified interval.

## Parked (2026-10-13, FUSE orchestrator)

Nothing can reach a spline join until a spline edge crosses a face. Even with the operand gate deleted (`splice/delete-the-boolean-operand-edge-gate`), the sweep refuses such an edge first, typed as `BooleanError::CrossingCarrierUnsupported`. When this join is finally reached, it refuses typed too (`JoinCarrierUnsupported`). So this row waits on REACHHOLD's NURBS crossing rung, the first producer of a spline edge between two faces. It drops to P2 because, until that rung lands, no input can reach it.

## Moved to SPLICE (2026-10-25)

From HONE. Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P2 → P0 and UN-PARKED (was `blocked_on: [nurbs-edge-crossing-rung-is-the-ring-composite]`). Its P2 rested on no input reaching it; with the crossing rung on this slate the two are built in order here, and the demo joins (klein wall 3, lily wall 8, teapot wall 3) are its inputs.

## Reached by a cylinder × sphere boolean (2026-10-28, SECT step 1b)

The cylinder × sphere chord lane (`sect/cylinder-sphere-chord`) is a
producer of spline edges between two faces: each chord is an arc cut
from the section's fitted loop. Where the op drops the edge a germ site
sat on, the two chords either side of it meet at a valence-2 vertex on
one surface pair, and the join refuses there,
`JoinCarrierUnsupported { carrier: Nurbs, closed: false }`, raised in
`topo::boolean::edge_join::joined_spec` (the `param_near` arm).
`crates/sweep/tests/cylinder_sphere_frame.rs` pins 26 of its 54 runs at
this door (the `Door::Join` rows). The two chords are cut from one
carrier over abutting intervals (`chord_join::cut_arc`), so their nets
meet on a shared knot value: the run-on is their concatenation, the
joint knot at multiplicity the degree, and its parameter foot is the
far chord's own end parameter.

Moved from SPLICE to SPLICECARRIER on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
