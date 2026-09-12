---
id: no-ci-run-on-a-conflicting-pr
kind: issue
title: A push to a PR that conflicts with main gets no CI run at all — GitHub creates no refs/pull/N/merge, so a silent 'no run' after a push means a conflict, not a stalled queue
status: closed
opened: 2026-09-05
closed: 2026-09-06
refs: [dirty-pr-gets-no-actions-run]
---


(PROPS orchestrator, from the riders lane, 2026-09-05.) Two fix-pass
heads in a row (`eba54d6c6` on #1977, `ae5e5c114` on #1980) got NO
workflow run: main had moved under them in `work/props/log.md` and
`docs/DOC-LEDGER.md` (tail-append conflicts only), the PRs were in
conflict, and GitHub creates no `refs/pull/N/merge` — hence no
`pull_request` run — for a conflicting PR. Merging `origin/main`
produced a run within seconds. Two asks: (1) one line in
`docs/prompts/implementer-discipline.md`'s verification section — a
push with no run is a conflict to merge out, not a queue to wait on;
(2) whether the change filter or a `check_suite` hook can post a visible
"no run: conflicting" status so a lane polling `get_check_runs` sees
something rather than nothing.

## Closed 2026-09-06 — duplicate of `dirty-pr-gets-no-actions-run`

The same finding, filed the same day by two orchestrators who could not see
each other's slate: this one from PROPS' riders lane, the other from SEAT-7's
fix pass (PR 1910). The other file carries the measurement — the check suites
that did and did not appear, the `mergeable_state: dirty` reading, the
restored run — so it survives, and the two instances above (`eba54d6c6` on
#1977, `ae5e5c114` on #1980) and ask (1) are folded into it.

Nothing is lost by this closure and nothing is scheduled by it: the work is
`dirty-pr-gets-no-actions-run`, open on this slate.
