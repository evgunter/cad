---
id: ssi-r3-a-state-landing-in-band-outside-the-slab-escalates-the-open-end
kind: issue
title: ssi/march: on the ℝ³ lane a step whose settled end lands in the band outside the slab escalates the open end, where a step shortened to the face would decide it
status: open
opened: 2026-10-05
priority: P2
cost: M
---


(SSI implementer on `ssi/neighbour-cap`, PR 4034.)

## What

`SlabExit::after_step` (`ssi/march.rs`) decides `ssi_branch_open_end`
on the march's settled next state. Where that state lands within the
band's escalate edge of a slab face, outside or inside, the decision
escalates (`TraceDecision::BranchOpenEnd`) and the whole op refuses,
though the branch plainly crosses the face. A branch that runs into a
face at a shallow angle crosses the escalation zone over a length of
the zone's width over the angle, and its steps land in it often: this
is a systematic defect of deciding the end at a step's state, not one
step's luck.

The clipped north loop of
`a_seed_settled_outside_the_slab_is_no_branch_and_the_arc_is_still_found`
(`tests/m5_pr7_ssi.rs`), its slab's top face swept over ±5 mm in 601
positions (`probe2_slab_face_sweep` on
`analysis/neighbour-cap-review/4034-r2`), 423 of which cut the loop:

| ε | main 396e5a8843 | PR 4034 |
|---|---|---|
| 1e-6 | 153 escalate | 225 escalate |
| 1e-9 | 0 | 2 |

PR 4034's residual test keeps shorter steps near the face than main's
curvature-sized ones, so more of them land in the zone. Halving a step
whose predicted end lands in the zone (tried in PR 4034) moved 105
of the escalations to `ssi_step_progress`, where the march near the face
could not step out of the zone at all; doubling it moved none. The row
stands down at ε 1e-6, naming this file.

## Done when

A march whose step crosses a slab face ends at the face whatever band
its settled end lands in (for example by deciding the branch's end at
the face crossing the boundary search finds, rather than at the step's
end), the sweep escalates at none of its 423 cutting positions at
ε 1e-6, and the row certifies there.
