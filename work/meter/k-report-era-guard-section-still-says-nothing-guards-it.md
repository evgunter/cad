---
id: k-report-era-guard-section-still-says-nothing-guards-it
kind: issue
title: k-report: the era-guard section describes the guard as unwritten and dates its figures as un-retaken; the row has landed
status: open
opened: 2026-09-08
refs: [k-report-era-witnesses-have-no-guard]
---


Disclosed by unit 11, which wrote the row that section asks for.
`docs/K-REPORT.md` was outside that unit's fence (it had just merged),
so the section is filed rather than edited.

## Finding

`docs/K-REPORT.md`'s **"These figures have no mechanical guard, and
here is the one that should exist"** (the M11 addendum's last section)
is a request for a guard, written when there was none. Four of its
sentences have gone false now that
`tools/k-lint/tests/threshold_provenance.rs` carries
`the_m7_era_still_carries_the_witnesses_the_report_names`:

- *"**The era claim is not covered by either**"* — it is covered now,
  by a row on every `k-lint (gate)` run.
- *"That claim is guardable and cheaply"* — present tense about
  something that has been done.
- *"A row asserting those three … **would** go red the moment the era
  claim went false … it is filed with the values and the extraction:
  `work/meter/k-report-era-witnesses-have-no-guard.md`"* — that item is
  closed, and its file goes with the program's directory at close, so
  the pointer dangles from then on.
- *"Until it lands, these figures are dated to `c39a904e` and nothing
  re-takes them"* — the three witnesses are re-taken on every gate
  run. The rest of the section's figures (the 281-name census, the
  corpus growth) are still un-retaken, so the sentence is half true,
  which is the harder half to spot.

## What the fix is not

Not a re-cut and not a re-measurement: every number in that section
still describes `c39a904e` correctly, and
`docs/k-report-data/README.md` rule 1 is untouched. What is stale is
the section's account of what the tree does — the class the M11
addendum fixed four instances of elsewhere in the same document.

## What the replacement has to keep

The guard's own limit, which is the part a reader will otherwise take
the wrong way. It reads committed files, so it says the values M7
carries are the values this report names, and it cannot say M7 is
still the era that SHOULD be shipping — a distribution that moved
under a corpus the row never opens leaves it green. That sentence
lives at the row (its doc comment states it) and the report's own
account should not claim more than the row does.

Refs: `docs/K-REPORT.md` (M11 addendum, final section),
`tools/k-lint/tests/threshold_provenance.rs`.
