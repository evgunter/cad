---
id: c15-transcribes-the-sized-row-identity-split
kind: issue
title: work/instr/C15.md transcribes the sized-row identity split present-tense, a census reading whose one home is baseline_census.rs
status: open
opened: 2026-09-16
priority: P4
cost: E
---



## Finding

`work/instr/C15.md` states the identity split among the sized rows in
present-tense prose, twice:

- in the **Finding** paragraph — *"among the sized rows five of the
  seven identity columns are constant — `chart` and the four trim-box
  edges — and only `nu`/`nv` separate them"*, with the very next
  sentence saying the count *"lives in one home … rather than in this
  row"*;
- under `S73` — *"**five of the seven identity entries discriminate
  nothing there and the pair actually separating them is `nu`/`nv`** —
  five plus that pair being the whole list"*.

**The split is a reading of the committed baseline.** Its one
executable home is `tools/tess-lint/tests/baseline_census.rs`, whose
own module docs state the rule these two copies break: *"a number
transcribed into prose is a number nothing can check, so both are
counted here rather than written down"*. A re-cut that moved a column
from the constant list to the discriminating one reds that test, with
the column named, and leaves both sentences above saying five.

**The tail of the second one is worse than stale-able — it is
unpinnable.** *"Five plus that pair being the whole list"* is
arithmetic no code computes: `baseline_census.rs` pins the constant
list against `["chart", "u0", "u1", "v0", "v1"]` and the
discriminating list against `["nu", "nv"]`, and nothing sums them. The
identical sentence in `tools/tess-lint/src/lib.rs`'s module docs —
*"Five plus that pair is the whole list of seven"* — was removed by
INSTR unit 4 for exactly that reason, in the diff that deleted the
assertion that had appeared to pin it. `C15.md` carries the surviving
copy.

The fix is the one-home rule: name the split's home, not the count.

## Who closes it

`C15.md` is unit 18's file, so whoever runs C15 is the natural closer
— and C15's own text already argues for this fix, in the sentence that
says the count lives in one home rather than in the row.

## What it interacts with

- `work/instr/baseline-census-test-name-encodes-a-moving-reading`
  names `C15.md` as a site quoting the `#[test]` name
  `five_of_the_seven_identity_entries_discriminate_nothing_among_the_sized_rows`.
  That row's rename and this row's de-transcription touch the same two
  sentences, so they want doing together or in a known order.
- `work/instr/tess-budget-doc-identity-column-list` is the sibling on
  the other quantity — the identity list's MEMBERSHIP, in
  `docs/TESS-BUDGET.md`. Same class, different subject and different
  closer.

## Was

Disclosed by INSTR unit 4 (`instr/u4-census-partition-assert`), first
as evidence on the membership row above and then split out: that row
is unit 2's and closes when unit 2 lands, and unit 2 will not touch
`work/instr/C15.md`, so the copies would still be live when the row
carrying their record was deleted. `work/README.md` settles it — a
residue disclosed inside a closed item's prose *"reads as a record of
work done, not as an open thread, so it is invisible to the re-homing
and dies with the directory."* The flag that got it looked at was
worth raising: where a finding is a different quantity from the row it
is added to, it wants splitting off rather than dropping.
