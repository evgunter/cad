---
id: render-lanes-red-at-missing-merge-ref
kind: issue
title: render lanes: ~100 hosted reds are couldn't find remote ref refs/pull/N/merge at checkout
status: closed
opened: 2026-09-02
github: 1607
pr: 1724
branch: ciw/render-lane-merge-ref
closed: 2026-09-04
---

## From GitHub issue 1607

Opened 2026-09-02; 0 comments.

(S-TCOST orchestrator) Filed from the S-TCOST red-history census (`docs/S-TCOST-LOG.md`, seam entry of 2026-09-02); out of that program's scope, recorded so it has a durable home.

## The fact

Across every completed `ci.yml` run since 2026-07-22, **103 failed render-lane jobs in 89 runs (2026-08-12 → 2026-09-02) failed at `actions/checkout` with**

```
##[error]fatal: couldn't find remote ref refs/pull/N/merge
##[error]The process '/usr/bin/git' failed with exit code 128
```

By job: `render lanes / scene inputs + uv sheet + wild montage` 72, `wild-corpus montage` 8, `demo tour (scene inputs)` 8, `freecad montages (kernel + freecad)` 5, `freecad montage` 4, `kernel montage` 3, `uv trim-loop sheet` 3. That is more failed jobs than clippy (99 readable) or the discipline gates (46) produced over the same window, and none of them says anything about the tree.

## Where the ref comes from

`render.yml` is a reusable workflow called from `ci.yml`'s `renders` job, and every lane checks out

```yaml
- uses: actions/checkout@v4
  with:
    ref: ${{ inputs.ref || github.ref }}
```

(`.github/workflows/render.yml`, the two checkout steps). Inside a `workflow_call` from a `pull_request` run, `github.ref` is the transient `refs/pull/N/merge`, which GitHub re-creates whenever the PR's head or base moves and deletes when the PR closes.

## What is NOT established

The census did not determine which of the two obvious mechanisms produces the failure, and this issue does not assert one:

- the ref being re-pointed by a newer push before the lane's checkout ran (the run then shows as `failure` rather than `cancelled` when the job dies at checkout before the concurrency cancel reaches it), or
- the PR being merged while a lane was still queued (the merge rule checks conclusions, and a queued render job is easy to miss).

`memories/agent-lane-operations.md` already warns that a check still running at merge "dies at checkout and can never be re-run"; this is that class, measured.

## What a fix would look like (for whoever owns render.yml)

Check out an object that outlives the ref (`github.sha` is the merge commit for a `pull_request` run and can be fetched by SHA), or end the lane as skipped with a stated reason when the ref is gone rather than red. Either way the lane should say in its log which it did. Not proposed here as a patch: `render.yml` carries its own `push_to`/`ref` contract and its author should decide the spelling.

Evidence: the census's `nontest_failures.json` (job ids, run ids, dates); any of the 103 job logs reproduces the four lines above.

## Home

`work/issues/` — `.github/workflows/` is S-QA's territory and S-QA is closed; no open program owns the render workflows, and S-TCOST filed it as out of its own scope.

## Closed (2026-09-04): the fix landed and the class is at zero

PR 1724 merged as **`a5d9f41a`** ("Merge pull request #1724 from
evgunter/ciw/render-lane-merge-ref", 2026-09-04T01:31:21Z), an ancestor
of `main`. `render.yml` grew a `checkout target` job that resolves the
object once and hands the lanes a SHA, and the lanes check that out
instead of `refs/pull/N/merge`. The item was left at `status: review`
after the merge; it is closed here, with the rate re-measured rather
than assumed.

**Since `a5d9f41a`, the class is at zero.** Over every completed
`ci.yml` run created between the merge and 2026-09-04T16:46Z —
**259 runs carrying render-lane jobs, 777 render-lane jobs** — there are
**8 failed render-lane jobs and none of them failed at
`actions/checkout`**. All eight failed at a lane's own content step
(`demo tour (…)` once, `the real gallery opens in the viewer (GUI-4
acceptance)` seven times), and the seven are one incident inside 25
minutes on 2026-09-04 that also reddened `main`'s own push run
(33841011297) — a real break in the tree, not a checkout that lost its
ref.

For comparison, the same reading over the window immediately BEFORE the
merge (2026-09-03T09:26Z → the merge) gives **5 failed render-lane jobs
in 116 render-bearing runs, all five at `actions/checkout`** — the tail
of the 103-in-89 population this item measured.

So the 103-in-89 figure is the PRE-FIX rate and must not be quoted
forward as the current one. `work/ciw/merge-queue-trial` decides the
`renders`-in-`gate-ok` question against the post-fix rate.
