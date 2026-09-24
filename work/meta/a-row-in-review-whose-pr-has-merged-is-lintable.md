---
id: a-row-in-review-whose-pr-has-merged-is-lintable
kind: issue
title: work.py lint cannot see a row whose status is review while its pr has merged
status: open
opened: 2026-09-21
---


Reported by VSEAM, which owns neither `scripts/work.py` nor this slate.
Filed rather than written for that reason.

## The gap

`status: review` means *a PR is open over this row*. Nothing checks that
the PR is still open. Five VIEW rows carried `review` for six days after
their PRs merged (#2622, #2662, #2666, #2670, #2672 — all merged 14–15
September, closed by PR #2976 on the 21st).

`work.py lint` already reads both fields. It does not ask whether they
agree, because answering needs the forge.

## Why it is worth a check rather than a habit

The staleness did not sit still. The four-track cut (#2806) read those
five `review` rows as *lanes in flight* and declined to re-home them on
that ground — so a stale field became a stale sentence in
`work/vseam/plan.md` (*"their lanes are in flight"*, over a list of
five it called four), which became a program's Order item waiting on a
trigger that had fired. **One un-updated field, three wrong artefacts,
six days.** The row files are lintable; the prose that quotes them is
not, so the field is the only place a check can bite.

## Shape

A row with `status: review` and a `pr:` whose PR is merged or closed is
a lint problem — a closed PR over an open-in-review row is a state the
tracker's own vocabulary says cannot persist.

Two things a taker has to decide, neither obvious:

- **Where the forge lookup lives.** `work.py lint` runs in CI and on
  every lane's box, and must not need network or a token to pass. A
  hosted-only check, an opt-in flag, or a locally-answerable proxy
  (is the branch merged into `main`?) are all plausible and they are not
  the same check. **The local proxy is the interesting one**: `pr:` has
  a sibling `branch:` field, and whether that branch is an ancestor of
  `origin/main` is answerable with no network at all — but it is a proxy
  for the property, and this repo's register has a standing rule about a
  classifier standing in for the thing it classifies.
- **What the check says when a `pr:` is merged but the row is genuinely
  still in review** — a follow-up PR, a revert. The rule must not make
  that unrepresentable.

## Not a substitute for the orchestrator's own pass

The orchestrator merged all five and closed none; a lint would have
caught it, and so would re-deriving the board from the files rather than
from memory. This row is the mechanical half only.

**Where**: `scripts/work.py`'s `lint`, and the `status`/`pr`/`branch`
frontmatter keys `work/README.md` defines.
