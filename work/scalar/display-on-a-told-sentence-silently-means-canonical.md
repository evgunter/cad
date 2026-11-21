---
id: display-on-a-told-sentence-silently-means-canonical
kind: issue
title: Display on a Tell type compiles inside a tell body and silently means metres
status: open
opened: 2026-11-20
priority: P2
cost: M
parent: patherror-display-renders-float-noise
rides_with: a-number-in-a-unit-is-rounded-once
---


A type that implements `geom_core::Tell` also implements `Display`,
as the rendering in metres and radians
(`crates/geom-core/README.md`, "A number told to a person": "`Display`
is that rendering in metres and radians"). So inside a `tell` body,
`{inner}` on a told quantity or on an inner sentence compiles, reads
correctly in every test that renders canonical, and tells the reader
in millimetres a number in metres. PR 7229's own port did it ten
times (compared-pair members passed as named arguments) before a
second sweep found them.

The interim guard is `scripts/gates/told-in-the-notation.sh`: in a
function that takes a `notation` and a `Formatter`, every formatted
argument is `.told_in(notation)`, a constant, or listed by name. Its
blind list says what it cannot see (what a listed name is bound to;
text built before it is formatted; a helper the notation was never
handed to).

## The question

Should the type system carry this instead of a gate? Shapes:
`Told` (and `Member`, `Position`, `Sentence`) lose `Display`, and a
refusal's own `Display` is the one place that names
`Notation::CANONICAL` (a `{}` on a quantity then does not compile;
every existing `{told}` outside a `tell` body is a call site to
change, and `Debug` and log lines need a spelling); or `tell` takes
a writer that is not a `fmt::Formatter`, so `write!` into it cannot
format a `Display` at all; or the gate stays the answer.

## Decided

Ev, PR 7261 (2026-11-21): settled by `a-number-in-a-unit-is-rounded-once`, which this row rides with and which builds the answer.
