---
id: nightly-demotions-have-never-run
kind: issue
title: Three jobs demoted to nightly.yml on 2026-09-03 have never executed; a fourth was found broken and never run the same day
status: open
opened: 2026-09-04
refs: [1650, 1654, 1655]
---



Found while auditing CIW's slate on 2026-09-04. Nothing is red; the
defect is that three gates currently run **nowhere**, and no instrument
says so.

## The three

TCOST-C1, C2 and C3 landed on main on 2026-09-03, between 03:33 and
04:06 PDT, each moving a row out of `ci.yml` and into `nightly.yml`:

| job in `nightly.yml` | moved by | merged |
|---|---|---|
| `corrupt input (release profile)` (`:492`) | TCOST-C1 | `1a1bc9fb` |
| `rustdoc (gate, every root)` (`:334`) | TCOST-C2 | `25b49f74` |
| `python suite (ungated re-take)` (`:597`) | TCOST-C3 | `59337aa5` |

**None of them has run.** The last completed nightly is run
`33741400551` (run_number 12, attempt 2, `schedule`, head `e7704028`,
concluded `success` 2026-09-03 15:57 UTC). Its job list has **nine**
jobs and none of the three above is among them — `e7704028` is
2026-09-03 09:38 UTC, before all three merges, so the workflow file it
ran did not contain them. Eight of those nine were skipped by the `has
main moved` gate; only that gate itself executed.

Run 13 (`33776282421`, `workflow_dispatch`, 16:03 UTC) is the only run
since, and it was **cancelled** five minutes in.

So the first execution of all three demoted rows is still ahead of us,
and it will happen unattended, at whatever hour the schedule fires, on
a tree nobody is watching.

## Why this is a class and not a slip

The same day, the same shape, already caught once by hand: commit
`c5263958`, *"nightly: the gated-suite re-take's pin-read step had
unbalanced quotes and never ran"*. That is a fourth demoted row whose
first hosted execution was also its first test, and it was broken. It
was found by a person reading a log, which is the compensating control
this class does not have.

**The general form:** a row demoted from a per-PR gate to a scheduled
workflow loses the thing that made it trustworthy — every PR ran it,
so a mistake in the move surfaced within minutes on someone's own
branch. In the nightly it surfaces at the next fire, to nobody, and a
row that fails to run at all reports the same green as a row that ran
and passed. `ci.yml`'s own tombstones carry the argument for each
move; none of them carries a first-run verification.

## What is owed

Not "run the nightly once" — that is the check, not the fix. What is
owed is that the demotion of a row be **verified at the demotion**:
a `workflow_dispatch` of the demoted job on the demoting PR's head, its
run id named in the PR body, so that the row is known to execute before
the per-PR copy is deleted. That is a convention with a home
(`docs/prompts/implementer-discipline.md` §2's verification-of-record
rule reaches it) and possibly a parity claim
(`scripts/check-ci-mirror-parity.py` already refuses a row that names a
path nothing runs; a row no schedule has ever fired is the same absence
one level out).

Ev's direction, 2026-09-04: do not force a dispatch now — read
tonight's scheduled run. This item carries the reading and the
convention; if tonight's run reds on any of the three, the repair is
CIW's and lands ahead of it.
