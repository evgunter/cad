---
id: viewer-param-exists-restates-var-name-taken
kind: issue
title: The viewer's ParamExists refusal restates the declare door's VarNameTaken
status: closed
opened: 2026-10-03
closed: 2026-10-04
---

Since INTENT-VARS-1 PR 2 the declare door refuses a taken name itself:
`DocEdit::DeclareVar` answers `EditError::VarNameTaken { name, holder }`
(`crates/editor-core/src/edit.rs`, the `VarNameTaken` arm). The viewer's
create door still pre-checks the name and refuses its own flat arm,
`Refusal::ParamExists` (`crates/viewer/src/session.rs:1978`,
`crates/viewer/src/session/refuse.rs:298`).

`crates/viewer/README.md` ("`Refusal`'s delegation discipline") says a flat
arm exists only where layer 3 is the only place the fact exists. That is
no longer true of this one, so the README's example for it was removed in
the same PR. What is left is the arm itself.

What to decide, with the viewer's ops moving onto `VarId` in PR 3: either
delegate (let `DeclareVar` refuse and forward its `VarNameTaken` through
`Refusal::Edit`), or keep the flat arm for the existing definition's
dimension it carries, and say that reason in the README.

Closed by INTENT-VARS-1 PR 3: the viewer's create op is `DeclareVar`,
and a taken name now reaches the user as the declare edit's own
`VarNameTaken` through `Refusal::Edit`. The flat `Refusal::ParamExists`
arm is gone; the add-parameter form still shows the holder's dimension
before the click, read off the document (`Refusal::exists_wording`).
