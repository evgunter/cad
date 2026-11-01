---
id: baseline-census-transcription-sweep-literals-stale
kind: issue
title: The transcription sweep's hit list certifies a sweep whose subject has moved, and nothing schedules a re-sweep
status: open
opened: 2026-09-16
priority: P3
cost: D
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

## Evidence from INSTR unit 1 (2026-09-16): the SIZING half, re-derived

`baseline-sizing-census-second-copy` carried the same defect one
subject over — its own sweep pattern was the SUPERSEDED sizing
literals (`46,019`, `110,811`, `93,066`, `44,162`), so re-running it
found the stale copies and said nothing about the live figures. Unit 1
re-derived that pattern from what the census asserts today and ran it:

```
grep -rnE '362[,_]?154|261[,_]?106|88[,_]?036|147[,_]?960|127[,_]?966|76[,_]?599|1\.6807|1\.1493|29[,_]?726' \
  --include=*.md --include=*.rs --include=*.py --include=*.sh --include=*.toml --include=*.yml .
```

**Two things that bear on this row.**

- The re-derived sweep covers `all_pairs == 29_726` and every figure
  `the_committed_baseline_sizes_this_much` asserts, so that much of
  the hit list above is discharged as of 2026-09-16: outside the
  census itself the only hits are dated tracker records
  (`work/instr/log.md`, this row's own quotation). It does NOT cover
  `pairs == 11`, `in_pairs == 22`, `all.len() == 1605`, `sized == 80`
  or the *five of the seven* split — those literals are short enough
  that a grep for them is noise, which is the same blind spot the
  frozen sweep had.
- **A re-derived pattern goes stale the moment it is written**, which
  is this row's point stated as a receipt: the pattern above was
  correct for exactly one cut, and the next re-cut moves six of its
  nine literals. Nothing schedules the re-derivation, and unit 1 did
  it only because its brief said the item was stale about itself. A
  register row saying "re-derive and re-sweep on each re-cut" is the
  remedy this row already proposes, and the sizing half now has a
  second instance of the cost.
