# GREEN — the log

## 2026-10-28 — opened

Cut across the infra group on the user's request: most infra tracks
carried one or two rows that matter beside a long P3/P4 tail, so the
cut follows the priority seam rather than a semantic one (the user, in
chat: *"i know we don't normally split on a priority seam but there
are a bunch of infra programs that have like 1 actually important
thing"*). Tracks with a reasonable P0/P1 count of their own could stay;
none had more than two.

Nightly 74 (run 40637814390, main `a1cfd1cfdb`) set the slate: red on
`full suite`, `rustdoc (gate, every root)`, `k-lint (dev-probe)` and
`PR CI latency (alarm)` (median 23.1 min, n=183).

**Moved here by `git mv`, ids and bodies unchanged:** from CIW
`pr-ci-wall-clock-root-cause-measured`,
`the-demos-job-is-pr-cis-long-pole-on-its-release-build`,
`a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar`,
`tests-red-under-all-features-never-run-by-ci`; from S-TCOST
`main-reds-editor-core-slow-set-and-the-k-probe-sweep`,
`main-red-tilted-cut-upper-moves-in-the-thread-count-digest-at-default-eps`;
from BUDGET `tess-lint-gate-red-on-main-23-findings`; from S-TINT
`pick-oracle-sweep-anti-vacuity-fails-one-seed-in-1300`; from VACUITY
`pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1`,
`random-integer-rays-search-trips-at-eps-1e-6-on-one-run`,
`cert10-strict-gap-floor-gates-on-a-varying-seed`; from GUARD
`gate-selftests-run-in-no-ci-job`. Filed new:
`nightly-rustdoc-gate-red-on-main`.

**Re-banded.** The latency rows to P0 (the user, in chat: *"the
latency alarm is definitely P0"*): every track's work passes through
the gate. The main reds to P1: a red nightly is a red main, and a new
regression lands unseen among old ones. The two pick-guard rows fold
into one fix (two become passengers, `cost: C`); the eps-1e-6 one
drops P1 → P3, since every recorded red names the guard and none the
oracle. Off this slate: `test-utils-is-production-source-to-every-narrowing-gate`
(TINT) and `mass-properties-bit-comparison-has-thirty-spellings` (HELPER)
P1 → P3, as test-side duplication rather than production architecture.
Closed `two-bisector-node-rows-break-the-fast-set-one-second-bar` (CIW),
already fixed on main.

**Left where they are, on purpose:** COMB's `S19` and `S43` (kernel
refusal and "can only be a bug" architecture, P1, design-heavy roll-ups)
and PERF's `fnv-digest-and-memo-machinery-copies` (production
duplication of the memo machinery, P1). Each is correctly P1, and none
is a green-main or gate-latency row.

## 2026-10-28 — five lanes dispatched

All five at once (`plan.md`, Lanes). The three that build heavily run
as their own cloud sessions, since this box has 4 cores and one build
slot: **slow-set reds** (`green/slow-set-reds*`), **tess-lint**
(`green/tess-lint-red`), **small reds** (`green/tilted-cut-upper`,
`green/rustdoc-red`). The two that are mostly workflow and data run as
subagents here: **latency** (`green/latency-*`) and **gate trust**
(`green/<row>`).

Review tiers, chosen at dispatch:
- latency: single FULL review per lever PR — a workflow change that
  takes a row off the gate can hide a red, which reading alone does not
  rule out;
- slow-set reds and tess-lint: single FULL review — every re-bless has
  to be explained, and the review falsifies the explanation;
- small reds: single STYLE review (an explained re-bless and a doc fix);
- gate trust: orchestrator's read for the seed and pick-guard fixes
  (mechanical, with witness seeds), STYLE review for the two new
  nightly rows.
