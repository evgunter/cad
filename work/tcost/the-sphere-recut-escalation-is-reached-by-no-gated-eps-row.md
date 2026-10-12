---
id: the-sphere-recut-escalation-is-reached-by-no-gated-eps-row
kind: issue
title: the sphere recut's mapped-source escalation arm (RECUT_MAPPED_ENCLOSURE_HI) is reached by no gated eps row
status: open
opened: 2026-09-25
---


Found by PATHS' `geom-brep-sketch-segment-full-turn` (#3254). The file
is shared with TINT (`territory` names tcost and tint); filed here.

## What stands

`sweep/tests/m5_s12_curved_ops_interval.rs`'s
`interval_sphere_subtract_decides_definitely_after_the_recut` and
`review_arceval_r1_probes.rs`'s
`e2_recut_escalation_hi_is_pinned_to_the_measured_constant` share
`common/sphere_recut.rs`'s constants. Each has an arm for
`ε < RECUT_DECIDES_FROM`: the plate − ball recut escalating on
`carrier_matches_mapped_source`.

A split edge's description keeps its parent's authored arc, carrier
included, and narrows only the sub-range it covers
(`MappedCurve::restrict`; PR 4491 moved the restriction there from
`SketchSegment`). The chain's enclosure therefore fits inside ε at every
gated row (1e-6, 1e-9, 1e-12), and the recut certifies at all three.

Measured with `CAD_TOLERANCE_EPS` at PR 4491's head:
- it certifies at 1e-13 and above (also 1.5e-13 and 2e-13);
- at 7e-14 it escalates with `hi = 7.494860650341783e-14`;
- at 5e-14 it escalates with `hi = 5.5528024056952343e-14`;
- at 3e-14 it refuses with `ClassificationInvariant` ("re-cut rotation
  failed to re-certify");
- at 2e-14 and 1e-14 the ball itself is not finished (`SliverDihedral`,
  `hi = 2.26e-14`).

Main before PR 4491 escalated at 1e-13 with `hi = 1.0653e-13`. The rows
then pinned 1.0679e-13, so they were already stale at that ε. PR 4491's
window form tightens the chain enough to certify there. The rows are
re-aimed:
- `RECUT_DECIDES_FROM = 1e-13` splits the definite arm from the
  escalation arm;
- `hi` is pinned bit-exactly at `RECUT_PIN_EPS = 5e-14`, the ε it was
  measured at;
- elsewhere in the arm `hi ≤ 1.25·ε` holds (measured 1.07–1.11·ε),
  because `hi` tracks ε.

Below 5e-14 neither row makes a claim.

**No gated ε row reaches the escalation arm.** It runs only when someone
sets `CAD_TOLERANCE_EPS` to 5e-14 or 7e-14 by hand.

## What is owed

A decision: either give the arm a row that reaches it (a per-process
`Tolerance::init` at 1e-13, which holds under nextest's one process
per test but not under `cargo test`), or accept that it is documentation
of a sub-matrix ε and say so there.
