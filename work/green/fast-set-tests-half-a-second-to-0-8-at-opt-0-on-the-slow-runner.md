---
id: fast-set-tests-half-a-second-to-0-8-at-opt-0-on-the-slow-runner
kind: issue
title: 50 fast-set tests run 0.5-0.8 s on the slow runner with integration binaries at opt 0 (24 of them under 0.5 s at opt 1), one runner-ratio from the 1 s bar
status: open
opened: 2026-11-02
priority: P4
cost: E
---


## Found (GREEN latency lane, 2026-11-02, PR 6368 run 41453582409)

With the PR test job's integration binaries at opt 0
(`scripts/rustc-test-crates-opt0.sh`), the kernel shard on an EPYC 7763
(the slow runner type) ran 59 fast-set tests at or above 0.5 s. 36 tests
ran that slow at opt 1 on the same runner.
- The 9 at or above 0.8 s moved to the slow set in the PR that took the
  wrapper. 8 of them were already 0.80–0.90 s at opt 1.
- **50 remain in 0.5–0.8 s.** For 24 of them, opt 0 is what put them
  there: they ran under 0.5 s at opt 1. They are mostly `geom-core::all`'s
  `m5_pr7b_tensor_compose` / `review_m5_pr7b_tensor` rows (0.13–0.23 →
  0.51–0.73 s), plus `sweep::all`'s `near_circular_sections` and band rows.
- **The flap risk:** the closed
  `work/ciw/fast-set-tests-over-one-second-are-not-enforced.md` saw
  hosted/local ratios up to 2.8, and saw tests at 1.04–1.37 s flap. A
  slower runner than this one, or a noisy neighbour, carries a 0.7 s row
  over the 1 s backstop and reds an unrelated PR.

## Regenerate

PR 6368's temporary step does it: commit `2f724bb84e3` on
`green/kernel-binary-ab2`, and `.github/kba-list.py` there. On one
runner, the step builds the test job both ways from one restored cache,
copies `target/nextest/ci/junit.xml` after each default-eps run, and
lists every test at or above 0.5 s with its opt-1 time beside it.

## Owed

Decide between two remedies. One is to move the 24 that opt 0 put over
0.5 s to the slow set, by the discipline's own 0.5 s rule. The other is
to leave them and watch `check-fast-set-times` reds for them.
