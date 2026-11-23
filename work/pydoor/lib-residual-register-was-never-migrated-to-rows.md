---
id: lib-residual-register-was-never-migrated-to-rows
kind: issue
title: LIB's residual register (categories A-F, 2026-08-10 and the 2026-08-28 re-survey) lived only in LIB's log and was never filed as rows
status: open
opened: 2026-11-03
priority: P3
cost: M
---

## What

LIB kept a register of library residuals in its log: "LIB residual
register" (2026-08-10, categories A to E), with category F folded in by
the re-survey of 2026-08-28 ("LIB re-survey", the same log). The
tracker migration of 2026-09-03 moved the log into `work/lib/` and
created only two rows from it (LIB-TUBE and LIB-G17). Nothing since
filed the register's open entries as rows.

Permanent docs still point at it as live:
- `docs/LIBRARY-DESIGN.md` (the status paragraph near the top, and the
  retired-table note: "the one place that tracks it");
- `docs/guide/north-star-audit.md` (the bindings-parity residual, and
  the G15 and G18 rows' "ASM cross-program deposit" of 2026-08-23);
- `crates/pncad-py/tests/test_binding_census.py` (three pointers into
  the register's paragraphs).

LIB closed on 2026-11-03 and its log left the tracker. Those citations
now name `docs/doc-ledger/lib-leaves-the-tracker.md`, which gives the
recovery command:

    git show 0f01217666:work/lib/log.md

## What to do

Read the register and the re-survey at that SHA. For each entry still
open in the tree, file a row on the track whose ground it lands on.
Then rewrite the citations above to name those rows, or to state the
residual in the doc itself, so no permanent doc depends on a log.
Many entries were probably discharged by LIB's later units; the
register marks some of them DISCHARGED and not others.
