---
id: ab-log-sample-numbers-collide-under-branch-side-block-records
kind: issue
title: Branch-side block records make 'prior highest sample on main' collide: TRIM-3 PR-2 and D290 both recorded #201
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## What

`docs/MODEL-AB-LOG.md`'s v6 amendment 3 (the band entry, item 3) says
the dual **sample number is assigned at merge, in main's merge order**,
and the CERT precedent row spells the tiebreak: "main's merge order
rules on a collision". Under the branch-side block-record shape (the
PCURVE redaction: a block's rows stay on the program's orchestrator
branch until the block's last slot's reviews conclude) an orchestrator
that reads "prior highest on main at merge" cannot see the rows other
programs hold branch-side, so two programs claim the same number.

The instance, main's first-parent merge order on 2026-09-15:

| main merge | PR | row | number the row claims | number the rule gives |
| --- | --- | --- | --- | --- |
| `4617fcc5b` 22:55 (−07) | #2535 | CURVED-TORUS PR-2 (TARM) | #200 | #200 |
| `0e5e396e5` 00:50 | #2461 | SCALAR D290 (branch-side until block close) | #201 | #201 |
| `f1e984812` 01:00 | #2554 | TRIM-3 PR-2 (T3B), on main at `docs/MODEL-AB-LOG.md` "TRIM-3 PR-2 RECORDED AT MERGE" | **#201** | **#202** |
| `d0a89e9d1` 02:30 | #2466 | SCALAR S393 (branch-side) | #202 (written against main's visible #201) | **#203** |

TRIM's row read main's visible highest (#200) and claimed #201 ten
minutes after D290 had merged with the same number branch-side; S393
then read TRIM's #201 and claimed #202. SCALAR's rows are corrected on
`scalar/orchestrator` before the block lands (D290 #201, S393 #203, the
v-reversal door #204) with the merge-order table as the reason. TRIM's
row is TRIM's to renumber (this program's `keep_out`: every program
writes its own rows) — announced to TRIM's orchestrator on PR #2554.

**Second instance, 2026-09-15**, the same mechanism from the other
side — this time SCALAR's row was the branch-side one another program
could not see:

| main merge | PR | row | number the row claims | number the rule gives |
| --- | --- | --- | --- | --- |
| `45af343f1` | #2667 | SCALAR EXHAUST-LANE (branch-side until block close) | #208 | #208 |
| `69bcd87a1` | #2469 | PROPS mignitude-floor, on main | **#208** ("prior highest #207 on main at merge") | **#209** |
| `ebbb1598c` | #2675 | SCALAR FRAME-WITNESS (branch-side) | #210 | #210 |
| `978fcda98` | #2668 | SCALAR SENSE-FOLD (branch-side) | #211 | #211 |

PROPS' row is PROPS' to renumber — announced on PR #2469. SCALAR's
three landed with the derived numbers at block close (PR for block
SCALAR-B3). Two instances in one day, in both directions, is the
measurement that the rule cannot be read by eye.

## What the rule needs

The rule is raceless only if the number is DERIVED, not read: at block
close, the sample number of each row is its PR's position in
`git log --first-parent main` among PRs that carry a dual row, counted
from the last number every program agrees on. Either the protocol text
says that derivation (and "prior highest on main" is retired as the
method), or a small script (`scripts/ab-sample.py`, META's ground)
prints the table above for a given PR list so no orchestrator reads
the ledger by eye. Until one of those lands, every block close should
run the first-parent check before writing its numbers.
