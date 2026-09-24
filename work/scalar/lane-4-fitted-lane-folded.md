---
id: lane-4-fitted-lane-folded
kind: unit
title: LANE-4: PcurveFittedLane folds into a FittedLane door value answered by AtRestPolicy::fitted_lane(); lane_name becomes a per-scalar name source
status: open
opened: 2026-09-24
branch: scalar/lane-4
---


## What

H5 ruling 3's last trait. `PcurveFittedLane` (four methods, five impls,
66 production bound sites in five crates) becomes `geom_brep::FittedLane<T>`,
a door value of three fn-pointer fields, answered at the per-scalar seam
by `topo::AtRestPolicy::fitted_lane()` (LANE-0/LANE-3's split: the type in
geom-brep, the answer in topo); `lane_name` becomes a per-scalar name
source on the policy, passed to the geom-brep doors; `certify_at_dual`
becomes a `compile_fail` doctest plus a runtime `None` row; the door is
pinned in LANE-4P's shape. The `None` comes from the policy everywhere,
validators included (orchestrator's ruling, logged 2026-09-24): no
verdict moves. No type change: a `None` hook is a refusal before any
cache exists. Spec: `docs/LANE-4-SPEC.md` (deleted at merge). Survey:
`/home/user/scalar-briefs/survey-lane4.md`.

**Review tier: DUAL** (cross-crate; certification-rights semantics;
five crates' bound sites).
