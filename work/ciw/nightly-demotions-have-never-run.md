---
id: nightly-demotions-have-never-run
kind: issue
title: A row demoted to the nightly is not verified at the demotion - the three from 2026-09-03 first ran two nights later, unwatched
status: review
opened: 2026-09-04
refs: [1650, 1654, 1655]
branch: ciw/demotion-verified
pr: 2124
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

## The reading, taken 2026-09-06 (CIW orchestrator)

Ev's direction was to read the scheduled run rather than force a dispatch.
Two scheduled nightlies have fired since this item was filed, and the second
carries the answer.

**Run 15** (`33957138686`, `schedule`, head `1817d8c2`, 2026-09-05
09:07:47Z → 09:33:14Z, conclusion `success`) executed **all three** demoted
rows. Read at the STEP level, not off the job name, because a green job over
a skipped step is exactly what this class does:

| row | job | the step that did the work | outcome |
| --- | --- | --- | --- |
| `corrupt input (release profile)` | `101282457535` | "corrupt-input suites, release profile" 09:08:40→09:09:38 | success |
| `rustdoc (gate, every root)` | `101282457521` | "rustdoc (gate, every root)" 09:08:42→09:12:53 | success |
| `python suite (ungated re-take)` | `101282457513` | "run the Python suite (unittest discover)" 09:09:55→09:10:21 | success |

**None of the three reds.** The repair this item reserved a place in the
queue for is not needed, and no figure here is inferred from a job name.

One row in the same run does sit green over skipped steps —
`nightly-only tests (demoted)` (`101282457492`), whose gate step "are any
tests demoted at all?" passed and whose four working steps skipped. That is
the gate doing its job when the demoted set is empty, not an instance of this
defect; it is noted so the next reader does not have to re-derive it.

## What is left, which is the whole point of the item

The reading is the check, not the fix. What is still owed is unchanged and is
stated above: **a demotion verified at the demotion** — a `workflow_dispatch`
of the demoted job on the demoting PR's head, its run id in the PR body,
before the per-PR copy is deleted — and the question of whether
`scripts/check-ci-mirror-parity.py` can refuse a row no schedule has ever
fired, the same absence one level out from the one it already refuses.

The evidence for the convention is now stronger rather than weaker: three
rows ran unattended two nights after their demotion and happened to be
correct, and nothing in the tree would have said otherwise if they had not
been. `c5263958` is the case where one was not.

## Disposition (PR 2124)

The convention is written. `docs/prompts/implementer-discipline.md` §2 now
carries it: `workflow_dispatch` the demoted job on the demoting PR's head,
read the STEP that does the work rather than the job name, name the run id in
the PR body, and only then delete the per-PR copy — with the 2026-09-03 three
and `c5263958` as its evidence. A second paragraph carries the same rule one
level in, for a `--selftest` sited only in a scheduled workflow.

**The parity claim this item reserved is NOT built, and that is the finding
rather than an omission.** "Has this scheduled job ever fired?" is a question
about run history; `scripts/check-ci-mirror-parity.py` is a static reader of
tracked files, and answering it needs the Actions API. A check that needs a
network call is a check that fails on a fork, offline, and in the local half —
so the demotion rule stays prose. What IS a tree question got one: claim 4
grew a second arm (see the sibling item) refusing a `--selftest` mode no
workflow invokes.

**The nightly's variable length, read so nobody re-derives it.** Run
`34024781262` (2026-09-06) concluded in 10 minutes and `34111341944`
(2026-09-07) in 27. Both have 13 jobs and both ran the body — `has main moved`
did not skip a night. The whole delta is `opt-level calibration`'s cadence
step: on 09-06 `do the measured arms have to run tonight` said no and steps
7-15 skipped (job: 23 s), on 09-07 it said yes and the two measured arms ran
10:26:27 → 10:51:51. All three demoted rows executed and passed at STEP level
on 09-06 as well.
