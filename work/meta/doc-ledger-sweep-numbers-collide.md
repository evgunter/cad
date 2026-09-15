---
id: doc-ledger-sweep-numbers-collide
kind: issue
title: DOC-LEDGER sweep numbers collide: three Sweep 6, three Sweep 7, and the sequence is not ordered — each closing program picks its own number by reading the file
status: open
opened: 2026-09-15
---


## Finding

Raised by the SUITE orchestrator, 2026-09-15, on writing sweep 15 —
after first writing it as **14**, which was already taken.

`docs/DOC-LEDGER.md` carries, by `grep -n "^## Sweep"`:

```
  83  Sweep 1    2026-08-20
 292  Sweep 2    2026-08-20
 331  Sweep 3    2026-08-28
 414  Sweep 4    2026-09-03
 487  Sweep 5    2026-09-03
 556  Sweep 6    2026-09-04  S-MATE
 583  Sweep 7    2026-09-06  S-CERT
 663  Sweep 8    2026-09-06  SEAT
 885  Sweep 6    2026-09-03  finished work leaves docs/     <-- collision
1089  Sweep 6    2026-09-04  VERBS                          <-- collision
1434  Sweep 7    2026-09-08  GATES                          <-- collision
1480  Sweep 7    2026-09-06  FILLET                         <-- collision
1669  Sweep 9    2026-09-08  EVAL
1759  Sweep 10   2026-09-09  METER
1930  Sweep 14   2026-09-14  DOCM
2194  Sweep 13   2026-09-13  M10
2441  Sweep 12   2026-09-12  CITE
2565  Sweep 11   2026-09-11  code-quality
```

**Three `Sweep 6`, three `Sweep 7`**, and the sequence is not monotonic
in either direction: 1–10 ascend, then 14, 13, 12, 11 descend. So a
number does not identify a sweep, and position does not order them.

## Why it happens, and why the obvious fix is the wrong one

Each closing program writes its own entry and picks the next number by
reading the file. Two programs closing in the same window each read the
same maximum and each take it — which is exactly what the two clusters
of collisions look like (2026-09-03/04 and 2026-09-06/08). One-file-one-
item cannot prevent it, because the ledger is deliberately **one** file
that every program appends to; that is what makes it the done-state of
record.

**Renumbering the existing entries is not the fix and should not be
done.** The numbers are cited from outside: `work/README.md` cites
*"`docs/DOC-LEDGER.md`, sweep 11"* for `work/code-quality/`'s deletion,
and `memories/docs-ledger.md` and several program headers cite sweeps by
number too. Renumbering would silently invalidate every one of those
citations, and the citations are the whole reason the ledger exists.

## Settled for future sweeps (Ev, 2026-09-15)

**A sweep is named for what it swept, not numbered** — `## SUITE sweep
— 2026-09-15`. A name is unique by construction, because a program
closes once, so nothing has to be read out of the file to pick it. The
convention is stated at `docs/DOC-LEDGER.md`'s "Naming a sweep" section,
above the entries, where the next closing program will meet it. SUITE's
own entry is the first to use it.

## What is still open: the collisions already in the file

Naming the next sweep does not renumber the four collided headings, and
**they should not be renumbered.** The numbers are cited from outside:
`work/README.md` cites *"`docs/DOC-LEDGER.md`, sweep 11"* for
`work/code-quality/`'s deletion, and `memories/docs-ledger.md` and
several program headers cite others. Renumbering would silently
invalidate every one of those citations, and the citations are the whole
reason the ledger exists.

So the open question is narrower than it was: **what, if anything, to do
about three `Sweep 6`, three `Sweep 7`, and a sequence that ascends to
10 and then descends from 14.** Three answers, none urgent:

1. **Nothing.** Leave them; the naming convention means the set never
   grows. A reader who follows a citation to "sweep 6" finds three
   entries and disambiguates by the subject line, which every one of
   them carries.
2. **Disambiguate in place** — append the subject to each collided
   heading (`## Sweep 6 — 2026-09-04: VERBS leaves the tracker` already
   does this; two of the six do not) without changing any number, so a
   citation still resolves and a reader can tell which is meant.
3. **A guard** that refuses a new duplicate heading. Largely moot under
   the naming convention, and it would have to permit the existing
   duplicates, so it guards against a mistake nobody can now make.

(2) is the cheap one and loses nothing. This row's author has not taken
it, because editing six headings another program wrote is the kind of
tidying that should be someone's deliberate choice rather than a
closing program's parting edit.

## The instance

SUITE's entry is **sweep 15** and was written as 14 first. The mistake
came from reading `grep -n "^## Sweep" | tail -3`, which returns the
**oldest** three in a file whose newest entries are at the bottom of an
ascending run and then descend — so the tail showed 13, 12, 11 and the
maximum 14 was off-screen. A `sort -n | tail -1` over the extracted
numbers is what found it. That is the same defect this program spent
its life on: **a filtered view read as a population.**
