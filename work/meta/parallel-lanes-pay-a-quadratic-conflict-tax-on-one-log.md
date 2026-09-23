---
id: parallel-lanes-pay-a-quadratic-conflict-tax-on-one-log
kind: issue
title: N parallel lanes on one program each conflict on log.md at every base merge, and the cost grows with the wave
status: closed
opened: 2026-09-22
priority: P3
cost: D
closed: 2026-09-23
---


Filed by the VGEOM orchestrator from the 2026-09-22 four-lane wave,
where it was measured rather than guessed.

## The finding

`work/<program>/log.md` is append-only narrative and every lane
appends its entry at the end. When a wave runs N lanes on one program,
each lane merges `origin/main` after each sibling lands, and **every
one of those merges conflicts on that one file** — same region, both
sides appending.

Measured on this wave: four lanes, plus the orchestrator's own branch.

| branch | base merges | log conflicts |
|---|---|---|
| `vgeom/camera-band` | 0 (landed first) | 0 |
| `vgeom/render-grid` | 1 | 1 |
| `vgeom/seam-refusals` | 2 | 2 |
| `vgeom/field-product` | 3 | 3 |
| `vgeom/orchestrator` | 3 | 2 (one in `work/tint/log.md`, from another program's wave) |

Eight resolutions of one file, none of them a real disagreement: every
single one was a union, both entries kept, markers deleted. The cost
is roughly quadratic in wave width, and it is paid by the LAST lane
most — the one already carrying the most rebasing.

## Why it is not just annoying

A union conflict resolved by hand eight times is eight chances to drop
someone's paragraph, and **the tree has an instance of exactly that
going wrong**: `work/props/log.md` carried a complete committed
conflict block on `main` — both sides intact, three marker lines
shipped — found by accident by a CHROME lane's tree-wide grep and
repaired at `97217090`. `work.py lint` does not read `log.md`, so
`main` was green over it.

The standing ruling is that a marker-detecting gate is **not** worth
building (`work/ciw/committed-conflict-markers-reach-main`, Ev,
2026-09-04: *"close it — the failure is rare and not worth the special
effort"*). **Nothing here reopens that.** What this row adds is the
other half of the cost model that ruling was made against: the failure
is rare *per merge*, and a wide wave multiplies the merges. Four
instances in about three weeks was the datum then; this one wave
produced eight resolutions on its own.

## What it is NOT

Not an argument for per-lane log fragments, or for a generated log, or
for any change to `work/README.md`'s append-only contract. Those are
tracker-shape decisions that bind future work, and they are Ev's
before they are anyone's. This row records the measurement and the
hazard so a decision about wave width, or about the log's shape, can
be made against a number instead of an impression.

The cheap mitigation that needs no ruling and was used here: the
orchestrator resolves every lane's log conflict itself at merge time,
which is one reader who has seen all N entries rather than N lanes
each seeing two.

## Closed (2026-09-23): the log merges by union

`.gitattributes` gives `work/*/log.md` git's `union` driver, so the
base merges this row measured go through without a conflict: both
entries kept, in order, which is the resolution all eight were given
by hand. The alert the conflict used to give by accident, for a note
another program leaves on a log, is now an explicit read:
`work.py incoming` at every check-in (`work/README.md`, "The log
merges by union"). Rejected: keeping every entry on the orchestrator's
branch and merging that branch into unit PRs. It avoids the conflict
through shared commits, but it carries the branch's other contents
into self-merged unit PRs, and lets a unit's A/B entry reach main
before that unit's reviews are done.

GitHub's own merge (the PR's mergeability check and the merge button)
is a separate question. A probe on the PR that made this change showed
a conflict, but `.gitattributes` was then on the head only, and
`git --attr-source=<main> merge-tree` reproduces that conflict locally,
so it does not settle whether GitHub reads the attribute. Where it
does not, a PR with a diverged log shows as conflicting and is brought
up to date by a local base merge, which the attribute does cover.

**GitHub's merge, settled (2026-09-23).** With `.gitattributes` on
both sides, PR 3109 appended one line to the end of `work/tess/log.md`
from `1f04912c` while main had appended 29 lines there since (#3107).
GitHub reported the PR `dirty`, and a local `git merge-tree` of the
same pair was clean. GitHub's merge ignores `merge=union`, so the
attribute pays off only in local base merges; `work/README.md` says so.
