# M10 — the error-propagation MVP (plan)

**STATUS: DRAFT — design conversation, awaiting Evan's sign-off.
Nothing dispatches until this is ratified.** The milestone builds
`docs/ERROR-DESIGN.md` (E1–E11, ratified #110 with the
chamber-containment amendment) at MVP scope. Every *decision* this
plan leans on is ratified elsewhere and is cited, not re-litigated.

Branch prefix (the #396 convention): **`m10/`** — unit branches
`m10/<unit>-<slug>`, orchestrator branch `m10/orchestrator`.
Away-channel tag `(M10 orchestrator)`. A/B ordinal band
**M10 = 500–599**, claimed in `docs/MODEL-AB-LOG.md`'s banding entry
in this same PR, per that entry's rule; implementer blocks are named
`M10-B1, M10-B2, …` (the GUI precedent — unit names occupy
`M10-<n>`). Live state is `docs/M10-LOG.md`'s tail, never this file.

## Ratified ground (cited, not re-litigated)

- **ERROR-DESIGN E1–E11** — distributions as document-layer
  parameter metadata; the one `Measure` sink; forward-`Dual`
  chamber-local sensitivities; the E6 driver with **no-flips v1**;
  the certified-worst-case-gates/RSS-advisory stackup; the E7
  trichotomy; tail-mass accounting with the merged unresolved-mass
  budget and chamber containment; E9 tangent-poison-never-refuses;
  E11's loud exclusions.
- **The D1 ruling (Evan, 2026-08-19)**: a `Dual` may not certify —
  *at least for now* — but it may have `Bounds`. The hedge is
  collected as the question M10 opens with (*what does a `Dual`
  actually have to do*), and M10-D below is where it gets answered.
- **CONTACT-DESIGN C5**: the signed gap is an ordinary
  scalar-generic E3 Measure (`gap(declaration)`, sign convention
  binding, smoothness statement telling the E4 lane where Clarke
  enclosures are the honest object).
- **PARAM-LINT-SPEC PL6** (the lint is DISCIPLINES' unit, not
  ours): same name ⇒ one comoving marginal; distinct names ⇒
  independent marginals; derived expressions comove through
  evaluation and need nothing.
- **PERF-PLAN**: M10 owns the parallel subdivision driver (D9
  idiom 1 over sub-boxes); MC is rayon-over-samples; GPU is tabled.
- **D4/Q1/D9 verbatim**: one ε per run; margined trilean
  predicates; bit-identical replay; the dual value channel IS the
  f64 build.

## Substrate facts the slate is shaped by (surveyed 2026-08-29)

What already exists: `DualInterval = Dual<Interval>` with the
Clarke/kink conventions implemented and tested at both base scalars
(`geom-core/src/dual.rs`); document builds at `T = Interval` green
over the whole corpus with save/load fingerprint identity
(`m4_pr6_roundtrip_interval.rs`); `VerdictLog` rows float-free and
scalar-independent, so f64 and Interval verdict vectors compare
exactly — the certified-leaf mechanism E6 needs; a subdivision
template with typed budget refusal and a receipt identity
(`geom-brep/src/ssi/exhaust.rs`); `bvh::Aabb::from_points<T: Bounds>`;
`topo::Separation::certify` already reasoning about `Dual` soundness
by value-channel delegation; the `checks`/`finding` reporting sink.

What must be built from zero (and where the walls are):
`Distribution` and everything named in E2/E3/E5/E10 (grep-clean:
no `Distribution`/`Measure`-node/`Assertion`/`Stackup`/
`ParamBoxVerdict` anywhere); `ContentBits for Dual` — the ONE
remaining lock on `evaluate::<Dual64>` (#687, pinned by
`editor-core/tests/e4_dual_door.rs`), and it carries the
memo-sharing design question; interval-valued parameter binding
(`Doc::param_env` embeds points only); the `drive(doc, box)` driver;
body-to-body minimum distance (only local clearance *screens* exist:
fillet battery, shell's planar `wall_clearance`); `Expr` has no
function-call or entity-reference vocabulary (16 arithmetic
variants), so E3's measurement primitives are new language surface;
the F1 lattice has no Length-power dimensions.

**One fact that re-scopes E8: the W2 sketch solver was never
built.** `WitnessSlot` is an empty struct; there is no constraint
type, no contraction-from-f64-witness, no `solver_branch_margin`.
Documents contain no witness-carrying nodes, so E8's solver-wall
composition (`Infeasible`, `Bifurcation(WitnessBifurcation)`) is
*vacuously satisfied* in v1 — the driver's refusal vocabulary
carries the variants (they are ratified E6 vocabulary and the enum
is cheap), but no v1 input can reach them, and the plan says so
rather than building solver machinery to make them reachable
(see Q1 below).

## The slate

Ordered by dependency; D is design-before-code (the M9-D
precedent). Each unit gets its own spec at dispatch; driver
constants, sampler choices and report shapes are PR-spec per
ERROR-DESIGN's own "Open after this doc" list.

- **M10-D — the Dual contract (design pass; design-conversation PR,
  Evan-ratified).** Answers the roadmap entry's collected question:
  *what does a `Dual` actually have to do*, and cleans up the
  `Bounds`/`CertifiedEnclosure` split on that answer. Concretely:
  (i) **#687** — `ContentBits for Dual` and whether the seed enters
  the memo key. The candidate resolution to probe first: content
  bits over BOTH channels (value + tangent), under which cross-pass
  memo sharing is exactly as sound as bit-equality — a node
  untouched by the seeded parameter carries identical bits in every
  pass (sound reuse), a node downstream of it differs in tangent
  bits (distinct keys) — so no key-schema fork is needed and E4's
  n-pass cost model keeps shared-subgraph reuse. (ii) What a
  `Dual<T>` document build DOES at the sites where certified lanes
  refuse (`PropsQuadLane`'s tier-3 volume arm first: the value
  channel is the f64 build bit-identically, so what the dual build
  is entitled to reuse vs. re-decide needs one stated rule, not
  four local ones). (iii) The `Bounds`/`CertifiedEnclosure`/
  `Enclosure` residue: #701 (the allowlist gate greps `Bounds`,
  not `Enclosure`, and a `Dual` IS an `Enclosure` since D1), the
  lapsed `sweep::fillet` seam justification recorded in #687, and
  whether the four refusing lane splits stay as-is (the lean: yes —
  *a dual may not certify* stands; E4/E9 need tangent data, never
  certification). (iv) The poison-vs-widen boundary at certified
  enclosure lanes, made concrete by a measured live case handed
  over by PCURVE on this plan's PR (2026-08-29): a chart residual
  provably exact at f64 (`0e0`) reaches the caller as
  `margin: Invalid` — poison, "never validly posed" — at Interval
  with a conversion present, A/B-controlled on one head; the class
  gets its own issue (PCURVE files it at M10's request; the
  instance's mechanism stays with P-1b), and M10-D consumes it as
  evidence, since a refusal that says *ill-posed* over exact
  geometry differs visibly from *the enclosure was too wide*. No
  unit implements against `Dual` before this ratifies.
- **M10-1 — distributions in the document (E1/E2).**
  `DocParam::Continuous` gains the optional `Distribution` (Band /
  Uniform / Normal / TruncatedNormal, offsets dimensioned by the
  param's own `dim`, `lo ≤ 0 ≤ hi` where bounded); `Count` stays
  distribution-free *by representation* (E11.3's refusal comes out
  unrepresentable, the house preference). Schema step v15 per the
  live clean-break mechanics (see Q4). The analysis-lane projection
  vocabulary (analyzed box from the quantile dial, seed vectors,
  the measure pricing leaves/tail) lands as derived, never
  persisted, types — no driver yet, but the tail-mass arithmetic
  and the Band-refuses-measure rule are this unit's testable
  surface, plus the PL6 independence semantics stated where the
  projection lives.
- **M10-2 — `Measure` nodes and `Assertion`s (E3 + E10's persisted
  half).** The one dimension-generic `Measure { expr }` sink; the
  F7 typed-function surface over `StableName` references —
  v1 primitives `distance`, `angle`, `min_clearance(sel)`, and
  C5's `gap(declaration)`; `Assertion { measure, bound, dir }`,
  report-only (E10's v1 answer). Evaluated at every `T` through the
  existing service; resolution failures typed through N-machinery;
  content-keyed like every node. Mass-property primitives and the
  F1 Length-power lattice growth are banked (Q2). Schema step v16.
- **M10-3 — the E6 subdivision driver.** The interval parameter
  door (`param_env` learns non-degenerate intervals);
  `drive(doc, box) -> ParamBoxVerdict`; leaf replay at
  `T = Interval` with certification = every predicate definite AND
  the leaf's `VerdictLog` matching the f64 witness build's
  (float-free comparison, already exact); no-flips refusals naming
  flipped predicates; the named deterministic split rule; terminal
  slivers and typed `Budget`; product-measure leaf/tail accounting
  summing to 1 with the merged unresolved budget; the chamber
  containment predicate on the leaf set; k_stats funnel rows from
  driver-path predicates (the E6 obligation — the first genuinely
  ill-conditioned population K sees); rayon idiom 1 over leaves.
  `Infeasible`/`Bifurcation` variants present, unreachable-in-v1
  documented at the type.
- **M10-4 — sensitivities and the stackup (E4/E5); after M10-D and
  M10-2.** `evaluate::<Dual64>` opens (the `e4_dual_door` suite
  flips to its successor law); n seeded passes, pure and parallel;
  every sensitivity carries a chamber certificate (an M10-3 leaf)
  or `local_only` — no third state; the `Stackup` report with
  certified `worst_case` as the only gating number, advisory
  `per_param`/`rss` with `UnavailableBecause` (Band contributors;
  E9 tangent forfeiture read from the public tangent fields, not
  from `Bounds`); `Dual<Interval>` enclosures consumed for
  contribution bounds and E7 pruning only — never refusal (E9, and
  the D1 ruling unmoved: nothing here certifies through a dual).
- **M10-5 — clearance and self-intersection (E7); after M10-3.**
  The two nested subdivisions: E6 leaves outer, geometry-domain
  interval exclusion inner (the SSI-exhaustiveness posture run with
  interval parameters, its receipt-identity template reused); a
  conservative interval BVH (`Aabb` is already `T: Bounds`-ready);
  the trichotomy `Holds / Violated{witnesses} / Refused{sliver |
  budget}` with the f64-verified violation witness; global
  self-intersection as the census made global and parametric
  (non-adjacent pairs strictly positive distance; carriers without
  interval evaluators refuse `Unsupported`, never sample);
  monotonicity pruning by sign-definite `Dual<Interval>` ∂d/∂pᵢ as
  an accelerator only. Named consumer: **#1055** — the shell verb's
  curved wall-clearance window cites this certificate as what
  closes it (Q5 below decides where that arm lands). VERBS
  registered the demand on the plan PR (2026-08-29): the
  curved-neck shell case is a ready-made acceptance with fixtures
  in-tree (the ordinal-101/103 probe suites' dumbbell/hexagon
  families), and #1019's perf box names the shell body as a
  measurement fixture.
- **M10-6 — reporting, CI rows, the advisory lanes, the demo
  (E10/E11); after M10-4/M10-5.** Content-key-cached, serializable
  verdicts/stackups for goldening (bits of recipe slice, box, ε, K);
  the three E10 CI rows: assertion gating on corpus assertions
  (`Holds` + unresolved mass within the recorded budget), goldened
  refusal/tail-mass accounting on a margin-thin fixture, k_stats
  rows for driver predicates; the MC advisory estimator lane
  (E11.1's label discipline: sample count + recorded seed, never
  persisted as assertions — Q3) and the E11.6 leaf-mass histogram
  note; the worked example lands as a tour cell — the two-hole
  plate, certified worst-case vs. RSS optimism printed side by
  side with the tail riding every line, which is the MVP's reason
  to exist rendered as a demo.

Cross-program interfaces, named so "error" does not become a
bucket: the PARAM-LINT unit (DISCIPLINES) is not ours and blocks
nothing here; PCURVE owns edge-description migration (M10-5
consumes carriers through whatever descriptions exist at dispatch);
mass-property enclosure quality (#870) is props' own meter, not an
M10 unit.

## Open for Evan (this plan's questions)

- **Q1 — the sketch solver is NOT in this slate (proposed).** The
  roadmap entry folds "sketch solver when sketches should become
  constraint-driven" into M10, but nothing in E1–E11 requires it,
  no document today carries a witness, and building W2 machinery
  mid-milestone so that E8's refusal arms become reachable is
  speculative scope. Proposal: M10 = the ERROR-DESIGN MVP; the
  solver (DESIGN.md Q3, ezpz-vs-roll-our-own) re-opens as its own
  design pass when constraint-driven sketches have a consumer.
  Counterargument, honestly: E8's walls are the sharpest no-flips
  case and shipping the driver without ever exercising them leaves
  that composition untested until a solver exists.
- **Q2 — mass-property Measures banked (proposed).** They force
  the F1 Length-power lattice growth (recorded as additive in E3)
  and sit behind `PropsQuadLane`'s refusing dual arm. The v1
  primitive set (distance, angle, min_clearance, gap) covers the
  worked example and the C5 contract; the lattice growth lands
  with its first real consumer.
- **Q3 — the MC advisory lane rides M10-6 (proposed).** E11.1
  softened it to a labeled advisory lane and pure replay makes it
  cheap; the alternative is banking it post-milestone. In either
  case it never gates.
- **Q4 — schema steps follow the LIVE clean-break ruling.** E10's
  "the migration chain gains one explicit version-to-version step"
  predates LQ7a's empty-migration-table posture (every pre-release
  bump is a clean break refusing typed). Proposal: v15/v16 as
  clean breaks like v2–v14; E10's sentence reads as written for
  the post-release world. Flagged because it is a ratified-text
  divergence, however mechanical.
- **Q5 — where #1055's curved arm lands.** The certificate is
  M10-5's; the consuming gate site is `topo::shell` (VERBS
  territory). Proposal: the curved `wall_clearance` arm lands IN
  M10-5 as its acceptance-grade real consumer (the demo-purpose
  rule: real usage over synthetic fixtures), with VERBS holding
  right of first refusal per the A′ precedent.

## Process

Standard, v6: substrate → binding spec → one implementer + the
cross-model dual review + union fix pass; arms drawn per the
{opus, opus, fable} block rule, blocks recorded branch-side until
conclusion; ordinals claimed on main at review dispatch from band
500–599; record-at-merge with per-phase tokens/wall-clock; blinding
discipline verbatim (no `Co-Authored-By` in lane commits; no
arm-naming surface reviewers can read). Hosted CI is the only gate;
every new row ε-three-outcome honest; reviewer suites promote as-is
and may be retired per policy. Implementer dispatches point at
`docs/prompts/implementer-discipline.md` by path; reviewers get
explicit claims to falsify plus `docs/prompts/reviewer-style-lane.md`.

**This orchestrator runs in a remote container** (the GUI
precedent): no persistent `~/.local/share/cad-work`, GitHub through
MCP, session state committed and pushed obsessively because the
container is ephemeral. Disk is the binding constraint (~29 G):
lanes are worktrees sharing one object store, each with its own
`CARGO_TARGET_DIR`; at most ~2 concurrent lane targets plus
reviews, and a review lane's target is reclaimed the moment its
report is in hand. The build-slot mutex, CONFLICTING-means-silent-
CI, and push-early rules bind unchanged.

## Exit shape (proposed)

Distributions, Measures and Assertions persist and round-trip;
`drive` certifies, refuses and prices honestly with coverage
summing to 1 and chamber containment reported when it holds; the
e4 door is open and every sensitivity is chamber-certified or
`local_only`; stackups gate on certified worst-case only; the
trichotomy answers over box × domain with f64-verified violation
witnesses; the Dual question is ANSWERED and the
`Bounds`/`CertifiedEnclosure` cleanup landed (#687, #701 closed);
the three E10 CI rows are live; the two-hole-plate cell ships in
the tour; k_stats carries driver rows (the K re-open trigger
armed with real data); every unit merged on its own green hosted
head; the walk convention applies at exit.
