---
id: perf-plan-is-cited-by-twenty-nine-files-and-absent-from-tree-and-ledger
kind: issue
title: docs/PERF-PLAN.md is cited by path from 29 tracked files and exists in neither the tree nor the ledger's deleted-doc records
status: closed
opened: 2026-09-04
closed: 2026-09-11
refs: [perf-plan-citations-name-a-path-that-moved]
---


## What

Found by DOCM-5's implementer lane (PR 1871) following a citation:
`docs/PERF-PLAN.md` — the performance discipline document ("brute force
until a measurement says otherwise", the deferred-quadratic record) —
is cited by path from 29 tracked files (`grep -rln PERF-PLAN crates
docs demos scripts memories work`), among them `crates/editor-core/src/checks.rs`'s
cost note (rewritten in that PR), `docs/DOCM-5-SPEC.md` (the
orchestrator's own spec, written from those citations), and
`docs/DOC-LEDGER.md:186`, which names it as the SUCCESSOR of an
archived milestone record. The file is not in the tree, and the ledger
records no deletion of it — the one document whose job is to say
where deleted docs went does not know this one is gone.

The live statement of the same rules is `memories/perf-measurement-lane.md`
(hosted producer, append-only history, a counter before a stopwatch),
which DOCM-5's measurement followed instead.

## What it wants

Either (a) the ledger gains the deletion record (the SHA at which the
file last existed, and what superseded it — presumably the memory
above and `docs/perf-data/`'s history), and the 29 citations are
repointed or rewritten to the live home; or (b) the document is
restored from its last SHA if it still says something nothing else
does. Either way, a citation to a path is a claim the path exists, and
today 29 of them are false. Not a program's slate: the ledger is the
repo's.

## Re-homed (2026-09-05)

Arrived in `work/issues/` after the 2026-09-04 re-home sweep had run
and was routed at that sweep's merge with main. Moved to `work/meta/`,
which opened the same day and holds `docs/DOC-LEDGER.md` in its
`paths`: this item's own closing sentence — *"Not a program's slate:
the ledger is the repo's"* — names a home that did not exist when it
was filed and does now.

Both dispositions it offers are this program's ground. (a) is a ledger
deletion record plus a citation repoint, which is the same class as
`stale-track-t-citations-in-fillet-and-cert` on this slate — a citation
across a fence going stale with nothing that reads it, at 29 instances.
(b) — restore the document — is a call this program can put but not
make alone, since what `docs/PERF-PLAN.md` said that nothing else says
is PERF's to judge.

## Closed (2026-09-11) — its premise did not survive the measurement

**`docs/PERF-PLAN.md` was never deleted. It was RENAMED to
`work/perf/plan.md` on 2026-09-03, with no content change, and it is in
the tree right now.**

```
$ git show --stat 4916f90c -- docs/PERF-PLAN.md work/perf/plan.md
 docs/PERF-PLAN.md => work/perf/plan.md | 0
 1 file changed, 0 insertions(+), 0 deletions(-)
```

`4916f90cfc5cd45c0092b9464fd1fed604f93140` is the tracker migration
(*"work: migrate m10, pcurve, verbs, lib, gui, gauth, seat, blend,
perf"*, PR #1619). The move was `scripts/work.py lint`'s own
`docs/*-PLAN.md` rule being obeyed, and `docs/DOC-LEDGER.md`'s sweep 4
already recorded it — under **"Moved, not deleted"**, as a class rule
covering every plan and log at once.

So all three claims in this item's title and §What are false:

| claim | measured 2026-09-11 |
| --- | --- |
| "cited by path from 29 tracked files" | **5 files** cite the path; 27 more name the DOCUMENT, which still carries that name as its own title |
| "absent from tree" | present, at `work/perf/plan.md`, byte-identical at the move |
| "the ledger records no deletion of it" | the ledger records the MOVE, generically, in sweep 4 |

Neither disposition it offered applies. (b) — *restore the document if
it still says something nothing else does* — is moot: nothing was lost,
and the call this item said was "PERF's to judge" does not exist. (a) —
*the ledger gains the deletion record* — would have written a false
entry.

## What landed instead

- **`docs/DOC-LEDGER.md` sweep 4 names the moved files** rather than
  only stating the class, with `PERF-PLAN.md → work/perf/plan.md` in
  its own row and the SHA the rename is followable from. A class rule
  is not a by-name lookup, and a by-name lookup is the whole service
  this ledger performs.
- **Its recovery section says to run `git log --all --full-history`
  before concluding a document is gone**, with this item as the worked
  example. `--diff-filter=D` — the recipe the ledger did offer — is
  EMPTY for a renamed file, which is precisely why the wrong conclusion
  was the easy one to reach here. The lane that filed this did the
  right thing with the tools the ledger named.

The five live path citations and the naming question behind the other
27 are `perf-plan-citations-name-a-path-that-moved`, filed rather than
described: none of the five is META's file to edit and each is routed
there by owner.
