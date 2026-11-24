---
id: a-kept-refusal-keeps-its-text-where-it-must-keep-its-sentence
kind: issue
title: A refusal stored before it is shown keeps a String, so its notation is chosen before its reader is known
status: open
opened: 2026-11-20
priority: P2
cost: M
parent: patherror-display-renders-float-noise
---


A told quantity renders in a notation (`geom_core::Tell`,
`crates/geom-core/src/told.rs`), and a `profile::PathError` is told in
one. Between the kernel and the person, these sites turn the refusal
into a `String` first, which is its `Display`, which is metres and
radians:

- `editor_core::program::ProgramRefusal::Geometry { rendered: String }`,
  filled in `ProfileProgram::replay_records`
  (`crates/editor-core/src/program.rs`) from `PathError::to_string()`,
  and `ProgramRefusal::Order { rendered: String }`, filled in
  `loop_shape` from `OrderFault::to_string()`. This is the chain a
  committed sketch's refusal takes to the viewer
  (`viewer::session::Refusal::Edit` → `EditError::ProfileProgramRefused`).
- `editor_core::eval`'s `Say` and `NodeError::spoken`
  (`crates/editor-core/src/eval/mod.rs`): `NodeErrorKind::ProfileReplay`
  holds a typed `profile::ReplayError<f64>` and writes it with `{error}`;
  the chain takes no notation. `viewer::tree::status_of` stores the
  result as `RowStatus::Failed { message: String }`.
- `editor_core::drive` (`DriveRefusal::WitnessDoesNotBuild { cause:
  String }`) and `Evaluation::reading_spoken`.
- In the viewer: `frame::refusal_message` and `frame::tool_news` (a
  `Message`'s text), `tools::PanelRefusal::new` (`why.to_string()`),
  and `session::refuse::Refusal::write`, which takes no notation
  (`DrivenByExpression` stores its own for its one number).

`geom_core::Sentence` is the type for the first kind: it keeps the
refusal and is told later (`viewer::sketch::PreviewError::Geometry`
holds one, so the sketch preview's refusal reads in the working
notation). The second kind needs the notation passed down the `Say`
chain and `Refusal::write`. The Python door and the CLIs have no
reader's notation and stay on `Display`.

Found by the notation unit's measure-first survey
(`a-refusal-is-told-in-the-readers-notation`).

## Re-banded P1 to P2 (2026-11-22)

Ev, in chat: work that is only about how a number or a refusal is formatted, with no further design ramification and little risk of compounding when built on, is skipped or P2 once the P0s are done. What entrenches here (the told type, the notation plumbing, what a typed number means) is built or in review; what is left on this row is coverage of that pattern, crate by crate.
