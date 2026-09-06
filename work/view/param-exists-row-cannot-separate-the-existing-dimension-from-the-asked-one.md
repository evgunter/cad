---
id: param-exists-row-cannot-separate-the-existing-dimension-from-the-asked-one
kind: issue
title: the ParamExists row asks for the dimension it already declared, so it cannot see the arm report the wrong one
status: open
opened: 2026-09-06
refs: [2053]
---


Found by the style review of PR 2053 (`view/refusal-all`).

## The premise that excludes the failing mode

`crates/viewer/tests/panel_edits.rs:495-509` creates the `ParamExists`
refusal by asking `SessionOp::CreateParam` for `thickness` with
`DocParam::continuous(Dimension::Length, 1.0)`, over the declaration
`common::parametric_plate` already made — also `Dimension::Length`
(`crates/viewer/tests/common/mod.rs:204`). It then asserts

    exists.to_string().contains("(length)")

`Refusal::ParamExists`'s own doc says the payload *"carries the
existing declaration's dimension so the offer can name what already
stands there"* (`crates/viewer/src/session/refuse.rs:145-156`), and
`create_param` implements that by reading `existing.dim()`
(`crates/viewer/src/session.rs:1236-1241`). With both dimensions equal,
a `create_param` that passed the ASKED-for dimension forward instead —
which is the one mistake this arm exists to prevent, since
`DocEdit::SetDocParam` is create-or-replace at the API — renders the
same string and the row stays green.

Asking for `Dimension::Angle` over a `Length` declaration separates
them: the sentence should say `(length)`, and the pre-click notice in
`add_param_ui` (`crates/viewer/src/pane/properties.rs:195`, the same
composer) should agree.

## Class

This is the roster-picks-its-own-sample shape `prose_census`'s header
names, in the row that pins 2053's fix. Worth checking the same way
wherever a refusal's payload is asserted through its rendering and the
fixture makes two candidate sources of that payload identical.
