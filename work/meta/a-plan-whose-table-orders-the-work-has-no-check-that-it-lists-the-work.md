---
id: a-plan-whose-table-orders-the-work-has-no-check-that-it-lists-the-work
kind: issue
title: a plan's table can drift from its program's item files in both directions with lint green
status: open
opened: 2026-09-16
priority: P3
cost: D
---


Found by CENSUS's orchestrator at the close-out of CENSUS-ERRORS-ARRIVAL
(2026-09-15) and **found again one unit later**, at CENSUS-ARRIVAL-RESIDUE's
close-out (2026-09-16), which is why it is a row rather than a fix.

## What happened, twice

`work/census/plan.md` carries a table it calls *the slate*, and its
`## Order` section reasons over that table — which row runs next, and
why. The table is a hand-written second copy of
`ls work/census/*.md` plus each file's `status`, and nothing re-derives
it.

- **2026-09-15.** Two open rows filed by CENSUS-PY-GETTERS
  (`tag-vocabularies-restated-in-py-doc-comments`,
  `ring-contact-and-census-contact-share-two-words-by-prose-alone`) had
  never reached the table, so two units' ordering was reasoned over a
  set that did not contain them; and four of six closed rows were still
  listed as if open, while two others said "Landed" in their prose —
  the table marked landing on two of six. Reconciled by hand.
- **2026-09-16, the very next unit.** Two more filed rows
  (`one-stub-convention-has-two-readers-in-two-languages`,
  `the-mint-reader-hosts-three-lexer-operations-of-its-own`) were
  absent again. Reconciled by hand again.

**The first reconciliation was a spelling correction and left no
instrument**, which is the defect CENSUS exists to prosecute, committed
by CENSUS's own orchestrator in CENSUS's own tracker. `work.py lint`
was green throughout, both times.

## Why the obvious rule is wrong, and what is proposed instead

**A repo-wide "every open item appears in its plan" rule is wrong and
was measured before this row was written.** `work/README.md:24` says
`plan.md` is *"the plan (narrative; present state only)"* and
`work/STATUS.md` is the generated board. A sweep over every program
found 25 open-and-unlisted items in `blend` and 44 in `bool` — normal,
because a plan is a narrative and most filed rows are awaiting an owner
rather than being planned. A lint rule over that would be noise on
almost every program.

What distinguishes CENSUS is that **its plan's table is the input to
its order**, so for that program an absent row is an absent row. So the
proposal is an **opt-in**: a program declares in `program.md` that its
plan indexes its items, and `work.py lint` then checks that program's
`plan.md` names every open item in its directory and marks every closed
one. Programs that do not declare it are unaffected.

The alternatives, neither taken here: rule that no plan may be an
order's input (which would cost CENSUS the thing its order is written
on), or generate the table (which would cost the per-row prose that is
most of its value).

## Why it is META's

`scripts/work.py` and `work/README.md` are META's
(`work/meta/program.md:11`). CENSUS cannot close this on its own slate;
it can only keep re-spelling the table by hand, which is what it has
now done twice.
