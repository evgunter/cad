---
id: a-pr-opened-already-conflicting-gets-no-merge-ref-red
kind: issue
title: A PR opened already conflicting with its base, with no push after, gets no merge ref red, and the check queues behind the 20-job cap
status: open
opened: 2026-11-22
priority: P3
cost: E
refs: [dirty-pr-gets-no-actions-run]
---



Residue of `dirty-pr-gets-no-actions-run`, which `.github/workflows/merge-ref.yml`
closed. That workflow runs on `push` (its header says why not
`pull_request_target`), so it has two known gaps:

- **A PR opened when it already conflicts.** The push that created the head
  ran `merge ref` before the PR existed, so the job found no open PR and
  passed; opening the PR is not a push, and no `pull_request` run is made.
  The silence lasts until the next push, which does go red. Rare for a lane
  that branches from a fresh base, common for a stacked PR opened on a base
  that already moved. A fix needs an event that fires on `opened` without a
  merge ref, which today is only `pull_request_target` (which runs only the
  default branch's copy, and which a GitHub changelog of 2026-11-02, read
  but not verified here, says public repositories block by default from
  2026-12-02 unless an Actions event policy allows the workflow: Ev's
  setting), or a poll helper
  (`local-scripts/bt-wait-pr.sh` reads only checks) that reads
  `mergeable_state` when a head has no run.
- **The check queues like every other job.** On the probe (PR 7413, run
  45062549663) the red job was created 19:45:27Z and started 19:49:11Z;
  the two greens after it waited 15 and 21 minutes, all behind the 20-job
  cap (`work/green/pr-runs-queue-behind-the-twenty-job-cap.md`). The
  job takes seconds; under saturation the red arrives when the queue lets
  it, which still beats the silence but is not "within a minute or two".
