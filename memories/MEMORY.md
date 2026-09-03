# Memory Index

Read the files as relevance dictates; this index says what each is for,
not what it says.

**Finding the work.** `work/` is the tracker and `work/README.md` its
contract: `work/STATUS.md` is the board, `work/<program>/` holds a
program's `program.md`, `plan.md`, `log.md` and one file per open
item; **live state is there and never here.** A program is closed when
its `docs/<NAME>-EXIT-WALK.md` is ratified, and that walk is then its
done-state of record. Ratified design is `docs/DESIGN.md` plus its
companion table. Merge gate = hosted Actions.

## Working with Ev

- [CAD working style](cad-working-style.md) — discuss → ratify into
  DESIGN.md → commit; doc prose states the present only; **how to write
  a memory** (the two tests, and be brief)
- [Docs ledger](docs-ledger.md) — `docs/` is pruned, not archived; a
  pointer to a missing `docs/` file resolves in `docs/DOC-LEDGER.md`
- [Ev profile](ev-profile.md) — differential-geometry fluent; define
  CAD jargon, don't simplify the math; probes fudged invariants
- [Git workflow](git-workflow.md) — the hazards CLAUDE.md's merge-only
  rules leave out: issue-closing keywords, stacked branches, unprotected
  main, identifiers that stay off GitHub
- [Demo purpose](demo-purpose.md) — demos demonstrate REAL usage;
  awkwardness is a library finding, never hidden (cited by name from
  source comments across the tree)

## Running the work

- [Orchestration model](orchestration-model.md) — orchestrator plans and
  meta-reviews, subagents code and review; when to self-merge vs wait
  for Ev; standing rules for monitors, channels and dispatches
- [Orchestrator switch runbook](orchestrator-switch-runbook.md) —
  RUNBOOK, read only when handing off to a successor
- [Agent lane operations](agent-lane-operations.md) — lane creation,
  build-slot locks, disk, liveness, death recovery, and the ways CI
  silently does not run
- [Model A/B experiment](model-ab-experiment.md) — the standing
  Opus-vs-Fable implementation experiment; `docs/MODEL-AB-LOG.md` is
  normative and owns every live number

## Testing, review, measurement

- [Local battery scope](local-battery-scope.md) — hosted CI is the gate,
  the cheap option, and the only producer of committed measurements;
  local runs only when they beat the gate to a failure
- [Review and dependency policy](review-and-dependency-policy.md) —
  reviews run real e2e demos; when a stated gap blocks; reviewer suites
  promote as-is and may always be retired; ~2-week dependency age
- [Test suite cost](test-suite-cost.md) — ask which SHAPE a test is
  before giving it a seed; effort dials; assertion-free tests never gate
- [Tessellation budget](tessellation-budget.md) — MEASURE whether a mesh
  is bigger than it needs to be; where instrument may live; the
  anisotropic-sliver hazard behind the NURBS schedule
- [Perf measurement lane](perf-measurement-lane.md) — where committed
  timings come from and what may be done with them; reporting, never
  gating (cited by name from nightly.yml and the perf-data READMEs)
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
- [A refusal's text is not evidence of its cause](refusal-text-is-not-cause.md)
  — measure-first is a mandatory checkpoint; the payload and the raising
  site are the instrument
