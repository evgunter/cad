---
id: rod-minus-brick-minus-slab-refuses-solids-do-not-cross
kind: issue
title: rod minus brick minus a slab across z in [3.5, 4] refuses 'the solids do not cross' although they overlap
status: open
opened: 2026-10-03
priority: P2
cost: M
---


Filed from PR 3984's dual review (r2 NOTE 7; identical on main, cause
unmeasured).

## Finding

Reviewer r2's probe (`analysis/reach-dual/3984-r2`,
`probes/reach-dual3984-r2/probe_3984_r2.rs`,
`b_conic_arm_builds_every_op_at_the_closed_form`): a circle-edged rod
minus a brick placed at `d = 0.0137`, `z ∈ [0.7, 3.3]`, then minus a
slab across `z ∈ [3.5, 4.5]` (scaled by `s`), refuses with "the solids
do not cross" although the slab takes `z ∈ [3.5, 4]` of what is left.
`point_in_solid` on the intermediate body answers
`WallOutlineUnsupported`. Identical at the merge base; the probe's
other 107 outcomes build at the closed form.

## Next

Measure first: run the probe's case alone, read the refusal's raising
site and payload, and find which door decides "do not cross" on an
overlapping pair.

## 2026-10-05 — the symptom text is gone, the cause is not

TOPO's PR 4055 dropped "the solids do not cross" from
`BooleanError::Containment`'s `Display` (it now reads "the Boolean
{e}"). So the symptom sentence in this row's title no longer
reproduces. Its cause does: the refusal is still `Containment`
carrying `WallOutlineUnsupported`, and nothing in that PR touched what
raises it. The row is still live; look for `Containment` and
`WallOutlineUnsupported`, not the old sentence.
