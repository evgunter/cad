---
id: viewer-sketch-claims-a-compile-break-a-probe-verb-does-not-cause
kind: issue
title: sketch.rs's lowering says a document verb breaks it at compile; a probe verb compiles clean, and PathTarget/Tgt::of have the same shape
status: closed
opened: 2026-09-16
closed: 2026-09-17
branch: viewer/path-form-uses-kernel-step
---


## Finding

`crates/viewer/src/sketch.rs`'s verb-lowering function carries this
header:

> **Exhaustive on [`PathStep`], and that is the point**: the step
> vocabulary is a mirror of `ProgramStep`, so a verb the document
> layer gains breaks this function rather than being silently
> unauthorable from the chrome.

The second half is false, and the first half is why. The match is
exhaustive on `PathStep` — the viewer's OWN enum — and `ProgramStep`
appears only on the constructing side of each arm. A variant added to
`ProgramStep` is therefore invisible here: nothing consumes a
`ProgramStep` in this file, so nothing can fail to cover one.

**Measured** (EDIT's PR #2738, on `editor-core`'s collapsed wire, and
the same measurement holds before it): a probe variant added to
`ProgramStep` produces exactly three `E0004`s, all in
`crates/editor-core/src/program.rs` — `step_slots`, `res_step` and
`step_bit_eq`. `crates/viewer/` compiles clean, and the new verb is
exactly what the header says it cannot be: silently unauthorable from
the chrome.

The claim is the wrong way round. What the exhaustiveness buys is the
other direction — a variant added to `PathStep` must be lowered — and
that is worth saying; it is just not what is written.

## The same shape, twice more

`PathTarget` and its lowering (`program_target`) are the same
construction one level down, and so is `pncad-py`'s `Tgt::of`: each
matches its own short enum and CONSTRUCTS the kernel/document form, so
each keeps compiling however the vocabulary it mirrors grows. Those two
are the live instances of
`work/lib/both-authoring-surfaces-are-short-of-the-target-vocabulary.md`
(LIB's — the class row and the ruling it asks for). This row is the
`viewer` prose: a sentence that asserts the guard those sites do not
have, which is worse than the silence at the other two because a reader
who checks will believe it.

## What closing it looks like

Two options, and they are not equivalent:

1. **Re-word the header** to the claim that is true — exhaustive on
   `PathStep`, so a chrome verb must be lowered; a DOCUMENT verb
   arrives unauthorable and nothing here reports it. Cheap, honest,
   and leaves the gap open with a name.
2. **Make the claim true** with a census keyed on the document
   vocabulary's own anchor. `editor_core::program::DOCUMENT_VOCABULARIES`
   and each enum's `ALL_NAMES` are projected from the declaration (see
   `crates/editor-core/tests/switch_program_vocabulary.rs`), so a
   viewer-side row asserting every `ProgramStep` name has a `PathStep`
   spelling is writable today and would red on the probe verb above.
   That is the instrument the LIB class row says did not exist before
   `TargetKind::ALL` landed, applied to the verbs rather than the
   targets.

(1) without (2) is a correct file; (2) is the thing the header was
trying to promise. A lane taking this should do (1) regardless and say
which it did.

## Filed by

EDIT's `C6` `WireStep` unit (PR #2738), whose review lane found the
sentence while reading which compile checks survive the collapse of
`editor-core`'s wire mirrors. `crates/viewer/src/sketch.rs` is
`chrome`'s and `view`'s by `work.py territory`; filed on CHROME's
slate, which carries the viewer's other doc-comment-premise rows.

## Closed (2026-09-17, branch `viewer/path-form-uses-kernel-step`)

The sentence is gone with the function it headed. The viewer no longer
has a step vocabulary to lower: a path is `profile::Step<f64>` and
lowers through `editor-core`'s own lift
(`LoopProgram::from_recorded_with_notation`), which is exhaustive on
the kernel step and tied to the document vocabulary by
`editor-core/tests/switch_program_vocabulary.rs`. A kernel verb reaches
the form with no edit and breaks `sketch::fresh_step` and
`widgets::path_step_fields` until it is given a starting step and
fields. `PathTarget`/`program_target` went too. The `pncad-py`
`Tgt::of` instance is unchanged and stays on LIB's class row,
`work/lib/both-authoring-surfaces-are-short-of-the-target-vocabulary.md`.
