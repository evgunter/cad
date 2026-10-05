---
id: ssi-r3-a-state-landing-in-band-outside-the-slab-escalates-the-open-end
kind: issue
title: ssi/march: on the ℝ³ lane a step whose settled end lands in the band outside the slab escalates the open end, where a step shortened to the face would decide it
status: open
opened: 2026-10-05
---


(SSI implementer on `ssi/neighbour-cap`, PR 4034.)

## What

`SlabExit::after_step` (`ssi/march.rs`) decides `ssi_branch_open_end`
on the march's settled next state. Where that state lands outside the
slab by less than the band's escalate edge, the decision escalates
(`TraceDecision::BranchOpenEnd`) and the whole op refuses, though the
branch plainly leaves the slab. Where the state lands is the step's
luck: the clipped north loop of
`a_seed_settled_outside_the_slab_is_no_branch_and_the_arc_is_still_found`
(`tests/m5_pr7_ssi.rs`) certifies at ε 1e-9 and 1e-12 and on main at
1e-6, and with PR 4034's residual-kept step its last state lands
2.76e-6 m outside the face at ε 1e-6, in the band. The row stands down
there, naming this file.

## Done when

A march whose step leaves the slab ends at the face whatever band its
settled end lands in (for example by halving the leaving step until
its end decides, or by deciding the branch's end at the face crossing
the boundary search finds rather than at the step's end), and the row
certifies at ε 1e-6.
