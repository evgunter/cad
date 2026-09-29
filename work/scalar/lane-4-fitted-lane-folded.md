---
id: lane-4-fitted-lane-folded
kind: unit
title: LANE-4: PcurveFittedLane folds into a FittedLane door value answered by AtRestPolicy::fitted_lane(); lane_name becomes a per-scalar name source
status: closed
opened: 2026-09-24
closed: 2026-09-29
branch: scalar/lane-4
pr: 3194
---


## What

H5 ruling 3's last trait. `PcurveFittedLane` (four methods, five impls,
66 production bound sites in five crates: 63 items outside geom-brep,
the `AtRestPolicy` supertrait edge, and geom-brep's own fitted-door
`impl` block and `run_fitted_checks`) becomes `geom_brep::FittedLane<T>`,
a door value of three fn-pointer fields, answered at the per-scalar seam
by `topo::AtRestPolicy::fitted_lane()` (LANE-0/LANE-3's split: the type in
geom-brep, the answer in topo); `lane_name` becomes a per-scalar name
source on the policy, passed to the geom-brep doors; `certify_at_dual`
becomes a `compile_fail` doctest plus a runtime `None` row; the door is
pinned in LANE-4P's shape. The `None` comes from the policy everywhere,
validators included (orchestrator's ruling, logged 2026-09-24): no
verdict moves — an absent door refuses at the fitted lane's check 4,
where the base refused, so checks 1–3 answer as at every scalar
(fix pass, 2026-09-25). No type change: a `None` hook is a refusal before any
cache exists. Spec: `docs/LANE-4-SPEC.md` (deleted at merge). Survey:
`/home/user/scalar-briefs/survey-lane4.md`.

**Review tier: DUAL** (cross-crate; certification-rights semantics;
five crates' bound sites).

## Closed (2026-09-29) — PR 3194

`PcurveFittedLane` is gone. The fitted door is `geom_brep::FittedLane<T>`,
which `AtRestPolicy::fitted_lane()` answers alongside `scalar_name()`, and
every consumer takes it from the policy, validators included. Dual review
(two Opus reviewers, APPROVE-WITH-FIXES each). R2 raised one MAJOR by
execution, which was upheld: the spec's up-front refusal for a `None` door
was observable through `nurbs_iso_derive`'s own-chart arm, where a stated
`General` image reaches `mint_face` directly. At `Dual64` a public-API
mint moved from `Ok` to `FittedLaneUnsupported`. The fix pass:
- restored the base's order in both fitted arms (`certify_general` takes
  `Option<FittedLane<T>>` and refuses at check 4; spec clause 3 amended);
- committed R2's body as `topo/tests/stated_general_image_mint.rs`, with
  rows at all five scalars;
- made the `m6_2` dual row run the dual, which turns R2's mutant red.

R2's table now reads identical from base to head, and the fix pass took
the reviews' prose and claim fixes and filed four rows. Main (about 2,190
commits) was merged at `fa6c01041a` with five conflicts resolved per hunk
and nine new trait sites on main re-spelled or dropped. Head `c8c6c107d0`,
run 36544026365, green. This is the first SCALAR dual-review tally
candidate (MINT-ORDER).
