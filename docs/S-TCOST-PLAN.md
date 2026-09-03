# S-TCOST — test-suite cost (plan)

**STATUS: OPEN (2026-09-02).** Opened on Evan's direction (in-chat,
2026-09-02): speed up the test suite "without too much cost to its
power to detect defects", with the six levers in the charter below
named as in scope, Opus subagents implementing, and the review split
in §Review ruled in the same conversation. Live state is
`docs/S-TCOST-LOG.md`'s tail, never this file.

Branch prefix (the #396 convention): **`tcost/`** — unit branches
`tcost/<unit>-<slug>`, orchestrator branch `tcost/orchestrator` (the
harness-designated session branch carries the opening PR and is
otherwise unused, per the S-CERT/S-QA/S-MESH precedent). Away-channel
tag `(S-TCOST orchestrator)`. A/B ordinal band **S-TCOST =
1400–1499**, claimed in `docs/MODEL-AB-LOG.md`'s banding entry in this
same PR, per that entry's rule; implementer blocks are named
`TCOST-B<n>` (unit names occupy `TCOST-<n>`).

## Charter (Evan, in-chat, 2026-09-02 — in substance)

Make the suite cheaper while keeping its detection power. In scope, by
name:

1. the history of CI — which tests have EVER been red, and what each
   red was;
2. the per-test timing history CI already records (the
   `test cost report` step of every test job; `scripts/slowest-tests.py`);
3. **gating tests that are specific to the logic of a few files to
   changes to THOSE files**, rather than to any ancestor of them
   (today's change filter keys on the crate closure);
4. combining tests that share initialization into one test that
   asserts several things;
5. deleting tests already covered by other tests;
6. making tests use simpler objects.

Ruled with the charter (Evan, in-chat, same day, on the orchestrator's
three questions): units land as **their own PRs**, merged to main by
this orchestrator; **build-side levers are in scope too** (what makes
the test-binary build slow, under the same review split); the
per-file gate mechanism (lever 3) is **self-merged** with a full
writeup as an elaboration of the ratified gating rule, reviewed
retroactively.

## Ratified ground (cited, not re-litigated)

- `memories/test-suite-cost.md` — the three shapes of a test and which
  wants a varying seed; the EFFORT dial; **a fuzzer that is not gated
  is a defect in the fuzzer**; failure isolation is worth less than
  per-run cost (merge tests sharing an expensive fixture, LABEL every
  assertion); a codomain assertion is a deletion; an assertion-free
  test never gates; cost concentrates savagely — profile before
  cutting; per-leg timings are not comparable without normalising.
- `memories/review-and-dependency-policy.md` — **retirement is always
  permitted**: delete (naming the row that now owns the claim),
  `#[ignore]` a reporting row, or gate a sweep on the change filter.
  A promoted reviewer suite's independence is worth keeping where it
  pulls its weight; that is not a prohibition on retiring the rest.
- `docs/CI-MINUTES-2026-08.md` §*What is NOT sampled* — a run may skip
  a detector whose subject PERSISTS in the tree; it may not skip a
  detector of ABSENCE. A gated test's break persists, so gating is
  the persistence case, argued per row.
- `memories/perf-measurement-lane.md` / `memories/local-battery-scope.md`
  — committed numbers come from hosted CI only; local timings are
  iteration tools and are never quoted as the result. This program's
  measurements of record are the `Slowest N tests` blocks and the
  job durations of hosted runs, read before and after each unit.
- `memories/output-stability-as-justification.md` — a test that is
  kept only because its output has not changed has not been justified.
- The aggregation invariant (`scripts/gates/test-aggregation.sh`, one
  test target per crate) and `autotests = false` — a retired suite
  file leaves `tests/all.rs` in the same commit.

## Where the cost is (hosted, 2026-09-02, one run per lane)

Read from runs 33645902263 (default lane, ε = 1e-12) and 33693735802
(interval lane, ε = 1e-12); re-taken by every run's own report, not
by this file.

| | default lane | interval lane |
|---|---|---|
| `build + archive` (critical path) | 10.5 min | 11.1 min |
| test shard wall | 63 s / 89 s | 197 s / 166 s |
| test cpu-s per shard (2/2, 1/2) | 165 | 388 |
| share of cpu-s in the top 20 tests | 68 % | 58 % |
| tests per shard | ~2 044 | ~2 445 |

Test source is ~300 k lines against ~300 k lines of `src/`; the
build job compiles all of it once per lane. So the two halves of the
bill are (a) execution, held by a few dozen tests, and (b) the
compile of the test targets, held by test-code volume and its
generic instantiations. The three censuses in §Method size both.

## Method

Three read-only censuses run first, each as a report under
`~/tcost-work/` (lane-private, never the shared scratchpad) and
summarised in the log:

- **Red history** — every nextest test that ever went red on hosted
  CI, how often, on which lane/ε, classified (defect caught / stale
  pin / ε-band or lane sensitivity / infra / inherited red). A test
  family that is expensive and has never been red is a deletion
  candidate only together with the question of what it would catch.
- **Timing history** — per-test cpu-s across ~50 green runs spanning
  both lanes and all ε rows, normalised per leg, aggregated by test
  and by suite file; the trend of the suite total.
- **Build profile** — `cargo build --timings` under CI's profile env
  on this container: lib-vs-test split, per-crate test-target compile
  time against test source lines and binary size, the incremental
  funnel.

Units are cut from the censuses, largest share first. Every unit's PR
states the before/after from hosted runs (the cost report's block,
and the job durations) and names, for every retired or merged row,
the row that now owns each claim.

## Levers and their units (fixed after the censuses; live list in the log)

- **TCOST-1 — the per-file gate** (charter lever 3; mechanism spec at
  `docs/TCOST-1-SPEC.md`). An in-file marker names the source paths a
  suite covers; `scripts/ci-filter.py` reads the markers and the diff
  and emits a nextest filterset that skips a gated suite whose named
  paths and own file are untouched; both CI halves consume it; it
  fails OPEN (no filterset) on tier `all`, on any unresolvable marker,
  and on any parse error. The nightly runs the gated set ungated so a
  break caused by a change the marker did not name surfaces within a
  day. First users: the existing fuzz rows and randomized sweeps,
  which the ratified rule already says must be gated.
- **TCOST-2…** — content units per suite family (levers 4–6): merge
  rows that rebuild one fixture, delete rows another row owns, replace
  heavy objects with the smallest object that carries the claim, and
  gate what is file-specific. One unit per family or crate, cut from
  the timing census's top families.
- **TCOST-K<n>** — kernel-logic units: where the census shows a test
  is slow because the CODE it exercises is slow in a way the real
  program pays too, the fix is a kernel change and runs under the
  A/B protocol (§Review).
- **TCOST-B<n>… build-side** — cut from the build profile: test-code
  volume, generic instantiations in test code, dev-dependency graph.
  Any change to CI's build knobs (profile, cache, sharding) is out of
  this program unless a unit's measurement makes the case, and then
  it is its own PR with its own hosted measurement.

## Review (Evan, in-chat, 2026-09-02)

Two tracks, decided per unit by what the diff touches:

- **Test-only changes** (`crates/*/tests/`, `#[cfg(test)]` modules,
  `scripts/`, workflow wiring for the gate): Opus implementer; a
  **light style review by Opus over a BATCH of several units**
  (`docs/prompts/reviewer-style-lane.md` by path, plus the batch's
  claims: every retired claim has a named owner, no assertion
  weakened, labels unambiguous, the before/after is hosted). **Not
  recorded in `docs/MODEL-AB-LOG.md`** — these are not
  implementation rows.
- **Changes to underlying logic** that makes a test (and the real
  program) slow: a binding spec, then the standard v6 unit — arm
  drawn per the current block rule in `docs/MODEL-AB-LOG.md` (read on
  main at each dispatch), ordinal claimed on main at review dispatch
  from band 1400–1499, cross-model dual review with the union fix
  pass, record-at-merge with per-phase tokens/wall-clock, blinding
  discipline verbatim (no `Co-Authored-By` in lane commits; no
  arm-naming surface a reviewer reads).

Hosted CI is the only gate. Implementer dispatches point at
`docs/prompts/implementer-discipline.md` by path.

## Process

**This orchestrator runs in a remote container** (the S-MESH
precedent): no persistent `~/.local/share/cad-work`, no script
monitors, GitHub through MCP; lanes are worktrees under
`~/tcost-work/`, each with its own `CARGO_TARGET_DIR` outside the
worktree, at most one heavy cargo build at a time on this box
(4 cores, 15 GB, ~29 GB free). Decisions taken unilaterally are
logged in `docs/S-TCOST-LOG.md`.

## Keep-outs

- No test is deleted for being slow alone: every deletion names the
  row that owns the claim, and a fuzz or property row keeps its
  detection power by being GATED or by its EFFORT dial, never by a
  cut count that leaves it running on every run at a weaker depth.
- No fixed seed is introduced; no `#[ignore]` on a row that gates
  (only on rows that report).
- Reviewer suites that pull their weight keep their independence from
  shipped fixtures (`memories/review-and-dependency-policy.md`).
- Nothing here gates on a millisecond: cost is reported, not
  thresholded (`scripts/slowest-tests.py`'s header).
