---
id: no-door-refuses-a-blank-parameter-name
kind: issue
title: No door refuses a blank parameter name — ParamName::new validates nothing and write_doc_param does not ask
status: open
opened: 2026-09-19
priority: P1
cost: E
---

Filed by a VNEWS census lane (`work/vnews/a-disabled-control-says-why-
in-four-shapes`) at merge base
`2654cc111417da806d9786c40136106469096fec`. An announced crossing:
`crates/editor-core` is EDIT's and MSOLVE's. **Nothing under `crates/`
was touched.**

## The hole

`crates/editor-core/src/doc.rs`'s `ParamName` is
`pub struct ParamName(pub String)` and its `ParamName::new` is a
`impl Into<String>` passthrough — no predicate, and the field is `pub`,
so the constructor is not even the only route.

`crates/editor-core/src/edit.rs`'s `write_doc_param` — which the header
names as the one place every declaration door routes through, *"the
check order is the LOAD door's"* — runs four checks and none is about
the name: the non-finite walk (`DocParam::first_non_finite`), the
distribution's `check`, the structural/continuous divide
(`is_continuous_count`), and the unit/dimension pairing (`measures()`).
Then `new.params.insert(name.clone(), value)`.

`crates/viewer/src/session.rs`'s `Session::create_param` — the door
above it — refuses exactly one thing, `Refusal::ParamExists`.

So `DocEdit::SetDocParam { name: ParamName::new(""), … }` **succeeds**
and the document holds a parameter declared under the empty name.

## What that costs

- An expression referring to it cannot be written.
  `crates/editor-core/src/parse.rs`'s lexer opens an identifier only on
  `c.is_alphabetic() || c == '_'` and then takes more, so `Tok::Ident`
  is non-empty by construction and the declaration is unreachable from
  the one surface parameters exist for.
- `Refusal::NoSuchParam`, `EditError::ContinuousParamCannotBeCount` and
  every other refusal that frames a name render it through
  `ParamName`'s bare-name `Display` (the *"one home for the spelling
  every refusal that FRAMES a parameter name in a sentence of its own
  uses"*), so each of those sentences gets a hole where the name goes:
  *"no document parameter named  — declare it first"*.
- The save/load round trip carries it: nothing in the persistence
  validator asks either, and the header's own claim is that the edit
  door and the load door *"refuse the same declarations"*.

## How it was found, and what it is not

The VNEWS census classified `crates/viewer/src/pane/properties.rs`'s
add-parameter Create button. Its gate is `ready = !name.is_empty() &&
dimension.is_some()`, and the name conjunct is **silent** — no
`on_disabled_hover_text`, no sentence beside it. By the census's rule
that is correct *given the doors as they are*: there is no refusal for
a blank name, so there is no sentence for the control to be a second
copy of.

That is the wrong reason for a correct outcome. The chrome's
`!name.is_empty()` is the tree's **only** enforcement of a rule every
door is silently relying on, and it lives in a GUI panel. The repair is
a door-side refusal, at which point the chrome's silent conjunct
becomes an ordinary census hit and gets the door's own sentence.

## What a fix has to decide

Whether "blank" is the whole rule. A name is a parse-time identifier
elsewhere (`ParseError::UnknownParam` echoes the bytes an author
typed), so a door that refuses `""` and admits `"1 2"` has moved the
inconsistency rather than closed it. Siblings on this slate that share
the question: `quoted-parameter-name-in-error-prose-has-no-decision`,
`param-ref-refusals-spell-two-facts-four-ways`.

## Home

EDIT's: `crates/editor-core/src/doc.rs` and
`crates/editor-core/src/edit.rs`. `crates/viewer/src/session.rs` and
`crates/viewer/src/pane/properties.rs` are VSEAM's and VNEWS's, and the
chrome side of this is a hand-off that lands after the door exists.
