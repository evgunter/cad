---
id: the-tilt-u-newell-residual-is-the-next-wall
kind: issue
title: after rule F takes the tilt-u carrier wall the derived boss refuses on a newell_plane_residual straddle the tier does not prove
status: closed
opened: 2026-09-15
priority: P2
cost: H
closed: 2026-09-22
---


## What

Found by SYM-8's Phase 1, by execution
(`crates/editor-core/tests/m10_derived_frame_tilted_interval`'s
`sym8_phase1_the_tilt_u_wall_with_and_without_the_manifest_sign` and
`sym8_phase1_the_tilt_u_ladder`).

The tilt-`u` derived document — an authored
`Datum::Frame { u: (1,0,t), v: (0,1,0) }` with `t = 0.25 ± half`, a
cube extruded from it, a `FaceFrame` on its cap, a boss on that — is
the document SYM-5's rule E turned a DEGREE wall into a TERM wall on
and stopped at. SYM-8's rule F (the manifest sign) takes that wall:
under `ProfileLift::Guided` at `half = 1e-3` the refused
`carrier_endpoint_end` goes 24/0/0/1 → 33/0/0/0, every decision a
theorem.

The document still refuses, and on a different predicate:

```
node 5 — the extrude op refused: side plane at loop 0 segment 1:
newell: residual at vertex 0 escalated: predicate
'newell_plane_residual' indeterminate:
enclosure [-5.7440607169936285e-2, 5.7442724246194295e-2]
```

That is a plain STRADDLE, not a clause-1 `Invalid`, so it is not the
`work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`
defect the tilt-`v` document leaves at `5e-2`. The tier does not prove
it: the early form is 662 terms at degree 40 over 370 at degree 39,
with THREE frozen nodes on its decision path (`Mul`, `Sub`, `Add`), and
`frozen` for the replay is 720 against 37 with rule F off — the rule
un-freezes the carrier chain and the walk then builds what the freeze
used to cut off, right up to a new budget wall.

The split at that point is `newell_plane_residual` 32/0/0/1: 32 of the
33 decisions ARE theorems and one is not.

## What it is not

- Not a decision rule F lost. With the rule off the replay never
  reaches this site: it refuses earlier, at the carrier gate, and its
  `newell_plane_residual` is 24/0/0/0 over a smaller population.
- Not the value channel's. The enclosure is a straddle the numeric
  channel cannot decide, not a domain refusal.
- Not the budget's, at first reading — but that is untested here:
  nobody has re-run this document at 4,096 / 65,536 with rule F on, and
  that is the first thing the unit that takes this row should do
  (SYM-5 did exactly that for the tilt-`v` wall and it settled which
  kind of wall it was).

## Where the fix lives

`geom_core::sym`, the same tier. SYM-8 stops here by its spec's own
stop condition — the wall it was measured against fell, and this is the
next one, not the one it was drawn for.

## Home

SYM. Filed by SYM-8 at its PR (2026-09-15).

## ANSWERED by DECIDE-3 (2026-09-21)

The wall is gone. With rule G (the canonical square root at the one
mint site) and the decision read shipped, the tilt-`u` derived boss at
`half = 1e-3` under `Guided` CERTIFIES: `carrier_endpoint_end` is
32/16/0/0 (nothing numeric) and no refusal is left, where the row
`m10_the_tilt_u_derived_boss_stops_on_the_newell_residual_and_names_it`
used to assert a `newell_plane_residual` straddle. The row is renamed
`m10_the_tilt_u_derived_boss_certifies_once_the_read_settles_its_frame`
and asserts the certification; the F-off half is unchanged, so the day
rule F's own wall moves it still reds and says which.

Why the straddle went: the residual was over a frame whose candidate
norms the tier keyed as unrelated atoms. Rule G keys every root on its
argument's value class, so `S` and `sqrt(1/S²)` are one quantity again
and the sum cancels; the sixteen comparisons that are left are the
frame's conditioning floors, which no form settles and the read
certifies over the box.

This row closes with DECIDE-3's merge.

Closed at DECIDE-3's merge (#3039, 2026-09-22).
