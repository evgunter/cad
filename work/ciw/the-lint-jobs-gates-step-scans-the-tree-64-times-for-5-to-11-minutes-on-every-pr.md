---
id: the-lint-jobs-gates-step-scans-the-tree-64-times-for-5-to-11-minutes-on-every-pr
kind: issue
title: The lint job's gates step runs 64 single-threaded scans of the tree, 5 to 11 minutes on every PR, a docs-only one included
status: closed
opened: 2026-11-22
priority: P0
cost: M
closed: 2026-11-23
---


Filed by BOXFINISH (2026-11-22) on Ev's word (PR 6993: "the lint is
running really slowly for some reason. if it's genuinely slow and not
having some one-off issue that should be filed as a P0"). It is
genuinely slow, and systematic. `work/README.md` (Priority, P0) names
PR CI slower than its latency alarm as P0 work; this step is the
floor under every PR's time to `gate ok`.

**Measured** (read from the Actions jobs API and five job logs,
2026-11-22; nothing was re-run).

- `lint (fmt, clippy, gates)` over 12 PR runs today: run time median
  7:55, range 5:37 to 18:34. Docs-only (clippy skipped): 6:31, 5:37 to
  11:09, n = 9. Code: 14:22, 11:57 to 18:34, n = 3.
- **One step is the cause: `gates (scripts/gates)`.** 5:22 to 10:54,
  median 6:54, and 94 to 97% of a docs-only lint job. It has no `if:`,
  so a docs-only diff runs it whole; only toolchain, cache, rustfmt,
  clippy and rustdoc are skipped on such a diff.
- **Inside it** (`scripts/run-gates.sh`, PR 7011's log): 64 gates,
  four at a time. 29 take 61 to 134 s each, 14 take 7 to 35 s, 21 take
  5 s or less. Slowest, on a slow and a fast runner:
  `bare-rounding-allowlist` 134 s / 81 s, `escalation-question-marks`
  104 / 58, `cover-kind-match` 96 / 55, `reporting-margin-door` 83 / 47,
  `interval-square-allowlist` 82 / 45. `payload-rung-sweep.py` adds 14
  to 22 s.
- **Why each is slow:** every gate is its own single-threaded bash and
  awk scan of 773 to 1,015 source files (some over 3,000) through
  `gate_rust_code` in `scripts/gates/lib.sh`. Nothing is shared between
  gates.
- **The hosted runner doubles it.** The 64 gates cost 1,370
  CPU-seconds on an AWS-hosted runner (step 6:09) and 2,433 to 2,440 on
  two Azure-hosted ones (step 10:52 and 10:54). The 11-minute lint jobs
  are that.
- **It was about five minutes three days ago** (docs-only lint 5:13,
  5:17, 5:27 on 11-18 and 11-20; gates 4:59 to 5:12), so it grows with
  the tree and the gate count. Not separated: tree growth against
  runner mix.
- **Queue wait is second, and separate:** lint's wait for a runner was
  0:01 to 0:03 before 03:10 UTC and 0:02 to 5:12 between 03:14 and
  04:20, with CI runs active repo-wide at about twice the baseline (10
  to 15 against 5 to 7). `change filter`, which lint `needs`, costs
  1:05 to 1:33 before lint can queue (a full-history checkout 33 to
  58 s, classify 17 to 30 s).
- **The push-to-`gate ok` times that raised this** (docs-only PRs):
  PR 7011 16:03 (lint ran 11:09), PR 7337 14:32 (lint ran 6:29, queued
  3:55), PR 6993 lint queued 2:25 then ran 11:09.

**Also found, each small:**

- Code PRs log `No cache found.` for `shared-key: lint`: `save-if` is
  main-only and lint does not run on pushes to main (it needs `filter`,
  which is `if: github.event_name != 'push'`), so clippy and the
  rustdoc pass compile cold on every code PR (2 to 3 min and 3 to
  4:37).
- `scripts/ci-filter.py`'s header says a `docs-only ok` marker job "is
  the whole gate" for `TIER=docs`; `ci.yml` has no such job.

**Not measured:** job-level concurrency against the account's cap
(`ci.yml`'s header says 20; unconfirmed); the runner's cloud for 7 of
today's 12 jobs; 11-17 and 11-19.

**What this row asks.** The gates step under a minute or two on a
docs-only PR, and not growing with the gate count: whether by one
shared read of the tree that every gate consumes, by running only the
gates whose inputs a diff touches, or otherwise is the owner's to
weigh (`scripts/gates/*` is GUARD's ground by CIW's `keep_out`; a gate
that reads prose or tracker files still has to run on a docs-only
diff). The bar is `memories/orchestration-model.md`'s: CI takes 15
minutes at most and typically under 10.
(BOXFINISH orchestrator)

## Closed

PR 7439 (`ciw/gates-step-latency`). The gates step on a hosted runner went
from 5:22–10:54 (median 6:54) to 1:06 (run 45069292031, lint job, step
21:44:41→21:45:47, `payload-rung-sweep.py` included); locally 790 → 152
CPU-seconds. `run-gates.sh` reads the tree once into a view cache
(`lib.sh`, "THE VIEW CACHE") that every gate's reader serves from,
`view-cache-agrees.sh` holds it to the reader on every run, the mount
resolver answers a file's declarations in one pass, the Python gates
share one jumping lexer (`scripts/gates/rust_strip.py`), and the slowest
gates' own costs were cut. Verdicts unchanged: plain output byte-identical
on a clean and on a planted-red tree against main, self-tests unchanged.
The step's cost now grows with the tree's views, not with the gate count.
The lint cache's cold compile is primed by `cache-prime-lint`.
