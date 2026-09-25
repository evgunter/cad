---
id: ab-sample-230-claimed-twice-on-main-and-branch-side
kind: issue
title: A/B sample #230 is claimed twice: SYM-11 on main (prior highest +1) and RING-0 branch-side (main's first-parent order)
status: open
opened: 2026-09-21
---


## What

Two units carry A/B sample number #230. SYM-11 (PR 3028, merged at
`11616e2a3e`) recorded "sample #230 — prior highest #229 on main at
merge" in `docs/MODEL-AB-LOG.md` on main (line "SYM-11 RECORDED AT
MERGE"). RING-0 (PR 2993, merged at `4fe98af47b`, earlier in main's
first-parent order) carries "sample #230" on the branch-side block
SCALAR-B5 rows table on `scalar/orchestrator`, and LANE-1 (PR 3010,
merged at `9ed348d6ed`, also before SYM-11) carries #231 there. Both
readings follow a convention the log states: "prior highest on main at
merge + 1" reads only what has landed, and block records that land
when a block closes (SCALAR's, SYM's own "the block record stays on
`sym/b3-block`") are invisible to it; "main's first-parent merge
order" (the v6 instrument's sentence on every SCALAR row) counts every
A/B unit merge whether or not its row has landed.

## Why it matters

The sample number is the join key between the A/B log's rows and any
tally across programs; a duplicate makes two rows answer to one key.
By first-parent order the numbers are RING-0 #230, LANE-1 #231, SYM-11
#232, and SYM-12 or any later unit shifts by one; by the landed-highest
rule SCALAR's branch-side rows are the ones to move. Neither
orchestrator can renumber the other's record.

## Proposed disposition (for the protocol's owner)

State the rule once in `docs/MODEL-AB-LOG.md`'s protocol section:
either (a) the number is main's first-parent order over A/B unit
merges, and an orchestrator claiming a number at merge must count
every A/B unit PR merged on main since the last landed record, not
just landed rows (the check is `git log --first-parent` over the PR
merges plus each program's open block branch); or (b) the number is
claimed on main at merge in the unit PR itself (the SYM shape), and
branch-side block records carry the number the unit's merge commit
recorded. SCALAR's B5 record will land with a note naming this row
and the two readings, leaving its numbers as first-parent order until
the rule is stated.
