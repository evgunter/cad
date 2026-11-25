---
id: a-test-timed-alone-still-reads-the-runners-speed-class
kind: issue
title: A test timed alone still reads the runner's speed class: two drift runs named 134 and 242 tests
status: open
opened: 2026-11-22
priority: P1
cost: M
design: true
refs: [7448, 7397]
---


Filed by the GREEN `green/fast-set-bar` lane (U11, PR 7448), 2026-11-22.

## What was measured

PR 7448 has the drift check (`scripts/fast-set-drift.py`) re-time fast-set tests alone, three times each, with the fastest kept, on main's build. Two dispatches of it ran on the same two PR runs' readings:
- run 45084526868 named 134 tests (65 app, 69 kernel);
- run 45093266519 named 242 tests (30 app, 212 kernel).

The screen differed between the two runs (a median of 0.324 s against 0.25 s), but that does not explain the gap. On the tests both runs timed, the second run's alone readings were **0.768×** the first's on the app shard (68 tests) and **1.323×** on the kernel shard (236 tests). These are the runner speed classes U10 found (about 0.8, 1.0 and 1.33), and timing alone does not remove them. 95 tests were over 0.25 s alone in both runs, and 281 were over in at least one.

## What it costs

Both readers of the 0.25 s bar take one runner's reading:
- **The gate** (`check-fast-set-times.py`) reads the PR's own runner. A test that costs 0.2 s on a 1.0 runner reads about 0.26 s on the slow class. Designer B accepted that flap in PR 7397, as a small one on the author's side.
- **The drift check** reads its own runner, so its list swings with that runner's class. On the slow class the list includes cheap tests, and moving those cuts the fast set for nothing. On the fast class it misses tests.

## What is not decided

- Take the drift list only from tests that are over the bar on more than one day's run.
- Normalise each run's readings by a reference workload timed on the same runner.
- Take the bar on the median runner class: drop or rescale a run whose reference reads a slow or fast class.

Choosing among these changes how the ratified rule is measured, so it is a design question for GREEN (and Ev, if it changes the rule's text).

## Who reads the drift

GREEN's orchestrator reads the latest run of `.github/workflows/fast-set-drift.yml` at the start of each sitting, since a scheduled red notifies only by email. Nothing in `memories/` or `docs/prompts/` says so yet: writing it there is Ev's call (PR 7448's review, finding 8).

## 2026-11-24: two designers agree; asked of Ev

Two Opus designers weighed this independently. Their reports are at
`~/.local/share/cad-work/green-speedclass-designer-{a,b}.md`, and both
reach the same recommendation.
- **The speed class is one multiplier per runner, not noise.** One test
  timed three times alone on one runner agrees to within about 1–2%. The
  two drift runs differ by one factor over the whole population: 0.77× on
  app and 1.32× on kernel, with deciles of 0.75–0.81 and 1.26–1.49.
- **Dividing that factor out makes the lists agree.** The app lists then
  agree on 68/68 and the kernel lists on 223/238, and the 15 left over sit
  near the bar.
- **The recommendation:** time a fixed, dependency-free reference
  workload in each re-time pass, and scale every alone reading to the
  median hosted runner. One definition, in `scripts/fast_set_alone.py`,
  serves the gate and the drift check alike. Both screens widen to the
  bar × 0.75.
- **What it costs:** about 0.25 s per re-time pass, and only on jobs
  that re-time anything.
- **Rejected:**
  - persistence across days, which filters noise and cannot remove a
    bias;
  - dropping off-class runs, which leaves no drift reading on most days;
  - a table from CPU model to speed class.

The gate half rewords ratified text (`implementer-discipline.md` §2) and
reverses PR 7397's acceptance of the gate-side flap, so it is asked of Ev
in an `[ev]` PR.

**Until Ev rules,** the orchestrator moves only tests the drift names at
0.33 s or more raw. A test that reads that raw costs at least 0.25 s on
any runner class.
