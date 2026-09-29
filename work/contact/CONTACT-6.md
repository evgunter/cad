---
id: CONTACT-6
kind: unit
title: point_in_solid stops answering a false Out inside a tilted-cut cylinder cavity: trace the cause, then fix it at its home
status: closed
opened: 2026-09-28
priority: P0
cost: M
branch: contact/6-cut-cavity
closed: 2026-09-28
---


Carries `point-in-solid-reads-out-inside-a-tilted-cut-cylinder-cavity`.
Spec: `docs/CONTACT-6-SPEC.md`.

Review tier: set when the trace hands back. A fix that changes what a
certified walk answers goes to a dual; a local fix under a stated
invariant goes to a single full review.

## Closed

`split_finish` sets each section face's sense from its loop's winding
about the chart normal (`finish::section_sense`, through
`Body::planar_loop_winding`, the function check 6 uses). An unsigned
winding refuses as `SplitFinishError::SectionWindingUndecided`. A new
door, `Body::set_face_surface_and_sense`, re-charts a face and writes
its sense in one call. Blend surgery, step-import and the split now use
it, so no caller can re-chart and forget the bit.

Measured, base against head:
- the tilt-1.0 cavity: 1,274 false `Out` became 0 over six poses;
- the hole class (brick minus rod, the bored cylinder, flat and
  tilted): each upper half's `LoopRoleInverted` on the disc is gone;
- the two steep flipped cuts keep a cap residue identical at base,
  filed.

Review:
- A single full review returned APPROVE-WITH-FIXES with one MAJOR. The
  first fix's "both section faces `true`" was false for a section with
  a hole, and made both halves of a horizontally split bored body fail
  tier 3.
- The fix pass reads the winding instead, and adds the re-chart door.
- The orchestrator read the fix pass.

Filed on REACH:
- `a-split-through-a-bore-makes-a-square-and-a-disc-not-an-annulus` (P1);
- `split-hands-out-a-body-without-running-tier-3` (P2);
- `split-leaves-a-ringed-cap-fragment-invalid-under-a-steep-cut`.
