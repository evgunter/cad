---
id: viewer-param-vocabulary-names-a-variable
kind: issue
title: The viewer's ops, refusals and selection call a variable a param
status: open
opened: 2026-10-04
---

INTENT-VARS-1 PR 3 moved the viewer's operations onto `VarId`, but its
vocabulary still calls a variable a parameter:

- `SessionOp::SetParam`, `SetParamUnit`, `SetParamText`
  (`crates/viewer/src/session/op.rs:133`, `:152`, `:181`), and the
  `BeginParamGesture`/`PreviewParamGesture`/`CommitParamGesture` ops
  and `ValueGestureName::Param` (`op.rs:899`);
- `Refusal::NoSuchParam` (`crates/viewer/src/session/refuse.rs:274`);
- `Selection::Param` (`crates/viewer/src/session/select.rs:187`) and the
  second `Param { present }` arm in the same file;
- `props::param_rows` and `ParamRow` (`crates/viewer/src/props.rs:928`).

PR 3's review asked for `SetParam`, `NoSuchParam` and `Selection::Param`
to be renamed where that is mechanical. It is not, taken alone: the
three sit in a vocabulary of a dozen names, and "parameters" is also
what the panel shows a person. Renaming three leaves the vocabulary half
moved; renaming all is a decision about what the GUI calls a variable,
which the next unit (where the GUI's variable affordances are designed)
is the place for.

Raised by PR 3's dual review (r2 S4).
