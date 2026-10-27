---
id: a-spiric-elbow-a-hundred-kilometres-out-exhausts-the-quadrature-budget
kind: issue
title: The klein elbow hollow translated by 1e5 refuses QuadratureBudget at the reporting walk (width 7.9e-6 against target 1.0e-6 at round 13): the spiric lane's width grows with distance from the origin
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [4554]
---


Found by PR 4554's verifier on `d43a02f36a`, its NOTE 3, and measured.
The klein elbow hollow, translated rigidly by 1e5 m, refuses
`mass_properties` with `Face { 5v1, QuadratureBudget { width_len: 7.92e-6,
target_len: 1.024e-6, rounds: 13 } }`, on the reporting walk itself and
on both re-derive reads. At offsets 0, 1e1 and 1e3 it measures, with a
width of about 6.7e-7.

The verifier did not check whether main, or the review head
`74fd2a71`, behave the same. The first step is to measure that. Then
find which term grows with the distance from the origin: the
`(C − centre)·A⃗` flux lever, the cap's `c + n·d − ref` chord, or the
wall's absolute-frame trig. The far-carrier-origin class in
`a-planar-face-sums-its-area-about-a-far-carrier-origin` is the likely
neighbour.

Two smaller verifier notes go with this row:
- **Mutant 5** (dropping `1/f` from the u′ bound) is killed only by the
  synthetic-envelope row, with an 8% margin. A partial weakening such as
  `1/√f` would probably survive. A tighter row is wanted.
- **The untight pass never recentres.** Since the fallback was removed, a
  body with a spiric face never reads as recentred on the untight pass,
  so every undecided sign read on it pays for the tight pass. That costs
  time, not width.

(FLUX orchestrator)

## Update 2026-10-27: the refusal this row names is gone

PR 5741 (QUAD's measure door, unit C) removed `PropsError::QuadratureBudget`.
`mass_properties` now takes a required resolution and answers a bracket
whose `stop` is `Floor` or `Wall` when the resolution is not met. So
this body no longer refuses at the number door; what remains to
establish is whether its bracket is wide or stops at `Wall`. **First
step: re-measure on current main**, then either close the row as
overtaken or re-title it as the width it now reads.
