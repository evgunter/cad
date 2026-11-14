---
id: nightly-reds-editor-core-digests-and-goldens
kind: unit
title: Nightly reds: the editor-core digests and goldens, the STEP fixtures and the thread-count golden
status: open
opened: 2026-11-14
priority: P1
cost: M
---

From nightly 43371638371 (main `e36675c1`, 2026-11-13), each leg's junit read whole. GREEN's rule holds (`work/green/program.md`): a red is greened here, not fixed. A move a merge explains is re-blessed, naming the merge; anything else is pinned at what main produces, or `#[ignore]`d naming the row it waits on, and the fix row goes on the program that owns the code.

**Difficulty letter:** M. **Review tier:** single review, STYLE — Each re-bless is a claim that a moved body is right, which a reader can check against the merge it names. Both logged before any draw (2026-11-14).

## What is red

All three ε rows:
- `editor-core::all lib_g16_corpus_name_digests::every_corpus_documents_name_tables_are_golden`
- `editor-core::all perf2_name_keying_differential::every_corpus_documents_name_tables_and_persisted_text_are_pinned`
- `editor-core::all perf12_census_goldens::the_at_rest_census_verdicts_are_goldened_bit_exact` (line 75)
- `editor-core::all seat4_verb_lowering::the_blend_documents_evaluate_to_their_committed_digests` (`die_fillet`) and `::the_boolean_documents_evaluate_to_their_committed_digests` (`crossing_slots`)
- `editor-core::all seat7_sweep_lowering::the_sweep_documents_evaluate_to_their_committed_digests` (`corner_table`, `boss_union`, `kitchen_sink`)
- `editor-core::all sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling` (`two_hole_plate`, `r1_annulus`, `r2_link`, `r2_filleted_bracket`)
- `editor-core::all m10_5_clearance_interval::the_answer_is_deterministic_across_repeats` (receipt has splits 991; new on 11-13)
- `step-export::all export::committed_fixtures_are_byte_golden` (`filleted_die.step`; `composed_die.step` drifts behind it)
- `sweep::all mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads` (`tilted_cut_upper`)

This slate's rows for them: `main-red-the-filleted-die-moved-in-seat4-and-the-step-export-fixture` (bisected to PRs 6533 and 6482), `main-red-two-slow-digest-rows-at-8ae2b9b`, `main-red-seat7-corner-table-moved-at-6832-and-kitchen-sink-after-it`, `seat7-sweep-digest-drifted-on-main`, `main-red-the-four-thread-walk-misses-tilted-cut-uppers-serial-golden`. Elsewhere (unchecked): `work/onecopy/main-red-the-perf12-at-rest-census-golden-moved-…`, `work/onehome/sym11s-symbolic-column-fell-on-four-documents-wi…`, `work/ciw/a-diff-that-moves-corpus-ids-runs-none-of-the-sl…`.
