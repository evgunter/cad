---
id: territory-base-main-is-stale-in-every-agent-checkout
kind: issue
title: work.py territory --base main answers against a ref that is ~1000 commits stale in a fresh worktree, and the false number reads as a real crossing
status: open
opened: 2026-09-22
priority: P3
cost: E
---


## Finding

`work/README.md` and `docs/prompts/implementer-discipline.md` both give
the spelling `python3 scripts/work.py territory --base main`, and every
CHROME dispatch has been repeating it. In a remote-session checkout and
in a fresh agent worktree, **the local `main` ref is not `origin/main`
and can be a thousand commits behind it**, so that command diffs the
branch against a tree from weeks ago and reports most of the repository
as touched.

Measured today, 2026-09-22, in two independent checkouts:

- The CHROME orchestrator's own remote-session checkout: local `main` at
  `4638091074b`, `origin/main` at `8ab7ada49` — **981 commits behind**.
- A lane's harness-created worktree, same session: the same stale local
  `main`, reported as 983 behind at that moment.

The lane on `chrome/rowstatus-exhaustive` ran the documented spelling and
got **575 paths in another program's territory** on a two-file diff. The
same command with `--base origin/main` answers correctly: two paths, both
double claims, no crossings.

## Why this is worth a row rather than a habit

**The wrong answer is not shaped like an error.** `territory` warns
rather than blocking, and its output is a list of real paths owned by
real programs — it reads exactly like a genuine, large crossing. A lane
that has just been told by its dispatch to announce seams has every
reason to believe it and no reason to re-check. The failure mode is a
lane either escalating a crossing that does not exist, or — worse, and
the reason this is filed — learning that `territory`'s output is noise
and skipping it thereafter. That is the same dynamic
`work/README.md` records for the retired double-claim lint warning: a
warning nobody can act on teaches people to skip warnings.

It is also invisible to `--selftest`, which runs where `main` is fresh.

## What a taker decides

Two readings, and the choice is `meta`'s:

1. **A documentation fix.** `--base origin/main` is the correct spelling
   for any checkout an agent works in, and the three places that give
   `--base main` are wrong. Cheapest, but it fixes the instances and not
   the class — the next doc to quote the command gets it wrong again.
2. **A guard in `work.py`.** When the base ref resolves and its
   remote-tracking counterpart is reachable and ahead, say so before
   printing the report — *"base `main` is 981 commits behind
   `origin/main`; did you mean `--base origin/main`?"* — or default
   `--base` to the remote-tracking ref when one exists. This makes the
   documented spelling safe rather than making every reader remember.

The second is the one I would take, because the first leaves the trap
armed for the next reader. But `scripts/work.py` is `meta`'s and so is
the call.

## Provenance

Found by the CHROME implementer lane on `chrome/rowstatus-exhaustive`
(PR 3055), which noticed its own 575-path answer was implausible for a
two-file diff and checked the base ref rather than reporting the
crossing. Re-measured independently in the orchestrator's checkout
before filing. Filed under `docs/prompts/implementer-discipline.md` §6:
`scripts/work.py` is `meta`'s ground, not CHROME's.

Signed (CHROME orchestrator).

## A second, separate defect in the same command — `meta` may want to split this out

Found 2026-09-22 while adjudicating PR 3058, and recorded here rather
than in its own file only because it is one line of the same function;
split it if that reading is wrong.

**`territory` calls every shared path "a double claim" however many
programs claim it.** `scripts/work.py` builds the line as

```
f"{mine} claims it too — a double claim, not a crossing"
```

with no reference to how many co-claimants it just listed. So a path
four programs claim prints *"also claimed by author, vgeom, view;
chrome claims it too — a double claim, not a crossing"* — which names
three and then calls it double in the same sentence.

**Why it is worth the line.** It is not merely inelegant: it was
believed. PR 3058's body summarised its seven paths as *"all double
claims"*, which is a faithful reading of the output and is wrong about
five of them; the CHROME orchestrator then corrected the PR and
attributed the miscount to its author, which was also wrong. Two
readers in one day took the tool's word over the list printed
immediately before it. `work/README.md` ratified on 2026-09-20 that
two open programs may claim one path and that shared ground is
expected — so the *count* is the interesting part of that line now,
and it is the part the sentence contradicts.

The fix is presumably "a shared claim" or the count itself. Note that
`--selftest` asserts the current phrase, so it moves in the same
commit.
