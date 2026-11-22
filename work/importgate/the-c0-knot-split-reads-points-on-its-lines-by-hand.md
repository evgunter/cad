---
id: the-c0-knot-split-reads-points-on-its-lines-by-hand
kind: issue
title: the C0-knot split reads whether a point lies on its iso line, its seam column or a crease by hand, beside the kernel's point-on-span reading
status: open
opened: 2026-11-18
priority: P2
cost: E
---


Found by the §5 sweep of `importgate/singularity-meet`, which moved the
singularity pass's meetings onto `geom_brep::curve_pair::meet` and
deleted its hand-written point-on-carrier test (`param_on`: a foot and
`|C(t) − x|` read Zero at the band).

## What

`crates/step-import/src/knot_split.rs`, `split_face_once` (as of the
`importgate/interp-layer` head), decides three point-on-curve questions
by a projection and a raw comparison against the band's escalation, with
no named decision and no in-band arm:

- `on` (about :983): a vertex lies on an iso column (the knot's iso
  curve, or the chart's seam column) where `column.project(p)`'s
  `distance <= band.escalate()`. Read through `along` it only proposes a
  pcurve the certificate then decides, but on the seam branch
  (`on(&seam, pa)`) it decides the seam edge's parameters outright;
- `side_at` (about :967): a point's side of the knot line is read only
  where its chart foot's `distance <= band.escalate()`, then by raw
  `across < knot` / `across > knot`;
- the crease snap (about :1084): a meeting `meet` placed inside a spline
  edge is moved onto a crease knot's point where
  `(q - m.point).norm() <= band.escalate()`.

Each takes the escalation as the "on" threshold, so a distance in the
band reads on; `[verdicts]` has it escalate. The kernel's door for a point
on a carrier span is `geom_brep::curve_pair::Span::read` (On / End /
Carrier / Off, a refusal in band), which the singularity pass now reads
its singular points through.

## Fix

Read each through `Span::read` (or `k_stats::decide` under a named
reading) at the file band, refusing in band as the split's other
placements do (`StepImportError::C0KnotUnsplit`).

## Also

`knot_split.rs`, `split_face_once` (about :1051): the edge's meeting
with the knot line, `curve_pair::meet(...).map_err(|_| refuse(...))`,
folds every `MeetRefusal` arm into one sentence. An escalation (a
distance the file's band cannot call, whose recourse is an ε_in), a
reading that did not run to an answer, and a kernel fault all end the
same way. The singularity pass's `meet_refused` reads them arm by arm
(`MeetFault::kind`); the split should too. (From the review of PR 7117.)
