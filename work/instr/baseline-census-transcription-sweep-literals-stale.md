---
id: baseline-census-transcription-sweep-literals-stale
kind: issue
title: The transcription sweep's hit list certifies a sweep whose subject has moved, and nothing schedules a re-sweep
status: open
opened: 2026-09-16
---



## Finding

`tools/tess-lint/tests/baseline_census.rs`'s module docs carry a
section headed *"## The transcription sweep, and its hit list"*,
written in the live voice: *"The pattern swept for was that
paragraph's own quantities — the literals `8 pairs` / `16 of` /
`22,545` / `1327` / `five of the eight`"*, followed by a four-row hit
list and a **What it could not match** clause.

**Every literal in that pattern matches nothing the file asserts
today.** The same module's tests assert `pairs == 11`, `in_pairs ==
22`, `all.len() == 1605`, `all_pairs == 29_726`, and the split is
*five of the seven*. So the sweep's subject has moved under it in
every dimension, and the hit list below certifies coverage of a
pattern no longer spelled anywhere in the tree it swept.

The record is not wrong — the section's own **The FIGURES in that list
are frozen** paragraph settles that a dated sweep may keep the numbers
it reported. What is missing is anything that would run the sweep
again at the CURRENT literals. Nothing does, and the cost is
concrete: a re-sweep would have found `lib.rs`'s *"Five plus that pair
is the whole list of seven"*, a live prose copy of a census reading,
which INSTR unit 4 found only by reading the file.

## Shape

The Q6 measurement shape: a figure with no guard, no register entry,
and no written reason it can have neither. The remedy is one of the
three, not necessarily the guard — a register row saying "re-sweep on
each re-cut" would discharge it.

## Was

Disclosed by INSTR unit 4's style review (`instr/u4-census-partition-assert`).
