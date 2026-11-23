---
id: dependents-slow-sets-skip-a-kernel-pr
kind: issue
title: The nightly attributes each newly red slow or ε row to its merge, so no lane bisects a main red by hand
status: open
opened: 2026-10-16
priority: P1
cost: M
---

Ev ruled on PR 6731 (2026-11-12) that a newly red slow test is bisected
by the nightly and written up as a row. No PR runs a crate's whole slow
set (items 1 to 3 of the policy below this row's history). Nothing
implements it yet: neither `.github/workflows/nightly.yml` nor `scripts/`
mentions an attribution. Its only bisection aid is the `sha` input at
`nightly.yml:16`, which someone has to drive by hand.

**What it would save.** From 2026-10-22 to 11-22, 157 `main-red-*` rows
were opened across `work/`. About 140 of them are slow or non-default-ε
rows that the breaking PR's gate did not run. Many were bisected by hand:
ONECOPY's `main-reds-triage` and `slow-set-reach`, GREEN's `goldens`, and
SCALAR. A bisect takes about 7 to 13 probes at about 6 min each
(`work/ciw/log.md`, 2026-11-10). Lanes also hit reds on their merge base
and had to prove them inherited.

**The cost.** It costs nothing on PRs and adds runner time to the
nightly only. That time is at most five bisects a night, each probe
rebuilding one test binary.

**What it subsumes.** The per-PR canary that catches most of these reds
before merge is `a-stored-data-change-runs-none-of-the-slow-rows-that-read-it`.
This row keeps the policy and the counts already recorded here, and
feeds that row's list.

What would close it: a nightly job that bisects up to five newly red slow
or ε rows over the first-parent merges between two nightly heads and
writes or appends the `main-red` row the policy specifies.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): - The row holds Ev's 11-12 policy: the nightly attributes each newly red
slow test to its merge. That is not implemented. `nightly.yml` has
only a manual `sha` input (line 16), and no script bisects or writes
rows.
- It costs no PR latency and replaces hand bisects (7 to 13 probes at
about 6 min each). About 130 of the month's 157 main reds are of the
kind it would attribute.
- Reform at `reform/dependents-slow-sets-skip-a-kernel-pr.md`, retitled
to the attribution. Its counts feed the P1 canary list.
- 2026-11-22 — **Re-banded P2 → P1 by the CIW orchestrator.** Ev already ruled for it (PR 6731), it costs no PR time, and it removes the hand bisect from most of last month's ~130 main reds of this kind. It is the zero-PR-cost half of `a-stored-data-change-runs-none-of-the-slow-rows-that-read-it`, and the one to build first.
