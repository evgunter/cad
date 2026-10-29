---
id: a-pin-or-disabled-red-can-cite-a-closed-or-missing-row
kind: issue
title: a pinned or disabled red can cite a row that is closed or does not exist, and nothing reds
status: open
opened: 2026-10-29
priority: P3
cost: E
---


## What

GREEN's rule pins or disables a red and cites the row that owns its
fix: `#[ignore = "disabled red: work/<program>/<row>"]`,
`#[ignore = "pinned red: …"]`, `test_utils::pinned_red::row` call sites
with their row comment, and nightly.yml filters that cite a row. Nothing
checks that the cited path exists and is open, or that a closed row
leaves no pin behind.

Instance: #6027's first head routed 24,045 k-lint samples to
`work/cleave/lily-walls-curved-clearance-crowds-the-band-under-k-lint.md`,
closed on 2026-10-07 (caught in review, 2026-10-29).

There are two spellings, `pinned red:` and `disabled red:`, and nothing
reads either.

## Owed

A check (a `scripts/gates/` gate, or `work.py lint` reading the tree)
that every `pinned red:` / `disabled red:` citation names an open
`work/` row. Rows that close must therefore take their pins with them.
