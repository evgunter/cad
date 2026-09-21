---
id: probe-run-floor-does-not-hold-the-probe-f64-differential
kind: issue
title: RUN_FLOOR's m4_pr8_k_probe ignored row is 1, so the Probe-vs-f64 differential can be deleted silently
status: open
opened: 2026-09-15
priority: P3
cost: D
---


## What

`scripts/gates/probe-suite-census.sh`'s `RUN_FLOOR` rosters
`ignored:editor-core:m4_pr8_k_probe:1`. That module now carries TWO
`#[ignore]`d tests: `dump_corpus_k_samples` (the K-telemetry dump) and
`probe_agrees_with_f64_bit_for_bit_over_the_corpus` (SUITE/D114's
`Probe`-vs-f64 differential, the only check of the recording scalar's
wrapper property **at the evaluation lane**).

**A correction to this row's own first draft, kept rather than
silently fixed.** It said "the only check of the recording scalar's
wrapper property in the tree". That was false and it was a restatement
of the false premise in `work/suite/D114.md` / S168:
`crates/profile/tests/review_m2_pr2_probe.rs` ::
`probe_canonical_form_is_bit_identical_to_f64` and
`probe_errors_equal_f64_errors` have compared the two scalars bit for
bit at the canonical-form lane since M2, and are rostered
(`plain:profile:review_m2_pr2_probe:2`). The floor raise this row asks
for is unaffected — what it turns on is that `m4_pr8_k_probe`'s two
`#[ignore]`d tests do different jobs, not on the differential being
unique.

Deleting the differential leaves every check in this gate green:

- the `ignored` floor is 1, and the dump alone meets it;
- THE COMPLEMENT RULE compares `plain.ignored` against
  `ignored.passed`, and deleting an `#[ignore]`d test drops BOTH by
  one, so they stay equal. The rule closes the set against a test no
  rostered selection *reaches*; it says nothing about a test that stops
  existing.

So the instrument that carries the whole wrapper-property claim can be
removed silently, which is the state this gate exists to end one
selection over.

## Why this is not just "floors need not be maintained upward"

`RUN_FLOOR`'s own comment says growth is caught by the complement rule
rather than here, and for a suite whose tests are interchangeable that
is right. It is not right for a suite whose two `#[ignore]`d tests do
different jobs: the dump PRODUCES the CSV (its loss is loud — `run_dump`
refuses a file with nothing past its header), while the differential
only ASSERTS, so its loss is silent by construction.

## Fix

Raise the row to `ignored:editor-core:m4_pr8_k_probe:2`.

## Why GUARD's

`scripts/gates/` is GUARD's ground (`scripts/work.py territory
--files -`: *owned by guard*), and D114's fence keeps SUITE out of it.

Filed by SUITE/D114.
