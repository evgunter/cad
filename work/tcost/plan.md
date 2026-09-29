# S-TCOST — test-suite cost (plan)

Opened on Ev's direction (in-chat, 2026-09-02): speed up the test suite
"without too much cost to its power to detect defects", with the six
levers in the charter below named as in scope and Opus subagents
implementing. §Review carries what binds today. Live state is
`work/tcost/log.md`'s tail, never this file.

Branch prefix (the #396 convention): **`tcost/`** — unit branches
`tcost/<unit>-<slug>`, orchestrator branch `tcost/orchestrator`.
Away-channel tag `(S-TCOST orchestrator)`. Unit names occupy
`TCOST-<n>`, kernel units `TCOST-K<n>`, build-side units `TCOST-B<n>`.
The A/B ordinal band **S-TCOST = 1400–1499** is claimed and closed —
three ordinals drawn (1400–1402) and no more, since the protocol is off
for this program (§Review); `docs/MODEL-AB-LOG.md`'s banding entry is
the record.

## Charter (Ev, in-chat, 2026-09-02 — in substance)

Make the suite cheaper while keeping its detection power. In scope, by
name:

1. the history of CI — which tests have EVER been red, and what each
   red was;
2. per-test timing, read from hosted runs;
3. **gating tests that are specific to the logic of a few files to
   changes to THOSE files**, rather than to any ancestor of them (the
   change filter keys the build on the crate closure; `gated_to!`
   markers key a suite on its files);
4. combining tests that share initialization into one test that
   asserts several things;
5. deleting tests already covered by other tests;
6. making tests use simpler objects.

Ruled with the charter (Ev, in-chat, same day): units land as **their
own PRs**, merged to main by this orchestrator; **build-side levers are
in scope too** (what makes the test-binary build slow, under whatever
§Review binds); the per-file gate mechanism (lever 3) is **self-merged**
with a full writeup as an elaboration of the ratified gating rule,
reviewed retroactively.

## Ratified ground (cited, not re-litigated)

- `docs/prompts/implementer-discipline.md` §8 — the three shapes of a
  test and which wants a varying seed; the EFFORT dial; a fuzzer runs
  at EFFORT = 1 and its `gated_to!` marker names the paths whose change
  raises it; merge tests sharing an expensive fixture and LABEL every
  assertion; an assertion-free test never gates.
- `memories/review-and-dependency-policy.md` — **retirement is always
  permitted**: delete (naming the row that now owns the claim),
  `#[ignore]` a reporting row, or gate a sweep on the change filter.
  A promoted reviewer suite's independence is worth keeping where it
  pulls its weight; that is not a prohibition on retiring the rest.
- A run may skip a detector whose subject PERSISTS in the tree; it may
  not skip a detector of ABSENCE. A gated test's break persists, so
  gating is the persistence case, argued per row.
- `memories/perf-measurement-lane.md` / `memories/local-battery-scope.md`
  — committed numbers come from hosted CI only; local timings are
  iteration tools and are never quoted as the result.
- `memories/output-stability-as-justification.md` — a test that is
  kept only because its output has not changed has not been justified.
- The aggregation invariant (`scripts/gates/test-aggregation.sh`, one
  test target per crate) and `autotests = false` — a retired suite
  file leaves `tests/all.rs` in the same commit.

## Where the cost is

`work/ciw/latency-cut.md` carries the measurement the current CI is cut
on: the build is the floor, and test time is a Pareto tail — the tests
at ≥ 1 s hold most of it and form the slow set (`.config/nextest.toml`'s
`ci` profile), which the per-PR gate skips except for the crates a diff
seeds, and which the nightly runs at every eps row. So the two halves of
the bill are (a) the compile of the test targets, held by test-code
volume and its generic instantiations, and (b) the slow set's
execution, which moves nightly cost and a seeded PR's latency.

## Method

A unit is cut from a census, largest share first:

- **Red history** — which tests ever went red on hosted CI, how often,
  at which eps row, classified (defect caught / stale pin / eps-band
  sensitivity / infra / inherited red). An expensive test that has never
  been red is a deletion candidate only together with the question of
  what it would catch.
- **Timing** — per-test time from hosted runs' nextest output,
  aggregated by test and by suite file.
- **Build profile** — `cargo build --timings` under CI's profile env:
  lib-vs-test split, per-crate test-target compile time against test
  source lines and binary size.

Every unit's PR states its before/after from hosted runs and names, for
every retired or merged row, the row that now owns each claim.

## Levers and their units (live list in the log)

- **TCOST-1 — the per-file gate** (charter lever 3; spec at
  `work/tcost/TCOST-1.md`). An in-file `gated_to!` marker names the
  source paths a suite covers; `scripts/ci-filter.py` reads the markers
  and the diff and emits a nextest filterset (`TEST_FILTER`) that the
  per-PR `test` job applies; it fails OPEN (no filterset) on tier `all`,
  on any unresolvable marker, and on any parse error. The nightly's
  `full-suite` runs every gated suite, so a break a marker did not name
  surfaces within a day. `fuzz-depth-not-existence-run-everything-at-effort-1`
  turns the filter from existence into depth.
- **TCOST-2…** — content units per suite family (levers 4–6): merge
  rows that rebuild one fixture, delete rows another row owns, replace
  heavy objects with the smallest object that carries the claim, and
  gate what is file-specific.
- **TCOST-K<n>** — kernel-logic units: where a test is slow because the
  CODE it exercises is slow in a way the real program pays too, the fix
  is a kernel change. §Review decides the review track, on the unit's
  risk of being wrong.
- **TCOST-B<n> — build-side**: test-code volume, generic instantiations
  in test code, dev-dependency graph. Any change to CI's build knobs
  (profile, cache) is out of this program unless a unit's measurement
  makes the case, and then it is its own PR with its own hosted
  measurement.

## Review (Ev, in-chat, 2026-09-12)

**The A/B protocol is off for this program.** Ev, in-chat 2026-09-12:
*"don't use the AB protocol and just do style reviews unless it's a unit
with high risk of being wrong which deserves a full review"*. So there
is **one default and one exception**:

- **Default: a style review** (`docs/prompts/reviewer-style-lane.md` by
  path), plus the unit's own claims to falsify. No arm is drawn, no
  ordinal is claimed, nothing is recorded in `docs/MODEL-AB-LOG.md`.
- **Exception: a full review**, where the unit carries a **high risk of
  being WRONG** — whether a defect could land and go unseen. What raises
  it here: a claim of bit-identical output that only a digest can check,
  a refusal class that must not move, a change to what CI RUNS (a gate
  or a partition can drop tests and read as a faster run), and a
  measurement whose conclusion inverts on the figure being right. A
  test-only diff can qualify on any of those, and a kernel diff can fail
  to.

The orchestrator names which track a unit gets **in its dispatch**, with
the reason, so a reviewer knows what standard is being applied to them.

Hosted CI is the only gate. Implementer dispatches point at
`docs/prompts/implementer-discipline.md` by path; reviewer dispatches
point at `docs/prompts/reviewer-style-lane.md` by path.

## Process

The orchestrator may run in a remote container: no persistent
`~/.local/share/cad-work`, no script monitors, GitHub through MCP; lanes
are worktrees under `~/tcost-work/`, each with its own
`CARGO_TARGET_DIR` outside the worktree, at most one heavy cargo build
at a time on the box. Decisions taken unilaterally are logged in
`work/tcost/log.md`.

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
  thresholded.
