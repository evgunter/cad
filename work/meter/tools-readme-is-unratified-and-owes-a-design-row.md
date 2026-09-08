---
id: tools-readme-is-unratified-and-owes-a-design-row
kind: issue
title: tools/README.md carries clause law six code sites cite and has no ratification
status: closed
opened: 2026-09-08
closed: 2026-09-08
branch: meter/tools-readme-ratification
pr: 2147
---



## What

`tools/README.md` states the cross-column rule for `tools/tess-lint`
and `tools/k-lint` as clauses `CC1`–`CC5`. Both crates cite it by path
and by id, from their module headers, their `Admissible` tables, every
check at their reading boundaries and their `main.rs` exit constants;
`tools/tess-meter`'s `columns` carries a back-citation. Every one of
them reads as clause law. `rg -n 'tools/README\.md' tools` is the
roster, and each citing crate's
`every_clause_this_crate_cites_is_on_the_page` reds if the page or a
clause id moves.

**The page has no ratification of any kind.** `docs/DESIGN.md`'s
companion table carries no row for it. It is a lane's writeup that the
tree now cites.

## Why it is a file and not a line in `D203.md`

`D203` is closed, and a closed unit's record goes when
`work/meter/` goes at METER's close. This is the residue that outlives
the unit.

## What is owed, and by whom

An `[ev]` PR adding one companion-table row, in the spelling the
precedent uses. `scripts/gates/README.md`'s row landed in `ac1e55900`
— an `[ev]` PR — reading *"RECOMMENDATION — awaiting Ev"*, and
`4b3ed4478` flipped that cell to *"Ratified (Ev, 2026-09-06)"* when Ev
answered. `work/gates/program.md` is `status: open` to this day, so
neither step waited on a program close.

**Not a lane's to open** (CLAUDE.md: PRs that ratify design questions
wait for Ev's sign-off). The orchestrator puts the page to Ev; this
item is what keeps that owed after `D203`'s record is gone.

The page says on itself that it is unratified, so a reader who never
reaches this file is not misled in the meantime.

## Refs

Disclosed by METER unit 3's fix pass (`D203`), which moved the rule to
`tools/README.md` and made the citations rot loudly.

## Closed

Ratified. `docs/DESIGN.md` carries the row, reading *"Ratified (Ev,
2026-09-08)"*, and `tools/README.md`'s own `## Ratification` section
says the same.

Ev also settled the scope question the ratification surfaced: the page
is the **reading-boundary** rule, of which cross-column admissions are
the largest instance, not the cross-column rule. `CC1` and `CC5` are
now stated over readings generally and `CC2`, `CC3` and `CC4` carry
explicit per-column scope labels, which is the cost of the broad
reading and the thing that keeps the page from drifting the other way.
