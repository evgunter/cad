---
id: interval-test-preamble-is-copied-across-the-m10-files
kind: issue
title: The Sym/Interval test preamble is copied across the m10_* files: param_doc, failures, the budget constants and the evaluate-one-leaf helper have ten near-identical spellings
status: open
opened: 2026-09-14
priority: P3
cost: E
---


## What

Found by SYM-5's dual review (style item S5 on PR #2568), by census at
`d0d430fc7`. Four helpers are copied, near-identically, across the
`editor-core` integration suites that drive a parameterised document
through `Sym<Interval>`:

- **the budget**, in TWO spellings for one thing —
  `SymbolicDials::default()`'s `max_terms`/`max_degree` in 10 files
  (`m10_10_evidence_interval`, `m10_3_driver_interval`,
  `m10_7_census_probe`, `m10_7_probe_interval`,
  `m10_7_r1_census_probe`, `m10_7_r1_probes_interval`,
  `m10_7_r2_probes_interval`, `m10_8_arc_family_interval`,
  `m10_8_harness`, `m10_8_r2_probes_interval`) against the constants
  `DEFAULT_SYM_MAX_TERMS`/`DEFAULT_SYM_MAX_DEGREE` in 8
  (`docm2_part_interval`, `m10_7_census_probe`,
  `m10_7_r2_probes_interval`, `m10_8_arc_family_interval`,
  `m10_9_evidence_interval`, `m10_9_pins_interval`,
  `m10_derived_frame_interval`, `m10_sym_profile_interval`). Two files
  spell it BOTH ways. The two agree today only because `SymbolicDials`
  defaults to those constants;
- **`param_doc`** — the `DocEdit::SetDocParam` /
  `DocParam::Continuous` / `Distribution::Uniform` preamble, inline in
  **37** `m10_*` and `docm*` suites (75 files under `editor-core/tests` mention `DocEdit::SetDocParam` at all; the 37 are the `m10_*` and `docm*` suites the row is about);
- **`failures`** — the `ev.order` walk that renders
  `NodeResult::Failed` and `NodeResult::Poisoned` into `Vec<String>`,
  in 13 files with three slightly different string formats;
- **the one-leaf evaluator** — `analyzed_box` → `ParamBox::of` →
  `EvalOptions { param_box, profile_lift, .. }` → `evaluate`, which
  every one of the above wraps.

## Why it is worth a row

`m10_8_harness` already exists as the shared probe for the *bound*
vocabulary (`over_band_set`, `bound`, `ceiling`, `nominal_box`), and
the module header of `sym.rs` says of it: *"The instrument is one home
… and nothing in the tree spells a bound any other way."* The same
claim is not true of the preamble below it, and the budget's two
spellings are the visible cost: a change to `SymbolicDials`' defaults
moves ten files and not the other eight.

## What is owed

The helpers above lifted into `m10_8_harness` (or a sibling named for
the job), and the call sites moved. NOT a behaviour change: every
helper is a pure builder, and the goldens and counts must be
byte-identical across the move — that identity is the test of the
sweep, not a justification for it.

What this row does NOT claim: that every one of the 45 `SetDocParam`
sites wants the same helper. Several suites build several parameters
with different dimensions and distributions; the row is about the
`m10_*`/`docm*` family's single-continuous-length-parameter shape,
which is the 37.

## Home

SYM — `crates/editor-core/tests/m10*` are the tier's own suites and
SYM is the program working in them. Filed by SYM at SYM-5's fix pass
(2026-09-14).

## One more copy (SYM-7, 2026-09-15)

`crates/editor-core/tests/m10_sym_drive_memo_interval.rs`, the unit's
own suite, carries the preamble again: `eps()`, `slab()` (the M10-3
bounded chamber at the profile suite's three half-widths) and
`the_plate()` (the two-hole plate at `5.0e-5` / `1.0e-5`) are copied
from `m10_sym_profile_interval`, verbatim, so that the two suites'
numbers are about one document. That is the eleventh spelling of the
budget/fixture preamble this row counts, and it was written knowing so:
the alternative — importing them from the sibling suite — makes the
importing file's gated-suite marker name the exporting one, and couples
the unit's rows to a file it does not otherwise depend on.
