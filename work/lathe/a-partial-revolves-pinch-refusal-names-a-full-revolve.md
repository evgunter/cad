---
id: a-partial-revolves-pinch-refusal-names-a-full-revolve
kind: issue
title: A partial revolve of an arc running cusp to cusp refuses with the full revolve's pinch sentence, whose recourse is the partial revolve it already is
status: open
opened: 2026-11-22
priority: P3
cost: E
---

Found by LATHE's one-side-of-the-axis lane (branch
`lathe/one-side-of-the-axis`).

`revolve::axis::classify_segment` refuses an arc whose two ends are
both its horn circle's cusp (`interior` Zero, both ends pinned) with
`RevolveError::NonManifoldAxisContact` for every revolution, partial
included ("its face reaches the cusp from both sides and pinches there,
however far it is revolved"). The variant's sentence
(`RevolveError::write`, the `NonManifoldAxisContact` arm) was written
for the full revolve's own pinch: "a full revolve of loop 0 would pinch
at vertex 0, where the loop touches the axis outside an on-axis
segment. Recourse: lay that stretch of the loop along the axis, or use
a partial revolve".

Measured: a full circle of radius 1 centred `(1, 0)` with its vertex at
`(0, 0)`, revolved `Partial(±1.0)` about the sketch `y` axis, refuses
with that sentence. The revolve is already partial, so the sentence
names an operation the author did not ask for and its second recourse
is the input as written. The arc-arm site wants its own sentence (the
arc runs from its circle's cusp on the axis round to it again, so its
face pinches there however far it turns; recourse: end the arc short
of the cusp, or move the circle clear of the axis), or its own variant.
