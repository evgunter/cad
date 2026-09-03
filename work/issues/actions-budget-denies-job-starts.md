---
id: actions-budget-denies-job-starts
kind: issue
title: hosted CI job starts denied: 'an Actions budget is preventing further use'
status: open
opened: 2026-09-03
needs_ev: true
---


From 2026-09-03 ~11:52 UTC, hosted jobs in this repository stop being
given a runner. The shape is distinctive and is NOT a flake, so it is
worth naming once rather than re-diagnosing per lane:

* the job is `completed` / `failure` **3 seconds** after `started_at`,
* its `steps` array is **empty** (`GET /actions/jobs/<id>` shows `[]`),
* the job log is a `BlobNotFound` XML body, not a log,
* and its check run carries exactly one annotation:
  `The job was not started because an Actions budget is preventing
  further use.` (`GET /check-runs/<id>/annotations`).

Observed on runs 33750997288 (`tcost/b3-rust-cache-misses`, the two
`test (eps = default, N/2)` legs), 33752521466 and 33752464194 (`main`),
33752449318 (`verbs/1031b-winding`), 33752489221 / 33752198053 /
33752050494 (`m10/m10-6-reporting`), 33752276261 (`mngr/kernel-verbs`) —
every ci.yml run created after 11:52 UTC failed this way, on `change
filter`, i.e. before anything of a branch's own runs at all.

**Not a code finding and not any lane's to fix**: it is the account's
Actions spending limit, settable only by whoever holds the repository
settings. Recorded because the failure presents as an ordinary red on
an unrelated job name, and a lane that reads it as its own will push
fixes at it. **A red job with zero steps and a three-second wall is
this, until the annotation says otherwise.**

While it holds, hosted CI cannot be the verification of record for
anything (`docs/prompts/implementer-discipline.md` §2), and a PR whose
gate is half-run should say which jobs did run rather than claim green.

**Orchestrator's verification (S-TCOST, 2026-09-03 12:05 UTC):** every
`ci.yml` run created after 11:52 UTC fails within seconds with no
job started — on `main` (runs 33752464194, 33752521466), on five PR
branches, and on a `workflow_dispatch`; the last run to get a runner
was 33751935948 at 11:51. The job annotation reads "The job was not
started because an Actions budget is preventing further use." This
is the account's Actions spending limit, not a tree, a workflow or
a runner pool. **Ev: raise the limit** (Settings → Billing → Spending
limits → Actions). Until then no PR can gate; lanes are told not to
push empty commits to re-gate. Context for the number: today's
S-TCOST CI-posture units cut ~5 billed minutes per code-tier run, and
TCOST-B3 (PR 1684, ungated by this) measures ~45 billed minutes an
hour going to dependency rebuilds on a cache no branch could read —
the primer it lands should cut most of that.
