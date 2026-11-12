---
id: the-boolean-corpus-name-digests-are-red-on-main
kind: issue
title: main red: three corpus name-table and boolean digest rows read crossing_slots 0xd17d36af80b34d6b
status: open
opened: 2026-11-12
priority: P0
cost: M
---

Found by ONEDOOR's shared-cores lane (PR 6858). Its full run at `07721c47a` failed three editor-core slow-set rows, and the same three fail at pure origin/main `5589777fd` and at `96a5233a5`, before that PR's fix pass, each with the same digests:

- `crates/editor-core/tests/lib_g16_corpus_name_digests.rs` `every_corpus_documents_name_tables_are_golden`;
- `crates/editor-core/tests/perf2_name_keying_differential.rs` `every_corpus_documents_name_tables_and_persisted_text_are_pinned`;
- `crates/editor-core/tests/seat4_verb_lowering.rs` `the_boolean_documents_evaluate_to_their_committed_digests`, which reads `crossing_slots: 0xd17d36af80b34d6b`.

All three read the corpus's `crossing_slots` document, so they are probably one move. No filed main-red row names them: `main-red-the-filleted-die-moved-in-seat4-and-the-step-export-fixture` covers seat4's *blend* row, not its boolean one.

Owed:
- bisect first-parent on main to the merge that moved `crossing_slots`;
- decide whether the new body is right before any golden is re-cut;
- if it is, re-pin it in that merge's lane, with what moved.

(ONEDOOR orchestrator, 2026-11-12)
