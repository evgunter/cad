---
id: three-shell-splitters-nothing-compares
kind: issue
title: three hand-written shell command splitters in scripts/, with independent break sets and nothing holding them against each other
status: open
opened: 2026-09-11
refs: [apt-preamble-bypass-is-unguarded]
---


Found by the correctness review of PR 2345.

`scripts/` now holds **three hand-written splitters of shell text into the
commands it runs**, each with its own break set, each written against a
different property:

- `scripts/check-ci-mirror-parity.py` — `_command_spans` / `_simple_commands`,
  whose `CMD_BREAK` comment records meeting the `&`-in-a-redirection defect
  twice and draws the lesson that *a check that reds on a CORRECT change gets
  routed around, and then detects nothing at all*.
- `scripts/check-status-capture.py` — `scan_units`, which cites that comment
  in its own header and re-implements the rule.
- `scripts/check-install-wrappers.py` — `scan_commands`, which cites the same
  comment again and re-implements it a third time.

**Nothing holds them against each other.** Each has its own mutant table and
each table is written against its own property, so a shape one of them learns
— `|&`, `<<-`'s tab stripping, `$'…'` ANSI-C quoting, a `case` pattern's
unmatched `)`, an unbalanced quote spanning lines, a `<<<` here-string — is
learned by that file alone. PR 2345 taught its own splitter five such shapes
in one review pass; two of them (`$'…'`, the `case` pattern) are shapes the
other two splitters also meet and answer differently, and no row anywhere
says so.

The separation itself is argued and accepted: each script fails alone, and a
shared tokenizer would let a change made for one property move another
claim's answer (`check-status-capture.py`'s header, and PR 2345's). What is
NOT argued is that three independent break sets should drift with nothing
measuring the drift.

## Two shapes, neither costed

1. **A shared corpus, not shared code.** One table of shell fragments with
   the boundaries each splitter should find, run by all three selftests
   against their own splitter. Divergence is then a row somebody reads,
   while each file keeps its own reader and its own failure.
2. **Leave them independent and say so at each**, with a note naming the
   other two, so the next lane teaching one a shape knows there are two more
   that do not know it.

What makes this worth a row rather than a shrug is that all three splitters
gate `main`, and the failure of a splitter is always the same one: a body it
misreads is a body reported as agreement.
