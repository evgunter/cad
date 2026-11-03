---
id: a-spline-join-beside-another-carrier-has-no-far-parameter
kind: issue
title: A join keeping a spline edge beside a piece on another carrier of its locus refuses JoinCarrierUnsupported: no far parameter carries over and a spline has no inverse
status: open
opened: 2026-11-03
priority: P2
cost: M
refs: [joining-a-spline-carrier-is-unbuilt]
---

Filed by the spline-join lane (`splice/spline-join`), as the residue of
`joining-a-spline-carrier-is-unbuilt`.

## What

The curved join (`topo::boolean::edge_join::joined_spec`) states the
joined edge on the kept edge's own carrier. Its first rung carries the
far parameter (`carried_far`): where the killed edge lies on the kept
edge's very carrier, compared by content, and stands at the kept edge's
parameter at the vertex, the killed edge's own far end parameter is the
joined edge's. Where the carriers differ, the far parameter is read off
the kept carrier instead: its period for a closed join,
`Curve3::param_near` for an open one. A spline has neither, so a kept
spline beside a piece on another carrier of the same locus refuses
`BooleanError::JoinCarrierUnsupported { carrier: Nurbs, .. }`. The
piece can be a line, a circle or another spline. The kept spline does
not crease at the vertex; a piece that creases there takes the killed
role (`crease_at`), as the witness's clamped piece does.

## Reach

Nothing produces it through the public boolean or any finishing door on
main. A split restates its parent's carrier verbatim, so its two
children always carry. Section chords are clamped pieces, so they
crease at both ends and their vertex stands. The one witness is
construction: a spline edge split, with one child restated on a
carrier of other values
(`crates/sweep/tests/a_split_edge_joins_back_on_its_carrier.rs`,
`a_spline_beside_a_piece_of_another_carrier_refuses_typed`, and
`join_door_is_whole.rs`).

## What it would need

A parameter on the kept spline for the killed piece's far end: a
certified foot on the spline (a projection with its interval), or the
two pieces restated as one spline. P2 until a producer reaches it.
