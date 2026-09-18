---
id: tier-blind-rationale-has-five-prose-spellings
kind: issue
title: the tier-blind siting rule is argued in five places and declared in one
status: open
opened: 2026-09-11
---


*A gate must be sited where it can fire on its own inputs* (Ev,
2026-08-20, on S61) has exactly one machine-readable home:
`scripts/check-ci-mirror-parity.py`'s `TIER_BLIND` tuple, whose entries
are checked to sit in a ci.yml job with no `if:` and above the local
half's docs exit. The ARGUMENT for each entry is re-written in prose
beside every copy of the row:

- `scripts/check-ci-mirror-parity.py:170-215` — the rule, then a
  per-entry comment for most entries.
- `.github/workflows/ci.yml` — a comment block at each such step
  (`local-scripts/` classifies TIER=docs; every hosted job but `mirror`
  deletes it).
- `local-scripts/ci-local.sh:248-282` — the same argument again, plus a
  per-row note for the rows that need one.
- each checker's own module docstring — `check-status-capture.py` and
  `check-render-lane-parity.py` both carry it.
- and now `local-scripts/render-hosted.sh`'s lane-table comment, which
  says where its guard runs and why.

**Nothing compares them, and one of them is already the load-bearing
one**: the `TIER_BLIND` entry. The rest are a reader's aid that goes
stale the way every unheld roster does — and the fifth copy was added by
`render-hosted-knows-four-lanes-and-there-are-six`, so this is an
instance count that grows by one per unit that adds a tier-blind row.

**Not this unit's, and not obviously a defect.** Each copy is at a site
where a reader meets the question, which is the case FOR restating it.
The cheap half if it is worth doing: give `TIER_BLIND` entries a
one-line reason field, have the parity check require one, and let the
prose copies cite the entry instead of re-arguing it. Costed in no unit
so far.
