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

## Merged with `no-ci-run-on-a-conflicting-pr` (2026-09-06, CIW orchestrator)

The same finding reached this slate twice on 2026-09-05, from two
orchestrators who could not see each other: SEAT's (this file, from SEAT-7's
fix pass, PR 1910) and PROPS' (`no-ci-run-on-a-conflicting-pr`, from the
riders lane). Same mechanism, same symptom, same two candidate fixes. This
file survives because it carries the measurement; the other is closed
pointing here, and its two instances are folded in:

- `eba54d6c6` on PR #1977 and `ae5e5c114` on PR #1980 — two fix-pass heads in
  a row with no workflow run. `main` had moved under them in
  `work/props/log.md` and `docs/DOC-LEDGER.md`, tail-append conflicts only,
  which is enough to make a PR conflicting and therefore runless. Merging
  `origin/main` produced a run within seconds, both times.

So the population is **three measured occurrences in one day**, across three
programs, none of them a code conflict — all three were tail-append conflicts
in a log or a ledger, which is the shape this repository generates constantly
by construction.

PROPS' ask (1) is kept and is the cheaper half of the fix above: one line in
`docs/prompts/implementer-discipline.md`'s verification section saying a push
with no run is a conflict to merge out, not a queue to wait on.
