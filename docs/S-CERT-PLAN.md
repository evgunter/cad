# S-CERT — certified-enclosure soundness (plan)

**STATUS: DRAFT (design conversation).** Opened on Evan's direction
(in-chat, 2026-08-29: "can you orchestrate its program") from the
ratified stream cut in `docs/WORK-STREAMS-2026-08.md` (§S-CERT, merged
at #1200 with the M10 territory fold). The cut is the charter and is
cited, not re-litigated; what needs Evan here is the **Rulings sought**
section below. Units marked *dispatchable pre-ratification* are defect
fixes named in the charter itself and proceed while this document
waits.

Branch prefix (the #396 convention): **`cert/`** — unit branches
`cert/<unit>-<slug>`, orchestrator branch `cert/orchestrator`.
Away-channel tag `(S-CERT orchestrator)`. A/B ordinal band
**S-CERT = 700–799**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in this same PR, per that entry's rule; implementer blocks are
named `CERT-B1, CERT-B2, …` (unit names occupy `CERT-<n>`). Live
state is `docs/S-CERT-LOG.md`'s tail, never this file.

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
  program supplies instances, never the answer.
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
- **PCURVE P-2 (#1177, wind-down WIP with a declared resume) carries
  the #1157 `orthonormal_basis` fix**, written and measured in
  `geom-core/src/linalg/vec.rs`. The cut's keep-out concretizes to:
  no S-CERT unit edits `vec.rs`; #924's unit works in `affine.rs`
  beside it and merges main frequently.
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
  spline-product hulls) lives in recognition machinery adjacent to
  PCURVE's fence; **route 1** (tighten the rational patch-flux
  enclosure) is in-fence for this program and is the route CERT-5
  takes — it also serves #453, which route 2 cannot.

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
- **CERT-4 — #1191, period-fold widening under the f64-bit
  constraint (L; possibly two PRs: profile sites, then topo
  sites).** The deliverable the issue names, minus the decision it
  reserves: f64 bits do NOT move (the cession's stated constraint),
  so the work is an interval-lane-honest fold — an enclosure
  computed from the true angular difference rather than a `floor`
  over a straddling box — plus straddle-driving rows for the topo
  sites nothing currently exercises. The hit list in the issue is
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
- **CERT-6 — #870, the area-enclosure meter (design conversation
  PR, Evan-ratified; then implementation).** The issue supplies the
  measurements; the proposal to be argued: the funnel reads
  `area.width()` against a relative gauge on `area.lo()` with a
  floor set from the issue's fixture-B anchor; `QUAD2_AREA_PIECES`
  recomputed per round; a typed area-side budget refusal (the flux
  side's `QuadratureBudget` shape) — acknowledging it changes which
  faces certify, which is exactly why it waits for the ruling.
  Re-derives `review_m6_3_chart_probes.rs:354` rather than deleting
  it; #873's ceilings and S230's unrouted consumers named in the
  spec.
- **CERT-7 — the offset_fit family (#1005, #1008, #1007) (M).**
  One unit, three commits' worth of coherent scope in
  `geom-brep/src/offset_fit.rs`: the weighted composite (#1005 —
  flip the two reviewer rows back to containment), per-cell
  recentring (#1008 — re-measure the small-|d| row), directional
  refinement with the stall guard (#1007). #1006 is **not** here —
  it is a design decision (Rulings sought Q2).
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
S-MESH's future owner inherits the `closing_column` assertion note.

## Rulings sought (Evan)

1. **Q1 — the #870 proposal** (CERT-6): does the area funnel read
   `area.width()` with a floored relative gauge and a typed
   area-side refusal, per the recommendation above? This changes
   which faces certify, so it is a fork, not an elaboration.
2. **Q2 — #1006**: whether the three patch-hull assemblies share a
   home in `geom_core::spline`, whether `nurbs_cert`'s whole-face
   arm collapses into a fold over `patch_bound`'s cells, and who
   owns the magnitude-reading retirement's re-baseline (it moves
   render and tess-budget baselines). Recommendation: shared home
   yes, the collapse yes if bit-preserving on the integral arm's
   returns, and the retirement scheduled as its own unit with the
   baseline move attached — but all three are cross-crate design
   calls.
3. **Q3 — two stated fence seams**, recorded for veto rather than
   ratification: CERT-2 executes two Q-track rows (D285/D286) as
   #762's named residue; CERT-4's hit list crosses profile/topo
   fences wherever #1191's sites live. Both grounds have no live
   claimant and both issues are assigned here by the merged cut.
4. **Q4 — #390 route choice**: CERT-5 takes route 1 (the flux
   enclosure); route 2 (the algebraic CYLINDER certificate) stays
   available to PCURVE if they want the recognition win
   independently. Veto if you want route 2 led from here instead.

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
