---
id: the-sphere-recut-recharts-operand-clones-outside-the-kept-verdict
kind: issue
title: The no-crossings sphere re-cut re-charts operand clones inside the door, which the operands' kept verdict never saw: unprobed
status: open
opened: 2026-10-03
priority: P3
cost: E
refs: [3987]
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

## Evidence (2026-11-23, ORBIT `orbit/one-sphere-escape-cut`, PR 7602)

The re-chart is retired, so no operand clone is rotated or grafted any more. Every escape is now cut in along a meridian of the ball's own chart (`apply_cut_ins`), and the pipeline re-enters on those cut clones. The row's concern survives for the cut. The cut clone is a body the kept verdict never saw, and the re-entry checks it only with a `debug_assert` of tiers 1 and 2 (`validate_closed`, in `through_the_join`), never tier 3 and never in release. The pipeline is renamed `boolean_pipeline`, and `NoCrossings::Recut` is now `NoCrossings::CutIn`. Still unprobed: tier 3 on the cut clone at all three ε.
