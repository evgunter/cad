---
id: fork-log-row-numbers-have-no-single-allocator
kind: issue
title: Design-fork log row numbers collide: each program takes max+1 on its own branch, so rows 251, 255, 259, 260 and 261 each appear two or three times and 'fork N' citations are ambiguous
status: open
opened: 2026-11-12
priority: P2
cost: E
---


## What

`docs/DESIGN-FORK-LOG.md` numbers its rows by hand. Each program's `[ev]` PR picks the next number on its own branch, so concurrent forks take the same number. On main at `da77d875`:
- 259 appears three times (lines 66, 69, 369);
- 260 four times (70, 123, 366, 370);
- 261 twice (65, 71);
- 251 twice (361, 362);
- 255 was renumbered once already (ONEHOME, PR 6589).

A citation such as "fork 259" in a work row, a commit or a PR body cannot be resolved. PR 6820's delta review found this (Style Q2).

A number with no single allocator is a fact decided at several sites. The row already has a unique key: its analysis branch, `analysis/design-fork/<slug>`, minted at dispatch (protocol rule 3).

## Done when

A row's citation is unique by construction: the row is keyed by its fork slug, or the number is allocated at one place, and a lint check refuses a duplicate. The duplicates on main today are resolved and their citations rewritten.

The protocol file is Ev's to change (`docs/DESIGN-FORK-PROTOCOL.md`, "Changing the protocol"). So keying rows by slug is an `[ev]` question, unless a lint check alone suffices without changing the protocol.
