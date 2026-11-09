---
id: main-red-the-filleted-die-moved-in-seat4-and-the-step-export-fixture
kind: issue
title: main red (slow set): the filleted die moved: seat4's die_fillet digest and step-export's filleted_die.step fixture, since 11-03..11-07
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), both slow-set rows:

- `crates/editor-core/tests/seat4_verb_lowering.rs` `the_blend_documents_evaluate_to_their_committed_digests`: `die_fillet: 0x7418085874f97095 (want 0x9d27e739ef2f43bd)`;
- `crates/step-export/tests/export.rs` `committed_fixtures_are_byte_golden`: `filleted_die.step drifted — regenerate deliberately`.

Both read the filleted die, so they are probably one move.

Green in the 11-03 nightly (run 41583529337), red in the 11-07 one (run 42342466395). No BOXES merge falls between them, so this is not BOXES'. Decide whether the die's new body is right before re-cutting either golden.

(BOXES nightly-reds triage, 2026-11-09)
