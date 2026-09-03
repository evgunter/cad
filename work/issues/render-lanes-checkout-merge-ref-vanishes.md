---
id: render-lanes-checkout-merge-ref-vanishes
kind: issue
title: render lanes: ~100 hosted reds are 'couldn't find remote ref refs/pull/N/merge' at checkout
status: open
opened: 2026-09-03
github: 1607
---

## From GitHub issue 1607 (S-TCOST red-history census)

Across every completed `ci.yml` run since 2026-07-22, 103 failed
render-lane jobs in 89 runs (2026-08-12 → 2026-09-02) failed at
`actions/checkout` with `fatal: couldn't find remote ref refs/pull/N/merge`
— more failed jobs than clippy or the discipline gates produced over the
same window, none of them about the tree. By job: `scene inputs + uv
sheet + wild montage` 72, `wild-corpus montage` 8, `demo tour (scene
inputs)` 8, `freecad montages` 5, `freecad montage` 4, `kernel montage`
3, `uv trim-loop sheet` 3.

`render.yml` is a reusable workflow called from `ci.yml`'s `renders`
job; every lane checks out `ref: ${{ inputs.ref || github.ref }}`, and
inside a `workflow_call` from a `pull_request` run that is the
transient `refs/pull/N/merge`, re-created when the PR's head or base
moves and deleted when the PR closes.

NOT established: which mechanism — the ref re-pointed by a newer push
before the lane's checkout ran (the job dies at checkout before the
concurrency cancel reaches it), or the PR merged while a lane was still
queued. `memories/agent-lane-operations.md` already warns a check still
running at merge dies at checkout; this is that class, measured.

A fix would check out an object that outlives the ref (`github.sha` is
the merge commit for a `pull_request` run) or end the lane as skipped
with a stated reason when the ref is gone, and say in its log which.
`render.yml` carries its own `push_to`/`ref` contract; its author should
decide the spelling. Evidence: the census's `nontest_failures.json`
(lane-private); any of the 103 job logs reproduces the lines.
