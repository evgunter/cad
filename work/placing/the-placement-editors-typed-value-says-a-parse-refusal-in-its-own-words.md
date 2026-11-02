---
id: the-placement-editors-typed-value-says-a-parse-refusal-in-its-own-words
kind: issue
title: The placement editor's typed value says a parse refusal in its own words, not the slot door's
status: open
opened: 2026-11-02
---


Found by FORMS' held-expression unit (branch `forms/held-expression`),
which gave every text door that parses an expression one home:
`session::parse_text` (`crates/viewer/src/session.rs`), the slot
door's parse and its refusal, `Refusal::Parse` — said as "the
expression did not parse: …". The slot row, the variable row and the
profile editor's held fields all go through it.

The placement editor's typed value does not.
`matetool::typed_value` (`crates/viewer/src/matetool.rs`, near line
1465) calls `parse_formula` itself and wraps the error in
`TypedValueError::Parse`, whose `Display` writes the bare parse error
(`Self::Parse(error) => write!(f, "{error}")`). So the same text typed
into a mate's value field and into a slot's field is refused in two
spellings.

The shape of a fix: `typed_value` parses through `session::parse_text`
and carries its `Refusal` (or its sentence), keeping its own dimension
check after the parse, which is a question the slot door does not ask.
