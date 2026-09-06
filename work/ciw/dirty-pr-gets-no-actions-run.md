---
id: dirty-pr-gets-no-actions-run
kind: issue
title: A PR that goes mergeable_state dirty against a moved main gets NO Actions run on its next push — an absence, not a red — and the lane cannot tell it from a queue
status: open
opened: 2026-09-05
refs: [1910]
---


(SEAT orchestrator) Process finding from SEAT-7's fix pass (PR 1910),
filed per the durable-home rule; unowned — CI wiring is CIW's ground.

**Measured.** The fix-pass push `6679e034` (04:41Z, 2026-09-05) got only
the `cursor` and `claude` check suites and NO Actions check suite and no
workflow run, while other branches' runs started normally on either
side of it. Cause: the PR had gone `mergeable_state: dirty` against a
moved `main`; with no computable `refs/pull/N/merge`, GitHub dispatches
no `pull_request` run. The symptom is an ABSENCE: nothing is red, the
required `gate ok` check simply never appears, and a lane polling for
a run sees a queue that never drains. Merging `main` into the branch
restored the run. An empty commit pushed to re-trigger did nothing
(and is the shape the drive-to-green rules forbid anyway).

**What a fix needs.** Either the poll scripts/lane briefs learn the
signal (a push with no Actions suite after ~2 min + `mergeable_state:
dirty` ⇒ merge main first), or a lightweight workflow on
`pull_request_target`/`push` posts a visible "no merge ref — merge the
base" status so the absence becomes a red. Cost of leaving it: ~40
minutes per occurrence, silently.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/ciw/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). CI wiring and the lane poll scripts are CIW's ground, as the body says.
