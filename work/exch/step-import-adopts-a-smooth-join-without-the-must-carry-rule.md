---
id: step-import-adopts-a-smooth-join-without-the-must-carry-rule
kind: issue
title: the adoption ladder offers no conventional rung for an under-determined tangency between distinct surfaces, so a native body with one does not re-import
status: open
opened: 2026-10-09
priority: P1
cost: M
refs: [split-tangent-chord-mints-tangency-without-the-must-carry-rule]
---

## Finding

Found by the §5 sweep of the PR that routed the split's tangent
section chord through `geom_brep::must_carry_over_edge`
(`work/tang/split-tangent-chord-mints-tangency-without-the-must-carry-rule.md`).
That rule is the one home of what a definitely-smooth join stores:
jet-determinate ⇒ `TangentIntersection`, under-determined ⇒ a chart
image, in band ⇒ a typed refusal.

`crates/step-import/src/adopt.rs`'s edge ladder (the candidates loop
that ends in `StepImportError::Adoption`) does not ask it. It offers
`Intersection`, then `TangentIntersection`, then a conventional
`MappedCurve` rung only in two cases. One is the NURBS rim
(`nurbs_rim`, from `nurbs_plane_pair`: a plane against a NURBS wall).
The other is two surface records that COINCIDE (the M7-2 rung, which is
right to refuse to offer a chart image wherever an intersection
refuses). So a smooth join between two distinct, non-coincident
analytic surfaces that the rule calls under-determined has no
candidate that certifies. The import then refuses a body the kernel
built tier-3 valid.

## Measured

The native sweep mints such joins today, through the extrude's own
must-carry call. PR 4394's review measured the declared-tangent
stadium: a profile whose line–arc joints are tangent joints, extruded
`h`, written by `step_export::step_string` and read back by
`step_import::import_step` with default options. The wall–wall line
edges store a chart image wherever the rule answers under-determined.
At `h = 0.5·√(2ε)` the import refuses `Adoption` at every ε row, and at
1e-6 it refuses even at `h = 1e-3`. This is on main, independent of
that PR (probe `probe_step_round_trip_of_a_thin_tangent_joint_extrusion`
on branch `tang/split-chord-review-probes`).

PR 4394 adds a second producer: a split tangent to a curved wall along
a ruling. The rounded shoulder of `crates/sweep/tests/wedge_end_doors.rs`
extruded `h` and split by `shoulder_cut()` (`y = 1`) gives a `y < 1`
piece whose seam is a chart image on the section plane wherever
`h²/2 ≤ ε`. At `Tol::witness()`:

| h | outcome |
|---|---|
| 1, 1e-3 | imports |
| 4e-5, 1e-5 | `Adoption { attempts: [Intersection: NotTransverse (Zero), TangentIntersection: NotSecondOrderSeparated (Zero, 8e-10 / 5e-11)] }` |

`import_step`'s own contract says files `step_string` writes from
finished kernel bodies import cleanly.

## Fix shape

The rung for a definitely-smooth edge between two distinct surfaces is
the rule's answer over the carrier: `must_carry_over_edge` read at ε on
the resolved surfaces, `Conventional` offering the chart image
(certified like any candidate, so a wrong surface still refuses), an
in-band verdict refusing typed rather than falling through. Which
adjacent chart, and how ε_in enters (D7: ε_in interprets, ε builds),
is this program's call.
