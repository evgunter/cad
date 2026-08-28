# Memory Index

Read the files as relevance dictates; this index says what each is for,
not what it says. **Live status is never here** — it is the tail of the
relevant `docs/*-LOG.md`.

**The three concurrent programs and their live logs:** kernel
milestones (`docs/M9-LOG.md`, plan `M9-PLAN.md`), LIB —
usable-as-a-library (`docs/LIB-LOG.md`), ASM — assemblies
(`docs/ASM-LOG.md`, plan `ASM-PLAN.md`). Ratified design is
`docs/DESIGN.md` plus its companion table. Merge gate = hosted Actions.

## Working with Evan

- [CAD working style](cad-working-style.md) — discuss → ratify into
  DESIGN.md → commit; doc prose states the present only; **how to write
  a memory** (the two tests, and be brief)
- [Docs ledger](docs-ledger.md) — `docs/` is pruned, not archived; a
  pointer to a missing `docs/` file resolves in `docs/DOC-LEDGER.md`
- [Evan profile](evan-profile.md) — differential-geometry fluent; define
  CAD jargon, don't simplify the math; probes fudged invariants
- [Git workflow](git-workflow.md) — merge-only, no history rewriting;
  documentation lives in PR descriptions; agents self-merge to main
- [Demo purpose](demo-purpose.md) — demos demonstrate REAL usage;
  awkwardness is a library finding, never hidden

## Running the work

- [Orchestration model](orchestration-model.md) — orchestrator plans and
  meta-reviews, subagents code and review; when to self-merge vs wait
  for Evan; standing operational rules (monitors, away-channel, subagent
  brief headers)
- [Orchestrator switch runbook](orchestrator-switch-runbook.md) —
  RUNBOOK, read only when handing off to a successor
- [Agent lane operations](agent-lane-operations.md) — lane creation,
  build-slot locks, disk, liveness, death recovery, CONFLICTING
- [Usage limit protocol](usage-limit-protocol.md) — why sessions must
  stop BEFORE the window fills, and the WARN/STOP/RESET actions
- [Model A/B experiment](model-ab-experiment.md) — the standing
  Opus-vs-Fable implementation experiment; `docs/MODEL-AB-LOG.md` is
  normative and owns every live number

## Testing, review, measurement

- [Local battery scope](local-battery-scope.md) — hosted CI is the gate
  AND the cheap option; local runs only when they beat the gate to a
  failure
- [Review and dependency policy](review-and-dependency-policy.md) —
  reviews run real e2e demos; reviewer suites promote as-is and may
  always be retired; ~2-week minimum dependency age
- [Test suite cost](test-suite-cost.md) — ask which SHAPE a test is
  before giving it a seed; effort dials; assertion-free tests never gate
- [Perf measurement lane](perf-measurement-lane.md) — a timing is worth
  nothing without its box; hosted CI produces them, history is
  append-only, reporting never gating
- [Tessellation budget](tessellation-budget.md) — MEASURE whether a mesh
  is bigger than it needs to be; the anisotropic-sliver lesson; and
  WHERE instrument belongs (gating does not answer volume)
- [FreeCAD render lane](freecad-render-lane.md) — CI renders and
  re-baselines the lanes; PRs REPORT (neutral, not a failure), main
  COMMITS; FreeCAD's two failure modes; the per-process budget

## Kernel rules

- [Output stability as justification](output-stability-as-justification.md)
  — byte/bit-preservation may choose among equivalent implementations,
  never justify keeping code; and the three uses of that vocabulary it
  does not touch
- [K telemetry state](../docs/K-REPORT.md) — not a memory: K = 10 is the
  permanent ratified default, #89 CLOSED; check a landing's margin
  DIMENSION before reading it as K evidence
- [A refusal's text is not evidence of its cause](refusal-text-is-not-cause.md) — five measured instances; measure-first is mandatory; the payload and raising site are the instrument
