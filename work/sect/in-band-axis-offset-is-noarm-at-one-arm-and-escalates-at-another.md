---
id: in-band-axis-offset-is-noarm-at-one-arm-and-escalates-at-another
kind: issue
title: An axis offset in the band folds to NoArm at the cylinder × sphere frame and escalates at the parallel cylinder arm
status: closed
opened: 2026-10-04
priority: P1
cost: E
refs: [4031, cylinder-sphere-tangency-is-decided-twice-and-its-offset-computed-three-times]
rides_with: the-section-of-a-surface-pair-is-one-object
closed: 2026-10-27
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
`work/intent/d10-one-way-to-say-intent-is-unbuilt.md` is unbuilt.

## Decide

One policy for an axis offset in the band: escalate (fail loud, the
offer census can execute it) or fold into the arm's no-arm refusal (the
hold's "leave coaxial refusals as they are"). Then make both arms say
it.

## Decided (2026-10-27): escalate

Both SECT step-1 designers rule escalate, and say it is not a fork. C5's
`cs_coaxial` and the parallel-cylinder arm already escalate, E kept "the
sliver band still refuses", and only the transverse frame folds the band
into `NoArm`, which misnames the conditioning verdict as a missing arm.
That frame retires in `the-section-of-a-surface-pair-is-one-object`,
which this row rides with; the section door escalates.

## Closed

2026-10-27, with `the-section-of-a-surface-pair-is-one-object`. The
transverse frame retired; the pose escalates an offset in the band
(`cs_coaxial`, `SectionError::Escalated`), which the join refuses as
the section undecided, as the parallel-cylinder arm does.
