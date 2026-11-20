---
id: the-in-arc-pairing-turns-the-pole-arcs-lane-gap-into-an-indeterminate-at-eps-1e-6
kind: issue
title: Since PR 6832 the tilted sphere pair's arc through the pole refuses Escalated Indeterminate on pcurve_projected_sector at eps 1e-6, not the azimuth lane gap its row pins, so main is red there
status: closed
opened: 2026-11-12
priority: P1
cost: E
closed: 2026-11-20
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

## The pin comes off: the arc builds (CHARTTAIL, 2026-11-20)

`charttail/pole-section` moves this row. The arc that escalated here does
not pass through the pole: it ENDS on it, at a vertex the join already
leaves there (`work/charttail/a-section-circle-through-a-sphere-faces-pole-mints-no-pcurve.md`).
The `Pose` refusal at 1e-9 and this escalation at 1e-6 are the same
reading. The piece holding that end has a radial hull whose lower end
sits on zero, decided negative at 1e-9 and read in band at 1e-6. Neither
was a fault of the in-arc pairing. With the producer stating the pole
end, the image reads its azimuth there off the reduced radial
(`geom_brep::PoleEnd`). No piece holds the axis any more, and the union
builds at its closed form at 1e-6, 1e-9 and 1e-12.

The test is renamed
`tilted_sphere_pair::a_section_passing_near_a_pole_or_through_it_builds`,
and its `pinned_red` wrapper is gone. Neither of this row's two
alternatives is taken: the arc is neither left on the gap nor pinned on
the new refusal. The row has nothing left to decide once that PR lands.

## Closed (2026-11-20)

By PR 7180: an arc that ends on a sphere chart's pole images through the projected lane's pole-ended piece (its azimuth read off the reduced radial part `g`, its end decided from the vertex's incidence, the envelope charged a pole term). The plane-through-pole cap, its subtract at (1,2,0) and the two-ball radical circle build at their closed forms at ε 1e-6, 1e-9 and 1e-12. The section was already split at the pole; the gap was the lane's.
