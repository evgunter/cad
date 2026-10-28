---
id: a-torus-faces-convention-is-decided-where-the-face-is-admitted
kind: issue
title: A torus face's convention (tube radius and ring gap each clear of the band) is decided where the face is admitted, required by the torus arm and re-decided at rest — waits on the import-validation question
status: open
opened: 2026-10-15
priority: P3
cost: M
design: true
refs: [band-scale-torus-geometry-ends-as-a-kernel-defect, can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches]
---


## What

Decision 2 of `[ev]` PR 5113, split out at Ev's request ("can we split
out the part that waits on the import conversation from this PR? i
don't want to lose track of it but i also don't want to hold this PR
indefinitely", 2026-10-15).

Today `chart_pcurve`'s torus arm decides the face's convention (a tube
radius `r` and a ring gap `R − r` each clear of the band) per row, and
a band-scale tube or a near-horn torus ends as `Unsized::Defect`. The
two designers on fork-log row 142 recommended deciding it where a
torus face is admitted (`set_face_surface`, revolve, import), ending in
`TorusConvention::{Tube, Ring}.sized()`, required again by the torus
arm before its class gates, and re-decided at rest by tier 3, so a row
reads `r` and `R − r` only as decided data.

## The question for Ev

Whether a face's convention is decided at admission and re-decided at
rest depends on how import validation works: Ev asked (5113, decision
2) why re-deciding is necessary, and tied the answer to the
import-validation design question
`exch/can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches`.
This waits on that conversation.
