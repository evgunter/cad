---
id: corpus-node-kinds-roster-is-hand-written
kind: issue
title: corpus::NODE_KINDS is a hand-written roster not welded to Node, and omits InstantiatePart and Mate
status: open
opened: 2026-09-28
---

Filed by the GATHER lane on PR 3335 (`verbatim-edge-is-not-tied-to-the-evaluator`, fix pass).

**The finding.** `crates/editor-core/tests/corpus/mod.rs`, `NODE_KINDS`, is the
domain of `m4_pr8_corpus::vocabulary_coverage_is_total`. It is a hand-typed list
of 21 strings. `corpus::node_kind` is exhaustive over `Node`, so a new variant
fails the build there, but nothing checks that the roster names every string
`node_kind` can return. Today it leaves out two: `InstantiatePart` and `Mate`.
The coverage census therefore never requires either kind, and it reports
neither as a zero. The documents that exercise them are assembly suites
(`asm_*`, `fix_pattern_mate_crossing`), not registry documents.

**Already welded elsewhere.** `tests/switch_slots.rs` (`NODE_KIND`) and
`tests/names_verbatim_edge_evaluator.rs` (`NODE_KIND`) each weld a roster to
`Node` through `test_utils::f6_variants!`, so the corpus's roster is a third,
unwelded spelling of the same vocabulary.

**Repair shape.** Weld `NODE_KINDS` through `f6_variants!` (or derive it from
`node_kind`'s match). Then decide for `InstantiatePart` and `Mate` whether the
registry should carry an assembly document or exempt them. An exemption would
sit beside `corpus::NEVER_EVALUATES` / `corpus::BESIDE_THE_REGISTRY`, the
frontier's one home since PR 3335, with its reason (registry membership runs
every ε row, the Interval lane, persistence and latency, and needs a
`PartResolver` for these kinds).
