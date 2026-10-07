---
id: the-past-the-ceiling-row-replays-only-validation-on-the-bracket-and-pad
kind: issue
title: The past-the-ceiling row replays only the profile's validation on the bracket and the pad
status: closed
opened: 2026-10-06
priority: P3
cost: E
closed: 2026-10-07
refs: [SYM-16, ignored-sym-receipt-rows-drifted-red-on-main-unattributed]
---


`sym11_the_exact_channel_never_contradicts_past_the_ceiling`
(`editor-core/tests/sym11_exact_channel_rows.rs`) replays each measured
document at `Study::refuses_at`, "where every leaf's residuals are the
widest the corpus produces", and asserts that the exact witness never
contradicts its own form there. Since #3774 (`33e5000fb8`) the bracket's
and the pad's `refuses_at` is the scale at which the profile's
validation refuses (`arc_span` at 7.624e2·ε, `line_span` at 2.7783e3·ε).
So past the ceiling, those two replays stop before the extrude.

Measured by SYM-16's fix pass, release, at ε = 1e-6, 1e-9 and 1e-12
(`[symbolic_zero, registered, numeric, frozen]`):

| document | past the ceiling | at `certifies_at` (`Study`) |
| --- | --- | --- |
| r2_filleted_bracket | `[644, 0, 516, 806]`, refuses at validation | `[1401, 49, 1050, …]`, whole |
| r2_rounded_pad | `[368, 0, 302, 302]`, refuses at validation | `[1340, 54, 1272, 3138]`, whole |

Before #3774 both replays reached the extrude, stopping at the
attachment gate and at `pcurve_envelope`. The row then read 574 and
1186 `numeric` decisions on them, and 149 and 152 registrations. It now
reads 516 and 302 `numeric` decisions, and no registration. Nothing is wrong in the code (the item's "#3774"
section): the row's scale is the measured refusal, and that refusal is
now the earliest stage there is.

What the stop clause still covers on these two documents is the
validation prefix. Their extrude and pcurve residuals are covered only
at `certifies_at`, by the gating row, and at a scale up to 2× below the
new ceiling. `Study::certifies_at` is still M10-9's bracket end
(3.870e2·ε and 2.083e3·ε), so it was not re-measured against the new
refusals.

**Remedy.** Choose the row's scale per document so that the replay
reaches every stage at its widest. For example, measure the certifying
end of the new bracket and replay just below the refusal, or replay at
the widest scale at which the extrude still runs. Then re-take the two
rows.

## Outcome

Resolved by the shallow arc×arc corner P0
(`work/paths/arc-arc-shallow-corner-legs-escalate-arc-span`), not by
moving the row's scale. The validation refusals that stopped both
replays were span readings in band at a candidate whose segment's
nearer end reads on the other carrier. Validation now settles
those by the segments' ends, so at the unchanged `refuses_at` the
replays reach every stage again. Measured in release at ε = 1e-6, 1e-9
and 1e-12, identical at all three:

| document | past the ceiling, now | where it stops |
| --- | --- | --- |
| r2_filleted_bracket | `[1380, 48, 990, 1811]` | nowhere: it replays whole |
| r2_rounded_pad | `[1196, 59, 1067, 2805]` | the extrude's `pcurve_envelope` |

The evidence probe `m10_9_ceilings_with_and_without_the_door`
re-measures both ceilings unchanged. The bracket's is 7.622e2..7.624e2·ε,
over-band `arc_span`. The pad's is 2.778e3·ε, over-band `line_span`
and `pcurve_envelope`. The row costs about 20 s in release, so it moved
to the slow set.
