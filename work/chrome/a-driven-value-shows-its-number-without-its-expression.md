---
id: a-driven-value-shows-its-number-without-its-expression
kind: issue
title: viewer: a value an expression drives reads 'x = 0.05' where it should read 'x = w/2 = 0.05', wherever expressions are allowed
status: open
opened: 2026-11-01
priority: P1
cost: M
refs: [the-profile-editor-spends-two-lines-per-driven-argument, a-variable-is-edited-in-a-separate-widget-not-on-its-own-row]
---
Reported by Ev (chat, 2026-11-01): "probably everywhere where
expressions are allowed it should display like 'x = w/2 = 0.05' or
whatever instead of just 'x = 0.05' if variables are used." Filed P1
at Ev's request.

## What happens

The defined-variable line already does this:
`defined_line` (`crates/viewer/src/pane/properties.rs:1629`) writes
`w = 2 * t = 40 mm`. Other places that show a value an expression
drives give only the number, or put the expression and the number on
separate lines (see
`the-profile-editor-spends-two-lines-per-driven-argument`, where the
step field reads `target x = l` and the value sits in a sentence
below). The shared homes are `props::field_text`
(`crates/viewer/src/props.rs:987`), `props::computed_text`
(`props.rs:1074`) and `props::DRIVEN` (`props.rs:1081`).

## What Ev wants

Every place that shows a value that can be an expression writes
`<name> = <expression> = <value>` when the value is driven, and
`<name> = <value>` when it is literal. That covers slot rows, profile
step arguments, tree/feature row summaries and form fields. Survey
every reader of `field_text` / `computed_text` and give it one shape,
written in one place, rather than fixing each site by hand.
