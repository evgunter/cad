---
id: editor-core-refusals-spell-the-recourse-label-by-hand
kind: issue
title: editor-core refusals write the literal 'Recourse:' label instead of sentence::Recourse, the label's one spelling
status: open
opened: 2026-10-20
priority: P4
cost: E
---


Found by the STYLE review of PR 5427, which moved
`ParamBoxError::SymbolsOverABox` (`crates/editor-core/src/analysis.rs`,
`Display`) onto `crate::sentence::Recourse`, the re-export of
`geom_core::Recourse` (`crates/geom-core/src/predicate.rs:1356`,
"the one spelling of the label a refusal's way through opens with").

Other editor-core refusals still write the label as a literal inside
their format strings, so a change to the label would miss them:

- `crates/editor-core/src/refactor.rs:215-229`, `SplitError`'s
  `Display` (four arms: the `{LACKS}` carries and the ranked-run one);
- `crates/editor-core/src/decision.rs:211-340`, nineteen lines.

A grep for `"Recourse: ` across `crates/editor-core/src` finds 70 lines
in 12 files (`eval/mod.rs` 13, `mate.rs` 15, `checks.rs` 5, `pose.rs`
3 among them). Some are test expectations or doc text, which may keep
the literal; each wants reading.

## Repair shape

Write each refusal's ending through `Recourse(..)` (with
`format_args!` where it carries a value), leaving literals only where a
test pins the rendered sentence.

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).
