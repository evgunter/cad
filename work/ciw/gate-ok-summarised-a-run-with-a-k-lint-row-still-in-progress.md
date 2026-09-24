---
id: gate-ok-summarised-a-run-with-a-k-lint-row-still-in-progress
kind: issue
title: gate ok ran while k-lint (gate, release-default) was still in_progress although needs: names k-lint — the roll-up reported red on a run whose every job concluded green
status: open
opened: 2026-09-15
priority: P4
cost: E
---


## Finding

Run 34949109141 (PR 2466, head `6a25b1d7f`, 2026-09-15 08:49–09:28 UTC):
`gate ok` concluded `failure` with its own diagnostic — "these jobs had
not finished when this gate ran: k-lint (gate, release-default)
(in_progress)" — while `.github/workflows/ci.yml`'s `gate-ok` job DOES
name `k-lint` in `needs:` (the whole matrix), and that k-lint row went
on to conclude `success`. So the roll-up's premise ("this gate runs last
because `needs:` names every other job") held in the file and failed in
the run: `needs:` was satisfied before every matrix row of `k-lint` had
concluded. The final jobs list for the run is 35 success, 3 skipped, 2
neutral (the render drift reports), 1 failure (the gate itself).

The likely mechanism, not verified from this side: a matrix row that
lost its runner and was re-queued by the platform within the same run
attempt, so the `needs` edge resolved on the first incarnation's
conclusion while the second was still running. Whatever the mechanism,
the roll-up's contract — a summary of a run it saw the end of — has a
hole the diagnostic already knows how to name; what it cannot do is
wait. Either the roll-up re-polls the jobs API until no job is
`in_progress`/`queued` (bounded), or the workflow documents that a
re-queued matrix row can outlive the gate and how a merger reads that.

The PR was merged on the job-level evidence (every substantive job on
the sha green and concluded), with a standing-down comment naming this
run. The `rerun-failed-jobs` API is not reachable from the orchestrator
integration (403), so the gate could not be re-taken from there.

## A second instance, and a sharper reading (VREV fix pass, 2026-09-15)

Run **34949708108** (PR 2627, head `54648886f`, 09:25 UTC) failed the
same way over 34 green jobs, with `k-lint (gate, release-default)`
again the row read as `in_progress`. The lane's reading: the `needs:`
edge resolves when the runner reports the job done, and `gate ok`'s
single un-retried read of the jobs API can land before that API
reports the job `completed` — so the diagnostic is right that a job
was still "going" from the API's view and wrong that `needs:` did not
wait. Either way the roll-up owes a bounded re-poll until no job reads
`in_progress`/`queued`. The lane's own row for this
(`gate-ok-reads-the-jobs-api-before-it-has-settled`) is folded here —
one file per item.
