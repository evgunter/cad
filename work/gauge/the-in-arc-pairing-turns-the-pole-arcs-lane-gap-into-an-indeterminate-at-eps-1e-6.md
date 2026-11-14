---
id: the-in-arc-pairing-turns-the-pole-arcs-lane-gap-into-an-indeterminate-at-eps-1e-6
kind: issue
title: Since PR 6832 the tilted sphere pair's arc through the pole refuses Escalated Indeterminate on pcurve_projected_sector at eps 1e-6, not the azimuth lane gap its row pins, so main is red there
status: open
opened: 2026-11-12
priority: P1
cost: E
---


Found by BAND on PR 6520's CI, after main `6c70512339` was folded into the
PR's test merge. The PR's own head passes this row; main fails it.

`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --test all -E
'test(a_section_passing_near_a_pole_builds)'` fails
`tilted_sphere_pair::a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed`
at `crates/sweep/tests/tilted_sphere_pair.rs:578`. The row pins δ 0's union
refusing `PcurveCertifyError::NoGeneralLane { gap: LaneGap::Pose { channel:
SectorChannel::Azimuth, .. } }` (lines 578–596, "the general lane's own
gap, PCERT E1's P3"). At 1e-6 it now gets:

```
Err(Pcurves { source: Certify { half_edge: HalfEdgeKey(11v1), error:
  Escalated { check: GeneralLane, sample: 0, cause: Indeterminate { margin:
  Value(-2.1396e-6), band: Band { zero: 1e-6, escalate: 1e-5 },
  predicate: Some("pcurve_projected_sector") } } } })
```

Bisected on main's first-parent line: the row passes at `9a25634807^1`
(`d02dacd2a3`) and fails at `9a25634807`, the merge of PR 6832
(`gauge/in-arc-pairing`, which rewrote `crates/topo/src/boolean/insert.rs`).
So the arc now reaches the general lane and reads in band, where before it
stopped at the lane's declared gap. Either the pairing should leave this arc
on the gap, or the row's pin moves to the new typed refusal with a stated
reason that the in-band reading is the right one. Deciding which is this row.

## Confirmed and pinned (GREEN, 2026-11-14)

Re-bisected, first-parent, at ε 1e-6: green at `c44f50da6` and `d02dacd2a3`, red at `9a25634807` (PR 6832) and at main `b363f5fa1`, with the message above. The nightly passed it on 11-11 and has failed since 11-12.

The row is under `test_utils::pinned_red::row` at ε 1e-6 on the escalation (`green/eps-1e-6`). Take the pin off with the decision.
