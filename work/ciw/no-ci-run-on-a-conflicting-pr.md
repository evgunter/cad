---
id: no-ci-run-on-a-conflicting-pr
kind: issue
title: A push to a PR that conflicts with main gets no CI run at all — GitHub creates no refs/pull/N/merge, so a silent 'no run' after a push means a conflict, not a stalled queue
status: open
opened: 2026-09-05
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
