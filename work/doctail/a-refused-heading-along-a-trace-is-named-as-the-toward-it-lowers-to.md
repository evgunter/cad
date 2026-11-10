---
id: a-refused-heading-along-a-trace-is-named-as-the-toward-it-lowers-to
kind: issue
title: A heading along a trace refused off the lattice is named as the Toward it lowers to, and offered the loop forms
status: open
opened: 2026-11-06
priority: P4
cost: E
---


Found by the STYLE review of ROUND's
`fillet-step-discoverability-six-door-refusals` (PR 6549).

`ProgramStep::AlongTrace` lowers to `toward(dx, dy)` (`crate::trace`,
`StepShape::verb`), so the replay that refuses it off the lattice
reports `Verb::Toward`. A program holding one runs from the pen
(`ProfileProgram::runs_from_the_pen`), so the edit door's check and
the persist door's `first_program_fault` both pass it, and the refusal
first surfaces at evaluation:
`crates/editor-core/src/eval/wire.rs` maps the replay's error straight
into `NodeErrorKind::ProfileReplay { loop_, error }`, which renders
`profile::ReplayError`'s sentence. That sentence
- names the step `Toward`, a step the author did not write; and
- says its recourse in `profile::Vocabulary::Program`, so at the
  entry it offers `Circle`, `CircleSplit` and `Rectangle`, which a
  chain cannot spell (the edit door says `Vocabulary::Chain`).

The repair needs the authored step at the refusal — the `ProgramStep`
variant at `error.step` of a `LoopProgram::Chain`, which the evaluation
has where it maps the error — and a payload that carries it, so it is
not the ROUND unit's small one. `eval/wire.rs` is HELDRECORDS', TALLY's
and WIREREST's ground; the sentence is DOCTAIL's.
