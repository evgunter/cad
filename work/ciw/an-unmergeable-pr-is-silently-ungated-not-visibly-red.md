---
id: an-unmergeable-pr-is-silently-ungated-not-visibly-red
kind: issue
title: A PR whose merge ref cannot be computed gets ZERO check runs, which reads as green unless you count jobs
status: open
opened: 2026-09-16
priority: P4
cost: E
---


## Finding

When `main` has moved far enough that GitHub cannot compute
`refs/pull/<n>/merge`, **no `pull_request` event fires and the head
carries zero check runs.** The PR does not go red. It shows nothing —
which reads exactly like green to anyone who does not count jobs.

Hit **twice in one day** by WIRE's `wire-e2` lane on PR #2688, at
`18736473a` and again later in the same unit, each time recovered only
because the lane counted jobs rather than reading a summary. Merging
`main` restored gating both times.

## Why it is worth a row

This tree moves roughly **12,000 commits in three days**, so a PR that
sits through one review round is a normal candidate for this state, not
an edge case. The failure is silent and it is in the *safe-looking*
direction:

- `docs/prompts/implementer-discipline.md` tells every lane that a green
  run means green at all six lane/eps points and all five k-lint
  unifications, and to *"find out what narrowed it"* if fewer than twelve
  `test (…)` jobs appear. **That instruction assumes jobs exist.** It
  does not cover zero.
- A lane polling for "no failures" concludes success. A lane polling for
  "all checks complete" concludes success. Only a lane that asserts a
  **positive count** notices.

## What a taker owes

Something that distinguishes *ungated* from *green*. The cheapest shapes,
in rough order of strength:

1. A required check that runs unconditionally, so zero-jobs is
   structurally impossible rather than merely unlikely.
2. A branch-protection or merge-queue rule that refuses a head with no
   run attached.
3. Failing that, a documented instruction — in
   `implementer-discipline.md` §2 beside the twelve-jobs rule — that a
   run with **zero** jobs is the one case that looks green and is not,
   with the remedy (merge `main`, re-push).

(3) alone is the weakest, because it asks every lane to remember
something the gate could enforce; it is listed because it is
implementable today and the other two may not be.

## Filed from outside the fence

Filed by the WIRE orchestrator under
`docs/prompts/implementer-discipline.md` §6. `.github/workflows/*` is
CIW's by `work.py territory`. Reported rather than fixed: a change to
what gates a PR is CIW's call, not a passing lane's.
