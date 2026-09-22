# Sweep 11 — 2026-09-11: code-quality leaves the tracker

**code-quality** — *"where a structural finding waits until a program
claims it"* — opened 2026-08-18 as the tracker home of the 2026-08
structural-findings register (`docs/SMELL-SCAN-2026-08.md`, sweep 4) and
its Tracks K–X schedule, and closed 2026-09-11. It is the first program to
close empty by design: on 2026-09-11 all 110 of its remaining live rows
were claimed at once by eleven programs opened for them
(`docs/WORK-TRACKS-2026-09.md` addendum 3, PR #2370).

**No exit walk was written, on Ev's direction** (in chat, 2026-09-11: *"can
you delete all the closed items in issues, and the code-quality dir
entirely"*) — the contract's exception, recorded here because the absence
would otherwise read as an omission.

Deleted: `work/code-quality/` whole (46 tracked files — `program.md`,
`log.md`, 32 closed rows, and `logs/`, the ten closed tracks' execution
records `SMELL-{C,E,F,G,H,I,KPW,T,UV}` plus
`migration-census-2026-09-03.md`, about 11,000 lines), and seven closed
items in `work/issues/`. `work/issues/README.md` stays and the directory
keeps its purpose. Eighteen `refs:` on sixteen live rows were rewritten
first; each names what changed in its own `## Refs at code-quality's
sweep` section, including the five references dropped rather than re-aimed
because the row closed with no PR to cite.

Sweep SHA `8851abb6daff4822f5a55c98e940c4c061223953`.

    git show 8851abb6daff4822f5a55c98e940c4c061223953:work/code-quality/<FILE>
    git show 8851abb6daff4822f5a55c98e940c4c061223953:work/issues/<FILE>

## Amendment (2026-09-11, same day): the two relocated documents went too

The sweep first moved `plan.md` and `process-observations.md` into `docs/`;
**Ev rejected that** (in chat: *"we don't want to mint any new rows because
we're using the in repo issue tracker now, not the one big doc that
descends from"*), and both were deleted, recoverable at the SHA above.
What was live in them went to where it is used: the reviewer rule *the fix
mints a fresh instance of the defect it closes* to
`docs/prompts/reviewer-style-lane.md` §1; the ordering, partition, seam and
fence rules to the eleven citing `plan.md` charters, inlined; the rest is a
closed program's retrospective.

**The block ledger is retired, not relocated.** Ids in this tracker come
from an item's name; the per-track `D<N>`/`S<N>` blocks were the 2026-08
register's numbering. **No new row is minted from a block.** Rows carrying
such an id keep it — ids are stable for life — and nothing allocates
another. `work/README.md` says so.

Done-state of record: this note and the amendment above;
`docs/WORK-TRACKS-2026-09.md` addendum 3 for where its 110 live rows went;
the merged PRs of its closed tracks, named in the logs recoverable at the
SHA above.
