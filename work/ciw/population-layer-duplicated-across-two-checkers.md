---
id: population-layer-duplicated-across-two-checkers
kind: issue
title: two checkers carry the same population reader nine functions deep, and scripts/ already imports siblings by path
status: open
opened: 2026-09-11
refs: [apt-preamble-bypass-is-unguarded]
---


Found by the correctness review of PR 2345, which landed
`scripts/check-install-wrappers.py` beside `scripts/check-status-capture.py`.

## The measurement

Function-by-function similarity between the two files, as measured at that
review:

| function | similarity |
| --- | --- |
| `_tracked` | 1.00 |
| `_write` | 1.00 |
| `_add` | 1.00 |
| `yaml_files` | 0.89 |
| `yaml_bodies` | 0.88 |
| `_heredoc_delim` | 0.83 |
| `shell_files` | 0.81 |
| `Carrier` | 0.75 |

plus five module constants that are identical (`YAML_ROOTS`,
`SHELL_SUFFIXES`, `SHEBANG_RE`, `RUN_KEY`, `BLOCK_SCALAR`).

## Why it is a finding and not a style note

**The argument that was made covers `scan_commands` only.** PR 2345 argued
that the two TOKENIZERS must stay separate, because they disagree about
heredocs and quoting in exactly the places each one's property lives, and
that a shared one would make a change for one property move the other's
answers. That argument is accepted and is not in question here. **None of
the functions above touches a tokenizer.** They answer "what files are in
the population" and "where are the `run:` bodies", which the two checkers
answer identically — and the review found exactly one sentence of
duplication prose in the whole new file, at `yaml_bodies`.

**It has already cost a real defect, in the direction duplication always
does.** The same `yaml_bodies` carried two holes in both copies: a quoted
inline `run:` scalar read as one shell word, and a block header of
`|2` / `|2-` / `|-2` / `| # note` falling through to the inline arm with the
body never read. Both were found in the new file and had to be fixed twice,
in the same PR, by hand. The next hole will be found once and fixed once.

**A shared module is not one of the two shapes the PR argued against.**
`scripts/` already imports sibling scripts as modules by path in four places
— `check-interval-cfg-additive.py:299`, `interval-only-selection.py:100`,
`pr-added-tests.py:108`, `check-ci-mirror-parity.py:2472` — so the precedent
for "one script reads another" exists and is used. What is open is whether
the shared thing should be a module both import, or whether the duplication
is the price of "each fails alone", stated where a reader meets it rather
than inferred.

## What to decide

1. A `scripts/_ci_population.py` (or similar) holding `_tracked`,
   `shell_files`, `yaml_files`, `yaml_bodies` and the constants, imported by
   both; the `Carrier`/`_write`/`_add` selftest helpers are a second, smaller
   question with the same answer.
2. Or: keep two copies and write the reason at both, with a rule for what
   happens when one is fixed (the sweep obligation already says: fix both in
   the same PR — this item exists because that only works if someone
   remembers).

Whichever is taken, the tie-break is that a fix to a population reader is a
fix to what a gate READS, and a gate that reads less than it says it reads
is this program's oldest defect.
