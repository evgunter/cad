---
id: ab-log-rows-do-not-match-their-tables-declared-width
kind: issue
title: 32 A/B log rows carry fewer or more cells than the 14-column table they sit in, so a positional reader drops them or mis-assigns their columns
status: open
opened: 2026-09-11
priority: P3
cost: E
---



## The measurement

Derived on `docs/MODEL-AB-LOG.md` at the escaping pass below, by reading each
table's width from its own separator row, skipping the header line above it, and
splitting body rows on unescaped pipes only (`(?<!\\)\|` — the same rule a
correct markdown reader applies):

- **27 tables**, declaring two widths: 14 (the standard dispatch table) and 9
  (the PERF block table at `docs/MODEL-AB-LOG.md:5101`, which is internally
  consistent and not part of this finding).
- **310 body rows**, of which **32 do not carry their own table's width**. Every
  one of them sits under the standard 14-column header.

| cells | rows | ids |
| --- | --- | --- |
| 6 | 8 | `SHELL-1`, `SHELL-2`, `SHELL-5`, `SHELL-6`, `SHELL-7`, `SHELL-8`, `SHELL-9`, `SHELL-10` |
| 9 | 12 | `QA-1`, `QA-2`, `QA-3`, `QA-9`, `QA-6`, `QA-5`, `QA-8`, `QA-6B`, `PROPS-1`, `Span`, `k-stats`, `coeffs` |
| 10 | 1 | `QA-7` |
| 15 | 10 | `RING`, `M10-DI`, `M10-P`, `CERTM1`, `SEAT5`, `SEAT6`, `SEAT7`, `K1`, `K2`, `K3` |
| 16 | 1 | `DOCM-5` |

## Why it matters

A positional reader has no way to be right about these. A 6-cell row under a
14-column header renders with eight empty trailing cells and reads, positionally,
as though its `review` column were its `arm` column; a 15-cell row shifts every
column after the extra one. The analysis pipeline's `blind_extract.py` takes the
conservative branch and **drops** any row that is not exactly 14 wide, which is
why it extracts 257 of 310 rows — silently, with no diagnostic. The alternative
branch, reading positionally anyway, is worse: it would attribute one row's
tokens figure to another row's column.

Neither the reader nor this program can repair them. A row short by five or
eight cells does not say which columns were omitted, and only the orchestrator
that wrote it knows whether `SHELL-1`'s six cells are the first six columns, a
different schema it meant to declare, or a row cut off mid-write.

## Not this item, and already done

**The ambiguous-pipe class is closed.** 21 rows carried raw `|` inside cell
prose (`|Δ|≤π−δ`, `\|onto\|²`), so the cell boundaries were not recoverable by
any reader splitting on a bare pipe; they now spell those pipes `\|`, which is
what the file already did elsewhere. That pass changed no text but the escaping,
verified by normalising the escapes away and comparing to the original
byte-for-byte, and it leaves `blind_extract.py`'s output identical (257 rows,
same bytes) because it touched no row that previously parsed at 14. It does not
fix this finding: escaping makes a row's boundaries recoverable, it does not
change how many cells the row has.

## Routing

The rows belong to `S-QA`, `SHELL`, `PROPS`, `M10`, `CERT`, `SEAT`, `DOCM`,
Track K and `RING`'s owner, not to this program: META owns the log's rules,
rosters and format — the band allocation, the protocol sections, the stopping
rule — and never another program's row, which is the same fence
`stale-track-t-citations-in-fillet-and-cert` records. Each owner re-cuts its own
rows to 14 columns, or declares the narrower table it meant with its own header
and separator, as the PERF block does.

The cheap tripwire, if one is wanted: the same census as a check, erroring on a
body row whose width is not its table's. It belongs with the log's format, which
is this program's.

## Four of the 32 fixed at source (PROPS orchestrator, 2026-09-15)

The four 9-cell rows in this finding's list that are PROPS's own —
`PROPS-1`, `Span`, `k-stats`, `coeffs` — now carry 14 cells, so
`blind_extract.py` stops dropping them and the program's whole A/B
contribution becomes visible to the analysis. Nothing was rewritten:
each row's oversized review cell split at the seams its own prose
already had, `Silent deviations:` opening the silent-deviations cell
and `Fix pass` opening the fix-pass-size cell, with the idiom, tests
and docs cells filled from the `idiom N / test N / doc N` figures
stated inside each arm's parenthesis (recorded as `R1 x / R2 y`, the
shape the other dual rows use). The review, battery, tokens and
wall-clock cells are byte-identical to what they were. The new
`mignitude` row (ordinal 2404) was written 14-wide to begin with.

That leaves **28** rows in the finding, none of them PROPS's. The
remaining 9-cell group is QA's eight; the 6-cell group is SHELL's
eight; the 15- and 16-cell groups are M10's, CERT's, SEAT's, K's and
DOCM's. Each is its own program's to fix at source the same way, and
the mechanical part is the same four seams every time — which is
probably worth saying once in the protocol section rather than eight
times in eight programs' logs.
