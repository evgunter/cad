---
id: a-diff-to-a-shared-fixture-list-skips-the-slow-rows-that-iterate-it
kind: issue
title: The PR gate does not run the slow-set rows that iterate a shared fixture list when a diff adds to that list, so the nightly finds the red
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Filed by the EXPORT orchestrator from the full review of PR 7077 (`export/void-shells`), 2026-11-17.

## What happened

PR 7077 added a `voided` body to step-export's shared fixture corpus (`crates/step-export/tests/common`, `SOLID_FIXTURES`, the corpus lists the import tests read). Two slow-set rows iterate those lists:
- `step-import` `perf12_census_goldens::the_import_census_verdicts_are_goldened_bit_exact` (a census golden per ε row);
- `step-export` `m5_pr13_curved::the_export_corpus_obeys_the_exactness_frame_sense_and_nurbs_laws`.

The reviewer ran both locally and both went red. Hosted CI on the head (run 44165309662) was green, because `ci.yml` runs only the slow tests a diff *adds or edits*, and this diff edited the list, not the tests. Merged, the PR would have reddened the nightly.

## The class

A slow-set test whose input is a shared list (a fixture corpus, a census table, a named-files array) is coupled to every diff that changes that list. The change filter selects slow tests by the test file's own path or name appearing in the diff, so it cannot see the coupling. Every corpus-iterating slow row is exposed the same way.

## Where to look

- `scripts/ci-filter.py`: how slow-set rows are selected (the "slow-set tests this diff adds or edits" step).
- `.config/nextest.toml`'s `ci` filter: the slow set.
- Candidate shapes, for the owner to weigh: a declared map from a shared-list file to the slow rows that read it; or treating an edit to a `tests/common` / fixture path as an edit to every slow row in that crate.

## Done when

A diff that adds an entry to a shared fixture list runs, on its PR, the slow-set rows that iterate that list.
