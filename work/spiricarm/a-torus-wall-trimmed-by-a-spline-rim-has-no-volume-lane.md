---
id: a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane
kind: issue
title: A torus wall trimmed by a spline rim has no volume lane: the re-imported spiric cavity refuses check 7 there
status: open
opened: 2026-10-27
priority: P0
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

`torus_doors_join::a_rod_off_the_axis_line_stops_at_the_conic_trim`, the donut
`∩` a radius-0.2 rod at `(1.9, 0.5)`, passes the spline join since
PR 6013. On main `a285cdbf`, at every ε row, it refuses at the result
gate's volume with `QuadratureUnsupported { "conic trim on a
cone/sphere/torus chart …" }` on a torus face its chords trim (`Stop::ConicTrim`).
It is pinned there, citing this row.

This row also owns two kinds of `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`
cell at β 0.5. One is `"conic trim volume"` (∩ at spins 0, 0.5 and 1), the same
conic-trim refusal on a torus face the chords trim. The other is
`"ring volume"` (∪ and the rod-first difference), the ring gate
(`MassPropsError::RingOnCurvedFace`, `topo::props`) refusing a trimmed torus
wall with no spiric that holds a ring. A lane for a spline-trimmed torus
wall has to read that wall's rings too.

## Re-banded P0 (2026-11-03)

Teapot wall 2's next stop. The handle's torus face is trimmed by the sphere × torus section's fitted chord, and `a-projected-boundary-on-an-analytic-chart-has-no-volume-lane` (P0, the sphere's arm, teapot wall 2's current stop) hands a conic trim on a torus face to this row. The donut ∩ rod above reaches it too. A blocker inherits the band of what waits on it (`work/README.md`, Priority); tracked in `work/issues/curved-demo-joins-build.md`.
