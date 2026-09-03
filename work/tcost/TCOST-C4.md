---
id: TCOST-C4
kind: unit
title: re-run the sccache trial and write its verdict down
status: review
pr: 1648
branch: tcost/c4-sccache-reread
opened: 2026-09-03
refs: [852, 853, 1648]
---

CI-posture unit, cut with TCOST-C1..C3 (Ev's ask, 2026-09-03): the cross-run
lever the audit left open. The rig from #852 had been inert since it landed —
`vars.SCCACHE` is `"0"`, so both build jobs reported `install sccache` and
`sccache stats` as `skipped` — and the reading it was landed for was never
taken.

Taken on this branch, with the condition dropped for a trial window (a branch
cannot change a repo variable). **Negative, and structurally so**: sccache
0.16.0 refuses `--crate-type bin`, which is every test binary in the nextest
archive and 82 % of the build job's compile time. On the one run where both
caches restored it took 18 units (the workspace libs) and refused 47. The
object cache (~205 MB per lane) also failed to survive a 38- and a 60-minute
gap between runs.

The rig stays, off by default (`vars.SCCACHE == '1'`). Verdict and runs:
`docs/CI-MINUTES-2026-08.md` F4, raw readings under
`docs/perf-data/sccache-trial/`, and the local half of the question in
`docs/LOCAL-BUILD-PERF.md`.

Closes `sccache-trial-verdict-to-read` when 1648 merges. The larger finding
the trial turned up — `Swatinem/rust-cache` restores nothing on most build
jobs — is F4's closing paragraph and is not this unit's to fix.
