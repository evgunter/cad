---
id: a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane
kind: issue
title: A torus wall trimmed by a spline rim has no volume lane: the re-imported spiric cavity refuses check 7 there
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Filed by SECT's `sect-toruscert` lane (branch `sect/torus-certificate`,
2026-10-27). Once the edge certificate's uniqueness tube had a torus arm,
the STEP round trip of the sectioned vessel's cavity got one door further.

## Measured

`crates/step-import/tests/spiric_roundtrip.rs`
`a_spiric_rim_exports_as_a_spline_and_its_reimport_door_is_pinned`: the
cavity's two spiric rims export as cubic splines (`uncertainty_m = 1e-4`).
On import, each spline's `Intersection` reading now adopts: limb 2 holds
against the plane and the torus, and the tube certifies. The import then
refuses at tier 3:

`TierInvalid { errors: [VolumeUncomputable { source: Face { face:
FaceKey(6v1), source: QuadratureUnsupported { what: "conic trim on a
cone/sphere/torus chart — ..." } } }] }`

The face is a torus wall bounded by the adopted spline. `crates/topo/src/props/quad_lane.rs`
routes a torus wall to `rim_face` only when a `Curve3::Spiric` is on one
of its loops. A wall whose rim is a spline falls through to the
cylinder-only flux algebra below that, which refuses every non-cylinder
chart.

## What the taker owes

A volume lane for a torus wall trimmed by a spline rim, or a routing of
that wall to the spline-rim arm the plane cap already takes. Then
re-pin the round-trip row on whatever the import does next.

## A boolean reaches it too (GREEN lane, 2026-11-02)

`torus_doors_join::a_rod_off_the_axis_line_stops_at_the_join`, the donut
`∩` a radius-0.2 rod at `(1.9, 0.5)`, passes the spline join since
PR 6013. On main `a285cdbf`, at every ε row, it refuses at the result
gate's volume with `QuadratureUnsupported { "conic trim on a
cone/sphere/torus chart …" }` on a torus face its chords trim (`Stop::ConicTrim`).
It is pinned there, citing this row. `germ_tilted_rod`'s other-spins
pin cites this row beside the projected-boundary one, since its
`quadrature volume` stops do not say which chart refused.
