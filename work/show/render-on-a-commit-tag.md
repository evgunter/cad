---
id: render-on-a-commit-tag
kind: unit
title: a [render] tag in a PR head commit's message renders that branch's lanes and commits them back, with CI re-run on the new head
status: review
opened: 2026-10-02
priority: P3
cost: M
pr: 3791
---

## Why

Agents cannot dispatch `render.yml`: the GitHub integration answers
`actions_run_trigger` with `403 Resource not accessible by integration`
(measured by `snowman-cell`'s lane on PR 3787). The implementer
discipline's "a change you expect to move frames renders itself" then
has no door, and a scene PR merges with frames nobody looked at. Ev, in
chat, 2026-10-02, asked whether rendering could be triggered by the
contents of a commit message "kind of like skip ci", and approved this
shape.

## What

1. `ci.yml`, on a `pull_request` run: read the PR HEAD commit's message
   (not the merge commit's); if it contains `[render]`, dispatch
   `render.yml` on the PR's head branch with the workflow's own token
   (`workflow_dispatch` is the event a `GITHUB_TOKEN` may trigger;
   the job needs `actions: write`). Everything downstream is the
   existing dispatch path, which already commits re-baselined lanes to
   the branch it ran on.
2. `render.yml`, on that dispatch, after it commits re-baselined cells
   to a branch with an open PR: dispatch `ci.yml` on that branch, so
   the new head carries a full CI run rather than stranding every
   check on its parent (the reason render.yml gives for PRs never
   committing). Check runs attach to the sha, whatever event made
   them.
3. Without the tag nothing changes: PRs report, main commits (Ev's
   2026-08-17 ruling stands; the tag is a dispatch spelled in a
   commit).
4. Docs: render.yml's header, `demos/README.md`'s "Rendering the
   montages" (which still says every PR run renders every lane — it
   does not; ci.yml calls no render lane and nightly.yml does), and
   the implementer discipline's dispatch sentence where it names
   `render-hosted.sh` — **that last file is `docs/prompts/`, so its
   edit waits for Ev (CLAUDE.md)**: put it in its own `[ev]` PR or
   leave the sentence and note it, do not ride it on this one.

## Territory

`.github/workflows/*` is CIW's. Announce the crossing in the PR body
and on `work/ciw/log.md`.

## Acceptance

Measured on the unit's own PR: a `[render]` commit produces a render
dispatch on the branch, a bot commit of the re-baselined cells (if any
drift; make one cell drift on purpose if needed and revert), and a CI
run whose checks sit on that new head. A commit without the tag
dispatches nothing.

Review tier: single Opus review (workflow logic with a recursion and
a token-permission question; correctness claims plus style).
