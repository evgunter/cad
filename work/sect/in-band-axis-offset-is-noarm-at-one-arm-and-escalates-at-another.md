---
id: in-band-axis-offset-is-noarm-at-one-arm-and-escalates-at-another
kind: issue
title: An axis offset in the band folds to NoArm at the cylinder × sphere frame and escalates at the parallel cylinder arm
status: open
opened: 2026-10-04
priority: P3
cost: E
refs: [4031, cylinder-sphere-tangency-is-decided-twice-and-its-offset-computed-three-times]
design: true
---


Found by PR 4031's review (S5).

## What

Two arms decide "how far is this axis from that centre or axis" with a
length margin, and treat the band differently:

- `boolean::join::cs_transverse_frame`, `bool_germ_frame_cs_offset`
  (the sphere centre's distance from the cylinder axis): `Zero`, and an
  in-band verdict too, fold into `FrameError::NoArm`, the pair's
  `GermFrameUnsupported` refusal;
- `boolean::join::parallel_radical_plane`, `bool_join_cc_axis_offset`
  (the axis-to-axis offset of a parallel cylinder pair): `Zero` keeps
  the join's `CurvedBooleanUnsupported`, and an in-band verdict
  escalates `Coincidence(Section, Moot)`, an undecided coincidence that
  the offer census executes (`parallel_axes_offset_in_band`).

Both in-band cases are coaxial ground, unreached through a public door
(the coincidence ladder and the crossing layer meet such poses first),
and held under D10 while
`work/recipe/d10-one-way-to-say-intent-is-unbuilt.md` is unbuilt.

## Decide

One policy for an axis offset in the band: escalate (fail loud, the
offer census can execute it) or fold into the arm's no-arm refusal (the
hold's "leave coaxial refusals as they are"). Then make both arms say
it.
