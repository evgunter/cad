---
id: two-rows-cite-a-nightly-python-job-that-is-deleted
kind: issue
title: Two items name nightly.yml's 'python suite (ungated re-take)' as their coverage lane; that job is deleted and the PR gate now runs it
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the S-TCOST orchestrator at the merge of PR 2434.
Routed rather than fixed across the fence: both items are LIB's and
one-file-one-item makes two programs editing one item a merge conflict by
design.

**The premises moved in the SAFE direction** — both items wanted the
`python` feature lane covered more widely, and it now is. Neither
finding is invalidated; both cite a lane that no longer exists as the
thing covering them.

## What changed

S-TCOST unit C3 made `ci.yml`'s `python suite (wheel + guide +
north-star)` **seed-keyed** on 2026-09-03 and added
`nightly.yml`'s `python suite (ungated re-take)` to cover the seeds the
key missed — its header names them: `viewer` (above the wheel) and
`test-utils` (a dev-dependency edge `maturin build` does not follow).
That trade was argued in **billed Actions minutes**, a currency that
stopped existing when the repository went public on 2026-09-03.

PR 2434 re-costed it on wall clock (the job is ~106 s median against a
run whose pole is the `build → test` chain), **restored the suite to
every code-tier run with no seed key, and deleted the nightly re-take**
as strictly duplicated.

## The two sites

- `work/lib/the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row.md`
- `work/lib/pncad-py-python-feature-clippy-lane-is-red.md`

Both name `nightly.yml`'s `python suite (ungated re-take)` as the lane
that covers the `python` feature clippy row, one of them alongside
`local-scripts/ci-local.sh`.

## What each should now say

The `python` feature clippy row runs in `ci.yml`'s
`python suite (wheel + guide + north-star)` on **every code-tier run**,
gated only on `run_build`. So:

- The lane both items point at is gone.
- The coverage both items wanted is better than it was: every PR rather
  than one nightly firing, and with attribution to the PR that broke it
  rather than to the night's merges.
- Whether either item's underlying finding is thereby **closed** is LIB's
  call and this row does not make it. `pncad-py-python-feature-clippy-lane-is-red`
  in particular is about a lane being RED, which restoring it to the gate
  does not fix — it makes it red on every PR instead of nightly, which
  may be urgent rather than resolved.

That last point is why this is filed rather than left: a reader
reconciling those items against the tree will find the named job missing
and may read that as the finding having been handled.

## Related

`work/tcost/nightly-demotions-c1-c3-were-bought-with-billed-minutes`
(closed at PR 2434) carries the re-cost, the hosted readings and the
deletion rationale.
