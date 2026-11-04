---
id: joining-a-spline-carrier-is-unbuilt
kind: issue
title: A curved join on a spline carrier refuses JoinCarrierUnsupported: the kept edge cannot be run on along a carrier with no period or parameter inverse
status: closed
opened: 2026-10-07
priority: P0
cost: H
branch: splice/spline-join
closed: 2026-11-03
pr: 6434
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
one surface pair. The join refused there,
`JoinCarrierUnsupported { carrier: Nurbs, closed: false }`, raised in
`topo::boolean::edge_join::joined_spec` (the `param_near` arm), on 26 of
`crates/sweep/tests/cylinder_sphere_frame.rs`'s 54 runs, until PR 6013
left that vertex standing (below). The two chords are cut from one
carrier over abutting intervals (`chord_join::cut_arc`), so their nets
meet on a shared knot value: the run-on is their concatenation, the
joint knot at multiplicity the degree, and its parameter foot is the
far chord's own end parameter.

Moved from SPLICE to SPLICECARRIER on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

## No sweep row reaches it since PR 6013 (GREEN lane, 2026-11-02)

PR 6013 (FOREIGN's C⁰-knot split) added `edge_join::crease_at`. A
vertex where both edges crease (a spline piece at its clamped end, or at
an interior knot of full multiplicity) is never joined. The two chords
above are clamped pieces cut end to end from one loop, so their vertex
is now left standing and the join is not attempted. On main
`a285cdbf`, at every ε row, none of `cylinder_sphere_frame`'s 54 runs,
`conic_edge_curved_face`'s six ball runs, `torus_doors_join`'s five
rows or `germ_tilted_rod`'s other-spins table refuses
`JoinCarrierUnsupported` any more. Each passes the join and stops at the
result gate's volume, owed on SPIRICARM. Those rows are re-pinned there.
A producer that reaches this door now needs a spline kept edge beside an
edge that does not crease at the vertex.

Moved from SPLICECARRIER to SPLICE on 2026-11-02, by `git mv`, when SPLICECARRIER merged back into SPLICE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the four tracks of SPLICE's 2026-10-28 split came in under one budget.

## Closed (SPLICE, PR 6434, 2026-11-03)

The curved join's far parameter has a first rung for every carrier kind: where the killed edge's carrier is the kept edge's by content (`geom::Curve3::same_content`, exact, every field of every kind destructured) and both stand at one parameter at the vertex, the killed edge's own far end is the joined edge's far parameter, with no inverse and no period. The period and `param_near` remain for distinct carriers of one locus, and a spline beside a different carrier refuses `JoinCarrierUnsupported` (`a-spline-join-beside-another-carrier-has-no-far-parameter`, P2, no producer). A split pair joined back is now the parent's interval exactly on every kind; on main a circle (15 of 63 split fractions), an ellipse (3 of 63) and the spiric joined back off it by a few ulps. No public boolean reaches the door today; `Body::split_edge` then `Body::join_edges` does (`join_door_is_whole`, `a_split_edge_joins_back_on_its_carrier`). Filed: `a-germ-site-is-footed-on-its-section-again` (the `join.rs` lead).
