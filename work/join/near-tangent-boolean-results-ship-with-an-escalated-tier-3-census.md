---
id: near-tangent-boolean-results-ship-with-an-escalated-tier-3-census
kind: issue
title: Near-tangent booleans ship results whose tier-3′ census escalates: 139 runs on main, 72 more built by PR 4026 at 26 poses
status: open
opened: 2026-10-04
priority: P0
cost: H
refs: [boolean-door-adopts-the-finished-body-type, two-copies-of-a-pierce-carry-edges-that-run-within-the-band]
---


## What

Found by PR 4026's dual review (r1 m1, r2 M1), and measured by its fix
pass in release, against main `45dc18f9`.

Near-tangent poses build bodies with tier 2, the certificate and the
exact volume, but tier 3′ (`validate_pseudomanifold`) does not certify
them. Every finding is `CensusEscalated`: an `Indeterminate` margin in
the sliver band (zero 1e-9, escalate 1e-8), mostly on the census's
edge-edge predicates (`pm_census_ee_span`, `pm_census_ee_parallel`,
`pm_census_ee_overlap`, `pm_census_ee_gap`), and on some poses on its
edge-face ones (`pm_census_ef_residual`, `pm_census_ef_cut_gap`; PR
4038's review r2). In these poses the plane of a cube's face
lies 1e-5 to 1e-8 rad off a prism corner's edge, which leaves vertices
and edges of the result a few bands apart.

The batteries are:
- `crates/sweep/examples/r1_pierce_probes.rs` (`cube`, and with
  `R1_NT_D=1e-2,-1e-2,1e-4,-1e-4,1e-5,-1e-5,3e-6,-3e-6,1e-7,-1e-7,1e-8,-1e-8`);
- `crates/sweep/tests/join_pierce_r2_probes.rs` `r2_shapes_battery`.

Both are on PR 4026's review branches. Each BAD line carries
`t3=escalated`.

- **On main: 139 runs.** 66 notch307, 63 shallow200, 5 R315m, 1 R315,
  3 convex, 1 Lcvx. For example `R315m nf1_1e-6k1s1 cp U` and
  `R315m ne1_1e-6k2s-1 pc U`/`pc S`.
- **New with PR 4026: 72 runs at 26 poses** that main refused
  `JoinDesync`: 41 "a section vertex's null-edge copies have not
  exactly one kept end", 28 "every chord arc separates a loose
  scaffolding pair", and 3 "derived ring role order separates a loose
  scaffolding pair". The PR's new reach (the copy bookkeeping, the
  ring facing and the pierce weld) carries them to rest. Runs per pose:
- `R315m nf1_1e-6k2s-1` (3)
- `notch307 nt e0 a0 d1e-7` (1)
- `notch307 nt e0 a1 d1e-7` (1)
- `notch307 nt e0 a11 d-1e-8` (5)
- `notch307 nt e0 a3 d1e-7` (3)
- `notch307 nt e0 a3 d1e-8` (4)
- `notch307 nt e0 a8 d-1e-7` (2)
- `notch307 nt e0 a9 d-1e-7` (2)
- `notch307 nt e1 a12 d-1e-8` (2)
- `notch307 nt e1 a4 d1e-6` (3)
- `notch307 nt e1 a4 d1e-8` (4)
- `notch307 nt e1 a4 d3e-6` (3)
- `notch307 nt e1 a7 d1e-7` (1)
- `shallow200 nt e0 a0 d1e-8` (4)
- `shallow200 nt e0 a1 d1e-8` (2)
- `shallow200 nt e0 a10 d-1e-8` (3)
- `shallow200 nt e0 a3 d1e-6` (3)
- `shallow200 nt e0 a8 d-1e-8` (3)
- `shallow200 nt e0 a9 d-1e-8` (4)
- `shallow200 nt e1 a13 d-1e-8` (3)
- `shallow200 nt e1 a14 d-1e-8` (2)
- `shallow200 nt e1 a15 d-1e-8` (2)
- `shallow200 nt e1 a4 d1e-5` (3)
- `shallow200 nt e1 a4 d3e-6` (3)
- `shallow200 nt e1 a5 d1e-8` (2)
- `shallow200 nt e1 a7 d1e-8` (4)

None of them is a definite finding. The one definite finding among
the near-tangent poses, `shallow200 nt e0 a3 d1e-7`, is a census false
positive. It is filed on CONTACT
(`the-census-edge-edge-collinear-lane-reads-the-offset-at-the-long-edges-start`)
and pinned by
`join_pierce_runs_sweep::a_near_tangent_two_run_pierce_builds_with_edges_in_band_only_at_its_copies`.

The same pose at ε = 1e-12 (the 1e-7 tilt is then 1e4 bands; CI's
extra-ε pass) joins the class: main refuses both orders (`JoinDesync`,
"…not exactly one kept end"). The head builds prism ∪ cube and
prism ∖ cube at the oracle volume, tier 3′ passing. With the operands
swapped, cube ∪ prism and cube ∖ prism ship with `CensusEscalated` on
`pm_census_ee_span` (margin 6.69e-12 against an escalate edge of
1e-11). At ε = 1e-6 the 1e-7 tilt lies inside the band: both trees
record a vertex-vertex contact there, and every op passes, identically.

The census also passes, without escalating, an in-band edge pair at the
copies of many of these poses' neighbours (116 tier-3′-SOUND lines at
89 poses main refused). That is filed on CONTACT as
`two-copies-of-a-pierce-carry-edges-that-run-within-the-band`.

REACH's `boolean-door-adopts-the-finished-body-type` gates results at
tier 3′. It lists 11 other `CensusEscalated` results as a prerequisite;
this class is a larger instance of the same prerequisite.

**17 more runs, built by `join/pierce-pinch-families`** (the pinch
crossing, `zip::cross_pinches`, and the walk's facing in every op),
against main `81dde823` in release on `r1_pierce_probes cube` with the
`R1_NT_D` set above. Main refused each: 12 "every chord arc separates a
loose scaffolding pair" and 5 `Euler(SelfLoopEdge)`. Every tier-3′
finding on them is `CensusEscalated` (118 findings, `ee_span`,
`ee_overlap`, `ee_parallel`, margins 1.0e-9 to 5.5e-9), none definite.
All lie at poses listed above:
- notch307 `nt e0 a0 d1e-7` pc I; `nt e0 a1 d1e-7` pc I;
  `nt e0 a3 d1e-8` pc I, cp I; `nt e0 a11 d-1e-8` cp S;
  `nt e1 a4 d1e-8` pc I, cp I; `nt e1 a7 d1e-7` pc I;
  `nt e1 a12 d-1e-8` cp S;
- shallow200 `nt e0 a0 d1e-8` pc I, cp I; `nt e0 a1 d1e-8` pc I, cp I;
  `nt e1 a5 d1e-8` cp I; `nt e1 a7 d1e-8` pc I; `nt e1 a13 d-1e-8` cp S;
  `nt e1 a14 d-1e-8` cp S.

**More from PR 4038's dual review** (base `8793177b`), again all
`CensusEscalated` with no definite finding, volume, tier 2 and the
certificate passing:
- review r1 (`r1b_pinch_probes nt`, turned L corners), 5 runs at
  d = ±1e-7, margins 1.06e-9 to 3.95e-9, `pm_census_ee_gap` as well as
  `ee_span`: `Ltop_r-45 nt e0 a2 d1e-7` pc I, `Ltop_r-45 nt e1 a14 d-1e-7`
  cp S, `Lbot_r-45 nt e0 a15 d1e-7` cp I, `Lbot_r-45 nt e1 a2 d-1e-7`
  cp S, `Ltop_r30 nt e0 a10 d-1e-7` cp S;
- review r2 (`r2_pinch_probes nt`, face placement, d = 1e-8, all ∩),
  19 runs: notch307 `e1 a2` cp; notchbot `e0 a6` pc, cp; shallow200
  `e0 a0` pc, cp, `e0 a1` pc, cp, `e1 a3` cp; vee300 `e0 a0` pc, cp,
  `e0 a1` pc, `e1 a2` pc, cp, `e1 a3` pc, cp; asym `e0 a0` pc, cp,
  `e0 a1` pc, cp. Three vee300 runs escalate on edge-face predicates.

## The shape to give

Take one pose per predicate and read which pair the census cannot
decide:
- if it is a real sliver within the band, the boolean should have glued
  or refused it, and the fix belongs upstream of the join;
- if the census's own margin is too coarse for a definite pair, the fix
  is CONTACT's.

Then decide whether a door may ship a body its census cannot certify.

## More poses (PR 4036's dual review)

PR 4036 (the four-germ vertex pairs) builds near-tangent poses that main
refused, and some land in this class. Each has the exact volume, tier
2, the certificate and a legal operand, and only tier 3′ escalates.
None is definite.

- **r1** (`join/reflex-corner-vertex-vertex-review-r1`,
  `join_vv_review_r1_probes.rs` tilt battery): 3 runs on `notch343 edge
  base=4 psi=0 p=1 t=-1e-7`, pc I, pc S and cp I. The census reports
  `CensusEscalated` `pm_census_ef_cut_gap`, margin 8.4e-9. At the same
  tilt main ships this class on four other shapes. Detail probe:
  `join_vv_review_r1_tilt_detail`.
- **r2** (`join/reflex-corner-vertex-vertex-review-r2`,
  `review_r2_vv_probes.rs`, `R2_SWEEP=tilt R2_SHAPES=flat181`, eps =
  1e-6): 15 runs on flat181, e.g. `flat181 corner frame=11 eps=1e-6
  ax=1 cs U`. On main they refused `PairingMismatch`, `JoinDesync` or
  `Euler`. The census reports `CensusEscalated` `pm_census_ef_residual`,
  margin ±3.54e-9. Main ships 42 identical bodies of this pose family.
  r2 also counts 21 such runs on `valley4` (roof ∪ roof), which only
  head builds.

## More poses (PR 4050's dual review)

PR 4050 builds six-crossing vertex pairs that refused `PairingMismatch`
on main. Its review r2 bisected each change of the crossing count along
ψ to a tangency and posed either side of it, at ±1e-3, ±1e-5, ±1e-7
and ±1e-9 rad (57 048 runs).
- 1 187 runs at ±1e-7 and ±1e-9 go `PairingMismatch` → BAD.
- They span notch343, mirrored notch343, notch300, notch5, reflex315
  and a 359° wedge, on a cube's edge and corner.
- Every one fails tier 3′ only, with `CensusEscalated` (`pm_census_*`,
  margins about 1.4e-9, inside the band). The volume is exact, and t2,
  the certificate and the legal-operand check pass. Checked without the
  census, each body is right.
- Main ships 2 such bodies in the same set.

Repro: `mnotch343 corner i=13 j=3 k=72 d=+1e-7 ab U`, with
`SX_SWEEP=near`, in `crates/sweep/tests/review_sixx_r2_probes.rs` on
branch `join/six-crossing-pairing-review-r2`.
