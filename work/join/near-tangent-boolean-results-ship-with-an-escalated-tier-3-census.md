---
id: near-tangent-boolean-results-ship-with-an-escalated-tier-3-census
kind: issue
title: Near-tangent booleans ship results whose tier-3′ census escalates: 139 runs on main, 72 more built by PR 4026 at 26 poses
status: open
opened: 2026-10-04
priority: P0
cost: H
refs: [3987, two-copies-of-a-pierce-carry-edges-that-run-within-the-band]
branch: join/near-tangent-census-measure
needs_ev: true
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

## Measured (pinch unit, branch `join/pinch-one-vertex-per-cone-build`)

Review r2's near-tangent battery (`r2_pinch_probes nt`), main
`f9bf3bca` vs the pinch unit's head:
- **19 → refusal.** Lines that shipped `BAD` with tier 3′
  `CensusEscalated` now refuse `ResultInvalid { ShellRoleUndecided:
  Escalated }`, e.g. `shallow200 nt e1 a3 d1e-8 face psi=0 cp I`.
- **3 `SOUND` → the same refusal.** All at `d1e-8`, e.g. `shallow200 nt
  e1 a3 d1e-8 face psi=0 pc I`.
- **7 refusals → `BAD`.** Lines that refused `JoinDesync` now ship `BAD`
  with tier 3′ `CensusEscalated`, e.g. `vee300 nt e1 a3 d1e-8 face psi=0
  pc U`.

The pinch's lumps now meet only at a point, so they become shells of
their own, and at 1e-8 the gate cannot decide one shell's role.

## Measured (JOIN measurement unit, branch `join/near-tangent-census-measure`)

Main `047d10d5`, release. Everything below comes from two tools.
- **The probe:** `crates/sweep/examples/near_tangent_census_probe.rs`. It
  ports `r1_pierce_probes cube`'s near-tangent set and adds the main
  battery's vee300, asym, w345 and w60 corners. That makes 10 corners × 3
  edges × 16 turns, ∪ ∩ ∖ in both orders, 2 880 runs per tilt. With
  `NT_DUMP=1` it dumps every pair the census decided against.
- **The classifier:** `scripts/oracles/near_tangent_census_classify.py`.
  It reproduces each census margin bit for bit in f64 and re-reads it at
  60 digits on the same coordinates. Every escalation matched its pair:
  1 520/1 520 at ε = 1e-9, 663/663 at the ±3 tilts, 2 877/2 877 at 1e-12,
  803/803 at 1e-6.

**Population on main.** Main's `near_tangent_battery` gives 70 BAD, all at
d = 1e-7 (vee300 34, asym 14, w60 12, w345 10). Every one fails tier 3′
only. It also gives 366 boolean refusals and no `ShellRoleUndecided`.

The probe at ε = 1e-9 (d = ±1e-5 to ±1e-9, 28 800 runs):
- **818 BAD.** All are at the oracle volume, with t2, the certificate and
  the operand check passing. 788 fail on `CensusEscalated` alone. 30 add
  a definite finding: 17 `CensusUndecidable`, 7 `UndeclaredContact` and
  6 `StaleContactDeclaration`.
- **34 refuse `ResultInvalid { ShellRoleUndecided }`.** All are ∩ at
  d = +1e-8: vee300 12, shallow200 8, asym 6, notch307 4, w345 4.
- **Below 3 bands nothing builds.** At ±2e-9 and ±3e-9 the boolean
  refuses every run itself (`Escalated`).

**One pose per predicate** (ε = 1e-9; the margin is the census's f64
value):

| predicate | pose, pair | margin | exact / true distance | class | at fault |
|---|---|---|---|---|---|
| ee_span | `notch307 nt e0 a0 d1e-7 pc I`, E18v1×E20v3 | 1.10e-9 | exact span 7.5e-54; the edges meet only at `v`, end to end | b, arithmetic | `ee_cross_spans`: d = eb.p0 − ea.p0 over \|n\|², error ulp·\|d\|/θ |
| ee_parallel | `vee300 nt e0 a0 d1e-8 cp S`, E34v1×E35v3 | 5.79e-9 | exact; the edges meet only at their shared vertex, end to end | b, proxy | sin θ of the lines × the shorter length |
| ee_gap | `w345 nt e0 a8 d1e-8 pc I`, E25v1×E33v3 | 1.84e-9 | segments 9.85e-2 apart | b, proxy | the line-to-line gap is read before the spans |
| ee_overlap | `notch307 nt e0 a3 d1e-8 pc U`, E13v1×E25v5 | 3.22e-9 | segments 2.32e-8 apart | b, proxy | the offset is read at `eb.p0` (filed: collinear-lane row) |
| ef_residual | `vee300 nt e0 a0 d1e-8 pc U`, E44v1×F20v3 | 5.79e-9 | the end is 0.576 outside the face | b, proxy | the plane is read, not the face's region |
| ef_cut_gap | `Ltop nt e2 a8 d1e-9 pc U`, E34v1×F10v3 | 2.0e-9 | the vertex is 1.005 past the edge's end | b, proxy | the line gap is read before the span |
| vf_residual (+ve_line_gap, ef_residual, ee_parallel) | `w345 nt e2 a6 d-3e-8 pc U`, V24v1×F14v1 | 9.55e-9 | the vertex is 9.55e-9 from the face, inside it; an edge 3.7e-8 long | a | the split: an in-band vertex, neither glued nor refused |
| ShellRoleUndecided | `notch307 nt e0 a3 d1e-8 pc I`, shell 2v5 | V/A 3.05e-9 | the oracle's lump: 7.7e-16 m³, 2.2e-8 thick at its far end, V/A 3.9e-9 | a | the cone split keeps a lump that is in band |
| (ee_span, beneath) | `notch307 nt e0 a3 d1e-8 pc S`, E24v5×E25v5 | 2.76e-9 | exact span 2e-54; but one face's corner at `v` is 3.6e-8 rad wide, within K·ε for 0.28 | b, with an a beneath | arithmetic over a sliver the split left |

**All escalations by class at ε = 1e-9** (±1e-5 to ±1e-9, plus the ±3
tilts):
- `ee_span`: 1 547. b-arith 1 063, b-arith over a real sliver 478,
  b-proxy 6.
- `ee_parallel`: 240 b-proxy, 6 a.
- `ef_residual`: 195 b-proxy, 18 a.
- `ef_cut_gap`: 157 b-proxy.
- `ee_gap`: 58 b-proxy, 6 a.
- `ee_overlap`: 20 b-proxy.
- The vertex lanes: 12 b-proxy, 12 a.

All 42 class-a escalations are at the one pose family `w345 e2` ±3e-8.

**What follows the tilt** (escalations, by class):

| ε | b-arith (ee_span) | b, proxies | a |
|---|---|---|---|
| 1e-9 | d = 3e-7 to 1e-8 (none at ≥ 1e-6 or ≤ 3e-9) | every tilt, 1e-5 to 1e-9 | slivers beneath the arithmetic at 1e-7 to 1e-8; the vertex-on-face at ±3e-8; lumps (SRU) at 3e-8 and 1e-8 |
| 1e-12 | escalated at 1e-5 to 1e-8 (2 331); **definite false `EdgeEdgeCross`** at 1e-5 to 1e-11 (845, every one at a shared point) | every tilt | slivers beneath at 1e-10 to 1e-11; the vertex-on-face at ±1e-11 (w60); lumps with V/A in band at ±1e-11 |
| 1e-6 | none (the f64 error is ≪ ε) | 1e-3 to 1e-9 | lumps at 1e-5; at ≤ 1e-7 the tilt is inside the band and the census reads 347 `EdgeFaceOverlap` + 38 `VertexOnFace` undeclared (D10 hold ground; not classified) |

- **b-arith** follows θ, not ε. The f64 error grows as the tilt falls,
  so it enters the band near d ≈ 1e-7 to 1e-8 at ε = 1e-9, near 1e-5 to
  1e-7 at 1e-12, and past K·ε below that.
- **b-proxy** is independent of the tilt. It catches whatever line or
  plane happens to pass within the band.
- **a** begins where the sliver's own width d·L falls to a few K·ε: about
  3e-8 at ε = 1e-9, 1e-11 at 1e-12, and 1e-5 at 1e-6.
- **The ε = 1e-12 lumps are a different case.** At 1e-8 to 1e-10 the
  role read refuses on a certified enclosure 1.4e-9 to 5.2e-7 wide around
  a V/A of about 34 K·ε. That is b (ENCL).

**May a door ship a body its census cannot certify?** The evidence, for
the design pass:
- **Every body here is right.** All 818 BAD bodies at ε = 1e-9 have the
  oracle volume, t2, the certificate and the operand check.
- **The escalations do not show uncertainty.** None of the 1 520 at the
  six named predicates is class a as read. Each is the census's f64
  arithmetic (982) or a line or plane read for a pair definitely apart
  (538).
- **The census can be wrong, not just undecided.** The same arithmetic
  at ε = 1e-12 gives 845 definite false findings. A door gating on
  tier 3′ would refuse right bodies there with a wrong reason.
- **Real slivers do occur, and they are what an exact census would
  miss:**
  - 305 of the 982 arithmetic escalations at ε = 1e-9 sit on two edges
    within K·ε for 0.06 to 1.0. An exact census would pass all of them
    silently.
  - The vertex-on-face (42 escalations) is escalated correctly.
  - The lumps are refused correctly, at the gate, typed.

  So a fixed census leaves a residue that is real in-band geometry. That
  residue is the split's to decide (glue, refuse, or certify as a
  sliver), not the door's.

**Filed.**
- CONTACT, one row per class-b predicate:
  - `the-census-crossing-lane-misplaces-a-shared-points-crossing-on-a-near-collinear-pair`
  - `the-census-parallel-test-reads-two-edges-through-one-point-end-to-end-as-a-near-parallel-pair`
  - `the-census-crossing-lane-escalates-a-line-gap-where-the-segments-lie-far-apart`
  - `the-census-edge-face-lane-escalates-a-plane-residual-at-an-end-far-outside-the-face`
  - `the-census-edge-face-cut-escalates-a-line-gap-at-a-boundary-vertex-beyond-the-edge`
  - `ee_overlap` is covered by
    `the-census-edge-edge-collinear-lane-reads-the-offset-at-the-long-edges-start`
    (evidence added there).
- JOIN, one row per class-a stage:
  - `a-near-tangent-split-leaves-a-face-corner-that-runs-within-the-band`
    (its two-vertex form is CONTACT's
    `two-copies-of-a-pierce-carry-edges-that-run-within-the-band`;
    evidence added there)
  - `a-near-tangent-intersections-sliver-lump-reads-its-role-in-band-and-refuses`
  - `a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded`
- ENCL: `the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell`.
- CLEAVE: `near-tangent-pierce-poses-reach-three-classification-invariants`
  (345 runs, outside this row's question).

## The shape to give

No door ships a body its census cannot certify. Each question has one owner. The three DESIGN.md additions on this branch state them: Q1's conditioning premise, tier 3′'s census clause, and D10's "Booleans" clause.

- **The census (CONTACT), one unit.** Each pair predicate decides the gap between the two closed cells beyond what they share structurally, through formulas whose f64 error is a few ulp at the model's extent. That retires every b-arith and b-proxy escalation above, along with the 845 definite false `EdgeEdgeCross` findings at ε = 1e-12. Two edges leaving one vertex are told apart by their angle levered at the shorter edge, so a thin corner passes when its far ends are definitely apart: the witness `notch307 nt e0 a3 d1e-8 pc S` has its far ends 8.9e-8 apart, about 9 Kε. The five class-b CONTACT rows, `pair_edge_edge`'s missing shared-point rung, and `ee_cross_spans`' unguarded division are one re-posing.
- **The split (JOIN).** Every reading is a Q1 trilean and refuses `Escalated` in band. The w345 vertex 9.55e-9 off a face is not such a reading. PR 4338 logged every decision on the four poses, and every one was definite. The vertex is a pierce 3.69e-8 along a 345° bottom edge, against the wedge's *own* 0° side face, which meets that edge only at the corner. Their gap, r·sin 15°, is a composed quantity no split reading compares. So the door gate catches it, as it catches the sliver lump (`a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded`).
- **The door.** The finished-body gate decides "no in-band pair or shell" on the result alone, because definite readings can compose into an in-band quantity no reading compared. The sliver lump is the witness: every cut is definite, and its V/A is in band. A finding born of an in-band margin refuses as the operands' ill-conditioning (`Escalated`); a definite finding is a kernel defect (`ResultInvalid`). The lump's `ShellRoleUndecided` takes the first typing now. Census escalations take it when the parked `boolean-door-runs-the-census-over-its-result` lands. This class does not join that row's blockers.
- **Nothing is glued.** No "certified sliver" state and no shipped-uncertified state exist.
- **What it rests on.** The typing is honest only where every margin the gate reads meets the conditioning premise. ENCL's world-origin volume (`the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell`) is a conditioning defect under it.

**Re-dispositions once this lands.**
- **`a-near-tangent-split-leaves-a-face-corner-that-runs-within-the-band`:** a legal needle under the lever rule, unless a pair's far-end gap is itself in band. Count the 478 pairs by sin α × the shorter length.
- **The lump row:** closes as `Escalated` at the gate.
- **The vertex-on-face row:** a composed pair (PR 4338). It moves to the door's typing unit.
