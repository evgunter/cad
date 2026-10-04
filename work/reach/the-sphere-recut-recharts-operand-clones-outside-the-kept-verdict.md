---
id: the-sphere-recut-recharts-operand-clones-outside-the-kept-verdict
kind: issue
title: The no-crossings sphere re-cut re-charts operand clones inside the door, which the operands' kept verdict never saw: unprobed
status: open
opened: 2026-10-03
priority: P3
cost: E
refs: [boolean-door-adopts-the-finished-body-type]
---


Found by PR 3987's dual review (F18), not exercised.

The boolean door takes finished operands, but the sphere re-cut
(`ops::boolean_op_recut`, `SphereRecut`) rigidly re-charts a sphere
group of an operand CLONE about the first escape plane's normal and
re-enters the pipeline (`recut = false`). The re-charted clone is a
body the kept verdict did not see: the type fence covers the API, not
the door's own internals. The result gate re-validates the output, so
a defect would refuse rather than ship, but a re-chart that breaks an
operand invariant the pipeline reads (description adjacency, pcurve
envelope) would surface as a classification-time refusal or invariant,
not as a typed operand refusal.

What closes it: a probe through the re-cut (a sphere group escaping
through a plane face, `m5_s13_*` poses) that validates the re-charted
clone at tier 3 before re-entry, run at all three ε; then either a
debug assertion at re-entry or the clone re-finished through the
at-rest gate.
