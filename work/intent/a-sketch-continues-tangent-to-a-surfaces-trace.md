---
id: a-sketch-continues-tangent-to-a-surfaces-trace
kind: unit
title: INTENT stage 6 PR C: a sketch continues tangent to another surface's trace in its plane
status: review
opened: 2026-10-29
priority: P0
cost: H
design: true
pr: 6043
---


INTENT stage 6, PR C. Spec: `docs/INTENT-STAGE6-SPEC.md` §4.

D10 `[coincidence]`: "tangency is constructed (a sketch may read another surface's trace in its plane and continue tangent to it)". Ev, 2026-10-03: "setting a slope in the sketch plane based on a derivative of a shape as it appears sliced by the sketch plane". Nothing builds it: no node, pose or profile step reads another body's surface, and a profile reads no plane (stage 3 D; `ProfileProgram`'s doc).

`design: true` for **FORK-S6-2** (what a sketch reads, given that a profile reads no plane, no construction reads a pose, a 2-D value is never a variable of a 2-D kind, and a construction reads no observed value; every option changes ratified text) and **FORK-S6-3** (which point of the trace a step continues from). Both are in spec §7. C lands the trace, the step, their refusals, names, persistence and Python; it changes no kernel decision, and the tangency row it makes stays unproven until D.
