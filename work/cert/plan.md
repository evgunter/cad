# S-CERT — certified-enclosure soundness (plan)

**STATUS: RATIFIED (Evan's in-chat rulings, 2026-08-29, all four
folded at Rulings below; the opening PR merges on Evan's sign-off of
the D9 row-5-boundary addendum text riding with it).** Opened on
Evan's direction (in-chat, 2026-08-29: "can you orchestrate its
program") from the ratified stream cut in
`docs/WORK-STREAMS-2026-08.md` (§S-CERT, merged at #1200 with the M10
territory fold). The cut is the charter and is cited, not
re-litigated.

Branch prefix (the #396 convention): **`cert/`** — unit branches
`cert/<unit>-<slug>`, orchestrator branch `cert/orchestrator`.
Away-channel tag `(S-CERT orchestrator)`. A/B ordinal band
**S-CERT = 700–799**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in this same PR, per that entry's rule; implementer blocks are
named `CERT-B1, CERT-B2, …` (unit names occupy `CERT-<n>`). Live
state is `work/cert/log.md`'s tail, never this file.

## Charter (from the cut, verbatim in substance)

The wrong-but-green and uselessly-wide certificate cluster — the
largest real-defect group in the tracker and nobody's territory:

- **Accepting defects first**: #723 / #893 (sphere meridian/rim
  certify wrong volumes near poles; S82 feeds the same conversation).
  VERBS' staged SPHSPH unit stops if acceptance needs the props fix,
  so these sit on that unit's critical path.
- **Interval-mode honesty**: #924 (rotation anchor width), #1191
  (period-fold widening, taken under the issue's stated f64-bit
  constraint; M10-3's driver is its first heavy consumer), #762
  (chart-speed guard — see substrate: mostly landed, residue remains).
- **Enclosure quality and metering**: #870 (area never metered),
  #453 / #390 (one rational-patch-flux lane, native and import
  sides), #528, #501, #303, #1006; the offset_fit sub-family
  #1005–#1008.
- **SMELL tracks M and N** (`geom-core` scalars/bvh; `geom`
  spline/linalg) — claimed whole, per the single-owner track rule.
  The track-M claim carves out `crates/bvh`'s interval lift (M10-5's);
  bvh work here is f64 box quality only, anything past that
  coordinates with M10 first.

## Ratified ground (cited, not re-litigated)

- The stream cut and its keep-outs (`docs/WORK-STREAMS-2026-08.md`
  §S-CERT, including the M10 fold of 2026-08-29).
- **D2 addendum** (`docs/DESIGN.md`): a refusal of valid input the
  lane could serve is row 2 — every new refusal this program mints
  gets classified there.
- **D4/Q1/D9**: one ε per run, margined trilean predicates,
  bit-identical replay. #1191's f64-bit constraint is this doctrine
  applied: the interval lane may be reformulated only where the f64
  expression's bits do not move.
- **The #1143 poison-vs-widen contract is M10-D's (ratified)**: this
  program supplies instances, never the answer. Concretely (PCURVE,
  on the opening PR): a correlated expression evaluated naively
  under `Interval` — #1157's shape — goes to #1143's audit as a
  member, not to a new issue.
- **H-R3 / #867**: "tightening to `CertifiedBounds` works at least
  for now" — governs track M's trait ground; **read `H-R16` before
  starting `H5` or `S90-impl`**, and **#883 stays parked** (reserved
  as lane H-f, not this program's to unpark).

## Substrate facts the slate is shaped by (surveyed 2026-08-29)

- **#762 is substantially landed on main** (outside any program, at
  `91164e3b`): `geom-brep/src/ssi.rs:991` now guards
  `!speed.is_finite()`. What remains is the residue the SMELL scan
  measured: `ssi/march.rs:423` still spells `is_nan() || <= 0.0` so
  `+∞` passes (Q-track row **D285**); the seeding guard made the ℝ⁴
  control-net poison arm unreachable by magnitude (**D286** — wants a
  fixture by another route or a recorded verdict that none exists);
  and the issue's NaN-dropping `max` fold and `exhaust.rs:285`
  rewording want verifying before #762 closes.
- **PCURVE P-2 (#1177, in blinded review on frozen head `0ecd3f7e`)
  carries the #1157 `orthonormal_basis` fix**, written and measured
  in `geom-core/src/linalg/vec.rs`. The keep-out, time-boxed by the
  PCURVE orchestrator on the opening PR: no S-CERT unit **edits**
  `vec.rs` until #1177 lands; the frozen head is readable now.
  #924's unit works in `affine.rs` beside it and merges main
  frequently.
- **#723's mechanism is confirmed live** (`props/curved.rs` sphere
  arms still fold endpoint latitudes through `min_max`), on both the
  rim-bearing and the rimless (two-band) arm — the second instance is
  measured in #723's fourth comment (−29% from a hemisphere split at
  ±π/4). The issue's reproduction artifacts lived in a dead machine's
  `cad-work`; the STEP half-cap fixtures are re-derived from the
  issue text and committed this time.
- **`props/quad.rs` consolidation (C3/C-m, D30) is Track R's and
  stays gated behind #723** — a correctness fix here does not do the
  consolidation, but CERT-1 and CERT-5 answer C-m's recorded
  questions in their PR descriptions (which engine is authoritative,
  what the convergence-block change implies for the other copies).
- **VERBS-SPHSPH is staged behind CERT-1** (VERBS-PLAN item 9): a
  lane minting sphere faces with polar rims must not treat
  `props_rim_level` as a closed premise. CERT-1 is therefore this
  program's first dispatch, ahead of everything.
- **#390's route 2** (tighten M7-6's CYLINDER certificate via exact
  spline-product hulls) is surface-certification work and reads
  **unclaimed** (PCURVE orchestrator on the opening PR: PCURVE is
  edge-description work and goes to its exit walk after P-2, so a
  reservation there would park the route on a closing program).
  **Route 1** (tighten the rational patch-flux enclosure) is
  in-fence for this program and is the route CERT-5 takes — it also
  serves #453, which route 2 cannot. Whoever ever takes route 2
  should know it has a second beneficiary: promoting the wall to an
  analytic `Cylinder` makes its interior isos analytic, which
  sidesteps #1195's missing knot-insertion machinery (PCURVE's
  observation, offered unmeasured).

## The slate

Ordered by urgency and dependency. Each unit gets its own spec at
dispatch; difficulty is logged pre-draw per the protocol.

- **CERT-1 — the sphere polar acceptance defects (#723 + #893);
  dispatchable pre-ratification** (both issues are named in the
  charter; the fix shapes below are the issues' own
  recommendations). Scope: `geom-brep/src/props/curved.rs` (+
  `props/mod.rs` docs, the audit table row, fixtures). (i) #723 by
  its option (2), the torus's own move: take the sphere v-extent
  from the traversed **arc's span**, not endpoint latitudes — on the
  rim-bearing AND rimless arms (the rimless arm has no rim, so only
  (2) generalises there). (ii) #893's three asks: a failing row at a
  near-polar interior rim first; a rim lever that meters the
  separation honestly toward the poles (the recommendation: margin
  in latitude/arc terms, not `R·|Δ sin v|`) or an explicit refusal
  in that regime, classified per the D2 addendum; the audit row
  corrected from `OK`. (iii) The STEP half-cap twins committed as
  fixtures (accepted-with-correct-volume, and the no-split twin's
  refusal pinned). (iv) PR description answers C-m's three recorded
  questions. Out of scope: `mesh::walk`'s `closing_column`
  debug_assert (reported to S-MESH's future owner via the issue),
  quad-engine consolidation (C3). S82's verdict line is answered by
  this unit's record — flagged for Evan's eyes at ratification.
- **CERT-2 — #762 close-out and its guard residue (S).** Verify the
  main fix against the issue's four asks; fix `ssi/march.rs`'s
  sibling guard (D285: `StepCollapsed` on non-finite speed, by the
  guard's own comment's argument); NaN-propagating fold if still
  absent; `exhaust.rs:285` reworded to the reachable cause; D286
  answered with a fixture by a non-magnitude route or a recorded
  verdict that none exists. Lands the Q-table row deletions in the
  same PR per §D rule 3. **Fence note**: `ssi*` is SMELL track Q's
  ground with no live claimant; this unit takes exactly the named
  residue of an issue the cut assigned here, no more (seam recorded
  under Rulings sought Q3).
- **CERT-3 — #924, the rotation-anchor round-trip (S/M).**
  `geom-core/src/linalg/affine.rs`: spell the translation so the
  vanishing factor multiplies rather than cancels ((I − R)·q with an
  exactly-representable identity case); an Interval row pinning
  zero-angle width preservation (red first); re-measure the
  `RevolvedPoint` mapped-source enclosures; its own k-lint pass (the
  constructor sits under every rigid transform). Keep-out: `vec.rs`
  (P-2 in flight). Track N fence ground — the unit re-merges main
  before opening its PR.
- **CERT-4 — #1191, period-fold widening (L; possibly two PRs:
  profile sites, then topo sites).** The deliverable: a fold whose
  interval enclosure is computed from the true angular difference
  rather than a `floor` over a straddling box, plus
  straddle-driving rows for the topo sites nothing currently
  exercises. The cession's f64-bit constraint is RESTATED
  SEMANTICALLY (Evan, in-chat, 2026-08-29: bit preservation is not
  the bar; a flipped classification is fine when semantically
  correct and the code cleaner): the unit may reformulate both
  lanes if that is the cleaner shape, PROVIDED the exact-fit
  guarantee survives — a true tangency must still classify as an
  exact fit, by a preserved structural zero or by a re-derived
  gate, never by a re-baselined near-miss. The hit list in the issue is
  the scope; its stated blind spots are re-swept at merge base.
  Sequenced early enough that M10-3's driver consumes the fixed
  fold, coordinated with M10 on timing.
- **CERT-5 — the rational-patch-flux lane, native and import sides
  (#453 + #390 route 1) (M/L).** `props/quad.rs`'s rational
  composite: knot-aligned composite cells (retiring the Θ(1/p)
  straddle floor), with the `w`-uniform-in-v exact arm
  (`quad.rs:1958`'s own named candidate) taken if it falls out
  cleanly — it is the true analogue of `patch_flux_exact`. A
  regression row at 6+ stations with off-grid knots. Flip
  conditions carried in the spec: dm1 first-class (`WILD_IMPORTS`
  9→10), the lily leaf demos certify and their flip-when-fixed
  paragraph retires.
- **CERT-6 — #870, the area-gauge tripwire and its calibration
  (under the Q1 ruling) (S/M).** No always-on metering. (i) The A2
  gauge — `area.width()` against a certified perimeter lower bound,
  a mean edge displacement, the direct analogue of the flux
  funnel's mean-boundary-displacement gauge — asserted as a
  row-5-boundary `debug_assert` at the patch lanes' area pass, with
  a GENEROUS ceiling calibrated from the corpus and the calibration
  documented in-file (the `closing_column` model; its
  nine-orders-off estimate on the #723 input is the cautionary
  half). Falls back to the relative gauge on `area.lo()` if a
  certified perimeter is not cheaply reachable in the lane. (ii)
  #873's ceilings re-derived as the calibration record;
  `review_m6_3_chart_probes.rs:354`'s deliberate lower-bound row
  re-derived, not deleted. (iii) The opt-in refinement door
  (caller-requested area target, per-round resolution, typed
  refusal) filed as a demand-triggered valve, NOT built — no
  consumer asks today. (iv) The order bump on
  `area_midpoint_taylor` optional if it falls out cheap. S26/S230
  pointers updated at merge.
- **CERT-7 — the offset_fit family (#1005, #1008, #1007) (M).**
  One unit, three commits' worth of coherent scope in
  `geom-brep/src/offset_fit.rs`: the weighted composite (#1005 —
  flip the two reviewer rows back to containment), per-cell
  recentring (#1008 — re-measure the small-|d| row), directional
  refinement with the stall guard (#1007). #1006 is **not** here —
  it is CERT-10, under the Q2 ruling.
- **CERT-8 — chart-stretch honesty (#501, then #528) (M/L).**
  #501 first: give `topo` the Floater stretch bounds
  (`nurbs_stretch_bounds`) through a properly-layered export, meter
  `azimuth_arm`'s NURBS arm and both `v_meter` fallbacks, record
  the audit row OK, scale twin + three-outcome posture for newly
  refusing rows. #528 extends the chart-region positive-area lane
  with inf-side arms per its own per-kind derivations. The
  pcurve_cache seam is PCURVE-adjacent: layering reviewed with
  P-2's resume state in view.
- **CERT-9 — #303, `signed_volume` recentring (S).** Recentre on an
  interior point (bbox centre), pin the huge-offset probe as a
  gate. Mesh-fence ground with no live claimant; taken because the
  cut assigns it.
- **CERT-10 — the patch-hull consolidation (#1006, under the Q2
  ruling) (M/L; after CERT-5 and CERT-7, which edit two of its
  three sites).** Home the tensor derivative-net assembly in
  `geom_core::spline` beside `compose`; collapse `nurbs_cert`'s
  whole-face arm into a fold over `patch_bound`'s cells (bound
  tightens or holds; fold cost measured against the whole-net
  hull before the shape is chosen); retire the magnitude reading
  in favour of the strictly-tighter signed one, with the
  rational-face grid re-sizing and the render/tess-budget
  re-baseline owned by this unit's PR — what moved and why stated,
  per the render-lane conventions. Affected pinned rows re-derived,
  not preserved.
- **CERT-M / CERT-N — the absorbed SMELL tracks.** Worked as track
  lanes after the defect cluster clears, sequenced by their own
  tables (`SMELL-SCAN-2026-08.md` §Track M, §Track N), with this
  program's specific notes: `H5` splits into sub-lanes and its
  `Dual`-rewriting sub-lane is ADV; `H3+H4` is one lane; `D240`
  before `D241` (one lane), and Track T's `D320` follows what
  `D240` mints (filed, not taken); `D221` decides its header
  question before deleting anything; `S213`'s bound half only (the
  gate half is K's). Rows land per §D's conventions (delete the
  row in the landing PR).

Cross-program interfaces, named so "certificate" does not become a
bucket: Dual-at-certified-gates semantics, `dual.rs`, the
`AtRestPolicy` seam, `product.rs`'s Dual arms and the bvh interval
lift are **M10's** (the ratified fold); `orthonormal_basis` is
P-2's until #1177 lands; #1018–#1020 are OFF-D's under VERBS;
`props/quad.rs` consolidation is Track R's C3, gated behind CERT-1;
the `closing_column` assertion note was S-MESH's and is discharged
(issue 868: the condition is `topo::coherence`'s report now).

## Rulings (Evan, in-chat, 2026-08-29)

1. **Q1 — RULED (Evan, in-chat, 2026-08-29)**: no always-on area
   metering — the intent is that any realized geometry everywhere
   within ε of correct is valid, so the wide-but-sound default
   bracket stands and no funnel target is built (the O(h) cost
   arithmetic independently supports this: an ε-scale area target
   is a ~10³–10⁴× piece-count multiplier under the current rule).
   The check lands as a hefty `debug_assert` on the A2 gauge
   instead, under the row-5-boundary class this ruling also
   ratified into the D2 addendum (`docs/DESIGN.md`): expensive
   checks whose failure probably indicates a bug — currently on in
   every profile (`debug-assertions = true` in release), eventually
   debug/CI-only. Purchasable tightness (a caller-requested area
   target with typed refusal) is a demand-triggered valve, filed
   not built. CERT-6 is cut to this ruling.
2. **Q2 — RULED (Evan, in-chat, 2026-08-29)**: all three proceed —
   shared home in `geom_core::spline`; the whole-face arm collapses
   into a fold over `patch_bound`'s cells (per-cell-then-union is
   tighter or equal, so the bound improves; fold cost measured
   against the whole-net hull before the shape is chosen); the
   magnitude-reading retirement scheduled with the re-baseline
   attached and owned. Bit identity is explicitly NOT the bar
   (`memories/output-stability-as-justification.md` carries the
   stated principle); affected pinned rows are re-derived. Landed
   as CERT-10 in the slate.
3. **Q3 — RULED (Evan, in-chat, 2026-08-29: not a design question,
   "do as you see fit")**: CERT-2 executes two Q-track rows
   (D285/D286) as #762's named residue; CERT-4's hit list crosses
   profile/topo fences wherever #1191's sites live. Both grounds
   have no live claimant and both issues are assigned here by the
   merged cut.
4. **Q4 — RULED (Evan, in-chat, 2026-08-29)**: route 1, with
   knot-aligned composite cells as CERT-5's primary deliverable.
   Precision the ruling rests on, stated so it is not fudged:
   knot-aligned cells restore certified convergence to target
   (composite, refined) — the analytically *exact* pieces are the
   `w`-uniform-in-v arm (kept in CERT-5 as the strictly-better
   path where weights vary in one direction only, which covers
   loft walls and dm1) and route 2's recognition certificate.
   Route 2 (the algebraic CYLINDER certificate) reads **unclaimed**
   per the PCURVE orchestrator's answer on this PR, with #1195
   recorded as a second beneficiary for whoever takes it.

## Process

Standard, v6: substrate → binding spec → one implementer + the
cross-model dual review + union fix pass; implementer arms drawn per
the current block rule in `docs/MODEL-AB-LOG.md` (read on main at
each dispatch — that document owns every live number); ordinals
claimed on main at review dispatch from band 700–799;
record-at-merge with per-phase tokens/wall-clock; blinding
discipline verbatim (no `Co-Authored-By` in lane commits; no
arm-naming surface reviewers can read). Hosted CI is the only gate;
every new row ε-three-outcome honest; reviewer suites promote as-is
and may be retired per policy. Implementer dispatches point at
`docs/prompts/implementer-discipline.md` by path; reviewers get
explicit claims to falsify plus `docs/prompts/reviewer-style-lane.md`.

**This orchestrator runs in a remote container** (the M10/GUI
precedent): no persistent `~/.local/share/cad-work`, no script
monitors (PR watching via MCP subscriptions + scheduled self
check-ins; away-channel etiquette by hand under the `(S-CERT
orchestrator)` tag), GitHub through MCP. Disk (~29 G free) is the
binding constraint: lanes are worktrees sharing one object store,
each with its own `CARGO_TARGET_DIR`, ≤ ~2 concurrent lane targets,
review targets reclaimed the moment the report is in hand. The
build-slot mutex, CONFLICTING-means-silent-CI, and push-early rules
bind unchanged. The clone arrived shallow and was unshallowed with a
blob filter at opening.

## Exit shape (proposed)

No wrong-but-green certificate named by the charter survives: the
sphere polar arms take their extent from spans and their rim lever
holds at the poles (VERBS-SPHSPH unblocked); zero-angle rotations
are width-preserving at Interval; the period folds are honest at
Interval with f64 bits unmoved and M10-3 consuming them; the
chart-speed guard family refuses non-finite by name everywhere;
rational patch flux certifies through interior off-grid knots on
both the native and import doors (dm1 flips); the area enclosure is
metered under the ratified #870 proposal; the offset_fit composite
certifies rational fits, recentres, and refines directionally;
chart-stretch arms are metered from real bounds on both channels;
`signed_volume` is recentred; tracks M and N are empty in §D. Every
unit merged on its own green hosted head; the walk convention
applies at exit.
