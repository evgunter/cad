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

## Bisected (ONECOPY, `onecopy/main-reds-triage`, 2026-11-09)

The filleted die's move is PR 6533 (`onecopy/p0-sheared-fillet`, merge `fe7806519a`, "a blend band's directions are read off its stored geometry, not off its feet's points"), first-parent at ε 1e-9. At `fe7806519a^1` seat4's row passes and `filleted_die.step` regenerates byte-equal. At `fe7806519a` seat4 reads `die_fillet: 0x7418085874f97095 (want 0x9d27e739ef2f43bd)`, the digest main reads, and `filleted_die.step` drifts. So the two rows are one move. PR 6533's description does not disclose either. The open question is unchanged: is the die's new body right? PR 6533 changes how the blend band reads its directions, so the body may be meant to move; its lane owes the answer before either golden is re-cut.

**A second, hidden drift.** `committed_fixtures_are_byte_golden` stops at the first fixture that drifts, so it hides any later one. Regenerating every fixture on main (merged 2026-11-09) with `cargo run -p step-export --example export_fixtures` drifts two of 17 files: `filleted_die.step` and `composed_die.step`. `composed_die.step` already drifted at `fe7806519a^1`, before PR 6533, when the row failed on it alone. Its mover is being bisected from `f1ec7c3f93`, the last first-parent merge that wrote the fixture.
